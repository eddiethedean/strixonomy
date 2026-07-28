import * as path from "path";
import * as vscode from "vscode";
import {
  cancelActiveReasonerRequest,
  createOntology,
  exportOntology,
  getCatalogSnapshot,
  indexWorkspace,
  listPlugins,
  previewRefactor,
  runReasoner,
  searchEntities,
} from "../lsp/client";
import { requirePatchFullySynced } from "../lsp/patchFeedback";
import type {
  OntologyDocument,
  PatchOp,
  RefactorRequest,
  RunReasonerResult,
} from "../lsp/protocol";
import { focusRelay } from "../focus/focusRelay";
import { CommandRegistry } from "./registry";
import { getFocusedEntityIri } from "./uiState";
import { appendError, openErrorLog } from "../logging/errorLog";
import { normalizeFsPath, pathsEqual } from "../utils/pathUnder";
import { documentUriInWorkspace } from "../utils/workspacePath";
import {
  listPerspectives,
  persistPerspective,
  type Perspective,
} from "../webviews/layoutPersistence";
import {
  ontologyRegistry,
  saveCoordinator,
  workspaceTransactionManager,
} from "../workspace";
import { RefactorPreviewPanel } from "../webviews/refactorPreview";
import { ReasonerPanel } from "../webviews/reasonerPanel";
import {
  captureReasoningPreRun,
  reasoningStateForRunCancel,
  reasoningStateForRunStart,
  reasoningStateForRunSuccess,
} from "../webviews/reasonerPanelLogic";
import { PanelHost } from "../webviews/panelHost";

const ONTOLOGY_FILTERS: Record<string, string[]> = {
  "Ontology files": ["ttl", "owl", "rdf", "jsonld", "json-ld", "nt", "nq", "trig", "obo"],
};

/** Module context for test hooks that open dialogs without native pickers. */
let dialogRuntime:
  | { extensionUri: vscode.Uri; refresh?: () => Promise<void> }
  | undefined;

