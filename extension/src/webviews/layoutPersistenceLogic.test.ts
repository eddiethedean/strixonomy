import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  DEFAULT_REOPEN,
  PERSPECTIVES,
  graphRestoreState,
  isAllowedPanelRestoreCommand,
  resolvePanelRestoreState,
  sanitizePanelRestoreState,
  type PanelRestoreState,
} from "./layoutPersistenceLogic";

describe("layoutPersistenceLogic", () => {
  it("exposes default Modeling/Reasoning/Review perspectives", () => {
    assert.deepEqual(
      PERSPECTIVES.map((p) => p.name),
      ["Modeling", "Reasoning", "Review"]
    );
  });

  it("falls back to default reopen commands when no saved state", () => {
    const restore = resolvePanelRestoreState(undefined, "strixonomyQueryWorkbench");
    assert.equal(restore?.command, "strixonomy.openQueryWorkbench");
  });

  it("sanitize without defaults leaves missing session panels empty", () => {
    // Session capture must use sanitizePanelRestoreState / remembered-only lookups,
    // not resolvePanelRestoreState — otherwise DEFAULT_REOPEN reopens reasoner/diff.
    assert.equal(sanitizePanelRestoreState(undefined), undefined);
    assert.equal(
      sanitizePanelRestoreState({ command: "strixonomy.runReasoner" })?.command,
      "strixonomy.runReasoner"
    );
  });

  it("prefers saved restore state over defaults", () => {
    const saved: PanelRestoreState = {
      command: "strixonomy.showExplanation",
      args: ["http://example.org#A", "el"],
      title: "Explanation: A",
    };
    assert.deepEqual(
      resolvePanelRestoreState({ strixonomyExplanation: saved }, "strixonomyExplanation"),
      saved
    );
  });

  it("rejects non-allowlisted restore commands (#309)", () => {
    assert.equal(isAllowedPanelRestoreCommand("workbench.action.terminal.new"), false);
    assert.equal(isAllowedPanelRestoreCommand("vscode.open"), false);
    assert.equal(isAllowedPanelRestoreCommand("strixonomy.showEntityInspector"), true);
    assert.equal(isAllowedPanelRestoreCommand("strixonomy.openEntity"), true);
    assert.equal(isAllowedPanelRestoreCommand("strixonomy.evil;rm"), false);
    assert.equal(
      sanitizePanelRestoreState({
        command: "workbench.action.terminal.new",
        args: [],
      }),
      undefined
    );
  });

  it("falls back to default when saved restore command is not allowlisted", () => {
    const restore = resolvePanelRestoreState(
      {
        strixonomyInspector: {
          command: "workbench.action.terminal.new",
          args: ["--dangerous"],
        },
      },
      "strixonomyInspector"
    );
    assert.deepEqual(restore, DEFAULT_REOPEN.strixonomyInspector);
  });

  it("graphRestoreState maps graphKind to restore commands", () => {
    assert.deepEqual(graphRestoreState({ graphKind: "class" }, "Class Graph"), {
      command: "strixonomy.openClassGraph",
      title: "Class Graph",
    });
    assert.deepEqual(graphRestoreState({ graphKind: "property" }, "Property Graph"), {
      command: "strixonomy.openPropertyGraph",
      title: "Property Graph",
    });
    assert.deepEqual(
      graphRestoreState({ graphKind: "object_property" }, "Object Property Graph"),
      {
        command: "strixonomy.openObjectPropertyGraph",
        title: "Object Property Graph",
      }
    );
    assert.deepEqual(
      graphRestoreState({ graphKind: "data_property" }, "Data Property Graph"),
      {
        command: "strixonomy.openDataPropertyGraph",
        title: "Data Property Graph",
      }
    );
    assert.deepEqual(graphRestoreState({ graphKind: "import" }, "Import Graph"), {
      command: "strixonomy.openImportGraph",
      title: "Import Graph",
    });
    assert.deepEqual(
      graphRestoreState({ graphKind: "dependency" }, "Dependency Graph"),
      {
        command: "strixonomy.openDependencyGraph",
        title: "Dependency Graph",
      }
    );
    assert.deepEqual(
      graphRestoreState(
        { graphKind: "individual", rootIri: "http://ex#Alice" },
        "Individual"
      ),
      {
        command: "strixonomy.openIndividualGraph",
        args: ["http://ex#Alice"],
        title: "Individual",
      }
    );
    assert.deepEqual(
      graphRestoreState(
        { graphKind: "neighborhood", rootIri: "http://ex.org#Person" },
        "Neighborhood"
      ),
      {
        command: "strixonomy.openNeighborhoodGraph",
        args: ["http://ex.org#Person"],
        title: "Neighborhood",
      }
    );
    assert.equal(
      isAllowedPanelRestoreCommand(
        graphRestoreState({ graphKind: "neighborhood", rootIri: "x" }).command
      ),
      true
    );
    assert.equal(
      isAllowedPanelRestoreCommand("strixonomy.openObjectPropertyGraph"),
      true
    );
  });
});
