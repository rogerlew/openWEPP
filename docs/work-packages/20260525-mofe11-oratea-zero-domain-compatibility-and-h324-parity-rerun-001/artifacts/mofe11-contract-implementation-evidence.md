# MOFE11 Contract Implementation Evidence

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Updated canonical authority for decomposition-rate domain semantics:
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
    - `contract_version: 15`
    - Added `REF-PLANT-LEGACY-ORATE-DOMAIN`
    - Updated `INV-PLANT-022` from positive-only to non-negative (`>=0`)
    - Added explicit zero-value no-decay semantics and negative-value hard-fail
      posture.
  - `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
    - `contract_version: 10`
    - Added `REF-RESIDUE-LEGACY-ORATE-DOMAIN`
    - Updated `INV-RESIDUE-018` to non-negative (`>=0`)
    - Added explicit zero-value no-decay semantics and negative-value invalid
      state language.
- Legacy source references captured in contract authority:
  - `/workdir/wepp-forest_260430_baseline/src/infile.for`
  - `/workdir/wepp-forest_260430_baseline/src/decomp.for`

Ran:
- Contract amendments were completed before production runtime guard edits.