export function registerV017Commands(
  context: vscode.ExtensionContext,
  refresh?: () => Promise<void>
): void {
  dialogRuntime = { extensionUri: context.extensionUri, refresh };
  const registry = new CommandRegistry(context);
  const command = (id: string, handler: (...args: never[]) => unknown): void => {
    registry.register(id, async (...args) => {
      try {
        return await handler(...(args as never[]));
      } catch (error) {
        appendError(error, id);
        const message = error instanceof Error ? error.message : String(error);
        void vscode.window.showErrorMessage(`Strixonomy: ${message}`);
        return undefined;
      }
    });
  };

  command("strixonomy.newOntology", async () => {
    const target = await vscode.window.showSaveDialog({
      title: "New Ontology",
      filters: { "Ontology files": ["ttl", "obo"] },
      defaultUri: defaultWorkspaceUri("ontology.ttl"),
    });
    if (!target) return;
    openNewOntologyPanel(context.extensionUri, target.fsPath, refresh);
  });
  command("strixonomy.openOntology", async () => {
    const selected = await vscode.window.showOpenDialog({
      canSelectMany: false,
      filters: ONTOLOGY_FILTERS,
      openLabel: "Open Ontology",
    });
    if (selected?.[0]) {
      await ontologyRegistry.open(selected[0]);
      await refresh?.();
    }
  });
  command("strixonomy.openRecent", () =>
    vscode.commands.executeCommand("workbench.action.openRecent")
  );
  command("strixonomy.save", async () => {
    await saveCoordinator.saveActive();
    await refresh?.();
  });
  command("strixonomy.saveAll", async () => {
    await saveCoordinator.saveAll();
    await refresh?.();
  });
  command("strixonomy.undo", async () => {
    if (await workspaceTransactionManager.undo()) {
      await refresh?.();
      return;
    }
    await vscode.commands.executeCommand("undo");
  });
  command("strixonomy.redo", async () => {
    if (await workspaceTransactionManager.redo()) {
      await refresh?.();
      return;
    }
    await vscode.commands.executeCommand("redo");
  });
  command("strixonomy.closeProject", () =>
    vscode.commands.executeCommand("workbench.action.closeFolder")
  );
  command("strixonomy.saveAs", () => runExport(true));
  command("strixonomy.exportOntology", () => runExport(false));

  command("strixonomy.searchEntities", async () => {
    type PickItem = vscode.QuickPickItem & { iri: string };
    const qp = vscode.window.createQuickPick<PickItem>();
    qp.title = "Search Ontology Entities";
    qp.placeholder = "Type to search by name, label, or IRI…";
    qp.matchOnDescription = true;
    qp.matchOnDetail = true;
    let seq = 0;
    qp.onDidChangeValue((value) => {
      const my = ++seq;
      const trimmed = value.trim();
      if (!trimmed) {
        qp.items = [];
        return;
      }
      qp.busy = true;
      void searchEntities({ query: trimmed, limit: 100 })
        .then((result) => {
          if (my !== seq) {
            return;
          }
          qp.items = result.entities.map((detail) => ({
            label: detail.entity.labels[0] || detail.entity.short_name || detail.entity.iri,
            description: detail.entity.kind.replace(/_/g, " "),
            detail: detail.entity.iri,
            iri: detail.entity.iri,
          }));
        })
        .catch((err: unknown) => {
          if (my !== seq) {
            return;
          }
          qp.items = [];
          const message = err instanceof Error ? err.message : String(err);
          void vscode.window.showErrorMessage(`Strixonomy: search failed — ${message}`);
        })
        .finally(() => {
          if (my === seq) {
            qp.busy = false;
          }
        });
    });
    qp.onDidAccept(() => {
      const picked = qp.selectedItems[0];
      if (picked) {
        focusRelay.setEntityFocus(picked.iri, "search");
        void vscode.commands.executeCommand("strixonomy.openEntity", picked.iri);
      }
      qp.hide();
    });
    qp.onDidHide(() => qp.dispose());
    qp.show();
  });

  command("strixonomy.openPreferences", async () => {
    const choice = await vscode.window.showQuickPick(
      [
        {
          label: "General Strixonomy Settings",
          description: "Index, hierarchy, diagnostics, LSP",
          value: "settings",
        },
        {
          label: "Reasoner Settings",
          description: "Default profile and auto-detect",
          value: "reasoner",
        },
        {
          label: "Query Settings",
          description: "History limit and workbench defaults",
          value: "query",
        },
        {
          label: "Plugin Preferences",
          description: "Pages contributed by workspace plugins",
          value: "plugins",
        },
        {
          label: "Keyboard Shortcuts",
          description: "Strixonomy keybindings",
          value: "keys",
        },
      ],
      { title: "Strixonomy Preferences", matchOnDescription: true }
    );
    if (!choice) return;
    if (choice.value === "plugins") {
      await vscode.commands.executeCommand("strixonomy.plugins.openPreferences");
    } else if (choice.value === "keys") {
      await vscode.commands.executeCommand(
        "workbench.action.openGlobalKeybindings",
        "strixonomy"
      );
    } else if (choice.value === "reasoner") {
      await vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "@ext:strixonomy.strixonomy strixonomy.reasoner"
      );
    } else if (choice.value === "query") {
      await vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "@ext:strixonomy.strixonomy strixonomy.query"
      );
    } else {
      await vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "@ext:strixonomy.strixonomy"
      );
    }
  });

  command("strixonomy.copyEntityIri", () => copyFocused(false));
  command("strixonomy.copyEntityShortForm", () => copyFocused(true));

  command("strixonomy.setActiveOntology", async () => {
    const document = await pickOntologyDocument("Set Active Ontology");
    if (!document) return;
    await ontologyRegistry.activate(document.id);
    await refresh?.();
  });

  command("strixonomy.editOntologyMetadata", async () => {
    const document = await pickOntologyDocument("Edit Ontology Metadata");
    if (!document) return;
    const ontologyIri = await requiredInput(
      "Ontology IRI",
      document.base_iri ?? "https://example.org/ontology"
    );
    if (!ontologyIri) return;
    const predicate = await vscode.window.showInputBox({
      prompt: "Annotation predicate IRI (optional)",
      placeHolder: "http://www.w3.org/2000/01/rdf-schema#label",
    });
    if (predicate === undefined) return;
    const patches: PatchOp[] = [
      { op: "set_ontology_iri", ontology_iri: ontologyIri },
    ];
    if (predicate.trim()) {
      const value = await requiredInput("Annotation value");
      if (!value) return;
      patches.push({
        op: "add_ontology_annotation",
        ontology_iri: ontologyIri,
        predicate: predicate.trim(),
        value,
      });
    }
    await applyDocumentPatches(document, patches);
    await refresh?.();
  });

  command("strixonomy.managePrefixes", async () => {
    const document = await pickOntologyDocument("Manage Prefixes");
    if (!document) return;
    openPrefixManagerPanel(context.extensionUri, document, refresh);
  });
  command("strixonomy.showMetrics", async () => {
    const snapshot = await getCatalogSnapshot();
    const host = PanelHost.create(context.extensionUri, {
      viewType: "strixonomy.metrics",
      title: "Ontology Metrics",
      panel: "metrics",
    });
    host.postMessage({
      type: "loadMetrics",
      stats: snapshot.stats ?? {
        ontology_count: snapshot.documents.length,
        class_count: snapshot.entities.filter((entity) => entity.kind === "class").length,
        object_property_count: 0,
        data_property_count: 0,
        annotation_property_count: 0,
        individual_count: snapshot.entities.filter((entity) => entity.kind === "individual").length,
        axiom_count: 0,
        annotation_count: 0,
        triple_count: 0,
        error_count: 0,
        diagnostic_error_count: snapshot.diagnostics.length,
        diagnostic_warning_count: 0,
        diagnostic_info_count: 0,
      },
    });
  });

  command("strixonomy.mergeEntities", () =>
    runEntityRefactor(context, "merge_entities", refresh)
  );
  command("strixonomy.replaceEntity", () =>
    runEntityRefactor(context, "replace_entity", refresh)
  );

  command("strixonomy.startReasoner", () => runReasoning(context, "start"));
  command("strixonomy.synchronizeReasoner", () => runReasoning(context, "synchronize"));
  command("strixonomy.classifyOntology", () => runReasoning(context, "classify"));
  command("strixonomy.checkConsistency", async () => {
    const result = await runReasoning(context, "consistency");
    if (result) {
      void vscode.window.showInformationMessage(
        result.consistent
          ? "Strixonomy: ontology is consistent"
          : `Strixonomy: ontology is inconsistent (${result.unsatisfiable.length} unsatisfiable classes)`
      );
    }
  });
  command("strixonomy.stopReasoner", () => {
    cancelActiveReasonerRequest();
    ReasonerPanel.current?.cancelActiveRun();
    void vscode.window.showInformationMessage(
      "Strixonomy: reasoner cancelled; late server results will be ignored"
    );
  });
  command("strixonomy.configureReasoner", () =>
    vscode.commands.executeCommand(
      "workbench.action.openSettings",
      "@ext:strixonomy.strixonomy strixonomy.reasoner"
    )
  );

  command("strixonomy.validateWorkspace", async () => {
    const result = await indexWorkspace();
    await refresh?.();
    void vscode.window.showInformationMessage(
      `Strixonomy validation: ${result.stats.diagnostic_error_count} errors, ${result.stats.diagnostic_warning_count} warnings, ${result.stats.diagnostic_info_count} info`
    );
  });
  command("strixonomy.runBatchTools", () =>
    vscode.window.withProgress(
      {
        location: vscode.ProgressLocation.Notification,
        title: "Strixonomy: validating and collecting metrics",
      },
      async (progress) => {
        progress.report({ message: "Validating workspace…" });
        const result = await indexWorkspace();
        progress.report({ message: "Collecting metrics…" });
        const snapshot = await getCatalogSnapshot();
        await refresh?.();
        void vscode.window.showInformationMessage(
          `Strixonomy batch complete: ${result.stats.diagnostic_error_count} errors, ${snapshot.stats?.axiom_count ?? 0} axioms`
        );
      }
    )
  );

  command("strixonomy.switchPerspective", async () => {
    const picked = await vscode.window.showQuickPick(
      listPerspectives(context).map((perspective) => ({
        label: perspective.name,
        description: perspective.panels.join(", "),
        perspective,
      })),
      { title: "Switch Strixonomy Perspective" }
    );
    if (picked) await openPerspective(context, picked.perspective);
  });
  command("strixonomy.savePerspective", async () => {
    const name = await requiredInput("Perspective name");
    if (!name) return;
    const panels = await vscode.window.showQuickPick(
      PANEL_CHOICES,
      { canPickMany: true, title: "Panels in Perspective" }
    );
    if (!panels) return;
    await persistPerspective(context, {
      name,
      panels: panels.map((item) => item.value),
    });
  });
  for (const panel of PANEL_CHOICES) {
    command(`strixonomy.show${panel.commandSuffix}`, () =>
      openPanel(context, panel.value)
    );
  }

  command("strixonomy.showAbout", () => {
    PanelHost.create(context.extensionUri, {
      viewType: "strixonomy.about",
      title: "About Strixonomy",
      panel: "about",
    });
  });
  command("strixonomy.showPluginInfo", async () => {
    const plugins = await listPlugins();
    const details =
      plugins.plugins.map((plugin) => `${plugin.name} ${plugin.version}`).join(", ") ||
      "No plugins loaded";
    void vscode.window.showInformationMessage(`Strixonomy plugins: ${details}`);
  });
  command("strixonomy.openErrorLog", () => openErrorLog());
  command("strixonomy.exportDiagnostics", async () => {
    const snapshot = await getCatalogSnapshot();
    const target = await vscode.window.showSaveDialog({
      defaultUri: defaultWorkspaceUri("strixonomy-diagnostics.json"),
      filters: { JSON: ["json"] },
    });
    if (target) {
      await vscode.workspace.fs.writeFile(
        target,
        Buffer.from(JSON.stringify(snapshot.diagnostics, null, 2), "utf8")
      );
    }
  });
  command("strixonomy.openDocumentation", () =>
    vscode.env.openExternal(
      vscode.Uri.parse("https://strixonomy.readthedocs.io/en/latest/")
    )
  );
  command("strixonomy.openSupport", () =>
    vscode.env.openExternal(
      vscode.Uri.parse("https://github.com/eddiethedean/strixonomy/issues")
    )
  );
  command("strixonomy.openKeyboardShortcuts", () =>
    vscode.commands.executeCommand(
      "workbench.action.openGlobalKeybindings",
      "strixonomy"
    )
  );

  registry.startContextSync();
}

