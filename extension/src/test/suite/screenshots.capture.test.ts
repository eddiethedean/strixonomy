import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/node";
import { FIXTURE_IRIS, fixturesWorkspaceUri } from "./helpers";

interface StrixonomyTestHooks {
  openEntityInspector(iri: string): Promise<void>;
  waitForInspectorReady(timeoutMs?: number): Promise<void>;
  openQueryWorkbench(): Promise<void>;
  waitForQueryWorkbenchReady(timeoutMs?: number): Promise<void>;
  postWebviewMessage(panel: string, msg: unknown): Promise<void>;
  openReasonerPanel(): Promise<void>;
  runReasonerDefaults(): Promise<void>;
  waitForPanelReady(panel: string, timeoutMs?: number): Promise<void>;
  openSemanticDiff(leftRef: string, rightRef: string): Promise<void>;
  captureScreenshot(name: string): Promise<void>;
  settle(ms?: number): Promise<void>;
  disposeAllPanels(): Promise<void>;
}

interface StrixonomyTestApi {
  getClient(): LanguageClient | undefined;
  indexWorkspace(workspaceUri?: string): Promise<{
    stats: { error_count: number; class_count: number };
  }>;
  __test: StrixonomyTestHooks;
}

const CAPTURE = process.env.ONTOCODE_CAPTURE_SCREENSHOTS === "1";

suite("Strixonomy screenshot capture", function () {
  if (!CAPTURE) {
    // Keep normal e2e fast; enable via scripts/capture-screenshots.sh
    return;
  }

  this.timeout(180_000);

  let api: StrixonomyTestApi;

  suiteSetup(async function () {
    this.timeout(180_000);
    const ext = vscode.extensions.getExtension("strixonomy.strixonomy");
    assert.ok(ext, "Strixonomy extension must be loaded");
    const activated = await ext.activate();
    assert.ok(activated.__test, "ONTOCODE_TEST_FIXTURES must enable __test hooks");
    api = activated as StrixonomyTestApi;

    const client = api.getClient();
    assert.ok(client, "language client should start");
    // Activation already starts the client; wait until it reports running.
    const deadline = Date.now() + 60_000;
    while (Date.now() < deadline) {
      if (client.isRunning?.() ?? true) {
        break;
      }
      await new Promise((r) => setTimeout(r, 200));
    }

    const workspaceUri = fixturesWorkspaceUri();
    const indexed = await api.indexWorkspace(workspaceUri);
    assert.ok(
      indexed.stats.class_count > 0,
      "screenshot workspace should index classes"
    );

    // Prefer a real editor + Strixonomy activity bar for marketing shots.
    await vscode.commands.executeCommand("workbench.view.extension.strixonomy");
    const folders = vscode.workspace.workspaceFolders;
    assert.ok(folders && folders.length > 0, "workspace folder required");
    const ttl = vscode.Uri.joinPath(folders[0].uri, "example.ttl");
    if (fs.existsSync(ttl.fsPath)) {
      const doc = await vscode.workspace.openTextDocument(ttl);
      await vscode.window.showTextDocument(doc, {
        viewColumn: vscode.ViewColumn.One,
        preview: false,
      });
    }
    await api.__test.settle(1200);
  });

  suiteTeardown(async () => {
    if (api?.__test) {
      await api.__test.disposeAllPanels();
    }
  });

  test("capture explorer + entity inspector", async () => {
    await api.__test.disposeAllPanels();
    await vscode.commands.executeCommand("workbench.view.extension.strixonomy");
    await api.__test.openEntityInspector(FIXTURE_IRIS.person);
    await api.__test.waitForInspectorReady();
    await api.__test.settle(2000);
    await api.__test.captureScreenshot("explorer-inspector");
  });

  test("capture query workbench with results", async () => {
    await api.__test.disposeAllPanels();
    await api.__test.openQueryWorkbench();
    await api.__test.waitForQueryWorkbenchReady();
    await api.__test.postWebviewMessage("queryWorkbench", {
      type: "runQuery",
      mode: "sql",
      text: "SELECT short_name, iri FROM classes",
      runId: 1,
    });
    await api.__test.settle(2500);
    await api.__test.captureScreenshot("query-workbench");
  });

  test("capture reasoner panel with results", async () => {
    await api.__test.disposeAllPanels();
    await api.__test.openReasonerPanel();
    await api.__test.waitForPanelReady("reasoner");
    await api.__test.runReasonerDefaults();
    await api.__test.settle(2000);
    await api.__test.captureScreenshot("reasoner");
  });

  test("capture semantic diff HEAD vs WORKTREE", async () => {
    await api.__test.disposeAllPanels();
    await api.__test.openSemanticDiff("HEAD", "WORKTREE");
    await api.__test.waitForPanelReady("semanticDiff");
    // Allow loading → data/error to paint
    await api.__test.settle(3000);
    await api.__test.captureScreenshot("semantic-diff");
  });

  test("assemble product-tour.gif from captured frames", async () => {
    const dir = process.env.ONTOCODE_SCREENSHOT_DIR;
    assert.ok(dir, "ONTOCODE_SCREENSHOT_DIR required");
    const frames = [
      "explorer-inspector.png",
      "query-workbench.png",
      "reasoner.png",
      "semantic-diff.png",
    ].map((f) => path.join(dir, f));
    for (const f of frames) {
      assert.ok(fs.existsSync(f), `missing frame ${f}`);
    }
    const out = path.join(dir, "product-tour.gif");
    // out/test/suite → repo root is four levels up
    const script = path.resolve(
      __dirname,
      "..",
      "..",
      "..",
      "..",
      "scripts",
      "assemble-product-tour-gif.py"
    );
    assert.ok(fs.existsSync(script), `missing ${script}`);
    const { execFile } = await import("child_process");
    const { promisify } = await import("util");
    const execFileAsync = promisify(execFile);
    await execFileAsync("python3", [script, out, ...frames], {
      timeout: 60_000,
    });
    assert.ok(fs.existsSync(out), "product-tour.gif was not written");
  });
});
