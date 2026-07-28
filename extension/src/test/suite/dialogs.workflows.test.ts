import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/node";
import { FIXTURE_IRIS, fixturesWorkspaceUri, pathsEqual } from "./helpers";

interface StrixonomyTestHooks {
  openEntityInspector(iri: string): Promise<void>;
  waitForInspectorReady(timeoutMs?: number): Promise<void>;
  waitForInspectorIri(iri: string, timeoutMs?: number): Promise<void>;
  disposeAllPanels(): Promise<void>;
  openNewOntologyDialog(targetPath: string): Promise<void>;
  openPrefixManager(documentPath: string): Promise<void>;
  postWebviewMessage(
    panel: "newOntology" | "prefixManager" | "inspector",
    msg: unknown
  ): Promise<void>;
  waitForPanelReady(
    panel: "newOntology" | "prefixManager",
    timeoutMs?: number
  ): Promise<void>;
  disposePanel(panel: "newOntology" | "prefixManager"): Promise<void>;
}

interface StrixonomyTestApi {
  getClient(): LanguageClient | undefined;
  indexWorkspace(workspaceUri?: string): Promise<{
    stats: { error_count: number; class_count: number };
  }>;
  getCatalogSnapshot(): Promise<{
    entities: Array<{ iri: string; kind: string }>;
    documents: Array<{
      path: string;
      base_iri?: string;
      namespaces?: Record<string, string>;
    }>;
  }>;
  getEntity(iri: string): Promise<{
    detail: {
      entity: { iri: string };
      axioms: Array<{
        kind: string;
        properties?: string[];
      }>;
    };
  }>;
  __test: StrixonomyTestHooks;
}

function fixturesDir(): string {
  const fromEnv =
    process.env.STRIXONOMY_TEST_FIXTURES ??
    process.env.ONTOCODE_TEST_FIXTURES;
  if (fromEnv) {
    return fromEnv;
  }
  return path.resolve(__dirname, "..", "..", "..", "..", "fixtures");
}

async function restoreFixtureFile(
  filePath: string,
  original: string
): Promise<void> {
  fs.writeFileSync(filePath, original, "utf8");
  for (const doc of vscode.workspace.textDocuments) {
    if (!pathsEqual(doc.uri.fsPath, filePath)) {
      continue;
    }
    if (doc.isDirty) {
      await vscode.window.showTextDocument(doc, {
        preview: true,
        preserveFocus: true,
      });
      await vscode.commands.executeCommand("workbench.action.files.revert");
    }
  }
}

