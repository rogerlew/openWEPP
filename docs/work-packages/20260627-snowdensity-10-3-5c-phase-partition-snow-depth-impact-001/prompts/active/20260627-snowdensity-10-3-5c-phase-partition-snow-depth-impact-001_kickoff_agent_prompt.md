# Kickoff Prompt - SNOWDENSITY-10.3.5c

Execute `docs/work-packages/20260627-snowdensity-10-3-5c-phase-partition-snow-depth-impact-001/package.md`.

The objective is to run default `legacy_rst` versus opt-in
`OPENWEPP_SNOWDENSITY1035_PHASE_MODEL=harder_pomeroy_hourly` through the real
direct-production WAT path for the maritime/mixed paired snow-depth surfaces and
decide whether the phase selector improves coupled snow-depth evidence.

Guardrails:

- Use `openwepp-cli-hill --direct-production-executor`, not
  `openwepp-snowbench coe-melt`.
- Do not change production physics, defaults, fixtures, output schemas, parser
  surfaces, CLI selectors, melt, density, canopy, radiation, frost, or
  compatibility runtime code.
- Do not assign defect verdicts to observation-blocked surfaces.
- Close only with current evidence for every package acceptance gate.

Required final validation is defined in `package.md`.
