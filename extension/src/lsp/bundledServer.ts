import { execFileSync } from "child_process";
import * as fs from "fs";
import * as path from "path";

export function bundledServerFileName(platform: NodeJS.Platform = process.platform): string {
  return platform === "win32" ? "strixonomy-lsp.exe" : "strixonomy-lsp";
}

export function bundledServerRelativeDir(
  platform: NodeJS.Platform = process.platform,
  arch: string = process.arch
): string {
  return path.join("server", `${platform}-${arch}`, bundledServerFileName(platform));
}

export function bundledServerPath(
  extensionPath: string,
  platform: NodeJS.Platform = process.platform,
  arch: string = process.arch
): string {
  return path.join(extensionPath, bundledServerRelativeDir(platform, arch));
}

export function hasExecutePermission(mode: number): boolean {
  return (mode & 0o111) !== 0;
}

/**
 * VSIX/Marketplace installs often drop the Unix executable bit on bundled binaries.
 * Restores execute permission so `spawn` does not fail with EACCES.
 */
export function ensureBundledServerExecutable(serverPath: string): void {
  if (process.platform === "win32") {
    return;
  }
  try {
    const mode = fs.statSync(serverPath).mode;
    if (!hasExecutePermission(mode)) {
      fs.chmodSync(serverPath, mode | 0o755);
    }
    if (process.platform === "darwin") {
      try {
        execFileSync("xattr", ["-dr", "com.apple.quarantine", serverPath], {
          stdio: "ignore",
        });
      } catch {
        // xattr missing or attribute absent — ignore.
      }
    }
  } catch {
    // Spawn will fail with a clear error if this cannot be fixed.
  }
}

/**
 * Prepare a bundled server for launch after a non-executable install (Marketplace/VSIX).
 * Returns the binary path when present on disk.
 */
export function prepareBundledServerForLaunch(extensionPath: string): string | undefined {
  const bundled = bundledServerPath(extensionPath);
  if (!fs.existsSync(bundled)) {
    return undefined;
  }
  ensureBundledServerExecutable(bundled);
  return bundled;
}
