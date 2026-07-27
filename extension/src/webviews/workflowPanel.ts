import * as vscode from "vscode";
import { runPlugin } from "../lsp/client";

export class WorkflowPanel {
  private static output: vscode.OutputChannel | undefined;

  private static channel(): vscode.OutputChannel {
    if (!WorkflowPanel.output) {
      WorkflowPanel.output = vscode.window.createOutputChannel("Strixonomy Workflow");
    }
    return WorkflowPanel.output;
  }

  public static async runOwlmake(step = "qc"): Promise<void> {
    const ch = WorkflowPanel.channel();
    ch.show(true);
    ch.appendLine(`Running workflow plugin 'owlmake' step: ${step}`);
    try {
      const result = await runPlugin({
        plugin_id: "owlmake",
        action: "workflow",
        step,
      });
      if (result.logs) {
        ch.appendLine(result.logs);
      }
      for (const diag of result.diagnostics) {
        ch.appendLine(`[${diag.code}] ${diag.message}`);
      }
      if (!result.success) {
        void vscode.window.showErrorMessage(
          `Strixonomy: owlmake workflow step '${step}' failed — see Output → Strixonomy Workflow`
        );
        return;
      }
      void vscode.window.showInformationMessage(
        `Strixonomy: owlmake workflow step '${step}' completed`
      );
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      ch.appendLine(`Error: ${message}`);
      void vscode.window.showErrorMessage(`Strixonomy workflow failed: ${message}`);
    }
  }
}
