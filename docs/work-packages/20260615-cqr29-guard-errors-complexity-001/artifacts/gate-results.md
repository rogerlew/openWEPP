# Gate Results

Status: passed-with-warnings.

Ran:

```text
cargo fmt --check
```

Result: passed.

Ran:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

Result: passed after splitting the private display helper to remove the
intermediate `too_many_lines` finding.

Ran:

```text
cargo test --workspace
```

Result: passed.

Ran:

```text
cargo deny check
```

Result: passed: `advisories ok, bans ok, licenses ok, sources ok`.

Ran:

```text
markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr29-guard-errors-complexity-001 --format json
```

Result: passed: 22 files scanned, 0 errors, 0 warnings.

Ran:

```text
git diff --check
```

Result: passed.

Warn: `cargo crap` emitted 126 LCOV source-map warnings during before/after
metric generation. The target file appeared in LCOV and the target/helpers
closed below CRAP `30`.
