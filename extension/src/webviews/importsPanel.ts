import * as path from "path";
import * as vscode from "vscode";
import { applyAxiomPatch, getCatalogSnapshot } from "../lsp/client";
import {
  hasPatchFailureDiagnostics,
  isPatchFullySynced,
  patchFailureMessage,
  patchSyncCancelledMessage,
} from "../lsp/patchFeedback";
import { Entity, OntologyDocument } from "../lsp/protocol";
import {
  documentUriInWorkspace,
  WORKSPACE_DOCUMENT_OUTSIDE_MESSAGE,
} from "../utils/workspacePath";
import { workspaceTransactionManager } from "../workspace/transactionManager";
import { forgetPanelRestoreState, rememberPanelRestoreState } from "./layoutPersistence";
import {
  AMBIGUOUS_ONTOLOGY_HEADER_MESSAGE,
  entityBelongsToDocument,
  MISSING_ONTOLOGY_HEADER_MESSAGE,
  resolveOntologyIri,
} from "./importsOntology";
import { PanelHost } from "./panelHost";
import type { ImportsDocumentPayload, PatchOp, WebviewMessage } from "./messages";

type RefreshFn = () => Promise<void>;

const IMPORT_PATCH_OPS = new Set(["add_import", "remove_import"]);

function ontologyErrorForDocument(
  doc: OntologyDocument,
  entities: Entity[],
  selfIri: string | undefined
): string | undefined {
  if (selfIri) {
    return undefined;
  }
  const belonging = entities.filter(
    (e) => e.kind === "ontology" && entityBelongsToDocument(e, doc)
  );
  if (belonging.length > 1) {
    return AMBIGUOUS_ONTOLOGY_HEADER_MESSAGE;
  }
  return MISSING_ONTOLOGY_HEADER_MESSAGE;
}

function buildPayload(
  doc: OntologyDocument,
  allDocs: OntologyDocument[],
  entities: Entity[]
): ImportsDocumentPayload {
  const selfIri = resolveOntologyIri(doc, entities);
  const imported = new Set(doc.imports ?? []);
  const options = allDocs
    .filter((d) => d.format === "turtle" && d.path !== doc.path)
    .map((d) => {
      const iri = resolveOntologyIri(d, entities);
      if (!iri) {
        return undefined;
      }
      return {
        iri,
        path: d.path,
        label: path.basename(d.path),
      };
    })
    .filter((o): o is NonNullable<typeof o> => o !== undefined)
    .filter((o) => o.iri !== selfIri && !imported.has(o.iri));

  return {
    path: doc.path,
    ontology_iri: selfIri,
    imports_editable: selfIri !== undefined,
    error: ontologyErrorForDocument(doc, entities, selfIri),
    imports: doc.imports ?? [],
    options,
  };
}

function errorPayload(filePath: string, message: string): ImportsDocumentPayload {
  return {
    path: filePath,
    imports_editable: false,
    error: message,
    imports: [],
    options: [],
  };
}

export class ImportsPanel {
  public static current: ImportsPanel | undefined;

  private documentUri: string | undefined;
  private ontologyIri: string | undefined;
  private importsEditable = false;
  private filePath: string | undefined;
  private loadGeneration = 0;

