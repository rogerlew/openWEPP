# Close The Adversarial TESTGATE Clippy Blocker

Package ID: `20260719-testgate-adversarial-clippy-cleanup-001`

Queue ID: `TESTGATE-ACCEPT-CLIPPY-01`

Status: `EXECUTED / FAIL-SOCKET-PATH / SUCCESSOR ACTIVE`

Authorization: the verified closure blocker from
`20260719-testgate-policy-digest-alignment-001`, under Roger Lew's 2026-07-19
direction to scaffold and execute the adversarial acceptance exercise with
accepted patches.

This is a Defect-Closure ExecPlan governed by
`docs/defect_closure_execplans.md` and `docs/codex_exec_plans.md`.

## Objective

Close `TESTGATE-ACCEPT-CLIPPY-01`: workspace Clippy rejects the pre-existing
193-line `assert_workflow_and_rollback_contract` helper in
`tests/integration/testgate_ci_executor_contract.rs`. Split the helper along
existing contract boundaries without deleting, weakening, rewording, or
filtering any assertion.

## Progress

- [x] (2026-07-19) Reproduced through mechanical critical receipt
  `1ad770581b147ba8bb8797e431d2a2d81e6395a61a231f03f807f44bd5ee1d6e`.
- [x] (2026-07-19) Extracted three boundary helpers; the diff changes no
  existing assertion line and preserves all assertion/match inventories.
- [x] (2026-07-19) Focused Clippy passes and the integration target passes 2/2.
- [x] (2026-07-19) Executed the 10-node bounded-component terminal plan twice.
  Nine nodes pass; affected CRAP fails on an out-of-scope Unix-socket scratch
  path before the refactored test runs. The second attempt used a short outer
  root and confirmed the remaining length comes from the fixture label.
- [ ] `20260719-testgate-assurance-socket-path-portability-001` owns the exact
  one-line fixture-label correction and current-head terminal evidence.
- [ ] Complete dual review and terminal verification.

## Correction Authority Envelope

Observed defect: `clippy::too_many_lines` reports 193/100 lines at line 58.
The assertions already pass as runtime tests; structure alone is defective.

Allowed correction: extract private test helpers grouped by normal workflow,
verifier/signer, and conservative rollback/release boundaries. Preserve every
source-string assertion, match count, ordering assertion, and call order.

Protected boundaries: no `#[allow]`, lint-threshold change, assertion deletion
or weakening, production/workflow/policy/schema change, ignore/filter, fixture
change, or new test case.

## Declared Write Set

- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260719-testgate-policy-digest-alignment-001/**`
- `docs/work-packages/20260719-testgate-adversarial-clippy-cleanup-001/**`

Writes outside this set require prospective amendment.

## Conversion Rule And HOLD Legitimacy

The failure is reproduced, localized to one helper, and mechanically solvable
inside the envelope. The package must implement the split and may not stop at
HOLD for effort. HOLD is legitimate only if preserving every assertion cannot
satisfy Clippy or the mechanical terminal plan exposes a distinct out-of-scope
defect; any HOLD must name the exact owner and evidence.

## Gate Plan

Fast repair loop:

1. `cargo fmt --check`;
2. `cargo clippy --test testgate_ci_executor_contract -- -D warnings`;
3. `cargo nextest run --test testgate_ci_executor_contract`;
4. assertion inventory comparison, Markdown lint, and `git diff --check`.

After the focused loop passes, run one exact local TESTGATE plan from this
scaffold commit to the clean completion head. The repository planner owns the
terminal inventory and risk; no manual narrowing, extra workspace command,
workflow dispatch, or forest1 action is authorized. A passing focused command
is not repeated separately after documentation-only review edits.

## Execution Plan

1. Commit this scaffold before editing the test.
2. Record the pre-edit assertion and relevant string-literal inventories.
3. Extract helpers without changing assertion operands or source strings.
4. Run the focused loop and prove inventory preservation.
5. Commit the correction/evidence, then run the exact mechanical terminal plan.
6. Complete dual review, terminal verification, prompt archival, and final
   disposition. Return current-head critical evidence to the parent package.

## Acceptance

- [x] The formerly 193-line helper and every extracted helper pass Clippy's
  100-line limit without lint suppression.
- [x] Exact assertion/match/source-string inventory is preserved.
- [x] Focused Clippy and the two-test integration target pass.
- [ ] The mechanical terminal plan passes or truthfully fails on a distinct
  named blocker; no gate is manually omitted.
- [ ] No production, workflow, policy, schema, fixture, or test-selection byte
  changes.
- [ ] Dual review and dual terminal verification have no open finding.
- [ ] Line-count governance for production Rust is `NOT_APPLICABLE`.

## Review And Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only reviewer/verifier roles for
assertion preservation, test-evasion, exact diff, gate economy, receipt
integrity, non-deferral, HOLD legitimacy, and terminal disposition; expected
outputs are compact findings and PASS/HOLD/FAIL verdicts; write access is
read-only.

Subagent requirement: two independent reviewer/verifier roles are required. No
heavy-run subagent is selected; the parent runs the one mechanical plan.

## Security Impact

The test protects runner isolation, queue controls, current-head guards,
receipt attestation, and rollback separation. Refactoring must preserve every
security assertion exactly.

## Surprises And Discoveries

- The correction is smaller than the lint report implied: three helper
  boundaries plus a wrapper are sufficient. No assertion operand or asserted
  source string changes.
- The terminal plan proves the original Clippy blocker is closed: package and
  workspace Clippy both pass. Its affected-CRAP node instead exposed
  `assurance_dossier_build_contract` binding a Unix socket under a scratch label
  too long for nested executor roots. A shorter outer root fixed one socket
  case but not `assure03-release-symlink-preflight`; this is assigned to a
  separate test-only successor.

## Decision Log

- Decision: extract helpers instead of allowing the lint.
  Rationale: a lint waiver would conceal avoidable complexity in a security
  contract test and would not close the critical gate honestly.
  Date/author: 2026-07-19, parent agent.

## Outcomes And Retrospective

The bounded structural correction passes its focused loop and every selected
non-coverage node. This package cannot claim terminal PASS from its two failed
local receipts. The named successor owns the distinct socket-path correction;
dual review and final disposition remain after that evidence returns.
