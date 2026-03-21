---
name: Architectural Mentorship & Concept Deep Dive
description: Workflow triggered when the user asks to explain a concept, a compiler error, or a specific implementation choice.
trigger: "Teach me: [concept]" OR "Explain this: [code_block/error]"
---
# Workflow: Concept Deconstruction & Mentorship

**Step 1: The Pragmatic Anchor (Mental Model)**
- Start by mapping the requested Rust concept to familiar systems engineering paradigms. 
- When applicable, draw direct, practical comparisons to Go (Golang) equivalents to bridge the gap. For example: map Rust `trait` to Go `interface`, `Result<T, E>` to `(value, error)` tuples, or `tokio::mpsc` to Go channels. Focus on the structural similarities and the philosophical differences.

**Step 2: Minimal Viable Snippet (MVS)**
- Do not paste the full project code. 
- Isolate the concept into the smallest possible, fully compilable Rust snippet (under 20 lines if possible).
- Demonstrate the "Happy Path" clearly.

**Step 3: The Compiler's Perspective (The "Why")**
- Explain *why* `rustc` (the Rust compiler) forces this specific implementation. 
- If discussing ownership, lifetimes, or concurrency, explain what memory safety guarantee is being enforced at compile-time (e.g., preventing data races or use-after-free).
- Highlight if the feature is a "zero-cost abstraction" or if it introduces runtime overhead (like `Arc<Mutex<T>>`).

**Step 4: Anti-Patterns & Pitfalls**
- Show the "wrong" way to do it. 
- Highlight common beginner mistakes (e.g., fighting the borrow checker, excessive use of `.clone()` to appease the compiler, or over-engineering with unnecessary macros).
- Explain the idiomatic, pragmatic path forward.

**Step 5: Concept Check**
- Conclude by asking a brief, targeted question to ensure the core architectural constraint was understood before returning to writing production code.