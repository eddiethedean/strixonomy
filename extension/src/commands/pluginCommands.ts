import * as vscode from "vscode";
import { listPlugins, runPlugin } from "../lsp/client";
import type { PluginDescriptor } from "../lsp/protocol";
import { WorkflowPanel } from "../webviews/workflowPanel";
import { PluginViewPanel } from "../webviews/pluginViewPanel";

const registered = new Map<string, vscode.Disposable>();
const output = vscode.window.createOutputChannel("Strixonomy Plugins");

function actionForPluginKind(kind: string): "validate" | "export" | "workflow" {
  switch (kind) {
    case "exporter":
      return "export";
    case "workflow":
      return "workflow";
    case "validator":
    default:
      return "validate";
  }
}

export async function refreshPluginCommands(
  context: vscode.ExtensionContext
): Promise<PluginDescriptor[]> {
  for (const d of registered.values()) {
    d.dispose();
  }
  registered.clear();

  let plugins: PluginDescriptor[] = [];
  try {
    const result = await listPlugins();
    plugins = result.plugins;
  } catch {
    return plugins;
  }

  for (const plugin of plugins) {
    for (const cmd of plugin.ui.commands) {
      const commandId = `strixonomy.plugin.${cmd.id}`;
      const disposable = vscode.commands.registerCommand(commandId, async () => {
        output.appendLine(`Running plugin command: ${plugin.id} — ${cmd.title}`);
        output.show(true);

        // Legacy compatibility: owlmake is a workflow with well-known steps.
        if (plugin.id === "owlmake" || cmd.id.includes("owlmake")) {
          await WorkflowPanel.runOwlmake("qc");
          return;
        }

        const action = actionForPluginKind(plugin.kind);
        const step = action === "workflow" ? cmd.id : undefined;
        const result = await runPlugin({
          plugin_id: plugin.id,
          action,
          step,
        });

        if (result.logs) {
          output.appendLine(result.logs);
        }
        if (!result.success) {
          void vscode.window.showErrorMessage(
            `Strixonomy plugin failed: ${plugin.name} — ${cmd.title}`
          );
          return;
        }
        void vscode.window.showInformationMessage(
          `Strixonomy plugin ran: ${plugin.name} — ${cmd.title}`
        );
      });
      registered.set(commandId, disposable);
      context.subscriptions.push(disposable);
    }

    for (const view of plugin.ui.views ?? []) {
      const commandId = `strixonomy.plugin.view.${plugin.id}.${view.id}`;
      const disposable = vscode.commands.registerCommand(commandId, async () => {
        await PluginViewPanel.open(context.extensionUri, plugin, view);
      });
      registered.set(commandId, disposable);
      context.subscriptions.push(disposable);
    }
  }

  return plugins;
}

export function disposePluginCommands(): void {
  for (const d of registered.values()) {
    d.dispose();
  }
  registered.clear();
  output.dispose();
}
