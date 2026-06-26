# SNOWDENSITY-05E Gate Results

Evidence mode: Ran.

## Focused Gates

```text
cargo clippy -p openwepp-runner --bin openwepp-snowbench -- -D warnings
```

Result: pass.

```text
cargo test --test snowdensity05e_melt_adjudication -- --nocapture
```

Result: pass, `2 passed; 0 failed`.

```text
cargo test --test snowdensity05d_opt_in_coe_melt -- --nocapture
```

Result: pass, `4 passed; 0 failed`.

## Evidence Commands

```text
cargo build -q -p openwepp-runner --bin openwepp-snowbench
```

Result: pass.

```text
.venv/bin/python tools/snowfreeze_observed/coe_melt_adjudication.py --observations-dir tests/fixtures/snotel_observed/observations --output-dir target/snowdensity05e_coe_melt_adjudication_rerun --snowbench-binary target/debug/openwepp-snowbench
```

Result: pass. Aggregate artifacts copied to `artifacts/snotel-adjudication.*`.
An earlier rerun into `target/snowdensity05e_coe_melt_adjudication` was
terminated after all legacy replay outputs were written but before aggregate
profile refresh; the final clean rerun target above completed.

```text
cargo build -q -p openwepp-runner --bin openwepp-cli-hill
.venv/bin/python tools/snowfreeze_observed/non_snotel_rubric_baseline.py --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/snowdensity05e_non_snotel_rubric_baseline --binary target/debug/openwepp-cli-hill
```

Result: pass. Aggregate artifacts copied to `artifacts/non-snotel-baseline.*`.

## Required Workspace Gates

```text
cargo fmt --check
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

Result: pass: `advisories ok, bans ok, licenses ok, sources ok`.
