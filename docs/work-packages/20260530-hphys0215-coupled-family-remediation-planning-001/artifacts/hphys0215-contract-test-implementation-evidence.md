# HPHYS0215 Contract-Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Existing contract-derived tests observed
- Runner WB13 publication guards and FC/WP authority tests:
  `crates/openwepp-runner/src/hillslope/mod.rs` (HPHYS0202..0213 suites).
- Orchestrator WB19 realized-withdrawal tests:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`.

## Planning output for follow-on test obligations
HPHYS0216+ packages must each include stream-scoped contract-derived tests:
1. `HPHYS0216` (`ProfileFCStore`):
   - authority continuity tests for normalized profile vs layer-mapped
     thresholds,
   - guard-hard-fail coverage for non-finite/negative storage states.
2. `HPHYS0217` (`Dp`):
   - daily carry-state and percolation publication lineage tests,
   - unit/aggregation invariants under fixed forcing vectors.
3. `HPHYS0218` (`latqcc`):
   - `q/Qdd/Qd/SubRIn` coupling and decomposition invariants.
4. `HPHYS0219` (`Total-Soil`, `SoilWaterTotal`):
   - aggregate continuity from mutable layer state to WB13 publication.

## Gate confirmation
- Ran: `cargo test --workspace` passed for this package run root
  `/tmp/hphys0215_20260531T041655Z/`.
