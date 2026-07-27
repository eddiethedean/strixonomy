import * as vscode from "vscode";
import { focusRelay } from "../focus/focusRelay";
import { getExplanation } from "../lsp/client";
import type { GetExplanationResult } from "../lsp/protocol";
import {
  isExplanationStale,
  isLatestExplanationFetch,
  resolveExplanationProfile,
} from "./explanationPanelLogic";
import { forgetPanelRestoreState, rememberPanelRestoreState } from "./layoutPersistence";
import type { ExplanationPayload, WebviewMessage } from "./messages";
import { PanelHost } from "./panelHost";

export class ExplanationPanel {
  public static current: ExplanationPanel | undefined;
  /** Monotonic counter so concurrent cold opens share a total order (#276). */
  private static nextFetchGeneration = 0;
  private classIri = "";
  private profile = "el";
  private lastResult: GetExplanationResult | undefined;
  private unsubscribeCatalog?: () => void;
  private fetchGeneration = 0;
  private activeFetchGeneration = 0;

  private constructor(
    private readonly host: PanelHost,
    private readonly extensionUri: vscode.Uri
  ) {
    this.host.panel.onDidDispose(() => {
      void forgetPanelRestoreState("strixonomyExplanation");
      this.unsubscribeCatalog?.();
      ExplanationPanel.current = undefined;
    });
    this.unsubscribeCatalog = focusRelay.subscribeCatalog(() => {
      if (this.activeFetchGeneration !== 0) {
        return;
      }
      if (this.lastResult) {
        this.pushContent(this.classIri, this.lastResult, this.profile);
      }
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static async show(
    extensionUri: vscode.Uri,
    classIri: string,
    profileOverride?: string
  ): Promise<void> {
    const cfg = vscode.workspace.getConfiguration("strixonomy");
    const profile = resolveExplanationProfile({
      explicit: profileOverride,
      lastRunProfile: focusRelay.getReasoning()?.profile,
      settingsDefault: cfg.get<string>("reasoner.default"),
    });
    const generation = ++ExplanationPanel.nextFetchGeneration;
    if (ExplanationPanel.current) {
      ExplanationPanel.current.fetchGeneration = generation;
      ExplanationPanel.current.activeFetchGeneration = generation;
    }

    const result = await getExplanation({ class_iri: classIri, profile });

    // A newer show() was started while we awaited — discard this result (#276).
    if (!isLatestExplanationFetch(generation, ExplanationPanel.nextFetchGeneration)) {
      return;
    }

    if (ExplanationPanel.current) {
      ExplanationPanel.current.activeFetchGeneration = 0;
      ExplanationPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      ExplanationPanel.current.setContent(classIri, result, profile);
      return;
    }

    const host = PanelHost.create(extensionUri, {
      viewType: "strixonomyExplanation",
      title: `Explanation: ${classIri.split(/[#/]/).pop() ?? classIri}`,
      panel: "explanation",
      onMessage: async (message: WebviewMessage) => {
        const panel = ExplanationPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });
    const view = new ExplanationPanel(host, extensionUri);
    view.fetchGeneration = generation;
    ExplanationPanel.current = view;
    view.setContent(classIri, result, profile);
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "copyText") {
      await vscode.env.clipboard.writeText(message.text);
      return;
    }
    if (message.type === "rerunReasoner") {
      await vscode.commands.executeCommand("strixonomy.runReasoner");
      if (this.classIri) {
        await ExplanationPanel.show(this.extensionUri, this.classIri, this.profile);
      }
      return;
    }
    if (message.type === "openEntity") {
      await vscode.commands.executeCommand("strixonomy.openEntity", message.iri);
    }
  }

  private setContent(
    classIri: string,
    result: GetExplanationResult,
    profile: string
  ): void {
    this.classIri = classIri;
    this.profile = profile;
    this.lastResult = result;
    focusRelay.setCatalogFingerprint({
      indexedAt: result.indexed_at,
      contentHash: result.content_hash,
    });
    void rememberPanelRestoreState("strixonomyExplanation", {
      command: "strixonomy.showExplanation",
      args: [classIri, profile],
      title: `Explanation: ${classIri.split(/[#/]/).pop() ?? classIri}`,
    });
    this.host.panel.title = `Explanation: ${classIri.split(/[#/]/).pop() ?? classIri}`;
    this.pushContent(classIri, result, profile);
  }

  private pushContent(
    classIri: string,
    result: GetExplanationResult,
    profile: string
  ): void {
    const catalog = focusRelay.getCatalogFingerprint();
    const stale = isExplanationStale({
      shownContentHash: result.content_hash,
      shownIndexedAt: result.indexed_at,
      currentContentHash: catalog?.contentHash ?? result.content_hash,
      currentIndexedAt: catalog?.indexedAt ?? result.indexed_at,
    });

    const payload: ExplanationPayload = {
      classIri,
      profile,
      stale,
      indexed_at: result.indexed_at,
      content_hash: result.content_hash,
      justifications: [
        { title: "Justification 1", steps: result.steps, text: result.text },
        ...(result.alternatives ?? []).map((a, i) => ({
          title: `Justification ${i + 2}`,
          steps: a.steps,
          text: a.text,
        })),
      ],
    };
    this.host.postMessage({ type: "loadExplanation", payload });
  }
}
