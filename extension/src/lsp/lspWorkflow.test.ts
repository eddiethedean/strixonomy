/**
 * Behavioral LSP workflows: write-back + reasoner → explanation.
 * Uses a real strixonomy-lsp binary (not source-regex guards).
 */
import assert from "node:assert/strict";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process";
import { afterEach, describe, it } from "node:test";
import { resolveLspBinaryForTests } from "./lspTestHarness";

const tempDirs: string[] = [];
const children: ChildProcessWithoutNullStreams[] = [];

afterEach(() => {
  while (children.length > 0) {
    const child = children.pop();
    if (child && !child.killed) {
      child.kill();
    }
  }
  while (tempDirs.length > 0) {
    const dir = tempDirs.pop();
    if (dir) {
      fs.rmSync(dir, { recursive: true, force: true });
    }
  }
});

function writeMsg(stdin: NodeJS.WritableStream, body: object): void {
  const json = JSON.stringify(body);
  stdin.write(`Content-Length: ${Buffer.byteLength(json, "utf8")}\r\n\r\n${json}`);
}

type LspMsg = Record<string, unknown>;

class LspSession {
  private buffer = Buffer.alloc(0);
  private queue: LspMsg[] = [];
  private waiters: Array<(msg: LspMsg) => void> = [];

  constructor(private readonly child: ChildProcessWithoutNullStreams) {
    child.stdout.on("data", (chunk: Buffer) => {
      this.buffer = Buffer.concat([this.buffer, chunk]);
      this.drain();
    });
    // Prevent stderr pipe backpressure from stalling the language server.
    child.stderr.on("data", () => undefined);
  }

  private drain(): void {
    while (true) {
      const headerEnd = this.buffer.indexOf("\r\n\r\n");
      if (headerEnd === -1) {
        return;
      }
      const header = this.buffer.subarray(0, headerEnd).toString("utf8");
      const match = /Content-Length:\s*(\d+)/i.exec(header);
      if (!match) {
        throw new Error(`bad LSP header: ${header}`);
      }
      const length = Number.parseInt(match[1]!, 10);
      const bodyStart = headerEnd + 4;
      if (this.buffer.length < bodyStart + length) {
        return;
      }
      const body = this.buffer.subarray(bodyStart, bodyStart + length).toString("utf8");
      this.buffer = this.buffer.subarray(bodyStart + length);
      const msg = JSON.parse(body) as LspMsg;
      const waiter = this.waiters.shift();
      if (waiter) {
        waiter(msg);
      } else {
        this.queue.push(msg);
      }
    }
  }

  send(id: number, method: string, params: unknown): void {
    writeMsg(this.child.stdin, { jsonrpc: "2.0", id, method, params });
  }

  notify(method: string, params: unknown): void {
    writeMsg(this.child.stdin, { jsonrpc: "2.0", method, params });
  }

  async waitForId(id: number, timeoutMs = 30_000): Promise<LspMsg> {
    const deadline = Date.now() + timeoutMs;
    while (Date.now() < deadline) {
      const queued = this.queue.findIndex((m) => m.id === id);
      if (queued >= 0) {
        return this.queue.splice(queued, 1)[0]!;
      }
      let settle: ((msg: LspMsg) => void) | undefined;
      const msg = await Promise.race([
        new Promise<LspMsg>((resolve) => {
          settle = resolve;
          this.waiters.push(resolve);
        }),
        new Promise<null>((resolve) => setTimeout(() => resolve(null), 100)),
      ]);
      // If the timeout won, remove the still-pending waiter so it cannot
      // swallow the next message (orphaned waiter leak under load).
      if (settle) {
        const idx = this.waiters.indexOf(settle);
        if (idx >= 0) {
          this.waiters.splice(idx, 1);
        }
      }
      if (msg && msg.id === id) {
        return msg;
      }
      if (msg) {
        this.queue.push(msg);
      }
    }
    throw new Error(`timed out waiting for LSP response id=${id}`);
  }
}

