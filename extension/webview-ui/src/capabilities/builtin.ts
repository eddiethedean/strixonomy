import { register } from "./registry";
import type { DiagnosticsProvider, QueryProvider, ReasoningProvider } from "./types";

/** Register built-in Strixonomy capability stubs (delegate to extension host / LSP). */
export function registerBuiltinProviders(): void {
  const strixonomy: ReasoningProvider & QueryProvider & DiagnosticsProvider = {
    id: "strixonomy",
    version: "0.27.0",
    capabilities: ["reasoning", "query", "diagnostics"],
    async classify() {
      return { delegated: true, provider: "strixonomy" };
    },
    async runSql() {
      return { delegated: true, provider: "strixonomy" };
    },
    async validate() {
      return { delegated: true, provider: "strixonomy" };
    },
  };
  register(strixonomy);
}
