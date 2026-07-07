# Implementation

Status: EXECUTED-HOLD-D16-SUITE. Evidence mode: Static.

Completed:

- Added native `ow-lanuse-1` cropland/routing support to WEPPpy
  `managements.py`.
- Added validated Disturbed route coefficient defaults and provenance.
- Enriched static and generated extended lookup rows with route coefficients.
- Added an explicit Disturbed native management producer helper.
- Updated WEPPpy docs and parameterization ADR.
- Updated openWEPP LANUSE authority docs to name Disturbed as an explicit
  producer.
- Added a Disturbed-generated native management fixture and openWEPP runtime
  projection test.

Not completed:

- Full D16 active plain-vs-hybrid cohort execution at current mesh.
- H2637 plus contrasting active hydrologic sensitivity for the new coefficient
  table.

Reason for hold:

This package resolved the input-source authority blocker and proved downstream
parse/projection consumption. Executable D16 active suite/timing remains a
separate follow-on package because it requires current selected-cohort native
input generation and active run orchestration beyond this source-acquisition
increment.
