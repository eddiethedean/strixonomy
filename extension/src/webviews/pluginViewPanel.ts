import * as vscode from "vscode";
import { runPlugin } from "../lsp/client";
import type { PluginDescriptor, PluginViewContribution } from "../lsp/protocol";

export class PluginViewPanel {
  static async open(
    extensionUri: vscode.Uri,
    plugin: PluginDescriptor,
    view: PluginViewContribution
  ): Promise<void> {
    const panel = vscode.window.createWebviewPanel(
      "strixonomy.pluginView",
      `${plugin.name}: ${view.title}`,
      vscode.ViewColumn.Beside,
      {
        enableScripts: false,
        retainContextWhenHidden: true,
      }
    );

    panel.webview.html = `<html><body><p>Loading view…</p></body></html>`;

    try {
      const result = await runPlugin({
        plugin_id: plugin.id,
        action: "ui_view",
        view_id: view.id,
      });

      const html =
        result.view_html ??
        (result.logs
          ? `<pre>${escapeHtml(result.logs)}</pre>`
          : `<p>No view output.</p>`);

      panel.webview.html = wrapHtml(
        `${plugin.name}: ${view.title}`,
        html,
        extensionUri
      );
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      panel.webview.html = wrapHtml(
        `${plugin.name}: ${view.title}`,
        `<pre>${escapeHtml(message)}</pre>`,
        extensionUri
      );
    }
  }
}

function wrapHtml(title: string, body: string, _extensionUri: vscode.Uri): string {
  // Keep this minimal and CSP-safe; plugin HTML is treated as trusted-by-user content
  // because it runs locally and is explicitly installed in the workspace.
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline';" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>${escapeHtml(title)}</title>
    <style>
      :root {
        color-scheme: light dark;
      }
      body {
        margin: 0;
        padding: 16px;
        font-family: var(--vscode-font-family);
        font-size: var(--vscode-font-size);
        line-height: 1.5;
        color: var(--vscode-foreground);
        background: var(--vscode-editor-background);
      }
      h1, h2, h3 {
        margin: 0 0 12px;
        font-weight: 650;
        letter-spacing: -0.01em;
      }
      a { color: var(--vscode-textLink-foreground); }
      pre {
        white-space: pre-wrap;
        margin: 0;
        padding: 12px;
        border-radius: 8px;
        background: var(--vscode-textBlockQuote-background, var(--vscode-editorWidget-background));
        border: 1px solid var(--vscode-widget-border, transparent);
        font-family: var(--vscode-editor-font-family, ui-monospace, monospace);
      }
      button:focus-visible, a:focus-visible {
        outline: 2px solid var(--vscode-focusBorder);
        outline-offset: 2px;
      }
    </style>
  </head>
  <body>
    <header style="margin-bottom:12px">
      <h1 style="font-size:1.25rem">${escapeHtml(title)}</h1>
    </header>
    <main aria-label="${escapeHtml(title)}">
      ${body}
    </main>
  </body>
</html>`;
}

function escapeHtml(text: string): string {
  return text
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#039;");
}

