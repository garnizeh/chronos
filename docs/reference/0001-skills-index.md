# Skills Index for Project Chronos

> **Purpose:** Curated index of installed Antigravity skills with high relevance to the Chronos project.  
> **Skills Location:** `/home/userone/.gemini/antigravity/skills/`  
> **Usage:** Reference a skill with `@skill-name` in your prompt.  
> **Last Updated:** 2026-03-20

---

## 1. Rust & Systems Programming

Core language skills for building the Chronos engine.

| Skill | Relevance to Chronos |
|---|---|
| `rust-pro` | Master Rust 1.75+ — async patterns, type system, production systems |
| `rust-async-patterns` | Tokio, async traits, concurrent patterns — critical for the capture daemon and VLM worker |
| `systems-programming-rust-project` | Scaffolding production-ready Rust applications with proper module organization |
| `memory-safety-patterns` | RAII, ownership, smart pointers — essential for the zero-disk ring buffer |
| `bevy-ecs-expert` | ECS patterns in Rust (reference for modular component design) |

---

## 2. Architecture & Design Patterns

Guiding the overall structure and boundaries of Chronos.

| Skill | Relevance to Chronos |
|---|---|
| `architecture` | Requirements analysis, trade-off evaluation, ADR documentation |
| `architecture-patterns` | Clean Architecture, Hexagonal Architecture, DDD — trait-based boundaries |
| `architecture-decision-records` | Capturing rationale behind decisions (Rust over Go, SQLite over Postgres, etc.) |
| `clean-code` | Uncle Bob principles — naming, functions, modules |
| `uncle-bob-craft` | Code review and refactoring with clean code discipline |
| `software-architecture` | Quality-focused architecture guidance for any development task |
| `microservices-patterns` | Service boundaries and data management (future multi-component evolution) |

---

## 3. Database & Data Layer

SQLite, data modeling, and query optimization for the Chronos storage engine.

| Skill | Relevance to Chronos |
|---|---|
| `database-architect` | Data layer design from scratch, technology selection, schema modeling |
| `database-design` | Schema design, indexing strategy, serverless databases |
| `sql-pro` | Advanced SQL, OLTP optimization, query techniques |
| `sql-optimization-patterns` | Index tuning and query plan analysis for SQLite |
| `postgresql` | Reference for relational patterns (applicable to SQLite schema design) |
| `nosql-expert` | Mental models for query-first modeling (relevant for vector search design) |

---

## 4. AI, LLM & RAG

Local inference, embeddings, and retrieval-augmented generation — the intelligence core.

| Skill | Relevance to Chronos |
|---|---|
| `local-llm-expert` | Local LLM inference, Ollama, llama.cpp, VRAM optimization, quantization (GGUF) |
| `ai-engineer` | Production-ready LLM apps, RAG systems, intelligent agents |
| `rag-engineer` | Chunking, embedding dimensions, similarity metrics, retrieval quality |
| `rag-implementation` | Embedding selection, vector DB setup, chunking strategies |
| `embedding-strategies` | Selecting and optimizing embedding models for vector search |
| `similarity-search-patterns` | Semantic search, nearest neighbor queries, retrieval performance |
| `vector-database-engineer` | Vector databases and semantic search implementation |
| `vector-index-tuning` | HNSW parameters, quantization strategies, scaling vector search |
| `hybrid-search-implementation` | Combining vector and keyword search (sqlite-vec + FTS5) |
| `llm-structured-output` | Getting reliable JSON from LLMs — critical for VLM response parsing |
| `prompt-engineering` | Prompt design for consistent VLM/LLM outputs |
| `context-window-management` | Optimizing LLM context for batch synthesis |
| `computer-vision-expert` | Vision models, real-time spatial analysis (VLM frame processing) |

---

## 5. Security & Privacy

The non-negotiable pillar of Chronos.

| Skill | Relevance to Chronos |
|---|---|
| `privacy-by-design` | Data minimization, consent, encryption — built-in from day one |
| `security-auditor` | DevSecOps, comprehensive cybersecurity, compliance |
| `security-bluebook-builder` | Minimal security policy with MUST/SHOULD/CAN language |
| `007` | Security audit, hardening, threat modeling (STRIDE/PASTA) |
| `threat-modeling-expert` | STRIDE, PASTA, attack trees, security requirement extraction |
| `api-security-best-practices` | Secure API patterns for the local REST interface |
| `secrets-management` | Secrets handling for SQLCipher encryption keys |
| `zeroize-audit` | Detect missing zeroization of sensitive data in Rust code |
| `constant-time-analysis` | Prevent timing side-channels in crypto operations |

