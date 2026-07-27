const VIEWTYPE_MAP: ReadonlyArray<[string, string]> = [
  ["ontocodeInspector", "strixonomyInspector"],
  ["ontocodeGraph", "strixonomyGraph"],
  ["ontocodeQueryWorkbench", "strixonomyQueryWorkbench"],
  ["ontocodeImports", "strixonomyImports"],
  ["ontocodeReasoner", "strixonomyReasoner"],
  ["ontocodeExplanation", "strixonomyExplanation"],
  ["ontocodeSemanticDiff", "strixonomySemanticDiff"],
  ["ontocodeManchesterEditor", "strixonomyManchesterEditor"],
  ["ontocodeRuleBrowser", "strixonomyRuleBrowser"],
  ["ontocodeRuleEditor", "strixonomyRuleEditor"],
  ["ontocodeRefactorPreview", "strixonomyRefactorPreview"],
  ["ontocodeSmoke", "strixonomySmoke"],
];

/** Rewrite OntoCode panel restore map keys and command IDs to Strixonomy. */
export function rewritePanelRestore(
  value: unknown
): Record<string, unknown> | undefined {
  if (!value || typeof value !== "object") {
    return undefined;
  }
  const input = value as Record<string, unknown>;
  const out: Record<string, unknown> = {};
  for (const [key, entry] of Object.entries(input)) {
    let nextKey = key;
    for (const [from, to] of VIEWTYPE_MAP) {
      if (key === from) {
        nextKey = to;
        break;
      }
    }
    if (entry && typeof entry === "object") {
      const e = { ...(entry as Record<string, unknown>) };
      if (typeof e.command === "string" && e.command.startsWith("ontocode.")) {
        e.command = `strixonomy.${e.command.slice("ontocode.".length)}`;
      }
      out[nextKey] = e;
    } else {
      out[nextKey] = entry;
    }
  }
  return out;
}
