# Gate Results

All listed commands were run from `/home/workdir/openWEPP`.

| Gate | Command | Exit | Result |
|---|---|---:|---|
| Focused runtime-input management tests | `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests::management` | 0 | PASS, 26 passed |
| Focused parser/runtime seam tests | `cargo test --test parser_runtime_seam_integration management_runtime_surface` | 0 | PASS, 10 passed |
| Package clippy loop | `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` | 0 | PASS |
| Before coverage summary | `cargo llvm-cov --workspace --ignore-run-fail --json --summary-only --output-path docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/coverage_before_summary.json` | 0 | PASS |
| Before LCOV | `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/lcov_before.info` | 0 | PASS |
| Before CRAP | `cargo crap --workspace --lcov docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/crap_before.json` | 0 | PASS |
| After coverage summary | `cargo llvm-cov --workspace --ignore-run-fail --json --summary-only --output-path docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/coverage_after_summary.json` | 0 | PASS |
| After LCOV | `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/lcov_after.info` | 0 | PASS |
| After CRAP | `cargo crap --workspace --lcov docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/crap_after.json` | 0 | PASS |
| Format | `cargo fmt --check` | 0 | PASS |
| Workspace clippy | `cargo clippy --workspace --all-targets -- -D warnings` | 0 | PASS |
| Workspace tests | `cargo test --workspace` | 0 | PASS |
| Dependency/license/advisory | `cargo deny check` | 0 | PASS |

Note: `cargo crap` emitted non-fatal unmatched-LCOV warnings while still
producing target-module rows. Target closure used the emitted rows for
`runtime_inputs/01_management.rs`.
