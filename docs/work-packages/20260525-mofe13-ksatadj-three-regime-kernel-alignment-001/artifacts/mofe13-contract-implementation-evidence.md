# MOFE13 Contract Implementation Evidence

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Canonical authority amended before production edits:
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
    - `contract_version: 0.1.6`
    - added disturbed-policy runtime seam publication authority for `solwpv`,
      `ksatadj`, `ksatfac`, `ksatrec`, `lkeff`.
  - `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
    - `contract_version: 19`
    - added WB14 `Ke` regime selection authority for `solwpv` 9001/9002/9003.
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
    - `contract_version: 37`
    - synchronized WB14 coupling addendum and guard expectations.
- Legacy baseline provenance captured in contract text:
  - `/workdir/wepp-forest_260430_baseline/src/input.for`
  - `/workdir/wepp-forest_260430_baseline/src/infpar.for`
  - `/workdir/wepp-forest_260430_baseline/src/cvgpar.inc`

Ran:
- Contract updates were completed before runtime/kernel implementation edits.
