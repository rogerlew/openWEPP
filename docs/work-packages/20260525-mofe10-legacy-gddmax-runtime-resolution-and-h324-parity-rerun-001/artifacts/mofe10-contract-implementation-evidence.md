# MOFE10 Contract Implementation Evidence

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Updated canonical authority in `SC-PLANT-001` (`contract_version 14`) for
  legacy `gddmax<=0` sentinel resolution:
  - Added authority anchors:
    - `/workdir/wepp-forest_260430_baseline/src/yldopt.for:121-200,271-277`
    - `/workdir/wepp-forest_260430_baseline/src/gdmax.for:1-130`
  - Added required monthly climate vectors for PL16 resolution:
    - `obmaxt[1..12]`, `obmint[1..12]`
  - Added annual/perennial branch semantics:
    - annual summer: `gdmax(jdplt,jdharv)`
    - annual winter: `gdmax(jdplt,365)+gdmax(1,jdharv)`
    - perennial: `gdmax(1,365)`
  - Added invariant `INV-PLANT-024` and typed fail-closed obligations when
    monthly vectors are missing or resolved `gddmax` is invalid.

Ran:
- Contract amendment was completed before production runtime edits.
