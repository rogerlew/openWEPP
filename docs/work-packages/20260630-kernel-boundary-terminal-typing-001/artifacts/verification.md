# Verification

Evidence class: Ran.

## Build And Static Gates

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check
bash tools/release/check_authority_suite_antievasion.sh
cargo nextest run --test auth11_required_suite_obligation_guards_contract
git diff --check
```

All commands exited `0`.

## Full Test Gate

Final run:

```text
cargo nextest run --workspace --profile full --no-fail-fast
```

Result: `1221` tests passed, `1` skipped, `3` slow; elapsed `703.514s`.

The first full run failed five stale/environment checks and was not accepted:

- HPHYS0296 still looked for deleted scheduler trace field names.
- The typed seed-authority source guard still expected a runtime-selection
  branch that no longer exists in direct-only setup.
- HPHYS0298 failed before its intended unit guard because `.venv` lacked
  `pandas`.
- Paradigm-2 static evidence still looked for a nullable meltwater-temperature
  fixture in the serializer instead of runner test fixtures.
- `owcmp env` failed because `.venv` lacked `pyarrow`.

After retargeting the stale assertions and installing the missing local venv
packages, the focused failed subset passed (`5`/`5`), then the full profile
passed.

## No-Compatibility Proof

Forbidden executable-runtime scan:

```text
rg -n "HillslopeWritebackSurface|HillslopeKernelRequest|KernelWritebackPayload|SymbolRegistry|HotSymbolTables|HillslopePhaseScheduler|HillslopeDayFrame|scheduler_trace|runtime_surface_symbol_value|require_runtime_surface_scalar" crates/openwepp-hillslope-orchestrator/src crates/openwepp-runner/src tests -S --glob '*.rs'
```

Result: matches only in source-guard test literals:

- `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`

`cargo nextest run --workspace --profile full` includes the direct source
guards that assert no compatibility edge or carrier reintroduction on the
production direct path.

## H2637 Endpoint

Ran:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
/usr/bin/time -v target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/stage1-seed-authority/after-1b/h2637.run \
  --output-dir /tmp/kernel-boundary-terminal-h2637/output \
  --manifest-path /tmp/kernel-boundary-terminal-h2637/output/manifest.json
```

Result: exited `0`; wall `1:10.69`; max RSS `79284 KiB`.

Manifest:

- `runtime_selection.selected`: `direct-production-executor`.
- `runtime_selection.compatibility_rollback_available`: `false`.
- `direct_runtime_counters.compatibility_edge_invocations`: `0`.
- `execution_provenance.scheduler_kernel_executed`: `false`.
- `wb13_publication.row_count`: `235961`.

The runfile writes protected outputs to
`/tmp/stage1-seed-authority/after-1b/output`. Those actual output paths were
byte-identical against `/tmp/typed-direct-carrier-identity/base/output` for
`H2637.hbp`, `H2637.loss.json`, `H2637.plot.parquet`, `H2637.wat.parquet`, and
`H2637.pass.parquet`.

## Docs

```text
markdown-doc lint --path docs/ROADMAP.md --path docs/architecture/array-native-runtime-specification.md --path docs/work-packages/README.md --path docs/work-packages/20260630-kernel-boundary-terminal-typing-001 --format json
markdown-doc validate --path docs/ROADMAP.md --path docs/architecture/array-native-runtime-specification.md --path docs/work-packages/README.md --path docs/work-packages/20260630-kernel-boundary-terminal-typing-001 --format json
```

Results: `9` files scanned; `0` lint errors, `0` warnings, `0` validation
errors.
