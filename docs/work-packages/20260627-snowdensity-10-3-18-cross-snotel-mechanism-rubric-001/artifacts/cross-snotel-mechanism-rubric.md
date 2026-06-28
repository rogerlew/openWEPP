# SNOWDENSITY-10.3.18 Cross-SNOTEL Mechanism Rubric

Evidence mode: Static/Ran diagnostic. No promotion or activation decision.

- Disposition: `DIAGNOSTIC-COMPLETE-NO-PROMOTION-DECISION`
- Models scored: `8`
- Supported direct-runtime models: `5`
- Activated robust fail count: `17`
- Activated robust score: `172`
- Signature improvements vs activated: `38`
- Top supported next lever: `harder_pomeroy_partition`
- Humid-New-England read: `NOT-REPRESENTATIVE`

## Model Summary

| Model | Availability | Robust fail | Robust score | SWE median bias | Depth median bias | Density median bias |
|---|---|---:|---:|---:|---:|---:|
| `legacy_baseline` | `current_direct_runtime` | 16 | 176 | -0.1041 | -0.0774412 | -55.5638 |
| `activated_bundle` | `current_direct_runtime` | 17 | 172 | -0.268769 | -0.327801 | 0.288915 |
| `harder_pomeroy_partition` | `current_direct_runtime` | 15 | 179 | -0.219035 | -0.294542 | 23.6234 |
| `open_sublimation_stage_a_10_3_16` | `current_direct_runtime` | 20 | 153 | -0.274443 | -0.437107 | -23.0487 |
| `shallow_pack_guard_10_3_17` | `current_direct_runtime` | 17 | 172 | -0.268769 | -0.324425 | -9.72131 |
| `spring_densification_10_3_11` | `archival_not_current_selector` | 0 | 0 |  |  |  |
| `winter_thaw_state_loss_10_3_7` | `archival_snowbench_only` | 0 | 0 |  |  |  |
| `pysnobal_reference` | `snotel_h_flag_profile` | 28 | 11 | -0.38608 | -1.1176 | -287.079 |

## Supported Lever Rank

| Rank | Model | Robust score delta | Robust fail delta | Better cells | Worse cells |
|---:|---|---:|---:|---:|---:|
| 1 | `harder_pomeroy_partition` | 7 | 2 | 9 | 2 |
| 2 | `shallow_pack_guard_10_3_17` | 0 | 0 | 1 | 1 |
| 3 | `open_sublimation_stage_a_10_3_16` | -19 | -3 | 2 | 16 |

## Signature Improvements

