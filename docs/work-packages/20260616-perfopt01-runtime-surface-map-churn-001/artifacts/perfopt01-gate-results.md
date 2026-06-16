# PERFOPT01 Gate Results

Status: PASS 2026-06-16
Evidence mode: **Ran**

## Focused Checks

```text
cargo test -p openwepp-kernel-contract
```

Result: PASS (`14 passed`; doc tests `0 passed`).

```text
cargo check -p openwepp-runner --bin openwepp-cli-hill
```

Result: PASS.

```text
cargo test -p openwepp-runner --bin openwepp-cli-hill
```

Result: PASS (`0 passed`; binary test target compiled).

## Required Closure Gates

```text
cargo fmt --check
```

Result: PASS.

```text
cargo clippy --workspace --all-targets -- -D warnings
```

Result: PASS.

```text
cargo test --workspace
```

Result: PASS. Cargo exited 0 after workspace integration/unit/doc tests.

```text
cargo deny check
```

Result:

```text
advisories ok, bans ok, licenses ok, sources ok
```

## Fixture Gates

Ran optimized H2637 without UI and with UI to exit 0:

```text
PERFOPT01_AFTER case=h2637 source=p2637 ofe_count=19 elapsed_s=849.86 user_s=849.30 sys_s=0.42 maxrss_kb=235792
PERFOPT01_AFTER case=h2637_with_ui source=p2637 ofe_count=19 elapsed_s=851.40 user_s=850.83 sys_s=0.44 maxrss_kb=236352
```

Bit identity and determinism: PASS, see `perfopt01-bit-identity-and-determinism-evidence.md`.

