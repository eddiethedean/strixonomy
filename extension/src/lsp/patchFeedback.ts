import type { ApplyAxiomPatchClientResult, ApplyPatchResult } from "./protocol";

/** User-facing message when applyAxiomPatch returns applied: false or preview diagnostics. */
export function patchFailureMessage(result: ApplyPatchResult): string {
  const first = result.diagnostics?.find((d) => d.message.trim().length > 0);
  if (first) {
    return first.message;
  }
  return "Strixonomy: patch was not applied";
}

export function hasPatchFailureDiagnostics(result: ApplyPatchResult): boolean {
  return (result.diagnostics?.length ?? 0) > 0;
}

/** True when patch was written and the open editor buffer was synced (or no sync was needed). */
export function isPatchFullySynced(result: ApplyAxiomPatchClientResult): boolean {
  return result.applied && result.editor_synced;
}

/** User-facing message when disk write succeeded but workspace-edit sync was cancelled. */
export function patchSyncCancelledMessage(): string {
  return "Strixonomy: changes written to disk but editor sync was cancelled";
}

/**
 * Throw when a patch did not fully apply, including cancelled editor sync.
 * Callers that refresh UI / close panels should use this instead of checking `applied` alone.
 */
export function requirePatchFullySynced(result: ApplyAxiomPatchClientResult): void {
  if (!result.applied) {
    throw new Error(patchFailureMessage(result));
  }
  if (!isPatchFullySynced(result)) {
    throw new Error(patchSyncCancelledMessage());
  }
}
