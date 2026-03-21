---
description: Git workflow for executing roadmap steps — one branch per step, one commit per task, PR after human approval.
globs: "**/*"
---
# Step Execution Workflow

Every roadmap step MUST follow this git workflow. No exceptions.

## 1. Start: Create the Step Branch

Before touching any code, branch off `main`:

```bash
git checkout main && git pull origin main
git checkout -b step-<N>/<short-description>
```

**Naming convention:** `step-<N>/<kebab-case-summary>`

Examples:
- `step-1/workspace-skeleton`
- `step-2/core-domain-models`
- `step-7/pipeline-integration`

## 2. Execute: One Commit Per Task

Each numbered task (e.g., **1.1**, **1.2**, **3.4**) gets its own atomic commit on the step branch. Commits MUST follow `08-conventional-commits.md`.

**Commit message pattern:**

```
<type>(<scope>): <task description>

Step <N>, Task <N.X>
```

Examples:
```bash
git commit -m "chore: create root Cargo.toml workspace

Step 1, Task 1.1"

git commit -m "feat(chronos-core): add Frame and SemanticLog domain models

Step 2, Task 2.2"
```

**Rules:**
- Run `cargo check` (or equivalent verification) BEFORE committing — never commit broken code
- Update the detailed task checklist and the master milestone roadmap, and stage those documentation changes together with the code
- Group trivially related sub-steps into a single commit only if they are part of the same numbered task
- Never squash multiple numbered tasks into one commit

## 3. Pause: Human Review

When all tasks in the step are done and verified (tests pass, clippy clean):

1. Push the branch: `git push origin step-<N>/<short-description>`
2. **STOP and wait for explicit human approval** of the implementation
3. Do NOT create the PR until the human says the step is approved

## 4. Finish: Create the PR

After human approval, create a Pull Request targeting `main`:

- **Title:** `feat: step <N> — <description>` (or `chore:` if no user-facing features)
- **Body:** Summary of what was built, link to the roadmap section, and list of tasks completed
- **Labels:** step number if available
- **Do NOT merge** — let CI run and the human merge when ready

## Summary

```
main ──┬──────────────────────────────────── merge ← PR
       └── step-N/name ── T1 ── T2 ── T3 ── push ── review ── PR
```
