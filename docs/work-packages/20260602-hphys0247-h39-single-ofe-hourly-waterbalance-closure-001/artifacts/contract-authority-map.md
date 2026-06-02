# Contract Authority Map

Status: updated

Evidence mode: static

Static:
- `SC-SNOWFREEZE-001` version `10`: clarified that
  `snow.options.snow_file_present` is sidecar discoverability/override
  provenance only and cannot gate winter execution when legacy runtime triggers
  are true.
- `SC-SUBHYD-001` version `25`: added `INV-SUBHYD-024` for baseline WB19
  `meblfc`, active-layer `tdvv`, and `fffx` lateral conductivity lineage.
- `SC-WATBAL-001` version `73`: added `INV-WATBAL-035` tying H39 hourly closure
  evidence to snow-trigger and WB19 lateral-capacity authority.
- Legacy provenance: `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Runtime aliases used: `snow.runtime_swe`, `snow.options.snow_file_present`,
  `snow.hourly.*`, `winter.hourly.*`, `wb18_perc_theta_####`,
  `wb18_perc_fc_####`, `wb18_perc_ul_####`, `wb18_perc_ssc_####`, `q`,
  `Qdd`, `Qd`, `latqcc`.

Ran:
- Not applicable; this is a static authority map.
