# Fixture Cohort Plan

Status: QUEUED
Evidence mode: Static scaffold.

## Primary Decision Cohort

The production mesh-policy decision must be priced on the real active-plain
selected cohort, not on H2637 alone.

| Member | Role | Climate | Prior routed-day shape | Materialization source |
|--------|------|---------|------------------------|------------------------|
| `mn_corn_h4` | Real row-crop/agriculture | `p4.cli` | 2557 days seen, 209 routed | `20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json` |
| `n_idaho_forest_h1` | Real forest | `p1.cli` | 1461 days seen, 185 routed | same |
| `wa_cascades_forest_h1` | Real wet forest/runtime stress | `p1.cli` | 2192 days seen, 1381 routed | same |

## Synthetic Stress Case

| Member | Role | Climate | Prior routed-day shape | Materialization source |
|--------|------|---------|------------------------|------------------------|
| `h2637` | Synthetic short-OFE stress only | `p2637.cli` | 731 days seen, 610 routed | selected-cohort and ADR-0037 plain identity artifacts |

## Execution Requirements

- Materialize package-local copies or stable references for each selected member
  before timing/comparator evidence.
- Record input hashes, runfile paths, output directories, release binary hash,
  and environment for every timing/comparator run.
- Report real-cohort aggregate and H2637 stress evidence separately.
- Do not treat H2637 as fleet-general proof.
