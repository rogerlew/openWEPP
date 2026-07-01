# Verification

Evidence mode: Ran.

## Focused Tests

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator --lib cqr_row5 -- --nocapture
```

Result:

- `8` tests passed.

## CRAP

Ran:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-row5-after.lcov
cargo crap --workspace --lcov /tmp/openwepp-row5-after.lcov --min 0 --format json > /tmp/openwepp-crap-row5-after.json
jq -r '[.entries[] | select(.file | contains("/crates/openwepp-hillslope-orchestrator/src/hydrology/")) | select(.crap > 30)] | length' /tmp/openwepp-crap-row5-after.json
```

Result:

- Full workspace LCOV tests passed and wrote `/tmp/openwepp-row5-after.lcov`.
- Full workspace CRAP JSON written to
  `/tmp/openwepp-crap-row5-after.json`.
- Row #5 owned functions above CRAP 30: `0`.

See `crap-after.md` for the primary closure disposition.

## Rust Gates

Ran:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo deny check
```

Results:

- `cargo fmt --check`: exited 0.
- `cargo clippy --workspace --all-targets -- -D warnings`: exited 0.
- `cargo nextest run --workspace --profile full`: `1254` tests run, `1254`
  passed, `1` skipped. Slow test:
  `openwepp::snowdensity05e_melt_adjudication::coe_melt_snowbench_runs_both_models_as_diagnostic_only`.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Authority Guards

Ran:

```text
bash tools/release/check_authority_suite_antievasion.sh
cargo nextest run --test auth11_required_suite_obligation_guards_contract
```

Results:

- Authority anti-evasion script: `PASS: authority suite anti-evasion checks
  passed.`
- Auth11 obligation guard: `2` tests run, `2` passed.

## Markdown Docs

Ran:

```text
markdown-doc lint --path docs/work-packages/20260701-kernel-boundary-cqr-row5-hydrology-wb-kernel-001 --path docs/work-packages/README.md
markdown-doc validate --path docs/work-packages/20260701-kernel-boundary-cqr-row5-hydrology-wb-kernel-001 --path docs/work-packages/README.md
```

Final result after artifact edits:

- `markdown-doc lint`: 10 files, 0 errors, 0 warnings.
- `markdown-doc validate`: 10 files, 0 errors.

## H2637 Endpoint

Ran:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
/usr/bin/time -v target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/stage1-seed-authority/after-1b/h2637.run \
  --output-dir /tmp/kernel-boundary-cqr-row5-h2637/output \
  --manifest-path /tmp/kernel-boundary-cqr-row5-h2637/output/manifest.json
```

Results:

- Release build exited 0.
- H2637 exited 0; wall `1:08.04`; max RSS `79916 KiB`.
- Manifest `runtime_selection.selected`: `direct-production-executor`.
- Manifest `direct_runtime_counters.compatibility_edge_invocations`: `0`.
- Manifest `execution_provenance.scheduler_kernel_executed`: `false`.
- Manifest `wb13_publication.row_count`: `235961`.
- CLI output files were emitted through the runfile output path
  `/tmp/stage1-seed-authority/after-1b/output`; the explicit row #5 output
  directory contains the manifest.

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

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs
```

Result:

```text
   619 crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs
   342 crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs
  1287 crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs
  1800 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs
  1967 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs
  1915 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs
  1228 crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs
  9158 total
```

Disposition: see `line-count-governance.md`.
