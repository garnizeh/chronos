//! chronos-capture: Hardware-level screen capture and RAM-buffered frame management.
//!
//! This crate handles the low-level interaction with the OS display server (e.g., X11)
//! and provides a high-performance, bounded ring buffer to keep recent frames
//! available in memory without persisting them to disk.

/// Thread-safe, bounded memory storage for frames.
pub mod ring_buffer;
/// X11-based screen capture implementation using xcap.
pub mod x11;
