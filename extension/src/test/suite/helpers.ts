import * as path from "path";
import * as vscode from "vscode";

/** Fixture workspace path (set by runVscodeTest / CI). */
export function fixturesWorkspaceUri(): string {
  const fromEnv =
    process.env.STRIXONOMY_TEST_FIXTURES ??
    process.env.ONTOCODE_TEST_FIXTURES;
  if (fromEnv) {
    return vscode.Uri.file(fromEnv).toString();
  }
  return vscode.Uri.file(
    path.resolve(__dirname, "..", "..", "..", "..", "fixtures")
  ).toString();
}

/** Stable IRIs from fixtures/example.ttl */
export const FIXTURE_IRIS = {
  person: "http://example.org/people#Person",
  organization: "http://example.org/people#Organization",
  worksFor: "http://example.org/people#worksFor",
  alice: "http://example.org/people#alice",
} as const;

/** Match production path equality (Windows case + `\\?\` verbatim prefixes). */
export function pathsEqual(a: string, b: string): boolean {
  const normalize = (filePath: string): string => {
    let normalized = filePath;
    if (normalized.startsWith("\\\\?\\UNC\\")) {
      normalized = `\\\\${normalized.slice("\\\\?\\UNC\\".length)}`;
    } else if (normalized.startsWith("\\\\?\\")) {
      normalized = normalized.slice("\\\\?\\".length);
    }
    return path.resolve(normalized);
  };
  const left = normalize(a);
  const right = normalize(b);
  if (process.platform === "win32") {
    return left.toLowerCase() === right.toLowerCase();
  }
  return left === right;
}
