import { useCallback, useEffect, useMemo, useRef, useState, type KeyboardEvent } from "react";
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  useNodesState,
  useEdgesState,
  type Node,
  type Edge,
  type ReactFlowInstance,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { LiveAnnouncer, PanelMain, usePrefersReducedMotion } from "../a11y";
import {
  Badge,
  ButtonBar,
  Callout,
  Card,
  CheckboxRow,
  EmptyState,
  InlineCode,
  RangeField,
  Section,
} from "../components/ui";
import { useWorkspaceHost } from "../context/HostContext";
import { postFocusToHost } from "../hooks/useFocusSync";
import {
  subscribeFocus,
  subscribeWorkspaceEvents,
  useWorkspaceStore,
} from "../store";
import {
  GraphPayload,
  HostMessage,
  isHostMessage,
} from "../messages";
import type { WorkspaceProps } from "../workspaces/types";

const GRAPH_KINDS = [
  "class",
  "object_property",
  "data_property",
  "property",
  "individual",
  "import",
  "dependency",
  "neighborhood",
  "query_result",
  "refactor_preview",
] as const;

type GraphKindOption = (typeof GRAPH_KINDS)[number];
type ViewMode = "graph" | "list";
type HistoryEntry = { graphKind: string; rootIri?: string; depth: number };

function readInitialParams(): { graphKind: string; rootIri?: string } {
  const params = new URLSearchParams(window.location.search);
  return {
    graphKind: params.get("graphKind") ?? "class",
    rootIri: params.get("root") ?? undefined,
  };
}

