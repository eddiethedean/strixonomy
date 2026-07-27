import * as vscode from "vscode";
import { indexWorkspace } from "../lsp/client";
import { isOntologyDocument } from "../commands/uiState";
import { normalizeFsPath } from "../utils/pathUnder";
import { focusRelay } from "../focus/focusRelay";
import { workspaceEventBus } from "./eventBus";
import { ontologyRegistry } from "./ontologyRegistry";
import {
  findOpenDocumentForEntry,
  noOpenDocumentSaveOutcome,
} from "./saveCoordinatorLogic";
import { noteSelfWrite } from "./selfWriteGuard";

export interface SaveResult {
  saved: string[];
  failed: Array<{ path: string; error: string }>;
}

export class SaveCoordinator {
  async saveActive(): Promise<SaveResult> {
    const active = ontologyRegistry.getActiveEntry();
    if (!active) {
      const dirty = ontologyRegistry.getSnapshot().filter((e) => e.dirty);
      if (dirty.length === 1) {
        return this.saveEntry(dirty[0]);
      }
      void vscode.window.showWarningMessage(
        "Strixonomy: no active ontology — use Save All or set an active ontology"
      );
      return { saved: [], failed: [] };
    }
    return this.saveEntry(active);
  }

  async saveAll(): Promise<SaveResult> {
    const dirty = ontologyRegistry.getSnapshot().filter((e) => e.dirty);
    const saved: string[] = [];
    const failed: Array<{ path: string; error: string }> = [];
    for (const entry of dirty) {
      const result = await this.saveEntry(entry);
      saved.push(...result.saved);
      failed.push(...result.failed);
    }
    if (saved.length > 0) {
      void vscode.window.showInformationMessage(
        `Strixonomy: saved ${saved.length} ontolog${saved.length === 1 ? "y" : "ies"}`
      );
    }
    return { saved, failed };
  }

  async saveEntry(
    entry: { uri: string; path: string; id: string }
  ): Promise<SaveResult> {
    const match = findOpenDocumentForEntry(
      vscode.workspace.textDocuments.map((doc) => ({
        uriString: doc.uri.toString(),
        fsPath: doc.uri.fsPath,
        isDirty: doc.isDirty,
      })),
      entry.uri,
      entry.path
    );
    const document = match
      ? vscode.workspace.textDocuments.find(
          (doc) =>
            doc.uri.toString() === match.uriString ||
            normalizeFsPath(doc.uri.fsPath) === normalizeFsPath(match.fsPath)
        )
      : undefined;

    if (!document) {
      const outcome = noOpenDocumentSaveOutcome();
      try {
        await indexWorkspace();
        if (outcome.markClean) {
          ontologyRegistry.markClean(entry.id);
          ontologyRegistry.bumpVersion(entry.id);
          workspaceEventBus.publish("OntologySaved", {
            id: entry.id,
            path: entry.path,
          });
          focusRelay.markReasoningDirty();
        }
        // #299: never claim a file was saved when we did not call document.save().
        return {
          saved: outcome.claimSaved ? [entry.path] : [],
          failed: [],
        };
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        return { saved: [], failed: [{ path: entry.path, error: message }] };
      }
    }
    if (!document.isDirty && !ontologyRegistry.isDirty(entry.id)) {
      return { saved: [], failed: [] };
    }
    try {
      noteSelfWrite(document.uri.fsPath);
      await document.save();
      ontologyRegistry.markClean(entry.id);
      ontologyRegistry.bumpVersion(entry.id);
      await indexWorkspace();
      workspaceEventBus.publish("OntologySaved", { id: entry.id, path: entry.path });
      focusRelay.markReasoningDirty();
      return { saved: [entry.path], failed: [] };
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      return { saved: [], failed: [{ path: entry.path, error: message }] };
    }
  }

  async saveDocument(document: vscode.TextDocument): Promise<boolean> {
    if (!isOntologyDocument(document)) {
      await vscode.commands.executeCommand("workbench.action.files.save");
      return true;
    }
    const path = normalizeFsPath(document.uri.fsPath);
    const entry = ontologyRegistry.getEntryByPath(path);
    if (!entry) {
      noteSelfWrite(document.uri.fsPath);
      await document.save();
      return true;
    }
    const result = await this.saveEntry(entry);
    return result.saved.length > 0 || !ontologyRegistry.isDirty(entry.id);
  }
}

export const saveCoordinator = new SaveCoordinator();
