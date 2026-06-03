# Worker Handoff

Status: completed

Evidence mode: static+ran

Static: This handoff summarizes the next executable package scope from
HPHYS0263.

Ran: Final diagnostics root is `/tmp/hphys0263_20260603T070311Z`.

## Current State

- WB11 PMET demand seeding is migrated from pinned `evappm.for:181-388`.
- H1/H7/H39 select `evappm_pmet` and show day-1 `Ep` residual `+0.001823 mm`.
- Post-commit Claude Code review is accepted: the migrated PMET seed currently
  publishes actual/stressed `pmet.ep_m` into a downstream WB17 seam that still
  treats `wb11_et_demand` as potential demand.
- PMET `pmet.es_m` is diagnostic-only; WB17 still computes `Es` through the
  existing PT-style partition/stage path.
- Full H1..H39 semantic pass remains `0/39`.
- Full `evappm.for` closure remains `HOLD` because lines `391-454` are not
  migrated and because the PMET seam must be corrected before redistribution
  work builds on top of it.

## Recommended HPHYS0264 Objective

Diagnose, contract, correct, and validate the WB11/WB17 PMET seam before
continuing into pinned
`/workdir/wepp-forest_260430_baseline/src/evappm.for:391-454` post-ET soil
evaporation redistribution.

The package must decide contract-first whether PMET mode:

- passes through baseline `pmet.es_m`/`pmet.ep_m` as authoritative actual ET
  components and bypasses duplicate WB17 partition/SWU stress, or
- seeds a true potential/reference demand surface and leaves WB17 partition/SWU
  as the authoritative consumer.

## Required Starting Evidence

- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/package.md`
- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/full-39-suite-metrics.md`
- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/targeted-h1-h7-h39-evappm-classification.md`
- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/hphys0263_evappm_demand_migration_classification.json`
- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/review_claude_code.md`
- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/review_claude_code_disposition.md`
- `/tmp/hphys0263_20260603T070311Z/reports`

## Execution Notes

- Preserve HPHYS0263 PMET branch selection; do not reintroduce a
  Priestley-Taylor fallback for `iflget != 1`.
- Do not assume `wb11_et_demand = pmet.ep_m` is a valid long-term seam; it is
  the accepted HPHYS0263 review concern.
- Add a mid-season H1 PMET trace that compares `wb11_et_demand`, `Etp`, `Ui`,
  `Ep`, `pmet.ep_m`, `Es`, and `pmet.es_m` before any redistribution work.
- Preserve runfile-sidecar default discovery for `pmetpara.txt`.
- Preserve runtime projection of `canhgt`, `deglat`, and `elevm`.
- Start with days/cases where `Ep`, `Total-Soil`, and `SoilWaterTotal`
  residuals remain largest after PMET demand migration.
- Map legacy post-ET mutations to openWEPP soil-layer storage variables before
  editing production code.
- Require contract-first sequencing, contract-derived tests, red gate evidence,
  implementation, full H1..H39 diagnostics, and disposition.

## Suggested Exit Criteria

- Canonical contracts state the PMET-mode WB11/WB17 boundary unambiguously and
  prohibit double partitioning/double stress.
- A contract-derived test fails before seam correction and passes after it.
- H1/H7/H39 mid-season diagnostics show published `Ep`/`Es` lineage is
  consistent with the selected PMET boundary.
- Canonical `SC-EVAP-001` and `SC-WATBAL-001` explicitly describe the
  `evappm.for:391-454` storage redistribution semantics.
- H1/H7/H39 diagnostics classify whether remaining `Ep`/storage residuals move
  downstream after post-ET redistribution migration.
- Full H1..H39 semantic metrics are rerun and recorded.
- Disposition remains `HOLD` if any touched baseline-authoritative EVAPPM
  routine segment remains unported.
