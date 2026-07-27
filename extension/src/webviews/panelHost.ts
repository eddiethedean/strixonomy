import * as vscode from "vscode";
import { getWebviewHtml } from "./getWebviewHtml";
import { focusRelay } from "../focus/focusRelay";
import type { HostMessage, PanelKind, WebviewMessage } from "./messages";
import { isWebviewMessage } from "./messages";

export interface PanelHostOptions {
  viewType: string;
  title: string;
  panel: PanelKind;
  column?: vscode.ViewColumn;
  extraQuery?: Record<string, string>;
  onMessage?: (message: WebviewMessage, panel: vscode.WebviewPanel) => void | Promise<void>;
}

export class PanelHost {
  private static openByKind = new Map<PanelKind, PanelHost>();

  private disposed = false;
  private webviewReady = false;
  private pendingMessages: HostMessage[] = [];
  private processMessage?: (data: unknown) => Promise<void>;

  constructor(
    public readonly panel: vscode.WebviewPanel,
    private readonly extensionUri: vscode.Uri,
    public readonly panelKind: PanelKind
  ) {
    panel.onDidDispose(() => {
      this.disposed = true;
      this.unregisterFocus?.();
      if (PanelHost.openByKind.get(this.panelKind) === this) {
        PanelHost.openByKind.delete(this.panelKind);
      }
    });
    this.unregisterFocus = focusRelay.registerHost(this);
  }

  private unregisterFocus?: () => void;

  get isDisposed(): boolean {
    return this.disposed;
  }

  /** True after the webview posted `{ type: "ready", panel: … }`. */
  isWebviewReady(): boolean {
    return this.webviewReady;
  }

  getWebviewHtml(): string {
    return this.panel.webview.html;
  }

  /** Post a host message; buffers until the webview sends `ready`. */
  postMessage(message: HostMessage): void {
    if (this.disposed) {
      return;
    }
    if (!this.webviewReady) {
      this.pendingMessages.push(message);
      return;
    }
    void this.panel.webview.postMessage(message);
  }

  private flushPending(): void {
    if (this.disposed) {
      return;
    }
    const queued = this.pendingMessages;
    this.pendingMessages = [];
    for (const message of queued) {
      void this.panel.webview.postMessage(message);
    }
  }

  private onWebviewReady(): void {
    if (this.webviewReady || this.disposed) {
      return;
    }
    this.webviewReady = true;
    void this.panel.webview.postMessage({ type: "init", panel: this.panelKind });
    focusRelay.syncHost(this);
    this.flushPending();
  }

  /** Open host for a panel kind, if any. */
  static getOpen(kind: PanelKind): PanelHost | undefined {
    return PanelHost.openByKind.get(kind);
  }

  /** Dispose registered hosts for the given kinds (no-op if not open). */
  static disposeKinds(kinds: PanelKind[]): void {
    for (const kind of kinds) {
      const host = PanelHost.openByKind.get(kind);
      if (host && !host.isDisposed) {
        host.panel.dispose();
      }
    }
  }

  /**
   * Deliver a message into the same validation + handler path as
   * `webview.onDidReceiveMessage` (VS Code e2e injection).
   */
  async deliverMessageForTests(data: unknown): Promise<void> {
    if (!this.processMessage) {
      throw new Error(`PanelHost(${this.panelKind}) has no message processor`);
    }
    await this.processMessage(data);
  }

  static create(
    extensionUri: vscode.Uri,
    options: PanelHostOptions
  ): PanelHost {
    const panel = vscode.window.createWebviewPanel(
      options.viewType,
      options.title,
      options.column ?? vscode.ViewColumn.Beside,
      {
        enableScripts: true,
        retainContextWhenHidden: true,
        localResourceRoots: [
          vscode.Uri.joinPath(extensionUri, "webview-ui", "dist"),
        ],
      }
    );

    panel.webview.options = {
      enableScripts: true,
      localResourceRoots: [
        vscode.Uri.joinPath(extensionUri, "webview-ui", "dist"),
      ],
    };

    panel.webview.html = getWebviewHtml(
      panel.webview,
      extensionUri,
      options.panel,
      options.extraQuery
    );

    const host = new PanelHost(panel, extensionUri, options.panel);
    PanelHost.openByKind.set(options.panel, host);

    const processMessage = async (data: unknown): Promise<void> => {
      if (!isWebviewMessage(data)) {
        return;
      }
      if (data.type === "ready" && data.panel === options.panel) {
        host.onWebviewReady();
      }
      if (data.type === "closeDialog") {
        panel.dispose();
        return;
      }
      if (data.type === "setFocus") {
        focusRelay.setFocus(data.focus, { broadcast: true });
      }
      if (data.type === "showNotification") {
        const level = data.level ?? "info";
        if (level === "error") {
          void vscode.window.showErrorMessage(data.message);
        } else if (level === "warning") {
          void vscode.window.showWarningMessage(data.message);
        } else {
          void vscode.window.showInformationMessage(data.message);
        }
      }
      if (options.onMessage) {
        try {
          await options.onMessage(data, panel);
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          void vscode.window.showErrorMessage(`Strixonomy: ${message}`);
        }
      }
    };

    host.processMessage = processMessage;
    panel.webview.onDidReceiveMessage((data: unknown) => {
      void processMessage(data);
    });

    return host;
  }
}
