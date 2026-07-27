import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { HostProvider } from "../context/HostContext";
import type { WorkspaceHost } from "../host";
import { SmokePanel } from "../panels/SmokePanel";

function createMockHost(): WorkspaceHost & { messages: unknown[] } {
  const messages: unknown[] = [];
  return {
    messages,
    postToCore(message) {
      messages.push(message);
    },
    getTheme: () => "dark",
    showNotification: () => {},
    onMessage: () => () => {},
  };
}

describe("HostContext", () => {
  it("provides host to SmokePanel", () => {
    const host = createMockHost();
    render(
      <HostProvider host={host}>
        <SmokePanel />
      </HostProvider>
    );
    expect(screen.getByRole("heading", { name: "Strixonomy React" })).toBeInTheDocument();
    expect(host.messages).toContainEqual({ type: "ready", panel: "smoke" });
  });
});