  private constructor(
    private readonly host: PanelHost,
    private readonly onRefresh?: RefreshFn
  ) {
    host.panel.onDidDispose(() => {
      void forgetPanelRestoreState("strixonomyImports");
      ImportsPanel.current = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static async show(
    extensionUri: vscode.Uri,
    filePath: string,
    onRefresh?: RefreshFn
  ): Promise<ImportsPanel> {
    void rememberPanelRestoreState("strixonomyImports", {
      command: "strixonomy.manageImports",
      args: [filePath],
      title: `Imports: ${path.basename(filePath)}`,
    });
    if (ImportsPanel.current) {
      ImportsPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      await ImportsPanel.current.load(filePath);
      return ImportsPanel.current;
    }

    const host = PanelHost.create(extensionUri, {
      viewType: "strixonomyImports",
      title: "Manage Imports",
      panel: "imports",
      onMessage: async (message: WebviewMessage) => {
        const panel = ImportsPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });

    const instance = new ImportsPanel(host, onRefresh);
    ImportsPanel.current = instance;
    await instance.load(filePath);
    return instance;
  }

  public async refresh(): Promise<void> {
    if (!this.filePath) {
      return;
    }
    await this.load(this.filePath);
  }

  private async load(filePath: string): Promise<void> {
    const generation = ++this.loadGeneration;
    this.filePath = filePath;

    const snapshot = await getCatalogSnapshot();
    if (generation !== this.loadGeneration) {
      return;
    }

    const doc = snapshot.documents.find((d) => d.path === filePath);
    if (!doc) {
      this.clearEditableState();
      void vscode.window.showErrorMessage(
        `Strixonomy: no indexed Turtle document at ${filePath}`
      );
      this.host.postMessage({
        type: "loadImports",
        payload: errorPayload(
          filePath,
          `No indexed Turtle document at ${filePath}`
        ),
      });
      return;
    }
    if (doc.format !== "turtle") {
      this.clearEditableState();
      void vscode.window.showErrorMessage(
        "Strixonomy: imports management is only available for Turtle (.ttl) files"
      );
      this.host.postMessage({
        type: "loadImports",
        payload: errorPayload(
          filePath,
          "Imports management is only available for Turtle (.ttl) files"
        ),
      });
      return;
    }

    const payload = buildPayload(doc, snapshot.documents, snapshot.entities);
    this.documentUri = documentUriInWorkspace(doc.path);
    this.ontologyIri = payload.ontology_iri;
    this.importsEditable =
      payload.imports_editable && this.documentUri !== undefined;

    if (generation !== this.loadGeneration) {
      return;
    }

    this.host.panel.title = `Imports: ${path.basename(doc.path)}`;
    this.host.postMessage({
      type: "loadImports",
      payload: {
        ...payload,
        imports_editable: this.importsEditable,
      },
    });
  }

  private clearEditableState(): void {
    this.documentUri = undefined;
    this.ontologyIri = undefined;
    this.importsEditable = false;
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type !== "applyPatch") {
      return;
    }
    if (!this.documentUri) {
      const msg = WORKSPACE_DOCUMENT_OUTSIDE_MESSAGE;
      this.host.postMessage({ type: "error", message: msg });
      void vscode.window.showErrorMessage(msg);
      return;
    }
    if (!this.ontologyIri || !this.importsEditable) {
      const msg = "Strixonomy: imports are not editable for this document";
      this.host.postMessage({ type: "error", message: msg });
      void vscode.window.showErrorMessage(msg);
      return;
    }
    const { parseApplyPatchMessage } = await import("./messages");
    const parsed = parseApplyPatchMessage(message, undefined);
    if (!parsed) {
      void vscode.window.showErrorMessage(
        "Strixonomy: ignored invalid applyPatch message from imports panel"
      );
      return;
    }
    if (
      !parsed.patches.every(
        (p) =>
          IMPORT_PATCH_OPS.has(p.op) &&
          typeof p.ontology_iri === "string" &&
          p.ontology_iri === this.ontologyIri
      )
    ) {
      void vscode.window.showErrorMessage(
        "Strixonomy: ignored non-import patch from imports panel"
      );
      return;
    }
    await this.runPatch(parsed.patches, parsed.previewOnly);
  }

  private async runPatch(
    patches: PatchOp[],
    previewOnly: boolean
  ): Promise<void> {
    if (!this.documentUri) {
      return;
    }
    try {
      const result = previewOnly
        ? await applyAxiomPatch({
            document_uri: this.documentUri,
            patches,
            preview_only: true,
          })
        : await workspaceTransactionManager.apply(
            this.documentUri,
            this.filePath ?? vscode.Uri.parse(this.documentUri).fsPath,
            patches,
            "Imports apply"
          );
      if (previewOnly) {
        if (hasPatchFailureDiagnostics(result)) {
          void vscode.window.showErrorMessage(patchFailureMessage(result));
          return;
        }
        if (result.preview_text) {
          this.host.postMessage({ type: "preview", text: result.preview_text });
        }
        return;
      }
      if (!result.applied) {
        void vscode.window.showErrorMessage(patchFailureMessage(result));
        return;
      }
      if (result.reindex_warning) {
        void vscode.window.showWarningMessage(
          `Strixonomy: changes saved but reindex failed — ${result.reindex_warning}`
        );
      }
      if (this.onRefresh) {
        await this.onRefresh();
      }
      const snapshot = await getCatalogSnapshot();
      const doc = snapshot.documents.find((d) => d.path === this.filePath);
      if (doc) {
        const payload = buildPayload(doc, snapshot.documents, snapshot.entities);
        this.importsEditable =
          payload.imports_editable && this.documentUri !== undefined;
        this.host.postMessage({
          type: "loadImports",
          payload: {
            ...payload,
            imports_editable: this.importsEditable,
          },
        });
      }
      if (isPatchFullySynced(result)) {
        void vscode.window.showInformationMessage("Strixonomy: imports updated");
      } else {
        void vscode.window.showWarningMessage(patchSyncCancelledMessage());
      }
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      void vscode.window.showErrorMessage(`Strixonomy: import patch failed — ${msg}`);
    }
  }
}
