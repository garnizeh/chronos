//! chronos-daemon: The central engine and CLI for the Chronos context system.
//!
//! This crate implements the core orchestration logic, CLI command handling,
//! and persistent storage. It ties together screen capture from `chronos-capture`,
//! AI inference from `chronos-inference`, and models from `chronos-core`.

pub mod cli;
pub mod database;
pub mod handlers;
pub mod pipeline;
