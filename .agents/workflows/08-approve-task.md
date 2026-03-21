---
description: Approve a completed task, commit changes, and update tracking documents
---

# Workflow: Approve Task

This workflow must be executed whenever a granular task (e.g., "Task 2.1: Implement ChronosError") is successfully implemented, verified with tests, and explicitly approved by the user.

## 1. Commit Code Changes
Before updating any documentation, ensure that the functional code changes for the task are safely persisted.
- Stage all files: `git add .`
- Determine the appropriate Conventional Commit message as defined at the bottom of the detailed task document.
- Execute the commit: `git commit -m "<conventional-commit-message>"`
- **Strict Adherence:** This guarantees compliance with the rule that every single task must have its own isolated commit before the next task begins.

## 2. Update the Detailed Task Document
Open the specific markdown file for the task just completed (e.g., `docs/roadmap/tasks-step-<number>-<slug>/01-error-enum.md`).
- Update the **Implementation Steps (Checklist)** section.
- Mark all actionable checkboxes as completed (change `- [ ]` to `- [x]`).

## 3. Update the Milestone Roadmap
Open the main milestone roadmap document (e.g., `docs/roadmap/0001-milestone-01-mvp-roadmap.md`).
- Locate the exact sub-task under the current Step's `Tasks:` section.
- Check off the task by changing `- [ ]` to `- [x]`.

## 4. Hand-off & Proceed
- Respond to the user confirming that the commit was successful and that all tracking documentation (both the granular checklist and the macroscopic roadmap) is up-to-date.
- Declare readiness to begin the **next sequential task** and wait for the user's permission to execute it.
