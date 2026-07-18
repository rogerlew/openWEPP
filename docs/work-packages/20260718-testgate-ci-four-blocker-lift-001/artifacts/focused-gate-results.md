# Focused Gate Results

Ran: evidence below distinguishes pre-review evidence from the remediated
implementation tree on 2026-07-18.

- Pre-review `cargo test -p openwepp-gate-planner executor::tests`:
  PASS, 12/12 executor tests. This included real PASS, FAIL plus blocked
  dependent, timeout, spawn failure, mutation-invalid, external Nextest config,
  and combined artifact cases.
- Pre-review fresh combined affected gate:
  `bash tools/release/run_adjudicated_crap_gate.sh --scope affected --package
  openwepp-gate-planner --nextest-profile affected --base-ref HEAD --output-dir
  target/affected-crap-smoke`, with all work/output roots externally selected.
  PASS, 36/36 tests, 0 skipped; adjudicated CRAP raw/actionable counts 0/0;
  production entry count 822; source manifests identical at
  `19e5a2eceb400d9e47c3c4d979cf7d7941d0ebb4ebb2b7baec07617044375345`.
  Retained root:
  `/tmp/openwepp-testgate-four-blocker-final-lJfizstz/target/affected-crap-smoke/`.
- `.venv/bin/python -m unittest tests.python.test_adjudicated_crap_gate
  tests.python.test_testgate_shadow`: PASS, 19/19.
- Remediated focused adversarial runs: PASS for malformed identity, live
  inventory drift, output collision, zero-work, rollback isolation, signal
  termination, per-node mutation stop, partial failed-JUnit derivation in both
  executor and verifier, normalized JUnit reconstruction, and truthful
  FAIL/BLOCKED receipt verification. The latter ran once in 275.58 seconds.
- `cargo test --test testgate_ci_executor_contract`: PASS, 2/2 after behavioral
  rollback proof was added.
- Remediated `cargo clippy -p openwepp-gate-planner --all-targets -- -D
  warnings`: PASS.
- Second-review remediation: shell command-string variants, reconstruction
  workspace/child symlinks, rootless verifier provider, and behavioral global
  escalation tests PASS; targeted clippy PASS after removal of the superseded
  unconfined mutation-reconstruction helper.
- Terminal-clippy remediation: `cargo clippy --test
  testgate_ci_executor_contract -- -D warnings` PASS and `cargo test --test
  testgate_ci_executor_contract` PASS 2/2 after mechanically splitting the
  114-line integration assertion function. No production code changed.
- Terminal-Nextest remediation: focused
  `contradictory_pass_receipts_fail_closed` PASS after restoring the precise
  schema invariant that non-zero-work PASS/PASS_WITH_RETRY requires at least
  one executed item. Truthful FAIL/BLOCKED receipts may still have zero.
- Shell syntax, Python byte-compilation, and `git diff --check`: PASS.
- `markdown-doc lint` over both touched work packages and the work-package
  catalog: PASS, 23 files, 0 errors, 0 warnings.

The combined gate had two non-test publication retries while the external
Nextest/cargo-llvm-cov store contract was corrected. Those failed artifacts are
retained under distinct `/tmp/openwepp-testgate-four-blocker-final-*` roots and
are not cited as closure evidence.

Post-review combined-gate attempts are retained truthfully:

- `/tmp/openwepp-testgate-remediated-ic6Tkk/`: FAIL, because two partial-JUnit
  fixtures omitted the `NEXTEST_V1` discriminator. The corrected focused cases
  passed immediately.
- `/tmp/openwepp-testgate-remediated-jwSeBn/`: FAIL, because package evidence
  documentation changed while source-bound verifier reconstructions were in
  flight. The fail-closed reconstruction verdict was correct.
- `/tmp/openwepp-testgate-remediated-OTxvW1/`: all 42 tests PASS and before/after
  production manifests identical, followed by adjudicated CRAP FAIL on one row:
  `verify_receipt`, 81.8% coverage and CRAP 32.71. The added confinement branch
  was mechanically extracted into a typed helper before retry.

Stable-tree remediated closure:

- `/tmp/openwepp-testgate-remediated-OPSnFC/work/affected-crap/`: PASS, 42/42
  tests, 0 skipped; adjudicated CRAP raw/actionable counts 0/0; production entry
  count 856; identical before/after production manifests at
  `a96b401ca452a28211c952eefed120a2b473ac15fe11e994d1f828eca7397394`.
- After second-review bypass remediation,
  `/tmp/openwepp-testgate-remediated-LOSdRx/work/affected-crap/`: PASS, 44/44
  tests, 0 skipped; adjudicated CRAP raw/actionable counts 0/0; production entry
  count 864; identical before/after production manifests at
  `593a15272f4cfe00adfd0a1dd72c2288c6bfe51b947c5ff3ed8ca7d2717ec556`.
- After the final structural shell-family remediation,
  `/tmp/openwepp-testgate-remediated-e3qODb/work/affected-crap/`: PASS, 44/44
  tests, 0 skipped; adjudicated CRAP raw/actionable counts 0/0; production entry
  count 864; identical before/after production manifests at
  `de9ea86a5989fcaa5ea70ecc6045c0e1fe442f8470370026dac2c38a810ce44f`.
- After closing the shell-like environment-assignment ambiguity,
  `/tmp/openwepp-testgate-remediated-Bh1sE8/work/affected-crap/`: PASS, 44/44
  tests, 0 skipped; adjudicated CRAP raw/actionable counts 0/0; production entry
  count 864; identical before/after production manifests at
  `a1d7421b25ce5871cb38f6dc44f64c63883e4ce70f20f93f10d7ac21df9e0d7e`.
