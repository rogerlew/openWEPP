# WB14 Replay Provenance and Infiltration Lineage

Status: `completed`
Evidence mode: `Static + Ran`

## Provenance Lineage
- Contract authority amended in:
  - `SC-RUNOFFPART-001` (WB14 infiltration + hyetograph addendum)
  - `SC-WATBAL-001` (WB14 coupling addendum)
  - `SC-CLIMATE-001` (WB14 hyetograph forcing addendum)
- Runtime implementation in:
  - `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- Contract-derived verification in:
  - `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`

## Infiltration Source Transition
- Prior posture: reconciliation accepted externally seeded `wb12_infiltration` values.
- WB14 posture: infiltration is computed from hyetograph + soil state inside runoff reconciliation.
- Writeback preserves downstream symbol continuity by updating `wb12_infiltration` with the computed value.

## Replay Signal
- WB14 nominal contract vector passes with computed outputs:
  - `wb12_infiltration = 2.909931093255933`
  - `Q = 0.290068906744067`
  - `wb12_runoff_closure_delta` within `1.0e-6`

## Authority Notes
- Green-Ampt lineage branch behavior is implemented as deterministic runtime branching (no empirical regression fit substitution).
- WB14 malformed hyetograph and missing-symbol failures are typed and fail-fast.
