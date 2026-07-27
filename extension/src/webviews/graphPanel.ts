import * as vscode from "vscode";
import { getGraph } from "../lsp/client";
import { PanelHost } from "./panelHost";
import type { GraphFilters, WebviewMessage } from "./messages";
import { forgetPanelRestoreState, rememberPanelRestoreState } from "./layoutPersistence";
import { graphRestoreState } from "./layoutPersistenceLogic";

export interface GraphPanelOptions {
  graphKind: string;
  rootIri?: string;
  rootIris?: string[];
  depth?: number;
  includeInferred?: boolean;
  filters?: GraphFilters;
}

export class GraphPanel {
  public static currentPanel: GraphPanel | undefined;
  private refreshGeneration = 0;

  private constructor(
    private readonly host: PanelHost,
    private options: GraphPanelOptions
  ) {
    host.panel.onDidDispose(() => {
      void forgetPanelRestoreState("strixonomyGraph");
      GraphPanel.currentPanel = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static async show(
    extensionUri: vscode.Uri,
    options: GraphPanelOptions,
    title = "Strixonomy Graph"
  ): Promise<GraphPanel> {
    void rememberPanelRestoreState(
      "strixonomyGraph",
      graphRestoreState(options, title)
    );
    if (GraphPanel.currentPanel) {
      GraphPanel.currentPanel.host.panel.reveal(vscode.ViewColumn.Beside);
      GraphPanel.currentPanel.host.panel.title = title;
      GraphPanel.currentPanel.options = options;
      await GraphPanel.currentPanel.refresh();
      return GraphPanel.currentPanel;
    }

    const host = PanelHost.create(extensionUri, {
      viewType: "strixonomyGraph",
      title,
      panel: "graph",
      extraQuery: {
        graphKind: options.graphKind,
        ...(options.rootIri ? { root: options.rootIri } : {}),
      },
      onMessage: async (message: WebviewMessage) => {
        const panel = GraphPanel.currentPanel;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });

    const instance = new GraphPanel(host, options);
    GraphPanel.currentPanel = instance;
    await instance.refresh();
    return instance;
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "requestGraph") {
      this.options = {
        graphKind: message.graphKind,
        rootIri: message.rootIri,
        rootIris: message.rootIris,
        depth: message.depth,
        includeInferred: message.includeInferred,
        filters: message.filters,
      };
      await this.refresh();
    }
    if (message.type === "selectNode" || message.type === "revealInHierarchy") {
      await vscode.commands.executeCommand("strixonomy.openEntity", message.iri);
    }
    if (message.type === "jumpToEditor") {
      await vscode.commands.executeCommand("strixonomy.jumpToSource", message.iri);
    }
    if (message.type === "exportGraph") {
      const filters: Record<string, string[]> = {
        JSON: ["json"],
        CSV: ["csv"],
      };
      const defaultExt = message.format === "csv" ? "csv" : "json";
      const uri = await vscode.window.showSaveDialog({
        defaultUri: vscode.Uri.file(
          message.suggestedName ?? `strixonomy-graph.${defaultExt}`
        ),
        filters,
      });
      if (!uri) {
        return;
      }
      await vscode.workspace.fs.writeFile(
        uri,
        Buffer.from(message.payload, "utf8")
      );
      void vscode.window.showInformationMessage(
        `Strixonomy: graph exported to ${uri.fsPath}`
      );
    }
  }

  private async refresh(): Promise<void> {
    const generation = ++this.refreshGeneration;
    try {
      const result = await getGraph({
        graph_kind: this.options.graphKind,
        root_iri: this.options.rootIri,
        root_iris: this.options.rootIris,
        depth: this.options.depth ?? 2,
        include_inferred: this.options.includeInferred ?? false,
        filters: this.options.filters,
      });
      if (generation !== this.refreshGeneration) {
        return;
      }
      this.host.postMessage({
        type: "graphData",
        graph: result.graph,
        rootIri: this.options.rootIri,
      });
    } catch (err) {
      if (generation !== this.refreshGeneration) {
        return;
      }
      const msg = err instanceof Error ? err.message : String(err);
      this.host.postMessage({ type: "error", message: msg });
    }
  }
}
