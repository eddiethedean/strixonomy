import * as vscode from "vscode";
import { getCatalogSnapshot, setActiveOntology } from "../lsp/client";
import type { OntologyDocument } from "../lsp/protocol";
import { isOntologyDocument } from "../commands/uiState";
import { documentUriInWorkspace, isUriInWorkspace, resolveWorkspaceDocumentUri } from "../utils/workspacePath";
import { normalizeFsPath, pathIdentityKey, pathsEqual } from "../utils/pathUnder";
import { workspaceEventBus } from "./eventBus";
import {
  entryFromDocument,
  type OntologyRegistryEntry,
  type OntologyRole,
} from "./types";

const ACTIVE_ONTOLOGY_KEY = "strixonomy.activeOntology";
const REGISTRY_VERSION_KEY = "strixonomy.registryVersions";

export class OntologyRegistry {
  private entries = new Map<string, OntologyRegistryEntry>();
  private semanticDirty = new Set<string>();
  private activeId: string | undefined;
  private context: vscode.ExtensionContext | undefined;
  private versionById = new Map<string, number>();

  bindContext(context: vscode.ExtensionContext): void {
    this.context = context;
    this.activeId = context.workspaceState.get<string>(ACTIVE_ONTOLOGY_KEY);
    const savedVersions =
      context.workspaceState.get<Record<string, number>>(REGISTRY_VERSION_KEY) ?? {};
    for (const [id, version] of Object.entries(savedVersions)) {
      this.versionById.set(id, version);
    }
  }

  getSnapshot(): OntologyRegistryEntry[] {
    return [...this.entries.values()].sort((a, b) => a.path.localeCompare(b.path));
  }

  getActiveId(): string | undefined {
    return this.activeId;
  }

  getActiveEntry(): OntologyRegistryEntry | undefined {
    if (!this.activeId) {
      return undefined;
    }
    return this.entries.get(this.activeId);
  }

  getEntry(id: string): OntologyRegistryEntry | undefined {
    return this.entries.get(id);
  }

  getEntryByPath(path: string): OntologyRegistryEntry | undefined {
    const key = pathIdentityKey(path);
    return [...this.entries.values()].find(
      (entry) => pathIdentityKey(entry.path) === key
    );
  }

  getDirtyCount(): number {
    return [...this.entries.values()].filter((entry) => entry.dirty).length;
  }

  isDirty(id: string): boolean {
    return this.entries.get(id)?.dirty ?? false;
  }

  async syncFromCatalog(): Promise<void> {
    const snapshot = await getCatalogSnapshot();
    const documents = snapshot.documents;
    const activeFromServer = snapshot.active_ontology_id ?? this.activeId;
    const next = new Map<string, OntologyRegistryEntry>();

    for (const document of documents) {
      const uri =
        documentUriInWorkspace(document.path) ??
        vscode.Uri.file(document.path).toString();
      const normalizedPath = normalizeFsPath(document.path);
      const pathKey = pathIdentityKey(document.path);
      const bufferDirty = vscode.workspace.textDocuments.some(
        (doc) =>
          isOntologyDocument(doc) &&
          pathsEqual(doc.uri.fsPath, document.path) &&
          doc.isDirty
      );
      const dirty =
        bufferDirty ||
        this.semanticDirty.has(document.id) ||
        this.semanticDirty.has(normalizedPath) ||
        this.semanticDirty.has(pathKey);
      const version = this.versionById.get(document.id) ?? 0;
      const entry = entryFromDocument(document, documents, {
        uri,
        dirty,
        active: document.id === activeFromServer,
        version,
      });
      next.set(document.id, entry);
    }

    for (const doc of vscode.workspace.textDocuments) {
      if (!isOntologyDocument(doc) || doc.isUntitled) {
        continue;
      }
      const path = normalizeFsPath(doc.uri.fsPath);
      if ([...next.values()].some((e) => pathsEqual(e.path, path))) {
        continue;
      }
      const id = path;
      const format = formatFromPath(path);
      const entry: OntologyRegistryEntry = {
        id,
        uri: doc.uri.toString(),
        path,
        format,
        role: "scratch",
        editable: format === "turtle" || format === "obo" || format === "owl" || format === "rdf_xml" || format === "owl_xml",
        dirty: doc.isDirty || this.semanticDirty.has(id),
        version: this.versionById.get(id) ?? 0,
        active: id === activeFromServer,
      };
      next.set(id, entry);
    }

    const previousIds = new Set(this.entries.keys());
    for (const id of next.keys()) {
      if (!previousIds.has(id)) {
        workspaceEventBus.publish("OntologyOpened", next.get(id));
      }
    }
    for (const id of previousIds) {
      if (!next.has(id)) {
        workspaceEventBus.publish("OntologyClosed", { id });
      }
    }

    this.entries = next;
    if (activeFromServer && next.has(activeFromServer)) {
      await this.setActive(activeFromServer, { syncLsp: false, silent: true });
    } else if (this.activeId && !next.has(this.activeId)) {
      this.activeId = undefined;
      await this.persistActive();
    }

    workspaceEventBus.publish("DirtyChanged", { count: this.getDirtyCount() });
  }

