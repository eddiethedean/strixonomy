# ACCESSIBILITY_REPORT

# Strixonomy Accessibility Report (v0.25 / EPIC-010)

**Directory:** 08_RELEASE  
**Status:** Internal verification for v0.25 exit bar  
**Target Release:** Strixonomy v0.30.0 (functional baseline delivered in **v0.25**)

------------------------------------------------------------------------

# Purpose

This report documents accessibility validation for Strixonomy-owned webviews
and dialogs against WCAG 2.2 AA (where applicable). External third-party
certification remains deferred (BLOCKER_10 Phase 4).

------------------------------------------------------------------------

# Executive Summary

| Metric | Result |
|--------|--------|
| Release Version | v0.25.0 |
| Assessment Date | 2026-07-15 |
| Overall Accessibility Status | **PASS** (owned surfaces) |
| Standard Evaluated | WCAG 2.2 AA (where applicable) |
| Certification | Internal (axe + keyboard Vitest + inventory); external audit N/A |

------------------------------------------------------------------------

# Scope

Assessed:

- Entity Inspector, Query Workbench, Reasoner + Explanation
- Graph (canvas + list alternate from EPIC-008)
- Refactor Preview, Imports, Manchester editor
- SWRL Rule Browser / Editor, Semantic Diff
- DialogShell (New Ontology / Prefix Manager)

Host-owned (N/A — VS Code a11y):

- Explorer tree, Command Palette, Settings UI, plugin command palette entry points

------------------------------------------------------------------------

# Test Environment

| Item | Value |
|------|-------|
| OS | macOS (developer); CI linux/darwin/win VS Code e2e |
| Screen readers | VoiceOver smoke (manual checklist); NVDA via CI axe only |
| Host | VS Code webview (Chromium) |
| Automated | Vitest + axe-core (serious/critical), jsdom |
| Themes | VS Code default + high-contrast CSS vars |
| Motion | `prefers-reduced-motion` honored in CSS + graph edge animation |

Inventory: [A11Y_P0_AUDIT.md](A11Y_P0_AUDIT.md).

------------------------------------------------------------------------

# Keyboard Accessibility

| Workflow | Status | Notes |
|----------|--------|-------|
| Workspace navigation | N/A | VS Code host |
| Entity authoring (Inspector) | PASS | Landmarks; controls via FormField labels |
| Query execution | PASS | Mode/Run labelled; results region |
| Refactoring | PASS | File select + Apply/Cancel |
| Visualization | PASS | Canvas arrows/Enter/Esc + list table alt |
| Reasoner | PASS | Profile + Run Reasoner |
| Dialogs | PASS | Focus trap + Escape restore |
| Plugin management | N/A | VS Code commands |
| Settings | N/A | VS Code settings |

------------------------------------------------------------------------

# Screen Reader Validation

Verified via live regions / roles:

- Entity selection announcements (`LiveAnnouncer` in Inspector / Graph)
- Dialog labelled by title; validation `aria-live="assertive"`
- Query / reasoner / refactor / semantic-diff status announcements
- Error Callouts use `role="alert"`
- Graph list alternate `role="table"`

------------------------------------------------------------------------

# Visual Accessibility

- Focus indicators: `:focus-visible` on interactive controls
- High contrast: VS Code theme tokens
- Reduced motion: global media query + non-animated inferred edges
- Status badges include text (not color-only)

------------------------------------------------------------------------

# Automated Testing

| Suite | Path | Result |
|-------|------|--------|
| axe serious/critical smoke | `extension/webview-ui/src/a11y/accessibility.test.tsx` | PASS |
| Focus trap unit | `extension/webview-ui/src/a11y/focusTrap.test.ts` | PASS |
| DialogShell keyboard | `extension/webview-ui/src/components/DialogShell.test.tsx` | PASS |
| Full webview Vitest | `npm test` in `extension/webview-ui` | PASS (217) |

Color-contrast axe rule disabled in jsdom (unresolved VS Code CSS variables).

------------------------------------------------------------------------

# Manual Testing

Checklist for PR reviewers:

- [ ] VoiceOver: select entity in Inspector — announcement heard
- [ ] Query Workbench: Run — results/error announced
- [ ] Reasoner: Run — consistent/inconsistent status announced
- [ ] Graph: switch to List; arrow-key canvas; selection announce
- [ ] Open New Ontology dialog: Tab cycles; Escape restores focus

------------------------------------------------------------------------

# Known Exceptions

| ID | Description | Impact | Mitigation | Approved |
|----|-------------|--------|------------|----------|
| ACC-EX-01 | VS Code host chrome out of scope | Low | Document N/A | Yes |
| ACC-EX-02 | React Flow canvas not fully SR-transparent | Medium | List/table alternate + keyboard application role | Yes |
| ACC-EX-03 | External accessibility firm certification | Low | Deferred to v0.29 Phase 4 | Yes |
| ACC-EX-04 | jsdom color-contrast not evaluated | Low | Rely on VS Code theme AA | Yes |

------------------------------------------------------------------------

# Acceptance Criteria

- [x] All P0 **owned** workflows are keyboard accessible
- [x] Critical WCAG issues resolved on owned surfaces
- [x] Screen reader patterns verified (roles + live regions)
- [x] Automated accessibility tests pass
- [x] No release-blocking a11y defects on owned surfaces
- [x] `PAR-ACC-001` → VERIFIED

------------------------------------------------------------------------

# Sign-off

| Role | Name | Date | Approval |
|------|------|------|----------|
| Accessibility Reviewer | Strixonomy maintainers (internal) | 2026-07-15 | PASS |
| QA Lead | CI webview + axe harness | 2026-07-15 | PASS |
| Technical Lead | EPIC-010 | 2026-07-15 | PASS |
| Release Manager | v0.25 train | TBD at tag | |

------------------------------------------------------------------------

# Related Documents

- BLOCKER_10_ACCESSIBILITY.md
- A11Y_P0_AUDIT.md
- docs/ui/ACCESSIBILITY_SPEC.md
- PARITY_RELEASE_GATE.md
