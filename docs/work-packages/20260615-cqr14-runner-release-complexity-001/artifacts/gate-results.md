# Gate Results

Ran: `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/lcov_before.info` -> pass.

Ran: `cargo crap --workspace --lcov docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/crap_before.json` -> pass with LCOV source-match warnings for test/integration sources.

Ran: `cargo test -p openwepp-runner release::` before production refactor ->
pass, `10` passed.

Ran: `cargo fmt --check && cargo test -p openwepp-runner release::` after
production refactor -> pass, `10` passed.

Ran: refreshed final `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/lcov_after.info` -> pass.

Ran: final `cargo crap --workspace --lcov docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/crap_after.json` -> pass with LCOV source-match warnings for test/integration sources.

Ran: `cargo fmt --check` -> pass.

Ran: `cargo clippy --workspace --all-targets -- -D warnings` -> pass.

Ran: `cargo test --workspace` -> pass.

Ran: `cargo deny check` -> pass: `advisories ok, bans ok, licenses ok, sources ok`.

Ran: `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr14-runner-release-complexity-001 --format json` -> pass, `files_scanned: 22`, `errors: 0`, `warnings: 0`.

Ran: `git diff --check` -> pass.
