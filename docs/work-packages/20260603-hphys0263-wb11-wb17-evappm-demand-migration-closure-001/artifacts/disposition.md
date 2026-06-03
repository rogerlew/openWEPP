# HPHYS0263 Disposition

Status: hold

Evidence mode: static+ran

Static: HPHYS0263 implemented the baseline-authoritative WB11 PMET demand seed
subset from pinned `evappm.for`, updated canonical contracts, and recorded the
remaining unmigrated routine scope.

Ran: Focused tests, full workspace gates, `cargo deny check`, and the full
H1..H39 hillslope diagnostics completed; results are recorded in
`gate-results.md`.

## Decision

`HOLD` for full `evappm.for` migration closure.

## Completed Scope

- Contract authority now requires migrated EVAPPM PMET demand seeding when
  `pmetpara.mode.iflget != 1`.
- WB11 ET-demand seeding now selects the `evappm_pmet` branch for H1/H7/H39
  PMET-mode runs.
- PMET demand intermediates are traceable through the HPHYS0245/0263 branch
  trace.
- Default runfile-sidecar discovery now finds `pmetpara.txt` in run
  directories.
- Runtime projection now publishes `canhgt`, `deglat`, and `elevm` required by
  the migrated demand path.

## Evidence Summary

- H1/H7/H39 day-1 baseline `Ep`: `0.150000 mm`.
- H1/H7/H39 day-1 candidate `Ep`: `0.151823 mm`.
- H1/H7/H39 day-1 `Ep` residual: `+0.001823 mm`.
- HPHYS0262 comparison point before migration: `+0.235294 mm`.
- Final H1/H7/H39 branch classification:
  `EVAPPM_MIGRATED_BRANCH_OBSERVED`.
- Full H1..H39 semantic pass remains `0/39`.

## Hold Rationale

- Pinned `/workdir/wepp-forest_260430_baseline/src/evappm.for:391-454`
  remains unported.
- That remaining routine segment performs post-ET soil evaporation
  redistribution and storage mutation after demand has been computed.
- Post-commit Claude Code review found an accepted high-priority seam issue:
  HPHYS0263 publishes PMET `pmet.ep_m` through `wb11_et_demand`, but downstream
  WB17 still treats that surface as potential demand and applies PT-style
  partition plus SWU stress.
- PMET `pmet.es_m` is currently diagnostic-only; WB17 still computes `Es` from
  the existing PT-style lineage, so PMET `Ep`/`Es` are not end-to-end
  lineage-consistent.
- Root governance prohibits claiming full process closure when known
  baseline-authoritative process physics remains in scope but unported.
- Independent dual sub-agent review/verification is not claimed because the
  current HPHYS0263 user instruction did not explicitly request sub-agent
  dispatch; local review and verification artifacts were completed instead.

## Continuation Recommendation

Open HPHYS0264 as a contract-first work package focused first on the WB11/WB17
PMET seam contract: decide and implement whether PMET mode passes through
baseline `pmet.es_m`/`pmet.ep_m` as authoritative components or seeds a true
potential/reference demand surface consumed by WB17 partition/SWU. Only after
that seam is resolved should HPHYS0264 continue into baseline-authoritative
`evappm.for:391-454` post-ET soil evaporation redistribution and storage
effects. Use `/tmp/hphys0263_20260603T070311Z` and the package artifacts as the
starting evidence set.