/**
 * Open New Ontology dialog for a concrete path (skips showSaveDialog).
 * Used by VS Code e2e hooks when ONTOCODE_TEST_FIXTURES is set.
 */
export function openNewOntologyDialog(targetPath: string): void {
  if (!dialogRuntime) {
    throw new Error("Strixonomy dialog commands are not registered");
  }
  openNewOntologyPanel(
    dialogRuntime.extensionUri,
    targetPath,
    dialogRuntime.refresh
  );
}

/**
 * Open Prefix Manager for a catalog document path (skips multi-doc quick pick).
 * Used by VS Code e2e hooks when ONTOCODE_TEST_FIXTURES is set.
 */
export async function openPrefixManager(documentPath: string): Promise<void> {
  if (!dialogRuntime) {
    throw new Error("Strixonomy dialog commands are not registered");
  }
  const snapshot = await getCatalogSnapshot();
  const document = snapshot.documents.find((doc) =>
    pathsEqual(doc.path, documentPath)
  );
  if (!document) {
    throw new Error(`No indexed ontology document at ${documentPath}`);
  }
  openPrefixManagerPanel(
    dialogRuntime.extensionUri,
    document,
    dialogRuntime.refresh
  );
}

function openNewOntologyPanel(
  extensionUri: vscode.Uri,
  targetPath: string,
  refresh?: () => Promise<void>
): PanelHost {
  const host = PanelHost.create(extensionUri, {
    viewType: "strixonomy.newOntology",
    title: "New Ontology",
    panel: "newOntology",
    onMessage: async (message, panel) => {
      if (message.type !== "submitNewOntology") return;
      const result = await createOntology({
        path: targetPath,
        ontology_iri: message.ontologyIri,
        version_iri: message.versionIri,
        format: formatForPath(targetPath),
      });
      await refresh?.();
      panel.dispose();
      await vscode.window.showTextDocument(
        await vscode.workspace.openTextDocument(result.path)
      );
    },
  });
  host.postMessage({
    type: "loadNewOntology",
    path: targetPath,
    defaultIri: "https://example.org/ontology",
  });
  return host;
}

