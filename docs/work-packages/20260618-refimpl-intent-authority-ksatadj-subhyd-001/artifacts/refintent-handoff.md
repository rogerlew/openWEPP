# REFINTENT Handoff

Package:
`20260618-refimpl-intent-authority-ksatadj-subhyd-001`

## Handoff Verdict

`OPENWEPP-DEFECTIVE`.

The previous STAGE2-LATQCC `CONTRACT-GAP` is no longer an authority gap after
ADR-0024 and `SC-SUBHYD-001#INV-SUBHYD-032`. The remaining issue is an
implementation divergence in the `ksatadj` saturation-fraction operand lineage.

## First Actionable Item

Close defect `REFINTENT001-KSATADJ-SATFRAC`.

Required correction:

- Update the `wb14` `ksatadj` effective-conductivity path so `sat_frac` follows
  `SC-SUBHYD-001#INV-SUBHYD-032`:
  `avsat / (avpor * avcpm)`, with top-two tillage-layer total-water formation,
  source-intent caps, and branch-specific guards.
- Preserve the existing matched 9001, 9002+, and 9003 branch formulas unless
  tests prove a separate formula defect.
- Add a non-aliased unit test where storage-over-upper-limit differs from the
  source-intent saturation fraction.

Likely files:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage/02_ksat_adjustment.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
- focused hydrology/runtime tests that publish or consume `por_####`,
  `cpm_####`, `thetfc_####`, `thetdr_####`, `dg_####`, `wb18_perc_theta_####`,
  and `wb18_perc_ul_####`.

## FARPOINT01 State

Static:

- FARPOINT01 is not closed by this package.
- The flag is reclassified from absolute-magnitude authority gap to
  source-intent implementation defect pending `REFINTENT001-KSATADJ-SATFRAC`.

## Future ADR-0024 Applications

Static:

- `qdry` and `ksflag` remain future source-intent applications.
- This package does not amend their contracts or adjudicate their
  implementations.
