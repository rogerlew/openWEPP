# PL17 Contract Implementation Evidence

Status: `complete`
Evidence mode: `Static`

## Canonical Contract Amendments Implemented

1. `SC-RESIDUE-001` updated for PL17 decomposition kinetics authority:
- `contract_version: 7 -> 8`
- Added explicit PL17 decomposition equation/update addendum for tracked pools (`sumrtm_seed`, `sumsrm_seed`)
- Added PL17 branch/guard map rows (`BR-RES-PL17-DECOMP-EQUATION`, `BR-RES-PL17-DECOMP-EVENT-TRANSFER`)
- Added invariants `INV-RESIDUE-017` and `INV-RESIDUE-018`
- Added PL17 decomposition test-vector obligations and required-symbol failure posture

2. `SC-PLANT-001` updated for PL17 runtime projection authority:
- `contract_version: 9 -> 10`
- Added decomposition parameter projection authority for `oratea` and `orater`
- Added runtime projection invariant `INV-PLANT-022`

3. `science-contracts/index.md` lifecycle notes updated:
- `SC-PLANT-001` note now records PL17 decomposition-parameter projection authority
- `SC-RESIDUE-001` note now records PL17 equation-driven decomposition payload update authority

## Production Behavior Authority Alignment

Static diff review confirms implementation symbols and guard posture in
`openwepp-hillslope-orchestrator` now match PL17 contract authority for:
- decomposition equation input validation (`tmax`, `tmin`, `prcp`, `Ws`, `oratea`, `orater`),
- equation-driven exponential decay updates for tracked decomposition seed pools,
- same-day annual/perennial management modifiers on updated pools,
- hard-fail typed behavior for missing/non-finite/out-of-domain required symbols.
