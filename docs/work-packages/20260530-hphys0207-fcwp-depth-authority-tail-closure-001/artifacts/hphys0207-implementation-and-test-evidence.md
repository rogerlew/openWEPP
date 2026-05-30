# HPHYS0207 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production implementation
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  - Promoted `wb13_profile_fc_store_mm`/`wb13_profile_wp_store_mm` emission to
    normalized-profile storage aggregates computed from corrected normalized
    layers (`Wb13ProfileSymbols.fc_store/wp_store`).
  - Removed parser-layer-depth aggregate publication for WB13 FC/WP storage
    seeds in this path.
- Static: `crates/openwepp-runner/src/hillslope/mod.rs`
  - WB13 row builder now consumes required runtime symbols
    `wb13_profile_fc_store_mm`/`wb13_profile_wp_store_mm` for
    `ProfileFCStore`/`ProfileWPStore`.
  - Added explicit typed guards for FC/WP storage non-negativity and ordering:
    `ProfilePorosityCap >= ProfileFCStore >= ProfileWPStore`.
  - Removed WB13 FC/WP fallback synthesis from parser-layer aggregates.
- Static: test surfaces updated in:
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
  - `tests/integration/parser_runtime_seam_integration.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs` (unit tests)

## Workspace validation gates
- Ran: `cargo fmt --check` -> pass.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- Ran: `cargo test --workspace` -> pass.
- Ran: `cargo deny check` -> pass (non-fatal duplicate/license allowance
  warnings only).

## 39-hillslope diagnostic rerun (MEASURE-HP207-004)
- Ran: run root:
  `/tmp/hphys0207_20260530T042607Z/parity/`
- Ran: hillslope batch status:
  `/tmp/hphys0207_20260530T042607Z/parity/reports/hillslope_batch_status.tsv`
  - `39/39` hillslopes `rc=0`.
- Ran: semantic comparator status:
  `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic_status.tsv`
  - `39/39` semantic jobs `rc=0`.
- Ran: summary:
  `/tmp/hphys0207_20260530T042607Z/parity/reports/hillslope_semantic_summary.json`

## Predecessor deltas (fail-count + residual magnitude)
- Ran + Static: FC/WP fail-hillslope deltas:
  - vs HPHYS0206 (`/tmp/hphys0206_20260530T032538Z/parity/reports/hillslope_semantic_summary.json`)
    - `ProfileFCStore`: `39 -> 27` (improved by `12` hillslopes)
    - `ProfileWPStore`: `39 -> 1` (improved by `38` hillslopes)
  - vs HPHYS0205 (`/tmp/hphys0205_20260530T022235Z/parity/reports/hillslope_semantic_summary.json`)
    - `ProfileFCStore`: `39 -> 27` (improved by `12` hillslopes)
    - `ProfileWPStore`: `39 -> 1` (improved by `38` hillslopes)
  - vs HPARITY02 baseline (`/tmp/hparity02_20260529T204555Z/parity/reports/hillslope_semantic_summary.json`)
    - `ProfileFCStore`: `27 -> 27` (no change)
    - `ProfileWPStore`: `1 -> 1` (no change)
- Ran: FC/WP residual-magnitude deltas (mean abs diff, average across H1..H39):
  - `ProfileFCStore`: `6.4922` (HPHYS0205) -> `2.0527` (HPHYS0207), `-4.4395`
  - `ProfileWPStore`: `1.8894` (HPHYS0205) -> `0.0573` (HPHYS0207), `-1.8321`
  - `ProfileFCStore`: `7.2212` (HPHYS0206) -> `2.0527` (HPHYS0207), `-5.1685`
  - `ProfileWPStore`: `2.2445` (HPHYS0206) -> `0.0573` (HPHYS0207), `-2.1872`

## Comparator dependency note
- Ran: initial semantic batch failed because `pyarrow` was not available in the
  system Python path.
- Ran: installed locked comparator dependency using:
  `uv pip sync tools/legacy_comparison_suite/requirements.lock.txt --python .venv/bin/python`
  then re-ran semantic comparator successfully with `.venv/bin/python`.
