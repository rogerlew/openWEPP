# HPHYS0216D Implementation and Test Evidence

Status: completed
Evidence mode: Static + Ran

## Production implementation landed
1. `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
   - publishes explicit `wb13_profile_fc_tail_mm` from normalized-profile FC
     minus parser-layer authoritative FC aggregation.
2. `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`
   - added typed runtime-input errors for non-finite/negative FC tail
     contributions.
3. `crates/openwepp-runner/src/hillslope/mod.rs`
   - `ProfileFCStore` publication now consumes layer aggregation plus
     `wb13_profile_fc_tail_mm` with typed fail-closed guards.

## Contract/test updates landed
- Canonical contracts: `SC-WATBAL-001`, `SC-SOIL-001`, `SC-PERC-001`,
  `SC-SYSTEM-001`.
- Contract-derived tests:
  - `tests/integration/hphys0202_profile_fc_wp_lineage_contract.rs`
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs` test module.

## Ran validation
- See `artifacts/gate-results.md` for full command list and outcomes.

## Implementation closure statement
- FC publication authority is now explicit layer+tail.
- Legacy regression root cause (tail omission) is addressed in code and tests.
- Integrated `HOLD` remains due downstream coupled-family closure sequence, not
  unresolved FC authority implementation steps.
