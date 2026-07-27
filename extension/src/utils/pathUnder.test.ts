import assert from "node:assert/strict";
import path from "node:path";
import { describe, it } from "node:test";
import { isPathUnderFolder, normalizeFsPath, pathsEqual } from "./pathUnder";

describe("isPathUnderFolder", () => {
  it("matches the folder itself and descendants", () => {
    const folder = path.join(path.sep, "workspace", "proj");
    assert.equal(isPathUnderFolder(folder, folder), true);
    assert.equal(
      isPathUnderFolder(path.join(folder, "ontologies", "a.ttl"), folder),
      true
    );
  });

  it("rejects sibling folders that share a path prefix (#151)", () => {
    const folder = path.join(path.sep, "workspace", "proj");
    const sibling = path.join(path.sep, "workspace", "proj-other", "b.ttl");
    assert.equal(isPathUnderFolder(sibling, folder), false);
  });

  it("rejects paths outside the folder", () => {
    const folder = path.join(path.sep, "workspace", "proj");
    assert.equal(
      isPathUnderFolder(path.join(path.sep, "workspace", "other", "a.ttl"), folder),
      false
    );
  });

  it("matches Rust canonicalize verbatim-prefixed descendants", () => {
    if (process.platform !== "win32") {
      return;
    }
    const folder = "D:\\workspace\\proj";
    const child = "\\\\?\\D:\\workspace\\proj\\a.ttl";
    assert.equal(isPathUnderFolder(child, folder), true);
  });
});

describe("normalizeFsPath", () => {
  it("strips Windows extended-length prefixes from Rust canonicalize()", () => {
    if (process.platform !== "win32") {
      return;
    }
    assert.equal(
      normalizeFsPath("\\\\?\\D:\\a\\strixonomy\\fixtures\\example.ttl"),
      path.resolve("D:\\a\\strixonomy\\fixtures\\example.ttl")
    );
    assert.equal(
      normalizeFsPath("\\\\?\\UNC\\server\\share\\ontology.ttl"),
      path.resolve("\\\\server\\share\\ontology.ttl")
    );
  });
});

describe("pathsEqual", () => {
  it("equates verbatim-prefixed and normal paths", () => {
    if (process.platform !== "win32") {
      return;
    }
    assert.equal(
      pathsEqual(
        "D:\\a\\strixonomy\\fixtures\\example.ttl",
        "\\\\?\\D:\\a\\strixonomy\\fixtures\\example.ttl"
      ),
      true
    );
    assert.equal(
      pathsEqual(
        "d:\\a\\strixonomy\\fixtures\\example.ttl",
        "\\\\?\\D:\\a\\strixonomy\\fixtures\\example.ttl"
      ),
      true
    );
  });
});
