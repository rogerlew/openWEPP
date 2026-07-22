# Implementation

Static: the correction replaces the private inline `include!` wrapper in
`verifier.rs` with `mod coverage_tests;` and moves the source to Rust's natural
`src/verifier/tests/coverage_tests.rs` module path.

The moved source retains identical executable tokens and behavior. `cargo fmt`
reflowed eight existing expression/signature layouts, so the move is
behavior-preserving and formatting-only rather than byte-identical. No
production verifier logic, API, fixture semantics, visibility, or module
hierarchy changed.

Ran: a first path probe without the move failed at compile-time because Rust
resolved the nested path under `src/verifier/tests/`; no test ran. The package
authorized the portable move at `fa2d305f` before applying it.
