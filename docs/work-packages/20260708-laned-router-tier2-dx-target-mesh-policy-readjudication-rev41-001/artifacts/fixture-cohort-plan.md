# Fixture Cohort Plan

Status: `EXECUTED-COMPLETE`
Evidence mode: Static.

The package reuses the selected-cohort materialization from
`docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json`.

Real selected-cohort members:

| Member | Role |
|--------|------|
| `mn_corn_h4` | row-crop/agriculture real cohort |
| `n_idaho_forest_h1` | forest real cohort |
| `wa_cascades_forest_h1` | wet forest/runtime stress real cohort |

Synthetic stress member:

| Member | Role |
|--------|------|
| `h2637` | short-OFE synthetic stress only; not fleet-general proof |

Rungs:

| Rung | Meaning |
|------|---------|
| `baseline_fixed10` | current production default, judged as a candidate |
| `dx20` | diagnostic target `20 m` |
| `dx10` | diagnostic target `10 m` |
| `dx5` | diagnostic target `5 m` |
| `dx2p5` | candidate fine reference |
| `dx1p25` | one further halving for reference adequacy |
