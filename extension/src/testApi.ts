import { execFile } from "child_process";
import * as path from "path";
import { promisify } from "util";
import * as vscode from "vscode";
import type {
  DialogPanelKind,
  InjectablePanelKind,
  StrixonomyTestHooks,
} from "./api";
import {
  openNewOntologyDialog as openNewOntologyDialogAt,
  openPrefixManager as openPrefixManagerAt,
} from "./commands/v017Commands";
import { focusRelay } from "./focus/focusRelay";
import { navigationManager, ontologyRegistry } from "./workspace";
import { EntityInspectorPanel } from "./webviews/inspector";
import { PanelHost } from "./webviews/panelHost";
import { QueryWorkbenchPanel } from "./webviews/queryWorkbenchReact";
import { ReasonerPanel } from "./webviews/reasonerPanel";
import { SemanticDiffPanel } from "./webviews/semanticDiffPanel";
import { assertWebviewHtmlRoutesPanel } from "./webviews/webviewBootstrap";
import type { PanelKind } from "./webviews/messages";

const execFileAsync = promisify(execFile);

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForWebviewReady(
  isReady: () => boolean,
  timeoutMs: number
): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (isReady()) {
      return;
    }
    await sleep(100);
  }
  throw new Error("webview did not report ready before timeout");
}

function requireHost(
  panel: DialogPanelKind | "reasoner" | "semanticDiff" | "queryWorkbench"
): PanelHost {
  const host = PanelHost.getOpen(panel);
  if (!host || host.isDisposed) {
    throw new Error(`${panel} webview is not open`);
  }
  return host;
}

function extensionUri(): vscode.Uri {
  const ext = vscode.extensions.getExtension("strixonomy.strixonomy");
  if (!ext) {
    throw new Error("Strixonomy extension is not loaded");
  }
  return ext.extensionUri;
}

