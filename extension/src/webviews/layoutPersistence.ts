import * as vscode from "vscode";
import {
  DEFAULT_REOPEN,
  PERSPECTIVES,
  resolvePanelRestoreState,
  sanitizePanelRestoreState,
  type PanelRestoreState,
  type Perspective,
} from "./layoutPersistenceLogic";

export type { PanelRestoreState, Perspective };
export { PERSPECTIVES };

const PERSPECTIVE_KEY = "strixonomy.perspectives";
const PANEL_STATE_KEY = "strixonomy.panelRestoreState";

let extensionContext: vscode.ExtensionContext | undefined;

export function bindLayoutPersistenceContext(
  context: vscode.ExtensionContext
): void {
  extensionContext = context;
}

export async function persistPerspective(
  context: vscode.ExtensionContext,
  perspective: Perspective
): Promise<void> {
  const existing = listPerspectives(context).filter(
    (item) => item.name.toLowerCase() !== perspective.name.toLowerCase()
  );
  await context.workspaceState.update(PERSPECTIVE_KEY, [...existing, perspective]);
}

export function listPerspectives(context: vscode.ExtensionContext): Perspective[] {
  return [
    ...PERSPECTIVES,
    ...(context.workspaceState.get<Perspective[]>(PERSPECTIVE_KEY) ?? []),
  ];
}

export function loadPerspective(
  context: vscode.ExtensionContext,
  name: string
): Perspective | undefined {
  return listPerspectives(context).find(
    (perspective) => perspective.name.toLowerCase() === name.toLowerCase()
  );
}

export async function rememberPanelRestoreState(
  viewType: string,
  state: PanelRestoreState
): Promise<void> {
  if (!extensionContext) {
    return;
  }
  const safe = sanitizePanelRestoreState(state);
  if (!safe) {
    return;
  }
  const all =
    extensionContext.workspaceState.get<Record<string, PanelRestoreState>>(
      PANEL_STATE_KEY
    ) ?? {};
  all[viewType] = safe;
  await extensionContext.workspaceState.update(PANEL_STATE_KEY, all);
}

/** Drop remembered restore state when a panel is closed (session must not reopen it). */
export async function forgetPanelRestoreState(viewType: string): Promise<void> {
  if (!extensionContext) {
    return;
  }
  const all =
    extensionContext.workspaceState.get<Record<string, PanelRestoreState>>(
      PANEL_STATE_KEY
    ) ?? {};
  if (!(viewType in all)) {
    return;
  }
  delete all[viewType];
  await extensionContext.workspaceState.update(PANEL_STATE_KEY, all);
}

/**
 * Session capture: only panels the user actually opened (raw workspaceState).
 * Do **not** fall back to DEFAULT_REOPEN — that would reopen reasoner/diff on every restore.
 */
export function getRememberedPanelRestoreState(
  context: vscode.ExtensionContext,
  viewType: string
): PanelRestoreState | undefined {
  const all =
    context.workspaceState.get<Record<string, PanelRestoreState>>(PANEL_STATE_KEY) ??
    {};
  return sanitizePanelRestoreState(all[viewType]);
}

export function getPanelRestoreState(
  context: vscode.ExtensionContext,
  viewType: string
): PanelRestoreState | undefined {
  const all =
    context.workspaceState.get<Record<string, PanelRestoreState>>(PANEL_STATE_KEY) ??
    {};
  return resolvePanelRestoreState(all, viewType);
}

const SERIALIZED_VIEWS = Object.keys(DEFAULT_REOPEN);

/**
 * Session restore owns live panel reopen (`reopenPanels`). VS Code serializers
 * previously revived stub “Reopen panel” tabs in parallel (#300) — dispose them.
 */
export function registerWebviewPanelSerializers(
  context: vscode.ExtensionContext
): void {
  bindLayoutPersistenceContext(context);
  for (const viewType of SERIALIZED_VIEWS) {
    context.subscriptions.push(
      vscode.window.registerWebviewPanelSerializer(viewType, {
        async deserializeWebviewPanel(panel): Promise<void> {
          // Prefer session-driven live reopen; discard placeholder stubs (#300).
          panel.dispose();
        },
      })
    );
  }
}
