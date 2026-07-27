import * as vscode from "vscode";
import { applyRefactor, findUsages, previewRefactor } from "../lsp/client";
import { applyLspWorkspaceEdit } from "../lsp/workspaceEdit";
import { RefactorPlan, RefactorRequest } from "../lsp/protocol";
import { PanelHost } from "./panelHost";
import type { WebviewMessage } from "./messages";
import { openWorkspaceTextDocument } from "../utils/workspacePath";
import { byteColToUtf16 } from "../utils/positions";
import { isUsageJumpLineInDocument, usageJumpLineIndex } from "./refactorPreviewLogic";

export async function showEntityUsages(iri: string): Promise<void> {
  const result = await findUsages(iri);
  if (result.usages.length === 0) {
    void vscode.window.showInformationMessage(`No usages found for ${iri}`);
    return;
  }
  const picked = await vscode.window.showQuickPick(
    result.usages.map((u) => ({
      label: `${u.file.split(/[/\\]/).pop() ?? u.file}:${u.line ?? 0}`,
      description: u.kind,
      detail: u.context,
      usage: u,
    })),
    { placeHolder: `Usages of ${iri}` }
  );
  if (!picked) {
    return;
  }
  const doc = await openWorkspaceTextDocument(picked.usage.file);
  if (!doc) {
    return;
  }
  const editor = await vscode.window.showTextDocument(doc, vscode.ViewColumn.One);
  const line = usageJumpLineIndex(picked.usage.line);
  if (!isUsageJumpLineInDocument(line, doc.lineCount)) {
    void vscode.window.showWarningMessage(
      `Strixonomy: usage line ${picked.usage.line} is out of range`
    );
    return;
  }
  const lineText = doc.lineAt(line).text;
  const byteCol = picked.usage.column ?? 0;
  const pos = new vscode.Position(line, byteColToUtf16(lineText, byteCol));
  editor.selection = new vscode.Selection(pos, pos);
  editor.revealRange(new vscode.Range(pos, pos));
}

export async function renameEntityIri(
  extensionUri: vscode.Uri,
  iri?: string,
  onApplied?: (newIri?: string) => Promise<void>
): Promise<void> {
  const from =
    iri ??
    (await vscode.window.showInputBox({
      prompt: "Entity IRI to rename",
    }));
  if (!from) {
    return;
  }
  const to = await vscode.window.showInputBox({
    prompt: "New IRI",
    placeHolder: "http://example.org/ontology#NewName",
  });
  if (!to) {
    return;
  }
  await runRefactorPreview(
    extensionUri,
    {
      kind: "rename_iri",
      from_iri: from,
      to_iri: to,
    },
    async () => {
      if (onApplied) {
        await onApplied(to);
      }
    }
  );
}

export async function migrateNamespace(
  extensionUri: vscode.Uri,
  onApplied?: () => Promise<void>
): Promise<void> {
  const from = await vscode.window.showInputBox({
    prompt: "Old namespace base IRI",
  });
  if (!from) {
    return;
  }
  const to = await vscode.window.showInputBox({
    prompt: "New namespace base IRI",
  });
  if (!to) {
    return;
  }
  await runRefactorPreview(
    extensionUri,
    {
      kind: "migrate_namespace",
      from_base: from,
      to_base: to,
    },
    onApplied
  );
}

export async function moveEntity(
  extensionUri: vscode.Uri,
  iri?: string,
  onApplied?: () => Promise<void>
): Promise<void> {
  const entity =
    iri ??
    (await vscode.window.showInputBox({ prompt: "Entity IRI to move" }));
  if (!entity) {
    return;
  }
  const target = await vscode.window.showOpenDialog({
    canSelectMany: false,
    filters: { Turtle: ["ttl"] },
    openLabel: "Move entity here",
  });
  if (!target?.[0]) {
    return;
  }
  await runRefactorPreview(
    extensionUri,
    {
      kind: "move_entity",
      entity_iri: entity,
      target_file: target[0].fsPath,
    },
    onApplied
  );
}

