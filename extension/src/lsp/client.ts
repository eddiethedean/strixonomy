import * as fs from "fs";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";
import {
  assertCatalogSnapshot,
  assertGetEntityResult,
  assertIndexWorkspaceResult,
  assertApplyPatchResult,
  assertTabularQueryResult,
  assertDlQueryResult,
  assertSearchResult,
  assertParseManchesterResult,
  assertRunReasonerResult,
  assertGetExplanationResult,
  assertGetGraphResult,
  assertRunRobotResult,
  assertFindUsagesResult,
  assertPreviewRefactorResult,
  assertApplyRefactorResult,
  assertSemanticDiffResult,
  assertListPluginsResult,
  assertRunPluginResult,
} from "./protocolGuards";
import { patchSyncCancelledMessage } from "./patchFeedback";
import { noteSelfWrite, noteSelfWrites } from "../workspace/selfWriteGuard";
import {
  ApplyAxiomPatchClientResult,
  ApplyAxiomPatchParams,
  ApplyPatchResult,
  ApplyRefactorResult,
  CatalogSnapshot,
  CreateOntologyParams,
  CreateOntologyResult,
  DeleteImpactParams,
  DeleteImpactResult,
  DlQueryParams,
  DlQueryResult,
  ExportOntologyParams,
  ExportOntologyResult,
  FindUsagesResult,
  GetDialogSchemaResult,
  GetEntityResult,
  GetExplanationParams,
  GetExplanationResult,
  GetGraphParams,
  GetGraphResult,
  IndexWorkspaceResult,
  ListCommandsResult,
  ListPluginsResult,
  ListSwrlRulesResult,
  ParseManchesterParams,
  ParseManchesterResult,
  ParseSwrlRuleParams,
  ParseSwrlRuleResult,
  PreviewRefactorResult,
  RefactorPlan,
  RefactorRequest,
  RunPluginParams,
  RunPluginResult,
  RunReasonerParams,
  RunReasonerResult,
  RunRobotParams,
  RunRobotResult,
  SearchParams,
  SearchResult,
  SemanticDiffParams,
  SemanticDiffResult,
  SetActiveOntologyParams,
  SetActiveOntologyResult,
  TabularQueryResult,
  ValidateSwrlRuleParams,
  ValidateSwrlRuleResult,
  WorkspaceUiState,
  WorkspaceUiStateParams,
} from "./protocol";
import { focusRelay } from "../focus/focusRelay";
import {
  bundledServerPath,
  ensureBundledServerExecutable,
} from "./bundledServer";

let client: LanguageClient | undefined;
let starting: Promise<LanguageClient> | undefined;
/** Bumped on stop so in-flight startups do not publish a superseded client (#91). */
let startGeneration = 0;

export function getClient(): LanguageClient | undefined {
  return client;
}

export async function startLanguageClient(
  context: vscode.ExtensionContext
): Promise<LanguageClient> {
  if (client) {
    return client;
  }
  if (starting) {
    return starting;
  }

  const generation = ++startGeneration;
  starting = (async () => {
    const serverPath = resolveServerPath(context);
    const serverOptions: ServerOptions = {
      run: { command: serverPath, transport: TransportKind.stdio },
      debug: { command: serverPath, transport: TransportKind.stdio },
    };

    const clientOptions: LanguageClientOptions = {
      documentSelector: [
        { scheme: "file", language: "turtle" },
        { scheme: "file", language: "obo" },
        { scheme: "file", pattern: "**/*.{ttl,owl,rdf,owx,jsonld,json-ld,nt,nq,trig,obo}" },
      ],
      synchronize: {
        fileEvents: vscode.workspace.createFileSystemWatcher(
          "**/*.{ttl,owl,rdf,owx,jsonld,json-ld,nt,nq,trig,obo}"
        ),
      },
      outputChannelName: "Strixonomy Language Server",
    };

    const created = new LanguageClient(
      "strixonomy-lsp",
      "Strixonomy Language Server",
      serverOptions,
      clientOptions
    );

    context.subscriptions.push({
      dispose: () => {
        void stopLanguageClient();
      },
    });

    try {
      // vscode-languageclient v9: await start() — onReady() was removed.
      await created.start();
      if (generation !== startGeneration) {
        try {
          await created.stop();
        } catch {
          // Best-effort cleanup of a superseded startup.
        }
        throw new Error("Strixonomy language server startup was cancelled");
      }
      client = created;
      return created;
    } catch (err) {
      if (client === created) {
        client = undefined;
      }
      const detail = err instanceof Error ? err.message : String(err);
      throw new Error(
        `${detail} (server: ${serverPath}). See Output → Strixonomy Language Server.`
      );
    }
  })();

  try {
    return await starting;
  } finally {
    starting = undefined;
  }
}

