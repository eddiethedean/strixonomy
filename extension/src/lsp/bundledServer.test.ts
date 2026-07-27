import assert from "node:assert/strict";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import { afterEach, describe, it } from "node:test";
import {
  bundledServerPath,
  ensureBundledServerExecutable,
  hasExecutePermission,
  prepareBundledServerForLaunch,
} from "./bundledServer";

const tempDirs: string[] = [];

afterEach(() => {
  while (tempDirs.length > 0) {
    const dir = tempDirs.pop();
    if (dir) {
      fs.rmSync(dir, { recursive: true, force: true });
    }
  }
});

function makeTempExtensionLayout(): string {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "strixonomy-ext-"));
  tempDirs.push(root);
  const bundled = bundledServerPath(root);
  fs.mkdirSync(path.dirname(bundled), { recursive: true });
  fs.writeFileSync(bundled, "#!/bin/sh\nexit 0\n");
  return root;
}

describe("bundledServer", () => {
  it("bundledServerPath follows extension/server/<platform>-<arch>/ layout", () => {
    const p = bundledServerPath("/ext", "linux", "x64");
    assert.match(p, /\/ext\/server\/linux-x64\/strixonomy-lsp$/);
  });

  it("ensureBundledServerExecutable restores execute bit after chmod 644", () => {
    if (process.platform === "win32") {
      return;
    }

    const root = makeTempExtensionLayout();
    const bundled = bundledServerPath(root);
    fs.chmodSync(bundled, 0o644);
    assert.equal(hasExecutePermission(fs.statSync(bundled).mode), false);

    ensureBundledServerExecutable(bundled);

    assert.equal(hasExecutePermission(fs.statSync(bundled).mode), true);
  });

  it("prepareBundledServerForLaunch returns a spawn-ready bundled binary", () => {
    if (process.platform === "win32") {
      return;
    }

    const root = makeTempExtensionLayout();
    const bundled = bundledServerPath(root);
    fs.chmodSync(bundled, 0o644);

    const prepared = prepareBundledServerForLaunch(root);
    assert.equal(prepared, bundled);
    assert.equal(hasExecutePermission(fs.statSync(bundled).mode), true);
  });
});