async function withLspSession(
  workspace: string,
  run: (session: LspSession, workspaceUri: string) => Promise<void>
): Promise<void> {
  const binary = resolveLspBinaryForTests();
  assert.ok(fs.existsSync(binary), `LSP binary missing: ${binary}`);
  const child = spawn(binary, [], {
    stdio: ["pipe", "pipe", "pipe"],
    env: { ...process.env, RUST_LOG: "error" },
  }) as ChildProcessWithoutNullStreams;
  children.push(child);

  let stderrBuf = "";
  child.stderr.on("data", (chunk: Buffer) => {
    stderrBuf += chunk.toString("utf8");
    if (stderrBuf.length > 8_000) {
      stderrBuf = stderrBuf.slice(-4_000);
    }
  });

  const spawnError = await new Promise<Error | null>((resolve) => {
    child.once("error", resolve);
    child.once("spawn", () => resolve(null));
  });
  if (spawnError) {
    throw spawnError;
  }

  const session = new LspSession(child);
  const workspaceUri = `file://${path.resolve(workspace)}`;

  session.send(1, "initialize", {
    processId: process.pid,
    rootUri: workspaceUri,
    capabilities: {},
  });
  let init: LspMsg;
  try {
    init = await session.waitForId(1, 20_000);
  } catch (err) {
    const exit = child.exitCode;
    throw new Error(
      `${String(err)}; binary=${binary}; exitCode=${exit}; stderr=${stderrBuf.slice(-1500)}`
    );
  }
  assert.ok(init.result, `initialize failed: ${JSON.stringify(init)}`);
  session.notify("initialized", {});

  await run(session, workspaceUri);

  session.send(99, "shutdown", null);
  await session.waitForId(99, 5_000).catch(() => undefined);
  session.notify("exit", null);
}

describe("LSP ontology workflows", { concurrency: false }, () => {
  it("applyAxiomPatch write-back updates file and catalog label", async () => {
    const workspace = fs.mkdtempSync(path.join(os.tmpdir(), "strixonomy-patch-"));
    tempDirs.push(workspace);
    const ttlPath = path.join(workspace, "people.ttl");
    fs.copyFileSync(
      path.resolve(__dirname, "..", "..", "..", "fixtures", "example.ttl"),
      ttlPath
    );
    const documentUri = `file://${ttlPath}`;

    await withLspSession(workspace, async (session, workspaceUri) => {
      session.send(2, "strixonomy/indexWorkspace", { workspace_uri: workspaceUri });
      const indexed = await session.waitForId(2);
      assert.equal(indexed.error, undefined, JSON.stringify(indexed));

      session.send(3, "strixonomy/applyAxiomPatch", {
        document_uri: documentUri,
        preview_only: false,
        patches: [
          {
            op: "add_label",
            entity_iri: "http://example.org/people#Person",
            value: "WorkflowLabel",
          },
        ],
      });
      const patched = await session.waitForId(3);
      assert.equal(patched.error, undefined, JSON.stringify(patched));
      const result = patched.result as { applied?: boolean };
      assert.equal(result.applied, true, JSON.stringify(result));

      const disk = fs.readFileSync(ttlPath, "utf8");
      assert.match(disk, /WorkflowLabel/);

      session.send(4, "strixonomy/indexWorkspace", { workspace_uri: workspaceUri });
      await session.waitForId(4);

      session.send(5, "strixonomy/getEntity", {
        iri: "http://example.org/people#Person",
      });
      const entity = await session.waitForId(5);
      assert.equal(entity.error, undefined, JSON.stringify(entity));
      assert.match(JSON.stringify(entity.result), /WorkflowLabel/);
    });
  });

  it("runReasoner then getExplanation agree on unsat Invalid class", async () => {
    const workspace = fs.mkdtempSync(path.join(os.tmpdir(), "strixonomy-reason-"));
    tempDirs.push(workspace);
    fs.copyFileSync(
      path.resolve(__dirname, "..", "..", "..", "fixtures", "reasoner-unsat.ttl"),
      path.join(workspace, "reasoner-unsat.ttl")
    );

    await withLspSession(workspace, async (session, workspaceUri) => {
      session.send(2, "strixonomy/indexWorkspace", { workspace_uri: workspaceUri });
      await session.waitForId(2);

      session.send(3, "strixonomy/runReasoner", { profile: "el", auto_detect: false });
      const classified = await session.waitForId(3, 60_000);
      assert.equal(classified.error, undefined, JSON.stringify(classified));
      const result = classified.result as {
        consistent: boolean;
        unsatisfiable: string[];
        profile_used: string;
      };
      assert.equal(result.profile_used, "el");
      assert.equal(result.consistent, false);
      assert.ok(
        result.unsatisfiable.some((iri) => iri.endsWith("#Invalid")),
        `expected Invalid unsatisfiable: ${JSON.stringify(result.unsatisfiable)}`
      );

      session.send(4, "strixonomy/getExplanation", {
        class_iri: "http://example.org/reasoner-unsat#Invalid",
        profile: "el",
      });
      const explained = await session.waitForId(4, 60_000);
      if (explained.error) {
        assert.match(JSON.stringify(explained.error), /Invalid|explanation|unsatisfiable/i);
      } else {
        const exp = explained.result as { class_iri?: string; text?: string; steps?: unknown[] };
        assert.equal(exp.class_iri, "http://example.org/reasoner-unsat#Invalid");
        assert.ok((exp.text ?? "").length > 0 || (exp.steps?.length ?? 0) > 0);
      }
    });
  });
});
