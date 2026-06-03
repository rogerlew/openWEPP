# PMET Seam Diagnosis

Status: completed

Evidence mode: Static + Ran

Static:

- Pinned baseline `evappm.for:430-459` computes PMET `es`, `ep`, and
  `et = es + ep` directly.
- Pinned baseline `evappm.for:460-523` performs post-ET redistribution; when
  `es - resint < 0`, it returns the residue deficit to top-layer storage rather
  than publishing a material negative `es`.
- Pinned baseline `watbal_hourly.for:978-981` calls `swu` when `ep > 0` and
  `rtd > 0`.
- Pinned baseline `swu.for:122-191` consumes positive `ep` as demand and sets
  final `ep` from root uptake `Σu(k)`.

Ran:

- H1/H7/H39 traces from `/tmp/hphys0264_20260603T083941Z` show
  `wb11_et_seed_branch = evappm_pmet`, `pmet_iflget = 2.0`, selected
  `kcb = 0.95`, selected `rawp = 0.8`, selected line index `1`, and no fallback
  first-row lookup.
- Day-1 H1/H7/H39 PMET values show `wb11_et_demand_m = pmet_ep_m =
  etp_m = 0.00015182345787105985` and `pmet_es_m =
  0.0000007289343936380457`.

Conclusion:

- The WB11/WB17 seam correction is branch-aware and component-preserving.
- Remaining H1..H39 residuals are no longer attributable to the old PMET
  double-partition seam, but full EVAPPM/storage parity is not closed.