  async open(uri: vscode.Uri): Promise<OntologyRegistryEntry | undefined> {
    const document = await vscode.workspace.openTextDocument(uri);
    await vscode.window.showTextDocument(document);
    await this.syncFromCatalog();
    const path = normalizeFsPath(uri.fsPath);
    return (
      this.getEntryByPath(path) ??
      [...this.entries.values()].find((e) => e.uri === uri.toString())
    );
  }

  async close(id: string): Promise<void> {
    const entry = this.entries.get(id);
    if (!entry) {
      return;
    }
    const uri = vscode.Uri.parse(entry.uri);
    const tabs = vscode.window.tabGroups.all
      .flatMap((group) => group.tabs)
      .filter((tab) => {
        const input = tab.input;
        if (input instanceof vscode.TabInputText) {
          return input.uri.toString() === uri.toString();
        }
        return false;
      });
    await vscode.window.tabGroups.close(tabs);
    this.entries.delete(id);
    this.semanticDirty.delete(id);
    this.semanticDirty.delete(normalizeFsPath(entry.path));
    if (this.activeId === id) {
      this.activeId = undefined;
      await this.persistActive();
    }
    workspaceEventBus.publish("OntologyClosed", { id });
  }

  async activate(id: string): Promise<OntologyRegistryEntry | undefined> {
    return this.setActive(id);
  }

  async reload(id: string): Promise<void> {
    const entry = this.entries.get(id);
    if (!entry) {
      return;
    }
    const uri = vscode.Uri.parse(entry.uri);
    const document = vscode.workspace.textDocuments.find(
      (doc) => doc.uri.toString() === uri.toString()
    );
    if (document) {
      const refreshed = await vscode.workspace.openTextDocument(uri);
      const editor = vscode.window.visibleTextEditors.find(
        (e) => e.document.uri.toString() === uri.toString()
      );
      if (editor) {
        await vscode.window.showTextDocument(refreshed, {
          viewColumn: editor.viewColumn,
          preserveFocus: true,
        });
      }
    }
    this.markClean(id);
    this.bumpVersion(id);
    await this.syncFromCatalog();
    workspaceEventBus.publish("OntologyReloaded", { id });
  }

  markDirty(idOrPath: string): void {
    const entry =
      this.entries.get(idOrPath) ?? this.getEntryByPath(idOrPath);
    const key = entry?.id ?? idOrPath;
    this.semanticDirty.add(key);
    if (entry?.path) {
      this.semanticDirty.add(normalizeFsPath(entry.path));
    }
    if (entry) {
      this.entries.set(entry.id, { ...entry, dirty: true });
    }
    workspaceEventBus.publish("DirtyChanged", { count: this.getDirtyCount() });
  }

  markClean(idOrPath: string): void {
    const entry =
      this.entries.get(idOrPath) ?? this.getEntryByPath(idOrPath);
    const key = entry?.id ?? idOrPath;
    this.semanticDirty.delete(key);
    if (entry?.path) {
      this.semanticDirty.delete(normalizeFsPath(entry.path));
    }
    if (entry) {
      const bufferDirty = vscode.workspace.textDocuments.some(
        (doc) =>
          isOntologyDocument(doc) &&
          normalizeFsPath(doc.uri.fsPath) === normalizeFsPath(entry.path) &&
          doc.isDirty
      );
      this.entries.set(entry.id, { ...entry, dirty: bufferDirty });
    }
    workspaceEventBus.publish("DirtyChanged", { count: this.getDirtyCount() });
  }

  onBufferChanged(document: vscode.TextDocument): void {
    if (!isOntologyDocument(document)) {
      return;
    }
    const path = normalizeFsPath(document.uri.fsPath);
    const entry = this.getEntryByPath(path);
    if (!entry) {
      void this.syncFromCatalog();
      return;
    }
    const dirty =
      document.isDirty ||
      this.semanticDirty.has(entry.id) ||
      this.semanticDirty.has(path);
    if (entry.dirty !== dirty) {
      this.entries.set(entry.id, { ...entry, dirty });
      workspaceEventBus.publish("DirtyChanged", { count: this.getDirtyCount() });
    }
  }

