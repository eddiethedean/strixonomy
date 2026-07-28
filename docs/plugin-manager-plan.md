# Plugin Manager and Registry Specification

> **Document type:** implementation plan for the v0.33–v0.35 plugin-management
> stream. This is not a shipped capability list.
>
> **Shipped contract:** [Plugin authoring](guides/plugins.md) and
> [Plugin SDK 1.0 policy](guides/plugin-policy.md). The manager must install
> and operate that frozen TOML + subprocess JSON contract without inventing a
> second plugin format.

## 1. Product outcome

Users can discover, verify, install, pin, update, disable, roll back, and remove
Strixonomy plugins from VS Code without manually copying binaries or editing
manifests. The same resolved plugin set runs locally and in CI.

The work is staged:

| Release | Outcome |
|---------|---------|
| **v0.33** | Dependable manager for explicit registries and organization catalogs |
| **v0.34** | Official curated registry with signed, cross-platform artifacts |
| **v0.35+** | Public marketplace, publisher workflows, reporting, and moderation |

The manager ships before the public marketplace. Official integrations can be
distributed from an explicit registry while marketplace governance matures.

## 2. Installation scopes

| Scope | Purpose | Default location |
|-------|---------|------------------|
| Workspace | Reproducible project plugin set | `.strixonomy/plugins/` + committed lockfile |
| User | Personal tools used across workspaces | Strixonomy user data directory |
| Organization | Administrator-approved catalog and policy | Managed configuration |
| Bundled | Plugins released with Strixonomy | Extension/LSP distribution |

Workspace configuration declares intent; the lockfile records the exact
resolution. User and organization installations must never silently modify a
repository.

```text
.strixonomy/
├── plugins.toml
├── plugins.lock
└── plugins/
    └── org.strixonomy.rudof/
        ├── manifest.toml
        └── bin/
            └── strixonomy-plugin-rudof
```

Read-only workspaces may use user- or organization-scoped plugins but must show
that their plugin set is not repository-pinned.

## 3. Workspace declaration and lockfile

`plugins.toml` is human-authored:

```toml
registry = ["https://plugins.strixonomy.dev/index.json"]

[[plugin]]
id = "org.strixonomy.rudof"
version = "^0.1"
enabled = true
```

`plugins.lock` is generated and should be committed:

```toml
lock_version = 1

[[plugin]]
id = "org.strixonomy.rudof"
version = "0.1.3"
registry = "https://plugins.strixonomy.dev/index.json"
api_version = "1"
artifact = "aarch64-apple-darwin"
sha256 = "..."
publisher = "org.strixonomy"
permissions = ["workspace.read", "external_process"]
dependencies = []
```

The lockfile records exact versions, registry origin, artifact target, digest,
publisher identity, API version, granted permissions, and resolved
dependencies. Timestamps must not affect reproducibility.

CI defaults to locked resolution and fails when the declaration and lockfile
diverge.

## 4. Registry protocol

Registry metadata is versioned, cacheable JSON:

```json
{
  "registry_version": 1,
  "generated_at": "2026-07-28T00:00:00Z",
  "plugins": [{
    "id": "org.strixonomy.rudof",
    "name": "rudof Shapes",
    "publisher": "org.strixonomy",
    "repository": "https://github.com/rudof-project/rudof",
    "license": "MIT OR Apache-2.0",
    "versions": [{
      "version": "0.1.3",
      "api_version": "1",
      "strixonomy": ">=0.33,<0.36",
      "permissions": ["workspace.read", "external_process"],
      "dependencies": [],
      "artifacts": {
        "aarch64-apple-darwin": {
          "url": "https://plugins.strixonomy.dev/artifacts/...",
          "sha256": "...",
          "signature": "..."
        }
      }
    }]
  }]
}
```

The protocol must define pagination, conditional requests, cache expiry,
mirrors, offline behavior, metadata revocation, and unsupported-platform
responses. Clients reject unknown registry major versions.

## 5. Compatibility and dependency resolution

Resolution considers:

- Plugin semantic version requirements
- Strixonomy host range
- Plugin SDK `api_version`
- Operating system and architecture
- Required capabilities
- Plugin dependencies and optional dependencies
- Organization allow/deny policy
- Revoked versions

The resolver must produce deterministic results from identical inputs.
Conflicting constraints, cycles, unavailable platform artifacts, and missing
dependencies are user-visible errors—never silent fallbacks.

External tool requirements such as `rudof`, `typst`, or `owlmake` must declare
whether the plugin bundles the tool, downloads a pinned artifact, or requires a
system executable. The details page and lockfile record the selected mode and
detected version.

## 6. Artifact verification and publisher trust

The registry implementation must adopt one documented signing system before
remote installation ships. The selected design must cover:

- Trusted roots
- Publisher enrollment and namespace ownership
- Artifact and metadata signatures
- Digest verification
- Key rotation and expiry
- Compromised-key and release revocation
- Offline verification
- Transparency/audit records where supported

The manager verifies metadata, signature, digest, platform, and compatibility
before extracting an artifact. Failed verification cannot be bypassed by a
normal Install button.

