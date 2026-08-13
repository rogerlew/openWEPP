# Reference Lineage

Static:

- Primary selected reference: CLM5 technical note, “Vegetation Phenology and
  Turnover,” release-clm5.0, sections 2.20.1 and 2.20.3. The retained reviewed
  PDF bytes are `REF-VEGETATION-027`, SHA-256
  `9ca0f0e5b7aff712a0ef7f5198f111c4b250cac4417a4f000e36c6c143f2e363`:
  <https://escomp.github.io/CTSM/release-clm5.0/tech_note/Vegetation_Phenology_Turnover/CLM50_Tech_Note_Vegetation_Phenology_Turnover.html>.
- Supporting allocation ownership: CLM5 technical note, “Carbon and Nitrogen
  Allocation,” release-clm5.0, sections 2.19.4--2.19.5:
  <https://escomp.github.io/CTSM/release-clm5.0/tech_note/CN_Allocation/CLM50_Tech_Note_CN_Allocation.html>.
- Architectural corroboration only: Tague and Band (2004), RHESSys,
  <https://doi.org/10.1175/1087-3562%282004%298%3C1%3ARRHSSO%3E2.0.CO%3B2>.

The exact six-tissue `f_stor_xfer=0.5` preparation and declining onset
deployment come from CLM5. RHESSys is not used to select a coefficient or
equation. V7 prospectively selects the CLM rule because openWEPP already owns
the corresponding onset event and has no canonical calendar-year rollover
owner.
