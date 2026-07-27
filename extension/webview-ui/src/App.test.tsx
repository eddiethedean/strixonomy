import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import App from "./App";

function renderApp(): ReturnType<typeof render> {
  return render(<App />);
}

function setPanelQuery(panel: string): void {
  window.history.replaceState({}, "", `/?panel=${panel}`);
}

describe("App", () => {
  it("renders smoke panel by default", () => {
    window.history.replaceState({}, "", "/");
    renderApp();
    expect(screen.getByRole("heading", { name: "Strixonomy React" })).toBeInTheDocument();
  });

  it("routes to query workbench panel", () => {
    setPanelQuery("queryWorkbench");
    renderApp();
    expect(screen.getByRole("heading", { name: "Query Workbench" })).toBeInTheDocument();
  });

  it("routes to entity inspector panel", () => {
    setPanelQuery("inspector");
    renderApp();
    expect(screen.getByRole("status")).toHaveTextContent("Loading entity…");
  });

  it("routes to semantic diff panel", () => {
    setPanelQuery("semanticDiff");
    renderApp();
    expect(screen.getByRole("status")).toHaveTextContent("Computing semantic diff…");
  });

  it("routes to refactor preview panel", () => {
    setPanelQuery("refactorPreview");
    renderApp();
    expect(screen.getByRole("status")).toHaveTextContent("Loading refactor preview…");
  });

  it("routes to manchester editor panel", () => {
    setPanelQuery("manchesterEditor");
    renderApp();
    expect(screen.getByRole("heading", { name: "Manchester Axiom Editor" })).toBeInTheDocument();
  });

  it("routes to graph panel", () => {
    setPanelQuery("graph");
    renderApp();
    expect(screen.getByText("Overview")).toBeInTheDocument();
    expect(screen.getByText("No graph data")).toBeInTheDocument();
  });

  it("routes to reasoner panel", () => {
    setPanelQuery("reasoner");
    renderApp();
    expect(screen.getByRole("heading", { name: "Reasoner" })).toBeInTheDocument();
  });

  it("routes to explanation panel", () => {
    setPanelQuery("explanation");
    renderApp();
    expect(screen.getByRole("status")).toHaveTextContent("Loading explanation…");
  });

  it("falls back to smoke for unknown panel param", () => {
    setPanelQuery("unknown");
    renderApp();
    expect(screen.getByRole("heading", { name: "Strixonomy React" })).toBeInTheDocument();
  });

  it("routes to inspector after host location bootstrap (VS Code webview)", () => {
    window.history.replaceState({}, "", "/");
    expect(window.location.search).toBe("");
    // Host inline script sets page query before React loads (see getWebviewHtml.ts).
    history.replaceState(null, "", "?panel=inspector");
    renderApp();
    expect(screen.getByRole("status")).toHaveTextContent("Loading entity…");
    expect(
      screen.queryByText("Webview foundation is active.")
    ).not.toBeInTheDocument();
  });

  it("routes to inspector when VS Code pre-populates unrelated query params", () => {
    window.history.replaceState({}, "", "/?vscodeWebviewId=abc");
    // Bootstrap merges panel=inspector into existing search (getWebviewHtml.ts).
    const merged = new URLSearchParams(window.location.search);
    merged.set("panel", "inspector");
    history.replaceState(null, "", `?${merged.toString()}`);
    renderApp();
    expect(screen.getByRole("status")).toHaveTextContent("Loading entity…");
    expect(
      screen.queryByText("Webview foundation is active.")
    ).not.toBeInTheDocument();
  });
});
