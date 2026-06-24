# Verification

Status: COMPLETE.

## Focused Gates

Ran:

```bash
cargo test -p openwepp-hillslope-orchestrator --lib r7g_ -- --nocapture
```

Result: PASS. `10` tests passed, including the new
`direct_runtime_r7g_snow` constructor, R4G mutation, and commit tests.

Ran:

```bash
cargo test -p openwepp-runner --lib r7g_direct_production -- --nocapture
```

Result: PASS. `3` tests passed, including the new direct-publication
winter-column snow source-scan test.

Ran:

```bash
cargo test -p openwepp-hillslope-orchestrator --lib r7b_typed_run -- --nocapture
cargo test -p openwepp-hillslope-orchestrator --lib r7b_constructor_type_size_layout_is_bounded -- --nocapture
```

Result: PASS. Size guard printed:
`DirectRunConstructorInputs=72; DirectLaneConstructorInputs=968;
DirectDayConstructorInputs=3120; DirectRunFrame=256; DirectLaneFrame=1184;
DirectDayFrame=11824`.

## Required Closure Gates

Ran:

```bash
cargo fmt --check
```

Result: PASS.

Ran:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Result: PASS.

Ran:

```bash
cargo test --workspace
```

Result: PASS.

Ran:

```bash
cargo deny check
```

Result: PASS: `advisories ok, bans ok, licenses ok, sources ok`.

Ran:

```bash
git diff --check
```

Result: PASS.

## Documentation Gates

Ran:

```bash
wctl doc-lint --path docs/work-packages/README.md --path docs/work-packages/20260623-r7g-snow-lane-migration-001
```

Result: PASS, but the wrapper reported only one file validated because the new
package directory was ignored by its path expansion.

Ran:

```bash
markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260623-r7g-snow-lane-migration-001 --format plain
```

Result: PASS. `9` files validated, `0` errors, `0` warnings.

## Dual Verification

Verifier A checked package exit criteria against implementation and source
scans. Result: PASS.

Verifier B checked focused tests, full gates, Markdown lint, and line-count
governance. Result: PASS.
