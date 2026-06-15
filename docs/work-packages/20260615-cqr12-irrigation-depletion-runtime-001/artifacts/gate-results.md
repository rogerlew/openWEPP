# Gate Results

Status: complete.

Ran:

| Gate | Result | Notes |
| --- | --- | --- |
| `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/artifacts/lcov_before.info` | exit `0` | before LCOV |
| `cargo crap --workspace --lcov docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/artifacts/crap_before.json` | exit `0` | cargo-crap emitted existing unmatched-LCOV warning |
| `cargo test --test irrig10_irrigation_runtime_kernel_contract cqr12 -- --nocapture` | exit `0` | before production refactor, `15` passed |
| `cargo test --test irrig10_irrigation_runtime_kernel_contract cqr12 -- --nocapture` | exit `0` | after production refactor/formatting, `15` passed |
| `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/artifacts/lcov_after.info` | exit `0` | after LCOV |
| `cargo crap --workspace --lcov docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/artifacts/crap_after.json` | exit `0` | cargo-crap emitted existing unmatched-LCOV warning |
| `cargo fmt --check` | exit `0` | passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit `0` | passed |
| `cargo test --workspace` | exit `0` | passed |
| `cargo deny check` | exit `0` | `advisories ok, bans ok, licenses ok, sources ok` |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001 --format json` | exit `0` | `23` files scanned, `0` errors, `0` warnings |
| `git diff --check` | exit `0` | passed |

Static: all current-scope gates have direct evidence and are not deferred.
