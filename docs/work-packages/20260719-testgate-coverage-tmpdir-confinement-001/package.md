# Shorten Confined Coverage Temporary Paths

Package ID: `20260719-testgate-coverage-tmpdir-confinement-001`

Queue ID: `TESTGATE-COVERAGE-TMPDIR-01`

Status: `IMPLEMENTED / BLOCKED-ADAPTER-DIGEST / SUCCESSOR ACTIVE`

Authorization: three verified TESTGATE receipts demonstrating Unix socket
`SUN_LEN` failures caused by the coverage runner's deep temporary root, under
Roger Lew's 2026-07-19 adversarial acceptance execution direction.

This is a Defect-Closure ExecPlan governed by
`docs/defect_closure_execplans.md` and `docs/codex_exec_plans.md`.

## Objective

Keep fresh coverage temporary files inside `OPENWEPP_GATE_ARTIFACT_ROOT` while
using its short top-level `tmp` directory instead of the nested
`target/affected-crap/tmp` directory. Preserve output, Cargo target, Nextest
store, evidence, and fail-closed contracts.

## Progress

- [x] (2026-07-19) Reproduced across three local receipts and two independent
  assurance socket fixtures.
- [x] (2026-07-19) Applied the confined-short-temp correction and bound it in the TESTGATE
  executor contract.
- [x] (2026-07-19) Shell syntax, focused Clippy, the 2/2 TESTGATE contract, and
  both formerly failing socket cases pass under the projected short temp root.
- [ ] Run the mechanical critical terminal
  plan.
- [x] (2026-07-19) Terminal planning failed closed before gate execution on
  `GATE-ADAPTER-DIGEST`; the runner edit invalidated both pinned CRAP adapter
  digests as expected.
- [ ] `20260719-testgate-crap-adapter-digest-alignment-001` updates only those
  two derived bindings and owns the final critical execution.
- [ ] Complete dual review and terminal verification.

## Correction Authority Envelope

Allowed correction: when `OPENWEPP_GATE_ARTIFACT_ROOT` is present and already
validated absolute, set `COVERAGE_TMP` to its `tmp` child; retain
`${OUTPUT_DIR}/tmp` for non-executor runs. Update only the existing integration
contract's exact source-string assertions for both branches.

Protected boundaries: no external temp path, output relocation, test skip,
fixture label, coverage scope, package selection, retry, timeout, CRAP rule,
workflow, policy, schema, or production change.

## Declared Write Set

- `tools/release/run_adjudicated_crap_gate.sh`
- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260719-testgate-policy-digest-alignment-001/**`
- `docs/work-packages/20260719-testgate-adversarial-clippy-cleanup-001/**`
- `docs/work-packages/20260719-testgate-assurance-socket-path-portability-001/**`
- `docs/work-packages/20260719-testgate-coverage-tmpdir-confinement-001/**`

## Conversion Rule And HOLD Legitimacy

The runner-level mechanism is reproduced and mechanically correctable. HOLD is
legitimate only if the exact confined path violates executor security or the
mechanical terminal plan exposes a distinct blocker; retain evidence and name
the next owner.

## Gate Plan

1. `bash -n tools/release/run_adjudicated_crap_gate.sh` and exact source/diff
   checks.
2. Focused Clippy and Nextest for `testgate_ci_executor_contract`.
3. Format, Markdown lint, and diff hygiene.
4. Commit, then execute one exact local TESTGATE plan. This gate-runner change
   is expected to classify `CRITICAL`; the planner owns the full inventory. No
   manual broad suite, GitHub dispatch, or forest1 action is authorized.

## Acceptance

- [x] Executor coverage temp is confined to
  `${OPENWEPP_GATE_ARTIFACT_ROOT}/tmp`; standalone fallback is unchanged.
- [x] Output/evidence paths and coverage/CRAP semantics are unchanged.
- [x] Focused contract gates pass with no test weakening.
- [ ] Mechanical terminal receipt passes every selected node.
- [ ] Dual review and dual terminal verification have no open finding.
- [ ] Production Rust line-count governance is `NOT_APPLICABLE`.

## Review And Delegation

Subagent authorization: this package explicitly authorizes two independent
read-only reviewer/verifier roles for confinement, shell safety, exact diff,
test-evasion, receipt integrity, gate economy, non-deferral, HOLD legitimacy,
and terminal disposition.

Subagent requirement: two independent reviewer/verifier roles are required. No
heavy-run subagent is selected; the parent runs the mechanical plan.

## Security Impact

All temporary bytes remain under the executor-confined external work root. The
change only removes unnecessary path depth and must not permit host-global temp
or evidence escape.

## Surprises And Discoveries

- Both independently failing socket fixtures pass together under the new
  projected path, confirming the runner-root mechanism without further fixture
  edits.
- The first terminal attempt ran no gate: policy correctly rejected the stale
  pinned adapter identity after the runner changed.

## Decision Log

- Decision: fix runner temp projection instead of shortening every socket
  fixture. Rationale: repeated fixtures proved the runner path is the common
  cause; per-test churn would mask it.
  Date/author: 2026-07-19, parent agent.

## Outcomes And Retrospective

The systemic correction passes focused validation. Its derived adapter binding
must be aligned before mechanical critical execution can begin.
