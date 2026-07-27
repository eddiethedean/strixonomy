import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import type { PanelKind } from "./messages";
import {
  formatWebviewQuery,
  webviewLocationBootstrapScript,
} from "./webviewBootstrap";

export interface WebviewAssetManifest {
  scriptUri: vscode.Uri;
  styleUri?: vscode.Uri;
}

function distDir(extensionUri: vscode.Uri): string {
  return path.join(extensionUri.fsPath, "webview-ui", "dist");
}

/**
 * Resolve built React bundle URIs from extension/webview-ui/dist.
 */
export function getWebviewAssets(
  webview: vscode.Webview,
  extensionUri: vscode.Uri
): WebviewAssetManifest {
  const dist = vscode.Uri.joinPath(extensionUri, "webview-ui", "dist");
  const assetsDir = path.join(distDir(extensionUri), "assets");

  let scriptName = "index.js";
  let styleName: string | undefined = "index.css";

  if (fs.existsSync(assetsDir)) {
    const files = fs.readdirSync(assetsDir);
    const js = files.find((f) => f.endsWith(".js") && !f.endsWith(".map"));
    const css = files.find((f) => f.endsWith(".css"));
    if (js) {
      scriptName = js;
    }
    styleName = css;
  }

  const scriptUri = webview.asWebviewUri(
    vscode.Uri.joinPath(dist, "assets", scriptName)
  );
  const styleUri = styleName
    ? webview.asWebviewUri(vscode.Uri.joinPath(dist, "assets", styleName))
    : undefined;
  return { scriptUri, styleUri };
}

export function getWebviewHtml(
  webview: vscode.Webview,
  extensionUri: vscode.Uri,
  panel: PanelKind,
  extraQuery?: Record<string, string>
): string {
  const nonce = getNonce();
  const { scriptUri, styleUri } = getWebviewAssets(webview, extensionUri);
  const query = formatWebviewQuery(panel, extraQuery);
  const csp = [
    `default-src 'none'`,
    `style-src ${webview.cspSource} 'unsafe-inline'`,
    `script-src 'nonce-${nonce}'`,
    `font-src ${webview.cspSource}`,
    `img-src ${webview.cspSource} data:`,
  ].join("; ");

  const styleTag = styleUri
    ? `<link rel="stylesheet" href="${styleUri}">`
    : "";

  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="Content-Security-Policy" content="${csp}" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  ${styleTag}
  <title>Strixonomy</title>
</head>
<body>
  <div id="root"></div>
  <script nonce="${nonce}">
    ${webviewLocationBootstrapScript(query)}
  </script>
  <script type="module" nonce="${nonce}" src="${scriptUri}"></script>
</body>
</html>`;
}

function getNonce(): string {
  const chars =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let text = "";
  for (let i = 0; i < 32; i++) {
    text += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return text;
}
