//! chronos-inference: Vision-Language Model (VLM) integration for screen analysis.
//!
//! This crate provides the glue logic between the Chronos pipeline and local
//! AI inference engines like Ollama. It handles image encoding, prompt management,
//! and structured JSON parsing of model outputs.

pub mod ollama;

pub use ollama::OllamaVision;
