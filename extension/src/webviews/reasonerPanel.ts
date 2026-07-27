import * as vscode from "vscode";
import { focusRelay } from "../focus/focusRelay";
import type { ReasoningStatePayload } from "../focus/types";
import { cancelActiveReasonerRequest, runReasoner } from "../lsp/client";
import type { RunReasonerResult } from "../lsp/protocol";
import { forgetPanelRestoreState, rememberPanelRestoreState } from "./layoutPersistence";
import type { ReasonerResultPayload, WebviewMessage } from "./messages";
import { PanelHost } from "./panelHost";
import {
  captureReasoningPreRun,
  reasoningStateForRunCancel,
  reasoningStateForRunError,
  reasoningStateForRunStart,
  reasoningStateForRunSuccess,
  summarizeResult,
} from "./reasonerPanelLogic";

function toPayload(result: RunReasonerResult): ReasonerResultPayload {
  return {
    profile_used: result.profile_used,
    consistent: result.consistent,
    unsatisfiable: result.unsatisfiable,
    inferred_edge_count: result.inferred_edge_count,
    new_inferences: result.new_inferences,
    warnings: result.warnings,
    duration_ms: result.duration_ms,
    snapshot: result.snapshot,
  };
}

export class ReasonerPanel {
  public static current: ReasonerPanel | undefined;
  private lastResult: RunReasonerResult | undefined;
  private runId = 0;
  private preRunSnapshot: ReasoningStatePayload | undefined;

  private constructor(private readonly host: PanelHost) {
    this.host.panel.onDidDispose(() => {
      void forgetPanelRestoreState("strixonomyReasoner");
      ReasonerPanel.current = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static show(extensionUri: vscode.Uri): ReasonerPanel {
    void rememberPanelRestoreState("strixonomyReasoner", {
      command: "strixonomy.classifyOntology",
      title: "Strixonomy Reasoner",
    });
    if (ReasonerPanel.current) {
      ReasonerPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      return ReasonerPanel.current;
    }
    const host = PanelHost.create(extensionUri, {
      viewType: "strixonomyReasoner",
      title: "Strixonomy Reasoner",
      panel: "reasoner",
      onMessage: async (message: WebviewMessage) => {
        const panel = ReasonerPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });
    ReasonerPanel.current = new ReasonerPanel(host);
    return ReasonerPanel.current;
  }

  public async runWithDefaults(): Promise<void> {
    const cfg = vscode.workspace.getConfiguration("strixonomy");
    const profile = cfg.get<string>("reasoner.default") ?? "el";
    const autoDetect = cfg.get<boolean>("reasoner.autoProfile") ?? true;
    await this.run(profile, autoDetect, ++this.runId);
  }

  /** Push an already-computed result into the React panel. */
  public presentResult(result: RunReasonerResult): void {
    this.lastResult = result;
    const runId = ++this.runId;
    this.host.postMessage({ type: "reasonerSyncRunId", runId });
    this.host.postMessage({
      type: "reasonerResult",
      runId,
      result: toPayload(result),
      summary: summarizeResult(result),
    });
  }

  /** Invalidate in-flight runs so late RPC results are ignored (#141). */
  public cancelActiveRun(): void {
    cancelActiveReasonerRequest();
    this.runId += 1;
    // Dedicated cancel message: must clear React `running` without using
    // reasonerSyncRunId (which intentionally does not clear running — #212/#269).
    this.host.postMessage({ type: "reasonerRunCancelled", runId: this.runId });
    const restore =
      this.preRunSnapshot ??
      captureReasoningPreRun(focusRelay.getReasoning());
    this.preRunSnapshot = undefined;
    focusRelay.setReasoningState(reasoningStateForRunCancel(restore));
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "runReasoner") {
      await this.run(message.profile, message.autoDetect, message.runId);
      return;
    }
    if (message.type === "explainUnsat") {
      const profile =
        this.lastResult?.profile_used ?? focusRelay.getReasoning()?.profile;
      await vscode.commands.executeCommand(
        "strixonomy.showExplanation",
        message.classIri,
        profile
      );
      return;
    }
    if (message.type === "showInferredHierarchy") {
      await vscode.workspace
        .getConfiguration("strixonomy")
        .update("hierarchy.mode", "combined", vscode.ConfigurationTarget.Workspace);
      void vscode.commands.executeCommand("strixonomy.refreshExplorer");
    }
  }

  private async run(
    profile: string,
    autoDetect: boolean,
    runId: number
  ): Promise<void> {
    this.runId = runId;
    this.preRunSnapshot = captureReasoningPreRun(focusRelay.getReasoning());
    try {
      focusRelay.setReasoningState(
        reasoningStateForRunStart(profile, this.preRunSnapshot)
      );
      const result = await runReasoner({ profile, auto_detect: autoDetect });
      if (runId !== this.runId) {
        return;
      }
      this.lastResult = result;
      const preRun = this.preRunSnapshot ?? captureReasoningPreRun(null);
      this.preRunSnapshot = undefined;
      focusRelay.setReasoningState(
        reasoningStateForRunSuccess(
          result.profile_used ?? profile,
          result.unsatisfiable ?? [],
          preRun
        )
      );
      this.host.postMessage({
        type: "reasonerResult",
        runId,
        result: toPayload(result),
        summary: summarizeResult(result),
      });
      void vscode.commands.executeCommand("strixonomy.refreshExplorer");
    } catch (err) {
      if (runId !== this.runId) {
        return;
      }
      const message = err instanceof Error ? err.message : String(err);
      const preRun = this.preRunSnapshot ?? captureReasoningPreRun(null);
      this.preRunSnapshot = undefined;
      focusRelay.setReasoningState(
        reasoningStateForRunError(profile, preRun, this.lastResult?.unsatisfiable)
      );
      this.host.postMessage({
        type: "reasonerResult",
        runId,
        error: message,
      });
    }
  }
}
