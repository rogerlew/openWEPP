# Source Scan

Evidence class: Ran.

Command:

```bash
rg -n "snow_melt_model:|SnowMeltModel::CoeShortwaveAlbedoV1|--model|coe_shortwave_albedo_v1" crates/openwepp-runner/src/bin crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs -S
```

Observed output:

```text
crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:3003:                snow_melt_model: openwepp_hillslope_orchestrator::SnowMeltModel::LegacyCoe,
crates/openwepp-runner/src/bin/openwepp-snowbench.rs:47:            "--model" => {
crates/openwepp-runner/src/bin/openwepp-snowbench.rs:50:                    .ok_or_else(|| "SNOWBENCH-E-CLI missing value for --model".to_string())?;
crates/openwepp-runner/src/bin/openwepp-snowbench.rs:91:        return Err("SNOWBENCH-E-CLI --model is only valid for coe-melt".to_string());
crates/openwepp-runner/src/bin/openwepp-snowbench.rs:115:        return Err("SNOWBENCH-E-CLI --model is only valid for coe-melt".to_string());
crates/openwepp-runner/src/bin/openwepp-snowbench.rs:170:        "openwepp-snowbench <export-pysnobal|physics-bulk|coe-melt> --run-dir <path> [--run-file <path>] --output-dir <path> [--variant <candidate_v1|slow_melt_v1|dense_slow_melt_v1|cold_dense_slow_melt_v1>] [--model <legacy_coe|coe_shortwave_albedo_v1>]"
```

Interpretation:

- Production direct-publication day input still selects
  `SnowMeltModel::LegacyCoe`.
- `--model` and `coe_shortwave_albedo_v1` are confined to the diagnostic
  `openwepp-snowbench coe-melt` tool.
- No production parser/runfile/CLI selector or output-schema activation surface
  was added by SNOWDENSITY-05F.
