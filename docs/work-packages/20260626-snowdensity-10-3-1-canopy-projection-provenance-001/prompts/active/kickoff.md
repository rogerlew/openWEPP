# Kickoff

Execute `SNOWDENSITY-10.3.1 Canopy Projection Provenance` inside the local
openWEPP repository.

Required reading:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3.1
- `tests/fixtures/cancov_forest/README.md`
- `/home/workdir/wepppy/docs/work-packages/20260626_deciduous_mixed_forest_managements/artifacts/winter-cancov-validation.md`

Scope:

- Archive raw fixture `.man` canopy values.
- Archive upstream wepppy projected winter `cancov` values by management class.
- Run current openWEPP diagnostics for runtime `cancov` across all eight
  fixtures.
- Disposition all raw/projection/runtime mismatches, especially Sleepers pasture
  and mixed/deciduous raw-vs-projected divergence.

Do not change production physics, defaults, output schemas, parser/runfile/user
selectors, or fixture input files.

