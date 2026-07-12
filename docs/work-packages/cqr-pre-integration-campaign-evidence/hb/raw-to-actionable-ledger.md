# High-B Raw-To-Actionable Ledger

Evidence class: **Ran + Static**

Source commit: `34a3f1abdf131bf1d7bcff450b8b62629adcf045`.
Metric authority: `/tmp/openwepp-cqr-preint-hb-start-crap.json`.

The exact six-field deduplication yields 21 raw rows in ten fixed modules. All
21 rows are actionable: 12 `E-SCIENCE`, nine `E-PRODUCTION`, zero retained
exception, and zero denominator exclusion. The two reviewers differed on
whether the watershed-structure parser and hillslope-area reader should be
named science or production behavior; the binding disagreement rule selects
`E-PRODUCTION`. Both reviewers agreed they remain actionable and that every
module is science tier.

| ID | Function | CC | Coverage | CRAP | Class |
| --- | --- | ---: | ---: | ---: | --- |
| HB-01 | `validate_erod13_inputs` | 31 | 77.966% | 41.280 | `E-SCIENCE` |
| HB-02 | `erosion_particle_composition` | 46 | 78.882% | 65.929 | `E-SCIENCE` |
| HB-03 | `assemble_wave1_continuity_inputs_quantum` | 34 | 91.124% | 34.808 | `E-SCIENCE` |
| HB-04 | `wave1_xcrit` | 36 | 92.593% | 36.527 | `E-SCIENCE` |
| HB-04 | `wave1_erod` | 53 | 76.265% | 90.561 | `E-SCIENCE` |
| HB-04 | `wave1_route` | 31 | 76.757% | 43.067 | `E-SCIENCE` |
| HB-04 | `validate_wave1_inputs` | 35 | 82.609% | 41.444 | `E-SCIENCE` |
| HB-04 | `wave1_apply_inter_ofe_continuity` | 30 | 91.250% | 30.603 | `E-SCIENCE` |
| HB-05 | `parse_watershed_structure_from_str` | 43 | 94.059% | 43.388 | `E-PRODUCTION` |
| HB-06 | `Wb13DailyWaterBalanceRow::from_surface` | 33 | 77.778% | 44.951 | `E-SCIENCE` |
| HB-07 | `Ws10ChannelImpoundmentKernel::ws11_muskingum_geometry_from_depth` | 15 | 52.308% | 39.408 | `E-SCIENCE` |
| HB-07 | `Ws10ChannelImpoundmentKernel::compute_variable_muskingum_cunge_state` | 32 | 70.455% | 58.410 | `E-SCIENCE` |
| HB-08 | `Ws10ChannelImpoundmentKernel::ws11_route_baseline_wave_series` | 44 | 85.106% | 50.396 | `E-SCIENCE` |
| HB-09 | `write_output_record_parquet_outputs` | 29 | 84.483% | 32.142 | `E-PRODUCTION` |
| HB-09 | `float64_value` | 69 | 100.000% | 69.000 | `E-PRODUCTION` |
| HB-10 | `run` | 58 | 77.446% | 96.596 | `E-PRODUCTION` |
| HB-10 | `hillslope_area_m2_from_source_runfile` | 11 | 45.614% | 30.465 | `E-PRODUCTION` |
| HB-10 | `parse_watershed_runfile` | 66 | 69.369% | 191.186 | `E-PRODUCTION` |
| HB-10 | `validate_manifest_publication_metadata` | 18 | 53.846% | 49.854 | `E-PRODUCTION` |
| HB-10 | `validate_manifest_per_ofe_wb13_publication_policies` | 11 | 45.714% | 30.357 | `E-PRODUCTION` |
| HB-10 | `validate_manifest_mofe_hourly_carry_metadata` | 17 | 51.724% | 49.515 | `E-PRODUCTION` |

No `R-*` or `X-*` proposal is accepted. Parser grammar, cardinality, validation,
area normalization, state/control flow, ordering, error priority, schema/value
mapping, and real publication behavior remain eligible under ADR-0021.

HB-04, HB-08, HB-09, and HB-10 exceed the 2,000-line WARN threshold; their
module records must name decomposition rationale and follow-on split intent.
No target file is at or above the 3,000-line blocking threshold.
