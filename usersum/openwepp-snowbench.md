# `openwepp-snowbench` — Snow model profiling benchmark CLI

## Purpose

Runs snow physics benchmark modes and produces a profile bundle for candidate comparison.

## CLI interface

```bash
openwepp-snowbench \
  <export-pysnobal|physics-bulk|coe-melt|coe-bound-density> \
  --run-dir <path> \
  [--run-file <path>] \
  --output-dir <path> \
  [--variant <candidate_v1|slow_melt_v1|dense_slow_melt_v1|cold_dense_slow_melt_v1|density_compaction_v1>] \
  [--model <legacy_coe|coe_shortwave_albedo_v1>]

openwepp-snowbench jennings-phase \
  --observations <file2.csv> \
  --thresholds <file3.csv> \
  --output-dir <path> \
  [--max-rows <n>]
```

## Required inputs

- For benchmark modes: a valid openWEPP run directory and, where relevant, a run file.
- For `jennings-phase`: observations and thresholds CSV inputs.
- `--output-dir` must be writable.

## Output

- Benchmark mode creates summary and diagnostic files under the output tree (e.g. `physics_bulk_summary.json`, `physics_bulk_summary.md`, forcing bridge tables).
- `jennings-phase` writes a run artifact set under the output directory.

## Practical notes

- The variant and model options are mode-specific and should match your selected benchmark command.
- Use output paths per scenario so forcing-bridge artifacts remain isolated.

## Exit behavior

- `0`: benchmark completed.
- Non-zero: missing run assets, unsupported mode/combination, or malformed input tables.

## Manual validation reference

- Successful benchmark run:
  - `target/release/openwepp-snowbench physics-bulk --run-dir tests/fixtures/cli01/hillslope_run_dir --run-file case.run --output-dir /tmp/openwepp-docs-cli-verify/snowbench-out`
- Log: `/tmp/openwepp-docs-cli-verify/snowbench.log` (exit `0`).
- Generated tree includes:
  - `physics_bulk_summary.md`, `physics_bulk_summary.json`, `physics_bulk_snow.csv`, `forcing_bridge/`.

## Standard handoff language

- Use this handoff target for page updates:
  - `handoff: usersum/documentation-agent`
  - `target_path: usersum/documentation-agent.md`
