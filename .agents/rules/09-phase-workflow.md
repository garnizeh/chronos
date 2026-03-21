---
description: Git workflow for executing roadmap phases — one branch per phase, one commit per task, PR after human approval.
globs: "**/*"
---
# Phase Execution Workflow

Every roadmap phase MUST follow this git workflow. No exceptions.

## 1. Start: Create the Phase Branch

Before touching any code, branch off `main`:

```bash
git checkout main && git pull origin main
git checkout -b phase-<N>/<short-description>
```

**Naming convention:** `phase-<N>/<kebab-case-summary>`

Examples:
- `phase-1/workspace-skeleton`
- `phase-2/core-domain-models`
- `phase-7/pipeline-integration`

## 2. Execute: One Commit Per Task

Each numbered task (e.g., **1.1**, **1.2**, **3.4**) gets its own atomic commit on the phase branch. Commits MUST follow `08-conventional-commits.md`.

**Commit message pattern:**

```
<type>(<scope>): <task description>

Phase <N>, Task <N.X>
```

Examples:
```bash
git commit -m "chore: create root Cargo.toml workspace

Phase 1, Task 1.1"

git commit -m "feat(chronos-core): add Frame and SemanticLog domain models

Phase 2, Task 2.2"
```

**Rules:**
- Run `cargo check` (or equivalent verification) BEFORE committing — never commit broken code
- Group trivially related sub-steps into a single commit only if they are part of the same numbered task
- Never squash multiple numbered tasks into one commit

## 3. Pause: Human Review

When all tasks in the phase are done and verified (tests pass, clippy clean):

1. Push the branch: `git push origin phase-<N>/<short-description>`
2. **STOP and wait for explicit human approval** of the implementation
3. Do NOT create the PR until the human says the phase is approved

## 4. Finish: Create the PR

After human approval, create a Pull Request targeting `main`:

- **Title:** `feat: phase <N> — <description>` (or `chore:` if no user-facing features)
- **Body:** Summary of what was built, link to the roadmap section, and list of tasks completed
- **Labels:** phase number if available
- **Do NOT merge** — let CI run and the human merge when ready

## Summary

```
main ──┬──────────────────────────────────── merge ← PR
       └── phase-N/name ── T1 ── T2 ── T3 ── push ── review ── PR
```
