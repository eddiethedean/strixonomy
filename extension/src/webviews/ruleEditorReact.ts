import * as vscode from "vscode";
import { applyAxiomPatch, getCatalogSnapshot, validateSwrlRule } from "../lsp/client";
import {
  hasPatchFailureDiagnostics,
  isPatchFullySynced,
  patchFailureMessage,
  patchSyncCancelledMessage,
} from "../lsp/patchFeedback";
import {
  documentUriInWorkspace,
  resolveWorkspaceDocumentUri,
  WORKSPACE_DOCUMENT_OUTSIDE_MESSAGE,
} from "../utils/workspacePath";
import { workspaceTransactionManager } from "../workspace";
import { forgetPanelRestoreState, rememberPanelRestoreState } from "./layoutPersistence";
import type { WebviewMessage } from "./messages";
import { PanelHost } from "./panelHost";

const DEFAULT_RULE_JSON = `{
  "body": [
    { "kind": "class", "class": "http://example.org/swrl#Person", "arg": { "variable": "x" } }
  ],
  "head": [
    { "kind": "class", "class": "http://example.org/swrl#Human", "arg": { "variable": "x" } }
  ],
  "enabled": true
}`;

export interface RuleEditorOptions {
  ruleJson?: string;
  /** When set, Apply uses replace_swrl_rule instead of add_swrl_rule. */
  originalRuleJson?: string;
  documentUri?: string;
  ontologyIri?: string;
}

export class RuleEditorPanel {
  public static current: RuleEditorPanel | undefined;
  private options: RuleEditorOptions;
  private activeValidateSeq = 0;

