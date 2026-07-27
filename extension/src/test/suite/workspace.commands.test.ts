import * as assert from "assert";
import * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/node";
import { FIXTURE_IRIS, fixturesWorkspaceUri } from "./helpers";

interface StrixonomyApi {
  getClient(): LanguageClient | undefined;
  indexWorkspace(workspaceUri?: string): Promise<{
    stats: { error_count: number; class_count: number };
  }>;
  getCatalogSnapshot(): Promise<{
    entities: Array<{ iri: string; kind: string }>;
    documents: Array<{ path: string }>;
  }>;
  getEntity(iri: string): Promise<{
    detail: { entity: { iri: string; labels: string[] } };
  }>;
}

suite("Strixonomy workspace commands (VS Code e2e)", () => {
  let api: StrixonomyApi;

  suiteSetup(async function () {
    this.timeout(120_000);
    const ext = vscode.extensions.getExtension("strixonomy.strixonomy");
    assert.ok(ext, "Strixonomy extension must be loaded");
    api = await ext.activate();
    assert.ok(ext.isActive, "extension should be active after activate()");
  });

  test("indexWorkspace command indexes fixture ontology files", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();

    await vscode.commands.executeCommand("strixonomy.indexWorkspace");

    const snapshot = await api.getCatalogSnapshot();
    assert.ok(snapshot.documents.length > 0, "catalog should list ontology documents");
    assert.ok(
      snapshot.entities.some((e) => e.iri === FIXTURE_IRIS.person),
      "Person class should be indexed"
    );
  });

  test("refreshExplorer after index returns catalog without NOT_INDEXED error", async function () {
    this.timeout(60_000);
    await api.indexWorkspace(fixturesWorkspaceUri());

    await vscode.commands.executeCommand("strixonomy.refreshExplorer");

    const snapshot = await api.getCatalogSnapshot();
    assert.ok(
      snapshot.entities.some((e) => e.iri === FIXTURE_IRIS.organization),
      "refresh should succeed with indexed catalog"
    );
  });

  test("getEntity returns detail for indexed fixture class", async function () {
    this.timeout(60_000);
    await api.indexWorkspace(fixturesWorkspaceUri());
    const { detail } = await api.getEntity(FIXTURE_IRIS.person);
    assert.equal(detail.entity.iri, FIXTURE_IRIS.person);
    assert.ok(
      detail.entity.labels.some((l) => /person/i.test(l)),
      "entity detail should include Person label"
    );
  });
});
