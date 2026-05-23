# INT10 Gate Results

Status: `complete`
Evidence mode: `Static + Ran`

## Pre-Implementation Contract Gate

- Command:
```bash
cargo test --test int10_plant_water_coupling_validation_contract -- --nocapture
```
- Result: `ok` (`3 passed`, `0 failed`).
- Sequencing note: this gate was executed before any production INT10 integration
  source edits. No production integration source edits were required in this
  package (`crates/openwepp-hillslope-orchestrator/src/lib.rs`,
  `crates/openwepp-kernel-contract/src/lib.rs`,
  `crates/openwepp-summary-accumulator/src/lib.rs` unchanged).

## Targeted Coupled Replay/Seam Checks

- `cargo test --test int10_plant_water_coupling_validation_contract`: `ok` (`3 passed`).
- `cargo test -p openwepp-hillslope-orchestrator annual_growth_phase_emits_typed_growth_context -- --nocapture`: `ok` (`1 passed`).
- `cargo test -p openwepp-hillslope-orchestrator perennial_growth_phase_emits_typed_growth_context -- --nocapture`: `ok` (`1 passed`).
- `cargo test --test parser_runtime_seam_integration management_rotation_fixture_projects_schedule_growth_and_decomp_runtime_surface_families -- --nocapture`: `ok` (`1 passed`).

## Required Rust Validation Gates

1. `cargo fmt --check`
- Result: `ok`

2. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: `ok`

3. `cargo test --workspace`
- Result: `ok`

4. `cargo deny check`
- Result: `ok` (`advisories ok, bans ok, licenses ok, sources ok`)
- Note: `license-not-encountered` warnings were emitted for unmatched allowlist
  entries; command exit status remained success.
