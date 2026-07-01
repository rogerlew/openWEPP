# CRAP Before

Ran: reused `/tmp/openwepp-crap-row4-after-final.json` from the final
post-row-4 full-workspace command:

```text
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path lcov.info && cargo crap --workspace --lcov lcov.info --min 0 --format json --output /tmp/openwepp-crap-row4-after-final.json
```

Row #7 extraction:

```text
file	function	line	cc	coverage	crap
00a_snow_frost_authority_impl.rs	layered_snow_frost_insulation_depth_density	502	18.0	0.0	342.0
01_publication.rs	DirectFrostLayerCarryProjection::validate_for_layer	102	13.0	0.0	182.0
00a_snow_frost_authority_impl.rs	invert_sturm1997_snow_density_kg_m3	612	10.0	0.0	110.0
00_builders_and_authority.rs	direct_production_sturm1995_climate_normals	2632	9.0	0.0	90.0
01_wb12_wb16_wb19_projection.rs	project_typed_wb11_frozen_depth_refresh	47	14.0	31.506849315068493	76.97925283470904
04_direct_publication.rs	validate_retained_direct_publication_frame	787	8.0	0.0	72.0
00a_snow_frost_authority_impl.rs	sturm1997_snow_conductivity_w_m_k	582	8.0	0.0	72.0
00_builders_and_authority.rs	snowdensity1015_default_snow_density_model	2549	10.0	17.24137931034483	66.68129074582804
00_builders_and_authority.rs	snowdensity1037_diagnostic_snow_melt_model	2763	7.0	0.0	56.0
01_frost_and_layer_helpers.rs	rebalance_direct_production_no_final_frost_layers_to_storage	296	18.0	51.76470588235295	54.361333604722155
00_builders_and_authority.rs	snowdensity1015_default_snow_melt_model	2788	9.0	20.0	50.47200000000001
00_builders_and_authority.rs	maybe_write_r7h_direct_production_wb15_trace	2462	8.0	15.068493150684931	47.20906284301201
00_builders_and_authority.rs	DirectProductionPriestleyTaylorAuthority::compute_demand	3282	6.0	0.0	42.0
00_builders_and_authority.rs	DirectProductionDayInputBuilder::build	1961	36.0	85.89211618257261	39.63907180421448
00_builders_and_authority.rs	direct_production_surface_litter_projection	3036	11.0	42.10526315789473	34.480244933663805
01_wb12_wb16_wb19_projection.rs	wb16_equivalent_plane_alpha	579	7.0	20.0	32.08800000000001
00_builders_and_authority.rs	direct_production_typed_growth_crop_authority	1118	25.0	78.50467289719626	31.207435167581878
```

Static: the execplan's original duplicated row count was 32; the current
post-row-4 measurement is 17 unique entries and 34 duplicated `cargo crap`
rows.
