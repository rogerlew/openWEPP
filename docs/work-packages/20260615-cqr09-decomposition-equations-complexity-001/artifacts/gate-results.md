# Gate Results

Status: complete.

Ran: baseline and implementation gates:

- `cargo test -p openwepp-hillslope-orchestrator decomposition -- --nocapture`
  before characterization: exit `0`, `4 passed`.
- `cargo test -p openwepp-hillslope-orchestrator decomposition -- --nocapture`
  after characterization before production refactor: exit `0`, `7 passed`.
- `cargo test -p openwepp-hillslope-orchestrator decomposition -- --nocapture`
  after production refactor: exit `0`, `7 passed`.
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`:
  exit `0`.

Ran: metric gates:

- `cargo llvm-cov clean --workspace`: exit `0` before baseline LCOV.
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path
  docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/lcov_before.info`:
  exit `0`.
- `cargo crap --workspace --lcov
  docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/lcov_before.info
  --min 0 --format json --output
  docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/crap_before.json`:
  exit `0`.
- `cargo llvm-cov clean --workspace`: exit `0` before after LCOV.
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path
  docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/lcov_after.info`:
  exit `0`.
- `cargo crap --workspace --lcov
  docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/lcov_after.info
  --min 0 --format json --output
  docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/crap_after.json`:
  exit `0`.

Ran: required closure gates completed so far:

- `cargo fmt --check`: initial exit `1`; rustfmt reported one formatting diff
  in `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/phase.rs`.
- `cargo fmt`: exit `0`.
- `cargo fmt --check`: exit `0`.
- `cargo clippy --workspace --all-targets -- -D warnings`: exit `0`.
- `cargo test --workspace`: exit `0`.
- `cargo deny check`: exit `0`; output `advisories ok, bans ok,
  licenses ok, sources ok`.

Ran: final documentation and diff gates:

- `markdown-doc lint --path docs/work-packages/README.md --path
  docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001
  --format json`: exit `0`; scanned `23` files with `0` errors and `0`
  warnings.
- `git diff --check`: exit `0`.
