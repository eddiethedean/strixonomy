import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import {
  legacyCommandId,
  rewriteWorkspaceSession,
} from "./fromOntoCodeLogic";

const MIGRATION_FLAG = "strixonomy.migratedFromOntoCode.v0.27";

const SETTING_KEYS = [
  "lspPath",
  "autoIndexOnOpen",
  "queryHistoryLimit",
  "reasoner.default",
  "reasoner.autoProfile",
  "hierarchy.mode",
  "indexCache",
  "robotPath",
  "diagnostics.rules",
] as const;

async function migrateSettings(): Promise<boolean> {
  const legacy = vscode.workspace.getConfiguration("ontocode");
  const next = vscode.workspace.getConfiguration("strixonomy");
  let migrated = false;
  const updates: Thenable<void>[] = [];
  for (const key of SETTING_KEYS) {
    const inspected = legacy.inspect(key);
    if (!inspected) {
      continue;
    }
    const nextInspected = next.inspect(key);
    const legacyWs = inspected.workspaceValue;
    if (
      legacyWs !== undefined &&
      nextInspected?.workspaceValue === undefined
    ) {
      updates.push(
        next.update(key, legacyWs, vscode.ConfigurationTarget.Workspace)
      );
      migrated = true;
    }
    const legacyGlobal = inspected.globalValue;
    if (
      legacyGlobal !== undefined &&
      nextInspected?.globalValue === undefined
    ) {
      updates.push(
        next.update(key, legacyGlobal, vscode.ConfigurationTarget.Global)
      );
      migrated = true;
    }
  }
  await Promise.all(updates);
  return migrated;
}

function migrateSessionFile(folder: vscode.WorkspaceFolder): boolean {
  const legacy = path.join(folder.uri.fsPath, ".ontocode", "session.json");
  const primaryDir = path.join(folder.uri.fsPath, ".strixonomy");
  const primary = path.join(primaryDir, "session.json");
  if (!fs.existsSync(legacy) || fs.existsSync(primary)) {
    return false;
  }
  try {
    const parsed: unknown = JSON.parse(fs.readFileSync(legacy, "utf8"));
    const migrated = rewriteWorkspaceSession(parsed);
    if (!migrated) {
      throw new Error("legacy session file is not a JSON object");
    }
    fs.mkdirSync(primaryDir, { recursive: true });
    fs.writeFileSync(primary, `${JSON.stringify(migrated, null, 2)}\n`, "utf8");
    return true;
  } catch (error) {
    throw new Error(
      `failed to migrate ${legacy}: ${
        error instanceof Error ? error.message : String(error)
      }`
    );
  }
}

/**
 * Keep keybindings and automation that invoke `ontocode.*` commands working
 * through the compatibility window. Aliases are derived from the manifest so
 * new commands cannot silently omit their legacy forwarding entry.
 */
export async function registerLegacyCommandAliases(
  context: vscode.ExtensionContext
): Promise<void> {
  const commands = (
    context.extension.packageJSON as {
      contributes?: { commands?: Array<{ command?: unknown }> };
    }
  ).contributes?.commands;
  const registered = new Set(await vscode.commands.getCommands(true));
  for (const contribution of commands ?? []) {
    if (typeof contribution.command !== "string") {
      continue;
    }
    const legacy = legacyCommandId(contribution.command);
    if (!legacy || registered.has(legacy)) {
      continue;
    }
    const target = contribution.command;
    context.subscriptions.push(
      vscode.commands.registerCommand(legacy, (...args: unknown[]) => {
        return vscode.commands.executeCommand(target, ...args);
      })
    );
    registered.add(legacy);
  }
}

/**
 * One-shot migration from recoverable OntoCode settings and session files.
 *
 * VS Code Memento state is private to an extension ID, so the new
 * `strixonomy.strixonomy` extension cannot read `ontocode.ontocode` workspaceState.
 * Safe to call on every activate; runs at most once per workspace via a flag.
 */
export async function migrateFromOntoCode(
  context: vscode.ExtensionContext
): Promise<void> {
  if (context.workspaceState.get<boolean>(MIGRATION_FLAG)) {
    return;
  }

  let settingsMigrated = false;
  try {
    settingsMigrated = await migrateSettings();
  } catch (error) {
    void vscode.window.showWarningMessage(
      `Strixonomy could not migrate OntoCode settings and will retry next activation: ${
        error instanceof Error ? error.message : String(error)
      }`
    );
    return;
  }
  let sessionMigrated = false;
  const sessionErrors: string[] = [];
  for (const folder of vscode.workspace.workspaceFolders ?? []) {
    try {
      if (migrateSessionFile(folder)) {
        sessionMigrated = true;
      }
    } catch (error) {
      sessionErrors.push(error instanceof Error ? error.message : String(error));
    }
  }

  if (sessionErrors.length > 0) {
    void vscode.window.showWarningMessage(
      `Strixonomy could not migrate an OntoCode session file and will retry next activation: ${sessionErrors.join(
        "; "
      )}`
    );
    return;
  }

  await context.workspaceState.update(MIGRATION_FLAG, true);

  if (settingsMigrated || sessionMigrated) {
    void vscode.window
      .showInformationMessage(
        "Strixonomy migrated recoverable OntoCode settings and session files. See the v0.27 migration guide.",
        "Open guide"
      )
      .then((choice) => {
        if (choice === "Open guide") {
          void vscode.env.openExternal(
            vscode.Uri.parse(
              "https://strixonomy.readthedocs.io/en/latest/migration/v0.27/"
            )
          );
        }
      });
  }
}
