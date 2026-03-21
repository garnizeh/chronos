---
description: Standard for creating detailed task documentation files that complement the high-level roadmap.
globs: docs/roadmap/**/*
---
# Detailed Task Documentation Standard

The high-level roadmap (`0001-milestone-01-mvp-roadmap.md`) describes **what** needs to be done.
Detailed task files describe **how**, **why**, and **what to watch out for**.

## Directory Structure

For each step, create a folder inside `docs/roadmap/`:

```
docs/roadmap/
├── 0001-milestone-01-mvp-roadmap.md       ← high-level roadmap (source of truth)
├── tasks-step-1-workspace-skeleton/        ← detailed tasks for Step 1
│   ├── 01-workspace-root.md
│   ├── 02-crate-stubs.md
│   └── 03-verification.md
├── tasks-step-2-core-domain-models/        ← detailed tasks for Step 2
│   ├── 01-error-types.md
│   └── ...
```

**Naming conventions:**
- Folder: `tasks-step-<N>-<slug>` (slug matches the step title in kebab-case)
- Files: `<NN>-<slug>.md` — numbered in execution order, grouped logically

## File Template

Every detailed task file MUST follow this structure:

```markdown
# <Title>

> **Step:** N | **Tasks:** N.X – N.Y | **Crate(s):** affected crate names

## Why This Matters

Brief explanation of why this group of tasks exists and how it fits
into the larger architecture. Link to design doc sections when relevant.

## Tasks

### Task N.X — <Short title>

**What:** Concrete description of what to create/change.

**Details:**
- Specific files to create or modify
- Exact content when applicable (code blocks, config snippets)
- Default values and why they were chosen

**Gotchas:**
- Common mistakes or edge cases to watch for

### Task N.Y — ...

## Pre-Research

Before starting, ensure you understand:
- [ ] Concept A — link or brief explanation
- [ ] Concept B — ...

## Commit Guidance

Suggested commit(s) following `08-conventional-commits.md` and `09-step-workflow.md`.

## Acceptance Criteria

How to verify these specific tasks are correctly done.
```

## Rules for Splitting Files

1. **One file = one logical unit of work** that can be reviewed independently
2. Avoid files longer than ~200 lines — split further if needed
3. Avoid files shorter than ~30 lines — merge with a related group
4. Tasks that touch the same file or crate usually belong together
5. Verification/validation tasks can be grouped into a single file

## When NOT to Create Detailed Task Files

- **Trivial steps** where the roadmap description is already unambiguous (e.g., Step 0)
- **Single-task steps** where the roadmap entry is detailed enough
