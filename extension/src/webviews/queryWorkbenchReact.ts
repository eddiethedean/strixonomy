import * as vscode from "vscode";
import {
  listSqlSchema,
  runDlQuery,
  runSparqlQuery,
  runSqlQuery,
} from "../lsp/client";
import { DlQueryResult, SavedQuery, TabularQueryResult } from "../lsp/protocol";
import { PanelHost } from "./panelHost";
import type { WebviewMessage } from "./messages";
import {
  parseRunQueryMessage,
  parseSaveQueryMessage,
} from "./messages";
import { forgetPanelRestoreState, rememberPanelRestoreState } from "./layoutPersistence";
import {
  SQL_TABLES,
  dlQueryToTabular,
  exportResultCsv,
  exportResultJson,
  mergeHistory,
  shouldDeliverQueryResult,
  upsertSavedQuery,
} from "./queryWorkbenchLogic";

const SAVED_KEY = "strixonomy.savedQueries";
const HISTORY_KEY = "strixonomy.queryHistory";
const DEFAULT_HISTORY_LIMIT = 20;

export class QueryWorkbenchPanel {
  public static current: QueryWorkbenchPanel | undefined;
  private host: PanelHost;
  private lastResult: TabularQueryResult | undefined;
  private lastDlResult: DlQueryResult | undefined;
  private lastResultRunId = 0;
  private runId = 0;

  private constructor(
    host: PanelHost,
    private readonly context: vscode.ExtensionContext
  ) {
    this.host = host;
    host.panel.onDidDispose(() => {
      void forgetPanelRestoreState("strixonomyQueryWorkbench");
      QueryWorkbenchPanel.current = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static async show(context: vscode.ExtensionContext): Promise<QueryWorkbenchPanel> {
    void rememberPanelRestoreState("strixonomyQueryWorkbench", {
      command: "strixonomy.openQueryWorkbench",
      title: "Strixonomy Query Workbench",
    });
    if (QueryWorkbenchPanel.current) {
      QueryWorkbenchPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      await QueryWorkbenchPanel.current.bootstrap();
      return QueryWorkbenchPanel.current;
    }
    const host = PanelHost.create(context.extensionUri, {
      viewType: "strixonomyQueryWorkbench",
      title: "Strixonomy Query Workbench",
      panel: "queryWorkbench",
      onMessage: async (message: WebviewMessage) => {
        const panel = QueryWorkbenchPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });
    const instance = new QueryWorkbenchPanel(host, context);
    QueryWorkbenchPanel.current = instance;
    void instance.bootstrap();
    return instance;
  }

  private async bootstrap(): Promise<void> {
    const saved = this.context.workspaceState.get<SavedQuery[]>(SAVED_KEY) ?? [];
    const history =
      this.context.workspaceState.get<SavedQuery[]>(HISTORY_KEY) ?? [];
    let sqlSchema: Awaited<ReturnType<typeof listSqlSchema>> | undefined;
    try {
      sqlSchema = await listSqlSchema();
    } catch {
      sqlSchema = undefined;
    }
    const sqlTables = sqlSchema?.map((t) => t.name) ?? [...SQL_TABLES];
    this.host.postMessage({
      type: "queryInit",
      saved,
      history,
      sqlTables,
      sqlSchema,
    });
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "runQuery") {
      const parsed = parseRunQueryMessage(message);
      if (!parsed) {
        return;
      }
      await this.runQuery(parsed.mode, parsed.text, parsed.runId, parsed.dlMode);
    }
    if (message.type === "saveQuery") {
      const parsed = parseSaveQueryMessage(message);
      if (!parsed) {
        return;
      }
      await this.saveQuery(parsed.mode, parsed.text, parsed.name, parsed.dlMode);
    }
    if (message.type === "exportQueryResult") {
      await this.exportResult(message.format, message.runId);
    }
    if (message.type === "openGraphFromResults") {
      await vscode.commands.executeCommand("strixonomy.openGraphFromResults", {
        graphKind: message.graphKind,
        rootIris: message.rootIris,
        title: message.title,
      });
    }
    if (message.type === "openEntity") {
      await vscode.commands.executeCommand("strixonomy.openEntity", message.iri);
    }
  }

  private async runQuery(
    mode: "sql" | "sparql" | "dl",
    text: string,
    runId: number,
    dlMode?: "asserted" | "inferred"
  ): Promise<void> {
    this.runId = runId;
    try {
      if (mode === "dl") {
        const dlResult = await runDlQuery({
          expression: text,
          mode: dlMode ?? "inferred",
        });
        if (!shouldDeliverQueryResult(runId, this.runId)) {
          return;
        }
        this.lastDlResult = dlResult;
        this.lastResult = dlQueryToTabular(dlResult);
        this.lastResultRunId = runId;
        this.host.postMessage({
          type: "queryResult",
          runId,
          dlResult,
          result: this.lastResult,
        });
      } else {
        const result =
          mode === "sql" ? await runSqlQuery(text) : await runSparqlQuery(text);
        if (!shouldDeliverQueryResult(runId, this.runId)) {
          return;
        }
        this.lastResult = result;
        this.lastDlResult = undefined;
        this.lastResultRunId = runId;
        this.host.postMessage({
          type: "queryResult",
          runId,
          result,
        });
      }
      const history = mergeHistory(
        this.context.workspaceState.get<SavedQuery[]>(HISTORY_KEY) ?? [],
        {
          name: `${mode} @ ${new Date().toLocaleTimeString()}`,
          mode,
          text,
          ...(mode === "dl" ? { dlMode: dlMode ?? "inferred" } : {}),
        },
        vscode.workspace
          .getConfiguration("strixonomy")
          .get<number>("queryHistoryLimit", DEFAULT_HISTORY_LIMIT)
      );
      await this.context.workspaceState.update(HISTORY_KEY, history);
      await this.bootstrap();
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      if (!shouldDeliverQueryResult(runId, this.runId)) {
        return;
      }
      this.lastResult = undefined;
      this.lastDlResult = undefined;
      this.lastResultRunId = 0;
      this.host.postMessage({ type: "queryResult", runId, error: msg });
    }
  }

  private async saveQuery(
    mode: "sql" | "sparql" | "dl",
    text: string,
    name: string,
    dlMode?: "asserted" | "inferred"
  ): Promise<void> {
    const entry: SavedQuery = { name, mode, text };
    if (mode === "dl") {
      entry.dlMode = dlMode ?? "inferred";
    }
    const saved = upsertSavedQuery(
      this.context.workspaceState.get<SavedQuery[]>(SAVED_KEY) ?? [],
      entry
    );
    await this.context.workspaceState.update(SAVED_KEY, saved);
    await this.bootstrap();
    void vscode.window.showInformationMessage(`Strixonomy: saved query "${name}"`);
  }

  private async exportResult(format: "csv" | "json", runId?: number): Promise<void> {
    if (!this.lastResult && !this.lastDlResult) {
      return;
    }
    if (runId !== undefined && runId !== this.lastResultRunId) {
      return;
    }
    const body =
      format === "csv"
        ? exportResultCsv(this.lastResult ?? dlQueryToTabular(this.lastDlResult!))
        : exportResultJson(this.lastDlResult ?? this.lastResult!);
    await vscode.env.clipboard.writeText(body);
    void vscode.window.showInformationMessage(
      `Strixonomy: ${format.toUpperCase()} copied to clipboard`
    );
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
}
