# Gate Results

Status: complete.

Ran: focused characterization before production refactor:

```bash
cargo test -p openwepp-hillslope-orchestrator fixeddate -- --nocapture
```

Result: exit `0`, `6 passed`.

Ran: focused characterization after production refactor:

```bash
cargo test -p openwepp-hillslope-orchestrator fixeddate -- --nocapture
```

Result: exit `0`, `6 passed`.

Ran: after formatting focused characterization:

```bash
cargo test -p openwepp-hillslope-orchestrator fixeddate -- --nocapture
```

Result: exit `0`, `6 passed`.

Ran: after LCOV and CRAP:

```bash
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001/artifacts/lcov_after.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001/artifacts/crap_after.json
```

Result: all exit `0`; CRAP command emitted the known LCOV unmatched-source
warning.

Required closure gates:

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | exit `0` |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit `0` |
| `cargo test --workspace` | exit `0` |
| `cargo deny check` | exit `0`; advisories, bans, licenses, sources ok |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001 --format json` | exit `0`; `23` files scanned, `0` errors, `0` warnings |
| `git diff --check` | exit `0` |

Gate Evidence Non-Deferral: satisfied for all current-scope gates.
