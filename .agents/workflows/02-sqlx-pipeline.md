---
name: Local SQLite & SQLx Pipeline
description: Workflow for adding new tables or queries using pure SQL and sqlx.
trigger: "Add database entity: [entity_name]"
---
# Workflow: SQLite & SQLx Integration

**Step 1: SQL Migration**

- Write the raw SQL statements for table creation (UP and DOWN migrations).
- Ensure strict SQLite types and appropriate indexing (e.g., for timestamps).
- Ask the user to approve the schema design.

**Step 2: Rust Data Structures**

- Create the Rust `struct` that maps to the new table.
- Implement `serde::Serialize` and `serde::Deserialize` if the struct will be used in HTTP or IPC boundaries.

**Step 3: Query Implementation**

- Implement the database repository using the `sqlx::query!` or `sqlx::query_as!` macros to ensure compile-time SQL verification.
- Write a local test that instantiates an in-memory SQLite database (`sqlite::memory:`) to verify the repository logic.

**Step 4: Verification**

- Run `cargo test` to ensure the queries are syntactically correct and the mappings match.
