# Fixture Cohort Plan

Status: EXECUTED-COMPLETE
Evidence mode: Static + Ran.

## Decision Cohort

The production decision was priced on the real selected cohort:

| Member | Role | Climate | Trace rows per rung | Materialization |
|--------|------|---------|--------------------:|-----------------|
| `mn_corn_h4` | Real row-crop/agriculture | `p4.cli` | 2557 | `20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json` |
| `n_idaho_forest_h1` | Real forest | `p1.cli` | 1461 | same |
| `wa_cascades_forest_h1` | Real wet forest/runtime stress | `p1.cli` | 10960 for completed rungs | same |

Synthetic stress evidence was reported separately:

| Member | Role | Climate | Trace rows per rung | Constraint |
|--------|------|---------|--------------------:|------------|
| `h2637` | Synthetic short-OFE stress only | `p2637.cli` | 13889 | Not fleet-general proof |

The ladder harness copied each source run directory into
`artifacts/mesh-ladder-runs/<member>/<rung>/run_dir/` and rewrote outputs to
`run_dir/output/`. The source materialization directories were not reused as
output targets.

## Outcome

The cohort was sufficient to reject production target-`dx` promotion:
- `mn_corn_h4` and `n_idaho_forest_h1` passed reference/candidate tolerances.
- `wa_cascades_forest_h1` failed the active closure guard for `dx2p5` and
  `dx1p25` at day 1122, so no adequate fine reference exists for that member.
- `wa_cascades_forest_h1` `dx10`/`dx5` completed but produced non-promotable
  routed outlet/storage magnitudes.
