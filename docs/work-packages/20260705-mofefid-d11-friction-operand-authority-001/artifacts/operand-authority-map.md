# Operand Authority Map

Status: executed-hold-source-authority
Evidence mode: Static

| Operand | Units | Timing/OFE basis | Source authority | Decision | Guard/default policy | Evidence |
|---|---|---|---|---|---|---|
| `I` | `m s^-1` | Time-varying event/hyetograph intensity; publication-side hourly candidate is per lane/day/hour | `SC-CLIMATE-001` WB14 addendum, `HillslopeDirectClimateDayForcing.intsty_m_s`, `DirectWb14HyetographInterval.intensity_m_s`; `wb14_hourly_rainfall_m[h] / 3600 s` if using hourly bins | source candidate accepted; wiring held | Active builder must consume finite nonnegative source intensity. Dry/no-rain zero may come only from source data, not missing-source default. | `SC-OFEROUTE-001` rev 19 alias/unit rows; `01_frost_and_layer_helpers.rs`; `direct_runtime/runoff.rs`. |
| `k_o` | dimensionless | OFE/cell surface-class operand | Woolhiser/KINEROS cited by `SC-OFEROUTE-001`, but no WEPP runtime surface-class mapping found | HOLD | No all-lane `k_o=500` default ratified. D-val constants are fixtures, not runtime defaults. | `SC-OFEROUTE-001#GAP-OFEROUTE-007`; `laned_shadow.rs` hardcoded path. |
| `C_d` | dimensionless | Form/vegetation drag coefficient; OFE/cell operand | Equation authority exists in `SC-OFEROUTE-001`; no runtime input lineage found | HOLD | Missing source/default must fail closed; do not infer from Chapter-10 friction terms. | `kinematic_wave.rs` field exists; no builder source. |
| `D_r` | `m` | Roughness-element tip height; OFE/cell operand | Equation/D-val fixture authority exists; no residue/roughness-to-`D_r` mapping found | HOLD | Missing source/default must fail closed. | D-val Case 2 fixture value is not a WEPP runtime default. |
| `lambda` | dimensionless, bounded `[0,1]` | Roughness concentration; OFE/cell operand | Equation authority exists; no runtime source found | HOLD | Missing source/default must fail closed; unrelated `lambda` symbols rejected. | `SC-OFEROUTE-001` rev 19. |
| `LAI` | `m^2 m^-2` | Daily plant state; lane/OFE after growth projection | `SC-PLANT-001` `LAI`; `DirectGrowthStateSurface.leaf_area_index`; management projection `lai` | source candidate accepted; consumer wiring held | Active builder may consume only a validated growth-state payload. Current shadow row does not carry it. | `SC-PLANT-001`; `growth.rs`; `00_builders_and_authority.rs`; `00c_day_input_builder_impl.rs`. |
| `h_c` | `m` | Canopy-height candidate; timing/source binding unresolved | `SC-PLANT-001` `Hc`; `canhgt` appears in PMET/frost paths | HOLD | Candidate is not sufficient until Lane D source timing and consumer field are ratified. | `SC-PLANT-001`; `00_builders_and_authority.rs`; `erosion.rs`. |

Disposition: `GAP-OFEROUTE-007` remains open as a source-authority hold. No
operator-approved/risk-ratified default set exists for the missing operands.
