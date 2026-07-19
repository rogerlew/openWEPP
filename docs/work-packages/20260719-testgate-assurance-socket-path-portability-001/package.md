# Shorten The Assurance Socket Test Scratch Path

Package ID: `20260719-testgate-assurance-socket-path-portability-001`

Queue ID: `TESTGATE-ACCEPT-SOCKET-01`

Status: `EXECUTED / FAIL-SYSTEMIC-TMPDIR / SUCCESSOR ACTIVE`

Authorization: the verified out-of-scope blocker from
`20260719-testgate-adversarial-clippy-cleanup-001`, under Roger Lew's 2026-07-19
direction to execute adversarial acceptance with accepted patches.

This is a Defect-Closure ExecPlan governed by
`docs/defect_closure_execplans.md` and `docs/codex_exec_plans.md`.

## Objective

Close `TESTGATE-ACCEPT-SOCKET-01`: the assurance symlink-evasion integration
test constructs a Unix socket path longer than `SUN_LEN` when TESTGATE runs
coverage beneath its isolated artifact root. Shorten only the test scratch
label so the existing socket and rejection assertions execute unchanged.

## Progress

- [x] (2026-07-19) Reproduced in two local terminal receipts, including one
  short-root retry that isolated the remaining fixture-label contribution.
- [x] (2026-07-19) Replaced only the oversized scratch label with `p`.
- [x] (2026-07-19) Passed the exact focused case under the formerly failing
  long `TMPDIR`; focused Clippy, format, and diff hygiene pass.
- [x] (2026-07-19) Executed the 10-node terminal plan. Nine nodes pass; affected
  CRAP progressed past the corrected test, then a different assurance socket
  fixture failed on the same deep coverage `TMPDIR` mechanism.
- [ ] `20260719-testgate-coverage-tmpdir-confinement-001` owns the systemic
  confined-short-temp correction.
- [ ] Complete dual review and terminal verification.

## Correction Authority Envelope

Allowed correction: in
`transition_preflight_rejects_symlink_evasions_before_release_directory`,
replace only temporary scratch label `assure03-release-symlink-preflight` with
`p`. The label is not an asserted operand or production path.

Protected boundaries: no assertion, socket target, fixture content, production
code, workflow, policy, schema, lint, filter, ignore, timeout, or test-selection
change.

## Declared Write Set

- `tests/integration/assurance_dossier_build_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260719-testgate-policy-digest-alignment-001/**`
- `docs/work-packages/20260719-testgate-adversarial-clippy-cleanup-001/**`
- `docs/work-packages/20260719-testgate-assurance-socket-path-portability-001/**`

## Conversion Rule And HOLD Legitimacy

The failure and one-line mechanism are directly reproduced. The package must
apply and verify the correction. HOLD is legitimate only if the exact focused
case still exceeds `SUN_LEN` or the mechanical terminal plan exposes another
distinct out-of-scope defect; any HOLD must retain its failed evidence and name
the next owner.

## Gate Plan

1. Run the exact failing case with `TMPDIR` set to the retained long coverage
   temporary directory.
2. Run focused Clippy for `assurance_dossier_build_contract`.
3. Compare assertion, test, and socket-target inventories; run format,
   Markdown lint, and diff hygiene.
4. Commit the correction/evidence, then execute one exact local TESTGATE plan
   from this scaffold to the clean head. The planner owns the inventory; no
   manual broad suite, GitHub dispatch, or forest1 action is authorized.

## Acceptance

- [x] The exact formerly failing case passes under the retained long `TMPDIR`.
- [x] Assertion, test, and socket-target inventories are unchanged.
- [x] Focused Clippy passes with no suppression.
- [ ] The mechanical terminal plan passes or truthfully names a distinct
  blocker without omitted gates.
- [ ] Only the temporary label and authorized package evidence change.
- [ ] Dual review and dual terminal verification have no open finding.
- [ ] Production Rust line-count governance is `NOT_APPLICABLE`.

## Review And Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only reviewer/verifier roles for
exact one-line diff, assertion and fixture preservation, portability proof,
gate economy, receipt integrity, non-deferral, HOLD legitimacy, and terminal
disposition; expected outputs are compact finding reports and PASS/HOLD/FAIL
verdicts; write access is read-only.

Subagent requirement: two independent reviewer/verifier roles are required. No
heavy-run subagent is selected; the parent runs the mechanical plan.

## Security Impact

The test must continue proving symlink and special-entry rejection before
release-directory creation. Only its non-semantic scratch prefix may change.

## Surprises And Discoveries

- The one-character label passes under the exact retained long coverage
  `TMPDIR`; no runner or test-behavior change is needed.
- Terminal receipt
  `065aec86b96505f794d7f8122a638272b928ad78107c7aabbc07dffa0f377b6a`
  proves this label fix works, then fails in
  `assurance_v2_source_contract::paths_symlinks_and_special_entries_fail_closed`
  on the same `SUN_LEN` mechanism. Further fixture-label churn is rejected;
  the runner's deep `TMPDIR` override is the systemic defect.

## Decision Log

- Decision: shorten the test label rather than weaken isolation or skip the
  case. Rationale: Unix socket portability belongs to the fixture; the gate
  runner's external evidence confinement remains intact.
  Date/author: 2026-07-19, parent agent.

## Outcomes And Retrospective

The one-line portability correction passes focused validation and its exact
case inside terminal coverage. The terminal receipt remains FAIL on a distinct
fixture with the same runner-root cause. The named systemic successor owns the
runner patch; dual review and final disposition remain.