| Site | Climate | Model | Cell | Activated | Candidate |
|---|---|---|---|---|---|
| `snotel_mica_creek_st_joe_id` | `northern_rockies_intermountain` | `legacy_baseline` | `long_term_snow_cover_duration` | `marginal` | `pass` |
| `snotel_mica_creek_st_joe_id` | `northern_rockies_intermountain` | `legacy_baseline` | `seasonal_peak_swe_date` | `marginal` | `strong` |
| `snotel_mica_creek_st_joe_id` | `northern_rockies_intermountain` | `legacy_baseline` | `seasonal_peak_depth_date` | `pass` | `strong` |
| `snotel_mica_creek_st_joe_id` | `northern_rockies_intermountain` | `legacy_baseline` | `seasonal_ablation_meltout_date` | `fail` | `marginal` |
| `snotel_paradise_wa` | `cascades_maritime` | `legacy_baseline` | `long_term_snow_cover_duration` | `marginal` | `strong` |
| `snotel_paradise_wa` | `cascades_maritime` | `legacy_baseline` | `seasonal_peak_swe_date` | `fail` | `strong` |
| `snotel_paradise_wa` | `cascades_maritime` | `legacy_baseline` | `seasonal_peak_depth_date` | `marginal` | `strong` |
| `snotel_paradise_wa` | `cascades_maritime` | `legacy_baseline` | `seasonal_ablation_meltout_date` | `fail` | `strong` |
| `snotel_css_lab_ca` | `sierra_maritime` | `legacy_baseline` | `long_term_snow_cover_duration` | `marginal` | `pass` |
| `snotel_css_lab_ca` | `sierra_maritime` | `legacy_baseline` | `seasonal_accumulation_onset_date` | `pass` | `strong` |
| `snotel_css_lab_ca` | `sierra_maritime` | `legacy_baseline` | `seasonal_peak_swe_date` | `marginal` | `strong` |
| `snotel_css_lab_ca` | `sierra_maritime` | `legacy_baseline` | `seasonal_ablation_meltout_date` | `marginal` | `strong` |
| `snotel_snowbird_ut` | `wasatch_intermountain` | `legacy_baseline` | `long_term_snow_cover_duration` | `marginal` | `strong` |
| `snotel_snowbird_ut` | `wasatch_intermountain` | `legacy_baseline` | `seasonal_peak_swe_date` | `fail` | `strong` |
| `snotel_snowbird_ut` | `wasatch_intermountain` | `legacy_baseline` | `seasonal_peak_depth_date` | `marginal` | `strong` |
| `snotel_snowbird_ut` | `wasatch_intermountain` | `legacy_baseline` | `seasonal_ablation_meltout_date` | `marginal` | `pass` |
| `snotel_niwot_co` | `front_range_continental` | `legacy_baseline` | `long_term_snow_cover_duration` | `marginal` | `pass` |
| `snotel_niwot_co` | `front_range_continental` | `legacy_baseline` | `seasonal_peak_swe_date` | `fail` | `strong` |
| `snotel_niwot_co` | `front_range_continental` | `legacy_baseline` | `seasonal_peak_depth_date` | `fail` | `strong` |
| `snotel_niwot_co` | `front_range_continental` | `legacy_baseline` | `seasonal_ablation_meltout_date` | `marginal` | `strong` |
| `harvard_hardwood` | `humid_new_england_hardwood` | `legacy_baseline` | `seasonal_densification_trajectory` | `fail` | `marginal` |
| `marcell_conifer` | `laurentian_continental_conifer` | `legacy_baseline` | `long_term_cold_season_bulk_density` | `marginal` | `pass` |
| `marcell_conifer` | `laurentian_continental_conifer` | `legacy_baseline` | `long_term_snow_cover_duration` | `pass` | `strong` |
| `marcell_conifer` | `laurentian_continental_conifer` | `legacy_baseline` | `seasonal_ablation_meltout_date` | `pass` | `strong` |
| `marcell_deciduous` | `laurentian_continental_deciduous` | `legacy_baseline` | `long_term_cold_season_bulk_density` | `marginal` | `pass` |
| `marcell_open` | `laurentian_continental_open` | `legacy_baseline` | `long_term_cold_season_bulk_density` | `marginal` | `pass` |
| `snotel_mica_creek_st_joe_id` | `northern_rockies_intermountain` | `harder_pomeroy_partition` | `seasonal_peak_depth_date` | `pass` | `strong` |
| `snotel_paradise_wa` | `cascades_maritime` | `harder_pomeroy_partition` | `long_term_snow_cover_duration` | `marginal` | `pass` |
| `snotel_paradise_wa` | `cascades_maritime` | `harder_pomeroy_partition` | `seasonal_peak_swe_date` | `fail` | `marginal` |
| `snotel_paradise_wa` | `cascades_maritime` | `harder_pomeroy_partition` | `seasonal_peak_depth_date` | `marginal` | `strong` |
| `snotel_css_lab_ca` | `sierra_maritime` | `harder_pomeroy_partition` | `seasonal_accumulation_onset_date` | `pass` | `strong` |
| `snotel_css_lab_ca` | `sierra_maritime` | `harder_pomeroy_partition` | `seasonal_peak_swe_date` | `marginal` | `pass` |
| `snotel_css_lab_ca` | `sierra_maritime` | `harder_pomeroy_partition` | `seasonal_ablation_meltout_date` | `marginal` | `pass` |
| `snotel_snowbird_ut` | `wasatch_intermountain` | `harder_pomeroy_partition` | `long_term_snow_cover_duration` | `marginal` | `pass` |
| `snotel_niwot_co` | `front_range_continental` | `harder_pomeroy_partition` | `seasonal_peak_depth_date` | `fail` | `marginal` |
| `snotel_niwot_co` | `front_range_continental` | `open_sublimation_stage_a_10_3_16` | `seasonal_peak_depth_date` | `fail` | `marginal` |
| `marcell_conifer` | `laurentian_continental_conifer` | `open_sublimation_stage_a_10_3_16` | `long_term_cold_season_bulk_density` | `marginal` | `strong` |
| `snotel_css_lab_ca` | `sierra_maritime` | `shallow_pack_guard_10_3_17` | `seasonal_accumulation_onset_date` | `pass` | `strong` |

Absolute SWE/depth magnitude cells are forcing-limited report-only cells under INV-SNOWFREEZE-050. Legacy and PySnobal profiles are ADR-0017 flags, not targets.
