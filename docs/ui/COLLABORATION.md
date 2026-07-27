# COLLABORATION.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Collaboration Specification

## Purpose

Collaboration is a first-class capability of Strixonomy. Teams should
collaborate around **semantic intent**, not line-based text differences.
Reviews, discussions, approvals, and change history should revolve
around ontology concepts and their meaning.

------------------------------------------------------------------------

# Vision

Ontology engineering should feel like modern software engineering.

Strixonomy should provide semantic equivalents of:

-   Pull Requests
-   Code Reviews
-   Issue Discussions
-   Pair Programming
-   Design Reviews
-   Architecture Reviews
-   Continuous Integration

while understanding the ontology's semantic structure.

------------------------------------------------------------------------

# Design Principles

## Semantic Over Textual

Review changes as:

-   Classes added
-   Classes removed
-   Relationships changed
-   Restrictions modified
-   Inferred knowledge changed

instead of raw RDF/Turtle line diffs.

## Context First

Every collaboration artifact is tied to semantic objects.

Comments attach to:

-   Entities
-   Relationships
-   Restrictions
-   Queries
-   Graph regions
-   Documentation

## Reviewable Workflows

Every significant change should support:

-   Preview
-   Discussion
-   Approval
-   Merge
-   Rollback

------------------------------------------------------------------------

# Collaboration Workspace

    +--------------------------------------------------------------+
    | Reviews | Discussions | Activity | Mentions | Notifications  |
    +--------------------------------------------------------------+

    | Semantic Diff             | Review Thread                  |
    |                            |                               |
    +--------------------------------------------------------------+

    | AI Review | Graph | History | Checks | Approvals            |
    +--------------------------------------------------------------+

------------------------------------------------------------------------

# Semantic Pull Requests

Display summaries such as:

-   12 Classes Added
-   4 Classes Removed
-   18 Relationships Changed
-   6 New Restrictions
-   2 Unsatisfiable Classes Introduced
-   Documentation Coverage +8%

Include semantic health before merge.

------------------------------------------------------------------------

# Semantic Diff

Show changes grouped by:

-   Entities
-   Relationships
-   Constraints
-   Documentation
-   Imports
-   Queries
-   Reasoning

Allow filtering by change type.

------------------------------------------------------------------------

# Review Threads

Threads can attach to:

-   Entity
-   Axiom
-   Graph node
-   Graph edge
-   Query
-   Documentation section

Support:

-   Resolve
-   Reopen
-   Mention users
-   AI summary

------------------------------------------------------------------------

# Live Collaboration

Future capabilities:

-   Presence indicators
-   Shared cursors
-   Live graph editing
-   Follow collaborator
-   Voice/video integration hooks

Edits synchronize through the Workspace Model.

------------------------------------------------------------------------

# Activity Feed

Track:

-   Entity edits
-   Refactorings
-   Reasoning runs
-   AI actions
-   Reviews
-   Merges
-   Comments

Filter by user, ontology, or module.

------------------------------------------------------------------------

# Notifications

Notify users about:

-   Mentions
-   Review requests
-   Failed checks
-   AI review complete
-   Merge conflicts
-   Assigned work

Notifications should be actionable.

------------------------------------------------------------------------

# Merge Experience

Before merge:

-   Run reasoning
-   Validate constraints
-   Execute configured checks
-   Detect semantic conflicts
-   Generate impact summary

Surface blocking issues clearly.

------------------------------------------------------------------------

# Semantic Conflicts

Detect conflicts beyond text.

Examples:

-   Divergent class hierarchies
-   Conflicting restrictions
-   Duplicate entities
-   Namespace collisions
-   Reasoning regressions

Offer guided resolution workflows.

------------------------------------------------------------------------

# AI Collaboration

AI assists by:

-   Summarizing changes
-   Explaining reasoning impact
-   Highlighting risky edits
-   Suggesting reviewers
-   Drafting review comments
-   Identifying missing documentation

AI augments human review.

------------------------------------------------------------------------

# Governance

Support:

-   Required reviewers
-   Approval rules
-   Protected ontologies
-   Audit history
-   Electronic sign-off
-   Policy checks

Suitable for enterprise and regulated environments.

------------------------------------------------------------------------

# Integrations

Integrate with:

-   GitHub
-   GitLab
-   Azure DevOps
-   Jira
-   Slack / Teams
-   CI pipelines

Semantic review complements existing developer workflows.

------------------------------------------------------------------------

# Plugin Extension Points

Plugins may contribute:

-   Review checks
-   Policy validators
-   Dashboards
-   Approval workflows
-   Notification providers
-   Collaboration widgets

------------------------------------------------------------------------

# Accessibility

Support:

-   Keyboard-first reviews
-   Screen readers
-   High contrast
-   Reduced motion
-   Accessible diff navigation

------------------------------------------------------------------------

# Success Criteria

Collaboration succeeds when teams discuss ontology meaning instead of
serialization details. Reviews become faster, higher quality, and easier
to understand because Strixonomy presents semantic intent, reasoning
impact, and contextual discussions as first-class concepts rather than
forcing reviewers to interpret raw RDF diffs.
