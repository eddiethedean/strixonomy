import * as assert from "assert";
import * as vscode from "vscode";
import { FIXTURE_IRIS, fixturesWorkspaceUri } from "./helpers";

interface StrixonomyTestHooks {
  openEntity(iri: string): Promise<void>;
  waitForInspectorIri(iri: string, timeoutMs?: number): Promise<void>;
  disposeAllPanels(): Promise<void>;
  getCanonicalFocus(): { kind: string; id: string; source?: string } | null;
}

interface StrixonomyTestApi {
  indexWorkspace(workspaceUri?: string): Promise<unknown>;
  __test: StrixonomyTestHooks;
}

suite("Focus relay sync (VS Code e2e)", () => {
  let api: StrixonomyTestApi;

  suiteSetup(async function () {
    this.timeout(120_000);
    const ext = vscode.extensions.getExtension("strixonomy.strixonomy");
    assert.ok(ext, "Strixonomy extension must be loaded");
    const activated = await ext.activate();
    assert.ok(activated.__test, "ONTOCODE_TEST_FIXTURES must enable __test hooks");
    api = activated as StrixonomyTestApi;
  });

  setup(async () => {
    await api.indexWorkspace(fixturesWorkspaceUri());
    await api.__test.disposeAllPanels();
  });

  suiteTeardown(async () => {
    await api.__test.disposeAllPanels();
  });

  test("explorer openEntity updates canonical focus relay", async function () {
    this.timeout(60_000);
    const { organization } = FIXTURE_IRIS;
    await api.__test.openEntity(organization);
    await api.__test.waitForInspectorIri(organization);
    const focus = api.__test.getCanonicalFocus();
    assert.equal(focus?.kind, "entity");
    assert.equal(focus?.id, organization);
    assert.equal(focus?.source, "explorer");
  });
});
