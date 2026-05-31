# HPHYS0216C Contract-Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Contract-derived test posture in this package
- No new tests were authored in HPHYS0216C (diagnostics-only scope).
- Existing tests consulted for authority posture:
  - `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs` (`hphys0216_*` tests)

## Diagnostic runs executed
Ran diagnostics to characterize the regression shape:
1. Parsed HPHYS0216 semantic reports under
   `/tmp/hphys0216_20260531T053959Z/parity/reports/semantic/`.
2. Joined baseline/candidate parquet rows with `duckdb` to compute signed FC
   offsets per hillslope and verify per-day constancy.
3. Queried source lineage in
   `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
   and `crates/openwepp-runner/src/hillslope/mod.rs`.

## Follow-up test obligations
The next remediation package must add contract-derived tests that fail unless:
1. FC layer-authority publication includes normalized-depth tail closure
   (or an equivalent explicit authority mapping).
2. FC publication no longer exhibits deterministic per-profile static offsets
   against baseline cohort under unchanged forcing.
