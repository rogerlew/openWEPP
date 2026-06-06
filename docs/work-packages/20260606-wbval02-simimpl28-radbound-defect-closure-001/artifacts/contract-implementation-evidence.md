# Contract Implementation Evidence

Status: complete

Evidence mode: `Static`

Static:

- Amended `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
  to version `23`.
- Authority:
  - `INV-CLIMATE-006`: generated `RA` must satisfy `RA <= RAmax`.
  - `INV-CLIMATE-013`: active SIMIMPL28 radiation must preserve `radly` in
    `Ly d^-1`, convert exactly once to `radmj`, and publish hourly `hradmj` in
    `MJ m^-2 h^-1`.
  - `REF-CLIMATE-WF-RADLY-RADMJ`: pinned-baseline `sunmap`/`radcur`/`hr_tmp`
    lineage, including `sunmap` horizontal daily potential `r3`/`rpoth`.
- Amendment:
  - Active SIMIMPL28 synthesis must fail closed at source symbol `radly` when
    finite daily radiation exceeds baseline `sunmap` horizontal daily potential
    for the same date and latitude.
  - The existing hourly HPHYS0277 guard remains in force for source-valid daily
    radiation that later violates the hourly bound.
  - The contract continues to prohibit clipping, capping, renormalization, or
    downstream compensation for physically impossible daily/hourly radiation.

Conclusion:

- WBVAL02 is authorized to reclassify the six observed failures as invalid
  upstream daily radiation with typed `radly` evidence.
