# `openwepp-cli-totalwatsed3` — Watershed Aggregation CLI

## Purpose

Aggregates hillslope parquet artifacts into a total-watershed parquet output.

## CLI interface

```bash
openwepp-cli-totalwatsed3 \
  --input-dir <interchange-dir> \
  --output <totalwatsed3.parquet> \
  [--pass H.pass.parquet] \
  [--wat H.wat.parquet] \
  [--soil H.soil.parquet] \
  [--element H.element.parquet]
```

## Required inputs

- `--input-dir`: input directory containing hillslope interchange files.
- `--output`: destination parquet file.

## Optional inputs

- `--pass`, `--wat`, `--soil`, `--element` can be set explicitly if you use custom filenames.

## Output

- Writes one parquet file containing the assembled total watershed data.

## Practical notes

- In practice, run this on a directory that includes the expected pass parquet for the hillslope case (commonly `H.pass.parquet` in interchange conventions).
- If required parquet files are missing, execution fails fast with a clear CLITW3 error and exits non-zero.

## Exit behavior

- `0`: aggregation succeeded.
- Non-zero: missing/invalid input parquet or unsupported/insufficient interchange inputs.

## Manual validation reference

- Blocking validation case due fixture mismatch:
  - `target/release/openwepp-cli-totalwatsed3 --input-dir tests/fixtures/cli01/hillslope_run_dir --output /tmp/openwepp-docs-cli-verify/totalwatsed3.parquet --pass tests/fixtures/cli01/hillslope_run_dir/output/H5.hbp --wat tests/fixtures/cli01/hillslope_run_dir/output/H5.wat.parquet`
- Log: `/tmp/openwepp-docs-cli-verify/totalwatsed3.log` (exit `1`, because required `H.pass.parquet` was not present).

## Standard handoff language

- Use this handoff target for page updates:
  - `handoff: usersum/documentation-agent`
  - `target_path: usersum/documentation-agent.md`
