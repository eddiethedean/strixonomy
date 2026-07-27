import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  hasPatchFailureDiagnostics,
  isPatchFullySynced,
  patchFailureMessage,
  patchSyncCancelledMessage,
  requirePatchFullySynced,
} from "./patchFeedback";
import type { ApplyAxiomPatchClientResult, ApplyPatchResult } from "./protocol";

describe("patchFeedback", () => {
  it("uses first diagnostic message when present", () => {
    const result: ApplyPatchResult = {
      applied: false,
      diagnostics: [{ severity: "error", message: "entity not found" }],
    };
    assert.equal(patchFailureMessage(result), "entity not found");
    assert.equal(hasPatchFailureDiagnostics(result), true);
  });

  it("falls back when diagnostics are empty", () => {
    const result: ApplyPatchResult = { applied: false };
    assert.equal(
      patchFailureMessage(result),
      "Strixonomy: patch was not applied"
    );
    assert.equal(hasPatchFailureDiagnostics(result), false);
  });

  it("isPatchFullySynced requires applied and editor_synced", () => {
    const synced: ApplyAxiomPatchClientResult = {
      applied: true,
      editor_synced: true,
    };
    const cancelled: ApplyAxiomPatchClientResult = {
      applied: true,
      editor_synced: false,
    };
    const failed: ApplyAxiomPatchClientResult = {
      applied: false,
      editor_synced: true,
    };
    assert.equal(isPatchFullySynced(synced), true);
    assert.equal(isPatchFullySynced(cancelled), false);
    assert.equal(isPatchFullySynced(failed), false);
  });

  it("requirePatchFullySynced throws when editor sync was cancelled", () => {
    const cancelled: ApplyAxiomPatchClientResult = {
      applied: true,
      editor_synced: false,
    };
    assert.throws(
      () => requirePatchFullySynced(cancelled),
      (error: unknown) =>
        error instanceof Error &&
        error.message === patchSyncCancelledMessage()
    );
  });

  it("requirePatchFullySynced throws when patch was not applied", () => {
    const failed: ApplyAxiomPatchClientResult = {
      applied: false,
      editor_synced: false,
      diagnostics: [{ severity: "error", message: "duplicate prefix" }],
    };
    assert.throws(
      () => requirePatchFullySynced(failed),
      (error: unknown) =>
        error instanceof Error && error.message === "duplicate prefix"
    );
  });

  it("requirePatchFullySynced accepts fully synced results", () => {
    requirePatchFullySynced({ applied: true, editor_synced: true });
  });
});
