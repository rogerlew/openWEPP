# CQR14 CRAP Before

Ran: `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/lcov_before.info`

Ran: `cargo crap --workspace --lcov docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/crap_before.json`

Static: target-file before LCOV summary:

- Lines: `254/426`, `59.62%`
- Functions: `22/38`, `57.89%`

Static: live CQR14 target identity:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `lint_release_directory` | 51 | 25.0 | 0.0 | 650.0 |

Static: highest out-of-scope before row in target file:
`validate_release_sidecar_unlocked`, line 250, CC `19.0`, coverage
`67.44186046511628`, CRAP `31.459079074798446`.

Static: before line counts:

- `crates/openwepp-runner/src/release.rs`: `560`
- `docs/work-packages/README.md`: `547`

Static: before suppression census found
`#[allow(clippy::too_many_lines)]` on `lint_release_directory`.
