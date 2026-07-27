import * as vscode from "vscode";
import { applyAxiomPatch, getEntity, listPlugins } from "../lsp/client";
import {
  hasPatchFailureDiagnostics,
  isPatchFullySynced,
  patchFailureMessage,
} from "../lsp/patchFeedback";
import { EntityDetail } from "../lsp/protocol";
import { entityKindLabel } from "../utils/iri";
import { documentUriInWorkspace, WORKSPACE_DOCUMENT_OUTSIDE_MESSAGE } from "../utils/workspacePath";
import { workspaceTransactionManager } from "../workspace/transactionManager";
import { PanelHost } from "./panelHost";
import type { EntityDetailPayload, PatchOp, WebviewMessage } from "./messages";
import { GraphPanel } from "./graphPanel";
import { acceptInspectorRevealRequest } from "./inspectorReveal";
import { forgetPanelRestoreState, rememberPanelRestoreState } from "./layoutPersistence";

type RefreshFn = () => Promise<void>;

function toPayload(
  detail: EntityDetail,
  documentInWorkspace: boolean
): EntityDetailPayload {
  return {
    entity: {
      iri: detail.entity.iri,
      short_name: detail.entity.short_name,
      kind: detail.entity.kind,
      labels: detail.entity.labels,
      comments: detail.entity.comments,
      deprecated: detail.entity.deprecated,
      obo_id: (detail.entity as { obo_id?: string }).obo_id,
    },
    parents: detail.parents,
    children: detail.children,
    axioms: detail.axioms,
    annotations: detail.annotations,
    characteristics: detail.characteristics,
    editable: documentInWorkspace && detail.editable,
    document_path: detail.document_path,
  };
}

export class EntityInspectorPanel {
  public static currentPanel: EntityInspectorPanel | undefined;
  private host: PanelHost;
  private readonly extensionUri: vscode.Uri;
  private iri: string | undefined;
  private oboId: string | undefined;
  private documentUri: string | undefined;
  private documentPath: string | undefined;
  private classOptions: string[] = [];
  private objectPropertyOptions: string[] = [];
  private activeRequestId = 0;
  private applying = false;

