# Worker Handoff

Status: complete

Evidence mode: ran

Static:

- HPHYS0305 executed the ADR-0016 Required Continuation Order step 2
  instrumentation package.
- Production physics edits remain unauthorized.

Ran:

- Added canonical HPHYS0305 paired instrumentation authority in
  `SC-WATBAL-001.md`.
- Added openWEPP trace fields for paired snow/rain/depth/density state
  diagnostics and bumped trace schema to `hphys0245-debug-v16`.
- Added fixed-comparator observe instrumentation archived as
  `fixed-baseline-instrumentation.patch`.
- Proved fixed release, observe-off, and observe-on WAT identity for H1, H7,
  and H39.
- Generated nine H1/H7/H39 target-window ledger rows.
- All nine rows route to `surface-gap-hold` because baseline paired `amelt`
  observations are incomplete in the required windows.
- No production physics correction is authorized from HPHYS0305.

## Required Continuation

Scaffold the next package as a baseline-observe semantics closure package before
any snow producer correction. The package should determine whether the missing
`amelt` rows are:

- an observe-instrumentation placement defect,
- a branch-active/inactive-hour contract gap where inactive melt terms must be
  explicitly represented,
- or a true producer lineage gap requiring a different paired source surface.

Required next package scope:

- Amend canonical SC authority for branch-active/inactive melt-term observation
  semantics.
- Add contract tests that require complete paired surfaces or typed
  missing-surface HOLD classification.
- Patch fixed comparator observe instrumentation only as evidence, proving WAT
  bit identity remains intact.
- Re-run H1/H7/H39 target windows and only then classify first divergent
  forcing/state/melt source.
- Keep downstream compensation prohibited.
