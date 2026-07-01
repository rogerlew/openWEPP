# CRAP Before

Ran: reused `/tmp/openwepp-crap-row7-after-final.json` from the final
post-row-7 full-workspace command:

```text
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path lcov.info && cargo crap --workspace --lcov lcov.info --min 0 --format json --output /tmp/openwepp-crap-row7-after-final.json
```

Row #9 extraction:

```text
file	function	line	cc	coverage	crap
evapotranspiration.rs	DirectEvapotranspirationPmetComputeInputs::compute	721	35.0	0.0	1260.0
evapotranspiration.rs	compute_stage_soil_evaporation	1363	14.0	0.0	210.0
evapotranspiration.rs	DirectEvapotranspirationPmetComputeInputs::transpiration_storage_terms	907	8.0	0.0	72.0
evapotranspiration.rs	DirectEvapotranspirationPmetComputeInputs::evaporation_storage_terms	864	8.0	0.0	72.0
00_core_frames.rs	validate_direct_snow_layers	1991	10.0	17.073170731707318	67.02761132310906
storage.rs	maybe_write_r7h_storage_trace	195	8.0	2.9629629629629632	66.47800599502108
evapotranspiration.rs	maybe_write_r7h_et_trace	72	8.0	3.4782608695652173	65.55133327854031
runoff.rs	maybe_write_r7h_runoff_rebalance_trace	69	8.0	5.333333333333334	62.296424296296294
00_core_frames.rs	validate_direct_day_constructor_inputs	1797	27.0	70.51282051282051	45.69077150659992
evapotranspiration.rs	DirectDayFrame::compute_r4n_surface_et	366	30.0	75.63025210084034	43.02553646273141
00_core_frames.rs	validate_direct_lane_constructor_inputs	1676	26.0	71.1864406779661	42.1710204061759
00_core_frames.rs	validate_direct_frost_runtime_fine_layers	2240	6.0	0.0	42.0
00_core_frames.rs	DirectLaneFrame::commit_day	968	17.0	56.9620253164557	40.038453347503946
runoff.rs	DirectDayFrame::rebalance_r4a_frost_projection_to_storage_target	1198	10.0	41.509433962264154	30.01047844865224
```

Static: row #9 now has 14 unique production offender entries above 30. Current
`cargo crap` report shape duplicates each row, giving 28 duplicated report rows.
