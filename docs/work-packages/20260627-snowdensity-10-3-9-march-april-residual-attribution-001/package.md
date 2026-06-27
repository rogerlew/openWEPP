# SNOWDENSITY-10.3.9 March/April Residual Attribution

Status: complete
Owner: Codex
Date: 2026-06-27

## Objective

Attribute the remaining March/April snow-depth residual failures after the
SNOWDENSITY-10.3.8 opt-in liquid holding-capacity correction. The package must
answer which single-process lever is most defensible next without changing snow
physics, defaults, fixtures, parser surfaces, or public output schemas.

## Context

SNOWDENSITY-10.3.8 improved the coupled direct-production WAT snow-control gate
from `1147` to `761` failures across `1415` paired Sleepers/Harvard rows, with
no paired surface worse. Snow control still does not clear, and the remaining
failures are reported as concentrated in March/April across open and
mixed/deciduous regimes. The next package must classify those residuals before
adding another lever.

## Required Reading

- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` §10.3
- `docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/package.md`
- `docs/work-packages/20260627-snowdensity-10-3-8-liquid-holding-capacity-001/artifacts/liquid-holding-capacity-coupled-wat.json`
- `tools/snowfreeze_observed/phase_partition_snowdepth_adjudication.py`
- `tools/snowfreeze_observed/observed_harness.py`
- `tests/fixtures/cancov_forest/observations/README.md`

## Scope

In scope:

- Add a diagnostic-only March/April residual attribution tool.
- Consume the committed 10.3.8 coupled WAT report and its real direct-production
  WAT output paths.
- Pair only installed snow-depth observation surfaces:
  Sleepers South field, Sleepers W9 hardwood, Harvard hardwood, and Harvard
  open.
- Report HJ Andrews and Hubbard Brook as observation-blocked for residual
  verdicts unless paired snow-depth observations are installed.
- Split all-season versus March/April residual statistics.
- Classify March/April failures into explicit mechanism buckets:
  over-persistent pack / too-deep residual, patchy meltout or snow-cover
  depletion, density or compaction deficit, SWE/mass excess where correspondence
  is defensible, under-persistence, and depth-only unresolved cases.
- Produce JSON and Markdown artifacts with a recommended next one-lever route.
- Add a focused integration guard test proving diagnostic confinement and
  artifact schema/disposition.

Out of scope:

- Production snow/frost physics changes.
- New selectors, defaults, parser/runfile/user surfaces, or public WAT schemas.
- Fixture input changes or site-specific tuning.
- Activation/retirement decisions for existing opt-in candidates.
- Treating Harvard SWE columns as decisive mass authority unless unit
  correspondence is explicitly proven; depth and source density remain usable
  for correspondence-caveated diagnostics.

## Closure Gates

Closure may be `complete` only if:

- The report uses the 10.3.8 opt-in coupled WAT candidate
  `coe_liquid_holding_capacity_v1` as its residual baseline.
- The report quantifies what fraction of remaining failures occur in March/April,
  by surface and cover class.
- The report identifies the leading residual-attribution mechanism and explains
  why the next route follows from the evidence.
- Observation-blocked surfaces are not counted as paired verdict surfaces.
- Diagnostic boundaries are preserved: no runtime production physics, defaults,
  selectors, fixtures, schema, coefficients, radiation, canopy, phase partition,
  density, melt, rain heat, longwave, or frost code changes.
- Focused gates pass:
  - `.venv/bin/python tools/snowfreeze_observed/march_april_residual_attribution.py`
  - `cargo test --test snowdensity10_3_9_march_april_residual_attribution`
  - `cargo clippy --test snowdensity10_3_9_march_april_residual_attribution -- -D warnings`

## Status Log

- 2026-06-27: Scaffolded package and diagnostic closure criteria.
- 2026-06-27: Added diagnostic attribution tool and guard test.
- 2026-06-27: Generated March/April residual attribution report from the
  SNOWDENSITY-10.3.8 coupled WAT candidate artifact.
- 2026-06-27: Closed package complete. March/April accounts for `282/761`
  remaining paired failures; dominant attribution is over-persistent depth with
  density/compaction and patchy meltout/depletion signals. Next route:
  spring pack-depletion and compaction adjudication.
