# Contract Implementation Evidence

Status: completed

Evidence mode: static

Static:

- `SC-EVAP-001` was amended to `contract_version: 15`.
- Added PMET demand-seeding authority for legacy `watbal_hourly.for:557-559`
  branch selection and `evappm.for:181-297` Penman-Monteith lineage.
- Added canonical variables and trace aliases for `iflget`, selected `kcb`,
  selected `rawp`, selected `pmetpara` line index, fallback status, WB11 ET
  demand, and actual WB11 ET seed branch.
- Added `BR-EVAP-WB11-PMET-SEED`, `INV-EVAP-020`, `OBL-EVAP-P-008`, and
  `GAP-EVAP-008`.
- `SC-WATBAL-001` was amended to `contract_version: 88`.
- Added `INV-WATBAL-048` requiring WB17 trace output to preserve PMET sidecar
  lineage, WB11 demand, and actual seed-branch observability.
- No package-local artifact replaces canonical `SC-*` authority.
