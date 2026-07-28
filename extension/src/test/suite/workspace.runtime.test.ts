import * as assert from "assert";
import * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/node";
import { FIXTURE_IRIS, fixturesWorkspaceUri } from "./helpers";

interface StrixonomyApi {
  getClient(): LanguageClient | undefined;
  indexWorkspace(workspaceUri?: string): Promise<unknown>;
  getCatalogSnapshot(): Promise<{ documents: Array<{ id: string; path: string }> }>;
  __test?: {
    getOntologyRegistrySnapshot(): Array<{
      id: string;
      path: string;
      editable: boolean;
      dirty: boolean;
      active: boolean;
    }>;
    getNavigationStack(): Array<{ kind: string; id: string; source: string }>;
    getCanonicalFocus(): { kind: string; id: string } | null;
    settle(ms?: number): Promise<void>;
  };
}

suite("Strixonomy workspace runtime (VS Code e2e)", () => {
  let api: StrixonomyApi;

  suiteSetup(async function () {
    this.timeout(120_000);
    const ext = vscode.extensions.getExtension("strixonomy.strixonomy");
    assert.ok(ext, "Strixonomy extension must be loaded");
    api = await ext.activate();
    assert.ok(ext.isActive);
    assert.ok(api.__test, "test hooks require STRIXONOMY_TEST_FIXTURES");
  });

  test("registry tracks indexed ontologies after indexWorkspace", async function () {
    this.timeout(60_000);
    await api.indexWorkspace(fixturesWorkspaceUri());
    await vscode.commands.executeCommand("strixonomy.refreshExplorer");
    const registry = api.__test!.getOntologyRegistrySnapshot();
    assert.ok(registry.length > 0, "registry should list open ontologies");
    assert.ok(
      registry.some((e) => e.editable),
      "fixture workspace should include editable ontologies"
    );
  });

  test("entity focus updates navigation stack", async function () {
    this.timeout(60_000);
    await api.indexWorkspace(fixturesWorkspaceUri());
    await vscode.commands.executeCommand("strixonomy.openEntity", FIXTURE_IRIS.person);
    await api.__test!.settle(500);
    const stack = api.__test!.getNavigationStack();
    assert.ok(
      stack.some((e) => e.id === FIXTURE_IRIS.person),
      "navigation stack should record focused entity"
    );
    const focus = api.__test!.getCanonicalFocus();
    assert.equal(focus?.id, FIXTURE_IRIS.person);
  });
});
