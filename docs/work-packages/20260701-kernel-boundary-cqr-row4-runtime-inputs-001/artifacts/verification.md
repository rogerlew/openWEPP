# Verification

Evidence class: Ran.

## CRAP

Before:

```text
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path lcov.info && cargo crap --workspace --lcov lcov.info --min 0 --format json --output /tmp/openwepp-crap-row4-before.json
```

Result: row #4 had 24 unique production offender entries above CRAP 30,
duplicated to 48 rows by current `cargo crap` report shape.

After:

```text
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path lcov.info && cargo crap --workspace --lcov lcov.info --min 0 --format json --output /tmp/openwepp-crap-row4-after-final.json
```

Result: row #4 had 0 entries above CRAP 30. Highest remaining row #4 score:
`growth_equation_parameter_values`, CRAP `26.762390670553938`.

## Focused Tests

```text
cargo nextest run -p openwepp-hillslope-orchestrator
```

Result: `118` tests passed.

## Rust Gates

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
```

Results:

- `cargo fmt --check`: exited 0.
- `cargo clippy --workspace --all-targets -- -D warnings`: exited 0.
- `cargo nextest run --workspace --profile full`: `1229` tests passed,
  `1` skipped, `2` slow; elapsed `650.299s`.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Authority Guards

```text
bash tools/release/check_authority_suite_antievasion.sh
cargo nextest run --test auth11_required_suite_obligation_guards_contract
```

Results:

- Anti-evasion: `PASS: authority suite anti-evasion checks passed.`
- Required-suite obligation guard: `2` tests passed.

## H2637 Endpoint

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
/usr/bin/time -v target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/stage1-seed-authority/after-1b/h2637.run \
  --output-dir /tmp/kernel-boundary-cqr-row4-h2637/output \
  --manifest-path /tmp/kernel-boundary-cqr-row4-h2637/output/manifest.json
```

Results:

- Release build exited 0.
- H2637 exited 0; wall `1:06.99`; max RSS `79684 KiB`.
- Manifest selected `direct-production-executor`.
- `compatibility_edge_invocations`: `0`.
- `scheduler_kernel_executed`: `false`.
- `wb13_publication.row_count`: `235961`.

Protected output comparison against
`/tmp/typed-direct-carrier-identity/base/output`:

```text
PASS H2637.hbp
PASS H2637.loss.json
PASS H2637.plot.parquet
PASS H2637.wat.parquet
PASS H2637.pass.parquet
```

Output hashes:

```text
18c7ddcd8b5b4205876e47e82eaa3931d56db0b98d37f96d5dcebb50b7f85c2e  H2637.hbp
73d588ee03c1316a75743dc6f33225282e8ac82e6647018b395ea66e0d03dcd6  H2637.loss.json
cb1259dda3b5113e58e6fe94ddc10ea8968589ea356a12fe3a358852cce3d223  H2637.plot.parquet
26d4b9415820e6da2e16869f2f926a8b5ddd39c565dfff612a0551477b7e09f6  H2637.wat.parquet
f4de3e5c2224556e6c913d6ca12d807415da56a07b182d4e3238fec1879a6e22  H2637.pass.parquet
```

## Line Counts

```text
1715 crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs
 197 crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs
1005 crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs
1289 crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs
  26 crates/openwepp-hillslope-orchestrator/src/runtime_inputs/07_series_helpers.rs
```

All row #4 files remain below the local 2000-line review threshold.

## Docs And Diff Hygiene

```text
git diff --check
markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260701-kernel-boundary-cqr-row4-runtime-inputs-001 --format json
markdown-doc validate --path docs/work-packages/README.md --path docs/work-packages/20260701-kernel-boundary-cqr-row4-runtime-inputs-001 --format json
```

Results:

- `git diff --check`: exited 0.
- Markdown lint: 9 files scanned; 0 errors; 0 warnings.
- Markdown validate: 9 files scanned; 0 errors.
