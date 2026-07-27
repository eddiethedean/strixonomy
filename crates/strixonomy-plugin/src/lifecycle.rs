//! Dependency resolution and activation ordering for the plugin host.

use crate::manifest::DiscoveredPlugin;
use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LifecycleError {
    #[error("plugin dependency cycle involving: {0}")]
    Cycle(String),
    #[error("plugin {0} depends on missing plugin {1}")]
    MissingDependency(String, String),
}

/// Topologically sort plugin ids by `depends_on`.
///
/// Only plugins present in `plugins` are ordered. Missing dependencies fail.
pub fn activation_order(plugins: &[DiscoveredPlugin]) -> Result<Vec<String>, LifecycleError> {
    let ids: HashSet<String> = plugins.iter().map(|p| p.plugin_id().to_string()).collect();
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    let mut indegree: HashMap<String, usize> = HashMap::new();

    for p in plugins {
        let id = p.plugin_id().to_string();
        indegree.entry(id.clone()).or_insert(0);
        let mut unique_deps = HashSet::new();
        for dep in &p.manifest.depends_on {
            if !ids.contains(dep) {
                return Err(LifecycleError::MissingDependency(id, dep.clone()));
            }
            if unique_deps.insert(dep.clone()) {
                deps.entry(dep.clone()).or_default().push(id.clone());
                *indegree.entry(id.clone()).or_insert(0) += 1;
            }
        }
    }

    // Stable seed order by plugin id for deterministic queues.
    let mut queue: VecDeque<String> =
        indegree.iter().filter(|(_, d)| **d == 0).map(|(id, _)| id.clone()).collect();
    let mut q_vec: Vec<_> = queue.drain(..).collect();
    q_vec.sort();
    queue.extend(q_vec);

    let mut order = Vec::new();
    while let Some(id) = queue.pop_front() {
        order.push(id.clone());
        if let Some(dependents) = deps.get(&id) {
            let mut next = Vec::new();
            for dep in dependents {
                if let Some(d) = indegree.get_mut(dep) {
                    *d = d.saturating_sub(1);
                    if *d == 0 {
                        next.push(dep.clone());
                    }
                }
            }
            next.sort();
            queue.extend(next);
        }
    }

    if order.len() != plugins.len() {
        let leftover: Vec<_> =
            indegree.into_iter().filter(|(_, d)| *d > 0).map(|(id, _)| id).collect();
        return Err(LifecycleError::Cycle(leftover.join(", ")));
    }
    Ok(order)
}

/// Plugins that directly or transitively depend on `plugin_id`.
pub fn dependents_of(plugins: &[DiscoveredPlugin], plugin_id: &str) -> Vec<String> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for p in plugins {
        let id = p.plugin_id().to_string();
        for dep in &p.manifest.depends_on {
            graph.entry(dep.clone()).or_default().push(id.clone());
        }
    }
    let mut out = Vec::new();
    let mut stack = vec![plugin_id.to_string()];
    let mut seen = HashSet::new();
    while let Some(cur) = stack.pop() {
        if let Some(deps) = graph.get(&cur) {
            for d in deps {
                if seen.insert(d.clone()) {
                    out.push(d.clone());
                    stack.push(d.clone());
                }
            }
        }
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::parse_manifest;
    use std::path::PathBuf;

    fn plugin(toml: &str) -> DiscoveredPlugin {
        DiscoveredPlugin {
            manifest: parse_manifest(toml).expect("manifest"),
            manifest_path: PathBuf::from("/tmp/.strixonomy/plugins/x.toml"),
        }
    }

    #[test]
    fn orders_dependencies() {
        let a = plugin(
            r#"
[plugin]
name = "a"
version = "0.1.0"
kind = "validator"
id = "a"
permissions = ["workspace.read"]
"#,
        );
        let b = plugin(
            r#"
[plugin]
name = "b"
version = "0.1.0"
kind = "graph"
id = "b"
depends_on = ["a"]
permissions = ["workspace.read"]
"#,
        );
        let order = activation_order(&[b, a]).expect("order");
        assert_eq!(order, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn detects_cycle() {
        let a = plugin(
            r#"
[plugin]
name = "a"
version = "0.1.0"
kind = "validator"
id = "a"
depends_on = ["b"]
permissions = ["workspace.read"]
"#,
        );
        let b = plugin(
            r#"
[plugin]
name = "b"
version = "0.1.0"
kind = "validator"
id = "b"
depends_on = ["a"]
permissions = ["workspace.read"]
"#,
        );
        assert!(matches!(activation_order(&[a, b]), Err(LifecycleError::Cycle(_))));
    }

    #[test]
    fn missing_dependency_errors() {
        let b = plugin(
            r#"
[plugin]
name = "b"
version = "0.1.0"
kind = "graph"
id = "b"
depends_on = ["missing"]
permissions = ["workspace.read"]
"#,
        );
        assert!(matches!(activation_order(&[b]), Err(LifecycleError::MissingDependency(_, _))));
    }
}