export async function extractModule(
  extensionUri: vscode.Uri,
  onApplied?: () => Promise<void>
): Promise<void> {
  const entitiesRaw = await vscode.window.showInputBox({
    prompt: "Entity IRIs (comma-separated)",
  });
  if (!entitiesRaw?.trim()) {
    return;
  }
  const output = await vscode.window.showSaveDialog({
    filters: { Turtle: ["ttl"] },
    defaultUri: vscode.Uri.file("module.ttl"),
  });
  if (!output) {
    return;
  }
  await runRefactorPreview(
    extensionUri,
    {
      kind: "extract_module",
      entity_iris: entitiesRaw
        .split(",")
        .map((s) => s.trim())
        .filter(Boolean),
      output_file: output.fsPath,
      leave_stub: false,
    },
    onApplied
  );
}

export class RefactorPreviewPanel {
  public static current: RefactorPreviewPanel | undefined;
  private host: PanelHost;
  private plan: RefactorPlan | undefined;
  private request: RefactorRequest | undefined;
  private onApplied?: () => Promise<void>;
  private applying = false;

  private constructor(host: PanelHost, onApplied?: () => Promise<void>) {
    this.host = host;
    this.onApplied = onApplied;
    host.panel.onDidDispose(() => {
      RefactorPreviewPanel.current = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static async show(
    extensionUri: vscode.Uri,
    plan: RefactorPlan,
    request: RefactorRequest,
    onApplied?: () => Promise<void>
  ): Promise<RefactorPreviewPanel> {
    if (RefactorPreviewPanel.current) {
      RefactorPreviewPanel.current.plan = plan;
      RefactorPreviewPanel.current.request = request;
      RefactorPreviewPanel.current.onApplied = onApplied;
      RefactorPreviewPanel.current.host.postMessage({
        type: "loadRefactorPlan",
        plan,
      });
      RefactorPreviewPanel.current.host.panel.reveal();
      return RefactorPreviewPanel.current;
    }
    const host = PanelHost.create(extensionUri, {
      viewType: "strixonomyRefactorPreview",
      title: "Strixonomy Refactor Preview",
      panel: "refactorPreview",
      onMessage: async (message: WebviewMessage) => {
        const panel = RefactorPreviewPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });
    const instance = new RefactorPreviewPanel(host, onApplied);
    instance.plan = plan;
    instance.request = request;
    RefactorPreviewPanel.current = instance;
    instance.host.postMessage({ type: "loadRefactorPlan", plan });
    return instance;
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "openGraphFromResults") {
      await vscode.commands.executeCommand("strixonomy.openGraphFromResults", {
        graphKind: message.graphKind,
        rootIris: message.rootIris,
        title: message.title,
      });
    }
    if (message.type === "applyRefactor" && this.plan && this.request && !this.applying) {
      this.applying = true;
      const planSnapshot = this.plan;
      const requestSnapshot = this.request;
      try {
        const result = await applyRefactor(planSnapshot, requestSnapshot, false);
        if (result.workspace_edit) {
          const applied = await applyLspWorkspaceEdit(result.workspace_edit, {
            expectChanges: true,
          });
          if (!applied) {
            void vscode.window.showWarningMessage(
              "Strixonomy: refactor wrote to disk but editor sync was cancelled"
            );
            return;
          }
        }
        if (result.reindex_warning) {
          void vscode.window.showWarningMessage(result.reindex_warning);
        }
        void vscode.window.showInformationMessage(
          `Strixonomy: refactor applied to ${result.files_written} file(s)`
        );
        this.host.panel.dispose();
        if (this.onApplied) {
          await this.onApplied();
        }
      } catch (err) {
        const msg = err instanceof Error ? err.message : String(err);
        void vscode.window.showErrorMessage(`Strixonomy refactor failed: ${msg}`);
      } finally {
        this.applying = false;
      }
    }
    if (message.type === "cancelRefactor") {
      this.host.panel.dispose();
    }
  }
}

async function runRefactorPreview(
  extensionUri: vscode.Uri,
  request: RefactorRequest,
  onApplied?: () => Promise<void>
): Promise<void> {
  const result = await previewRefactor(request);
  const plan: RefactorPlan = {
    changes: result.changes,
    warnings: result.warnings,
    affected_entity_count: result.affected_entity_count,
    affected_axiom_count: result.affected_axiom_count,
  };
  if (plan.changes.length === 0) {
    void vscode.window.showWarningMessage(
      "Refactor preview produced no file changes"
    );
    return;
  }
  await RefactorPreviewPanel.show(extensionUri, plan, request, onApplied);
}
