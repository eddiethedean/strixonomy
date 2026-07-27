import * as vscode from "vscode";
import { listSwrlRules } from "../lsp/client";
import { forgetPanelRestoreState, rememberPanelRestoreState } from "./layoutPersistence";
import type { WebviewMessage } from "./messages";
import { PanelHost } from "./panelHost";
import type { RuleEditorOptions } from "./ruleEditorReact";

export class RuleBrowserPanel {
  public static current: RuleBrowserPanel | undefined;

  private constructor(private readonly host: PanelHost) {
    this.host.panel.onDidDispose(() => {
      void forgetPanelRestoreState("strixonomyRuleBrowser");
      RuleBrowserPanel.current = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static show(extensionUri: vscode.Uri): RuleBrowserPanel {
    void rememberPanelRestoreState("strixonomyRuleBrowser", {
      command: "strixonomy.openRuleBrowser",
      title: "SWRL Rule Browser",
    });
    if (RuleBrowserPanel.current) {
      RuleBrowserPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      void RuleBrowserPanel.current.refresh();
      return RuleBrowserPanel.current;
    }
    const host = PanelHost.create(extensionUri, {
      viewType: "strixonomyRuleBrowser",
      title: "SWRL Rule Browser",
      panel: "ruleBrowser",
      onMessage: async (message: WebviewMessage) => {
        const panel = RuleBrowserPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });
    const instance = new RuleBrowserPanel(host);
    RuleBrowserPanel.current = instance;
    void instance.refresh();
    return instance;
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "refreshSwrlRules" || message.type === "ready") {
      await this.refresh();
      return;
    }
    if (message.type === "openSwrlRuleEditor") {
      await vscode.commands.executeCommand("strixonomy.openRuleEditor", {
        ruleJson: message.ruleJson,
        originalRuleJson: message.ruleJson,
        documentUri: message.documentUri,
        ontologyIri: message.ontologyIri,
      } satisfies RuleEditorOptions);
    }
  }

  private async refresh(): Promise<void> {
    try {
      const result = await listSwrlRules();
      this.host.postMessage({ type: "swrlRulesLoaded", rules: result.rules });
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      this.host.postMessage({ type: "error", message });
    }
  }
}
