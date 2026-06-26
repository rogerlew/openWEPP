# SNOWDENSITY-07 Runtime Opt-In

Status: complete.

Disposition: COMPLETE-07-RUNTIME-OPT-IN.

Package type: contract-first runtime opt-in implementation.

Primary authority: `SC-SNOWFREEZE-001` v86, especially
`INV-SNOWFREEZE-050`, `INV-SNOWFREEZE-055`, `INV-SNOWFREEZE-056`,
`INV-SNOWFREEZE-059`, and `INV-SNOWFREEZE-060`.

Closure target: one of
`COMPLETE-07-RUNTIME-OPT-IN`,
`COMPLETE-07-TYPED-OPT-IN-HOLD-RUNFILE`,
or `HOLD-07-<BLOCKER>`.

Objective: couple the SNOWDENSITY-06B accepted CoE-bound density result into
the typed winter-column/direct snow-coupling runtime behind an explicit
default-disabled selector while preserving `legacy_wepp` default and rollback.

## Scope

- Amend `SC-SNOWFREEZE-001` for runtime opt-in authority before code changes.
- Add a typed snow-density selector with `legacy_wepp` default behavior and
  `physics_bulk_density_compaction_v1` opt-in behavior.
- Keep CoE SWE/liquid/routed-melt boundaries authoritative under the opt-in
  path; the density model may mutate only physical snow depth and bulk density.
- Preserve direct/compatibility rollback: compatibility and surface-driven
  legacy paths remain `legacy_wepp`; opt-in is available only to typed callers.
- Project opt-in snow depth/density through state mutation, downstream operands,
  shadow projection, runtime carry, and publication-facing winter-column state.
- Prove default-disabled isolation, conservation/anti-alias behavior,
  no-site-tuning constants, and explicit boundary-state separation.

## Non-Scope

- No default activation.
- No runfile/parser/user CLI selector.
- No output-schema change beyond existing `Snow-Water` and `Snow-Depth`
  publication surfaces already sourced from runtime snow state.
- No mixed/deciduous low-canopy melt adjudication.
- No coefficient, canopy, radiation, albedo, or melt retuning.
- No frost attribution.
- No deletion of legacy/compatibility snow behavior.

## Acceptance Gates

- `SC-SNOWFREEZE-001` records SNOWDENSITY-07 authority before code changes.
- Focused tests prove:
  - `legacy_wepp` remains identity relative to the pre-existing typed snow
    partition result;
  - opt-in `physics_bulk_density_compaction_v1` preserves CoE SWE,
    routed-melt, liquid-forcing, and albedo/melt operands while changing only
    runtime depth/density;
  - CoE boundary depth/density carry remains separate from opt-in publication
    depth/density so the next CoE melt boundary is not contaminated;
  - direct R4G state mutation, downstream operands, shadow projection, and
    runtime carry receive the opt-in depth/density and the CoE boundary carry;
  - surface-driven compatibility/default path still uses `legacy_wepp` and
    exposes no default opt-in activation.
- Closure artifacts include operand-lineage and anti-alias evidence for SWE vs
  depth/density, dual reviews, review disposition, dual verification,
  line-count governance, gate results, worker handoff, and final disposition.
- Required gates pass:
  `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`,
  `cargo deny check`,
  focused SNOWDENSITY-07 tests,
  source scan for `qwet|frzftp`,
  and `git diff --check`.

## Phase Plan

1. Read SNOWDENSITY-06B package/handoff, the snow-frost strategy, and
   `SC-SNOWFREEZE-001`.
2. Amend `SC-SNOWFREEZE-001` and contract-derived tests.
3. Add typed snow-density model selector and CoE-bound density update core.
4. Wire selector through typed direct snow partition and direct R4G projection
   surfaces without changing default/surface-driven compatibility behavior.
5. Add focused runtime tests for default isolation, conservation, anti-alias,
   and projection/carry.
6. Run required gates and record evidence, reviews, verification,
   line-count governance, handoff, and disposition.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

## Completion Summary

SNOWDENSITY-07 is complete. The package added the typed
`physics_bulk_density_compaction_v1` runtime density opt-in while preserving
`legacy_wepp` as the default and rollback path. The opt-in mutates only runtime
physical snow depth and bulk density; SWE, liquid forcing, routed melt,
snowpack SWE loss, and albedo/melt operands remain the CoE-boundary result.
Separate CoE boundary depth/density/settle carry is projected through direct
R4G state, downstream operands, shadow projection, runtime carry, and
publication-facing winter-column state so later CoE melt boundaries are not
contaminated by the density experiment.

No default activation, parser/runfile/CLI selector, output schema,
coefficient tuning, radiation/albedo/melt retuning, mixed/deciduous canopy
adjudication, or frost attribution change was made.
