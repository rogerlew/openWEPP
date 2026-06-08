# REFACTOR011 gate results

Static:
- Required gates come from `package.md` exit criteria and are mandatory for closure.

Ran:
- `cargo fmt --check`: PASS (exit 0).
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS (exit 0).
- `cargo test -p openwepp-sim-contract --tests`: PASS (exit 0, no crate-level failures).
- `cargo test --workspace`: PASS (exit 0).
- `cargo deny check`: PASS (warnings only for pre-existing duplicate lock entries and unmatched license allow-list entries, no blocking errors).
