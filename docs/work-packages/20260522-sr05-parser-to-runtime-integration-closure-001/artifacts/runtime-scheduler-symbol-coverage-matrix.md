# Runtime Scheduler Symbol Coverage Matrix (SR05)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Matrix maps scheduler-consumed runtime symbol assertions to SR02/SR03 seam obligations and SR04 continuity expectations.

Ran:
- Covered symbol assertions were executed in passing SR05 integration tests.

| symbol family | representative symbols asserted | scheduler integration tests | source seam | coverage status |
|---|---|---|---|---|
| Soil canonical-first aliases | `solthk`, `dg`, `thetdr`, `thetfc`, `nsl`, `ssc` | `parser_to_hillslope_runtime_surface_closure`; `slope_and_soil_parser_outputs_propagate_to_hillslope_runtime_surface_closure` | SR03 | `covered` |
| Soil indexed layer aliases | `solthk_0002`, `dg_0002`, `ssc_0002` | `parser_to_hillslope_runtime_surface_closure`; `slope_and_soil_parser_outputs_propagate_to_hillslope_runtime_surface_closure` | SR03 | `covered` |
| Slope canonical-first aliases | `nslpts`, `slplen`, `avgslp`, `xinput_0002`, `slpinp_0002` | `slope_parser_to_hillslope_runtime_surface_closure`; `slope_and_soil_parser_outputs_propagate_to_hillslope_runtime_surface_closure` | SR02 | `covered` |
| Slope indexed OFE aliases | `ofe2_nslpts`, `ofe2_slplen`, `ofe2_avgslp`, `ofe2_xinput_0003`, `ofe2_slpinp_0003` | `slope_parser_to_hillslope_runtime_surface_closure`; `slope_and_soil_parser_outputs_propagate_to_hillslope_runtime_surface_closure` | SR02 | `covered` |
| Slope/soil seam guard failures | `HS-RUNTIME-E-014`, `HS-RUNTIME-E-023`, `HS-RUNTIME-E-028`, `HS-RUNTIME-E-033` | typed-failure tests at `:243`, `:320`, `:339`, `:359` | SR02 + SR03 | `covered` |
