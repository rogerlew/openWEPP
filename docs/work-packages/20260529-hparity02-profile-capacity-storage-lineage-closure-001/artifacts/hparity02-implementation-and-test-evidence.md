# HPARITY02 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production implementation
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  - Added WB13 profile-capacity runtime symbol projection:
    `wb13_profile_depth_mm`, `wb13_profile_porosity_cap_mm`,
    `wb13_profile_fc_store_mm`, `wb13_profile_wp_store_mm`.
  - Added legacy-shape layer normalization and `scon`-aligned profile-capacity
    helper path for baseline-authoritative symbol derivation.
- Static: `crates/openwepp-runner/src/hillslope/mod.rs`
  - Removed placeholder `ProfilePorosityCap` fallback path.
  - Switched WB13 publication to authoritative precomputed symbols when present.
  - Added typed guards for finite/positive profile-capacity publication values.

## Workspace gates
- Ran: `cargo fmt --check` -> pass
  (`/tmp/hparity02_20260529T204555Z/gates/cargo_fmt_check.*.log`)
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` -> pass
  (`/tmp/hparity02_20260529T204555Z/gates/cargo_clippy_workspace.*.log`)
- Ran: `cargo test --workspace` -> pass
  (`/tmp/hparity02_20260529T204555Z/gates/cargo_test_workspace.*.log`)
- Ran: `cargo deny check` -> pass
  (`/tmp/hparity02_20260529T204555Z/gates/cargo_deny_check.*.log`)

## Parity rerun execution
- Ran: generated candidate hillslope outputs (`H1..H39.wat.parquet`) via
  `openwepp-cli-hill` batch.
  - Status file:
    `/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_batch_status.tsv`
  - Result: `39/39` hillslopes `rc=0`.
- Ran: semantic comparator batch against baseline partitions with year-key
  alignment (`--candidate-year-offset 2012`).
  - Status file:
    `/tmp/hparity02_20260529T204555Z/parity/reports/semantic_status.tsv`
  - Report root:
    `/tmp/hparity02_20260529T204555Z/parity/reports/semantic/`
  - Summary:
    `/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_semantic_summary.json`

## Environment note
- Ran: comparator venv bootstrap for parquet support:
  - `uv venv /tmp/hparity02_20260529T204555Z/venv --python 3.12`
  - `uv pip sync --python /tmp/hparity02_20260529T204555Z/venv/bin/python tools/legacy_comparison_suite/requirements.lock.txt`
