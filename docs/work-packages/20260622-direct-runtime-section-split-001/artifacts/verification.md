# Verification

Evidence class: Ran.

The full workspace gate validates the current working tree, which also contains
the previously completed but uncommitted runner mechanical split. No failures
were observed.

## Commands

```text
cargo check -p openwepp-hillslope-orchestrator
```

Result: passed.

```text
cargo fmt --check
```

Initial result: failed on one formatting-only blank line before the first
`include!` in `direct_runtime.rs`.

```text
cargo fmt
cargo fmt --check
```

Result: passed.

```text
cargo test -p openwepp-hillslope-orchestrator \
  r2a_direct_runtime_source_excludes_compatibility_storage_tokens -- --nocapture
```

Result: `1 passed; 0 failed; 234 filtered out`.

```text
cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture
```

Result: `61 passed; 0 failed; 174 filtered out`.

```text
cargo clippy --workspace --all-targets -- -D warnings
```

Result: passed.

```text
cargo test --workspace
```

Result: passed, including crate tests, integration tests, and doc-tests.

```text
cargo deny check
```

Result: `advisories ok, bans ok, licenses ok, sources ok`.

```text
git diff --check
```

Result: passed with no output.

```text
git diff --cached --check
```

Initial staged result: failed on trailing EOF blank lines in four newly added
section files. Final result after cleanup: passed with no output.

```text
markdown-doc lint --path \
  docs/work-packages/20260622-direct-runtime-section-split-001 --no-ignore
markdown-doc lint --path docs/work-packages/README.md --no-ignore
```

Result: passed.

## Source Evidence

Static symbol and include scans confirmed:

- `src/lib.rs` continues to re-export from `direct_runtime`.
- `direct_runtime.rs` includes the five new section files in order.
- Major public symbols were found in the expected included files.
- The compatibility-token source scan covers all split sections.
