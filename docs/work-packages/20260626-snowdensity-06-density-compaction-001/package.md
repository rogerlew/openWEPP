# SNOWDENSITY-06 Density Compaction

Status: complete.

Package type: contract-first offline density-physics implementation and
adjudication.

Primary authority: `SC-SNOWFREEZE-001` v84, especially
`INV-SNOWFREEZE-050`, `INV-SNOWFREEZE-051`, `INV-SNOWFREEZE-056`,
`INV-SNOWFREEZE-057`, and `INV-SNOWFREEZE-058`.

Closure target: COMPLETE-06-DENSITY-COMPACTION-NO-RUNTIME-PROMOTION.

Objective: execute the first post-melt-modernization density package by
ratifying and testing an opt-in, density-only Anderson-1976/SNOBAL
compaction candidate under fixed melt boundaries. The package must not revive
the SNOWDENSITY-04 slow-melt compensation route.

## Scope

- Amend `SC-SNOWFREEZE-001` for SNOWDENSITY-06 density-only compaction
  authority.
- Keep `legacy_coe` as default/rollback and keep `coe_shortwave_albedo_v1` as
  opt-in diagnostic context only.
- Keep mixed/deciduous low-canopy melt work deferred to SNOWDENSITY-05H.
- Add a named `physics_bulk` candidate that changes density/compaction
  constants only and preserves baseline candidate melt constants.
- Expose SNOBAL-lineage PTM/POC compaction constants as named report fields
  instead of hidden literals.
- Extend adjudication evidence with density/densification cell summaries so the
  package cannot pass via melt timing compensation.

## Non-Scope

- No default activation.
- No production parser, runfile, CLI activation selector, or output-schema
  change.
- No mixed/deciduous or low-canopy melt adjudication.
- No melt coefficient, albedo constant, canopy, shared-radiation, or
  radiation-bridge retuning.
- No site-specific constants, SSD fitting, SNOTEL-trained defaults, or
  residual-minimized parameter selection.
- No frost attribution or non-SNOTEL snow-control verdict change.

## Acceptance Gates

- `SC-SNOWFREEZE-001` records SNOWDENSITY-06 density-only compaction authority,
  named constants, no-site-tuning, fixed-melt boundaries, and no default
  activation.
- Focused tests prove the new variant exists, preserves baseline melt constants,
  changes density constants only, and leaves `physics_bulk` confined to
  snowbench/diagnostic surfaces.
- SNOTEL adjudication emits five-site profiles and a density-cell summary for
  the new variant.
- Closure is one of:
  - `COMPLETE-06-DENSITY-COMPACTION`: the new density-only variant improves the
    density/densification robust-cell profile against legacy/as-built without
    site tuning and without melt changes; or
  - `COMPLETE-06-NON-PROMOTION`: finite evidence shows the density-only variant
    does not clear that profile gate, with a concrete follow-on.
- Required gates pass:
  `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`,
  `cargo deny check`,
  focused contract tests,
  source scans for `qwet|frzftp`,
  and `git diff --check`.

## Phase Plan

1. Read SNOWDENSITY-05G, strategy §7, `SC-SNOWFREEZE-001`, Anderson-1976
   annotation, and PySnobal `_time_compact.c` / `_h2o_compact.c` lineage.
2. Scaffold package prompts and artifacts.
3. Amend the contract and contract-derived tests.
4. Add the density-only `physics_bulk` variant and named compaction constants.
5. Extend adjudication output with density-cell summaries and run the five-site
   SNOTEL profile.
6. Record implementation evidence, reviews, verification, line-count
   governance, and final disposition.

## Subagent Authorization

Subagent authorization: this package does not explicitly authorize spawning or
delegating to subagents. Reviews and verification are local unless a later
operator request adds explicit delegation authorization.

## Completion Summary

Closure: `COMPLETE-06-DENSITY-COMPACTION-NO-RUNTIME-PROMOTION`.

Amended `SC-SNOWFREEZE-001` to v84 with `INV-SNOWFREEZE-058`,
`OBL-SNOWFREEZE-P-033`, and the SNOWDENSITY-06 Density Compaction Addendum.
Added offline `physics_bulk` variant `density_compaction_v1`, exposed named
SNOBAL-lineage PTM/POC/liquid-water compaction constants in
`physics_bulk_summary.json`, and preserved baseline candidate melt constants,
albedo, canopy, radiation, production defaults, and rollback.

Five-site SNOTEL adjudication produced finite evidence. The density-only
variant improves the density/densification robust-cell profile against
legacy/as-built (`fail 9 -> 7`, ordinal score `16 -> 22`) without site tuning
or melt changes. It does not clear whole-rubric promotion (`robust fail 9 ->
18`, ordinal score `84 -> 46`), so this package does not authorize runtime
activation, default activation, parser/runfile/CLI activation selectors, output
schemas, or frost attribution.

Next route: an offline CoE-bound density replay should feed the compaction
state update from fixed CoE melt/liquid/SWE-loss operands rather than the old
physics-bulk degree-day melt surrogate before any SNOWDENSITY-07 runtime opt-in.

## Closeout Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/implementation-evidence.md`
- `artifacts/snotel-adjudication.{json,md}`
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
