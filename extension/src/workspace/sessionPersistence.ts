import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { isOntologyDocument } from "../commands/uiState";
import { appendError } from "../logging/errorLog";
import {
  filterWorkspaceFileUris,
  isUriInWorkspace,
} from "../utils/workspacePath";
import { isNotIndexedError } from "../utils/lspErrors";
import { focusRelay } from "../focus/focusRelay";
import type { PanelRestoreState } from "../webviews/layoutPersistenceLogic";
import { isAllowedPanelRestoreCommand } from "../webviews/layoutPersistenceLogic";
import { getRememberedPanelRestoreState } from "../webviews/layoutPersistence";
import { ontologyRegistry } from "./ontologyRegistry";
import { navigationManager } from "./navigationManager";
import { selectionManager } from "./selectionManager";
import type { WorkspaceSessionSnapshot } from "./types";

export const SESSION_KEY = "strixonomy.workspaceSession";
const SESSION_FILE = ".strixonomy/session.json";
const MAX_PANEL_RESTORE = 12;
const NOT_INDEXED_RETRIES = 8;
const NOT_INDEXED_DELAY_MS = 250;

export class WorkspaceSessionPersistence {
  private context: vscode.ExtensionContext | undefined;
  private deferredSnapshot: WorkspaceSessionSnapshot | undefined;
  private deferredReopenPanels = true;
  private catalogRestoreScheduled = false;

  bindContext(context: vscode.ExtensionContext): void {
    this.context = context;
  }

  async capture(): Promise<WorkspaceSessionSnapshot> {
    const focus = focusRelay.getFocus();
    const panelRestore: WorkspaceSessionSnapshot["panelRestore"] = {};
    if (this.context) {
      const viewTypes = [
        "strixonomyInspector",
        "strixonomyGraph",
        "strixonomyQueryWorkbench",
        "strixonomyImports",
        "strixonomyReasoner",
        "strixonomyExplanation",
        "strixonomySemanticDiff",
        "strixonomyManchesterEditor",
        "strixonomyRuleBrowser",
        "strixonomyRuleEditor",
      ];
      for (const viewType of viewTypes) {
        // Only panels that were explicitly opened (remembered). Never inject
        // DEFAULT_REOPEN — that would restart reasoner / interactive diff on restore.
        const state = getRememberedPanelRestoreState(this.context, viewType);
        if (state?.command && isAllowedPanelRestoreCommand(state.command)) {
          panelRestore[viewType] = state;
        }
      }
    }
    return {
      // Persist open ontology editors/tabs — not the full catalog (#295).
      openOntologyUris: this.captureOpenOntologyUris(),
      activeOntologyId: ontologyRegistry.getActiveId(),
      focus: focus
        ? { kind: focus.kind, id: focus.id, source: focus.source }
        : undefined,
      navigation: navigationManager.getStack(),
      navigationIndex: navigationManager.getIndex(),
      panelRestore,
    };
  }

  /** Visible/open ontology editor URIs inside the workspace. */
  captureOpenOntologyUris(): string[] {
    const fromDocuments = vscode.workspace.textDocuments
      .filter((doc) => isOntologyDocument(doc) && isUriInWorkspace(doc.uri))
      .map((doc) => doc.uri.toString());

    const fromTabs: string[] = [];
    for (const group of vscode.window.tabGroups.all) {
      for (const tab of group.tabs) {
        const input = tab.input as { uri?: vscode.Uri } | undefined;
        const uri = input?.uri;
        if (!uri || uri.scheme !== "file" || !isUriInWorkspace(uri)) {
          continue;
        }
        if (!ONTOLOGY_PATH.test(uri.fsPath)) {
          continue;
        }
        fromTabs.push(uri.toString());
      }
    }

    return [...new Set([...fromDocuments, ...fromTabs])];
  }

  async persist(): Promise<void> {
    if (!this.context) {
      return;
    }
    const snapshot = await this.capture();
    await this.context.workspaceState.update(SESSION_KEY, snapshot);
    await this.writeSessionFile(snapshot);
  }

  async restore(options?: { reopenPanels?: boolean }): Promise<void> {
    if (!this.context) {
      return;
    }
    if (!vscode.workspace.workspaceFolders?.length) {
      return;
    }
    const snapshot =
      this.context.workspaceState.get<WorkspaceSessionSnapshot>(SESSION_KEY) ??
      (await this.readSessionFile());
    if (!snapshot) {
      return;
    }

    const openUris = filterWorkspaceFileUris(snapshot.openOntologyUris ?? []);
    ontologyRegistry.hydrateOpenUris(openUris);
    for (const uri of openUris) {
      try {
        await vscode.workspace.openTextDocument(vscode.Uri.parse(uri));
      } catch {
        // File may have moved; registry sync will reconcile.
      }
    }

    const catalogReady = await this.syncCatalogWithRetry();
    if (!catalogReady) {
      // Finish focus/nav now; retry active ontology + panels once the index settles (#294).
      this.deferredSnapshot = snapshot;
      this.deferredReopenPanels = options?.reopenPanels !== false;
      this.scheduleDeferredCatalogRestore();
    } else if (snapshot.activeOntologyId) {
      try {
        await ontologyRegistry.activate(snapshot.activeOntologyId);
      } catch (err) {
        appendError(
          `Session restore activate failed: ${err instanceof Error ? err.message : String(err)}`,
          "workspace"
        );
      }
    }

    if (snapshot.focus?.id) {
      selectionManager.hydrate(snapshot.focus.id);
    }

    if (snapshot.navigation?.length) {
      navigationManager.hydrate(
        snapshot.navigation,
        snapshot.navigationIndex ?? snapshot.navigation.length - 1
      );
    }

    if (options?.reopenPanels !== false && snapshot.panelRestore && catalogReady) {
      await this.reopenPanels(snapshot.panelRestore);
    }
  }