/** Test hooks exposed when ONTOCODE_TEST_FIXTURES is set (VS Code e2e). */
export function createStrixonomyTestHooks(): StrixonomyTestHooks {
  return {
    async openEntityInspector(iri: string): Promise<void> {
      await vscode.commands.executeCommand("strixonomy.showEntityInspector", iri);
    },

    getInspectorWebviewHtml(): string | undefined {
      return EntityInspectorPanel.currentPanel?.getWebviewHtmlForTests();
    },

    assertInspectorHtmlRoutesPanel(): void {
      const html = EntityInspectorPanel.currentPanel?.getWebviewHtmlForTests();
      if (!html) {
        throw new Error("entity inspector webview is not open");
      }
      assertWebviewHtmlRoutesPanel(html, "inspector");
    },

    async waitForInspectorReady(timeoutMs = 20_000): Promise<void> {
      const panel = EntityInspectorPanel.currentPanel;
      if (!panel) {
        throw new Error("entity inspector webview is not open");
      }
      await waitForWebviewReady(
        () => panel.isWebviewReadyForTests(),
        timeoutMs
      );
    },

    async openQueryWorkbench(): Promise<void> {
      await vscode.commands.executeCommand("strixonomy.openQueryWorkbench");
    },

    getQueryWorkbenchWebviewHtml(): string | undefined {
      return QueryWorkbenchPanel.current?.getWebviewHtmlForTests();
    },

    assertQueryWorkbenchHtmlRoutesPanel(): void {
      const html = QueryWorkbenchPanel.current?.getWebviewHtmlForTests();
      if (!html) {
        throw new Error("query workbench webview is not open");
      }
      assertWebviewHtmlRoutesPanel(html, "queryWorkbench");
    },

    async waitForQueryWorkbenchReady(timeoutMs = 20_000): Promise<void> {
      const panel = QueryWorkbenchPanel.current;
      if (!panel) {
        throw new Error("query workbench webview is not open");
      }
      await waitForWebviewReady(
        () => panel.isWebviewReadyForTests(),
        timeoutMs
      );
    },

    async openEntity(iri: string): Promise<void> {
      await vscode.commands.executeCommand("strixonomy.openEntity", iri);
    },

    getInspectorLoadedIri(): string | undefined {
      return EntityInspectorPanel.currentPanel?.getLoadedIriForTests();
    },

    getInspectorPanelTitle(): string | undefined {
      return EntityInspectorPanel.currentPanel?.getPanelTitleForTests();
    },

    async waitForInspectorIri(iri: string, timeoutMs = 20_000): Promise<void> {
      const deadline = Date.now() + timeoutMs;
      while (Date.now() < deadline) {
        if (EntityInspectorPanel.currentPanel?.getLoadedIriForTests() === iri) {
          return;
        }
        await sleep(100);
      }
      const loaded = EntityInspectorPanel.currentPanel?.getLoadedIriForTests();
      throw new Error(
        `inspector did not load ${iri} before timeout (last IRI: ${loaded ?? "none"})`
      );
    },

    getInspectorPanelRef(): object | undefined {
      return EntityInspectorPanel.currentPanel;
    },

    getCanonicalFocus() {
      return focusRelay.getFocus();
    },

    async openNewOntologyDialog(targetPath: string): Promise<void> {
      openNewOntologyDialogAt(targetPath);
    },

    async openPrefixManager(documentPath: string): Promise<void> {
      await openPrefixManagerAt(documentPath);
    },

    async postWebviewMessage(
      panel: InjectablePanelKind,
      msg: unknown
    ): Promise<void> {
      const host = PanelHost.getOpen(panel);
      if (!host || host.isDisposed) {
        throw new Error(`${panel} webview is not open`);
      }
      await host.deliverMessageForTests(msg);
    },

    async waitForPanelReady(
      panel: DialogPanelKind | "reasoner" | "semanticDiff" | "queryWorkbench",
      timeoutMs = 20_000
    ): Promise<void> {
      const host = requireHost(panel);
      await waitForWebviewReady(() => host.isWebviewReady(), timeoutMs);
    },

    getPanelHtml(panel: DialogPanelKind): string | undefined {
      return PanelHost.getOpen(panel)?.getWebviewHtml();
    },

    async disposePanel(panel: DialogPanelKind): Promise<void> {
      PanelHost.disposeKinds([panel]);
      await sleep(50);
    },

    async disposeAllPanels(): Promise<void> {
      EntityInspectorPanel.currentPanel?.disposeForTests();
      QueryWorkbenchPanel.current?.disposeForTests();
      ReasonerPanel.current?.dispose();
      SemanticDiffPanel.current?.dispose();
      PanelHost.disposeKinds(["newOntology", "prefixManager"]);
      await sleep(100);
    },

    async openReasonerPanel(): Promise<void> {
      ReasonerPanel.show(extensionUri());
    },

    async runReasonerDefaults(): Promise<void> {
      const panel = ReasonerPanel.current ?? ReasonerPanel.show(extensionUri());
      await panel.runWithDefaults();
    },

    async openSemanticDiff(leftRef: string, rightRef: string): Promise<void> {
      await SemanticDiffPanel.show(extensionUri(), { leftRef, rightRef });
    },

    async captureScreenshot(name: string): Promise<void> {
      const script =
        process.env.STRIXONOMY_CAPTURE_SCRIPT ??
        process.env.ONTOCODE_CAPTURE_SCRIPT;
      const dir =
        process.env.STRIXONOMY_SCREENSHOT_DIR ??
        process.env.ONTOCODE_SCREENSHOT_DIR;
      if (!script || !dir) {
        throw new Error(
          "STRIXONOMY_CAPTURE_SCRIPT and STRIXONOMY_SCREENSHOT_DIR must be set"
        );
      }
      const safe = name.replace(/[^a-zA-Z0-9._-]/g, "_");
      const out = path.join(dir, `${safe}.png`);
      await execFileAsync(script, [out], { timeout: 30_000 });
    },

    async settle(ms = 800): Promise<void> {
      await sleep(ms);
    },

    getOntologyRegistrySnapshot() {
      return ontologyRegistry.getSnapshot().map((entry) => ({
        id: entry.id,
        path: entry.path,
        editable: entry.editable,
        dirty: entry.dirty,
        active: entry.active,
      }));
    },

    getNavigationStack() {
      return navigationManager.getStack();
    },

    assertWebviewHtmlRoutesPanel(html: string, panel: PanelKind): void {
      assertWebviewHtmlRoutesPanel(html, panel);
    },
  };
}
