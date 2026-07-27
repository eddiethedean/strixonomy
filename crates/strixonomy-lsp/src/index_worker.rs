use crate::diagnostics::publish_diagnostics_for_state;
use crate::protocol::RunRobotResult;
use crate::state::ServerState;
use crossbeam_channel::{Receiver, Sender};
use lsp_server::{Message, Notification};
use lsp_types::{
    notification::{Notification as _, ShowMessage},
    MessageType, ShowMessageParams,
};
use std::path::{Path, PathBuf};
use std::thread;
use strixonomy_catalog::CatalogStats;

fn paths_equal(a: &Path, b: &Path) -> bool {
    a == b || a.canonicalize().ok().zip(b.canonicalize().ok()).is_some_and(|(x, y)| x == y)
}

fn seen_contains(seen: &[PathBuf], path: &Path) -> bool {
    seen.iter().any(|p| paths_equal(p, path))
}

/// Deduplicate index paths while preserving first-seen order (for silent debounce batches).
fn distinct_index_paths<'a>(paths: impl IntoIterator<Item = &'a Path>) -> Vec<PathBuf> {
    let mut seen = Vec::new();
    let mut out = Vec::new();
    for path in paths {
        if seen_contains(&seen, path) {
            continue;
        }
        seen.push(path.to_path_buf());
        out.push(path.to_path_buf());
    }
    out
}

struct IndexJob {
    workspace: PathBuf,
    reply: Option<Sender<Result<(CatalogStats, u64), String>>>,
}

struct RobotJob {
    robot_path: Option<String>,
    args: Vec<String>,
    reply: Sender<Result<RunRobotResult, String>>,
}

enum WorkerJob {
    Index(IndexJob),
    Robot(RobotJob),
}

/// Background worker that runs workspace reindex and ROBOT CLI off the LSP message thread.
#[derive(Clone)]
pub struct IndexWorker {
    job_tx: Sender<WorkerJob>,
}

impl IndexWorker {
    pub fn spawn(state: ServerState, lsp_sender: Sender<Message>) -> Self {
        let (job_tx, job_rx) = crossbeam_channel::unbounded();
        thread::spawn(move || run_worker(state, job_rx, lsp_sender));
        Self { job_tx }
    }

    /// Queue a debounced background reindex (no result returned to caller).
    pub fn enqueue(&self, workspace: PathBuf) {
        let _ = self.job_tx.send(WorkerJob::Index(IndexJob { workspace, reply: None }));
    }

    /// Queue a reindex and block until the worker finishes (used by `strixonomy/indexWorkspace`).
    pub fn enqueue_sync(&self, workspace: PathBuf) -> Result<(CatalogStats, u64), String> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.job_tx
            .send(WorkerJob::Index(IndexJob { workspace, reply: Some(tx) }))
            .map_err(|e| format!("index worker unavailable: {e}"))?;
        rx.recv().map_err(|e| format!("index worker dropped reply: {e}"))?
    }

    /// Run ROBOT CLI on the worker thread and block for the result.
    pub fn run_robot_sync(
        &self,
        robot_path: Option<String>,
        args: Vec<String>,
    ) -> Result<RunRobotResult, String> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.job_tx
            .send(WorkerJob::Robot(RobotJob { robot_path, args, reply: tx }))
            .map_err(|e| format!("index worker unavailable: {e}"))?;
        rx.recv().map_err(|e| format!("index worker dropped reply: {e}"))?
    }
}

fn run_worker(state: ServerState, job_rx: Receiver<WorkerJob>, lsp_sender: Sender<Message>) {
    while let Ok(first) = job_rx.recv() {
        match first {
            WorkerJob::Robot(job) => {
                let result = run_robot_job(job.robot_path.as_deref(), &job.args);
                let _ = job.reply.send(result);
            }
            WorkerJob::Index(first_index) => {
                let mut batch = vec![first_index];
                let mut pending_robots = Vec::new();
                // Drain without dropping non-Index jobs (Robot must not be lost).
                while let Ok(job) = job_rx.try_recv() {
                    match job {
                        WorkerJob::Index(next) => batch.push(next),
                        WorkerJob::Robot(robot) => pending_robots.push(robot),
                    }
                }

                // Coalesce silent debounce jobs by distinct path (#215). Sync callers
                // (reply: Some) must get a result for their requested workspace.
                let (silent, sync_jobs): (Vec<_>, Vec<_>) =
                    batch.into_iter().partition(|j| j.reply.is_none());

                if sync_jobs.is_empty() {
                    for path in distinct_index_paths(silent.iter().map(|j| j.workspace.as_path())) {
                        run_index_job(&state, &lsp_sender, path, &[]);
                    }
                } else {
                    // Run each distinct sync workspace once; attach silent jobs only when
                    // they match that path.
                    let mut seen = Vec::new();
                    for job in &sync_jobs {
                        let path = &job.workspace;
                        if seen_contains(&seen, path) {
                            continue;
                        }
                        seen.push(path.clone());
                        let replies: Vec<_> = sync_jobs
                            .iter()
                            .filter(|j| paths_equal(&j.workspace, path))
                            .filter_map(|j| j.reply.clone())
                            .collect();
                        run_index_job(&state, &lsp_sender, path.clone(), &replies);
                    }
                    // Silent jobs for paths not covered by a sync request.
                    for path in distinct_index_paths(silent.iter().map(|j| j.workspace.as_path())) {
                        if !seen_contains(&seen, &path) {
                            run_index_job(&state, &lsp_sender, path.clone(), &[]);
                            seen.push(path);
                        }
                    }
                }

                for robot in pending_robots {
                    let result = run_robot_job(robot.robot_path.as_deref(), &robot.args);
                    let _ = robot.reply.send(result);
                }
            }
        }
    }
}

fn run_index_job(
    state: &ServerState,
    lsp_sender: &Sender<Message>,
    workspace: PathBuf,
    replies: &[Sender<Result<(CatalogStats, u64), String>>],
) {
    let result = state.index_workspace(workspace);
    match &result {
        Ok(_) => {
            publish_diagnostics_for_state(lsp_sender, state);
        }
        Err(message) => notify_index_failure(lsp_sender, message),
    }
    for reply in replies {
        let _ = reply.send(result.clone());
    }
}

fn run_robot_job(robot_path: Option<&str>, args: &[String]) -> Result<RunRobotResult, String> {
    let output = strixonomy_robot::run_robot(robot_path, args).map_err(|e| e.to_string())?;
    Ok(RunRobotResult { exit_code: output.exit_code, stdout: output.stdout, stderr: output.stderr })
}

fn notify_index_failure(sender: &Sender<Message>, message: &str) {
    let params = ShowMessageParams {
        typ: MessageType::ERROR,
        message: format!("Strixonomy reindex failed: {message}"),
    };
    let notif = Notification {
        method: ShowMessage::METHOD.to_string(),
        params: serde_json::to_value(params).unwrap_or_default(),
    };
    let _ = sender.send(Message::Notification(notif));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silent_debounce_keeps_distinct_paths() {
        let a = PathBuf::from("/ws/a");
        let b = PathBuf::from("/ws/b");
        let paths = distinct_index_paths([&a, &b, &a].into_iter().map(|p| p.as_path()));
        assert_eq!(paths, vec![a, b]);
    }

    #[test]
    fn silent_debounce_dedupes_identical_paths() {
        let a = PathBuf::from("/ws/a");
        let paths = distinct_index_paths([&a, &a, &a].into_iter().map(|p| p.as_path()));
        assert_eq!(paths, vec![a]);
    }
}