function openPrefixManagerPanel(
  extensionUri: vscode.Uri,
  document: OntologyDocument,
  refresh?: () => Promise<void>
): PanelHost {
  const host = PanelHost.create(extensionUri, {
    viewType: "strixonomy.prefixManager",
    title: "Prefix Manager",
    panel: "prefixManager",
    onMessage: async (message, panel) => {
      if (message.type !== "submitPrefix") return;
      const namespaces = document.namespaces ?? {};
      const patch: PatchOp =
        message.action === "remove"
          ? { op: "remove_prefix", prefix: message.prefix }
          : Object.prototype.hasOwnProperty.call(namespaces, message.prefix)
            ? {
                op: "set_prefix",
                prefix: message.prefix,
                namespace_iri: message.namespaceIri ?? "",
              }
            : {
                op: "add_prefix",
                prefix: message.prefix,
                namespace_iri: message.namespaceIri ?? "",
              };
      await applyDocumentPatches(document, [patch]);
      await refresh?.();
      panel.dispose();
    },
  });
  host.postMessage({
    type: "loadPrefixes",
    path: document.path,
    prefixes: document.namespaces ?? {},
  });
  return host;
}

async function runExport(saveAs: boolean): Promise<void> {
  const document = await pickOntologyDocument(
    saveAs ? "Save Ontology As" : "Export Ontology"
  );
  if (!document) return;
  const target = await vscode.window.showSaveDialog({
    title: saveAs ? "Save Ontology As" : "Export Ontology",
    defaultUri: vscode.Uri.file(normalizeFsPath(document.path)),
    filters: ONTOLOGY_FILTERS,
  });
  if (!target) return;
  const result = await exportOntology({
    source_path: document.path,
    output_path: target.fsPath,
    format: formatForPath(target.fsPath),
  });
  if (result.success) {
    void vscode.window.showInformationMessage(
      `Strixonomy: exported ${path.basename(result.output_path)}`
    );
  } else {
    const detail = result.logs?.trim();
    void vscode.window.showErrorMessage(
      detail
        ? `Strixonomy: export failed — ${detail.slice(0, 300)}`
        : `Strixonomy: export failed for ${path.basename(result.output_path)}`
    );
  }
}

