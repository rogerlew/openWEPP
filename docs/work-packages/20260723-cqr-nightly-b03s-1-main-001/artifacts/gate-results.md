# Gate Results

- Ran: pre-refactor characterization: `cargo test -p openwepp-gate-planner --bin openwepp-gate-plan package_ -- --nocapture` — 3 passed, 0 failed.
- Ran: complete binary unit suite: `cargo test -p openwepp-gate-planner --bin openwepp-gate-plan` — 8 passed, 0 failed.
- Ran: focused executor/CLI integration target: `cargo nextest run --test testgate_ci_executor_contract` — 8 passed, 0 skipped in 38.569 seconds.
- Ran: focused lint: `cargo clippy -p openwepp-gate-planner --bin openwepp-gate-plan --tests -- -D warnings` — PASS.
- Ran: formatting: `cargo fmt --all -- --check` — PASS.
- Ran: diff hygiene: `git diff --check -- crates/openwepp-gate-planner/src/main.rs docs/work-packages/20260723-cqr-nightly-b03s-1-main-001` — PASS.
- Ran: focused coverage: `cargo llvm-cov -p openwepp-gate-planner --bin openwepp-gate-plan --lcov --output-path /tmp/cqr-b03s1-main.lcov` — 8 passed, 0 failed.
- Ran: focused CRAP: `cargo crap --path crates/openwepp-gate-planner --lcov /tmp/cqr-b03s1-main.lcov --format json --output /tmp/cqr-b03s1-main-crap.json` — all target/helper rows at most 4.
- Static: retained aggregate admission status is PASS in `aggregate-admission.json`.

Static: HEAVY, global CRAP, TESTGATE, dual review, and dual verification were not run by this implementation worker.
