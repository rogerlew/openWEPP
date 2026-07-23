# Focused Gate Evidence

Static: the focused pre-commit evidence below was produced from the RTR-044
correction over committed parent `c5e7a93faa5842c99785147a282bd8b11c2ddf47`.
No TESTGATE or HEAVY node was started while producing it.

Ran: the accepted correction was committed as
`51c7e06db1d7a9e2a9f1173f0e287c1168b2df28`. Exact-commit recovery history
reconstruction returned `READY` for 215/215 steps with chain ID
`ad9711222d35627026ef80d20624b1b2816b346665b3dec20c630128d7bf0cdb`;
the retained artifact is `/tmp/rtr044-recovery-51c7e06d.json`.

Ran: RTR-044 closed durably at ledger digest
`b5005a54709e0b415dae3180e3333085bdf5fd8a1c4e4ea851e8cb4ddb3468b6`,
bound to the exact correction commit. Both independent terminal verifiers
returned PASS without running HEAVY or TESTGATE.

Ran: `cargo fmt --all -- --check` passed after canonical formatting.

Ran: `cargo check -p openwepp-gate-planner --all-targets` passed.

Ran: `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings`
passed.

Ran: `cargo test -p openwepp-gate-planner package_validation::tests:: --
--nocapture` passed 14/14. The cases cover exact grammar/status classification,
single authority, sequential prerequisite authority, zero authority,
same-sequence ambiguity, same-commit retroactivity, malformed scaffold,
malformed child amendment, unmet prerequisite, explicit-anchor exclusion,
per-path composition, merge atomicity, inactive/deleted/symlink authority,
exact-path planning state, byte-preserving terminal prompt archival, terminal
directory shadowing, and retention of aggregate authority for unrelated shared
paths formerly named by an inactive package.

Ran: `.venv/bin/python -m unittest tests.python.test_testgate` passed 21/21.

Ran: the focused pre-HEAVY package-chain identity and bound prompt-digest tests
passed 1/1 each. The direct READY audit validation/execution/resume binding test
passed 1/1 in 57.20 seconds.

Ran: `cargo nextest run --test testgate_ci_executor_contract --test
testgate_assure_campaign_currency_contract` passed 11/11 in 76.229 seconds.

Ran: the real B02 history from aggregate base
`ddd0e4aae924b7d9d8eca91b377106676c4d4dcf` through committed HEAD `c5e7a93f`
reconstructed `READY` with the aggregate, both module packages, RTR-043, and
RTR-044 participating. Chain ID:
`9b80add64f5d517a8f365bbd489f46a89bbc30d356d49fb08d7f7a5ec91f4419`.
The retained disposable output was `/tmp/rtr044-b02-c5-final3.json`.

Ran: the complete recovery history from base
`ee4d9946da82252addec764fa7a3fb9a5993434b` through committed HEAD `c5e7a93f`
reconstructed `READY` with chain ID
`8df8334ffb515210f97678ae6db3171f79ce28ade0c398be82d30da8fd2e756a`.
The retained disposable output was `/tmp/rtr044-recovery-c5-final3.json`.

Static: `executor.rs` is 2,999 lines, below the 3,000-line nonexempt closure
threshold. `package_validation.rs`, `planner.rs`, `pre_heavy.rs`, and
`verifier.rs` are 2,000-line WARN surfaces; none exceeds 3,000 lines.

Static: line-WARN decomposition disposition:

- `executor.rs` retains the executor contract plus its large combined artifact
  fixture. It may not grow; the next authorized executor CQR must move that
  fixture into the existing coverage-test module before semantic edits.
- `package_validation.rs` keeps commit-order decisions adjacent to their Git
  evidence and adversarial history fixtures for this trust-boundary correction.
  It may not grow; a follow-on split should move Git/tree primitives and fixture
  builders into path-backed modules without duplicating authority semantics.
- `pre_heavy.rs` retains the ten-check audit state machine and ledger fixtures.
  A follow-on split should move ledger/recovery fixtures into a path-backed test
  module while preserving source attribution.
- `planner.rs` retains policy assembly and reconstruction. A follow-on split
  should isolate request validation/reconciliation from DAG construction.
- `verifier.rs` retains receipt, envelope, and execution-context verification.
  A follow-on split should isolate execution-context verification behind its
  existing typed boundary.

Static: these are bounded split intents, not waivers. Any touched WARN file
still must remain below 3,000 lines, and `executor.rs` has an explicit no-growth
posture.
