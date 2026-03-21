# Chronos

**Your personal context engine — a local-first screen capture daemon that understands what you're doing.**

[![CI](https://github.com/garnizeh/chronos/actions/workflows/ci.yml/badge.svg)](https://github.com/garnizeh/chronos/actions/workflows/ci.yml)

> 🚧 **Under active development** — Phase 0.1 MVP in progress.

---

## What is Chronos?

Chronos is a background daemon that periodically captures your screen, sends the screenshot to a **locally-running** Vision Language Model (via [Ollama](https://ollama.com)), and stores a semantic description of your activity in a local SQLite database. Later, you can query your activity history in natural language.

**Think of it as a personal, private, searchable memory of everything you did on your computer.**

## 🔒 Privacy-First Architecture

| Guarantee | How |
|---|---|
| **No cloud APIs** | All AI inference runs locally via Ollama on `localhost` |
| **No images on disk** | Captured frames live in a RAM-only ring buffer — never written to SSD |
| **No telemetry** | Zero outbound network calls beyond `localhost:11434` |
| **Local storage** | All data stored in a local SQLite database |

> *100% local-first. All AI inference runs on your machine via Ollama. No data ever leaves localhost.*

## Prerequisites

- **Rust** 1.94+ (`rustup` recommended)
- **Cargo** 1.94+ (`cargo` recommended)
- **Ollama** 0.17.4+ installed and running (`ollama serve`)
- A Vision Language Model pulled locally (e.g. `ollama pull moondream`)
- **Linux** with X11 display server (Wayland support planned)

## Build

```bash
cargo build --workspace
```

## Quick Start

```bash
# Start the capture daemon
cargo run -p chronos-daemon -- start

# Query your activity logs
cargo run -p chronos-daemon -- query --from 2025-01-01

# Check system status
cargo run -p chronos-daemon -- status
```

## Project Status

This is a **v0.1 MVP/POC** — Linux X11 only, single-monitor, basic VLM integration.

See the [design document](docs/design/0001-chronos-personal-context-engine.md) for the full architecture and future roadmap.

## License

[MIT](LICENSE)
