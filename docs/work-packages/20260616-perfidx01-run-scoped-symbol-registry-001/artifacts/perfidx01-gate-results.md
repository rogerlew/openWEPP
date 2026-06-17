# PERFIDX01 Gate Results

Status: PASS 2026-06-16
Evidence mode: **Ran**

## Focused Iteration Checks

```text
cargo test -p openwepp-kernel-contract
```

Result: PASS (`17 passed`; doc tests `0 passed`).

```text
cargo check -p openwepp-runner --bin openwepp-cli-hill
```

Result: PASS.

```text
RUSTFLAGS='-C force-frame-pointers=yes -C debuginfo=1' cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: PASS.

## Required Closure Gates

```text
cargo fmt --all -- --check
```

Result: PASS.

```text
cargo clippy --workspace --all-targets -- -D warnings
```

Result: PASS.

```text
cargo test --workspace
```

Result: PASS. Cargo exited 0 after workspace integration, unit, and doc tests.

```text
cargo deny check
```

Result:

```text
advisories ok, bans ok, licenses ok, sources ok
```

## Fixture Gates

Completeness audit: PASS. See `perfidx01-registry-and-invariants.md`.

Bit identity and determinism: PASS. See `perfidx01-bit-identity-evidence.md`.

