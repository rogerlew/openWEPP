# Verification

Evidence class: Static + Ran.

## Focused Checks

Ran:

```text
cargo fmt --check
```

Result: passed.

Ran:

```text
cargo check -p openwepp-runner
```

Result: passed.

Ran:

```text
cargo test -p openwepp-runner --lib hillslope -- --nocapture
```

Result: passed, `107 passed; 0 failed`.

Ran:

```text
cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture
```

Result: passed, `1 passed; 0 failed`.

## Static-Scan Fallout Checks

Initial full workspace test run failed:

```text
cargo test --workspace
```

Failure:

```text
tests/integration/mofe01_per_ofe_state_contract.rs:
runner must call the persistent per-OFE scheduler lifecycle
```

Root cause: source-level test scanned only
`00_runner_intake_and_lane_setup.rs`; the asserted call had moved
byte-identically into `05_runner_execution_and_outputs.rs`.

Fix verification:

```text
cargo test -p openwepp --test mofe01_per_ofe_state_contract -- --nocapture
```

Result: passed, `8 passed; 0 failed`.

Ran:

```text
cargo test -p openwepp --test mofe01_inter_ofe_route_contract -- --nocapture
```

Result: passed, `3 passed; 0 failed`.

## Required Closure Gates

Ran after test fallout fix:

```text
cargo fmt --check
```

Result: passed.

Ran:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

Result: passed.

Ran:

```text
cargo test --workspace
```

Result: passed.

Ran:

```text
cargo deny check
```

Result:

```text
advisories ok, bans ok, licenses ok, sources ok
```

## Diff And Docs Gates

Ran:

```text
git diff --check
```

Result: passed.

Ran:

```text
markdown-doc lint --path docs/work-packages/20260622-runner-intake-lane-setup-mechanical-split-001 --no-ignore
```

Result: passed.

Ran:

```text
markdown-doc lint --path docs/work-packages/README.md --no-ignore
```

Result: passed.
