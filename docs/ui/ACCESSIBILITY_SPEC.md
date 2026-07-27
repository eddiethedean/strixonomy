# Accessibility Specification

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Standard

Strixonomy targets WCAG 2.2 AA.

## 2. Keyboard Accessibility

Every feature must be keyboard accessible.

Required:

- visible focus indicators
- logical tab order
- keyboard shortcuts
- escape routes
- no keyboard traps

## 3. Screen Readers

Expose:

- entity names
- roles
- selected state
- diagnostics
- relationship summaries
- command descriptions

## 4. Color and Contrast

- Minimum contrast follows WCAG AA.
- Error state never relies on color alone.
- High contrast theme supported.

## 5. Motion

Respect reduced motion.

Disable or simplify:

- graph animations
- panel transitions
- focus zoom animations

## 6. Graph Accessibility

Provide alternative graph navigation:

- node list
- relationship table
- keyboard traversal
- textual neighborhood summary

## 7. AI Accessibility

AI outputs must be readable, structured, and navigable.

## 8. Testing

Shipped for v0.26 (EPIC-010):

- automated axe-core checks in `extension/webview-ui/src/a11y/accessibility.test.tsx` (serious/critical)
- focus-trap unit tests + DialogShell Escape/restore
- keyboard paths on Graph list/canvas (EPIC-008) and dialogs
- evidence: [ACCESSIBILITY_REPORT.md](../protege-parity/08_RELEASE/ACCESSIBILITY_REPORT.md)

Still aspirational:

- full high-contrast visual snapshots
- third-party screen reader certification (Phase 4)
