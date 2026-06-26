# SNOWDENSITY-09 Diagnostic Coupled WAT Rerun

Status: complete.

Package type: contract-first diagnostic runtime bridge / gate rerun.

Primary authority: `SC-SNOWFREEZE-001` v89, especially
`INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`, `INV-SNOWFREEZE-050`,
`INV-SNOWFREEZE-060`, `INV-SNOWFREEZE-061`, `INV-SNOWFREEZE-062`, and
`OBL-SNOWFREEZE-P-037`.

Closure target: one of
`COMPLETE-09-COUPLED-OPT-IN-WAT-RERUN-FROST-BLOCKED`,
`COMPLETE-09-COUPLED-OPT-IN-WAT-RERUN-FROST-AUTHORIZED`,
or `HOLD-09-<BLOCKER>`.

Objective: build and use an authorized diagnostic-only coupled WAT/publication
path for the non-SNOTEL frost fixtures so the current frost-site rubric can
compare default `legacy_wepp` WAT against the actual direct-runtime
`physics_bulk_density_compaction_v1` snow-depth state consumed through the
winter-column carry and WAT `Snow-Depth`.

## Scope

- Amend `SC-SNOWFREEZE-001` before runtime or harness edits.
- Add a diagnostic-only direct-production selector for
  `snow_density_model = physics_bulk_density_compaction_v1` that is unavailable
  by default and not surfaced through runfile/parser/user CLI activation.
- Preserve `legacy_wepp` as default and rollback.
- Extend direct-production snow trace evidence to publish the selected
  `snow_density_model`.
- Run the non-SNOTEL frost-site rubric twice:
  - default direct-production WAT path;
  - diagnostic coupled opt-in WAT path.
- Publish per-site deltas for snow-control status, snow-depth residuals, frost
  rubric cells, and whether frost attribution is authorized.

## Non-Scope

- No default activation.
- No production parser/runfile/user CLI selector.
- No output schema change.
- No WAT rewriting or offline snow-only substitution.
- No coefficient, canopy, radiation, albedo, melt, density, or frost-physics
  tuning.
- No compatibility-runtime deletion or rollback removal.
- No SNOTEL density rerun unless the package gate finds the SNOWDENSITY-08
  SNOTEL artifact missing or invalid.

## Acceptance Gates

- `SC-SNOWFREEZE-001` records SNOWDENSITY-09 authority before code changes.
- Default direct-production WAT remains `legacy_wepp` without the diagnostic
  environment selector.
- Diagnostic opt-in WAT runs set the exact package-bound selector and trace at
  least one direct-production snow row with
  `snow_density_model = physics_bulk_density_compaction_v1`.
- The opt-in WAT path is coupled: WAT `Snow-Depth` comes from
  `snow.runtime_depth_m`, and frost uses winter-column snow state rather than an
  offline rewritten snow-depth series.
- The decision report compares default and opt-in non-SNOTEL rubric profiles and
  refuses frost attribution unless snow-control passes under the coupled opt-in
  path for fixtures with observed snow-depth rows and SNOWDENSITY-08 SNOTEL
  density evidence remains cleared.
- Sites without observed snow-depth rows are retained as diagnostic-only
  out-of-gate evidence and cannot count as snow-control pass, fail, or blocker.
- No site constants, tuning, output schema, runfile/parser/user CLI selector, or
  default activation are introduced.
- Focused SNOWDENSITY-09 integration guards pass.
- Required gates pass:
  `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`,
  `cargo deny check`,
  source scan for `qwet|frzftp`,
  and `git diff --check`.

## Phase Plan

1. Read SNOWDENSITY-07, SNOWDENSITY-08, the non-SNOTEL rubric harness,
   `SC-SNOWFREEZE-001`, and direct-production snow/frost publication code.
2. Amend the contract and add contract-derived tests.
3. Add the diagnostic-only direct-production snow-density selector and trace
   evidence.
4. Add a compact SNOWDENSITY-09 runner under `tools/snowfreeze_observed/` that
   executes default and opt-in WAT paths and writes a decision report.
5. Run evidence, copy compact outputs to package artifacts, and record reviews,
   disposition, verification, gate results, line-count governance, worker
   handoff, and final status.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

## Completion Summary

SNOWDENSITY-09 is complete as
`COMPLETE-09-COUPLED-OPT-IN-WAT-RERUN-FROST-BLOCKED`.

The package built the authorized diagnostic coupled WAT path and proved the
selector reached the process that generated WAT: the default run traced
`75,610` direct-production snow rows with `legacy_wepp`, and the opt-in run
traced `75,610` rows with `physics_bulk_density_compaction_v1`.

The opt-in path improved snow-depth residuals at the three paired-snow
non-SNOTEL sites but did not pass snow control:

- Sleepers South mean signed snow-depth residual improved
  `0.4108 m -> 0.2964 m`; max absolute residual improved
  `1.5968 m -> 1.1601 m`.
- Sleepers W9 mean signed snow-depth residual improved
  `0.3233 m -> 0.2103 m`; max absolute residual improved
  `1.0599 m -> 0.7381 m`.
- GGD498 Morris mean signed snow-depth residual improved
  `0.0672 m -> 0.0586 m`; max absolute residual improved
  `0.3924 m -> 0.3215 m`.

Snow-control status remains three `SNOW_CONTROL_FAILED` gate-eligible
paired-snow sites. SCAN Mandan ND and Reynolds Creek ID have no observed
snow-depth rows and are reported as diagnostic-only out-of-gate evidence, not
as snow-control gate pass/fail/blocker inputs. Frost attribution remains
unauthorized under `INV-SNOWFREEZE-047/048/050/062`.

No default activation, parser/runfile/user CLI selector, output schema,
coefficient/canopy/radiation/albedo/melt/density/frost tuning, WAT rewriting,
site constants, or compatibility-runtime change was made.
