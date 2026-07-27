import { LanguageClient } from "vscode-languageclient/node";
import {
  CatalogSnapshot,
  GetEntityResult,
  IndexWorkspaceResult,
  ParseManchesterResult,
  TabularQueryResult,
} from "./lsp/protocol";
import type { PanelKind } from "./webviews/messages";
import type { CurrentFocus } from "./focus/types";

export type DialogPanelKind = "newOntology" | "prefixManager";
export type InjectablePanelKind =
  | DialogPanelKind
  | "inspector"
  | "queryWorkbench"
  | "reasoner"
  | "semanticDiff";

/** VS Code integration test hooks (only when ONTOCODE_TEST_FIXTURES is set). */
export interface StrixonomyTestHooks {
  openEntityInspector(iri: string): Promise<void>;
  getInspectorWebviewHtml(): string | undefined;
  assertInspectorHtmlRoutesPanel(): void;
  waitForInspectorReady(timeoutMs?: number): Promise<void>;
  openQueryWorkbench(): Promise<void>;
  getQueryWorkbenchWebviewHtml(): string | undefined;
  assertQueryWorkbenchHtmlRoutesPanel(): void;
  waitForQueryWorkbenchReady(timeoutMs?: number): Promise<void>;
  disposeAllPanels(): Promise<void>;
  assertWebviewHtmlRoutesPanel(html: string, panel: PanelKind): void;
  openEntity(iri: string): Promise<void>;
  getInspectorLoadedIri(): string | undefined;
  getInspectorPanelTitle(): string | undefined;
  waitForInspectorIri(iri: string, timeoutMs?: number): Promise<void>;
  getInspectorPanelRef(): object | undefined;
  getCanonicalFocus(): CurrentFocus | null;
  /** Open New Ontology dialog for `targetPath` without showSaveDialog. */
  openNewOntologyDialog(targetPath: string): Promise<void>;
  /** Open Prefix Manager for an indexed document path without multi-doc pick. */
  openPrefixManager(documentPath: string): Promise<void>;
  /** Inject a webview→host message into the open panel's validated handler. */
  postWebviewMessage(panel: InjectablePanelKind, msg: unknown): Promise<void>;
  waitForPanelReady(
    panel: DialogPanelKind | "reasoner" | "semanticDiff" | "queryWorkbench",
    timeoutMs?: number
  ): Promise<void>;
  getPanelHtml(panel: DialogPanelKind): string | undefined;
  disposePanel(panel: DialogPanelKind): Promise<void>;
  /** Open Reasoner panel (no run). */
  openReasonerPanel(): Promise<void>;
  /** Run reasoner with workspace defaults; resolves when RPC finishes. */
  runReasonerDefaults(): Promise<void>;
  /** Open Semantic Diff for left/right refs (skips input boxes). */
  openSemanticDiff(leftRef: string, rightRef: string): Promise<void>;
  /**
   * Capture the VS Code window via ONTOCODE_CAPTURE_SCRIPT into
   * ONTOCODE_SCREENSHOT_DIR/<name>.png (macOS screenshot pipeline).
   */
  captureScreenshot(name: string): Promise<void>;
  /** Brief UI settle delay for animations / layout. */
  settle(ms?: number): Promise<void>;
  /** Host-owned ontology registry snapshot (v0.20 workspace runtime). */
  getOntologyRegistrySnapshot(): Array<{
    id: string;
    path: string;
    editable: boolean;
    dirty: boolean;
    active: boolean;
  }>;
  getNavigationStack(): Array<{ kind: string; id: string; source: string }>;
}

/** Extension activation API (used by VS Code integration tests). */
export interface StrixonomyApi {
  getClient(): LanguageClient | undefined;
  indexWorkspace(workspaceUri?: string): Promise<IndexWorkspaceResult>;
  getCatalogSnapshot(): Promise<CatalogSnapshot>;
  getEntity(iri: string): Promise<GetEntityResult>;
  runSqlQuery(sql: string): Promise<TabularQueryResult>;
  runSparqlQuery(query: string): Promise<TabularQueryResult>;
  parseManchester(
    expression: string,
    axiomKind: string,
    entityIri?: string,
    documentUri?: string
  ): Promise<ParseManchesterResult>;
  /** Present when ONTOCODE_TEST_FIXTURES is set. */
  __test?: StrixonomyTestHooks;
}
