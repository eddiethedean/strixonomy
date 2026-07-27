import { render, screen, waitFor } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { HostProvider } from "../context/HostContext";
import { SmokePanel } from "../panels/SmokePanel";
import { postedMessages } from "../test/fixtures";

describe("SmokePanel", () => {
  it("renders brand content and posts ready message", async () => {
    render(
      <HostProvider>
        <SmokePanel />
      </HostProvider>
    );

    expect(screen.getByRole("heading", { name: "Strixonomy React" })).toBeInTheDocument();
    expect(screen.getByText("Webview foundation is active.")).toBeInTheDocument();

    await waitFor(() => {
      expect(postedMessages()).toContainEqual({ type: "ready", panel: "smoke" });
    });
  });
});
