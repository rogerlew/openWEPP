# Contract Implementation Evidence

Status: completed
Evidence mode: static

Static:
- Updated `SC-EVAP-001` front matter to contract version `23` and `last_reviewed: 2026-06-04`.
- Split process-rate `Esb` from final WAT publication `Es` so the canonical registry symbol `Es` declares registry unit `mm`.
- Added explicit `Variables and Units` rows for `Ep`, `Es`, and `Er` as WAT daily publication depths in `mm`.
- Added Symbol Alias Map rows covering registered aliases:
  - `hillslope_wat.Ep`
  - `hillslope_wat.Ep:mm`
  - `hillslope_wat.Es`
  - `hillslope_wat.Es:mm`
  - `hillslope_wat.Er`
  - `hillslope_wat.Er:mm`
- Updated existing runtime alias rows for `Er`, `Es`, and `Ep` to state conversion from internal daily `m` fluxes to registry `mm` WAT publication depths.
- Added revision-history entry `23` for HPHYS0282.
