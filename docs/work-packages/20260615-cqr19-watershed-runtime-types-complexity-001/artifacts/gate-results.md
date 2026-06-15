# Gate Results

Status: complete.

Ran: completed pre-gate evidence:

```text
cargo test -p openwepp-watershed-orchestrator runtime_input_error_characterizes  PASS
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path .../lcov_before.info  PASS
cargo crap --workspace --lcov .../lcov_before.info --min 0 --format json --output .../crap_before.json  PASS
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path .../lcov_after.info  PASS
cargo crap --workspace --lcov .../lcov_after.info --min 0 --format json --output .../crap_after.json  PASS
```

Required closure gates:

```text
cargo fmt --check                                                                 PASS
cargo clippy --workspace --all-targets -- -D warnings                             PASS
cargo test --workspace                                                            PASS
cargo deny check                                                                  PASS
markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr19-watershed-runtime-types-complexity-001 --format json  PASS
git diff --check                                                                  PASS
```

Ran: `cargo fmt --check` exited `0`.

Ran: `cargo clippy --workspace --all-targets -- -D warnings` exited `0`:

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.96s
```

Ran: `cargo test --workspace` exited `0`; final visible suite and doc-tests
completed successfully, including the new watershed orchestrator tests.

Ran: `cargo deny check` exited `0`:

```text
advisories ok, bans ok, licenses ok, sources ok
```

Ran: markdown-doc lint exited `0`:

```json
{
  "summary": {
    "files_scanned": 22,
    "errors": 0,
    "warnings": 0
  },
  "findings": []
}
```

Ran: `git diff --check` exited `0`.
