import { screen, waitFor, fireEvent } from "@testing-library/react";
import { renderWithProviders } from "../test/render";
import userEvent from "@testing-library/user-event";
import { describe, it, expect } from "vitest";
import { GraphPanel } from "./GraphPanel";
import {
  graphPayload,
  graphWithInferredEdge,
  lastPostedMessage,
  postHostMessage,
  postedMessages,
  postedMessagesOfType,
} from "../test/fixtures";

describe("GraphPanel", () => {
  it("posts ready on mount and shows empty state", async () => {
    renderWithProviders(<GraphPanel />);

    expect(screen.getByText("No graph data")).toBeInTheDocument();

    await waitFor(() => {
      expect(postedMessages()[0]).toEqual({ type: "ready", panel: "graph" });
    });
  });

  it("requests graph on init when no data loaded", async () => {
    renderWithProviders(<GraphPanel />);

    postHostMessage({ type: "init", panel: "graph" });

    await waitFor(() => {
      expect(lastPostedMessage()).toMatchObject({
        type: "requestGraph",
        graphKind: "class",
        depth: 2,
        includeInferred: false,
        filters: { hide_deprecated: false },
      });
    });
  });

  it("renders graph canvas when data arrives", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "graphData", graph: graphPayload });

    await waitFor(() => {
      expect(document.querySelector(".react-flow")).toBeInTheDocument();
    });
    await waitFor(() => {
      expect(document.querySelectorAll(".react-flow__node").length).toBeGreaterThan(0);
    });
    expect(screen.getByLabelText("Graph kind")).toHaveValue("class");
    expect(screen.getByText("Person")).toBeInTheDocument();
  });

  it("does not mount React Flow until laid-out nodes are ready", async () => {
    renderWithProviders(<GraphPanel />);
    expect(document.querySelector(".react-flow")).not.toBeInTheDocument();

    postHostMessage({ type: "graphData", graph: graphPayload });

    await waitFor(() => {
      expect(document.querySelector(".react-flow")).toBeInTheDocument();
    });
    // IRI node ids with '#' must still appear as RF nodes (#442).
    expect(
      document.querySelector('.react-flow__node[data-id="http://example.org#Person"]')
    ).toBeInTheDocument();
  });

  it("shows truncated badge when graph is truncated", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({
      type: "graphData",
      graph: { ...graphPayload, truncated: true },
    });

    expect(
      await screen.findByText("Truncated — focus root / reduce depth")
    ).toBeInTheDocument();
  });

  it("shows error empty state on host error", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "error", message: "Index missing" });

    expect(await screen.findByText("Graph error")).toBeInTheDocument();
    expect(screen.getByText("Index missing")).toBeInTheDocument();
  });

  it("refresh sends updated filter options", async () => {
    const user = userEvent.setup();
    renderWithProviders(<GraphPanel />);

    await user.selectOptions(screen.getByLabelText("Mode"), "combined");
    await user.click(screen.getByRole("button", { name: "Refresh graph" }));

    expect(lastPostedMessage()).toMatchObject({
      type: "requestGraph",
      includeInferred: true,
    });
  });

  it("updates depth via range control", () => {
    renderWithProviders(<GraphPanel />);

    const slider = screen.getByRole("slider");
    fireEvent.change(slider, { target: { value: "4" } });

    expect(screen.getByText("4")).toBeInTheDocument();
  });

  it("reads graphKind and root from URL query params", async () => {
    window.history.replaceState(
      {},
      "",
      "/?graphKind=property&root=http://example.org%23Person"
    );
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "init", panel: "graph" });

    await waitFor(() => {
      expect(lastPostedMessage()).toMatchObject({
        type: "requestGraph",
        graphKind: "property",
        rootIri: "http://example.org#Person",
      });
    });
  });

  it("updates rootIri when graphData includes rootIri", async () => {
    const user = userEvent.setup();
    renderWithProviders(<GraphPanel />);
    postHostMessage({
      type: "graphData",
      graph: graphPayload,
      rootIri: "http://example.org#Person",
    });

    await waitFor(() => {
      expect(document.querySelector(".react-flow")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Refresh graph" }));
    expect(lastPostedMessage()).toMatchObject({
      type: "requestGraph",
      rootIri: "http://example.org#Person",
    });
  });

  it("does not auto-request graph on init after data is loaded", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "graphData", graph: graphPayload });
    await waitFor(() => expect(document.querySelector(".react-flow")).toBeInTheDocument());

    const countBefore = postedMessagesOfType("requestGraph").length;
    postHostMessage({ type: "init", panel: "graph" });
    expect(postedMessagesOfType("requestGraph").length).toBe(countBefore);
  });

  it("includes hide_deprecated filter when toggled", async () => {
    const user = userEvent.setup();
    renderWithProviders(<GraphPanel />);

    await user.click(screen.getByLabelText("Hide deprecated"));
    await user.click(screen.getByRole("button", { name: "Refresh graph" }));

    expect(lastPostedMessage()).toMatchObject({
      type: "requestGraph",
      filters: { hide_deprecated: true },
    });
  });

  it("shows empty state when graph has zero nodes", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({
      type: "graphData",
      graph: { ...graphPayload, nodes: [] },
    });

    expect(await screen.findByText("No graph data")).toBeInTheDocument();
    expect(document.querySelector(".react-flow")).not.toBeInTheDocument();
  });

  it("renders with inferred edges without crashing", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "graphData", graph: graphWithInferredEdge });

    await waitFor(() => {
      expect(document.querySelector(".react-flow")).toBeInTheDocument();
    });

    const user = userEvent.setup();
    await user.selectOptions(screen.getByLabelText("Mode"), "combined");
    expect(document.querySelector(".react-flow")).toBeInTheDocument();
  });

  it("ignores invalid host messages", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "graphData", graph: graphPayload });
    await waitFor(() => expect(document.querySelector(".react-flow")).toBeInTheDocument());

    postHostMessage(null as never);
    postHostMessage({ type: "init", panel: "inspector" } as never);
    expect(document.querySelector(".react-flow")).toBeInTheDocument();
  });

  it("keeps class graph kind when entity focus arrives", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "graphData", graph: graphPayload });
    await waitFor(() => {
      expect(screen.getByLabelText("Graph kind")).toHaveValue("class");
    });

    postHostMessage({
      type: "focusState",
      focus: {
        kind: "entity",
        id: "http://example.org#Agent",
        source: "explorer",
        timestamp: Date.now(),
      },
    });

    await waitFor(() => {
      expect(screen.getByLabelText("Graph kind")).toHaveValue("class");
    });
  });

  it("switches to list alternate view", async () => {
    const user = userEvent.setup();
    renderWithProviders(<GraphPanel />);
    postHostMessage({ type: "graphData", graph: graphPayload });
    await waitFor(() => expect(document.querySelector(".react-flow")).toBeInTheDocument());

    await user.selectOptions(screen.getByLabelText("View mode"), "list");
    expect(await screen.findByRole("table", { name: "Graph list alternate" })).toBeInTheDocument();
    expect(screen.getByText("Person")).toBeInTheDocument();
  });

  it("shows improved truncated guidance", async () => {
    renderWithProviders(<GraphPanel />);
    postHostMessage({
      type: "graphData",
      graph: { ...graphPayload, truncated: true },
    });
    expect(
      await screen.findByText("Truncated — focus root / reduce depth")
    ).toBeInTheDocument();
  });

  it("includes ontology IRI filter in request", async () => {
    const user = userEvent.setup();
    renderWithProviders(<GraphPanel />);
    await user.type(screen.getByLabelText("Ontology IRI filter"), "http://ex/onto");
    await user.click(screen.getByRole("button", { name: "Refresh graph" }));
    expect(lastPostedMessage()).toMatchObject({
      type: "requestGraph",
      filters: { ontology_iri: "http://ex/onto", hide_deprecated: false },
    });
  });
});
