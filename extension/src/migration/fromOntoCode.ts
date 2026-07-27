import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { rewritePanelRestore } from "./fromOntoCodeLogic";

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

const WORKSPACE_STATE_KEYS: ReadonlyArray<[string, string]> = [
  ["ontocode.workspaceSession", "strixonomy.workspaceSession"],
  ["ontocode.activeOntology", "strixonomy.activeOntology"],
  ["ontocode.registryVersions", "strixonomy.registryVersions"],
  ["ontocode.selection", "strixonomy.selection"],
  ["ontocode.navigation", "strixonomy.navigation"],
  ["ontocode.perspectives", "strixonomy.perspectives"],
  ["ontocode.panelRestoreState", "strixonomy.panelRestoreState"],
  ["ontocode.savedQueries", "strixonomy.savedQueries"],
  ["ontocode.queryHistory", "strixonomy.queryHistory"],
];

function migrateSettings(): boolean {
  const legacy = vscode.workspace.getConfiguration("ontocode");
  const next = vscode.workspace.getConfiguration("strixonomy");
  let migrated = false;
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
      void next.update(key, legacyWs, vscode.ConfigurationTarget.Workspace);
      migrated = true;
    }
    const legacyGlobal = inspected.globalValue;
    if (
      legacyGlobal !== undefined &&
      nextInspected?.globalValue === undefined
    ) {
      void next.update(key, legacyGlobal, vscode.ConfigurationTarget.Global);
      migrated = true;
    }
  }
  return migrated;
}

function migrateWorkspaceState(context: vscode.ExtensionContext): boolean {
  let migrated = false;
  for (const [from, to] of WORKSPACE_STATE_KEYS) {
    const existing = context.workspaceState.get(to);
    if (existing !== undefined) {
      continue;
    }
    const legacy = context.workspaceState.get(from);
    if (legacy === undefined) {
      continue;
    }
    let value: unknown = legacy;
    if (from === "ontocode.panelRestoreState" || from === "ontocode.workspaceSession") {
      if (from === "ontocode.panelRestoreState") {
        value = rewritePanelRestore(legacy) ?? legacy;
      } else if (legacy && typeof legacy === "object") {
        const snap = { ...(legacy as Record<string, unknown>) };
        if (snap.panelRestore) {
          snap.panelRestore = rewritePanelRestore(snap.panelRestore);
        }
        value = snap;
      }
    }
    void context.workspaceState.update(to, value);
    migrated = true;
  }
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
    fs.mkdirSync(primaryDir, { recursive: true });
    fs.copyFileSync(legacy, primary);
    return true;
  } catch {
    return false;
  }
}

/**
 * One-shot migration from OntoCode (`ontocode.*`) settings and state to Strixonomy.
 * Safe to call on every activate; runs at most once per workspace via a flag.
 */
export async function migrateFromOntoCode(
  context: vscode.ExtensionContext
): Promise<void> {
  if (context.workspaceState.get<boolean>(MIGRATION_FLAG)) {
    return;
  }

  const settingsMigrated = migrateSettings();
  const stateMigrated = migrateWorkspaceState(context);
  let sessionMigrated = false;
  for (const folder of vscode.workspace.workspaceFolders ?? []) {
    if (migrateSessionFile(folder)) {
      sessionMigrated = true;
    }
  }

  await context.workspaceState.update(MIGRATION_FLAG, true);

  if (settingsMigrated || stateMigrated || sessionMigrated) {
    void vscode.window
      .showInformationMessage(
        "Strixonomy migrated OntoCode settings and workspace state. See the v0.27 migration guide.",
        "Open guide"
      )
      .then((choice) => {
        if (choice === "Open guide") {
          void vscode.env.openExternal(
            vscode.Uri.parse(
              "https://strixonomy-vs.readthedocs.io/en/latest/migration/v0.27/"
            )
          );
        }
      });
  }
}
