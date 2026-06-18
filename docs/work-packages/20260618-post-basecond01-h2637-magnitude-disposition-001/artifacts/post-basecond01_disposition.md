# POST-BASECOND01 Disposition

Evidence class: Static + Ran

Status: complete.

Package:
`20260618-post-basecond01-h2637-magnitude-disposition-001`

## Verdict

`COMPLETE-NO-DEFECT`.

The FARPOINT01 H2637 71% magnitude flag is resolved as
`CORRECT-BY-CONSTRUCTION` / `NO DEFECT` for the internal openWEPP lineage. The
absolute physical magnitude remains a documented `CONTRACT-GAP`.

## Evidence Used

Static:

- ADR-0017 comparator posture: legacy is a flag, not a target.
- `SC-INFILE-SOIL-001` and `SC-SUBHYD-001` contract lineage for soil
  conductivity projection and WB19 lateral flow.
- Source-intent evidence summarized by STAGE2-BASE-CONDUCTIVITY and
  BASECOND01.

Ran:

- FARPOINT01 H2637 closure and legacy contrast evidence.
- MAGPARITY01 transfer, area, export, and conservation checks.
- STAGE2-LATQCC selected-day WB19 equation reconstruction.
- REFINTENT001 H2637 remeasure showing `ksatadj` byte-inert on H2637.
- BASECOND01 H2637 no-UI rerun showing `runvol_pct_precip` unchanged at
  `71.0036550031206`.

This package did not rerun the full H2637 fixture again because it makes no
production or contract change; it synthesizes the completed ran evidence from
the dependency packages.

## Decisions

- No defect-closure package is opened from this disposition.
- No production code is changed.
- No science contract is changed.
- The FARPOINT01 71% magnitude flag is closed as no-defect / expected.
- The absolute magnitude question is recorded as a deferred backlog note:
  [`docs/backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md`](../../../backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md).

## Guardrails Retained

- Do not make `wb19_lateral_ssh` harmonic to chase legacy.
- Do not reopen vertical `ssc` as an unresolved H2637 defect.
- Do not reopen `ksatadj` for H2637 without evidence that H2637 has an active
  `ksatadj` branch.
- Do not treat the legacy `55.5%` value as a target.

## Closure

The H2637 lateral-magnitude investigation has reached the honest boundary of the
current authority model. Internal adjudication is exhausted; external absolute
magnitude authority is a future concept, not a blocker.
