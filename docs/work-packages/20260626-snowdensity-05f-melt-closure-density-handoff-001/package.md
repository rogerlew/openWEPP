# SNOWDENSITY-05F Melt Closure / Density Handoff

Status: complete.

Package type: contract closure, activation-boundary, and density handoff.

Primary authority: `SC-SNOWFREEZE-001` v82, especially
`INV-SNOWFREEZE-050`, `INV-SNOWFREEZE-052`, `INV-SNOWFREEZE-053`,
`INV-SNOWFREEZE-054`, `INV-SNOWFREEZE-055`, `INV-SNOWFREEZE-056`, and
obligations `OBL-SNOWFREEZE-P-027` through `OBL-SNOWFREEZE-P-031`.

Closure target: COMPLETE-05F-MELT-CLOSURE-DENSITY-HANDOFF or
HOLD-05F-MELT-CLOSURE-DENSITY-HANDOFF.

Closure: COMPLETE-05F-MELT-CLOSURE-DENSITY-HANDOFF.

Objective: close the SNOWDENSITY-05 melt-modernization ladder by freezing the
accepted opt-in melt boundary that downstream density work may consume, while
preserving `legacy_coe` as default/rollback and documenting that 05E does not
authorize default activation.

## Scope

- Amend `SC-SNOWFREEZE-001` with the 05F closure/routing decision.
- Decide and document the accepted selector, default, rollback, cold-start
  albedo policy, diagnostic surfaces, and activation evidence baseline.
- Add contract-derived tests proving the 05F closure authority is present and
  that production default selection remains `legacy_coe`.
- Update planning/work-package logs and handoff to SNOWDENSITY-06.
- Record source scans and evidence from 05D/05E without rerunning full SNOTEL
  adjudication unless the package changes evidence semantics.

## Non-Scope

- No default activation.
- No production parser/runfile/CLI selector for opt-in melt.
- No output schema changes.
- No melt coefficient fitting or site-specific defaults.
- No snow-only radiation scalar or radiation-provider selection.
- No density/pack physics implementation.
- No frost attribution or non-SNOTEL defect classification.

## Acceptance Gates

- `SC-SNOWFREEZE-001` records the 05F closure decision and keeps
  `coe_shortwave_albedo_v1` opt-in only.
- The density-facing handoff names exact melt selector, source operands,
  diagnostic operands, rollback path, and residual risks.
- The cold-start albedo policy exposed by 05E is ratified or the package closes
  `HOLD` with a blocker.
- Source scan proves production default still selects `legacy_coe` and no
  parser/output/default activation surface was added.
- Contract-derived tests enforce the closure decision and default confinement.
- Required gates pass:
  `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`,
  `cargo deny check`.

## Completion Summary

- Amended `SC-SNOWFREEZE-001` to v80 with `INV-SNOWFREEZE-056`,
  `OBL-SNOWFREEZE-P-031`, boundary disposition, and the 05F addendum.
- Closed `coe_shortwave_albedo_v1` as an accepted opt-in melt boundary for
  downstream density work only.
- Preserved `legacy_coe` as default and rollback.
- Ratified the same-day future snowfall cold-start albedo continuity rule.
- Bound activation evidence to both 05E diagnostic replay and H as-built
  context.
- Dispositioned independent Claude review caveats: 05E diagnostic replay is
  regime-limited until the harness uses configured coniferous-forest per-day
  canopy cover (winter `cancov` about `0.9`) and native/proven shortwave;
  SNOWDENSITY-06 must clear that harness-fidelity entry gate before density or
  activation verdicts.
- Verified the Brock-2000 albedo constants against
  `references/copyrighted/brock2000.pdf`.
- No default activation, parser/runfile/CLI selector, output schema,
  coefficient, radiation-source, or density-physics change was made.
- Added `snowdensity05f_melt_closure_handoff` contract/default-confinement
  integration test.

## Phase Plan

1. Read 05E handoff, 05D/05E evidence, strategy, and `SC-SNOWFREEZE-001`.
2. Scaffold package and required evidence files.
3. Amend contract authority for 05F closure and density handoff.
4. Add focused contract/default-confinement tests.
5. Run source scans and focused tests.
6. Run full gates, complete reviews/verification, and close package.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

Subagent authorization: not used.
