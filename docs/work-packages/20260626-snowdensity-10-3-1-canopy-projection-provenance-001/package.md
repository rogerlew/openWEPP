# SNOWDENSITY-10.3.1 Canopy Projection Provenance

Status: complete.

Package type: evidence-only canopy provenance / melt-precondition gate.

Primary authority: `docs/planning/snow-frost-fidelity-strategy.md` §10.3,
item 1 "Canopy Projection Provenance"; `SC-SNOWFREEZE-001`
`INV-SNOWFREEZE-050` as the downstream rubric discipline.

Closure target: `COMPLETE-10-3-1-CANOPY-PROJECTION-PROVENANCE-ARCHIVED`.

Objective: archive the raw fixture `.man` canopy values, the wepppy projected
per-day winter `cancov` authority, and the openWEPP runtime `cancov` currently
available for all eight `tests/fixtures/cancov_forest/` sites before any melt
adjudication interprets canopy leverage.

## Scope

- Read `docs/planning/snow-frost-fidelity-strategy.md` §10.3.1,
  `tests/fixtures/cancov_forest/README.md`, the eight fixture `.man` files, and
  the wepppy deciduous/mixed forest management validation package.
- Extract raw management and initial-condition `cancov` values from all eight
  fixture `.man` files.
- Archive the wepppy projected per-day winter-canopy evidence and map it by
  management class.
- Run the existing openWEPP snowbench exporter for all eight fixtures and
  archive its `primary_canopy_cover_fraction` runtime surface value.
- Disposition every raw-vs-projected-vs-runtime mismatch, including Sleepers
  pasture as the tentative lowest-cancov endpoint.

## Non-Scope

- No production physics changes.
- No melt, albedo, density, frost, partition, radiation, or coefficient tuning.
- No parser/runfile/user CLI selector, output schema, default activation, or
  fixture input edits.
- No attempt to make canopy-stratified Marcell/Harvard verdicts; that belongs
  to §10.3.2.

## Acceptance Gates

- Package artifacts include one per-fixture table with:
  raw `.man` plant-line `cancov`, raw initial-condition `cancov`, wepppy
  projected winter `cancov` where applicable, openWEPP runtime
  `primary_canopy_cover_fraction`, and mismatch disposition.
- The wepppy projected values are cited to the exact upstream artifact and are
  not treated as openWEPP runtime evidence when openWEPP lacks a per-day series.
- OpenWEPP runtime evidence is generated from the current repo using
  `openwepp-snowbench export-pysnobal` or a stricter existing diagnostic.
- Sleepers pasture endpoint is resolved as either a valid low-canopy endpoint or
  a named hold/follow-up blocker.
- No melt physics, production defaults, output schemas, or fixture inputs are
  changed.
- Focused checks pass: `git diff --check`, package evidence file presence, and a
  source scan proving no production physics or fixture input write set changed.

## Phase Plan

1. Scaffold package and active prompt.
2. Extract raw `.man` canopy fields.
3. Archive wepppy winter-cancov projection evidence.
4. Run openWEPP snowbench exporter across all eight fixtures and summarize
   runtime `cancov`.
5. Disposition mismatches and close complete or hold.
6. Record reviews, verification, line-count governance, owned-file manifest,
   worker handoff, and final disposition.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

## Completion Summary

Closure: `COMPLETE-10-3-1-CANOPY-PROJECTION-PROVENANCE-ARCHIVED`.

Archived raw fixture `.man` canopy inputs, upstream wepppy seasonal canopy
projection authority, and current openWEPP snowbench runtime-surface `cancov`
for all eight `cancov_forest` fixtures.

Key result: current openWEPP snowbench CoE melt diagnostics consume a static
runtime surface seeded from the fixture initial-condition `cancov`, not the
wepppy per-day seasonal canopy projection. This is correct provenance evidence,
but it blocks interpreting a mixed/deciduous gradient melt adjudication as
seasonal-canopy evidence until a later package passes per-day canopy trajectories
or explicitly accepts static initial-condition canopy for the question under
test.

Mismatch disposition:

- HJ Andrews and Tenderfoot close as high evergreen controls: raw initial,
  projected evergreen, and openWEPP runtime all equal `0.90`.
- Berthoud and Mores Creek are RAP_TS-adjusted conifer exceptions: openWEPP
  runtime equals raw initial `0.05` and `0.82`, not the generic evergreen
  projection `0.90`.
- Harvard and Marcell are static mixed-canopy diagnostics today:
  openWEPP runtime equals raw initial `0.55`, while upstream wepppy projected
  winter mean is `0.44446`.
- Hubbard Brook is static deciduous-canopy diagnostics today:
  openWEPP runtime equals raw initial `0.20`, while upstream wepppy projected
  winter mean is `0.06653`.
- Sleepers is not proven as the lowest-cancov endpoint in current openWEPP
  diagnostics: runtime `cancov = 0.50`, with no upstream pasture seasonal
  projection authority in the wepppy forest-management package.

No fixture input, production physics, parser/runfile/user CLI selector, output
schema, default, coefficient, radiation, albedo, density, melt, partition, or
frost behavior was changed.

## Closeout Artifacts

- `artifacts/canopy-provenance-evidence.md`
- `artifacts/mismatch-disposition.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/review-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`
