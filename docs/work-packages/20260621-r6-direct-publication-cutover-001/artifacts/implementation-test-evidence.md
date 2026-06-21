# Implementation Test Evidence

Status: executed-hold.
Evidence mode: Ran.

## Focused Tests

Ran:

```text
cargo test -p openwepp-runner r6_ -- --nocapture
```

Result: PASS.

Covered test:

- `r6_cutover_candidate_fails_closed_on_direct_publication_identity_gap`

The test executes the fixture with
`HillslopeRuntimeSelection::DirectPublicationFrameCutover`, expects a
fail-closed `R6-DIRECT-PUBLICATION-PARITY` error, and verifies direct runtime
publication counters:

- `run_frame_constructions == 1`
- `executor_constructions == 1`
- `skeleton_runs == 0`
- `publication_capture_runs == 1`
- `compatibility_edge_invocations == 0`

It also verifies the fail-closed candidate writes none of the public output
files or manifest before parity gates pass.

Ran:

```text
cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture
```

Result: PASS. The CLI contract test invokes
`--direct-publication-frame-cutover`, verifies non-zero exit with
`R6-DIRECT-PUBLICATION-PARITY`, and verifies no HBP, loss JSON, WAT parquet,
PASS parquet, or manifest output is written before gate success.

Ran:

```text
cargo test -p openwepp-runner r6a_ -- --nocapture
```

Result: PASS. The R6A direct publication shadow and frame-consumer tests still
pass after broadening the helper signatures.

Ran:

```text
cargo test -p openwepp-runner r2a_default_fixture_run_constructs_no_direct_runtime_skeleton -- --nocapture
```

Result: PASS. The default compatibility path still constructs no direct
runtime skeleton.

Ran:

```text
cargo test -p openwepp-runner
```

Result: PASS. Full runner package tests passed.

Ran:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Result: PASS.

Ran:

```text
markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260621-r6-direct-publication-cutover-001 --format json
git diff --check
```

Result: PASS.

Ran:

```text
cargo run -p openwepp-runner --bin openwepp-cli-hill -- \
  --run-dir /tmp/r6cutover.wv66Ba \
  --run-file case.run \
  --output-dir /tmp/r6cutover.wv66Ba/output \
  --direct-publication-frame-cutover
```

Result: exit status `1`. Error:
`R6-DIRECT-PUBLICATION-PARITY HBP byte identity failed: direct=1654 bytes
compatibility=1654 bytes`.

## Not Run

- Full output-family HBP/WAT/PASS/loss/manifest acceptance fixtures.
- PASS parquet fixture coverage for the cutover candidate.
- Independent reconstruction and anti-alias fixtures for accepted output-family
  cutover.

## Gate

PASS for the executed-hold claim. BLOCKED for R6 completion.
