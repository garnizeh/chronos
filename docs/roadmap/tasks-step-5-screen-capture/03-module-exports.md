# Task 5.3: Capture Module Exports and Verification

## Objective
Export the `ring_buffer` and `x11` modules from the `chronos-capture` crate and verify it conforms to CI standards.

## Mental Map / Go Parallel
This is the equivalent of running `go test ./pkg/capture/...` and ensuring your Go packages export the intended structs (capitalized vs uncapitalized). In Rust, we explicitly `pub mod` and optionally `pub use` to curate the public API surface area in `lib.rs`.

## Implementation Steps
- [ ] Update `crates/chronos-capture/src/lib.rs`:
  ```rust
  pub mod ring_buffer;
  pub mod x11;
  ```
- [ ] Run `cargo test -p chronos-capture` and verify all tests added in Tasks 5.1 and 5.2 are green.
- [ ] Run `cargo clippy -p chronos-capture -- -D warnings` and fix any linting errors.
- [ ] Run `cargo fmt -p chronos-capture`.

## Conventional Commit
`test(chronos-capture): verify ring buffer and x11 module exports`
