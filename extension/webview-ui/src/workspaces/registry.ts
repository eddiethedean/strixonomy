import type { PanelKind } from "../messages";
import { EntityInspectorPanel } from "../panels/EntityInspector";
import { GraphPanel } from "../panels/GraphPanel";
import { QueryWorkbenchPanel } from "../panels/QueryWorkbench";
import { RefactorPreviewPanel } from "../panels/RefactorPreview";
import { ManchesterEditorPanel } from "../panels/ManchesterEditor";
import { RuleBrowserPanel } from "../panels/RuleBrowser";
import { RuleEditorPanel } from "../panels/RuleEditor";
import { SemanticDiffPanel } from "../panels/SemanticDiffPanel";
import { ImportsPanel } from "../panels/ImportsPanel";
import { MetricsPanel } from "../panels/MetricsPanel";
import { AboutPanel } from "../panels/AboutPanel";
import { NewOntologyDialog } from "../panels/NewOntologyDialog";
import { PrefixManagerDialog } from "../panels/PrefixManagerDialog";
import { SmokePanel } from "../panels/SmokePanel";
import { ReasonerPanel } from "../panels/ReasonerPanel";
import { ExplanationPanel } from "../panels/ExplanationPanel";
import type { WorkspaceDefinition } from "./types";

const definitions = new Map<string, WorkspaceDefinition>();
const byPanelKind = new Map<PanelKind, WorkspaceDefinition>();

export function registerWorkspace(def: WorkspaceDefinition): void {
  definitions.set(def.id, def);
  byPanelKind.set(def.panelKind, def);
}

export function getWorkspace(id: string): WorkspaceDefinition | undefined {
  return definitions.get(id);
}

export function getWorkspaceByPanelKind(panel: PanelKind): WorkspaceDefinition | undefined {
  return byPanelKind.get(panel);
}

export function listWorkspaces(): WorkspaceDefinition[] {
  return [...definitions.values()];
}

function bootRegistry(): void {
  if (definitions.size > 0) {
    return;
  }
  const entries: WorkspaceDefinition[] = [
    {
      id: "entity",
      title: "Entity Inspector",
      panelKind: "inspector",
      component: EntityInspectorPanel,
    },
    {
      id: "graph",
      title: "Graph",
      panelKind: "graph",
      component: GraphPanel,
    },
    {
      id: "query",
      title: "Query Workbench",
      panelKind: "queryWorkbench",
      component: QueryWorkbenchPanel,
    },
    {
      id: "refactor",
      title: "Refactor Preview",
      panelKind: "refactorPreview",
      component: RefactorPreviewPanel,
    },
    {
      id: "manchester",
      title: "Manchester Editor",
      panelKind: "manchesterEditor",
      component: ManchesterEditorPanel,
    },
    {
      id: "ruleBrowser",
      title: "SWRL Rule Browser",
      panelKind: "ruleBrowser",
      component: RuleBrowserPanel,
    },
    {
      id: "ruleEditor",
      title: "SWRL Rule Editor",
      panelKind: "ruleEditor",
      component: RuleEditorPanel,
    },
    {
      id: "semanticDiff",
      title: "Semantic Diff",
      panelKind: "semanticDiff",
      component: SemanticDiffPanel,
    },
    {
      id: "imports",
      title: "Manage Imports",
      panelKind: "imports",
      component: ImportsPanel,
    },
    {
      id: "metrics",
      title: "Ontology Metrics",
      panelKind: "metrics",
      component: MetricsPanel,
    },
    {
      id: "about",
      title: "About Strixonomy",
      panelKind: "about",
      component: AboutPanel,
    },
    {
      id: "newOntology",
      title: "New Ontology",
      panelKind: "newOntology",
      component: NewOntologyDialog,
    },
    {
      id: "prefixManager",
      title: "Prefix Manager",
      panelKind: "prefixManager",
      component: PrefixManagerDialog,
    },
    {
      id: "reasoner",
      title: "Reasoner",
      panelKind: "reasoner",
      component: ReasonerPanel,
    },
    {
      id: "explanation",
      title: "Explanation",
      panelKind: "explanation",
      component: ExplanationPanel,
    },
    {
      id: "smoke",
      title: "Smoke",
      panelKind: "smoke",
      component: SmokePanel,
    },
  ];
  for (const entry of entries) {
    registerWorkspace(entry);
  }
}

bootRegistry();

/** Reset registry (tests only). */
export function resetWorkspaceRegistryForTests(): void {
  definitions.clear();
  byPanelKind.clear();
}
