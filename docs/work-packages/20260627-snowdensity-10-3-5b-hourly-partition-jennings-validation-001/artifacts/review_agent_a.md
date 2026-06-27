# Review Agent A

Status: complete
Evidence mode: Static/Ran

Findings:

| Severity | Finding | Disposition |
|---|---|---|
| Medium | The generalized opt-in partition obscured the exact legacy snowfall-depth source expression required by `hphys0299_static_openwepp_sources_publish_depth_and_water_equiv_separately`. This would weaken existing depth-vs-water-equivalent provenance evidence even though numerics were unchanged. | Fixed by adding `simimpl28_legacy_stmtim_snowfall_depth_m` with explicit `rain_m / wntdur * 10.0`; targeted guard and full workspace suite pass. |
| Low | New runtime diagnostic symbols must be registered or CLI03 multi-OFE registry checks fail closed. | Fixed by adding boundary catalog aliases and symbol-registry audit allowlist entries for `snow.hourly.stmtim.phase_model`, `rain_fraction`, `snow_fraction`, `relative_humidity`, and `hydrometeor_temperature_c`. |

Residual risk: the candidate remains opt-in and has only precipitation-phase
validation here. Snow-depth remediation still needs later packages to test
whether improved phase partition materially improves the maritime snow-depth
over-accumulation signatures.
