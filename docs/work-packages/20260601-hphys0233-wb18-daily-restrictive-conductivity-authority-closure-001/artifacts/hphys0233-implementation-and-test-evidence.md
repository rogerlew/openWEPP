# HPHYS0233 Implementation and Test Evidence

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Production implementation

Static:
1. Soil-runtime projection now publishes restrictive-layer controls:
   - `slflag` (binary),
   - `kslast` (converted to m/s),
   - `ui_bdrkth`.
   File:
   `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
2. WB18 percolation kernel now consumes restrictive controls with typed guards:
   - validates `slflag` domain,
   - requires `kslast > 0` when `slflag=1`,
   - applies bottom-layer harmonic effective conductivity in daily lane.
   File:
   `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
3. WB13 `Dp` publication now prefers flux-surface `D` over stale state-surface
   `D` to prevent lineage shadowing.
   File: `crates/openwepp-runner/src/hillslope/mod.rs`
4. Runtime-input tests assert restrictive symbol publication:
   File: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`

## Gate and test execution

Ran:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

Observed:
- all required gates passed.
- WB18 contract tests include passing vectors for:
  - daily restrictive harmonic branch,
  - `kslast` invalid-domain hard-fail.
- runner test suite includes passing anti-shadow vector for WB13 `Dp`.

## Cohort rerun and semantic comparison

Ran:
1. `H1..H39` rerun and semantic comparison batch for `unpalatable-rind`.
2. Semantic report aggregation for monitored HOLD columns.

Evidence root:
- `/tmp/hphys0233_20260601T211306Z/parity/`

Observed:
- execution coverage: `39/39` hillslopes (`rc=0`) from
  `hillslope_batch_status_h_only.tsv`.
- semantic coverage: `39/39` reports (`rc=0`) from `semantic_status.tsv`.
- monitored-column summary:
  - `Dp`: `mean_abs_diff_mean=0.22350421314678484` (improved from HPHYS0232),
  - `Total-Soil`: `134.12909172196171` (improved),
  - `SoilWaterTotal`: `134.12909172196171` (improved),
  - `latqcc`: `0.7903973406116435` (regressed),
  - `ProfileFCStore`: unchanged at `2.0526911601041165`.
