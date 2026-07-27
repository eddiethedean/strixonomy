import * as vscode from "vscode";
import { indexWorkspace } from "../lsp/client";
import { isOntologyDocument } from "../commands/uiState";
import { normalizeFsPath } from "../utils/pathUnder";
import { ontologyRegistry } from "./ontologyRegistry";
import { isSelfWrite } from "./selfWriteGuard";

type ExternalChangeChoice = "reload" | "keep" | "compare";

export class ExternalChangeRecovery {
  private watchers: vscode.FileSystemWatcher[] = [];
  private pending = new Map<string, Promise<ExternalChangeChoice>>();

  register(context: vscode.ExtensionContext): void {
    const pattern = "**/*.{ttl,owl,rdf,owx,obo,jsonld,nt,nq,trig}";
    const watcher = vscode.workspace.createFileSystemWatcher(pattern);
    watcher.onDidChange((uri) => void this.handleExternalChange(uri, "change"));
    watcher.onDidCreate((uri) => void this.handleExternalChange(uri, "create"));
    this.watchers.push(watcher);
    context.subscriptions.push(watcher);
  }

  async handleExternalChange(
    uri: vscode.Uri,
    kind: "change" | "create"
  ): Promise<void> {
    if (uri.scheme !== "file") {
      return;
    }
    const path = normalizeFsPath(uri.fsPath);
    // Ignore Strixonomy's own patch/save writes (#293).
    if (isSelfWrite(path)) {
      return;
    }
    const entry = ontologyRegistry.getEntryByPath(path);
    if (!entry && kind === "change") {
      await indexWorkspace().catch(() => undefined);
      return;
    }
    if (!entry) {
      await ontologyRegistry.syncFromCatalog();
      return;
    }

    const inflight = this.pending.get(path);
    if (inflight) {
      await inflight;
      return;
    }

    const decisionPromise = this.promptDecision(entry.dirty, path);
    this.pending.set(path, decisionPromise);
    const choice = await decisionPromise.finally(() => {
      this.pending.delete(path);
    });

    if (choice === "reload") {
      await ontologyRegistry.reload(entry.id);
      await indexWorkspace().catch(() => undefined);
      return;
    }
    if (choice === "compare") {
      const disk = await vscode.workspace.openTextDocument(uri);
      const editor = vscode.window.visibleTextEditors.find(
        (e) => normalizeFsPath(e.document.uri.fsPath) === path
      );
      if (editor) {
        await vscode.commands.executeCommand(
          "vscode.diff",
          uri,
          editor.document.uri,
          `${path.split("/").pop()} (disk ↔ buffer)`
        );
      } else {
        await vscode.window.showTextDocument(disk, { preview: true });
      }
      return;
    }
  }

  private async promptDecision(
    dirty: boolean,
    path: string
  ): Promise<ExternalChangeChoice> {
    const base = path.split("/").pop() ?? path;
    if (!dirty) {
      return "reload";
    }
    const picked = await vscode.window.showWarningMessage(
      `Strixonomy: "${base}" changed on disk while your buffer has unsaved edits.`,
      { modal: true },
      "Reload from disk",
      "Keep buffer",
      "Compare"
    );
    if (picked === "Reload from disk") {
      return "reload";
    }
    if (picked === "Compare") {
      return "compare";
    }
    return "keep";
  }

  onDocumentOpened(document: vscode.TextDocument): void {
    if (isOntologyDocument(document)) {
      void ontologyRegistry.syncFromCatalog();
    }
  }

  dispose(): void {
    for (const watcher of this.watchers) {
      watcher.dispose();
    }
    this.watchers = [];
    this.pending.clear();
  }

  resetForTests(): void {
    this.dispose();
  }
}

export const externalChangeRecovery = new ExternalChangeRecovery();