export function GraphPanel(_props?: WorkspaceProps): JSX.Element {
  const prefersReducedMotion = usePrefersReducedMotion();
  const host = useWorkspaceHost();
  const storeRootIri = useWorkspaceStore((s) => s.graph.rootIri);
  const unsatisfiable = useWorkspaceStore((s) => s.reasoning.unsatisfiable);
  const initial = useMemo(() => readInitialParams(), []);
  const [graph, setGraph] = useState<GraphPayload | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [depth, setDepth] = useState(2);
  const [graphMode, setGraphMode] = useState<"asserted" | "inferred" | "combined">(
    "asserted"
  );
  const [hideDeprecated, setHideDeprecated] = useState(false);
  const [ontologyIriFilter, setOntologyIriFilter] = useState("");
  const [entityKindFilter, setEntityKindFilter] = useState("");
  const [namespaceFilter, setNamespaceFilter] = useState("");
  const [relationshipFilter, setRelationshipFilter] = useState("");
  const [showUnsatOverlay, setShowUnsatOverlay] = useState(true);
  const [viewMode, setViewMode] = useState<ViewMode>("graph");
  const [contextMenu, setContextMenu] = useState<{
    x: number;
    y: number;
    iri: string;
  } | null>(null);
  const [graphKind, setGraphKind] = useState(initial.graphKind);
  const [rootIri, setRootIri] = useState<string | undefined>(
    initial.rootIri ?? storeRootIri ?? undefined
  );
  const [rootIris, setRootIris] = useState<string[] | undefined>();
  const [layout, setLayout] = useState<"grid" | "circle" | "stack">("grid");
  const [search, setSearch] = useState("");
  const [error, setError] = useState("");
  const [history, setHistory] = useState<HistoryEntry[]>([]);
  const [historyIndex, setHistoryIndex] = useState(-1);
  const skipHistory = useRef(false);
  const hasGraphData = useRef(false);
  const graphKindRef = useRef(graphKind);
  graphKindRef.current = graphKind;

  const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);
  const rf = useRef<ReactFlowInstance | null>(null);
  const canvasRef = useRef<HTMLDivElement | null>(null);
  /** Avoid one-shot fitView / virtualization against a 0×0 webview pane (#442). */
  const fittedForGraph = useRef<string | null>(null);
  const [viewportFitted, setViewportFitted] = useState(false);

  const buildFilters = useCallback(() => {
    const entity_kinds = entityKindFilter
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean);
    const namespaces = namespaceFilter
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean);
    const relationship_kinds = relationshipFilter
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean);
    const ontology = ontologyIriFilter.trim();
    return {
      hide_deprecated: hideDeprecated,
      ontology_iri: ontology || undefined,
      entity_kinds: entity_kinds.length ? entity_kinds : undefined,
      namespaces: namespaces.length ? namespaces : undefined,
      relationship_kinds: relationship_kinds.length ? relationship_kinds : undefined,
      search_text: undefined,
    };
  }, [
    hideDeprecated,
    ontologyIriFilter,
    entityKindFilter,
    namespaceFilter,
    relationshipFilter,
  ]);

  const requestGraph = useCallback(() => {
    host.postToCore({
      type: "requestGraph",
      graphKind,
      rootIri,
      rootIris,
      depth,
      includeInferred: graphMode !== "asserted",
      filters: buildFilters(),
    });
  }, [host, graphKind, rootIri, rootIris, depth, graphMode, buildFilters]);

  const pushHistory = useCallback(
    (entry: HistoryEntry) => {
      if (skipHistory.current) {
        skipHistory.current = false;
        return;
      }
      setHistory((prev) => {
        const next = [...prev.slice(0, historyIndex + 1), entry].slice(-40);
        setHistoryIndex(next.length - 1);
        return next;
      });
    },
    [historyIndex]
  );

  useEffect(() => {
    return subscribeWorkspaceEvents((event) => {
      if (event.type === "ReasoningCompleted" && graphMode !== "asserted") {
        requestGraph();
      }
    });
  }, [graphMode, requestGraph]);

  // Entity focus updates selection; only neighborhood graphs follow focus as root (#154).
  useEffect(() => {
    return subscribeFocus((focus) => {
      if (focus.kind !== "entity") {
        return;
      }
      setSelectedId(focus.id);
      setSelectedIds(new Set([focus.id]));
      if (graphKindRef.current === "neighborhood" && focus.id !== rootIri) {
        setRootIri(focus.id);
      }
    });
  }, [rootIri]);

  useEffect(() => {
    if (
      storeRootIri &&
      graphKind === "neighborhood" &&
      storeRootIri !== rootIri
    ) {
      setRootIri(storeRootIri);
    }
  }, [storeRootIri, rootIri, graphKind]);

  useEffect(() => {
    host.postToCore({ type: "ready", panel: "graph" });

    const handler = (event: MessageEvent): void => {
      if (!isHostMessage(event.data)) {
        return;
      }
      const msg: HostMessage = event.data;
      if (msg.type === "focusState" && msg.focus.kind === "entity") {
        setSelectedId(msg.focus.id);
        setSelectedIds(new Set([msg.focus.id]));
        if (graphKindRef.current === "neighborhood") {
          setRootIri(msg.focus.id);
        }
      }
      if (msg.type === "graphData") {
        hasGraphData.current = true;
        setError("");
        setGraph(msg.graph);
        setGraphKind(msg.graph.graph_kind);
        if (msg.rootIri !== undefined) {
          setRootIri(msg.rootIri);
        }
      } else if (msg.type === "error") {
        setGraph(null);
        setError(msg.message);
      } else if (msg.type === "init" && msg.panel === "graph") {
        if (!hasGraphData.current) {
          requestGraph();
        }
      }
    };
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  }, [requestGraph, host]);

  useEffect(() => {
    if (rootIri || (rootIris && rootIris.length > 0) || graphKind !== "neighborhood") {
      requestGraph();
    }
  }, [rootIri, rootIris, requestGraph, graphKind]);

  useEffect(() => {
    if (!hasGraphData.current) {
      return;
    }
    requestGraph();
  }, [
    depth,
    graphMode,
    hideDeprecated,
    ontologyIriFilter,
    entityKindFilter,
    namespaceFilter,
    relationshipFilter,
    requestGraph,
  ]);

  useEffect(() => {
    pushHistory({ graphKind, rootIri, depth });
    // eslint-disable-next-line react-hooks/exhaustive-deps -- history on navigation only
  }, [graphKind, rootIri, depth]);

  const filteredNodeIds = useMemo(() => {
    const q = search.trim().toLowerCase();
    if (!q || !graph) {
      return null;
    }
    const ids = new Set<string>();
    for (const n of graph.nodes) {
      if (
        (n.label || n.id).toLowerCase().includes(q) ||
        (n.namespace ?? "").toLowerCase().includes(q)
      ) {
        ids.add(n.id);
      }
    }
    return ids;
  }, [search, graph]);

  const unsatSet = useMemo(() => new Set(unsatisfiable), [unsatisfiable]);

  const flowNodes = useMemo(() => {
    if (!graph) {
      return [];
    }
    return layoutNodes(graph, layout, {
      searchIds: filteredNodeIds,
      unsatisfiable: showUnsatOverlay ? unsatSet : new Set(),
      selectedIds,
    });
  }, [
    graph,
    layout,
    filteredNodeIds,
    unsatSet,
    showUnsatOverlay,
    selectedIds,
  ]);

  const flowEdges = useMemo(() => {
    if (!graph) {
      return [];
    }
    return toFlowEdges(graph, graphMode, filteredNodeIds, prefersReducedMotion);
  }, [graph, graphMode, filteredNodeIds, prefersReducedMotion]);

  useEffect(() => {
    setNodes(flowNodes);
    setEdges(flowEdges);
  }, [flowNodes, flowEdges, setNodes, setEdges]);

  const graphFitKey = useMemo(() => {
    if (!graph) {
      return "";
    }
    return [
      graph.graph_kind,
      rootIri ?? "",
      graph.nodes.length,
      graph.edges.length,
      layout,
      graphMode,
    ].join("|");
  }, [graph, rootIri, layout, graphMode]);

  const fitGraphIntoView = useCallback((instance?: ReactFlowInstance | null) => {
    const api = instance ?? rf.current;
    const el = canvasRef.current;
    if (!api || !el || flowNodes.length === 0) {
      return false;
    }
    const { width, height } = el.getBoundingClientRect();
    if (width < 2 || height < 2) {
      return false;
    }
    void api.fitView({ padding: 0.2 });
    fittedForGraph.current = graphFitKey;
    setViewportFitted(true);
    return true;
  }, [flowNodes.length, graphFitKey]);

  useEffect(() => {
    fittedForGraph.current = null;
    setViewportFitted(false);
  }, [graphFitKey]);

  // Remount-safe fit: VS Code webviews often first paint at 0×0; refit once size is real (#442).
  useEffect(() => {
    if (flowNodes.length === 0 || viewMode !== "graph") {
      return;
    }
    const el = canvasRef.current;
    if (!el) {
      return;
    }
    const tryFit = (): void => {
      if (fittedForGraph.current === graphFitKey) {
        return;
      }
      fitGraphIntoView();
    };
    tryFit();
    const ro = new ResizeObserver(() => {
      tryFit();
    });
    ro.observe(el);
    return () => ro.disconnect();
  }, [flowNodes.length, viewMode, graphFitKey, fitGraphIntoView]);

  const selectedNode = useMemo(
    () => graph?.nodes.find((n) => n.id === selectedId),
    [graph, selectedId]
  );

  const visibleNodes = useMemo(() => {
    if (!graph) {
      return [];
    }
    if (!filteredNodeIds) {
      return graph.nodes;
    }
    return graph.nodes.filter((n) => filteredNodeIds.has(n.id));
  }, [graph, filteredNodeIds]);

  const visibleEdges = useMemo(() => {
    if (!graph) {
      return [];
    }
    if (!filteredNodeIds) {
      return graph.edges;
    }
    return graph.edges.filter(
      (e) => filteredNodeIds.has(e.source) && filteredNodeIds.has(e.target)
    );
  }, [graph, filteredNodeIds]);

  const inspectEntity = (iri: string): void => {
    postFocusToHost(host, { kind: "entity", id: iri, source: "graph" });
    host.postToCore({ type: "selectNode", iri });
  };

  const expandNeighborhood = (iri: string): void => {
    setGraphKind("neighborhood");
    setRootIri(iri);
    setRootIris(undefined);
    setDepth((d) => Math.min(5, Math.max(d, 2)));
  };

  const goHistory = (delta: number): void => {
    const next = historyIndex + delta;
    if (next < 0 || next >= history.length) {
      return;
    }
    const entry = history[next];
    skipHistory.current = true;
    setHistoryIndex(next);
    setGraphKind(entry.graphKind);
    setRootIri(entry.rootIri);
    setDepth(entry.depth);
  };

  const onCanvasKeyDown = (e: KeyboardEvent): void => {
    if (!graph || visibleNodes.length === 0) {
      return;
    }
    const ids = visibleNodes.map((n) => n.id);
    const idx = selectedId ? ids.indexOf(selectedId) : -1;
    if (e.key === "ArrowDown" || e.key === "ArrowRight") {
      e.preventDefault();
      const next = ids[(idx + 1 + ids.length) % ids.length];
      setSelectedId(next);
      setSelectedIds(new Set([next]));
    } else if (e.key === "ArrowUp" || e.key === "ArrowLeft") {
      e.preventDefault();
      const next = ids[(idx - 1 + ids.length) % ids.length];
      setSelectedId(next);
      setSelectedIds(new Set([next]));
    } else if (e.key === "Enter" && selectedId) {
      e.preventDefault();
      inspectEntity(selectedId);
    } else if (e.key === "Escape") {
      setSelectedId(null);
      setSelectedIds(new Set());
      setContextMenu(null);
    }
  };

  const selectionAnnounce = selectedNode
    ? `Selected ${selectedNode.label || selectedNode.id}`
    : viewMode === "list"
      ? "Graph list view"
      : graph
        ? `Graph with ${visibleNodes.length} nodes`
        : "";

  return (
    <PanelMain label="Ontology graph" className="graph-layout">
      <LiveAnnouncer message={selectionAnnounce} />
      <div
        className="graph-canvas"
        ref={canvasRef}
        tabIndex={0}
        role="application"
        aria-label="Ontology graph canvas"
        onKeyDown={onCanvasKeyDown}
        onClick={() => setContextMenu(null)}
      >
        {viewMode === "graph" && nodes.length > 0 ? (
          <ReactFlow
            nodes={nodes}
            edges={edges}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            // Virtualize only after a successful fit: culling against a 0×0 pane
            // permanently hides nodes on first open in VS Code webviews (#442).
            onlyRenderVisibleElements={viewportFitted && nodes.length > 200}
            multiSelectionKeyCode="Shift"
            onInit={(instance) => {
              rf.current = instance;
              requestAnimationFrame(() => {
                fitGraphIntoView(instance);
              });
            }}
            onNodeClick={(event, node) => {
              if (event.shiftKey) {
                setSelectedIds((prev) => {
                  const next = new Set(prev);
                  if (next.has(node.id)) {
                    next.delete(node.id);
                  } else {
                    next.add(node.id);
                  }
                  return next;
                });
              } else {
                setSelectedIds(new Set([node.id]));
              }
              setSelectedId(node.id);
              setContextMenu(null);
            }}
            onNodeContextMenu={(event, node) => {
              event.preventDefault();
              setSelectedId(node.id);
              setContextMenu({ x: event.clientX, y: event.clientY, iri: node.id });
            }}
            onSelectionChange={({ nodes: sel }) => {
              if (sel.length === 0) {
                return;
              }
              setSelectedIds(new Set(sel.map((n) => n.id)));
              setSelectedId(sel[0]?.id ?? null);
            }}
          >
            <Background />
            <Controls />
            <MiniMap />
          </ReactFlow>
        ) : null}
        {viewMode === "list" && graph && visibleNodes.length > 0 ? (
          <div className="graph-list" role="table" aria-label="Graph list alternate">
            <table>
              <thead>
                <tr>
                  <th>Label</th>
                  <th>Kind</th>
                  <th>IRI</th>
                </tr>
              </thead>
              <tbody>
                {visibleNodes.map((n) => (
                  <tr
                    key={n.id}
                    className={selectedId === n.id ? "is-selected" : undefined}
                    onClick={() => {
                      setSelectedId(n.id);
                      setSelectedIds(new Set([n.id]));
                    }}
                    onDoubleClick={() => inspectEntity(n.id)}
                  >
                    <td>
                      {n.label}
                      {showUnsatOverlay && unsatSet.has(n.id) ? " ⚠" : ""}
                    </td>
                    <td>{n.kind}</td>
                    <td>
                      <InlineCode>{n.id}</InlineCode>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
            <p className="oc-muted">
              {visibleEdges.length} edge{visibleEdges.length === 1 ? "" : "s"} among listed nodes
            </p>
          </div>
        ) : null}
        {(!graph || graph.nodes.length === 0) && viewMode === "graph" ? (
          <EmptyState
            title={error ? "Graph error" : "No graph data"}
            detail={
              error ||
              "Adjust filters or index the workspace, then refresh."
            }
          />
        ) : null}
        {contextMenu ? (
          <div
            className="graph-context-menu"
            style={{ left: contextMenu.x, top: contextMenu.y }}
            role="menu"
          >
            <button
              type="button"
              role="menuitem"
              onClick={() => {
                inspectEntity(contextMenu.iri);
                setContextMenu(null);
              }}
            >
              Inspect
            </button>
            <button
              type="button"
              role="menuitem"
              onClick={() => {
                host.postToCore({ type: "revealInHierarchy", iri: contextMenu.iri });
                setContextMenu(null);
              }}
            >
              Reveal in hierarchy
            </button>
            <button
              type="button"
              role="menuitem"
              onClick={() => {
                host.postToCore({ type: "jumpToEditor", iri: contextMenu.iri });
                setContextMenu(null);
              }}
            >
              Jump to editor
            </button>
            <button
              type="button"
              role="menuitem"
              onClick={() => {
                expandNeighborhood(contextMenu.iri);
                setContextMenu(null);
              }}
            >
              Expand neighborhood
            </button>
          </div>
        ) : null}
      </div>
      <aside className="graph-sidebar">
        <Section title="Overview" card>
          <div className="oc-badge-row">
            <Badge variant="kind">{graphKind}</Badge>
            {graph?.truncated ? (
              <Badge
                variant="warning"
                title="Graph capped for large ontologies; focus a root or reduce depth"
              >
                Truncated — focus root / reduce depth
              </Badge>
            ) : null}
            {selectedIds.size > 1 ? (
              <Badge variant="kind">{selectedIds.size} selected</Badge>
            ) : null}
          </div>
          {graph?.truncated ? (
            <Callout variant="info">
              Large ontology: set a root via Expand neighborhood, lower depth, or
              enable ontology / search filters.
            </Callout>
          ) : null}
        </Section>

        <Section title="Controls" card>
          <label className="oc-field">
            <div className="oc-field-label">View</div>
            <select
              aria-label="View mode"
              value={viewMode}
              onChange={(e) => setViewMode(e.target.value as ViewMode)}
            >
              <option value="graph">Graph</option>
              <option value="list">List</option>
            </select>
          </label>
          <label className="oc-field">
            <div className="oc-field-label">Graph kind</div>
            <select
              aria-label="Graph kind"
              value={graphKind}
              onChange={(e) => {
                const next = e.target.value as GraphKindOption;
                setGraphKind(next);
                if (
                  next !== "neighborhood" &&
                  next !== "individual" &&
                  next !== "query_result" &&
                  next !== "refactor_preview"
                ) {
                  setRootIris(undefined);
                }
              }}
            >
              {GRAPH_KINDS.map((k) => (
                <option key={k} value={k}>
                  {k}
                </option>
              ))}
            </select>
          </label>
          <label className="oc-field">
            <div className="oc-field-label">Mode</div>
            <select
              aria-label="Mode"
              value={graphMode}
              onChange={(e) => setGraphMode(e.target.value as typeof graphMode)}
            >
              <option value="asserted">Asserted</option>
              <option value="inferred">Inferred only</option>
              <option value="combined">Combined</option>
            </select>
          </label>
          <label className="oc-field">
            <div className="oc-field-label">Layout</div>
            <select
              aria-label="Layout"
              value={layout}
              onChange={(e) => setLayout(e.target.value as typeof layout)}
            >
              <option value="grid">Grid</option>
              <option value="circle">Circle</option>
              <option value="stack">Stack by kind</option>
            </select>
          </label>
          <RangeField
            label="Depth"
            value={depth}
            min={1}
            max={5}
            onChange={setDepth}
          />
          <CheckboxRow
            label="Hide deprecated"
            checked={hideDeprecated}
            onChange={setHideDeprecated}
          />
          <CheckboxRow
            label="Unsatisfiable overlay"
            checked={showUnsatOverlay}
            onChange={setShowUnsatOverlay}
          />
          <label className="oc-field">
            <div className="oc-field-label">Ontology IRI</div>
            <input
              aria-label="Ontology IRI filter"
              value={ontologyIriFilter}
              onChange={(e) => setOntologyIriFilter(e.target.value)}
              placeholder="Filter to ontology id…"
            />
          </label>
          <label className="oc-field">
            <div className="oc-field-label">Entity kinds</div>
            <input
              aria-label="Entity kinds filter"
              value={entityKindFilter}
              onChange={(e) => setEntityKindFilter(e.target.value)}
              placeholder="class,individual,…"
            />
          </label>
          <label className="oc-field">
            <div className="oc-field-label">Namespaces</div>
            <input
              aria-label="Namespaces filter"
              value={namespaceFilter}
              onChange={(e) => setNamespaceFilter(e.target.value)}
              placeholder="http://example.org#"
            />
          </label>
          <label className="oc-field">
            <div className="oc-field-label">Relationship kinds</div>
            <input
              aria-label="Relationship kinds filter"
              value={relationshipFilter}
              onChange={(e) => setRelationshipFilter(e.target.value)}
              placeholder="sub_class_of,domain,…"
            />
          </label>
          <label className="oc-field">
            <div className="oc-field-label">Search</div>
            <input
              aria-label="Search"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="Filter / dim by label or IRI…"
            />
          </label>
          <ButtonBar>
            <button type="button" aria-label="Back" onClick={() => goHistory(-1)}>
              Back
            </button>
            <button type="button" aria-label="Forward" onClick={() => goHistory(1)}>
              Forward
            </button>
            <button
              type="button"
              aria-label="Fit to view"
              onClick={() => {
                fittedForGraph.current = null;
                fitGraphIntoView();
              }}
            >
              Fit to view
            </button>
            <button type="button" aria-label="Refresh graph" onClick={requestGraph}>
              Refresh graph
            </button>
            <button
              type="button"
              aria-label="Expand graph depth"
              onClick={() => {
                setDepth((d) => Math.min(5, d + 1));
              }}
            >
              Expand
            </button>
            <button
              type="button"
              aria-label="Copy graph JSON"
              onClick={async () => {
                if (!graph) {
                  return;
                }
                const text = JSON.stringify(graph, null, 2);
                await navigator.clipboard.writeText(text);
              }}
            >
              Copy JSON
            </button>
            <button
              type="button"
              aria-label="Export graph as JSON"
              onClick={() => {
                if (!graph) {
                  return;
                }
                host.postToCore({
                  type: "exportGraph",
                  format: "json",
                  payload: JSON.stringify(graph, null, 2),
                  suggestedName: `strixonomy-${graphKind}-graph.json`,
                });
              }}
            >
              Export JSON…
            </button>
            <button
              type="button"
              aria-label="Export graph as CSV"
              onClick={() => {
                if (!graph) {
                  return;
                }
                const header = "row_kind,id,label,node_kind,source,target,inferred\n";
                const nodeRows = graph.nodes
                  .map(
                    (n) =>
                      `node,${csvCell(n.id)},${csvCell(n.label ?? "")},${csvCell(n.kind)},,,`
                  )
                  .join("\n");
                const edgeRows = graph.edges
                  .map(
                    (e) =>
                      `edge,,,,${csvCell(e.source)},${csvCell(e.target)},${e.inferred ? "true" : "false"}`
                  )
                  .join("\n");
                host.postToCore({
                  type: "exportGraph",
                  format: "csv",
                  payload: `${header}${nodeRows}\n${edgeRows}\n`,
                  suggestedName: `strixonomy-${graphKind}-graph.csv`,
                });
              }}
            >
              Export CSV…
            </button>
            <button
              type="button"
              onClick={() => {
                if (!filteredNodeIds || filteredNodeIds.size === 0) {
                  return;
                }
                const id = [...filteredNodeIds][0];
                setSelectedId(id);
                const node = nodes.find((n) => n.id === id);
                if (node) {
                  rf.current?.fitView({
                    nodes: [node],
                    padding: 0.5,
                    duration: 300,
                  });
                }
              }}
            >
              Center match
            </button>
          </ButtonBar>
        </Section>

        {selectedNode ? (
          <Section title="Selected node" card>
            <Card variant="inset">
              <p>
                <InlineCode>{selectedNode.label}</InlineCode>
                {showUnsatOverlay && unsatSet.has(selectedNode.id) ? (
                  <Badge variant="warning">Unsatisfiable</Badge>
                ) : null}
              </p>
              <p className="oc-muted">{selectedNode.id}</p>
            </Card>
            <ButtonBar>
              <button
                type="button"
                onClick={() => inspectEntity(selectedNode.id)}
              >
                Inspect entity
              </button>
              <button
                type="button"
                onClick={() => expandNeighborhood(selectedNode.id)}
              >
                Expand neighborhood
              </button>
            </ButtonBar>
          </Section>
        ) : (
          <Callout variant="info">
            Click a node on the canvas to inspect it. Use arrow keys when the
            canvas is focused.
          </Callout>
        )}
      </aside>
    </PanelMain>
  );
}

function layoutNodes(
  graph: GraphPayload,
  layout: "grid" | "circle" | "stack",
  opts: {
    searchIds: Set<string> | null;
    unsatisfiable: Set<string>;
    selectedIds: Set<string>;
  }
): Node[] {
  const decorate = (node: GraphPayload["nodes"][number], position: { x: number; y: number }): Node => {
    const dimmed =
      opts.searchIds !== null && !opts.searchIds.has(node.id);
    const unsat = opts.unsatisfiable.has(node.id);
    const selected = opts.selectedIds.has(node.id);
    const classes = [
      "oc-graph-node",
      dimmed ? "oc-graph-node--dimmed" : "",
      unsat ? "oc-graph-node--unsat" : "",
      selected ? "oc-graph-node--selected" : "",
    ]
      .filter(Boolean)
      .join(" ");
    return {
      id: node.id,
      position,
      data: {
        label: unsat ? `⚠ ${node.label || node.id}` : node.label || node.id,
      },
      className: classes,
      style: dimmed ? { opacity: 0.25 } : undefined,
      selected,
    };
  };

  if (layout === "circle") {
    const r = 320;
    const n = Math.max(1, graph.nodes.length);
    return graph.nodes.map((node, i) => {
      const angle = (2 * Math.PI * i) / n;
      return decorate(node, {
        x: Math.cos(angle) * r,
        y: Math.sin(angle) * r,
      });
    });
  }
  if (layout === "stack") {
    const byKind = new Map<string, number>();
    const colByKind = new Map<string, number>();
    const kinds = Array.from(new Set(graph.nodes.map((n) => n.kind)));
    kinds.forEach((k, i) => colByKind.set(k, i));
    return graph.nodes.map((n) => {
      const row = byKind.get(n.kind) ?? 0;
      byKind.set(n.kind, row + 1);
      const col = colByKind.get(n.kind) ?? 0;
      return decorate(n, { x: col * 240, y: row * 110 });
    });
  }
  const byKind = new Map<string, number>();
  return graph.nodes.map((n, i) => {
    const row = byKind.get(n.kind) ?? 0;
    byKind.set(n.kind, row + 1);
    return decorate(n, {
      x: (i % 8) * 180,
      y: row * 100 + (n.kind === "ontology" ? 0 : 40),
    });
  });
}

function toFlowEdges(
  graph: GraphPayload,
  mode: "asserted" | "inferred" | "combined",
  searchIds: Set<string> | null,
  prefersReducedMotion = false
): Edge[] {
  return graph.edges
    .filter((e) => {
      if (mode === "combined") {
        return true;
      }
      if (mode === "asserted") {
        return !e.inferred;
      }
      return e.inferred;
    })
    .filter((e) => {
      if (!searchIds) {
        return true;
      }
      return searchIds.has(e.source) && searchIds.has(e.target);
    })
    .map((e, i) => ({
      id: `${e.source}-${e.target}-${e.kind}-${i}`,
      source: e.source,
      target: e.target,
      label: e.kind,
      animated: e.inferred && !prefersReducedMotion,
      className: e.inferred
        ? "oc-graph-edge oc-graph-edge--inferred"
        : "oc-graph-edge",
    }));
}

function csvCell(value: string): string {
  if (/[",\n]/.test(value)) {
    return `"${value.replace(/"/g, '""')}"`;
  }
  return value;
}
