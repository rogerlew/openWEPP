# R6E Current Failure Reproduction

Evidence mode: Static + Ran.

Status: reproduced after R6E remediation.

## Direct CLI Reproduction

Ran:

```bash
tmpdir=$(mktemp -d /tmp/r6e_cutover_XXXXXX)
cp -a tests/fixtures/cli01/hillslope_run_dir/. "$tmpdir"
target/debug/openwepp-cli-hill \
  --run-dir "$tmpdir" \
  --run-file case.run \
  --output-dir "$tmpdir/output" \
  --direct-publication-frame-cutover
find "$tmpdir/output" -maxdepth 1 -type f -printf '%f\n'
```

Result:

- exit status: `1`;
- stdout: empty;
- stderr:
  `CLIHILL-E-011 runtime surface failure for direct_publication_cutover:
  HS-SIMOUT-E-001 R6-DIRECT-PUBLICATION-PARITY
  HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH HBP byte identity failed:
  direct=1654 bytes compatibility=1654 bytes`;
- output files: none.

The cutover flag now reaches HBP comparison and still fails closed before public
HBP/WAT/PASS/loss/manifest writes.

## Focused Counter / No-Output Tests

Ran:

```bash
cargo test -p openwepp-runner \
  r6e_cutover_candidate_reaches_direct_input_binding_then_fails_hbp_parity \
  -- --nocapture

cargo test -p openwepp-runner \
  r6_direct_publication_cutover_cli_flag_reaches_direct_binding_then_fails_hbp_parity \
  --test r6_direct_publication_cutover_cli_contract \
  -- --nocapture
```

Result:

- both tests passed;
- unit test asserts `run_frame_constructions == 1`;
- unit test asserts `executor_constructions == 1`;
- unit test asserts `skeleton_runs == 0`;
- unit test asserts `publication_capture_runs == 1`;
- unit test asserts direct day-frame constructions equal commits;
- unit test asserts direct compute, state mutation, downstream operand, and
  shadow projection counters are nonzero;
- unit test asserts `compatibility_edge_invocations == 0`;
- unit and CLI tests assert the old input-binding marker is absent;
- CLI integration test asserts no `H5.hbp`, `H5.loss.json`,
  `H5.wat.parquet`, `H5.plot.parquet`, or
  `openwepp_hillslope_run_manifest.json` is written.

## Compatibility Baseline Fixture Context

The same fixture without `--direct-publication-frame-cutover` writes:

- `H5.hbp`: `1654` bytes;
- `H5.loss.json`: `342` bytes;
- `H5.plot.parquet`: `202` bytes;
- `H5.wat.parquet`: `14503` bytes;
- `openwepp_hillslope_run_manifest.json`: `6864` bytes.

Static fixture scan shows the fixture configures HBP through
`case.run:14: pass = "output/H5.hbp"` and does not configure a PASS Parquet
target.

## Current First Blocker

`HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

The retained cutover execution is now produced by direct publication capture
from typed day inputs. The first remaining blocker is direct process parity for
HBP bytes, not absent production direct-runtime input binding.
