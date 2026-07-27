import { describe, expect, it, beforeEach } from "vitest";
import { clearRegistryForTests, list, register } from "./registry";
import { registerBuiltinProviders } from "./builtin";

describe("capability registry", () => {
  beforeEach(() => {
    clearRegistryForTests();
  });

  it("registers built-in ontocore provider", () => {
    registerBuiltinProviders();
    const providers = list();
    expect(providers.some((p) => p.id === "strixonomy")).toBe(true);
  });

  it("lists providers by capability kind", () => {
    register({
      id: "test.query",
      version: "0.1.0",
      capabilities: ["query"],
    });
    expect(list("query")).toHaveLength(1);
    expect(list("reasoning")).toHaveLength(0);
  });
});