  private constructor(
    host: PanelHost,
    extensionUri: vscode.Uri,
    private readonly onRefresh?: RefreshFn
  ) {
    this.host = host;
    this.extensionUri = extensionUri;
    host.panel.onDidDispose(() => {
      void forgetPanelRestoreState("strixonomyInspector");
      EntityInspectorPanel.currentPanel = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static show(
    extensionUri: vscode.Uri,
    detail: EntityDetail,
    classOptions: string[] = [],
    objectPropertyOptions: string[] = [],
    onRefresh?: RefreshFn,
    requestId?: number
  ): EntityInspectorPanel {
    void rememberPanelRestoreState("strixonomyInspector", {
      command: "strixonomy.openEntity",
      args: [detail.entity.iri],
      title: panelTitle(detail),
    });
    if (EntityInspectorPanel.currentPanel) {
      const existing = EntityInspectorPanel.currentPanel;
      if (!existing.isWebviewReady()) {
        existing.disposeForTests();
        EntityInspectorPanel.currentPanel = undefined;
      } else {
        existing.reveal(detail, classOptions, objectPropertyOptions, requestId);
        existing.host.panel.reveal();
        return existing;
      }
    }

    const host = PanelHost.create(extensionUri, {
      viewType: "strixonomyInspector",
      title: panelTitle(detail),
      panel: "inspector",
      onMessage: async (message: WebviewMessage) => {
        const panel = EntityInspectorPanel.currentPanel;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });

    const instance = new EntityInspectorPanel(host, extensionUri, onRefresh);
    EntityInspectorPanel.currentPanel = instance;
    instance.reveal(detail, classOptions, objectPropertyOptions, requestId);
    return instance;
  }

  private reveal(
    detail: EntityDetail,
    classOptions: string[] = [],
    objectPropertyOptions: string[] = [],
    requestId?: number
  ): void {
    if (!acceptInspectorRevealRequest(this.activeRequestId, requestId)) {
      return;
    }
    if (requestId !== undefined) {
      this.activeRequestId = requestId;
    }
    this.iri = detail.entity.iri;
    this.oboId = detail.entity.obo_id;
    this.classOptions = classOptions;
    this.objectPropertyOptions = objectPropertyOptions;
    this.documentPath = detail.document_path;
    this.documentUri = detail.document_path
      ? documentUriInWorkspace(detail.document_path)
      : undefined;
    this.host.panel.title = panelTitle(detail);
    this.host.postMessage({
      type: "loadEntity",
      detail: toPayload(detail, this.documentUri !== undefined),
      classOptions,
      objectPropertyOptions,
    });
    void this.postPluginsLoaded();
  }

  private async postPluginsLoaded(): Promise<void> {
    try {
      const { plugins } = await listPlugins();
      this.host.postMessage({
        type: "pluginsLoaded",
        plugins: plugins.map((p) => ({
          id: p.id,
          name: p.name,
          version: p.version,
          kind: p.kind,
          inspector_cards: p.ui.inspector_cards.map((c) => ({
            id: c.id,
            title: c.title,
            applies_to: c.applies_to ?? [],
            command: c.command,
          })),
        })),
      });
    } catch {
      // Plugins are optional when workspace is not indexed.
    }
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "jumpToSource" && this.iri) {
      await vscode.commands.executeCommand("strixonomy.jumpToSource", this.iri);
    }
    if (message.type === "applyPatch") {
      if (!this.documentUri) {
        this.postPatchError(WORKSPACE_DOCUMENT_OUTSIDE_MESSAGE);
        return;
      }
      const { parseApplyPatchMessage } = await import("./messages");
      const parsed = parseApplyPatchMessage(message, this.iri, this.oboId);
      if (!parsed) {
        void vscode.window.showErrorMessage(
          "Strixonomy: ignored invalid applyPatch message from webview"
        );
        return;
      }
      await this.runPatch(parsed.patches, parsed.previewOnly);
    }
    if (message.type === "openManchester" && this.iri && this.documentUri) {
      const axiomKind = message.axiom.kind;
      const initialExpression =
        axiomKind === "disjoint_class"
          ? (message.axiom.other_iri ?? message.axiom.manchester ?? "")
          : (message.axiom.manchester ?? "");
      await vscode.commands.executeCommand("strixonomy.openManchesterEditor", {
        iri: this.iri,
        documentUri: this.documentUri,
        axiomKind,
        initialExpression,
        mode: initialExpression ? "edit" : "add",
      });
    }
    if (message.type === "addManchesterAxiom" && this.iri && this.documentUri) {
      await vscode.commands.executeCommand("strixonomy.openManchesterEditor", {
        iri: this.iri,
        documentUri: this.documentUri,
        mode: "add",
      });
    }
    if (message.type === "openGraph") {
      await GraphPanel.show(this.extensionUri, {
        graphKind: "neighborhood",
        rootIri: message.rootIri ?? this.iri,
      });
    }
    if (message.type === "selectNode" || message.type === "openEntity") {
      await vscode.commands.executeCommand("strixonomy.openEntity", message.iri);
    }
    if (message.type === "findUsages" && this.iri) {
      const { showEntityUsages } = await import("./refactorPreview");
      await showEntityUsages(this.iri);
    }
    if (message.type === "renameIri" && this.iri) {
      const { renameEntityIri } = await import("./refactorPreview");
      const fromIri = this.iri;
      await renameEntityIri(this.extensionUri, fromIri, async (newIri) => {
        if (newIri) {
          await vscode.commands.executeCommand("strixonomy.openEntity", newIri);
        } else if (fromIri) {
          await this.loadEntity(fromIri);
        }
      });
    }
  }

  private async loadEntity(iri: string): Promise<void> {
    const iriAtStart = this.iri;
    const requestId = ++this.activeRequestId;
    const { detail } = await getEntity(iri);
    if (iriAtStart !== this.iri) {
      return;
    }
    this.reveal(detail, this.classOptions, this.objectPropertyOptions, requestId);
  }

  private postPatchError(message: string): void {
    this.host.postMessage({ type: "error", message });
    void vscode.window.showErrorMessage(message);
  }

  private async runPatch(
    patches: PatchOp[],
    previewOnly: boolean
  ): Promise<void> {
    if (this.applying) {
      return;
    }
    if (!this.documentUri) {
      this.postPatchError(WORKSPACE_DOCUMENT_OUTSIDE_MESSAGE);
      return;
    }
    this.applying = true;
    const iriAtStart = this.iri;
    const requestId = ++this.activeRequestId;
    try {
      const result = previewOnly
        ? await applyAxiomPatch({
            document_uri: this.documentUri,
            patches,
            preview_only: true,
          })
        : await workspaceTransactionManager.apply(
            this.documentUri,
            this.documentPath ?? vscode.Uri.parse(this.documentUri).fsPath,
            patches,
            "Inspector apply"
          );
      if (iriAtStart !== this.iri) {
        return;
      }
      if (previewOnly) {
        if (hasPatchFailureDiagnostics(result)) {
          this.postPatchError(patchFailureMessage(result));
          return;
        }
        if (result.preview_text) {
          this.host.postMessage({ type: "preview", text: result.preview_text });
        }
        return;
      }
      if (!result.applied) {
        this.postPatchError(patchFailureMessage(result));
        return;
      }
      // Disk write succeeded — refresh inspector even if editor sync was cancelled (#119).
      const deleted = patches.some((p) => p.op === "delete_entity");
      if (deleted) {
        this.host.panel.dispose();
        EntityInspectorPanel.currentPanel = undefined;
        if (this.onRefresh) {
          await this.onRefresh();
        }
        void vscode.window.showInformationMessage("Strixonomy: entity deleted");
        return;
      }
      if (result.reindex_warning) {
        void vscode.window.showWarningMessage(
          `Strixonomy: changes saved but reindex failed — ${result.reindex_warning}`
        );
      }
      if (result.entity_detail) {
        if (result.entity_detail.entity.iri !== this.iri) {
          return;
        }
        this.reveal(result.entity_detail, this.classOptions, this.objectPropertyOptions, requestId);
      } else if (this.iri) {
        const { detail } = await getEntity(this.iri);
        if (iriAtStart !== this.iri) {
          return;
        }
        this.reveal(detail, this.classOptions, this.objectPropertyOptions, requestId);
      }
      if (this.onRefresh) {
        await this.onRefresh();
      }
      if (isPatchFullySynced(result)) {
        void vscode.window.showInformationMessage("Strixonomy: changes applied");
      }
      // editor_synced:false already warned in applyAxiomPatch client helper
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      this.postPatchError(`Strixonomy: patch failed — ${msg}`);
    } finally {
      this.applying = false;
    }
  }

  isWebviewReady(): boolean {
    return this.host.isWebviewReady();
  }

  /** @internal VS Code integration tests */
  getWebviewHtmlForTests(): string {
    return this.host.getWebviewHtml();
  }

  isWebviewReadyForTests(): boolean {
    return this.host.isWebviewReady();
  }

    disposeForTests(): void {
    if (!this.host.isDisposed) {
      this.host.panel.dispose();
    }
  }

  /** @internal VS Code integration tests */
  getLoadedIriForTests(): string | undefined {
    return this.iri;
  }

  /** @internal VS Code integration tests */
  getPanelTitleForTests(): string {
    return this.host.panel.title;
  }
}

function panelTitle(detail: EntityDetail): string {
  return `${entityKindLabel(detail.entity.kind)}: ${
    detail.entity.labels[0] ?? detail.entity.short_name
  }`;
}
