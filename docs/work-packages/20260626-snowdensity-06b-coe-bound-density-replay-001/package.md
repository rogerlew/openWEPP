# SNOWDENSITY-06B CoE-Bound Density Replay

Status: complete.

Package type: contract-first offline diagnostic replay and adjudication.

Primary authority: `SC-SNOWFREEZE-001` v85, especially
`INV-SNOWFREEZE-050`, `INV-SNOWFREEZE-056`, `INV-SNOWFREEZE-057`,
`INV-SNOWFREEZE-058`, and `INV-SNOWFREEZE-059`.

Closure target: one of
`COMPLETE-06B-COE-BOUND-DENSITY-REPLAY`,
`COMPLETE-06B-NON-PROMOTION`, or `HOLD-06B-<BLOCKER>`.

Objective: replay the SNOWDENSITY-06 `density_compaction_v1` state update
against fixed CoE melt/liquid/SWE-loss boundaries so density evidence is no
longer contaminated by the older `physics_bulk` degree-day melt surrogate.

## Scope

- Amend `SC-SNOWFREEZE-001` for SNOWDENSITY-06B CoE-bound density replay
  authority.
- Add an offline `openwepp-snowbench coe-bound-density` command.
- Run `density_compaction_v1` against fixed `legacy_coe` and
  `coe_shortwave_albedo_v1` CoE boundaries.
- Preserve fixed canopy, shortwave, albedo, melt coefficients, and CoE
  liquid/SWE-loss operands.
- Publish CoE SWE identity and density/densification robust-cell evidence.
- Adjudicate the five SNOTEL fixtures against the same snow-frost rubric used
  by SNOWDENSITY-06.

## Non-Scope

- No default activation.
- No production parser, runfile, runtime selector, output-schema, or direct
  runtime change.
- No mixed/deciduous low-canopy work; that remains SNOWDENSITY-05H scope after
  modernization.
- No melt, albedo, canopy, radiation, or site-specific retuning.
- No frost attribution.
- No replacement of `legacy_coe` default/rollback.

## Acceptance Gates

- `SC-SNOWFREEZE-001` records SNOWDENSITY-06B authority before code changes.
- Focused tests prove the replay command exists, is offline-only, preserves CoE
  SWE exactly on daily rows, and changes density/depth only through the
  candidate compaction update.
- Five-site adjudication reports `legacy_coe` and `coe_shortwave_albedo_v1`
  CoE-bound density profiles, whole-rubric context, density-cell summaries, and
  comparator context.
- Closure is one of:
  - `COMPLETE-06B-COE-BOUND-DENSITY-REPLAY`: a CoE-bound candidate beats
    legacy/as-built and openWEPP/as-built on the declared profile gate without
    site tuning; or
  - `COMPLETE-06B-NON-PROMOTION`: finite evidence classifies the candidate as
    not ready for runtime activation and names the next blocker; or
  - `HOLD-06B-<BLOCKER>`: an in-scope gate cannot be run or a prerequisite is
    absent.
- Required gates pass:
  `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`,
  `cargo deny check`,
  focused contract tests,
  source scan for `qwet|frzftp`,
  and `git diff --check`.

## Phase Plan

1. Read SNOWDENSITY-06, SNOWDENSITY-05G, the strategy table, and
   `SC-SNOWFREEZE-001`.
2. Amend the contract and contract-derived tests.
3. Implement the offline CoE-bound density replay command.
4. Add focused tests for command confinement, CoE SWE identity, and density-only
   replay behavior.
5. Add or extend adjudication tooling and run the five-site bounded profile.
6. Record implementation evidence, reviews, verification, line-count
   governance, worker handoff, gate results, and final disposition.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

## Completion Summary

Closure: `COMPLETE-06B-COE-BOUND-DENSITY-REPLAY`.

Amended `SC-SNOWFREEZE-001` to v85 with `INV-SNOWFREEZE-059`,
`OBL-SNOWFREEZE-P-034`, and the SNOWDENSITY-06B addendum. Added the
diagnostic-only `openwepp-snowbench coe-bound-density` command and SNOTEL
adjudication tooling. The replay generates a fixed CoE melt boundary first,
then applies `density_compaction_v1` to physical depth and density while
preserving CoE daily `snow_water_m` identity.

Five-site SNOTEL adjudication produced finite evidence for both `legacy_coe`
and `coe_shortwave_albedo_v1` boundaries. Both beat openWEPP/legacy as-built
on the whole-rubric and density-cell profile gates while preserving CoE SWE
identity. Best candidate:
`coe_bound_density_compaction_v1_coe_shortwave_albedo_v1`, robust failures
`9 -> 5`, robust score `84 -> 110`, density failures `9 -> 5`, density score
`16 -> 41`, and maximum CoE SWE identity residual
`4.440892098500626e-16 m`.

This package does not authorize default activation, production runtime
selectors, parser/runfile/CLI activation surfaces, output-schema changes,
mixed/deciduous low-canopy adjudication, or frost attribution. Next route:
SNOWDENSITY-07 runtime opt-in with conservation, publication, anti-alias,
direct/compatibility rollback, no-site-tuning, and default-disabled isolation
gates.

## Closeout Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/implementation-evidence.md`
- `artifacts/coe-bound-density-adjudication.{json,md}`
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
