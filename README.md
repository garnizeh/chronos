# Chronos

**Your personal context engine — a local-first screen capture daemon that understands what you're doing.**

<div align="left">

[![GitHub Release](https://img.shields.io/github/v/release/garnizeh/chronos?display_name=tag&logo=github)](https://github.com/garnizeh/chronos/releases)
[![Rust Version](https://img.shields.io/badge/rust-1.94%2B-blue.svg?logo=rust)](https://www.rust-lang.org/)
[![Ollama Version](https://img.shields.io/badge/ollama-0.17%2B-black.svg?logo=ollama)](https://ollama.com/)
[![CI](https://github.com/garnizeh/chronos/actions/workflows/ci.yml/badge.svg)](https://github.com/garnizeh/chronos/actions/workflows/ci.yml)
[![CodeRabbit Pull Request Reviews](https://img.shields.io/coderabbit/prs/github/garnizeh/chronos?utm_source=oss&utm_medium=github&utm_campaign=garnizeh%2Fchronos&labelColor=171717&color=FF570A&label=CodeRabbit+Reviews)](https://coderabbit.ai)
[![codecov](https://codecov.io/gh/garnizeh/chronos/graph/badge.svg?token=UG4S5JZMDI)](https://codecov.io/gh/garnizeh/chronos)
[![Platform](https://img.shields.io/badge/platform-linux%20(X11)-lightgrey)](#prerequisites)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/garnizeh/chronos/pulls)

</div>

> 🚧 **Under active development** — Milestone 1 MVP in progress.

---

## 🧭 The Why

We've all been there: you step away from your PC for a meeting, a coffee, or the weekend, and when you return, the context is completely gone. What were you debugging? Which logs were you looking at? What was the name of that elusive command you ran an hour ago?

Chronos solves this. It acts as a **searchable photographic memory** for your workstation. By periodically capturing your screen and feeding it to a locally-hosted Vision Language Model (VLM), Chronos semanticizes your activity into a highly queryable, private SQLite database. 

You no longer need to remember *what* you were doing—you just ask Chronos.

## 🔒 Privacy-First by Design

When dealing with screen captures, trust is not optional. Chronos guarantees absolute privacy at the architectural level:

| Guarantee | How |
|---|---|
| **Zero Cloud APIs** | All AI inference runs locally via Ollama on `localhost`. Your screen never leaves your machine. |
| **Zero Disk I/O** | Captured frames live entirely in a RAM-only ring buffer. Raw images (`.png`/`.jpg`) are never written to your SSD, saving wear and tear. |
| **Zero Telemetry** | No analytics, no tracking, no outbound network calls beyond `localhost:11434`. |
| **Local SQLite** | Semantic data, vectors, and embeddings are stored purely in a local SQLite database. |

## 🛠️ The Pragmatic Stack

Built for performance, safety, and operational simplicity:

- **Rust 1.94+**: Memory-safe footprint, fearless concurrency, and zero-cost abstractions.
- **Ollama 0.17+**: Frictionless local inference for modern Vision Language Models.
- **SQLite / sqlx**: Battle-tested, serverless, and embedded data layer using direct, compile-time checked queries (no heavy ORMs).
- **Just**: Pragmatic command runner to encapsulate all developer workflows seamlessly.

## 📦 Prerequisites

- **Rust** 1.94+ (`rustup` recommended)
- **Ollama** 0.17.4+ running locally (`ollama serve`)
- A Vision Language Model pulled locally (e.g. `ollama pull moondream` or `llava`)
- **Just** command runner (`cargo install just`)
- **Linux** with X11 display server (Wayland support planned)

## 🏗️ Build & Development

We use `just` instead of bare `cargo` commands to streamline the development lifecycle.

```bash
# Install dependencies, setup local environment
just setup

# Reset and migrate the SQLite database
just db-reset

# Run the exact CI pipeline locally before committing to catch errors early
just ci-local

# Build the release binary
just build-release
```

## 🚀 Quick Start

Getting started is as simple as running the daemon:

```bash
# Start the capture daemon in the foreground
just run chronos-daemon start

# Query your activity logs from a specific date
just run chronos-daemon query --from 2025-01-01

# Check the system and daemon status
just run chronos-daemon status
```

## 📉 Project Status

This is a **v0.1 MVP/POC** — currently supporting Linux X11, single-monitor setups, and basic VLM interactions. 

See the [design document](docs/design/0001-chronos-personal-context-engine.md) for the full architecture, design choices, and future roadmap.

## 🤝 Contributing

Contributions are welcome! Here's how to get up and running:

```bash
# 1. Clone and enter the repo
git clone https://github.com/garnizeh/chronos.git && cd chronos

# 2. Install all dev tools (uses cargo-binstall for speed)
just setup

# 3. Setup the local SQLite database
just db-setup

# 4. Run the full CI pipeline locally before opening a PR
just ci-local
```

All commits must follow [Conventional Commits](https://www.conventionalcommits.org/). See the project's commit history for examples.

## 📄 License

[MIT](LICENSE)