suite("Dialog workflows (VS Code e2e)", () => {
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

  test("New Ontology creates file with ontology IRI and indexes it", async function () {
    this.timeout(90_000);
    const targetPath = path.join(fixturesDir(), `e2e-new-ontology-${Date.now()}.ttl`);
    const ontologyIri = "https://example.org/e2e-new-ontology";

    try {
      await api.__test.openNewOntologyDialog(targetPath);
      await api.__test.waitForPanelReady("newOntology");

      await api.__test.postWebviewMessage("newOntology", {
        type: "submitNewOntology",
        ontologyIri,
      });

      assert.ok(fs.existsSync(targetPath), "ontology file should exist");
      const content = fs.readFileSync(targetPath, "utf8");
      assert.ok(
        content.includes(ontologyIri),
        "file should contain ontology IRI"
      );

      await api.indexWorkspace(fixturesWorkspaceUri());
      const snapshot = await api.getCatalogSnapshot();
      assert.ok(
        snapshot.documents.some(
          (doc) =>
            pathsEqual(doc.path, targetPath) || doc.base_iri === ontologyIri
        ),
        "catalog should list the new ontology"
      );
    } finally {
      await api.__test.disposePanel("newOntology");
      if (fs.existsSync(targetPath)) {
        fs.unlinkSync(targetPath);
      }
      await api.indexWorkspace(fixturesWorkspaceUri());
    }
  });

  test("Prefix Manager add then remove updates file and namespaces", async function () {
    this.timeout(90_000);
    const examplePath = path.join(fixturesDir(), "example.ttl");
    const original = fs.readFileSync(examplePath, "utf8");
    const prefix = "e2etest";
    const namespaceIri = "https://example.org/e2e-prefix#";

    try {
      await api.__test.openPrefixManager(examplePath);
      await api.__test.waitForPanelReady("prefixManager");

      await api.__test.postWebviewMessage("prefixManager", {
        type: "submitPrefix",
        action: "add",
        prefix,
        namespaceIri,
      });

      const afterAdd = fs.readFileSync(examplePath, "utf8");
      assert.ok(
        afterAdd.includes(`@prefix ${prefix}:`) ||
          afterAdd.includes(`@prefix ${prefix} :`),
        "file should declare the new prefix"
      );
      assert.ok(
        afterAdd.includes(namespaceIri),
        "file should include namespace IRI"
      );

      await api.indexWorkspace(fixturesWorkspaceUri());
      const snapshot = await api.getCatalogSnapshot();
      const doc = snapshot.documents.find((d) =>
        pathsEqual(d.path, examplePath)
      );
      assert.ok(doc, "example.ttl should be in catalog");
      assert.equal(
        doc?.namespaces?.[prefix],
        namespaceIri,
        "catalog namespaces should include added prefix"
      );

      await api.__test.openPrefixManager(examplePath);
      await api.__test.waitForPanelReady("prefixManager");
      await api.__test.postWebviewMessage("prefixManager", {
        type: "submitPrefix",
        action: "remove",
        prefix,
      });

      const afterRemove = fs.readFileSync(examplePath, "utf8");
      assert.ok(
        !afterRemove.includes(`@prefix ${prefix}:`) &&
          !afterRemove.includes(`@prefix ${prefix} :`),
        "prefix should be removed from file"
      );
    } finally {
      await restoreFixtureFile(examplePath, original);
      await api.__test.disposeAllPanels();
      await api.indexWorkspace(fixturesWorkspaceUri());
    }
  });

  test("Property Chain add then remove via inspector applyPatch", async function () {
    this.timeout(90_000);
    const examplePath = path.join(fixturesDir(), "example.ttl");
    const original = fs.readFileSync(examplePath, "utf8");
    const worksFor = FIXTURE_IRIS.worksFor;
    const chainProps = [worksFor, worksFor];

    try {
      await api.__test.openEntityInspector(worksFor);
      await api.__test.waitForInspectorReady();
      await api.__test.waitForInspectorIri(worksFor);

      await api.__test.postWebviewMessage("inspector", {
        type: "applyPatch",
        previewOnly: false,
        patches: [
          {
            op: "add_property_chain",
            entity_iri: worksFor,
            properties: chainProps,
          },
        ],
      });

      const afterAdd = fs.readFileSync(examplePath, "utf8");
      assert.ok(
        /propertyChainAxiom/i.test(afterAdd),
        "file should contain propertyChainAxiom"
      );

      const { detail: afterAddDetail } = await api.getEntity(worksFor);
      const chainAxiom = afterAddDetail.axioms.find(
        (a) => a.kind === "property_chain"
      );
      assert.ok(chainAxiom, "getEntity should report property_chain axiom");
      assert.deepStrictEqual(chainAxiom?.properties, chainProps);

      await api.__test.postWebviewMessage("inspector", {
        type: "applyPatch",
        previewOnly: false,
        patches: [
          {
            op: "remove_property_chain",
            entity_iri: worksFor,
            properties: chainProps,
          },
        ],
      });

      const afterRemove = fs.readFileSync(examplePath, "utf8");
      assert.ok(
        !/propertyChainAxiom/i.test(afterRemove),
        "propertyChainAxiom should be removed"
      );

      const { detail: afterRemoveDetail } = await api.getEntity(worksFor);
      assert.ok(
        !afterRemoveDetail.axioms.some((a) => a.kind === "property_chain"),
        "getEntity should no longer report property_chain"
      );
    } finally {
      await restoreFixtureFile(examplePath, original);
      await api.__test.disposeAllPanels();
      await api.indexWorkspace(fixturesWorkspaceUri());
    }
  });
});
