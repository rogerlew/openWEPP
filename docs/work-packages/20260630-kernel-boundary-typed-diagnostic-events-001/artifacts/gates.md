# Gates

Evidence class: Ran plus Static.

## Ran

```bash
cargo fmt --check
```

Initial result: failed with rustfmt diffs after implementation.

Action:

```bash
cargo fmt
```

Re-run result after formatting: PASS.

```bash
cargo check -p openwepp-hillslope-orchestrator
```

Result: PASS.

```bash
cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings
```

Result: PASS.

```bash
git diff --check
```

Result: PASS.

```bash
wctl doc-lint --path docs/work-packages/20260630-kernel-boundary-typed-diagnostic-events-001
```

Result: `0 files validated, 0 errors, 0 warnings`. This does not count as a
meaningful Markdown gate because the scoped wrapper did not select files.

## Static

The typed direct-runtime event payloads contain no references to these carrier
types:

- `HillslopeWritebackSurface`
- `HillslopeKernelRequest`
- `KernelWritebackPayload`
- `SymbolRegistry`

The requested TRACE-class survivor files still contain carrier references, as
recorded in [Progress scan](progress-scan.md).

## Not Run

The full closure loop was not run because the package reached a current-scope
progress failure before consumer migration:

- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- authority anti-evasion
- required-suite obligation guard
- Markdown lint
- protected output identity
- trace output identity

Per the work-package gate non-deferral rule, this prevents complete
disposition.