---

## 6. Testing & Quality Assurance

TDD-first development as mandated by project rules.

| Skill | Relevance to Chronos |
|---|---|
| `tdd-orchestrator` | Red-green-refactor discipline, multi-agent TDD workflow |
| `tdd-workflow` | RED-GREEN-REFACTOR cycle principles |
| `test-driven-development` | Implementing features test-first, before writing code |
| `testing-patterns` | Factory functions, mocking strategies, TDD workflow |
| `unit-testing-test-generate` | Comprehensive unit tests with edge case coverage |
| `e2e-testing-patterns` | Reliable end-to-end test suites |
| `test-fixing` | Systematically identify and fix all failing tests |
| `systematic-debugging` | Bug diagnosis before proposing fixes |

---

## 7. CLI, TUI & User Interface

Building Chronos interfaces from terminal to desktop.

| Skill | Relevance to Chronos |
|---|---|
| `ai-native-cli` | 98 rules for CLI tools that AI agents can use safely (JSON output, error handling) |
| `bash-pro` | Defensive scripting for automation and CI/CD |
| `bash-defensive-patterns` | Production-grade shell scripts for install/setup |

---

## 8. DevOps, CI/CD & Infrastructure

Build, test, and distribute Chronos.

| Skill | Relevance to Chronos |
|---|---|
| `docker-expert` | Container optimization, multi-stage builds (future packaging) |
| `github-actions-templates` | CI/CD workflows for `cargo test`, `cargo clippy`, releases |
| `git-advanced-workflows` | Clean git history, collaboration patterns |
| `git-hooks-automation` | Pre-commit hooks for formatting and linting |
| `changelog-automation` | Automated changelog from commits |
| `deployment-procedures` | Safe deployment workflows, rollback strategies |

---

## 9. Documentation & Knowledge

Keeping the project well-documented and accessible.

| Skill | Relevance to Chronos |
|---|---|
| `docs-architect` | Technical documentation from codebases |
| `readme` | Comprehensive README generation |
| `mermaid-expert` | Diagrams for architecture, flows, ERDs |
| `api-documentation` | OpenAPI specs, developer guides |
| `tutorial-engineer` | Step-by-step tutorials from code |
| `wiki-architect` | Structured wiki and onboarding guides |

---

## 10. Code Quality & Review

Maintaining production-grade code standards.

| Skill | Relevance to Chronos |
|---|---|
| `code-reviewer` | AI-powered code review |
| `code-review-checklist` | Functionality, security, performance, maintainability |
| `code-simplifier` | Refine code for clarity and consistency |
| `find-bugs` | Security vulnerabilities and code quality issues |
| `debugger` | Debugging specialist for errors and unexpected behavior |
| `performance-optimizer` | Bottleneck identification and measurement |
| `performance-profiling` | Measurement, analysis, and optimization techniques |

---

## 11. Project Management & Planning

Organizing work and tracking progress.

| Skill | Relevance to Chronos |
|---|---|
| `plan-writing` | Structured task planning with dependencies and verification |
| `concise-planning` | Clear, actionable, atomic checklists |
| `blueprint` | Turn objectives into step-by-step construction plans |
| `kaizen` | Continuous improvement and error proofing |
| `progressive-estimation` | Estimate AI-assisted development work |

---

## Quick Reference: Top 15 Most Critical Skills

These are the skills you'll reach for most frequently during Chronos development:

| # | Skill | Why |
|---|---|---|
| 1 | `rust-pro` | Core language expertise |
| 2 | `rust-async-patterns` | Tokio async for daemon and workers |
| 3 | `local-llm-expert` | Ollama integration, model selection, VRAM |
| 4 | `rag-engineer` | Chunking, embeddings, retrieval quality |
| 5 | `database-architect` | SQLite schema and `sqlite-vec` design |
| 6 | `privacy-by-design` | Privacy guarantees at every layer |
| 7 | `tdd-workflow` | Test-first development (project rule) |
| 8 | `architecture-patterns` | Trait-based boundaries, DI |
| 9 | `memory-safety-patterns` | Ring buffer, zero-disk I/O |
| 10 | `llm-structured-output` | Reliable JSON from VLM responses |
| 11 | `hybrid-search-implementation` | sqlite-vec + FTS5 combined search |
| 12 | `clean-code` | Maintainable, documented code |
| 13 | `github-actions-templates` | CI/CD for Rust workspace |
| 14 | `threat-modeling-expert` | Security architecture review |
| 15 | `debugger` | Rapid issue diagnosis |
