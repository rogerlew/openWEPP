# HPHYS0218 Implementation and Test Evidence

Status: completed
Evidence mode: Static + Ran

## Scope
- Implement WB19 `drfc`-equivalent threshold consumption in production helpers
  and kernel phases.
- Add WB19 cpm-threshold contract tests and update WB19-dependent fixtures.
- Run workspace gates and full 39-hillslope rerun + semantic diagnostics.

## Production implementation (Static)
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - Added `wb19_cpm_symbol`.
  - Extended WB19 layer state load to require `cpm_####` and compute per-layer
    `drfc` threshold (`fc + (1-cpm)*dg`).
  - Switched WB19 drainable/withdrawal helpers to consume drain-threshold
    vectors.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - Lateral saturated-zone classification uses `drfc` thresholds.
  - Lateral/drainage drainable-pool and realized-withdrawal paths use
    `drfc` thresholds.
  - Drainage watertable classification uses `drfc` thresholds.
- `crates/openwepp-runner/src/hillslope/mod.rs`
  - Updated WB19 unit test fixtures to provide required `cpm_0001`.

## Contract-test implementation (Static)
- Added:
  `tests/integration/hphys0218_wb19_cpm_threshold_contract.rs`
- Updated WB19-invoking integration fixtures to include `cpm_0001/0002=1.0`
  for guard continuity.
- Added test target registration in `Cargo.toml`.

## Ran validation evidence
- `cargo fmt --check` (pass)
- `cargo clippy --workspace --all-targets -- -D warnings` (pass)
- `cargo test --workspace` (pass)
- `cargo deny check` (pass; duplicate/allowlist warnings only)
- Targeted WB19 tests:
  - `cargo test --test wb19_lateral_drainage_physics_kernel_contract --test hphys0218_wb19_cpm_threshold_contract` (pass)
  - `cargo test -p openwepp-runner hphys0213_wb19` (pass)

## Ran rerun/semantic evidence
- Rerun root:
  `/tmp/hphys0218_20260531T075251Z/`
- Hillslope run status:
  `/tmp/hphys0218_20260531T075251Z/parity/reports/hillslope_batch_status.tsv`
  (`39/39`, all `rc=0`)
- Semantic status:
  `/tmp/hphys0218_20260531T075251Z/parity/reports/semantic_status.tsv`
  (`39/39`, all `rc=0`)
- Semantic summary:
  `/tmp/hphys0218_20260531T075251Z/parity/reports/hillslope_semantic_summary.json`
  and `.tsv`

## Key rerun deltas vs HPHYS0217 (Ran)
- `ProfileFCStore`: unchanged (`27/39`, mean `~2.05269116`)
- `Dp`: fail count unchanged (`39/39`), mean worsened
  `0.2643680891653757 -> 0.3269109066808126`
- `latqcc`: fail count unchanged (`39/39`), mean improved
  `0.8131880775568228 -> 0.7496847101588174`
- `Total-Soil`: fail count unchanged (`39/39`), mean improved
  `140.87503038397858 -> 140.69907071572365`
- `SoilWaterTotal`: fail count unchanged (`39/39`), mean improved
  `140.87503038397858 -> 140.69907071572365`
