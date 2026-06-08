# Gate Results

Status: queued
Evidence mode: not-run

Static:
- queued

Ran:
- not run

Required gates (queued):
- cargo fmt --check
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test -p openwepp-hillslope-orchestrator --tests
- cargo test --workspace
- cargo deny check
