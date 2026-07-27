import * as vscode from "vscode";

let channel: vscode.OutputChannel | undefined;

export function getErrorLog(): vscode.OutputChannel {
  channel ??= vscode.window.createOutputChannel("Strixonomy");
  return channel;
}

export function appendError(error: unknown, context?: string): void {
  const message = error instanceof Error ? error.stack ?? error.message : String(error);
  getErrorLog().appendLine(
    `[${new Date().toISOString()}]${context ? ` ${context}:` : ""} ${message}`
  );
}

export function openErrorLog(): void {
  getErrorLog().show(true);
}

export function registerErrorLog(context: vscode.ExtensionContext): void {
  context.subscriptions.push(getErrorLog());
}
