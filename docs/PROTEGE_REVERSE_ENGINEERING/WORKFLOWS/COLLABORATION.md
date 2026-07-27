# COLLABORATION.md

# Ontology Collaboration Workflow
## Reverse Engineering Specification for Protégé and Design Blueprint for Strixonomy

## Purpose

Protégé Desktop is primarily a single-user application, while WebProtégé introduces collaborative ontology editing through shared projects, comments, change history, and permissions. Strixonomy should combine the strengths of both while adopting modern collaborative development practices inspired by Git, IDEs, and cloud-native tools.

---

# Goals

A collaboration workflow should:

- Enable simultaneous ontology editing
- Preserve semantic integrity
- Provide review and approval workflows
- Track every meaningful change
- Integrate with Git and version control
- Scale from individuals to enterprise teams

---

# High-Level Workflow

```text
Open Shared Workspace
        │
        ▼
Edit Ontology
        │
        ▼
Live Synchronization
        │
        ▼
Validation & Reasoning
        │
        ▼
Review Changes
        │
        ▼
Approve / Merge
        │
        ▼
Publish Updated Ontology
```

---

# Collaboration Models

## Local

Single-user workspace with optional Git integration.

## Shared Workspace

Multiple authenticated users editing the same ontology.

## Git-Centric

Users collaborate through branches, commits, pull requests, and reviews.

## Hybrid

Live editing backed by Git snapshots for durable history.

---

# Presence

The workspace should display:

- Active collaborators
- Current selections
- Cursor locations (optional)
- Active editors
- Current branch
- Synchronization status

---

# Live Editing

Capabilities:

- Optimistic updates
- Conflict detection
- Incremental synchronization
- Background reasoning refresh
- Entity locking only when required

---

# Comments

Users should be able to attach comments to:

- Classes
- Properties
- Individuals
- Axioms
- Imports
- Graph nodes

Each comment should support:

- Mentions
- Replies
- Resolution
- Attachments
- Links to ontology entities

---

# Reviews

Recommended workflow:

1. Create review request.
2. Review ontology changes.
3. Run validation.
4. Run reasoner.
5. Approve or request changes.
6. Merge.

Review summaries should include semantic impact.

---

# Change History

Every change should record:

- Timestamp
- User
- Command
- Entity
- Before/after values
- Optional commit reference

History should support filtering and replay.

---

# Conflict Resolution

Typical conflicts:

- Simultaneous rename
- Competing hierarchy edits
- Annotation conflicts
- Import changes

Provide:

- Visual diff
- Semantic diff
- Guided merge
- Re-run reasoning after merge

---

# Permissions

Suggested roles:

- Viewer
- Commenter
- Editor
- Reviewer
- Maintainer
- Administrator

Permissions should be configurable per workspace.

---

# Notifications

Notify users about:

- Mentions
- Review requests
- Merge conflicts
- Failed reasoning
- Validation failures
- Published ontology versions

---

# Git Integration

Recommended workflow:

1. Create feature branch.
2. Edit ontology.
3. Commit semantic changes.
4. Push branch.
5. Open pull request.
6. Review.
7. Merge.
8. Publish release.

Commit messages should reference ontology operations when possible.

---

# AI-Assisted Collaboration

AI may assist by:

- Summarizing changes
- Explaining semantic impact
- Drafting review comments
- Detecting risky merges
- Suggesting reviewers
- Generating release notes

AI suggestions should always remain reviewable.

---

# Events

Representative events:

- UserJoinedWorkspace
- UserLeftWorkspace
- EntityEdited
- CommentCreated
- ReviewRequested
- ReviewApproved
- MergeCompleted
- CollaborationSynced

---

# Accessibility

Requirements:

- Keyboard-first collaboration
- Accessible review tools
- Screen-reader compatible comments
- High-contrast presence indicators

---

# Strixonomy Modernization

Recommended enhancements:

- CRDT/OT-backed collaboration
- Live graph collaboration
- Shared command history
- Workspace timelines
- Team dashboards
- Integrated chat
- Voice/video extension points
- Offline editing with later synchronization

---

# Feature Parity Checklist

Collaboration

- [ ] Shared workspaces
- [ ] Presence
- [ ] Live editing
- [ ] Comments
- [ ] Reviews

History

- [ ] Change history
- [ ] Semantic diff
- [ ] Undo/redo
- [ ] Audit trail

Integration

- [ ] Git
- [ ] Reasoner updates
- [ ] Validation
- [ ] Notifications

Platform

- [ ] Permissions
- [ ] Accessibility
- [ ] Plugin support
- [ ] AI assistance

---

# Beyond Protégé

Strixonomy should move beyond document collaboration toward collaborative ontology engineering. Every change should be modeled as a semantic operation that integrates with reasoning, validation, visualization, automation, AI, and version control while remaining understandable to both ontology experts and software engineers.

---

# Summary

WebProtégé demonstrated the value of collaborative ontology editing, but modern ontology engineering demands deeper integration with Git, real-time synchronization, structured reviews, semantic change tracking, AI-assisted workflows, and enterprise governance. Strixonomy should provide a unified collaboration platform that feels as natural for ontology engineers as modern collaborative IDEs do for software developers.
