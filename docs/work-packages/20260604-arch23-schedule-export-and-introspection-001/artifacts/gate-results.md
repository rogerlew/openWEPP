# Gate Results

Status: complete
Evidence mode: Ran

## Required Gates

- `cargo fmt --check`
  - Status: passed
  - Evidence: exit 0.
- `cargo clippy --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --all-targets -- -D warnings`
  - Status: passed
  - Evidence: exit 0.
- `cargo test --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml`
  - Status: passed
  - Evidence: 97 library tests passed; binary/doc tests passed with 0 tests.
- `bash tools/release/check_hillslope_schedule_export.sh`
  - Status: passed
  - Evidence: printed `hillslope schedule export artifacts are congruent`.
- Intentional drift check for `tools/release/check_hillslope_schedule_export.sh`
  - Status: passed
  - Evidence: gate returned status 1 after temporary JSON drift, reported `hillslope schedule export drift detected for hillslope-phase-schedule.json`, artifact was restored, and the gate then passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Status: passed
  - Evidence: exit 0.
- `cargo test --workspace`
  - Status: passed
  - Evidence: exit 0; all workspace unit, integration, and doc tests passed.
- `cargo deny check`
  - Status: passed with warnings
  - Evidence: exit 0; reported duplicate-crate warnings for `getrandom`, `hashbrown`, and `twox-hash`, plus unmatched license allowances for `ISC` and `Unicode-DFS-2016`; final line: `advisories ok, bans ok, licenses ok, sources ok`.

## Blockers

None.
