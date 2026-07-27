# Milestone template

Use this template for every implementable milestone in [PRODUCT_ROADMAP_2.0.md](../ui/PRODUCT_ROADMAP_2.0.md) and release sections in [roadmap.md](../roadmap.md).

---

## Milestone: [Title]

**Phase ID:** (e.g. Phase 0)  
**Target release:** (e.g. v0.13)  
**Status:** planned | in progress | shipped

### Goal

One sentence: what this milestone achieves for the platform.

### User-visible outcome

What a user can do after this ships (demo script in 1–3 bullets).

### Technical scope

- Bullet list of engineering work
- Explicit in/out of scope

### Files / modules likely affected

| Path | Change |
|------|--------|
| `extension/webview-ui/src/...` | … |
| `extension/src/...` | … |
| `crates/...` | … (if Strixonomy) |

### Acceptance criteria

- [ ] Measurable criterion 1
- [ ] Measurable criterion 2

### Tests required

- Unit: (e.g. Vitest store tests)
- Integration: (e.g. webview message round-trip)
- E2E: (e.g. VS Code extension test)

### Risks

| Risk | Mitigation |
|------|------------|
| … | … |

### Dependencies

- Prior milestone or crate version
- External: Ontologos, design tokens, etc.

### Links

- Spec: …
- ADR: …
- Cursor prompt: …

---

Copy this block for each phase in PRODUCT_ROADMAP_2.0.
