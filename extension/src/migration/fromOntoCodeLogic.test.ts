import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { rewritePanelRestore } from "./fromOntoCodeLogic";

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
});
