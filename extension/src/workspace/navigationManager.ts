import { focusRelay } from "../focus/focusRelay";
import { workspaceEventBus } from "./eventBus";
import type { NavigationEntry } from "./types";

const MAX_NAV = 50;

export class NavigationManager {
  private stack: NavigationEntry[] = [];
  private index = -1;
  /** While > 0, focus-driven push must not mutate history (#292). */
  private historyPushSuppressed = 0;
  private context: { workspaceState: { get<T>(key: string): T | undefined; update(key: string, value: unknown): Thenable<void> } } | undefined;

  bindContext(context: {
    workspaceState: {
      get<T>(key: string): T | undefined;
      update(key: string, value: unknown): Thenable<void>;
    };
  }): void {
    this.context = context;
    const saved = context.workspaceState.get<{
      stack: NavigationEntry[];
      index: number;
    }>("strixonomy.navigation");
    if (saved?.stack?.length) {
      this.stack = saved.stack.slice(-MAX_NAV);
      this.index = Math.min(saved.index, this.stack.length - 1);
    }
  }

  getStack(): NavigationEntry[] {
    return [...this.stack];
  }

  getIndex(): number {
    return this.index;
  }

  canGoBack(): boolean {
    return this.index > 0;
  }

  canGoForward(): boolean {
    return this.index >= 0 && this.index < this.stack.length - 1;
  }

  /** Suppress focus→push feedback while applying Back/Forward (and follow-up openEntity). */
  beginHistoryNavigation(): void {
    this.historyPushSuppressed += 1;
  }

  endHistoryNavigation(): void {
    this.historyPushSuppressed = Math.max(0, this.historyPushSuppressed - 1);
  }

  async runHistoryNavigation<T>(fn: () => Promise<T> | T): Promise<T> {
    this.beginHistoryNavigation();
    try {
      return await fn();
    } finally {
      this.endHistoryNavigation();
    }
  }

  push(entry: NavigationEntry): NavigationEntry {
    if (this.historyPushSuppressed > 0) {
      return entry;
    }
    // Re-focusing the current history slot must not truncate forward history (#292).
    const current = this.index >= 0 ? this.stack[this.index] : undefined;
    if (current && current.kind === entry.kind && current.id === entry.id) {
      return current;
    }
    if (this.index < this.stack.length - 1) {
      this.stack = this.stack.slice(0, this.index + 1);
    }
    const last = this.stack[this.stack.length - 1];
    if (last && last.kind === entry.kind && last.id === entry.id) {
      return last;
    }
    this.stack.push(entry);
    if (this.stack.length > MAX_NAV) {
      this.stack.shift();
    }
    this.index = this.stack.length - 1;
    void this.persist();
    workspaceEventBus.publish("NavigationChanged", {
      stack: this.getStack(),
      index: this.index,
    });
    return entry;
  }

  back(): NavigationEntry | undefined {
    if (!this.canGoBack()) {
      return undefined;
    }
    this.index -= 1;
    const entry = this.stack[this.index];
    void this.persist();
    this.applyEntry(entry);
    workspaceEventBus.publish("NavigationChanged", {
      stack: this.getStack(),
      index: this.index,
    });
    return entry;
  }

  forward(): NavigationEntry | undefined {
    if (!this.canGoForward()) {
      return undefined;
    }
    this.index += 1;
    const entry = this.stack[this.index];
    void this.persist();
    this.applyEntry(entry);
    workspaceEventBus.publish("NavigationChanged", {
      stack: this.getStack(),
      index: this.index,
    });
    return entry;
  }

  trackEntityFocus(iri: string, source: string, label?: string): void {
    this.push({ kind: "entity", id: iri, source, label });
  }

  hydrate(stack: NavigationEntry[], index: number): void {
    this.stack = stack.slice(-MAX_NAV);
    this.index = Math.min(index, this.stack.length - 1);
    void this.persist();
  }

  resetForTests(): void {
    this.stack = [];
    this.index = -1;
    this.historyPushSuppressed = 0;
    this.context = undefined;
  }

  private applyEntry(entry: NavigationEntry | undefined): void {
    if (!entry) {
      return;
    }
    if (entry.kind === "entity") {
      focusRelay.setEntityFocus(entry.id, entry.source);
    }
  }

  private async persist(): Promise<void> {
    if (!this.context) {
      return;
    }
    await this.context.workspaceState.update("strixonomy.navigation", {
      stack: this.stack,
      index: this.index,
    });
  }
}

export const navigationManager = new NavigationManager();
