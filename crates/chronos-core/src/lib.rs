//! chronos-core: Shared domain models, error types, and trait abstractions for the Chronos ecosystem.
//!
//! This crate defines the foundational building blocks used by the daemon, capture engine,
//! and inference modules. By centralizing these definitions, we ensure type safety and
//! consistent data representation across the entire local-first pipeline.

/// Error types and specialized Result alias.
pub mod error;
/// Core data structures (Frame, SemanticLog, Configs).
pub mod models;
/// Behavioral abstractions (ImageCapture, VisionInference).
pub mod traits;
