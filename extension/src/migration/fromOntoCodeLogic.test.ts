import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  legacyCommandId,
  rewritePanelRestore,
  rewriteWorkspaceSession,
} from "./fromOntoCodeLogic";

describe("fromOntoCodeLogic", () => {
  it("maps ontocode panel restore commands to strixonomy", () => {
    const input = {
      ontocodeInspector: {
        command: "ontocode.showEntityInspector",
        args: ["http://ex#A"],
      },
    };
    const out = rewritePanelRestore(input);
    assert.ok(out);
    assert.ok(out.strixonomyInspector);
    assert.equal(
      (out.strixonomyInspector as { command: string }).command,
      "strixonomy.showEntityInspector"
    );
  });

  it("rewrites panel commands in a legacy session file", () => {
    const out = rewriteWorkspaceSession({
      openOntologyUris: ["file:///tmp/example.ttl"],
      panelRestore: {
        ontocodeGraph: { command: "ontocode.openClassGraph" },
      },
    });
    assert.ok(out);
    const panels = out.panelRestore as Record<string, { command: string }>;
    assert.equal(
      panels.strixonomyGraph?.command,
      "strixonomy.openClassGraph"
    );
  });

  it("maps primary command IDs to legacy aliases", () => {
    assert.equal(
      legacyCommandId("strixonomy.openQueryWorkbench"),
      "ontocode.openQueryWorkbench"
    );
    assert.equal(legacyCommandId("other.command"), undefined);
  });
});
