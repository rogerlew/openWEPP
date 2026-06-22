# Verification

Status: complete.

## Static Evidence

Static:

- Direct production mode is a distinct runtime selection:
  `HillslopeRuntimeSelection::DirectProductionExecutor`.
- CLI selection is explicit: `--direct-production-executor`.
- Default runner entrypoint still calls
  `execute_hillslope_run_with_runtime_selection(..., Compatibility)`.
- Direct production execution is selected once in
  `execute_hillslope_run_with_runtime_selection`; non-direct modes continue to
  use `execute_hillslope_climate_days`.
- Production direct selection skips symbol-registry audit and indexed-shadow
  diagnostic adapter setup.
- Direct production retained publication artifacts are consumed by direct
  publication output helpers, but R7C leaves output parity and producer
  authority as R7D/R7E-R7H scope.

## Ran

Focused tests:

```text
cargo test -p openwepp-runner r7c -- --nocapture
```

Result: pass, `2 passed`.

```text
cargo test -p openwepp-runner \
  r2a_default_fixture_run_constructs_no_direct_runtime_skeleton -- --nocapture
```

Result: pass, `1 passed`.

```text
cargo test -p openwepp-runner r6j -- --nocapture
```

Result: pass, `4 passed`.

Same-binary release benchmark evidence:

```text
/usr/bin/time -f 'release_build\t%e\t%M' \
  cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: pass, `54.09 s / 1123404 KB`.

```text
sha256sum target/release/openwepp-cli-hill \
  target/release/openwepp-cli-hill.json
```

Result: pass.

```text
env -u OPENWEPP_DIRECT_RUNTIME_AUDIT -u OPENWEPP_INDEXED_SHADOW_AUDIT \
  /usr/bin/time -f 'r7c_h2637_default_rep1\t%e\t%M' \
  target/release/openwepp-cli-hill ... --legacy-sidecar-discovery
```

Result: pass, `642.77 s / 228804 KB`, known
`MOFE01-MG-W-001` sidecar warning.

```text
env -u OPENWEPP_DIRECT_RUNTIME_AUDIT -u OPENWEPP_INDEXED_SHADOW_AUDIT \
  /usr/bin/time -f 'r7c_h2637_direct_production_rep1\t%e\t%M' \
  target/release/openwepp-cli-hill ... --direct-production-executor
```

Result: pass, `753.76 s / 625132 KB`.

Full closure gates:

```text
cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings
```

Result: pass.

```text
cargo test --workspace
```

Result: pass.

```text
cargo deny check
```

Result: pass.

Documentation and diff checks:

```text
markdown-doc lint --path docs/architecture/array-native-runtime-specification.md \
  --path docs/work-packages/README.md \
  --path docs/work-packages/20260622-r7c-production-direct-executor-path-001 \
  --no-ignore
```

Result: pass.

```text
git diff --check
```

Result: pass.

Discarded run:

- An earlier parallel invocation of the R2A and R6J focused tests raced on a
  generated binary sidecar JSON in `target/debug/deps` and produced an EOF
  parse failure in the R2A test. The same R2A command passed when rerun
  serially. The failed parallel run is not package evidence.