export async function stopLanguageClient(): Promise<void> {
  startGeneration += 1;
  const inFlight = starting;
  starting = undefined;
  if (inFlight) {
    try {
      const created = await inFlight;
      await created.stop();
    } catch {
      // Startup failed or was cancelled — nothing to stop.
    }
  }
  if (client) {
    const active = client;
    client = undefined;
    await active.stop();
  }
}

function resolveServerPath(context: vscode.ExtensionContext): string {
  const configured = vscode.workspace
    .getConfiguration("strixonomy")
    .get<string>("lspPath");
  if (
    configured &&
    configured.trim().length > 0 &&
    fs.existsSync(configured) &&
    vscode.workspace.isTrusted
  ) {
    return configured;
  }
  if (
    configured &&
    configured.trim().length > 0 &&
    fs.existsSync(configured) &&
    !vscode.workspace.isTrusted
  ) {
    void vscode.window.showWarningMessage(
      "Strixonomy: strixonomy.lspPath is ignored in Restricted Mode; using the bundled language server."
    );
  }

  const bundled = bundledServerPath(context.extensionPath);
  if (fs.existsSync(bundled)) {
    ensureBundledServerExecutable(bundled);
    return bundled;
  }

  throw new Error(
    "Bundled strixonomy-lsp binary not found. Rebuild the extension (npm run compile) or set strixonomy.lspPath to a local strixonomy-lsp binary."
  );
}

export async function indexWorkspace(
  workspaceUri?: string
): Promise<IndexWorkspaceResult> {
  const uri = workspaceUri ?? (await pickWorkspaceFolderUri());
  if (!uri) {
    throw new Error(
      "No workspace folder is open. Open a folder containing ontology files, then run Index Workspace."
    );
  }
  const diskCache = vscode.workspace
    .getConfiguration("strixonomy")
    .get<boolean>("indexCache", false);
  const result = await ontcoreRequest<unknown>("strixonomy/indexWorkspace", {
    workspace_uri: uri,
    disk_cache: diskCache,
  });
  const indexed = assertIndexWorkspaceResult(result) as IndexWorkspaceResult;
  const { focusRelay } = await import("../focus/focusRelay");
  focusRelay.setCatalogFingerprint({
    indexedAt: indexed.indexed_at,
  });
  return indexed;
}

async function pickWorkspaceFolderUri(): Promise<string | undefined> {
  const folders = vscode.workspace.workspaceFolders;
  if (!folders || folders.length === 0) {
    return undefined;
  }
  if (folders.length === 1) {
    return folders[0].uri.toString();
  }
  const picked = await vscode.window.showWorkspaceFolderPick({
    placeHolder: "Select workspace folder to index",
  });
  return picked?.uri.toString();
}

export async function getCatalogSnapshot(): Promise<CatalogSnapshot> {
  const result = await ontcoreRequest<unknown>(
    "strixonomy/getCatalogSnapshot",
    null
  );
  return assertCatalogSnapshot(result);
}

export async function listCommands(): Promise<ListCommandsResult> {
  return ontcoreRequest<ListCommandsResult>("strixonomy/listCommands", null);
}

export async function getWorkspaceUiState(
  params: WorkspaceUiStateParams = {}
): Promise<WorkspaceUiState> {
  return ontcoreRequest<WorkspaceUiState>("strixonomy/getWorkspaceUiState", params);
}

export async function getDialogSchema(
  dialogId: string
): Promise<GetDialogSchemaResult> {
  return ontcoreRequest<GetDialogSchemaResult>("strixonomy/getDialogSchema", {
    dialog_id: dialogId,
  });
}

export async function createOntology(
  params: CreateOntologyParams
): Promise<CreateOntologyResult> {
  return ontcoreRequest<CreateOntologyResult>("strixonomy/createOntology", params);
}

export async function exportOntology(
  params: ExportOntologyParams
): Promise<ExportOntologyResult> {
  return ontcoreRequest<ExportOntologyResult>("strixonomy/exportOntology", params);
}

export async function setActiveOntology(
  params: SetActiveOntologyParams
): Promise<SetActiveOntologyResult> {
  return ontcoreRequest<SetActiveOntologyResult>(
    "strixonomy/setActiveOntology",
    params
  );
}

export async function deleteImpact(
  params: DeleteImpactParams
): Promise<DeleteImpactResult> {
  return ontcoreRequest<DeleteImpactResult>("strixonomy/deleteImpact", params);
}

export async function listPlugins(): Promise<ListPluginsResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/listPlugins", null);
  return assertListPluginsResult(result);
}

