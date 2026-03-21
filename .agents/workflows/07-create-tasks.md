---
description: Create detailed task breakdown documents for a specific roadmap step
---

# Workflow: Create Detailed Tasks for a Milestone Step

This workflow guides the process of breaking down a high-level step from the milestone roadmap into granular, actionable, and test-driven task documents.

## 1. Context Gathering
1. Read the main milestone roadmap (e.g., `docs/roadmap/0001-milestone-01-mvp-roadmap.md`).
2. Locate the specific target **Step** you were asked to detail (e.g., "Step 2: Core Domain Models").
3. Understand the Step's goal, dependencies, target crates, and existing high-level bullet points.
4. Keep the target audience (a pragmatic Senior Go Engineer) and the project's strict architectural constraints in mind.

## 2. Branching Strategy
Before generating any documents or writing code, ALWAYS create a new git branch for the milestone step to isolate the work.
- Use the naming convention: `feat/step-<number>-<slug>` (e.g., `feat/step-2-core-domain-models`).
- Run `git checkout -b <branch-name>` via your terminal integration.

## 3. Directory setup
1. Determine the path for the new tasks directory: it should sit alongside the roadmap document.
2. The folder name must match the convention: `tasks-step-<number>-<slug>/` (e.g., `tasks-step-2-core-domain-models/`).
3. If the main roadmap document does not already point to this directory, update it to add: `> 📋 **Detailed tasks:** [tasks-step-<number>-<slug>/](...)` just below the Step's title.

## 4. Granular Task Decomposition
Break down the high-level tasks of the Step into a series of smaller, sequential operations. 
Because of the **Iterative & Verifiable Execution** rule, do not group massive features together.
Create a separate Markdown document for each sub-component or logical unit inside the new directory (e.g., `01-error-enum.md`, `02-frame-struct.md`, etc.).

## 5. Document Structure
Each generated detailed task document MUST contain the following sections:

- **Title (`# Task X.Y: <Name>`)**
- **Objective:** A very brief summary of what will be achieved.
- **Mental Map / Go Parallel:** A 1-2 sentence didactic explanation mapping the Rust concepts being implemented to Go constructs (e.g., Traits vs Interfaces, `Arc/Mutex` vs channels/goroutines, `thiserror` vs `fmt.Errorf`).
- **Implementation Steps (Checklist):**
  - Actionable `[ ]` checkboxes for the exact files to create or modify.
  - Checkboxes to write `#[cfg(test)]` cases *before* the actual logic.
  - Checkboxes to run verification commands (e.g. `cargo clippy`, `cargo test`).
- **Code Scaffolding (Optional but recommended):** Provide struct, enum, or trait definitions as starting points. Do not write full implementations; leave room for TDD execution.
- **Conventional Commit:** Suggest a commit message that follows the project rules once the task is complete (e.g., `feat(chronos-core): implement capture config model`).

## 6. Project Constraints Validation
Before finalizing the documents, double-check:
- **Are there tests?** Every struct/impl must be accompanied by its unit test.
- **Is it too large?** If a single `.md` file expects more than 100 lines of code changes, break it down further.
- **Are hardware boundaries mocked?** Ensure any IO/Side-effects are implemented via Traits.

## 7. Hand-off
Once the documents are created, list the generated files to the user and **PAUSE**. Wait for the user to review the detailed breakdown before beginning the execution phase of those tasks.
