import * as vscode from "vscode";
import { StrixonomyApi } from "./api";
import { registerCommands, refreshExplorer } from "./commands";
import { createStrixonomyTestHooks } from "./testApi";
import {
  getCatalogSnapshot,
  getClient,
  getEntity,
  indexWorkspace,
  parseManchester,
  runSqlQuery,
  runSparqlQuery,
  startLanguageClient,
  stopLanguageClient,
} from "./lsp/client";
import { ExplorerTreeProvider } from "./treeviews/explorer";
import { registerWebviewPanelSerializers } from "./webviews/layoutPersistence";
import { registerErrorLog } from "./logging/errorLog";
import { initializeWorkspaceRuntime } from "./workspace";
import { migrateFromOntoCode } from "./migration/fromOntoCode";

let providers: {
  ontologies: ExplorerTreeProvider;
  classes: ExplorerTreeProvider;
  properties: ExplorerTreeProvider;
  individuals: ExplorerTreeProvider;
  diagnostics: ExplorerTreeProvider;
} | undefined;

let diagnosticsRefreshTimer: ReturnType<typeof setTimeout> | undefined;

export async function activate(
  context: vscode.ExtensionContext
): Promise<StrixonomyApi> {
  try {
    await migrateFromOntoCode(context);
    await startLanguageClient(context);

    providers = {
      ontologies: new ExplorerTreeProvider("ontologies"),
      classes: new ExplorerTreeProvider("classes"),
      properties: new ExplorerTreeProvider("properties"),
      individuals: new ExplorerTreeProvider("individuals"),
      diagnostics: new ExplorerTreeProvider("diagnostics"),
    };

    context.subscriptions.push(
      vscode.window.registerTreeDataProvider(
        "strixonomy.explorer.ontologies",
        providers.ontologies
      ),
      vscode.window.registerTreeDataProvider(
        "strixonomy.explorer.classes",
        providers.classes
      ),
      vscode.window.registerTreeDataProvider(
        "strixonomy.explorer.properties",
        providers.properties
      ),
      vscode.window.registerTreeDataProvider(
        "strixonomy.explorer.individuals",
        providers.individuals
      ),
      vscode.window.registerTreeDataProvider(
        "strixonomy.explorer.diagnostics",
        providers.diagnostics
      ),
      vscode.languages.onDidChangeDiagnostics((event) => {
        if (!providers) {
          return;
        }
        const ours = event.uris.some((uri) =>
          vscode.languages
            .getDiagnostics(uri)
            .some((d) => d.source === "strixonomy" || d.source === "ontocore")
        );
        if (!ours) {
          return;
        }
        if (diagnosticsRefreshTimer) {
          clearTimeout(diagnosticsRefreshTimer);
        }
        diagnosticsRefreshTimer = setTimeout(() => {
          void refreshExplorer(providers!);
        }, 300);
      })
    );

    registerCommands(context, providers);
    registerErrorLog(context);
    registerWebviewPanelSerializers(context);
    initializeWorkspaceRuntime(context);

    context.subscriptions.push(
      vscode.workspace.onDidChangeConfiguration((e) => {
        if (e.affectsConfiguration("strixonomy.hierarchy.mode") && providers) {
          void refreshExplorer(providers);
        }
      })
    );

    // Server indexes on `initialized` (debounced); refresh explorer once it settles.
    setTimeout(() => {
      void refreshExplorer(providers!);
    }, 900);

    return {
      getClient,
      indexWorkspace,
      getCatalogSnapshot,
      getEntity,
      runSqlQuery,
      runSparqlQuery,
      parseManchester: (
        expression: string,
        axiomKind: string,
        entityIri?: string,
        documentUri?: string
      ) =>
        parseManchester({
          expression,
          axiom_kind: axiomKind,
          entity_iri: entityIri,
          document_uri: documentUri,
        }),
      ...(process.env.STRIXONOMY_TEST_FIXTURES ||
      process.env.ONTOCODE_TEST_FIXTURES
        ? { __test: createStrixonomyTestHooks() }
        : {}),
    };
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    void vscode.window.showErrorMessage(
      `Strixonomy failed to start language server: ${message}`
    );
    throw err;
  }
}

export async function deactivate(): Promise<void> {
  if (diagnosticsRefreshTimer) {
    clearTimeout(diagnosticsRefreshTimer);
  }
  await stopLanguageClient();
  providers = undefined;
}
