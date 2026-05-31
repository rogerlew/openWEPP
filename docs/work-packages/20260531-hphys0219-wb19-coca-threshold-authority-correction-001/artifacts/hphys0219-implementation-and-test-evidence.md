# HPHYS0219 Implementation and Test Evidence

Status: completed
Evidence mode: Static + Ran

## Scope
- Correct WB19 threshold authority from `cpm_####` to baseline-authoritative
  `coca_####`.
- Project `coca` symbols from corrected-layer runtime lineage.
- Run workspace gates and rerun `unpalatable-rind` 39-hillslope semantic lane.

## Production implementation (Static)
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - Replaced WB19 threshold symbol family from `cpm_####` to `coca_####`.
  - WB19 layer-state loader now validates `coca` domain and computes:
    `drfc = fc + (1-coca)*dg`.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  - Added `coca` to corrected-layer runtime lineage structs.
  - Added validation for `coca` (`0 < coca <= 1`).
  - Added projection for OFE-scoped and indexed `coca` symbols.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - Updated runtime-input assertions and tuple shapes to include `coca`.
- `crates/openwepp-runner/src/hillslope/mod.rs`
  - Updated WB19-touching test fixtures to provide `coca_0001`.

## Contract-test implementation (Static)
- Added:
  `tests/integration/hphys0219_wb19_coca_threshold_contract.rs`
- Removed:
  `tests/integration/hphys0218_wb19_cpm_threshold_contract.rs`
- Updated WB19-touching integration fixtures to include explicit `coca_0001`
  and `coca_0002` symbols where required.

## Ran validation evidence
- `cargo fmt --check` (pass)
- `cargo clippy --workspace --all-targets -- -D warnings` (pass)
- `cargo test --workspace` (pass)
- `cargo deny check` (pass; duplicate/allowlist warnings only)
- Targeted tests:
  - `cargo test --test hphys0219_wb19_coca_threshold_contract --test wb19_lateral_drainage_physics_kernel_contract` (pass)
  - `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::` (pass)

## Ran rerun/semantic evidence
- Rerun root:
  `/tmp/hphys0219_20260531T083756Z/parity`
- Hillslope run status:
  `/tmp/hphys0219_20260531T083756Z/parity/reports/hillslope_batch_status.tsv`
  (`39/39`, all `rc=0`)
- Semantic status:
  `/tmp/hphys0219_20260531T083756Z/parity/reports/semantic_status.tsv`
  (`39/39`, all `rc=0`)
- Semantic summary:
  `/tmp/hphys0219_20260531T083756Z/parity/reports/hillslope_semantic_summary.json`
  and `.tsv`

## Key rerun deltas vs HPHYS0218 (Ran)
- `ProfileFCStore`: unchanged (`27/39`, mean `~2.05269116`)
- `Dp`: fail count unchanged (`39/39`), mean improved
  `0.3269109066808126 -> 0.2808444379937233`
- `latqcc`: fail count unchanged (`39/39`), mean regressed
  `0.7496847101588174 -> 0.7948120660697657`
- `Total-Soil`: fail count unchanged (`39/39`), mean regressed
  `140.69907071572365 -> 140.82816405718864`
- `SoilWaterTotal`: fail count unchanged (`39/39`), mean regressed
  `140.69907071572365 -> 140.82816405718864`
