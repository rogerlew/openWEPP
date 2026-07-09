# Gate Results

Evidence label: Static/Ran.

Status: `PASS`

Focused/local gates:

- `cargo fmt -- crates/openwepp-runner/src/bin/openwepp-snowbench.rs` - exit
  `0`.
- `cargo nextest run -p openwepp-runner --bin openwepp-snowbench` - exit `0`,
  `9 tests run: 9 passed, 0 skipped` after the final `Default` derive fix.
- `cargo nextest run --test snowdensity05f_melt_closure_handoff --test snowdensity03_physics_bulk_offline_contract` - exit `0`, `5 tests run: 5 passed, 0 skipped`.
- Focused target coverage/CRAP command in `artifacts/crap-after.md` - exit
  `0`; max target CRAP `13.001854595336077`.
- `git diff --check` - exit `0`.
- `cargo fmt --check` - exit `0`.
- `cargo clippy -p openwepp-runner --bin openwepp-snowbench -- -D warnings`
  - exit `0` after deriving `Default` for `CommonSnowbenchArgs`.
- `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-03-runner-snowbench-001 --format json` - exit `0`, `21` files scanned, `0` errors, `0` warnings.
- `cargo clippy --workspace --all-targets -- -D warnings` - local fallback
  exit `0` while comparator runner was paused; superseded by delegated pass
  below.
- `cargo deny check` - local fallback exit `0`, `advisories ok, bans ok,
  licenses ok, sources ok`; superseded by delegated pass below.
- Output/API identity evidence - `PASS`, see
  `artifacts/numeric-equivalence.md`.

Delegated comparator-runner gates:

- `cargo llvm-cov clean --workspace` - exit `0`, log
  `artifacts/command-01-llvm-cov-clean.log`.
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly-03-final.lcov` - exit `0`, log
  `artifacts/command-02-llvm-cov.log`; underlying `cargo test` reported
  masked failures under `--ignore-run-fail`, including
  `cli03_runner_contract_derived_tests` and
  `erod16_wave1_continuity_fixture_conservation`, and emitted LCOV at
  `/tmp/openwepp-cqr-nightly-03-final.lcov`.
- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-03-final.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-03-final-crap.json` - exit `0`, log
  `artifacts/command-03-crap.log`.
- `cargo nextest run --workspace --profile full` - exit `0`, log
  `artifacts/command-05-nextest.log`, `1512 tests run: 1512 passed (7 slow),
  3 skipped`.
- `cargo clippy --workspace --all-targets -- -D warnings` - exit `0`, log
  `artifacts/command-06-clippy-full.log`.
- `cargo deny check` - exit `0`, log `artifacts/command-07-deny.log`,
  `advisories ok, bans ok, licenses ok, sources ok`.

Heavy full-workspace gates are delegated to `comparator_suite_runner` agent
`019f47a1-adb4-7f52-92bd-56d28e319d14` (`Kant`) against the latest 9-test
worktree.
