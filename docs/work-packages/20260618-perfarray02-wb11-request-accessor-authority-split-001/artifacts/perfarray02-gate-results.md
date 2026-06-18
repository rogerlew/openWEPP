# PERFARRAY02 Gate Results

Evidence: Ran.

## Focused Gates

```text
cargo check -p openwepp-runner
```

Result: pass.

```text
cargo test --test wb14_infiltration_hyetograph_kernel_contract perfarray02_wb14_runoff_reads_from_array_hot_state -- --nocapture
```

Result: pass.

```text
cargo test --test wb14_infiltration_hyetograph_kernel_contract
```

Result before final artifact writing: pass, `17` tests.

```text
cargo test -p openwepp-kernel-contract
```

Result before final artifact writing: pass.

```text
cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract
OPENWEPP_PERFARRAY02_ARRAY_RUNOFF_PILOT=1 cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract
```

Result before final artifact writing: pass.

## Release Build

```text
/usr/bin/time -f 'release_build_final\t%e\t%M' \
  cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: pass, `55.73s`, max RSS `1110732 KB`.

## Required Closure Gates

```text
cargo fmt --check
git diff --check
```

Result: pass.

```text
cargo clippy --workspace --all-targets -- -D warnings
```

Result: pass.

```text
cargo test --workspace
```

Result: pass.

```text
cargo deny check
```

Result:

```text
advisories ok, bans ok, licenses ok, sources ok
```

```text
markdown-doc lint --path docs/work-packages/20260618-perfarray02-wb11-request-accessor-authority-split-001 --path docs/work-packages/README.md --path docs/ROADMAP.md
```

Result: pass, `20 files validated, 0 errors, 0 warnings`.

## Not Run

Source-level anti-evasion guards were not run because this package did not touch
external-authority suite posture, cohort fixtures, or required-case bindings.
