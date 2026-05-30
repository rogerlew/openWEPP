# HPHYS0208 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production implementation
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  - Added runtime projection/publication of `sat`, `por_####`, and `cpm_####`
    (including primary OFE aliases).
  - Extended corrected-layer normalization/mapping to publish authoritative
    `por`/`cpm` alongside `thetfc`/`thetdr`.
  - Added corrected-layer `cpm` domain validation (`0 < cpm <= 1`).
- Static: `crates/openwepp-runner/src/hillslope/mod.rs`
  - Updated WB11 seed initialization to coupled threshold lineage:
    - `FCi = (thetfc_i-thetdr_i)*dg_i`
    - `ULi = (por_i-thetdr_i)*dg_i`
    - `st(i) = (((sat*por_i)*cpm_i)-thetdr_i)*dg_i`
    - `soilw(i) = st(i)+thetdr_i*dg_i`
  - Added typed guards for `por`, `cpm`, saturation floor derivation, and
    lane-specific `sat` caps (`daily=0.95`, `hourly=1.0`).
  - Propagated `ExecutionLane` into WB11 seed path.
- Static: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - Updated WB14 `ksatadj` top-two-layer metrics to consume `thetdr_####` when
    present (HPHYS0208 layout) with compatibility fallback for legacy FC/WP
    layout surfaces.
- Static: `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - Updated normalization/mapping expectations for `por`/`cpm` projection and
    `sat` projection continuity.
- Static: `tests/integration/hphys0208_fc_threshold_coupled_residual_contract.rs`
  - Added HPHYS0208 coupled-lineage contract execution coverage.
- Static: `Cargo.toml`
  - Added missing `[[test]]` registration so the new HPHYS0208 integration test
    is part of workspace test execution.

## Workspace validation gates
- Ran: `cargo fmt --check` -> pass.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- Ran: `cargo test --workspace` -> pass.
- Ran: `cargo deny check` -> pass (warnings only, exit code `0`).
- Ran: logs under `/tmp/hphys0208_20260530T155837Z/gates/`.

## Contract-derived targeted tests
- Ran: `cargo test -p openwepp --test hphys0208_fc_threshold_coupled_residual_contract` -> pass.
- Ran: `cargo test -p openwepp-runner hphys0208_` -> pass.

## 39-hillslope diagnostic rerun
- Ran: rerun root:
  `/tmp/hphys0208_20260530T155837Z/parity/`
- Ran: hillslope execution status:
  `/tmp/hphys0208_20260530T155837Z/parity/reports/hillslope_batch_status.tsv`
  - `39/39` hillslopes `rc=0`.
- Ran: semantic comparator status:
  `/tmp/hphys0208_20260530T155837Z/parity/reports/semantic_status.tsv`
  - `39/39` semantic jobs `rc=0`.
- Ran: semantic summary:
  `/tmp/hphys0208_20260530T155837Z/parity/reports/hillslope_semantic_summary.json`
  - `ProfileFCStore`: `27/39` fail hillslopes.
  - `Dp`: `39/39` fail hillslopes.
  - `latqcc`: `39/39` fail hillslopes.
  - `Total-Soil`: `39/39` fail hillslopes.
  - `SoilWaterTotal`: `39/39` fail hillslopes.

## Predecessor comparison (vs HPHYS0207)
- Ran + Static: fail-hillslope deltas vs
  `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic/H*.semantic.json`:
  - `ProfileFCStore`: `27 -> 27` (no change)
  - `Dp`: `39 -> 39` (no change)
  - `latqcc`: `39 -> 39` (no change)
  - `Total-Soil`: `39 -> 39` (no change)
  - `SoilWaterTotal`: `39 -> 39` (no change)
- Ran: mean-abs-diff average deltas (H1..H39):
  - `ProfileFCStore`: `2.0527 -> 2.0527` (`+0.0000`)
  - `Dp`: `0.1870 -> 40.1559` (`+39.9689`, regression)
  - `latqcc`: `83.5557 -> 173.2285` (`+89.6728`, regression)
  - `Total-Soil`: `122.1685 -> 116.0649` (`-6.1036`, improved)
  - `SoilWaterTotal`: `122.1685 -> 116.0649` (`-6.1036`, improved)

## Execution note
- Ran: hillslope rerun `H*.wat.parquet` surfaces were emitted under
  `/tmp/hphys0208_20260530T155837Z/parity/hillslope_output/`; semantic
  comparator used that authoritative candidate output path.
