# H39 Hourly Water-Balance Run Evidence

Status: updated

Evidence mode: ran

Static:
- Candidate run file: `p39_openwepp.run` from
  `/tmp/unpalatable_parity_20260529T192707Z/runs`.
- Candidate runner: `target/debug/openwepp-cli-hill`.

Ran:
- Pre-patch reproduction root: `/tmp/hphys0247_20260602T061908Z`.
- Patched H39 root: `/tmp/hphys0247_20260602T062939Z_patched`.
- Final H39 root: `/tmp/hphys0247_20260602T070132Z_final`.
- Run command:
  `OPENWEPP_HPHYS0245_TRACE_PATH=$ROOT/hillslope_output/H39.hphys0247.trace.jsonl OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=1461 target/debug/openwepp-cli-hill --run-dir $ROOT/runs --run-file p39_openwepp.run --output-dir $ROOT/hillslope_output --policy compat`.
- Output WAT parquet:
  `/tmp/hphys0247_20260602T070132Z_final/hillslope_output/H39.wat.parquet`.
- Trace:
  `/tmp/hphys0247_20260602T070132Z_final/hillslope_output/H39.hphys0247.trace.jsonl`.
- Manifest:
  `/tmp/hphys0247_20260602T070132Z_final/hillslope_output/openwepp_hillslope_run_manifest.json`.
- Manifest hourly evidence:
  `adapter_boundary.selected_lane=hourly`,
  `timestep_policy.selected_lane=hourly`,
  `execution_provenance.selected_lane=hourly`,
  `execution_provenance.scheduler_outcome_class=completed`,
  `execution_provenance.executed_day_count=1461`.
- Winter trigger evidence:
  `coupling_vectors.winter.active=true` and
  `coupling_vectors.winter.snow_file_present=false`, confirming sidecar
  discoverability no longer gates snow execution.
