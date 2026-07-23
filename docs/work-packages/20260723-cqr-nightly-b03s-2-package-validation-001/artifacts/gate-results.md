# Gate Results

- Ran: pre-decomposition characterization, 1/1 PASS.
- Ran: `cargo test -p openwepp-gate-planner --lib package_validation::tests:: -- --nocapture`, 15/15 PASS after decomposition.
- Ran: `cargo clippy -p openwepp-gate-planner --lib -- -D warnings`, PASS.
- Ran: `rustfmt --edition 2024 --check crates/openwepp-gate-planner/src/package_validation.rs`, PASS.
- Ran: focused `cargo llvm-cov` characterization, 1/1 PASS; target and extracted helpers have 100% function coverage.
- Ran: focused `cargo-crap 0.2.2`; target CRAP 4 and helper maximum CRAP 5.
- Ran: `cargo test -p openwepp-gate-planner package_validation::tests:: -- --nocapture`, BLOCKED before this package ran because concurrent `main.rs` tests do not compile (`Vec<String> == Value`; obsolete `ObservedSource::tree_sha256`). The library-only equivalent passed.
- Ran: `cargo fmt --all -- --check`, BLOCKED by formatting differences in concurrent `main.rs` changes. Target-file Rustfmt check passed.
- Ran: after `main.rs` settled, combined `cargo fmt --all -- --check`, package
  validation 15/15, and all-target planner Clippy passed before module
  disposition.
- Not run: HEAVY, TESTGATE, global CRAP, global coverage, and full workspace regression, per package scope.