async function copyFocused(shortForm: boolean): Promise<void> {
  const iri = getFocusedEntityIri();
  if (!iri) {
    void vscode.window.showWarningMessage("Strixonomy: no entity is selected");
    return;
  }
  const value = shortForm ? iri.slice(Math.max(iri.lastIndexOf("#"), iri.lastIndexOf("/")) + 1) : iri;
  await vscode.env.clipboard.writeText(value);
}

async function pickOntologyDocument(title: string): Promise<OntologyDocument | undefined> {
  await ontologyRegistry.syncFromCatalog();
  const snapshot = await getCatalogSnapshot();
  const active = ontologyRegistry.getActiveId() ?? snapshot.active_ontology_id;
  if (snapshot.documents.length === 1) return snapshot.documents[0];
  const picked = await vscode.window.showQuickPick(
    snapshot.documents.map((document) => ({
      label: document.base_iri ?? path.basename(document.path),
      description: document.format,
      detail: document.path,
      document,
    })),
    { title, placeHolder: active ? `Active: ${active}` : undefined }
  );
  return picked?.document;
}

async function applyDocumentPatches(
  document: OntologyDocument,
  patches: PatchOp[]
): Promise<void> {
  const documentUri = documentUriInWorkspace(document.path);
  if (!documentUri) {
    throw new Error(`document path is outside the workspace: ${document.path}`);
  }
  const result = await workspaceTransactionManager.apply(
    documentUri,
    document.path,
    patches
  );
  requirePatchFullySynced(result);
}

async function runEntityRefactor(
  context: vscode.ExtensionContext,
  kind: "merge_entities" | "replace_entity",
  refresh?: () => Promise<void>
): Promise<void> {
  const focused = getFocusedEntityIri();
  const from = focused ?? (await requiredInput(kind === "merge_entities" ? "Entity to merge" : "Entity to replace"));
  if (!from) return;
  const to = await requiredInput(
    kind === "merge_entities" ? "Entity to keep" : "Replacement entity IRI"
  );
  if (!to || to === from) return;
  const request: RefactorRequest =
    kind === "merge_entities"
      ? { kind, keep_iri: to, merge_iri: from }
      : { kind, from_iri: from, to_iri: to };
  const plan = await previewRefactor(request);
  await RefactorPreviewPanel.show(context.extensionUri, plan, request, refresh);
}

