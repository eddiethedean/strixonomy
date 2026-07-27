import * as vscode from "vscode";
import { focusRelay } from "../focus/focusRelay";
import { getCatalogSnapshot, getWorkspaceUiState } from "../lsp/client";
import type { WorkspaceUiState } from "../lsp/protocol";
import {
  getDirtyOntologyDocumentCount,
  getFocusedEntityIri,
  isOntologyDocument,
} from "./uiState";
import { appendError } from "../logging/errorLog";
import { ontologyRegistry } from "../workspace";

export type CommandHandler = (...args: unknown[]) => unknown;

export interface StrixonomyKeybinding {
  command: string;
  key: string;
  mac?: string;
  when?: string;
}

export interface KeybindingConflict {
  key: string;
  commands: string[];
}

export function detectKeybindingConflicts(
  bindings: readonly StrixonomyKeybinding[]
): KeybindingConflict[] {
  const commandsByKey = new Map<string, Set<string>>();
  for (const binding of bindings) {
    for (const key of [binding.key, binding.mac].filter(
      (value): value is string => Boolean(value)
    )) {
      const normalized = key.trim().toLowerCase().replace(/\s+/g, " ");
      const commands = commandsByKey.get(normalized) ?? new Set<string>();
      commands.add(binding.command);
      commandsByKey.set(normalized, commands);
    }
  }
  return [...commandsByKey]
    .filter(([, commands]) => commands.size > 1)
    .map(([key, commands]) => ({ key, commands: [...commands] }));
}

export class CommandRegistry {
  constructor(private readonly context: vscode.ExtensionContext) {}

  register(id: string, handler: CommandHandler): vscode.Disposable {
    const disposable = vscode.commands.registerCommand(id, handler);
    this.context.subscriptions.push(disposable);
    return disposable;
  }

  async syncContext(state: Partial<WorkspaceUiState>): Promise<void> {
    const reasoning = focusRelay.getReasoning();
    await Promise.all([
      this.setContext("strixonomy:hasOntology", state.has_ontology ?? false),
      this.setContext(
        "strixonomy:isDirty",
        state.is_dirty ?? getDirtyOntologyDocumentCount() > 0
      ),
      this.setContext("strixonomy:hasSelection", state.has_selection ?? false),
      this.setContext(
        "strixonomy:reasonerRunning",
        reasoning?.running ?? state.reasoner_running ?? false
      ),
      this.setContext(
        "strixonomy:canEditSelection",
        state.selection_editable ?? false
      ),
    ]);
  }

  startContextSync(context: vscode.ExtensionContext = this.context): vscode.Disposable {
    let disposed = false;
    let generation = 0;
    const refresh = async (): Promise<void> => {
      const currentGeneration = ++generation;
      const selectionIri = getFocusedEntityIri();
      const dirtyCount = Math.max(
        getDirtyOntologyDocumentCount(),
        ontologyRegistry.getDirtyCount()
      );
      const registrySnapshot = ontologyRegistry.getSnapshot();
      try {
        const state = await getWorkspaceUiState({
          selection_iri: selectionIri,
          dirty_document_count: dirtyCount,
          active_ontology_id:
            ontologyRegistry.getActiveId() ??
            context.workspaceState.get<string>("strixonomy.activeOntology"),
          ontology_registry: registrySnapshot,
        });
        if (!disposed && currentGeneration === generation) {
          await this.syncContext(state);
        }
      } catch (error) {
        try {
          const snapshot = await getCatalogSnapshot();
          if (!disposed && currentGeneration === generation) {
            await this.syncContext({
              has_ontology: snapshot.documents.length > 0,
              is_dirty: dirtyCount > 0,
              has_selection: selectionIri !== undefined,
              selection_editable: selectionIri
                ? snapshot.entities.some((entity) => entity.iri === selectionIri)
                : false,
            });
          }
        } catch (fallbackError) {
          appendError(fallbackError, "context sync");
          await this.syncContext({
            has_ontology: false,
            is_dirty: dirtyCount > 0,
            has_selection: selectionIri !== undefined,
            selection_editable: false,
          });
        }
      }
    };

    const focusSubscription = focusRelay.subscribe(() => void refresh());
    const changeSubscription = vscode.workspace.onDidChangeTextDocument((event) => {
      if (isOntologyDocument(event.document)) {
        void refresh();
      }
    });
    const saveSubscription = vscode.workspace.onDidSaveTextDocument((document) => {
      if (isOntologyDocument(document)) {
        void refresh();
      }
    });
    const timer = setInterval(() => void refresh(), 15_000);
    void refresh();

    const disposable = new vscode.Disposable(() => {
      disposed = true;
      clearInterval(timer);
      focusSubscription();
      changeSubscription.dispose();
      saveSubscription.dispose();
    });
    context.subscriptions.push(disposable);
    return disposable;
  }

  private async setContext(key: string, value: boolean): Promise<void> {
    await vscode.commands.executeCommand("setContext", key, value);
  }
}
