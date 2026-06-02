# HPHYS0246 Contract Implementation Evidence

Status: completed
Evidence mode: Static

## Contract Amendments
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
  - Bumped to `contract_version: 21`.
  - Added baseline provenance for `soilw(i) = st(i) + thetdr(i)*(dg(i)-frozen(i))`
    and `watcon = Σsoilw(i)` from pinned legacy WATBAL sources.
  - Added WB18 input obligations for `thetdr_####`, `dg_####`, and optional
    `wb18_perc_frozen_depth_####`.
  - Added `INV-PERC-013`, requiring WB18 aggregate `wb11_soil_water` publication
    from baseline `soilw` semantics rather than `Σtheta`.
  - Added typed guard/test-vector obligations for missing, non-finite, or
    domain-invalid residual-storage symbols.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - Bumped to `contract_version: 72`.
  - Added producer obligation `OBL-WATBAL-P-016` tying WB18 aggregate storage
    publication to `SC-PERC-001#INV-PERC-013`.
  - Clarified that WB13 reflects downstream storage and must not compensate for
    a WB18 `Σtheta`-only publication.
- `docs/specifications/science-contracts/index.md`
  - Updated SC-PERC and SC-WATBAL index rows for HPHYS0246.

## Baseline Provenance
- `/workdir/wepp-forest_260430_baseline/src/purk.for:152` to `:194`: bottom-up
  percolation mutates `st` and bottom-layer seepage.
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:960` to `:966`: daily
  recomputes `soilw(i)` and `watcon` from `st`, `thetdr`, `dg`, and `frozen`.
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1018` to `:1025`:
  hourly recomputes the same aggregate storage lineage.
- Pinned baseline commit:
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
