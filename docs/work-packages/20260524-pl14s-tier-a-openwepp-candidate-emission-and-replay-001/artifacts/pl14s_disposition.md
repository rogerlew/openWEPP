# PL14S Disposition

Status: `completed-with-hold`
Evidence mode: `Static + Ran`

## Static
- Disposition code: `PL14S_COMPLETE_WITH_HOLD`
- Package objective status:
  - replay lane + diagnostics tooling execution: completed
  - semantic parity pass: not achieved
- Scope discipline:
  - hillslope WB13 semantic lane only
  - erosion/sediment parity explicitly excluded

## Ran
- Exit criteria checks:
  - [x] Tier-A replay executed with openWEPP-emitted candidate output.
  - [x] Semantic comparator report generated with required diagnostics fields.
  - [x] Provenance/hash artifacts persisted.
  - [x] Required repository gates executed and passing.
  - [ ] Semantic parity pass achieved (`semantic_pass=false`).
- Hold reasons:
  1. Comparator row-key sets do not overlap (`common_row_count=0`).
  2. Semantic report indicates baseline-only rows (`1095`) and candidate-only rows (`1`).
  3. Candidate WAT emission path is not yet full daily watbal execution in runner/CLI:
     current lane emits first-day synthesized WB13-style output rather than full
     day-by-day scheduler/kernel watbal trajectory.
- Hold-lift expectation:
  - rerun PL14S/PL15S lane only after runner/CLI candidate emission is wired to
    full watbal day-loop execution and candidate WB13 trajectory aligns to the
    baseline key domain sufficiently to produce common-row comparisons.
