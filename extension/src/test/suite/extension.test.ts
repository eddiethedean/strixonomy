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
  }>;
  getEntity(iri: string): Promise<{
    detail: {
      axioms: Array<{ kind: string; display: string; manchester?: string }>;
    };
  }>;
  runSqlQuery(sql: string): Promise<{ columns: string[]; rows: Array<Record<string, string>> }>;
  runSparqlQuery(query: string): Promise<{ columns: string[]; rows: Array<Record<string, string>> }>;
  parseManchester(
    expression: string,
    axiomKind: string,
    entityIri?: string,
    documentUri?: string
  ): Promise<{
    normalized: string;
    diagnostics: Array<{ message: string }>;
    tree: unknown;
    completions: { classes: string[] };
  }>;
}

suite("Strixonomy in VS Code", () => {
  let api: StrixonomyApi;

  suiteSetup(async function () {
    this.timeout(120_000);
    const ext = vscode.extensions.getExtension("strixonomy.strixonomy");
    assert.ok(ext, "Strixonomy extension must be loaded");
    api = await ext.activate();
    assert.ok(ext.isActive, "extension should be active after activate()");
  });

  test("language client starts (bundled strixonomy-lsp)", () => {
    assert.ok(api.getClient(), "LSP client should be running");
    assert.ok(api.getClient()!.isRunning?.() ?? true, "client should report running");
  });

  test("indexes fixture workspace and returns Person in catalog", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();

    const result = await api.indexWorkspace(workspaceUri);
    assert.equal(result.stats.error_count, 0);
    // fixtures/ currently ships 18 classes across example + reasoner + clinic ontologies
    assert.ok(
      result.stats.class_count >= 15,
      `expected at least 15 classes in fixtures, got ${result.stats.class_count}`
    );

    const snapshot = await api.getCatalogSnapshot();
    const person = snapshot.entities.find(
      (e) => e.iri === FIXTURE_IRIS.person
    );
    assert.ok(person, "Person class should be in catalog");
    assert.equal(person?.kind, "class");
    assert.ok(
      snapshot.entities.some((e) => e.iri.includes("reasoner-el#Dog")),
      "Dog from reasoner-el fixture must be indexed"
    );
  });

  test("runs SQL query against indexed workspace", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();
    await api.indexWorkspace(workspaceUri);

    const result = await api.runSqlQuery("SELECT short_name FROM classes");
    assert.ok(result.columns.includes("short_name"));
    assert.ok(result.rows.length > 0);
    assert.ok(
      result.rows.some((row) => row.short_name === "Person"),
      "Person class should appear in SQL results"
    );
  });

  test("runs SPARQL query against indexed workspace", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();
    await api.indexWorkspace(workspaceUri);

    const result = await api.runSparqlQuery(
      "SELECT ?s WHERE { ?s a <http://www.w3.org/2002/07/owl#Class> } LIMIT 5"
    );
    assert.ok(result.rows.length > 0);
  });

  test("parses Manchester expression via LSP", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();
    await api.indexWorkspace(workspaceUri);

    const parsed = await api.parseManchester(
      "ex:hasRecord some ex:MedicalRecord",
      "sub_class_of",
      "http://example.org/clinic#Patient"
    );
    assert.equal(parsed.diagnostics.length, 0);
    assert.ok(parsed.normalized.includes("some"));
    assert.ok(parsed.completions.classes.length > 0);
  });

  test("returns structured axioms for complex subclass", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();
    await api.indexWorkspace(workspaceUri);

    const entity = await api.getEntity("http://example.org/clinic#Patient");
    const complex = entity.detail.axioms.find((a) => a.manchester);
    assert.ok(complex, "Patient should expose a Manchester axiom summary");
    assert.ok(
      complex!.manchester!.includes("some"),
      "complex subclass should serialize as Manchester"
    );
  });
});
