import * as vscode from "vscode";
import type { LspWorkspaceEdit } from "./protocol";

type DocChange = {
  textDocument?: { uri?: string; version?: number | null };
  text_document?: { uri?: string; version?: number | null };
  edits?: Array<{
    range?: {
      start: { line: number; character: number };
      end: { line: number; character: number };
    };
    newText?: string;
    new_text?: string;
    /** AnnotatedTextEdit may nest the TextEdit. */
    text_edit?: {
      range: {
        start: { line: number; character: number };
        end: { line: number; character: number };
      };
      newText?: string;
      new_text?: string;
    };
    annotationId?: string;
    annotation_id?: string;
  }>;
};

function documentChanges(
  edit: LspWorkspaceEdit & { documentChanges?: DocChange[] }
): DocChange[] {
  const changes =
    (edit as { documentChanges?: DocChange[] }).documentChanges ??
    edit.document_changes ??
    [];
  return changes as DocChange[];
}

function isUriInWorkspace(uri: vscode.Uri): boolean {
  if (uri.scheme !== "file") {
    return false;
  }
  return vscode.workspace.getWorkspaceFolder(uri) !== undefined;
}

/** Apply a language-server `WorkspaceEdit` to open VS Code editors. */
export async function applyLspWorkspaceEdit(
  edit: LspWorkspaceEdit | undefined,
  options?: { expectChanges?: boolean }
): Promise<boolean> {
  if (!edit) {
    return true;
  }
  const changes = documentChanges(edit as LspWorkspaceEdit & { documentChanges?: DocChange[] });
  if (!changes.length) {
    if (
      (edit as { documentChanges?: unknown }).documentChanges != null ||
      edit.document_changes != null
    ) {
      return false;
    }
    if (options?.expectChanges) {
      return false;
    }
    return true;
  }

  const workspaceEdit = new vscode.WorkspaceEdit();
  for (const docChange of changes) {
    const textDoc = docChange.textDocument ?? docChange.text_document;
    const uriString = textDoc?.uri;
    if (!uriString) {
      return false;
    }
    const uri = vscode.Uri.parse(uriString);
    if (!isUriInWorkspace(uri)) {
      void vscode.window.showErrorMessage(
        `Strixonomy: refusing to apply edit outside the workspace (${uriString})`
      );
      return false;
    }
    const document = await vscode.workspace.openTextDocument(uri);
    for (const rawEdit of docChange.edits ?? []) {
      const nested = rawEdit.text_edit;
      const range = rawEdit.range ?? nested?.range;
      if (!range?.start || !range?.end) {
        return false;
      }
      const endLine =
        range.end.line >= 0xfffffff0 ? document.lineCount : range.end.line;
      const endCharacter =
        range.end.line >= 0xfffffff0 ? 0 : range.end.character;
      if (
        !Number.isFinite(range.start.line) ||
        !Number.isFinite(range.start.character) ||
        !Number.isFinite(endLine) ||
        !Number.isFinite(endCharacter)
      ) {
        return false;
      }
      const vscodeRange = new vscode.Range(
        range.start.line,
        range.start.character,
        endLine,
        endCharacter
      );
      const newText =
        rawEdit.newText ??
        rawEdit.new_text ??
        nested?.newText ??
        nested?.new_text ??
        "";
      workspaceEdit.replace(uri, vscodeRange, newText);
    }
  }
  return vscode.workspace.applyEdit(workspaceEdit);
}