  /** Called after a successful catalog sync so cold-start restore can finish (#294). */
  async completeDeferredCatalogRestore(): Promise<void> {
    const snapshot = this.deferredSnapshot;
    if (!snapshot) {
      return;
    }
    const reopenPanels = this.deferredReopenPanels;
    this.deferredSnapshot = undefined;
    try {
      await ontologyRegistry.syncFromCatalog();
      if (snapshot.activeOntologyId) {
        await ontologyRegistry.activate(snapshot.activeOntologyId);
      }
      if (reopenPanels && snapshot.panelRestore) {
        await this.reopenPanels(snapshot.panelRestore);
      }
    } catch (err) {
      if (isNotIndexedError(err)) {
        this.deferredSnapshot = snapshot;
        this.deferredReopenPanels = reopenPanels;
        return;
      }
      appendError(
        `Deferred session restore failed: ${err instanceof Error ? err.message : String(err)}`,
        "workspace"
      );
    }
  }

  async reopenPanels(
    panelRestore: Record<string, PanelRestoreState>
  ): Promise<void> {
    // Do not execute restore commands from session.json in untrusted workspaces (#309).
    if (!vscode.workspace.isTrusted) {
      return;
    }
    const entries = Object.entries(panelRestore).slice(0, MAX_PANEL_RESTORE);
    for (const [, state] of entries) {
      if (!state.command || !isAllowedPanelRestoreCommand(state.command)) {
        continue;
      }
      try {
        await vscode.commands.executeCommand(state.command, ...(state.args ?? []));
      } catch {
        // Panel restore is best-effort.
      }
    }
  }

  private scheduleDeferredCatalogRestore(): void {
    if (this.catalogRestoreScheduled) {
      return;
    }
    this.catalogRestoreScheduled = true;
    void (async () => {
      for (let attempt = 0; attempt < NOT_INDEXED_RETRIES; attempt++) {
        await delay(NOT_INDEXED_DELAY_MS * (attempt + 1));
        if (!this.deferredSnapshot) {
          this.catalogRestoreScheduled = false;
          return;
        }
        try {
          await ontologyRegistry.syncFromCatalog();
          await this.completeDeferredCatalogRestore();
          // completeDeferredCatalogRestore may re-defer on NOT_INDEXED — keep retrying.
          if (!this.deferredSnapshot) {
            this.catalogRestoreScheduled = false;
            return;
          }
        } catch (err) {
          if (!isNotIndexedError(err)) {
            appendError(
              `Catalog sync during deferred restore failed: ${
                err instanceof Error ? err.message : String(err)
              }`,
              "workspace"
            );
            this.deferredSnapshot = undefined;
            this.catalogRestoreScheduled = false;
            return;
          }
        }
      }
      this.catalogRestoreScheduled = false;
    })();
  }

  private async syncCatalogWithRetry(): Promise<boolean> {
    for (let attempt = 0; attempt < NOT_INDEXED_RETRIES; attempt++) {
      try {
        await ontologyRegistry.syncFromCatalog();
        return true;
      } catch (err) {
        if (!isNotIndexedError(err)) {
          appendError(
            `Session catalog sync failed: ${
              err instanceof Error ? err.message : String(err)
            }`,
            "workspace"
          );
          return false;
        }
        await delay(NOT_INDEXED_DELAY_MS * (attempt + 1));
      }
    }
    return false;
  }

  private async writeSessionFile(snapshot: WorkspaceSessionSnapshot): Promise<void> {
    const folder = vscode.workspace.workspaceFolders?.[0];
    if (!folder) {
      return;
    }
    const dir = path.join(folder.uri.fsPath, ".strixonomy");
    const file = path.join(dir, "session.json");
    try {
      await fs.promises.mkdir(dir, { recursive: true });
      await fs.promises.writeFile(file, JSON.stringify(snapshot, null, 2), "utf8");
    } catch {
      // File session is optional for multi-root sharing.
    }
  }

  private async readSessionFile(): Promise<WorkspaceSessionSnapshot | undefined> {
    const folder = vscode.workspace.workspaceFolders?.[0];
    if (!folder) {
      return undefined;
    }
    const file = path.join(folder.uri.fsPath, ".strixonomy", "session.json");
    try {
      const raw = await fs.promises.readFile(file, "utf8");
      return JSON.parse(raw) as WorkspaceSessionSnapshot;
    } catch {
      return undefined;
    }
  }

  resetForTests(): void {
    this.context = undefined;
    this.deferredSnapshot = undefined;
    this.deferredReopenPanels = true;
    this.catalogRestoreScheduled = false;
  }
}

const ONTOLOGY_PATH = /\.(ttl|owl|rdf|jsonld|json-ld|nt|nq|trig|obo)$/i;

function delay(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export const workspaceSessionPersistence = new WorkspaceSessionPersistence();