async function runReasoning(
  context: vscode.ExtensionContext,
  action: string
): Promise<RunReasonerResult | undefined> {
  const config = vscode.workspace.getConfiguration("strixonomy");
  const profile = config.get<string>("reasoner.default", "el");
  const autoDetect = config.get<boolean>("reasoner.autoProfile", true);
  const titles: Record<string, string> = {
    start: "Strixonomy: Starting reasoner",
    synchronize: "Strixonomy: Synchronizing reasoner",
    classify: "Strixonomy: Classifying ontology",
    consistency: "Strixonomy: Checking consistency",
  };
  const title = titles[action] ?? "Strixonomy: Running reasoner";

  if (action === "start") {
    ReasonerPanel.show(context.extensionUri);
  }

  return vscode.window.withProgress(
    {
      location: vscode.ProgressLocation.Notification,
      title,
      cancellable: true,
    },
    async (progress, token) => {
      const preRun = captureReasoningPreRun(focusRelay.getReasoning());
      focusRelay.setReasoningState(reasoningStateForRunStart(profile, preRun));
      try {
        if (action === "synchronize") {
          progress.report({ message: "Reindexing workspace…" });
          await indexWorkspace();
          if (token.isCancellationRequested) {
            cancelActiveReasonerRequest();
            focusRelay.setReasoningState(reasoningStateForRunCancel(preRun));
            return undefined;
          }
        }
        progress.report({
          message:
            action === "consistency" ? "Checking consistency…" : "Classifying…",
        });
        const result = await runReasoner(
          { profile, auto_detect: autoDetect },
          token
        );
        if (token.isCancellationRequested) {
          ReasonerPanel.current?.cancelActiveRun();
          focusRelay.setReasoningState(reasoningStateForRunCancel(preRun));
          return undefined;
        }
        focusRelay.setReasoningState(
          reasoningStateForRunSuccess(
            result.profile_used,
            result.unsatisfiable,
            preRun
          )
        );
        if (action === "start" || action === "classify" || action === "synchronize") {
          const panel = ReasonerPanel.show(context.extensionUri);
          panel.presentResult(result);
        }
        void vscode.commands.executeCommand("strixonomy.refreshExplorer");
        return result;
      } catch (error) {
        if (token.isCancellationRequested) {
          focusRelay.setReasoningState(reasoningStateForRunCancel(preRun));
          return undefined;
        }
        focusRelay.setReasoningRunning(false);
        throw error;
      }
    }
  );
}

const PANEL_CHOICES = [
  { label: "Entity Inspector", value: "inspector", commandSuffix: "InspectorPanel" },
  { label: "Query Workbench", value: "query", commandSuffix: "QueryPanel" },
  { label: "Reasoner", value: "reasoner", commandSuffix: "ReasonerPanel" },
  { label: "Explanation", value: "explanation", commandSuffix: "ExplanationPanel" },
  { label: "Graph", value: "graph", commandSuffix: "GraphPanel" },
  { label: "Semantic Diff", value: "semanticDiff", commandSuffix: "SemanticDiffPanel" },
  { label: "Imports", value: "imports", commandSuffix: "ImportsPanel" },
] as const;

async function openPerspective(
  context: vscode.ExtensionContext,
  perspective: Perspective
): Promise<void> {
  for (const panel of perspective.panels) await openPanel(context, panel);
}

async function openPanel(
  context: vscode.ExtensionContext,
  panel: string
): Promise<void> {
  if (panel === "reasoner") {
    ReasonerPanel.show(context.extensionUri);
    return;
  }
  const commandByPanel: Record<string, string> = {
    inspector: "strixonomy.showEntityInspector",
    query: "strixonomy.openQueryWorkbench",
    explanation: "strixonomy.showExplanation",
    graph: "strixonomy.openClassGraph",
    semanticDiff: "strixonomy.semanticDiff",
    imports: "strixonomy.manageImports",
  };
  const command = commandByPanel[panel];
  if (command) await vscode.commands.executeCommand(command);
  else void vscode.window.showWarningMessage(`Strixonomy: unknown panel "${panel}"`);
}

async function requiredInput(
  prompt: string,
  value?: string
): Promise<string | undefined> {
  return vscode.window.showInputBox({
    prompt,
    value,
    ignoreFocusOut: true,
    validateInput: (input) => (input.trim() ? undefined : `${prompt} is required`),
  }).then((input) => input?.trim() || undefined);
}

function defaultWorkspaceUri(fileName: string): vscode.Uri | undefined {
  const folder = vscode.workspace.workspaceFolders?.[0];
  return folder ? vscode.Uri.joinPath(folder.uri, fileName) : undefined;
}

function formatForPath(filePath: string): string | undefined {
  const extension = path.extname(filePath).slice(1).toLowerCase();
  const formats: Record<string, string> = {
    ttl: "turtle",
    owl: "rdfxml",
    rdf: "rdfxml",
    jsonld: "jsonld",
    "json-ld": "jsonld",
    nt: "ntriples",
    nq: "nquads",
    trig: "trig",
    obo: "obo",
  };
  return formats[extension];
}
