# SNOWDENSITY-01 Evidence Reconciliation

Status: complete.

Package type: characterization / evidence reconciliation.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: consolidate the SNOWFROST-FIDELITY-D/E/F/H/I0 evidence that led to
the snow-density strategy, pin the openWEPP-vs-pinned-legacy density deltas from
the H SNOTEL three-way comparison, classify rubric cells into snow-physics vs
forcing-limited or mixed drivers, and review the Shen 2011/2012 thesis plus the
pinned `snowd.for` Eq. 3.7.5 divergence enough to route the next work package.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`docs/planning/snow-frost-fidelity-strategy.md`,
`docs/backlog/20260605-snow-code-deferred-science-review.md`,
SNOWFROST-FIDELITY-D/E/F/H/I0, and the local copyrighted thesis
`references/copyrighted/D_Shen_020312.pdf`.

Subagent authorization: none. Execute locally and record review/disposition in
package artifacts.

## Scope

In scope:

- Evidence-only reconciliation of already executed snow/frost fidelity packages.
- Static review of pinned legacy `/home/workdir/wepp-forest_260430_baseline/src/snowd.for`
  snow-density comments, settlement threshold handling, and Eq. 3.7.5 marker.
- Static summary of `references/copyrighted/D_Shen_020312.pdf` limited to
  thesis scope and snow-drift implications.
- JSON/Markdown artifact that pins H's openWEPP-vs-legacy density deltas.
- Rubric-cell classification that decides what belongs in SNOWDENSITY-02.
- Catalog/planning updates so the completed evidence package is discoverable.

Out of scope:

- No production snow/frost physics, constants, runtime options, `Qwet`, `frzftp`,
  compatibility deletion, direct default activation, or SNOTEL data mutation.
- No `SC-SNOWFREEZE-001` amendment. This package prepares the next contract
  package; it does not create new science authority.
- No PySnobal hardening or rerun.
- No claim that observation disagreement alone is `OPENWEPP-DEFECTIVE`.
- No per-site parameter selection or SSD tuning.

## Acceptance Criteria

- Required reading is recorded.
- H SNOTEL density deltas are pinned in an artifact with exact per-site values.
- The package records whether legacy and openWEPP are materially different for
  the as-built density residual.
- Rubric cells are classified into: actionable snow-physics candidate,
  forcing-limited, mixed/conditional, or unavailable/future-data.
- Shen 2011/2012 is classified without overclaiming it as densification
  authority.
- Pinned `snowd.for` Eq. 3.7.5 and related comments are summarized with line
  evidence.
- Review, verification, line-count governance, and worker handoff artifacts are
  complete.
- `git diff --check` passes.

## HOLD Boundaries

Close as `HOLD` only if the H three-way JSON is unavailable, the Shen thesis
cannot be text-extracted or summarized, the pinned baseline source is missing, or
the evidence contradicts the strategy route enough that SNOWDENSITY-02 cannot be
defined without new model runs.

## Execution Plan

1. Scaffold this package and active kickoff prompt.
2. Read required authority and evidence.
3. Extract H density/depth/PySnobal metrics from
   `target/snowfrost_fidelity_h/three_way_comparison.json`.
4. Review `snowd.for` and the Shen thesis for snow-density relevance.
5. Write evidence reconciliation, density delta, rubric classification, and
   archaeology artifacts.
6. Update package catalog and planning status.
7. Record review, verification, line-count governance, and handoff.
