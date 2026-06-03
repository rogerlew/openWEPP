# Baseline Provenance Map

Status: completed
Evidence mode: static

Static: HPHYS0273 is a governance-authoring package. It does not port baseline
physics or change runtime behavior.

## Provenance Touchpoints

- HPHYS0272 radiation-unit lineage remains the motivating concrete defect:
  `radly` is `Ly d^-1` at the daily parser seam and converts once to `radmj`
  in `MJ m^-2 d^-1` before hourly `hr_tmp` publication.
- `docs/specifications/unit-governance.md:52` records that HPHYS0272 authority
  as a canonical unit-governance example.
- No new equation, constant, or process-physics migration was introduced by
  HPHYS0273.

## Follow-Up Baseline Needs

Packages HPHYS0274 through HPHYS0279 must add baseline/provenance maps where
they touch concrete runtime units, conversion helpers, output metadata, or
guards.

Ran: not-run.
