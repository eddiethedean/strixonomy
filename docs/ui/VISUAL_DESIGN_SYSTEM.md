# VISUAL_DESIGN_SYSTEM.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Visual Design System

## Purpose

The Visual Design System defines the visual language of Strixonomy. Every
interface, component, icon, color, animation, and layout should
contribute to a cohesive, professional, and distraction-free engineering
environment.

The design language should feel closer to **Linear, JetBrains, Cursor,
Figma, and GitHub** than to traditional Eclipse-based desktop
applications.

------------------------------------------------------------------------

# Design Principles

## Calm

The interface should reduce cognitive load.

Avoid excessive borders, gradients, icons, and visual noise.

Whitespace is a feature.

------------------------------------------------------------------------

## Information First

Visual decoration must never compete with semantic information.

Entity names, relationships, diagnostics, and documentation always take
priority over ornamental styling.

------------------------------------------------------------------------

## Modern

Use contemporary UI patterns:

-   rounded corners
-   subtle elevation
-   restrained color palette
-   responsive layouts
-   smooth animations
-   typography-driven hierarchy

------------------------------------------------------------------------

## Consistency

Every component should share the same spacing, sizing, typography,
interaction states, and motion behavior.

Consistency builds user confidence.

------------------------------------------------------------------------

# Design Tokens

## Spacing Scale

Base spacing unit: **4px**

  Token     Value
  ------- -------
  xs          4px
  sm          8px
  md         16px
  lg         24px
  xl         32px
  xxl        48px

Layouts should align to this scale.

------------------------------------------------------------------------

## Border Radius

  Token      Value
  -------- -------
  Small        4px
  Medium       8px
  Large       12px
  Pill       999px

------------------------------------------------------------------------

## Elevation

Three elevation levels only:

-   Flat
-   Raised
-   Overlay

Avoid excessive shadows.

------------------------------------------------------------------------

# Typography

Use a highly legible sans-serif UI font.

Hierarchy:

-   Display
-   Title
-   Section Heading
-   Body
-   Caption
-   Code

Code views should use a monospaced font.

Entity names should always be visually prominent.

------------------------------------------------------------------------

# Color System

Use semantic colors instead of arbitrary colors.

Primary

-   Accent
-   Accent Hover
-   Accent Active

Status

-   Success
-   Warning
-   Error
-   Information

Semantic

-   Class
-   Individual
-   Property
-   Annotation
-   Diagnostic
-   AI Suggestion

Color should reinforce meaning---not replace it.

The interface must fully support light and dark themes.

------------------------------------------------------------------------

# Iconography

Icons should:

-   be simple
-   use a single visual style
-   remain recognizable at 16px

Icons supplement text.

Icons should rarely appear without labels.

------------------------------------------------------------------------

# Layout

Preferred layout:

  -------------------------------------------------------
  Command Palette / Search

  -------------------------------------------------------

| Explorer \| Workspace \| Inspector \|
|          \| \| \|
|          \| \| \|

+-------------------------------------------------------+

| Problems \| Query \| Graph \| AI \| Git \| Output \|

+-------------------------------------------------------+

Avoid floating windows whenever possible.

------------------------------------------------------------------------

# Cards

Inspector content should be organized using cards.

Example:

Patient

OWL Class

Healthy

17 References

3 Diagnostics

Relationships

History

Documentation

Cards improve scanning and reduce visual clutter.

------------------------------------------------------------------------

# Buttons

Button hierarchy:

1.  Primary
2.  Secondary
3.  Ghost
4.  Icon-only

Avoid more than one primary button in a view.

------------------------------------------------------------------------

# Forms

Forms should:

-   validate continuously
-   minimize required fields
-   use inline help
-   avoid modal dialogs

------------------------------------------------------------------------

# Tables

Tables should support:

-   sorting
-   filtering
-   resizing
-   virtualization
-   keyboard navigation

Sticky headers are preferred.

------------------------------------------------------------------------

# Graph Visualization

Graphs should emphasize clarity.

Support:

-   zoom
-   pan
-   clustering
-   filtering
-   semantic coloring
-   pinning
-   minimap

Animations should preserve user orientation.

------------------------------------------------------------------------

# Motion

Motion should communicate change.

Recommended durations:

Fast: 150 ms

Normal: 200 ms

Large transitions: 250 ms

Animations should never delay user interaction.

Respect reduced-motion preferences.

------------------------------------------------------------------------

# Empty States

Every empty state should answer:

What is this?

Why is it empty?

What can I do next?

Prefer illustrations only when they add meaning.

------------------------------------------------------------------------

# Loading States

Use skeleton placeholders instead of spinners whenever possible.

Users should understand what is loading.

------------------------------------------------------------------------

# Error States

Error views should contain:

-   plain-language explanation
-   affected entity
-   suggested actions
-   documentation links

Never display raw stack traces to end users.

------------------------------------------------------------------------

# Responsive Design

The interface should gracefully adapt to:

-   ultrawide monitors
-   laptops
-   tablets (future)
-   high-DPI displays

Panels should collapse before becoming unusable.

------------------------------------------------------------------------

# Accessibility

Support:

-   WCAG 2.2 AA
-   keyboard-only navigation
-   high contrast
-   reduced motion
-   scalable fonts
-   screen readers

Accessibility is part of the visual design, not an afterthought.

------------------------------------------------------------------------

# Plugin Consistency

Plugin authors should inherit:

-   spacing
-   typography
-   colors
-   icons
-   motion
-   component styles

Plugins should feel native to Strixonomy.

------------------------------------------------------------------------

# Success Criteria

Users should describe Strixonomy as:

-   Modern
-   Fast
-   Beautiful
-   Calm
-   Professional
-   Easy to navigate

The visual system should disappear into the background, allowing users
to focus entirely on understanding and evolving semantic knowledge.
