export interface Perspective {
  name: string;
  panels: string[];
}

export interface PanelRestoreState {
  command: string;
  args?: unknown[];
  title?: string;
}

export const PERSPECTIVES: readonly Perspective[] = [
  { name: "Modeling", panels: ["inspector", "query"] },
  { name: "Reasoning", panels: ["reasoner", "explanation", "graph"] },
  { name: "Review", panels: ["semanticDiff", "imports"] },
];

export const DEFAULT_REOPEN: Record<string, PanelRestoreState> = {
  strixonomyInspector: { command: "strixonomy.showEntityInspector" },
  strixonomyGraph: { command: "strixonomy.openClassGraph" },
  strixonomyQueryWorkbench: { command: "strixonomy.openQueryWorkbench" },
  strixonomyImports: { command: "strixonomy.manageImports" },
  strixonomyReasoner: { command: "strixonomy.runReasoner" },
  strixonomyRefactorPreview: { command: "strixonomy.findUsages" },
  strixonomyExplanation: { command: "strixonomy.showExplanation" },
  strixonomySemanticDiff: { command: "strixonomy.semanticDiff" },
  strixonomyManchesterEditor: { command: "strixonomy.openManchesterEditor" },
  strixonomyRuleBrowser: { command: "strixonomy.openRuleBrowser" },
  strixonomyRuleEditor: { command: "strixonomy.openRuleEditor" },
};

/** Commands permitted for panel reopen from session / layout restore state. */
export const ALLOWED_PANEL_RESTORE_COMMANDS: ReadonlySet<string> = new Set([
  ...Object.values(DEFAULT_REOPEN).map((state) => state.command),
  "strixonomy.openEntity",
  "strixonomy.openClassGraph",
  "strixonomy.openPropertyGraph",
  "strixonomy.openObjectPropertyGraph",
  "strixonomy.openDataPropertyGraph",
  "strixonomy.openIndividualGraph",
  "strixonomy.openImportGraph",
  "strixonomy.openDependencyGraph",
  "strixonomy.openNeighborhoodGraph",
]);

/**
 * Session/layout restore must never execute arbitrary VS Code commands from
 * workspaceState or `.strixonomy/session.json` (see #309).
 */
export function isAllowedPanelRestoreCommand(command: string): boolean {
  if (typeof command !== "string" || command.length === 0) {
    return false;
  }
  if (!command.startsWith("strixonomy.")) {
    return false;
  }
  // Reject unexpected characters that should never appear in our command IDs.
  if (!/^strixonomy\.[A-Za-z0-9.]+$/.test(command)) {
    return false;
  }
  return ALLOWED_PANEL_RESTORE_COMMANDS.has(command);
}

/** Return state only when its reopen command is allowlisted. */
export function sanitizePanelRestoreState(
  state: PanelRestoreState | undefined
): PanelRestoreState | undefined {
  if (!state?.command || !isAllowedPanelRestoreCommand(state.command)) {
    return undefined;
  }
  return state;
}

export function resolvePanelRestoreState(
  saved: Record<string, PanelRestoreState> | undefined,
  viewType: string
): PanelRestoreState | undefined {
  const savedState = sanitizePanelRestoreState(saved?.[viewType]);
  if (savedState) {
    return savedState;
  }
  return DEFAULT_REOPEN[viewType];
}

export interface GraphRestoreOptions {
  graphKind: string;
  rootIri?: string;
}

/** Map active graph mode to the layout-restore command + args. */
export function graphRestoreState(
  options: GraphRestoreOptions,
  title?: string
): PanelRestoreState {
  switch (options.graphKind) {
    case "class":
      return { command: "strixonomy.openClassGraph", title };
    case "property":
      return { command: "strixonomy.openPropertyGraph", title };
    case "object_property":
      return { command: "strixonomy.openObjectPropertyGraph", title };
    case "data_property":
      return { command: "strixonomy.openDataPropertyGraph", title };
    case "individual":
      return {
        command: "strixonomy.openIndividualGraph",
        args: options.rootIri ? [options.rootIri] : undefined,
        title,
      };
    case "import":
      return { command: "strixonomy.openImportGraph", title };
    case "dependency":
      return { command: "strixonomy.openDependencyGraph", title };
    case "neighborhood":
    default:
      return {
        command: "strixonomy.openNeighborhoodGraph",
        args: options.rootIri ? [options.rootIri] : undefined,
        title,
      };
  }
}
