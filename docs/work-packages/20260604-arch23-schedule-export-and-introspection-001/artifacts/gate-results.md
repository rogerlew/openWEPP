# Gate Results

Status: queued
Evidence mode: not-run

## Required Gates

- `cargo fmt --check`
  - Status: queued
  - Evidence:
- `cargo clippy --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --all-targets -- -D warnings`
  - Status: queued
  - Evidence:
- `cargo test --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml`
  - Status: queued
  - Evidence:
- `bash tools/release/check_hillslope_schedule_export.sh`
  - Status: queued
  - Evidence:
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Status: queued
  - Evidence:
- `cargo test --workspace`
  - Status: queued
  - Evidence:
- `cargo deny check`
  - Status: queued
  - Evidence:

## Blockers

- None recorded.
