import * as assert from "assert";
import * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/node";
import { FIXTURE_IRIS, fixturesWorkspaceUri } from "./helpers";

interface StrixonomyTestHooks {
  openEntityInspector(iri: string): Promise<void>;
  openEntity(iri: string): Promise<void>;
  getInspectorWebviewHtml(): string | undefined;
  assertInspectorHtmlRoutesPanel(): void;
  waitForInspectorReady(timeoutMs?: number): Promise<void>;
  waitForInspectorIri(iri: string, timeoutMs?: number): Promise<void>;
  openQueryWorkbench(): Promise<void>;
  getQueryWorkbenchWebviewHtml(): string | undefined;
  assertQueryWorkbenchHtmlRoutesPanel(): void;
  waitForQueryWorkbenchReady(timeoutMs?: number): Promise<void>;
  disposeAllPanels(): Promise<void>;
}

interface StrixonomyTestApi {
  getClient(): LanguageClient | undefined;
  indexWorkspace(workspaceUri?: string): Promise<{
    stats: { error_count: number; class_count: number };
  }>;
  __test: StrixonomyTestHooks;
}

suite("Strixonomy React webviews", () => {
  let api: StrixonomyTestApi;

  suiteSetup(async function () {
    this.timeout(120_000);
    const ext = vscode.extensions.getExtension("strixonomy.strixonomy");
    assert.ok(ext, "Strixonomy extension must be loaded");
    const activated = await ext.activate();
    assert.ok(activated.__test, "ONTOCODE_TEST_FIXTURES must enable __test hooks");
    api = activated as StrixonomyTestApi;
  });

  suiteTeardown(async () => {
    await api.__test.disposeAllPanels();
  });

  test("entity inspector HTML routes React to inspector panel", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();
    const personIri = FIXTURE_IRIS.person;

    await api.indexWorkspace(workspaceUri);
    await api.__test.openEntityInspector(personIri);

    const html = api.__test.getInspectorWebviewHtml();
    assert.ok(html, "inspector webview should be open");
    api.__test.assertInspectorHtmlRoutesPanel();
    assert.match(html!, /panel=inspector/, "bootstrap must include panel=inspector");
    assert.doesNotMatch(
      html!,
      /<script[^>]+src="[^"]*\?[^"]*panel=/,
      "panel query must not live only on script src"
    );
  });

  test("entity inspector webview reports ready for inspector panel", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();
    const personIri = FIXTURE_IRIS.person;

    await api.indexWorkspace(workspaceUri);
    await api.__test.disposeAllPanels();
    await api.__test.openEntityInspector(personIri);
    await api.__test.waitForInspectorReady();
  });

  test("query workbench HTML routes React to queryWorkbench panel", async function () {
    this.timeout(60_000);
    await api.__test.disposeAllPanels();
    await api.__test.openQueryWorkbench();

    const html = api.__test.getQueryWorkbenchWebviewHtml();
    assert.ok(html, "query workbench webview should be open");
    api.__test.assertQueryWorkbenchHtmlRoutesPanel();
  });

  test("query workbench webview reports ready for queryWorkbench panel", async function () {
    this.timeout(60_000);
    await api.__test.disposeAllPanels();
    await api.__test.openQueryWorkbench();
    await api.__test.waitForQueryWorkbenchReady();
  });

  test("second entity navigation keeps inspector panel routing", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();

    await api.indexWorkspace(workspaceUri);
    await api.__test.openEntityInspector(FIXTURE_IRIS.person);
    await api.__test.waitForInspectorReady();
    await api.__test.openEntity(FIXTURE_IRIS.organization);
    await api.__test.waitForInspectorIri(FIXTURE_IRIS.organization);

    api.__test.assertInspectorHtmlRoutesPanel();
    const html = api.__test.getInspectorWebviewHtml();
    assert.ok(html);
    assert.doesNotMatch(
      html!,
      /SmokePanel|webview foundation is active/i,
      "inspector must not fall back to smoke panel after entity switch"
    );
  });
});