  onBufferSaved(document: vscode.TextDocument): void {
    if (!isOntologyDocument(document)) {
      return;
    }
    const path = normalizeFsPath(document.uri.fsPath);
    const entry = this.getEntryByPath(path);
    if (entry) {
      this.markClean(entry.id);
    }
  }

  async resolveEditableDocument(
    preferredId?: string
  ): Promise<OntologyDocument | undefined> {
    await this.syncFromCatalog();
    const snapshot = await getCatalogSnapshot();
    const activeId = preferredId ?? this.activeId;
    if (activeId) {
      const activeEntry = this.entries.get(activeId);
      if (activeEntry?.editable) {
        const doc = snapshot.documents.find((d) => d.id === activeId);
        if (doc) {
          return doc;
        }
      }
    }
    const editable = snapshot.documents.filter((document) => {
      const entry = this.entries.get(document.id);
      return entry?.editable ?? false;
    });
    if (editable.length === 1) {
      return editable[0];
    }
    return undefined;
  }

  assertEditable(idOrPath: string): void {
    const entry =
      this.entries.get(idOrPath) ?? this.getEntryByPath(idOrPath);
    if (!entry) {
      throw new Error("Ontology is not registered in the workspace");
    }
    if (!entry.editable) {
      const reason =
        entry.role === "import"
          ? "imported ontologies are read-only"
          : `${entry.format} is not editable in this release`;
      throw new Error(`Cannot edit ontology: ${reason}`);
    }
  }

  bumpVersion(id: string): void {
    const next = (this.versionById.get(id) ?? 0) + 1;
    this.versionById.set(id, next);
    const entry = this.entries.get(id);
    if (entry) {
      this.entries.set(id, { ...entry, version: next });
    }
    void this.persistVersions();
  }

  hydrateOpenUris(uris: string[]): void {
    for (const uri of uris) {
      let parsed: vscode.Uri;
      try {
        parsed = vscode.Uri.parse(uri);
      } catch {
        continue;
      }
      if (!isUriInWorkspace(parsed)) {
        continue;
      }
      const path = normalizeFsPath(parsed.fsPath);
      if (!this.entries.has(path) && !this.getEntryByPath(path)) {
        const format = formatFromPath(path);
        this.entries.set(path, {
          id: path,
          uri: parsed.toString(),
          path,
          format,
          role: "scratch" as OntologyRole,
          editable: format === "turtle" || format === "obo" || format === "owl" || format === "rdf_xml" || format === "owl_xml",
          dirty: false,
          version: 0,
          active: false,
        });
      }
    }
  }

  setActiveId(id: string | undefined): void {
    this.activeId = id;
    for (const [key, entry] of this.entries) {
      this.entries.set(key, { ...entry, active: key === id });
    }
  }

  resetForTests(): void {
    this.entries.clear();
    this.semanticDirty.clear();
    this.activeId = undefined;
    this.versionById.clear();
    this.context = undefined;
  }

  private async setActive(
    id: string,
    options?: { syncLsp?: boolean; silent?: boolean }
  ): Promise<OntologyRegistryEntry | undefined> {
    const entry = this.entries.get(id);
    if (!entry) {
      return undefined;
    }
    const syncLsp = options?.syncLsp ?? true;
    if (syncLsp) {
      await setActiveOntology({ ontology_id: id });
    }
    this.setActiveId(id);
    await this.persistActive();
    workspaceEventBus.publish("OntologyActivated", entry);
    if (!options?.silent) {
      void vscode.window.showInformationMessage(
        `Strixonomy: active ontology set to ${entry.path.split("/").pop()}`
      );
    }
    return entry;
  }

  private async persistActive(): Promise<void> {
    if (!this.context) {
      return;
    }
    await this.context.workspaceState.update(ACTIVE_ONTOLOGY_KEY, this.activeId);
  }

  private async persistVersions(): Promise<void> {
    if (!this.context) {
      return;
    }
    const record = Object.fromEntries(this.versionById);
    await this.context.workspaceState.update(REGISTRY_VERSION_KEY, record);
  }
}

function formatFromPath(filePath: string): string {
  const ext = filePath.split(".").pop()?.toLowerCase() ?? "";
  const map: Record<string, string> = {
    ttl: "turtle",
    obo: "obo",
    owl: "owl",
    rdf: "rdf_xml",
    owx: "owl_xml",
    jsonld: "jsonld",
    "json-ld": "jsonld",
    nt: "ntriples",
    nq: "nquads",
    trig: "trig",
  };
  return map[ext] ?? ext;
}

export const ontologyRegistry = new OntologyRegistry();

export async function resolveDocumentUriForEntry(
  entry: OntologyRegistryEntry
): Promise<string | undefined> {
  return (
    documentUriInWorkspace(entry.path) ??
    resolveWorkspaceDocumentUri(entry.path) ??
    entry.uri
  );
}