export async function runPlugin(params: RunPluginParams): Promise<RunPluginResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/runPlugin", {
    plugin_id: params.plugin_id,
    action: params.action ?? "validate",
    step: params.step,
    view_id: params.view_id,
  });
  return assertRunPluginResult(result);
}

export async function getEntity(iri: string): Promise<GetEntityResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/getEntity", { iri });
  return assertGetEntityResult(result);
}

/** LSP applies patch ops as an atomic strixonomy-edit Transaction (v0.19+). */
export async function applyAxiomPatch(
  params: ApplyAxiomPatchParams
): Promise<ApplyAxiomPatchClientResult> {
  // Suppress external-change recovery for Strixonomy's own disk write (#293).
  if (!params.preview_only && params.document_uri) {
    try {
      noteSelfWrite(vscode.Uri.parse(params.document_uri).fsPath);
    } catch {
      // Invalid URI — LSP call will fail on its own.
    }
  }
  const result = await ontcoreRequest<unknown>(
    "strixonomy/applyAxiomPatch",
    params
  );
  const patch = assertApplyPatchResult(result);
  if (patch.applied) {
    focusRelay.markReasoningDirty();
    if (params.document_uri) {
      try {
        noteSelfWrite(vscode.Uri.parse(params.document_uri).fsPath);
      } catch {
        // ignore
      }
    }
  }
  if (patch.applied && patch.workspace_edit) {
    const { applyLspWorkspaceEdit } = await import("./workspaceEdit");
    const synced = await applyLspWorkspaceEdit(patch.workspace_edit);
    if (!synced) {
      void vscode.window.showWarningMessage(patchSyncCancelledMessage());
      return { ...patch, editor_synced: false };
    }
    return { ...patch, editor_synced: true };
  }
  return { ...patch, editor_synced: true };
}

type SqlTableSchema = {
  name: string;
  columns: Array<{ name: string; type: string }>;
};

function parseListSqlSchemaResult(result: unknown): SqlTableSchema[] {
  if (Array.isArray(result)) {
    return result as SqlTableSchema[];
  }
  if (
    result &&
    typeof result === "object" &&
    Array.isArray((result as { tables?: unknown }).tables)
  ) {
    return (result as { tables: SqlTableSchema[] }).tables;
  }
  throw new Error("Invalid listSqlSchema response");
}

export async function listSqlSchema(): Promise<SqlTableSchema[]> {
  const result = await ontcoreRequest<unknown>("strixonomy/listSqlSchema", {});
  return parseListSqlSchemaResult(result);
}

export async function runSqlQuery(sql: string): Promise<TabularQueryResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/query", { sql });
  return assertTabularQueryResult(result);
}

export async function runSparqlQuery(
  query: string
): Promise<TabularQueryResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/sparql", { query });
  return assertTabularQueryResult(result);
}

export async function runDlQuery(
  params: DlQueryParams
): Promise<DlQueryResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/dlQuery", params);
  return assertDlQueryResult(result);
}

export async function searchEntities(
  params: SearchParams
): Promise<SearchResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/search", params);
  return assertSearchResult(result);
}

export async function parseManchester(
  params: ParseManchesterParams
): Promise<ParseManchesterResult> {
  const result = await ontcoreRequest<unknown>(
    "strixonomy/parseManchester",
    params
  );
  return assertParseManchesterResult(result);
}

export async function listSwrlRules(): Promise<ListSwrlRulesResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/listSwrlRules", {});
  if (
    !result ||
    typeof result !== "object" ||
    !Array.isArray((result as { rules?: unknown }).rules)
  ) {
    throw new Error("Invalid listSwrlRules result from language server");
  }
  return result as ListSwrlRulesResult;
}

export async function validateSwrlRule(
  params: ValidateSwrlRuleParams
): Promise<ValidateSwrlRuleResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/validateSwrlRule", params);
  if (
    !result ||
    typeof result !== "object" ||
    !Array.isArray((result as { diagnostics?: unknown }).diagnostics)
  ) {
    throw new Error("Invalid validateSwrlRule result from language server");
  }
  return result as ValidateSwrlRuleResult;
}

export async function parseSwrlRule(
  params: ParseSwrlRuleParams
): Promise<ParseSwrlRuleResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/parseSwrlRule", params);
  if (
    !result ||
    typeof result !== "object" ||
    !(result as { rule?: unknown }).rule ||
    !Array.isArray((result as { diagnostics?: unknown }).diagnostics)
  ) {
    throw new Error("Invalid parseSwrlRule result from language server");
  }
  return result as ParseSwrlRuleResult;
}

/** Active reasoner RPC cancellation (Stop / progress Cancel). */
let reasonerCancelSource: vscode.CancellationTokenSource | undefined;

