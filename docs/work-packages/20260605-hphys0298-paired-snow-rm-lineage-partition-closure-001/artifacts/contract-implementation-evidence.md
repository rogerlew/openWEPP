# Contract Implementation Evidence

Status: complete

Evidence mode: static

Static:

- Updated `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` with `INV-SNOWFREEZE-029`, guard mapping, `OBL-SNOWFREEZE-P-017`, revision-history entry `30`, and Claude review disposition entry `31`.
- Updated `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` with `INV-RUNOFFPART-026`, guard mapping, `OBL-RUNOFFPART-P-012`, and revision-history entry `38`.
- Updated `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` with `INV-WATBAL-073`, guard mapping, `OBL-WATBAL-P-022`, revision-history entry `119`, and Claude review disposition entry `120`.
- Amendments make `/workdir/wepp-forest_260430_baseline` the required paired baseline source, require observe identity before trace use, enumerate all nine H1/H7/H39 windows, and prohibit WB13/WB17/WB18/WB19 downstream compensation for upstream snow/`RM` residuals.
- Original Claude review disposition treated the HPHYS0298 `hourly-forcing`
  verdict as porting-fidelity authority, but
  `artifacts/review_claude_hrsnow_unit_artifact.md` supersedes that
  conclusion: the `hrsnow` comparison paired baseline snowfall depth with
  openWEPP `snow_hourly_snowfall_water_equiv_sum_m`, so the verdict is not
  migration authority.
- Canonical HPHYS0299 amendments in `SC-SNOWFREEZE-001` and `SC-WATBAL-001`
  now carry the corrected depth-vs-depth `hrsnow` authority and prohibit reuse
  of HPHYS0298 production-migration authority without corrected evidence.
