# Cursor prompt: Design tokens and shared components

## Prerequisites

Read:

- [ ] `ui/DESIGN_TOKENS.md`
- [ ] `ui/COMPONENT_LIBRARY.md`
- [ ] `platform/ONTOUI.md`

## Non-goals

- Full visual design system v0.30
- Figma tokens import

## Current state

- extension/webview-ui/src/components/ui.tsx partial primitives
- ui/DESIGN_TOKENS.json exists

## Tasks

1. Create extension/webview-ui/src/tokens/cssVars.ts mapping from DESIGN_TOKENS.json keys to CSS variables
2. Import tokens in global.css
3. Extract Button, Input, Card from ui.tsx if not already separate; document in components/README.md
4. Add ui.test.tsx coverage for token class application

## Acceptance criteria

- [ ] CSS vars defined for primary/spacing tokens
- [ ] Existing panels unchanged visually or improved consistently

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not break VS Code theme contrast

## References

- [Cursor prompts index](README.md)
