# HPHYS0206 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production implementation
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
  - Added typed runtime failures for normalized corrected-layer authority:
    - `HS-RUNTIME-E-060` (`MissingCorrectedLayerNormalizationInput`)
    - `HS-RUNTIME-E-061` (`CorrectedLayerNormalizationUnavailable`)
    - `HS-RUNTIME-E-062` (`CorrectedLayerMappingIncomplete`)
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  - Removed permissive raw-theta fallback for authoritative layer publication.
  - Added normalized corrected-layer seed collection and correction pipeline.
  - Added deterministic overlap-weighted mapping from normalized corrected
    layers into parser layer intervals.
  - Added typed fail-closed behavior when normalized lineage/mapping cannot
    satisfy authoritative FC/WP publication.
- Static: test surfaces updated in:
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
  - `tests/integration/parser_runtime_seam_integration.rs`

## Workspace validation gates
- Ran: `cargo fmt --check` -> pass.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- Ran: `cargo test --workspace` -> pass.
- Ran: `cargo deny check` -> pass (non-fatal duplicate/license allowance
  warnings only).

## 39-hillslope diagnostic rerun (MEASURE-HP206-004)
- Ran: run root:
  `/tmp/hphys0206_20260530T032538Z/parity/`
- Ran: hillslope batch status:
  `/tmp/hphys0206_20260530T032538Z/parity/reports/hillslope_batch_status.tsv`
  - `39/39` hillslopes `rc=0`.
- Ran: semantic comparator status:
  `/tmp/hphys0206_20260530T032538Z/parity/reports/semantic_status.tsv`
  - `39/39` semantic jobs `rc=0`.
- Ran: semantic report root:
  `/tmp/hphys0206_20260530T032538Z/parity/reports/semantic/`
- Ran: summary:
  `/tmp/hphys0206_20260530T032538Z/parity/reports/hillslope_semantic_summary.json`

## Predecessor deltas (fail-count + residual magnitude)
- Ran + Static: FC/WP fail-hillslope deltas:
  - vs HPHYS0205 (`/tmp/hphys0205_20260530T022235Z/parity/reports/hillslope_semantic_summary.json`)
    - `ProfileFCStore`: `39 -> 39` (no improvement)
    - `ProfileWPStore`: `39 -> 39` (no improvement)
  - vs HPARITY02 baseline (`/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_semantic_summary.json`)
    - `ProfileFCStore`: `27 -> 39` (regressed)
    - `ProfileWPStore`: `1 -> 39` (regressed)
- Ran: FC/WP residual-magnitude deltas (mean abs diff, average across H1..H39):
  - `ProfileFCStore`: `6.4922` (HPHYS0205) -> `7.2212` (HPHYS0206), `+0.7290`
  - `ProfileWPStore`: `1.8894` (HPHYS0205) -> `2.2445` (HPHYS0206), `+0.3552`