  private constructor(
    private readonly host: PanelHost,
    options: RuleEditorOptions
  ) {
    this.options = options;
    this.host.panel.onDidDispose(() => {
      void forgetPanelRestoreState("strixonomyRuleEditor");
      RuleEditorPanel.current = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static async show(
    extensionUri: vscode.Uri,
    options: RuleEditorOptions = {}
  ): Promise<RuleEditorPanel> {
    const resolved = await resolveRuleEditorOptions(options);
    if (RuleEditorPanel.current) {
      RuleEditorPanel.current.options = resolved;
      RuleEditorPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      await RuleEditorPanel.current.bootstrap();
      RuleEditorPanel.current.persistRestoreState();
      return RuleEditorPanel.current;
    }
    const host = PanelHost.create(extensionUri, {
      viewType: "strixonomyRuleEditor",
      title: "SWRL Rule Editor",
      panel: "ruleEditor",
      onMessage: async (message: WebviewMessage) => {
        const panel = RuleEditorPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });
    const instance = new RuleEditorPanel(host, resolved);
    RuleEditorPanel.current = instance;
    instance.persistRestoreState();
    await instance.bootstrap();
    return instance;
  }

  private persistRestoreState(): void {
    void rememberPanelRestoreState("strixonomyRuleEditor", {
      command: "strixonomy.openRuleEditor",
      args: [this.options],
      title: "SWRL Rule Editor",
    });
  }

  private async bootstrap(): Promise<void> {
    this.host.postMessage({
      type: "swrlRuleInit",
      ruleJson: this.options.ruleJson ?? DEFAULT_RULE_JSON,
      documentUri: this.options.documentUri ?? "",
      ontologyIri: this.options.ontologyIri ?? "",
    });
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "validateSwrlRule") {
      await this.validate(message.ruleJson, message.seq);
      return;
    }
    if (message.type === "applySwrlRule") {
      await this.apply(message.ruleJson, message.previewOnly);
    }
  }

  private async validate(ruleJson: string, seq: number): Promise<void> {
    this.activeValidateSeq = seq;
    try {
      const result = await validateSwrlRule({ rule_json: ruleJson });
      if (seq !== this.activeValidateSeq) {
        return;
      }
      this.host.postMessage({
        type: "swrlRuleValidation",
        seq,
        diagnostics: result.diagnostics,
      });
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      if (seq !== this.activeValidateSeq) {
        return;
      }
      this.host.postMessage({ type: "swrlRuleValidation", seq, error: msg });
    }
  }

  private async apply(ruleJson: string, previewOnly: boolean): Promise<void> {
    const documentUri = resolveWorkspaceDocumentUri(this.options.documentUri ?? "");
    if (!documentUri) {
      void vscode.window.showErrorMessage(WORKSPACE_DOCUMENT_OUTSIDE_MESSAGE);
      return;
    }
    const ontologyIri = this.options.ontologyIri;
    if (!ontologyIri) {
      void vscode.window.showErrorMessage(
        "Strixonomy: SWRL apply requires an ontology IRI"
      );
      return;
    }

    // Host-side re-validate before apply (don't trust a stale webview state).
    try {
      const check = await validateSwrlRule({ rule_json: ruleJson });
      const hard = (check.diagnostics ?? []).filter((d) => d.severity === "error");
      if (hard.length > 0) {
        void vscode.window.showErrorMessage(
          `Strixonomy: SWRL rule has ${hard.length} validation error(s)`
        );
        return;
      }
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      void vscode.window.showErrorMessage(`Strixonomy: ${message}`);
      return;
    }

    const original = this.options.originalRuleJson;
    const isEdit = Boolean(original && original.trim() && original !== ruleJson);
    const patches = isEdit
      ? [
          {
            op: "replace_swrl_rule",
            ontology_iri: ontologyIri,
            old_rule_json: original as string,
            new_rule_json: ruleJson,
          },
        ]
      : original && original.trim() && original === ruleJson
        ? [] // no-op edit
        : [
            {
              op: "add_swrl_rule",
              ontology_iri: ontologyIri,
              rule_json: ruleJson,
            },
          ];

    if (patches.length === 0) {
      if (!previewOnly) {
        void vscode.window.showInformationMessage("Strixonomy: SWRL rule unchanged");
      }
      return;
    }

    try {
      const result = previewOnly
        ? await applyAxiomPatch({
            document_uri: documentUri,
            patches,
            preview_only: true,
          })
        : await workspaceTransactionManager.apply(
            documentUri,
            vscode.Uri.parse(documentUri).fsPath,
            patches,
            isEdit ? "Replace SWRL rule" : "Add SWRL rule"
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
      this.options.ruleJson = ruleJson;
      this.options.originalRuleJson = ruleJson;
      this.persistRestoreState();
      if (isPatchFullySynced(result)) {
        void vscode.window.showInformationMessage(
          isEdit ? "Strixonomy: SWRL rule updated" : "Strixonomy: SWRL rule applied"
        );
      } else {
        void vscode.window.showWarningMessage(patchSyncCancelledMessage());
      }
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      void vscode.window.showErrorMessage(`Strixonomy: ${message}`);
    }
  }
}

async function resolveRuleEditorOptions(
  options: RuleEditorOptions
): Promise<RuleEditorOptions> {
  let documentUri = options.documentUri
    ? resolveWorkspaceDocumentUri(options.documentUri) ?? options.documentUri
    : undefined;
  let ontologyIri = options.ontologyIri;

  if (!documentUri || !ontologyIri) {
    try {
      const snapshot = await getCatalogSnapshot();
      const ttl = snapshot.documents.find((d) =>
        d.path.toLowerCase().endsWith(".ttl")
      );
      if (!documentUri && ttl) {
        documentUri =
          documentUriInWorkspace(ttl.path) ??
          vscode.Uri.file(ttl.path).toString();
      }
      if (!ontologyIri && ttl?.base_iri) {
        ontologyIri = ttl.base_iri;
      }
    } catch {
      // optional
    }
  }

  const ruleJson = options.ruleJson ?? DEFAULT_RULE_JSON;
  return {
    ruleJson,
    // Only treat as edit when caller explicitly marked an on-disk original.
    originalRuleJson: options.originalRuleJson,
    documentUri,
    ontologyIri,
  };
}
