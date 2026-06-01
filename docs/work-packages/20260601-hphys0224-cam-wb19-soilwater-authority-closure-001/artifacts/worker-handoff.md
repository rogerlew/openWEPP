# HPHYS0224 Worker Handoff

Status: completed  
Evidence mode: Static + Ran

## Immediate Next Actions

1. Keep WB19 soil-water-cap suite (`cas_l4_subhyd_withdrawal_soilwater_cap_001`)
   in required/hard-fail lane for all follow-on kernel work touching
   lateral/drainage withdrawal paths.
2. Continue contract-first remediation for unchanged monitored residual families
   (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`) using
   HPHYS0224 rerun as the new reference.
3. Preserve valid semantic rerun settings for this cohort:
   `--candidate-year-offset 2012`, no partition filter.
4. Use `/tmp/hphys0224_20260601T054337Z/parity/reports/hillslope_semantic_summary.json`
   as the baseline for post-HPHYS0224 delta adjudication.
