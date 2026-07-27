import { focusRelay } from "../focus/focusRelay";
import type { CurrentFocus } from "../focus/types";
import { workspaceEventBus } from "./eventBus";
import { navigationManager } from "./navigationManager";

export class SelectionManager {
  private selectionIri: string | undefined;
  private context: { workspaceState: { get<T>(key: string): T | undefined; update(key: string, value: unknown): Thenable<void> } } | undefined;

  bindContext(context: {
    workspaceState: {
      get<T>(key: string): T | undefined;
      update(key: string, value: unknown): Thenable<void>;
    };
  }): void {
    this.context = context;
    const saved = context.workspaceState.get<{ iri?: string }>("strixonomy.selection");
    this.selectionIri = saved?.iri;
  }

  getSelectionIri(): string | undefined {
    const focus = focusRelay.getFocus();
    if (focus?.kind === "entity" || focus?.kind === "graphNode") {
      return focus.id;
    }
    return this.selectionIri;
  }

  setSelectionFromFocus(focus: CurrentFocus): void {
    if (focus.kind !== "entity" && focus.kind !== "graphNode") {
      return;
    }
    this.selectionIri = focus.id;
    void this.persist();
    workspaceEventBus.publish("SelectionChanged", { iri: focus.id });
    navigationManager.trackEntityFocus(focus.id, focus.source);
  }

  hydrate(iri: string | undefined): void {
    this.selectionIri = iri;
    if (iri) {
      focusRelay.setEntityFocus(iri, "session-restore");
    }
    void this.persist();
  }

  subscribeToFocus(): () => void {
    return focusRelay.subscribe((focus) => {
      if (!focus) {
        return;
      }
      workspaceEventBus.publish("FocusChanged", focus);
      this.setSelectionFromFocus(focus);
    });
  }

  resetForTests(): void {
    this.selectionIri = undefined;
    this.context = undefined;
  }

  private async persist(): Promise<void> {
    if (!this.context) {
      return;
    }
    await this.context.workspaceState.update("strixonomy.selection", {
      iri: this.selectionIri,
    });
  }
}

export const selectionManager = new SelectionManager();
