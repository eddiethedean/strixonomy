import * as vscode from "vscode";
import { semanticDiff } from "../lsp/client";
import { PanelHost } from "./panelHost";
import type { DiffPayload, WebviewMessage } from "./messages";
import { isDiffPayload } from "../lsp/protocolGuards";
import { formatSemanticDiffMarkdown } from "./semanticDiffMarkdown";

export class SemanticDiffPanel {
  public static current: SemanticDiffPanel | undefined;

  private refreshGeneration = 0;
  private lastDiff: DiffPayload | undefined;

  private constructor(private readonly host: PanelHost) {
    host.panel.onDidDispose(() => {
      SemanticDiffPanel.current = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static async show(
    extensionUri: vscode.Uri,
    params: { leftRef?: string; rightRef?: string; reasoner?: boolean }
  ): Promise<SemanticDiffPanel> {
    if (SemanticDiffPanel.current) {
      SemanticDiffPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      await SemanticDiffPanel.current.refresh(params);
      return SemanticDiffPanel.current;
    }

    const host = PanelHost.create(extensionUri, {
      viewType: "strixonomySemanticDiff",
      title: "Strixonomy Semantic Diff",
      panel: "semanticDiff",
      onMessage: async (message: WebviewMessage) => {
        const panel = SemanticDiffPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });

    const instance = new SemanticDiffPanel(host);
    SemanticDiffPanel.current = instance;
    await instance.refresh(params);
    return instance;
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "copyMarkdown") {
      const diff = this.lastDiff;
      if (!diff) {
        void vscode.window.showWarningMessage("Strixonomy: no diff to copy");
        return;
      }
      await vscode.env.clipboard.writeText(formatSemanticDiffMarkdown(diff));
      void vscode.window.showInformationMessage("Strixonomy: diff Markdown copied to clipboard");
    }
  }

  private async refresh(params: {
    leftRef?: string;
    rightRef?: string;
    reasoner?: boolean;
  }): Promise<void> {
    const generation = ++this.refreshGeneration;
    this.host.postMessage({ type: "loading" });
    try {
      const result = await semanticDiff({
        left_ref: params.leftRef ?? "HEAD",
        right_ref: params.rightRef ?? "WORKTREE",
        reasoner: params.reasoner,
      });
      if (generation !== this.refreshGeneration) {
        return;
      }
      if (!isDiffPayload(result.diff)) {
        throw new Error("Invalid semantic diff payload from language server");
      }
      this.lastDiff = result.diff;
      this.host.postMessage({ type: "semanticDiffData", diff: result.diff });
    } catch (err) {
      if (generation !== this.refreshGeneration) {
        return;
      }
      const message = err instanceof Error ? err.message : String(err);
      this.host.postMessage({ type: "error", message });
    }
  }
}
