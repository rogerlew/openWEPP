# Contract Implementation Evidence

Status: completed

Evidence mode: Static

Static:

- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md` is amended
  to version `18` with `REF-EVAP-LEGACY-PMET-SEAM`, branch
  `BR-EVAP-WB17-PMET-COMPONENT-SEAM`, and `INV-EVAP-022`.
- `INV-EVAP-022` requires EVAPPM PMET branch WB17 execution to consume
  `pmet.es_m` and `pmet.ep_m` directly, publish `Etp = pmet.ep_m` before SWU,
  prohibit Priestley-Taylor/LAI re-partitioning of PMET `ep`, reject material
  negative `pmet.es_m`, and keep `Er` non-negative.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` is amended
  to version `90` with `INV-WATBAL-050`, requiring WB13/WAT closure evidence to
  prove the same PMET seam lineage and final post-SWU `Ep`.
- Provenance cites pinned baseline
  `/workdir/wepp-forest_260430_baseline/src/evappm.for`, `swu.for`, and
  `watbal_hourly.for` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