export function cancelActiveReasonerRequest(): void {
  reasonerCancelSource?.cancel();
  reasonerCancelSource?.dispose();
  reasonerCancelSource = undefined;
}

export function isReasonerRequestActive(): boolean {
  return reasonerCancelSource !== undefined;
}

export async function runReasoner(
  params: RunReasonerParams,
  token?: vscode.CancellationToken
): Promise<RunReasonerResult> {
  cancelActiveReasonerRequest();
  const source = new vscode.CancellationTokenSource();
  reasonerCancelSource = source;
  if (token) {
    if (token.isCancellationRequested) {
      source.cancel();
    } else {
      token.onCancellationRequested(() => source.cancel());
    }
  }
  try {
    const result = await ontcoreRequest<unknown>(
      "strixonomy/runReasoner",
      params,
      source.token
    );
    return assertRunReasonerResult(result);
  } finally {
    if (reasonerCancelSource === source) {
      reasonerCancelSource = undefined;
    }
    source.dispose();
  }
}

export async function getExplanation(
  params: GetExplanationParams
): Promise<GetExplanationResult> {
  const result = await ontcoreRequest<unknown>(
    "strixonomy/getExplanation",
    params
  );
  const explained = assertGetExplanationResult(result);
  return explained;
}

export async function getGraph(params: GetGraphParams): Promise<GetGraphResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/getGraph", params);
  return assertGetGraphResult(result);
}

export async function runRobot(params: RunRobotParams): Promise<RunRobotResult> {
  let robotPath =
    params.robot_path ??
    vscode.workspace.getConfiguration("strixonomy").get<string>("robotPath");
  if (robotPath && robotPath.trim().length > 0 && !vscode.workspace.isTrusted) {
    void vscode.window.showWarningMessage(
      "Strixonomy: strixonomy.robotPath is ignored in Restricted Mode; using robot on PATH."
    );
    robotPath = undefined;
  }
  const result = await ontcoreRequest<unknown>("strixonomy/runRobot", {
    ...params,
    robot_path: robotPath || undefined,
  });
  return assertRunRobotResult(result);
}

export async function findUsages(iri: string): Promise<FindUsagesResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/findUsages", { iri });
  return assertFindUsagesResult(result);
}

export async function previewRefactor(
  request: RefactorRequest
): Promise<PreviewRefactorResult> {
  const result = await ontcoreRequest<unknown>(
    "strixonomy/previewRefactor",
    request
  );
  return assertPreviewRefactorResult(result);
}

export async function applyRefactor(
  plan: RefactorPlan,
  request: RefactorRequest,
  previewOnly = false
): Promise<ApplyRefactorResult> {
  // Suppress external-change recovery for Strixonomy's own refactor writes (#396).
  const notePlanWrites = (): void => {
    if (previewOnly) {
      return;
    }
    noteSelfWrites(plan.changes.map((change) => change.path));
  };
  notePlanWrites();
  const result = await ontcoreRequest<unknown>("strixonomy/applyRefactor", {
    plan,
    request,
    preview_only: previewOnly,
  });
  const applied = assertApplyRefactorResult(result);
  if (!previewOnly && applied.files_written > 0) {
    notePlanWrites();
    focusRelay.markReasoningDirty();
  }
  return applied;
}

export async function semanticDiff(
  params: SemanticDiffParams
): Promise<SemanticDiffResult> {
  const result = await ontcoreRequest<unknown>("strixonomy/semanticDiff", params);
  return assertSemanticDiffResult(result);
}

interface LspErrorPayload {
  code?: string;
  message?: string;
  user_action?: string;
  recoverable?: boolean;
}

function formatOntocoreRpcError(err: unknown): Error {
  if (err && typeof err === "object") {
    const rpc = err as {
      code?: number;
      message?: string;
      data?: LspErrorPayload;
    };
    const data = rpc.data;
    if (data?.message) {
      const action = data.user_action ? ` ${data.user_action}` : "";
      return new Error(`${data.code ?? "LSP_ERROR"}: ${data.message}${action}`);
    }
    if (rpc.message) {
      return new Error(rpc.message);
    }
  }
  if (err instanceof Error) {
    return err;
  }
  return new Error(String(err));
}

async function ontcoreRequest<T>(
  method: string,
  params: unknown,
  token?: vscode.CancellationToken
): Promise<T> {
  try {
    if (token) {
      return await requireClient().sendRequest<T>(method, params, token);
    }
    return await requireClient().sendRequest<T>(method, params);
  } catch (err) {
    throw formatOntocoreRpcError(err);
  }
}

function requireClient(): LanguageClient {
  if (!client) {
    throw new Error("Strixonomy language server is not running");
  }
  return client;
}
