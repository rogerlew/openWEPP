# SNOWDENSITY-10.3.1a Per-Day Cancov Direct Runtime

Status: complete.

Package type: contract-first direct-runtime diagnostic fidelity / blocker
resolution.

Primary authority: `docs/planning/snow-frost-fidelity-strategy.md` §10.3,
SNOWDENSITY-10.3.1 blocker disposition, and `SC-SNOWFREEZE-001`
`INV-SNOWFREEZE-050`, `INV-SNOWFREEZE-056`, and `INV-SNOWFREEZE-057`.

Closure target: `COMPLETE-10-3-1A-PER-DAY-CANCOV-DIRECT-RUNTIME`.

Objective: replace scalar diagnostic `cancov` replay in the snowbench/CoE melt
bridge with the per-day canopy trajectory produced by the direct production
growth-state path, so later mixed/deciduous melt adjudication is not blocked by
stale initial-condition canopy.

## Scope

- Amend `SC-SNOWFREEZE-001` before runtime edits.
- Surface the direct-production day-input canopy value used by snow liquid
  partition.
- Export a daily `cancov` series from the snowbench bridge using the direct production growth-state path exposed through the executor/day-input trajectory.
- Make `openwepp-snowbench coe-melt` consume that daily series rather than a
  scalar runtime-surface value.
- Publish series summary evidence in JSON/Markdown reports.
- Add focused tests proving the series exists, is bounded, is consumed by CoE
  replay, and leaves existing forcing/CoE-boundary schemas compatible.

## Non-Scope

- No melt, density, albedo, radiation, frost, or canopy tuning.
- No fixture input edits.
- No default activation.
- No parser/runfile/user CLI selector.
- No production output schema change.
- No compatibility-runtime deletion or rollback removal.
- No canopy-stratum adjudication or low-canopy verdict.

## Acceptance Gates

- `SC-SNOWFREEZE-001` records the per-day canopy authority and obligation.
- Snowbench export writes `canopy_series.csv` with one row per simulation day,
  date alignment, and values in `[0, 1]`.
- The series is generated from direct production day inputs after growth-state
  publication and before snow liquid partition, not from a repeated static
  runtime-surface scalar.
- CoE melt replay consumes `canopy_series.csv` by date and fails closed on
  missing, duplicated, non-finite, out-of-bounds, or length-mismatched rows.
- Existing PySnobal `forcing.csv` schema and CoE `coe_melt_snow.csv` boundary
  schema remain compatible with downstream readers.
- Focused tests pass for the export bridge, CoE melt replay, and CoE-bound
  density replay.
- Required gates pass or close with an explicit `HOLD`:
  `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`,
  `cargo deny check`,
  and `git diff --check`.

## Phase Plan

1. Read 10.3.1/10.3.2 evidence, the strategy doc, `SC-SNOWFREEZE-001`, and
   direct production snow/canopy code.
2. Amend the contract and add contract-derived tests.
3. Add direct publication day-input canopy evidence and snowbench export
   series.
4. Switch CoE melt replay to date-aligned series consumption.
5. Run focused evidence, then full gates as feasible.
6. Record review, verification, line-count governance, owned files, handoff,
   and final disposition.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

## Completion Summary

Closure: `COMPLETE-10-3-1A-PER-DAY-CANCOV-DIRECT-RUNTIME`.

The package resolves the SNOWDENSITY-10.3.1 blocker by replacing scalar
snowbench/CoE canopy replay with a direct-production per-day canopy series.

Implemented changes:

- `SC-SNOWFREEZE-001` v90 now binds `cancov_daily_series`,
  `INV-SNOWFREEZE-063`, `OBL-SNOWFREEZE-P-038`, and the 10.3.1a addendum.
- `DirectPublicationDayInput` carries optional `canopy_cover_fraction` evidence
  and the executor validates it when present.
- `DirectProductionDayInputBuilder` sets that field from
  `growth_state_for_publication.canopy_cover_fraction`, the same value used by
  snow liquid partition and canopy interception.
- `export_pysnobal_inputs` runs a direct-production publication capture to write
  `canopy_series.csv` plus source/min/max/mean/first/last/dynamic report
  evidence.
- `openwepp-snowbench coe-melt` reads the sidecar by date and fails closed on
  missing, duplicate, out-of-order, non-finite, out-of-bounds, or
  length-mismatched canopy rows.
- The PySnobal forcing CSV and CoE boundary CSV schemas remain unchanged.

Validation passed:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`
- `rg -n "qwet|frzftp" crates || true` (no hits)

No melt, density, albedo, radiation, frost, canopy tuning, fixture edits,
default activation, parser/runfile/user CLI selector, production output schema
change, or compatibility-runtime deletion was made.

## Closeout Artifacts

- `artifacts/implementation-evidence.md`
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
