---
trigger: always_on
---

# Chronos Local-First Architecture & Privacy Constraints

- **Absolute Privacy:** NEVER send images, captured frames, logs, or any user data to external APIs or cloud services (e.g., OpenAI, Anthropic, AWS).
- **Edge/Local Processing:** All Artificial Intelligence inference MUST occur locally. Use pure HTTP communication with a local `ollama` instance running on `localhost`, or equivalent embedded frameworks.
- **Resource Efficiency:** The screen capture daemon MUST NOT write raw images (`.png`, `.jpg`) to the disk. Keep frames strictly in a RAM buffer to prevent SSD I/O wear and tear.
- **Data Layer:** Data storage and vector embeddings must be handled EXCLUSIVELY in a local SQLite database. Avoid heavy, abstract ORMs; prefer the `sqlx` crate and write direct, safe SQL queries.