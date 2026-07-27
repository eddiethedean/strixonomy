import * as vscode from "vscode";

export {
  isPathUnderFolder,
  normalizeFsPath,
  pathsEqual,
} from "./pathUnder";
import { normalizeFsPath } from "./pathUnder";

/** True when a file URI belongs to an open workspace folder. */
export function isUriInWorkspace(uri: vscode.Uri): boolean {
  if (uri.scheme !== "file") {
    return false;
  }
  return vscode.workspace.getWorkspaceFolder(uri) !== undefined;
}

/** Open a file only when it lies inside the workspace; show an error otherwise. */
export async function openWorkspaceTextDocument(
  filePath: string
): Promise<vscode.TextDocument | undefined> {
  const uri = vscode.Uri.file(normalizeFsPath(filePath));
  if (!isUriInWorkspace(uri)) {
    void vscode.window.showErrorMessage(
      "Strixonomy: path is outside the workspace"
    );
    return undefined;
  }
  return vscode.workspace.openTextDocument(uri);
}

/** Validate a document path from the language server before editing. */
export function documentUriInWorkspace(documentPath: string): string | undefined {
  const uri = vscode.Uri.file(normalizeFsPath(documentPath));
  if (!isUriInWorkspace(uri)) {
    return undefined;
  }
  return uri.toString();
}

/** User-facing message when an LSP document path is outside workspace folders. */
export const WORKSPACE_DOCUMENT_OUTSIDE_MESSAGE =
  "Strixonomy: entity document path is outside the workspace";

/** Resolve a file path or `file://` URI to a workspace document URI string. */
export function resolveWorkspaceDocumentUri(
  uriOrPath: string
): string | undefined {
  if (uriOrPath.startsWith("file:")) {
    try {
      return documentUriInWorkspace(vscode.Uri.parse(uriOrPath).fsPath);
    } catch {
      return undefined;
    }
  }
  return documentUriInWorkspace(uriOrPath);
}

/** Keep only `file:` URIs that resolve inside an open workspace folder (#311). */
export function filterWorkspaceFileUris(uris: string[]): string[] {
  const kept: string[] = [];
  for (const value of uris) {
    try {
      const uri = vscode.Uri.parse(value);
      if (!isUriInWorkspace(uri)) {
        continue;
      }
      kept.push(uri.toString());
    } catch {
      // Drop malformed entries.
    }
  }
  return kept;
}
