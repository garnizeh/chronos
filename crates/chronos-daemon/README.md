# chronos-daemon

## Role
**The Orchestrator & CLI.**
`chronos-daemon` is the entry point of the application. It wires together the capture loops, the inference engine, and the SQLite storage layer. It also provides the CLI for starting the service and querying history.

## Architectural Constraints
- **Lifecycle Management**: Responsible for managing the Tokio runtime and coordinating the graceful shutdown of background threads via `tokio::sync::watch`.
- **Data Persistence**: Manages the local SQLite connection pool (`sqlx`) and ensures schema migrations are handled automatically.
- **CLI Authority**: Uses `clap` to provide a robust, documented CLI for both human users and shell scripts.

## Testing Strategy
- **Database Mocks**: Uses SQLite in-memory databases (`sqlite::memory`) for fast, isolated integration testing of the data layer.
- **End-to-End focus**: Since `main.rs` is primarily wiring, testing focus is shifted towards E2E integration tests that verify the full pipeline (Capture -> Inference -> Database) using mock implementations of the side-effect traits.
