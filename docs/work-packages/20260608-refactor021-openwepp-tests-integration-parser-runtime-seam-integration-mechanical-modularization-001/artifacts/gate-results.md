# Gate Results

Status: queued
Evidence mode: not-run

Static:
- queued

Ran:
- not run

Required gates:
- cargo fmt --check
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test -p openwepp --test parser_runtime_seam_integration
- cargo test --workspace
- cargo deny check
