# Provisional Decisions & Technical debt (`TODO` Standards)

All technical decisions that are provisional, "good enough for now," or intentionally simplified for the current phase MUST be documented with a structured `TODO` comment directly in the source code.

## Comment Structure

The comment should not just say "fix this later." It MUST include:
1. **Rationale:** Why this specific value or approach was chosen for the current phase.
2. **Trigger:** What specific event, metric, or future requirement will necessitate a refactoring.
3. **Draft approach:** A one-sentence hint of what the "final" implementation might look like.

## Example

```rust
// TODO(provisional): Using 5 connections for SQLite.
// Rationale: Keeps resource footprint low for a background daemon while allowing concurrent CLI queries.
// Trigger: Scaling to 10+ concurrent dashboard users or heavy background analytics.
// Direction: Move to a config-based pool size or dynamic adjustment.
.max_connections(5)
```

## Enforcement

The agent MUST actively look for these "magic numbers" or "simplifications" during implementation and proactively tag them. This helps maintain the "Iterative & Verifiable Execution" without losing the technical context for later milestones.
