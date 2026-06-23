# R7D8 Direct HBP EROD15 Export Alias Parity

Status: complete.

Package type: Array-native runtime defect-closure implementation package.

Objective: close
`HOLD-R7D7-HBP-EROD15-SEDIMENT-EXPORT-ALIASES-DIRECT-PRODUCER-GAP` by
implementing direct producer authority for HBP EROD15 sediment export aliases
without compatibility runtime wrapping.

Rationale: R7D7 closed the R7D6 WB16/PASS `peakro` residual. Fresh H2637
5-day evidence then had WAT and PASS byte identity and direct
`compatibility_edge_invocations = 0`. HBP differed only because
compatibility publishes runtime EROD15 aliases
`total_detachment_kg = 0.6` and
`sediment_concentration_kg_m3_0001 = 6.816136920064195` for the latest event,
while direct HBP published `0.0` for both. The remaining blocker was direct
EROD15/HBP sediment-export alias authority, not peak runoff publication.

Included scope:

- Re-read `SC-SED-001`, `SC-SYSTEM-001` EROD15/HBP boundary-carry authority,
  R7D6/R7D7 artifacts, and the MOFE03 Wave-2 seed/runtime code.
- Trace compatibility HBP aliases
  `total_detachment_kg`, `total_deposition_kg`, and
  `sediment_concentration_kg_m3_0001` from producer/update source through HBP
  serialization.
- Implement direct producer authority for the same HBP aliases using typed
  direct EROD13/EROD14/EROD15 state and any required MOFE sediment-coupled
  handoff inputs.
- Iterate on the H2637 5-day fixture until HBP byte identity is achieved or a
  narrower contract/science blocker is proven with exact field evidence.
- Preserve WAT/PASS byte identity and direct `compatibility_edge_invocations =
  0`.

Excluded scope:

- Re-opening WB16 `peakro` publication unless new evidence contradicts R7D7.
- Forcing direct HBP sediment aliases to zero.
- Copying compatibility `HillslopeWritebackSurface` values into direct
  publication or using compatibility WB13/PASS rows as direct authority.
- Broad default activation, performance, or release-readiness work beyond this
  HBP closure.

Intended write set:

- `docs/work-packages/20260623-r7d8-direct-hbp-erod15-export-alias-parity-001/**`
- `docs/work-packages/README.md`
- `docs/architecture/array-native-runtime-specification.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/**`
- focused tests under touched crates.

Acceptance gates:

- H2637 5-day direct production exits `0` with
  `compatibility_edge_invocations = 0`.
- HBP byte identity holds against fresh compatibility output.
- WAT and PASS byte identity from R7D7 remains intact.
- HBP payload parse confirms parity for peak, duration, total detachment,
  total deposition, sediment concentration, and particle flow fraction.
- Focused direct-production and direct-runtime sediment/HBP tests pass.
- Review, verification, parity, line-count, and worker-handoff artifacts are
  updated with `Static:` and `Ran:` evidence.

Final disposition: complete.

R7D8 closed
`HOLD-R7D7-HBP-EROD15-SEDIMENT-EXPORT-ALIASES-DIRECT-PRODUCER-GAP` for the
current H2637 5-day direct-production acceptance fixture. Fresh evidence under
`/tmp/r7d8ad-h2637-5day` has default and direct exits at `0`,
HBP/loss/PASS/PLOT/WAT byte identity, parsed HBP latest-event parity for peak,
duration, total detachment, total deposition, sediment concentration, and
particle flow fraction, and direct manifest
`compatibility_edge_invocations = 0`.

The package also fixed two closure blockers found during final gates:

- PASS `peakro` parity now indexes simulation-owned PASS rows by
  `sim_day_index` during the R6J cutover adapter instead of reusing a stale
  final runtime scalar for every row.
- The former R6I direct/compatibility PMET seed parity expectation was replaced
  by an R7D boundary test proving direct day-2 PMET seed state follows direct
  WB14 lineage and is not forced through compatibility stale infiltration
  authority.
