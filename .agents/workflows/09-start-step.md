---
description: Start a new milestone step by creating the branch, planning documents, and initiating the first task.
---

# Workflow: Start Step

This workflow controls the overarching transition from one completed Step (e.g., "Step 2") to the next sequential Step (e.g., "Step 3") outlined in the project's roadmap.

## 1. Environment & Branch Isolation
Before generating any new documents or writing code, isolate the new phase of work:
- Checkout the main branch: `git checkout main`.
- Merge the previous step's branch into main (if not already merged).
- Create and switch to the new feature branch: `git checkout -b feat/step-<number>-<slug>`.

## 2. Generate Detailed Task Documents (If Missing)
Review the core goal of the new Step from the parent roadmap (`docs/roadmap/0001-milestone-01-mvp-roadmap.md`).
- If granular task breakdowns **do NOT exist** yet:
  - Create a new directory: `docs/roadmap/tasks-step-<number>-<slug>/`.
  - Generate separated markdown files for each task (e.g., `01-feature-a.md`, `02-feature-b.md`).
  - Ensure every generated task file strictly specifies TDD requirements, didactic context (Rust vs Go), and includes the `⚠️ CRITICAL RULE` forcing isolated conventional commits.
  
## 3. Synchronize State Tracking
- Update the main roadmap document to reflect the new state (e.g., mark the new Step as `In Progress` in the header).
- Update the active AI task artifact (`task.md`) with the `[ ]` checklist mapping the newly created tasks.
- If planning documents were newly created via Step 2, run a strict commit on the branch mapping the planning Phase: `git add docs && git commit -m "docs(roadmap): create detailed tasks for step X"`.

## 4. Auto-Initiate Task 1
Once the branch is verified and the documents are stable:
- Summarize the preparation to the user.
- **Immediately begin the execution of the first task** (e.g., "Task X.1"), writing code, writing tests, and running verifications without halting.
- Halt and wait for user review ONLY after the first task passes tests successfully, awaiting the `/08-approve-task` workflow.
