# HPHYS0236 Implementation and Test Evidence

Status: completed  
Evidence mode: Ran

## Production Code Changes

1. `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
   - WB18 percolation now executes hourly lane as explicit iterative substeps
     (`24` loop) with per-substep recomputation from mutable layer state.
   - Per-layer `pei` and bottom-layer `D/Pe` publication are accumulated across
     substeps, preserving existing guard/fail-closed behavior.
2. `tests/integration/wb18_percolation_physics_kernel_contract.rs`
   - Added iterative projection helper and anti-regression assertions to
     enforce non-divisor-only hourly semantics.

## Workspace Gates

1. `cargo build -p openwepp-runner --bin openwepp-cli-hill` -> pass
2. `cargo fmt --check` -> pass
3. `cargo clippy --workspace --all-targets -- -D warnings` -> pass
4. `cargo test --workspace` -> pass
5. `cargo deny check` -> pass (existing duplicate/license-not-encountered
   warnings only; exit success)

## Cohort Rerun and Semantic Readjudication

Ran:
1. `H1..H39` hillslope rerun root:
   `/tmp/hphys0236_20260601T230600Z/parity`
2. Execution status:
   `/tmp/hphys0236_20260601T230600Z/parity/reports/hillslope_batch_status_h_only.tsv`
   (`39/39`, all `rc=0`)
3. Semantic comparator status:
   `/tmp/hphys0236_20260601T230600Z/parity/reports/semantic_status.tsv`
   (`39/39`, all `rc=0`)
4. Aggregated semantic summary:
   `/tmp/hphys0236_20260601T230600Z/parity/reports/hillslope_semantic_summary.json`

## Result

Implementation closure measures were executed end-to-end and all required gates
passed. Residual monitored-family adjudication remains `HOLD` and is captured
in `hphys0236-residual-authority-gap-matrix.md` and `hphys0236_disposition.md`.
