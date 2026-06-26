# SNOWDENSITY-08 Snow/Frost Gate Rerun

Status: complete.

Package type: contract-first adjudication / gate rerun.

Primary authority: `SC-SNOWFREEZE-001` v87, especially
`INV-SNOWFREEZE-048`, `INV-SNOWFREEZE-050`, `INV-SNOWFREEZE-059`,
`INV-SNOWFREEZE-060`, `INV-SNOWFREEZE-061`, and `OBL-SNOWFREEZE-P-036`.

Closure target: one of
`COMPLETE-08-SNOTEL-CLEARED-FROST-ATTRIBUTION-BLOCKED`,
`COMPLETE-08-SNOW-FROST-GATE-CLEARED`,
or `HOLD-08-<BLOCKER>`.

Objective: rerun the snow/frost gates after SNOWDENSITY-07 made
`physics_bulk_density_compaction_v1` available as a typed runtime opt-in, while
preserving `legacy_wepp` as the default and refusing to resume frost attribution
unless the non-SNOTEL frost-site WAT surface is a coupled opt-in run.

## Scope

- Amend `SC-SNOWFREEZE-001` before evidence production.
- Rerun the SNOTEL density gate for the accepted
  `physics_bulk_density_compaction_v1` lineage through the CoE-bound density
  replay.
- Rerun the non-SNOTEL frost-site rubric on the current direct production WAT
  surface.
- Publish one compact decision artifact with:
  - SNOTEL robust/density-cell deltas;
  - non-SNOTEL snow-control status counts;
  - whether a coupled opt-in WAT/publication path exists;
  - `frost_attribution_authorized`;
  - the next blocker.
- Preserve CoE boundary anti-alias proof and no-site-constants evidence.

## Non-Scope

- No default activation.
- No parser/runfile/user CLI density selector.
- No production output-schema change.
- No coefficient, canopy, radiation, albedo, melt, density, or frost-physics
  retuning.
- No WAT rewriting or offline snow-only substitution as a coupled frost result.
- No mixed/deciduous low-canopy melt adjudication.

## Acceptance Gates

- `SC-SNOWFREEZE-001` records SNOWDENSITY-08 authority before gate evidence.
- SNOTEL rerun publishes a same-lineage CoE-bound replay with daily SWE identity
  and no site constants.
- Non-SNOTEL rerun publishes current direct WAT snow-control/frost rubric
  status.
- The decision report refuses frost attribution unless the non-SNOTEL WAT path
  is coupled through the opt-in runtime density selector.
- Focused SNOWDENSITY-08 integration guards pass.
- Required gates pass:
  `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`,
  `cargo deny check`,
  source scan for `qwet|frzftp`,
  and `git diff --check`.

## Phase Plan

1. Read SNOWDENSITY-06B, SNOWDENSITY-07, I0 non-SNOTEL baseline, the strategy
   document, and `SC-SNOWFREEZE-001`.
2. Amend the contract and add package/test authority.
3. Add a compact gate-rerun aggregator under `tools/snowfreeze_observed/`.
4. Build diagnostics, rerun SNOTEL and non-SNOTEL gate evidence, and copy the
   compact decision outputs to package artifacts.
5. Record reviews, disposition, verification, gate results, line-count
   governance, handoff, and final package status.

## Subagent Authorization

Subagent authorization: none. Execute locally and record review/disposition in
package artifacts unless an operator explicitly requests delegation.

## Completion Summary

SNOWDENSITY-08 is complete as
`COMPLETE-08-SNOTEL-CLEARED-FROST-ATTRIBUTION-BLOCKED`.

The SNOTEL gate still clears for the accepted density lineage:
`coe_bound_density_compaction_v1_coe_shortwave_albedo_v1` remains the best
candidate with robust failures `9 -> 5`, robust score `84 -> 110`, density
failures `9 -> 5`, density score `16 -> 41`, and CoE daily SWE identity
residual about `4.44e-16 m`.

The non-SNOTEL frost-site rerun remains blocked for frost attribution:
three sites fail snow control on the current direct-production WAT path and
two sites have modeled snow depth but no paired observed snow rows. More
importantly, the current WAT path is still the default `legacy_wepp` density
path. SNOWDENSITY-07 intentionally exposed `physics_bulk_density_compaction_v1`
only to typed callers and did not add a parser/runfile/CLI selector, so no
authorized coupled non-SNOTEL opt-in WAT/publication run exists yet.

No default activation, parser/runfile/CLI selector, output schema,
coefficient/canopy/radiation/albedo/melt/density/frost tuning, or production
physics change was made.