“Verified publisher” means identity and namespace control were checked; it does
not mean Strixonomy endorses plugin behavior.

## 7. Permissions and execution tiers

The permission review displays requested capabilities, why they are needed,
and what changed since the installed version.

| Tier | Guarantee |
|------|-----------|
| Built-in | Released and reviewed with Strixonomy |
| WASI sandbox | Enforced filesystem/network/resource capabilities |
| Native subprocess | Arbitrary native code after workspace/user trust |

The current native subprocess permission list is disclosure and policy input;
it is not an OS sandbox. The UI must say so plainly. A future Wasmtime/WASI
tier may enforce capabilities, but it does not retroactively sandbox native
plugins.

Permission expansion blocks automatic update until the user or organization
policy explicitly approves it.

## 8. Transactional install, update, and rollback

Install/update is an atomic transaction:

1. Resolve the requested plugin set.
2. Show compatibility, dependencies, permissions, and publisher.
3. Download into a temporary location.
4. Verify metadata, signature, and digest.
5. Extract with path and size limits.
6. Run manifest validation and a bounded self-test.
7. Write the new lockfile candidate.
8. Atomically activate the new version.
9. Retain the previous working version for rollback.
10. Restore the previous version automatically if activation fails.

Removal disables the plugin first, confirms dependent impact, removes managed
artifacts, and preserves user data unless the user separately chooses to
delete it.

## 9. Failure isolation and quarantine

The host records crashes, timeouts, invalid JSON, excessive output, activation
failure, and resource-limit violations.

Repeated failure places a plugin in quarantine and offers:

- Open logs
- Disable
- Roll back
- Retry once
- Report plugin
- Remove

The manager must not enter an activation crash loop. Registry revocation can
disable a vulnerable version according to organization policy, but must leave
an audit record and recovery guidance.

## 10. VS Code experience

**Strixonomy: Manage Plugins** opens these views:

- Discover
- Installed
- Updates
- Workspace recommendations
- Organization approved
- Disabled and quarantined

The plugin details page shows:

- Name, publisher, verification, repository, license
- Installed and available versions
- Strixonomy/API/platform compatibility
- Permissions and permission changes
- Commands, views, providers, and other contributions
- Dependencies and external tool requirements
- Changelog and security notices
- Installation scope and update policy
- Disable, update, rollback, remove, and report actions

Install flow:

1. Choose installation scope.
2. Review publisher, artifact, compatibility, permissions, and dependencies.
3. Confirm.
4. Observe verified download and activation progress.
5. See the contributed commands/views immediately.

Workspace recommendations never auto-install. Organization policy may
preapprove or require a plugin, but the UI must show that policy source.

## 11. Marketplace governance

Public submission does not block the v0.33 manager or v0.34 official registry.
Before public publishing opens, define:

- Publisher identity and plugin-ID ownership
- Namespace disputes and abandoned-plugin transfer
- Accepted licenses and required source disclosure
- Automated malware, secret, and dependency scanning
- Security disclosure and response process
- Removal, quarantine, appeal, and reinstatement policy
- Rating/review abuse controls
- Maintainer and moderation ownership

Official, verified, community, deprecated, and quarantined states are distinct.

## 12. CLI and CI parity

The VS Code manager wraps commands that are also available noninteractively:

```text
strixonomy plugins search <query>
strixonomy plugins install <id> [--version <range>] [--scope workspace|user]
strixonomy plugins update [<id>]
strixonomy plugins lock --check
strixonomy plugins rollback <id>
strixonomy plugins remove <id>
strixonomy plugins verify
```

CI supports `--locked`, offline cache use, organization policy, and
machine-readable verification output.

## 13. Performance and accessibility

- Manager startup does not fetch remote metadata synchronously.
- Registry refresh and downloads are cancellable and resumable.
- Plugin activation remains lazy unless declared otherwise.
- Per-plugin startup, CPU, memory, and output budgets are visible.
- All flows are keyboard and screen-reader accessible.
- Security, permission, and failure states are announced without relying on
  color alone.

## 14. Release gates

### v0.33 manager

- Explicit registry URL and organization catalog support
- Workspace/user scopes
- Deterministic resolver and committed lockfile
- Permission review
- Transactional install/update/disable/remove
- Automatic rollback and quarantine
- CLI/CI parity

### v0.34 official registry

- Selected signing and trust-root design
- Signed metadata and cross-platform artifacts
- Verified publisher and namespace process
- Revocation and offline verification
- Official rudof, Regorus, Typst, mdBook, and EBISPOT/owlmake entries as ready

### v0.35+ marketplace

- Publisher submission and review
- Public discovery, ratings/reviews, and reporting
- Moderation, security response, transfer, and appeal procedures
- Permission-change and security-notice update UX

## 15. Explicit non-goals

- Installing code without user, workspace, or organization authority
- Treating declared native-process permissions as enforced sandboxing
- Silent automatic permission expansion
- Mutable unpinned CI plugin resolution
- Reimplementing external projects inside Strixonomy
- Blocking official plugin distribution on public marketplace completion
