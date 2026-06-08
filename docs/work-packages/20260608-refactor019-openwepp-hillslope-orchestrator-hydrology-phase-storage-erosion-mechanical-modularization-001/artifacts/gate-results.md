# Gate Results

Status: complete
Evidence mode: Static/Ran

Static:
- package scope and exit criteria captured in `package.md`
- required commands and outputs summarized below

Ran:
- 2026-06-08T22:50:27Z: `cargo fmt --check` (exit 0, no diffs)
- 2026-06-08T22:50:27Z: `cargo clippy --workspace --all-targets -- -D warnings` (pass)
- 2026-06-08T22:50:27Z: `cargo test -p openwepp-hillslope-orchestrator --tests` (107 passed, 0 failed)
- 2026-06-08T22:50:27Z: `cargo test --workspace` (pass, full suite exit 0)
- 2026-06-08T22:50:27Z: `cargo deny check` (advisories ok, bans ok, licenses ok, sources ok)

Required gates:
- cargo fmt --check
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test -p openwepp-hillslope-orchestrator --tests
- cargo test --workspace
- cargo deny check
