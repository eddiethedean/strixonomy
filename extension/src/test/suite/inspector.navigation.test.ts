import * as assert from "assert";
import * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/node";
import { FIXTURE_IRIS, fixturesWorkspaceUri } from "./helpers";

interface StrixonomyTestHooks {
  openEntityInspector(iri: string): Promise<void>;
  openEntity(iri: string): Promise<void>;
  waitForInspectorReady(timeoutMs?: number): Promise<void>;
  waitForInspectorIri(iri: string, timeoutMs?: number): Promise<void>;
  getInspectorLoadedIri(): string | undefined;
  getInspectorPanelTitle(): string | undefined;
  getInspectorPanelRef(): object | undefined;
  disposeAllPanels(): Promise<void>;
}

interface StrixonomyTestApi {
  getClient(): LanguageClient | undefined;
  indexWorkspace(workspaceUri?: string): Promise<{
    stats: { error_count: number; class_count: number };
  }>;
  getCatalogSnapshot(): Promise<{
    entities: Array<{ iri: string; kind: string }>;
  }>;
  __test: StrixonomyTestHooks;
}

suite("Entity Inspector navigation (VS Code e2e)", () => {
  let api: StrixonomyTestApi;

  suiteSetup(async function () {
    this.timeout(120_000);
    const ext = vscode.extensions.getExtension("strixonomy.strixonomy");
    assert.ok(ext, "Strixonomy extension must be loaded");
    const activated = await ext.activate();
    assert.ok(activated.__test, "STRIXONOMY_TEST_FIXTURES must enable __test hooks");
    api = activated as StrixonomyTestApi;
  });

  setup(async () => {
    await api.indexWorkspace(fixturesWorkspaceUri());
    await api.__test.disposeAllPanels();
  });

  suiteTeardown(async () => {
    await api.__test.disposeAllPanels();
  });

  test("reuses the same inspector panel when opening a second entity", async function () {
    this.timeout(60_000);
    const { person, organization } = FIXTURE_IRIS;

    await api.__test.openEntityInspector(person);
    await api.__test.waitForInspectorReady();
    await api.__test.waitForInspectorIri(person);
    const panelRef = api.__test.getInspectorPanelRef();
    assert.ok(panelRef, "inspector panel should exist");

    await api.__test.openEntity(organization);
    await api.__test.waitForInspectorIri(organization);

    assert.strictEqual(
      api.__test.getInspectorPanelRef(),
      panelRef,
      "second entity should reuse the existing inspector panel"
    );
    assert.equal(api.__test.getInspectorLoadedIri(), organization);
    assert.match(
      api.__test.getInspectorPanelTitle() ?? "",
      /Organization/i,
      "panel title should reflect the new entity"
    );
  });

  test("navigates class → property → individual in one inspector", async function () {
    this.timeout(60_000);
    const { person, worksFor, alice } = FIXTURE_IRIS;

    await api.__test.openEntityInspector(person);
    await api.__test.waitForInspectorReady();
    await api.__test.waitForInspectorIri(person);

    await api.__test.openEntity(worksFor);
    await api.__test.waitForInspectorIri(worksFor);
    assert.match(api.__test.getInspectorPanelTitle() ?? "", /works for|worksFor/i);

    await api.__test.openEntity(alice);
    await api.__test.waitForInspectorIri(alice);
    assert.match(api.__test.getInspectorPanelTitle() ?? "", /Alice/i);
  });

  test("rapid sequential opens resolve to the latest entity", async function () {
    this.timeout(60_000);
    const { person, organization, worksFor } = FIXTURE_IRIS;

    await Promise.all([
      api.__test.openEntityInspector(person),
      api.__test.openEntity(organization),
      api.__test.openEntity(worksFor),
    ]);

    await api.__test.waitForInspectorIri(worksFor);
    assert.equal(api.__test.getInspectorLoadedIri(), worksFor);
  });

  test("openEntity accepts plain IRI from explorer tree click", async function () {
    this.timeout(60_000);
    const { organization } = FIXTURE_IRIS;

    await vscode.commands.executeCommand("strixonomy.openEntity", organization);

    await api.__test.waitForInspectorIri(organization);
    assert.equal(api.__test.getInspectorLoadedIri(), organization);
  });

  test("openEntity accepts tree item object from context menu", async function () {
    this.timeout(60_000);
    const { organization } = FIXTURE_IRIS;

    await vscode.commands.executeCommand("strixonomy.openEntity", {
      iri: organization,
      label: "Organization",
    });

    await api.__test.waitForInspectorIri(organization);
    assert.equal(api.__test.getInspectorLoadedIri(), organization);
  });
});
