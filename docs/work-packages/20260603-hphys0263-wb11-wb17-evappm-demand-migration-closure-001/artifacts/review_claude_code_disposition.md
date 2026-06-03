# Claude Code Review Disposition

Status: completed

Evidence mode: static+ran

Static: Disposition of
`docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/review_claude_code.md`
against the HPHYS0263 source tree after commit `72e4bd7`.

Ran: `git diff --check` passed after disposition updates.

## Summary

The review is accepted. HPHYS0263 faithfully migrated the pinned EVAPPM
equations it touched, but it exposed a WB11/WB17 seam defect: the migrated PMET
branch publishes `pmet.ep_m` as `wb11_et_demand`, while the downstream WB17
kernel still treats `wb11_et_demand` as a potential ET surface and applies the
existing partition/root-uptake stress path. That makes the current PMET path an
internally mixed PMET/PT lineage and blocks further EVAPPM closure claims.

## Finding 1: Actual Transpiration Seeded Into Demand Seam

Disposition: accepted, high priority.

Static evidence:

- `compute_evappm_wb11_et_demand` returns `Wb11EtDemandSeed { demand_m:
  ep_m, .. }`.
- The WB17 ET phase consumes `wb11_et_demand` as `et_demand` and computes
  PT-style soil/transpiration partition surfaces from it.
- The WB17 root-uptake phase consumes `Etp` and applies SWU root uptake/stress
  to derive final `Ep`.

Implication: PMET-mode HPHYS0263 can double-apply crop/stress reductions if
`pmet.ep_m` is already actual/stressed and then feeds a second partition/SWU
path.

Action: Do not build HPHYS0264 post-ET redistribution on top of this seam until
the WB11/WB17 PMET boundary contract is corrected.

## Finding 2: PMET Soil Evaporation Is Diagnostic Only

Disposition: accepted, high priority.

Static evidence:

- `compute_evappm_wb11_et_demand` computes `pmet.es_m`.
- `pmet.es_m` is published only as trace diagnostics.
- WB17 still computes `Es` from the existing PT-style partition/stage path.

Implication: PMET-mode `Ep` and `Es` currently do not share one
baseline-authoritative lineage.

Action: The continuation package must decide contract-first whether PMET mode
passes through `pmet.es_m`/`pmet.ep_m` as authoritative components or instead
seeds a true potential/reference surface and lets WB17 own partitioning.

## Finding 3: PMET Stress Reads Known Storage-Drained State

Disposition: accepted as inherited/binding continuation risk.

Static evidence:

- PMET `etks` depends on root-zone water availability from
  `wb18_perc_theta`.
- HPHYS0249-0263 already show seasonal storage depletion remains unresolved.

Implication: even a correct PMET seam will inherit seasonal storage defects
until WB18/WB19/storage availability is corrected.

Action: Continue treating hillslope storage availability as the binding
seasonal residual owner.

## Finding 4: Naming Is Misleading

Disposition: accepted, low priority but important for contracts.

Static evidence:

- `Wb11EtDemandSeed::demand_m` and `wb11_et_demand` can currently hold a
  stressed PMET actual.

Action: The continuation package should either rename branch-specific surfaces
or make `wb11_et_demand` semantically true by changing PMET wiring.

## Updated Continuation Rule

HPHYS0264 should not start with `evappm.for:391-454` storage redistribution.
It should first resolve the WB11/WB17 PMET seam:

1. Amend canonical contracts to state the authoritative PMET-mode boundary:
   pass-through actual `Es`/`Ep`, or true demand/reference input plus WB17
   partition.
2. Add contract-derived tests proving PMET mode does not double-partition or
   double-stress transpiration.
3. Add H1/H7/H39 mid-season trace evidence comparing published `Ep`/`Es`
   against `pmet.ep_m`/`pmet.es_m` and WB17 `Etp`/`Ui`.
4. Only after the seam is resolved, proceed to pinned `evappm.for:391-454`
   post-ET redistribution and storage mutation.
