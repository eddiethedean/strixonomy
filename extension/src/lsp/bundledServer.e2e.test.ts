import assert from "node:assert/strict";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import { afterEach, describe, it } from "node:test";
import {
  bundledServerPath,
  hasExecutePermission,
  prepareBundledServerForLaunch,
} from "./bundledServer";
import { resolveLspBinaryForTests, smokeInitializeLsp } from "./lspTestHarness";

const tempDirs: string[] = [];

afterEach(() => {
  while (tempDirs.length > 0) {
    const dir = tempDirs.pop();
    if (dir) {
      fs.rmSync(dir, { recursive: true, force: true });
    }
  }
});

function copyBundledLayout(extensionRoot: string, lspSource: string): string {
  const bundled = bundledServerPath(extensionRoot);
  fs.mkdirSync(path.dirname(bundled), { recursive: true });
  fs.copyFileSync(lspSource, bundled);
  fs.chmodSync(bundled, 0o755);
  return bundled;
}

/** Simulates Marketplace/VSIX install stripping the executable bit. */
async function assertSpawnAfterMarketplaceChmod(extensionRoot: string): Promise<void> {
  if (process.platform === "win32") {
    return;
  }

  const bundled = bundledServerPath(extensionRoot);
  assert.ok(fs.existsSync(bundled), `expected bundled binary at ${bundled}`);
  fs.chmodSync(bundled, 0o644);
  assert.equal(hasExecutePermission(fs.statSync(bundled).mode), false);

  const prepared = prepareBundledServerForLaunch(extensionRoot);
  assert.equal(prepared, bundled);
  assert.equal(hasExecutePermission(fs.statSync(bundled).mode), true);

  await smokeInitializeLsp(bundled);
}

describe("bundledServer e2e", () => {
  it("spawns strixonomy-lsp after Marketplace-style chmod 644 on bundled binary", async () => {
    const lspSource = resolveLspBinaryForTests();
    const extensionRoot = fs.mkdtempSync(path.join(os.tmpdir(), "strixonomy-e2e-ext-"));
    tempDirs.push(extensionRoot);
    copyBundledLayout(extensionRoot, lspSource);
    await assertSpawnAfterMarketplaceChmod(extensionRoot);
  });

  it("spawns from unpacked VSIX extension root when ONTOCODE_E2E_EXTENSION_ROOT is set", async (t) => {
    const extensionRoot = (
      process.env.STRIXONOMY_E2E_EXTENSION_ROOT ??
      process.env.ONTOCODE_E2E_EXTENSION_ROOT
    )?.trim();
    if (!extensionRoot) {
      t.skip("ONTOCODE_E2E_EXTENSION_ROOT not set");
      return;
    }
    await assertSpawnAfterMarketplaceChmod(extensionRoot);
  });
});
