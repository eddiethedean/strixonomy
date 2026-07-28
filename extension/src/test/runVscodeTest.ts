import * as path from "path";
import * as fs from "fs";
import { downloadAndUnzipVSCode, runTests } from "@vscode/test-electron";

async function main(): Promise<void> {
  // Cursor/agent host shells often set ELECTRON_RUN_AS_NODE=1, which makes the
  // VS Code Electron binary treat launchArgs (workspace folder) as a Node script.
  delete process.env.ELECTRON_RUN_AS_NODE;

  const extensionRoot = path.resolve(__dirname, "..", "..");
  const extensionTestsPath = path.resolve(__dirname, "suite", "index");
  const fixturesPath =
    process.env.STRIXONOMY_TEST_FIXTURES ??
    path.resolve(extensionRoot, "..", "fixtures");
  const version = process.env.VSCODE_VERSION ?? "stable";

  // Electron binary (not the `code` CLI) — CLI can attach to an existing instance.
  const vscodeExecutablePath = await downloadAndUnzipVSCode(version);

  // Isolated user-data dir avoids "another instance of Code is running" when a
  // previous e2e host is still alive, and keeps capture runs reproducible.
  const userDataDir =
    process.env.STRIXONOMY_VSCODE_USER_DATA ??
    path.join(
      extensionRoot,
      ".vscode-test",
      process.env.STRIXONOMY_CAPTURE_SCREENSHOTS === "1"
        ? "user-data-screenshots"
        : "user-data"
    );
  fs.mkdirSync(userDataDir, { recursive: true });

  const extensionTestsEnv: Record<string, string> = {
    STRIXONOMY_TEST_FIXTURES: fixturesPath,
  };
  for (const key of [
    "STRIXONOMY_CAPTURE_SCREENSHOTS",
    "STRIXONOMY_SCREENSHOT_DIR",
    "STRIXONOMY_CAPTURE_SCRIPT",
  ] as const) {
    const value = process.env[key];
    if (value) {
      extensionTestsEnv[key] = value;
    }
  }

  await runTests({
    vscodeExecutablePath,
    extensionDevelopmentPath: extensionRoot,
    extensionTestsPath,
    launchArgs: [
      fixturesPath,
      "--disable-workspace-trust",
      "--user-data-dir",
      userDataDir,
    ],
    extensionTestsEnv,
  });
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
