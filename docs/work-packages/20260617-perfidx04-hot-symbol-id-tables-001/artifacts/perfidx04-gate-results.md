# PERFIDX04 Gate Results

Ran:
- `cargo fmt --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo test --workspace`: pass.
- `cargo deny check`: pass (`advisories ok, bans ok, licenses ok, sources ok`).
- `git diff --check`: pass.

Ran:
- Gate logs are stored under `/tmp/perfidx04/artifacts/gates/`.

Static:
- `cargo check -p openwepp-kernel-contract -p openwepp-hillslope-orchestrator -p openwepp-runner` also passed during implementation, but it is not counted as a workflow substitute for the full test gate.
