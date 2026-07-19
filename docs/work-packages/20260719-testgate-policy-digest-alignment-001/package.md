# Close TESTGATE Policy Digest Drift

Package ID: `20260719-testgate-policy-digest-alignment-001`

Queue ID: `TESTGATE-POLICY-DIGEST-01`

Status: `IN PROGRESS / CRITICAL GATES REQUIRED`

Authorization: the accepted and terminal-confirmed blocker from
`20260719-testgate-adversarial-agent-acceptance-001`, under Roger Lew's
2026-07-19 direction to execute the adversarial acceptance exercise with
accepted findings patched.

This is a Defect-Closure ExecPlan governed by
`docs/defect_closure_execplans.md` and `docs/codex_exec_plans.md`.

## Objective

Close defect `TESTGATE-POLICY-DIGEST-01`: valid current repository input fails
planner admission with `GATE-POLICY-DIGEST-DRIFT` because
`gate-policy/v1/impact-map.json` binds an obsolete digest for the canonical
testing strategy.

## Progress

- [x] (2026-07-19) Reproduced and independently reviewed the exact mismatch.
- [x] (2026-07-19) Updated only the stale `policy_sha256` binding and committed
  the exact correction as `734a7861f25e38a6d1a37ca453905bd607cd779e`.
- [x] (2026-07-19) Completed the focused planner inventory as 57 retained
  passes plus a 5/5 clean-commit rerun, then passed all 15 focused TESTGATE
  integration tests.
- [x] (2026-07-19) Dual review accepted the exact correction but found that
  the focused-only gate plan improperly downgraded the policy-declared
  `CRITICAL` change; findings `TGDA-A-01`, `TGDA-A-02`, `RB-01`, `RB-02`, and
  `RB-03` are accepted.
- [ ] Execute and independently verify the mechanically selected CRITICAL gate
  plan; do not substitute the focused evidence for that plan.
- [ ] Complete dual review, terminal verification, and disposition.

## Correction Authority Envelope

Defect: current strategy SHA-256 is
`02b9033ca5504cf41411695d73be0b3cbe3bbeb71daecfa94c8410911c0973b3`,
while `gate-policy/v1/impact-map.json` binds
`e5a4341832babf04ea7ca79263e7da8c4826b047649e797d82d1e6e24f4ee063`.
Commit `43dc0e8a` changed the strategy without updating its exact binding.

Authorized write set:

- `gate-policy/v1/impact-map.json`
- `docs/work-packages/README.md`
- `docs/work-packages/20260719-testgate-policy-digest-alignment-001/**`

## Declared Write Set

- `gate-policy/v1/impact-map.json`
- `docs/work-packages/README.md`
- `docs/work-packages/20260719-testgate-policy-digest-alignment-001/**`

This canonical planner-readable declaration restates, without broadening, the
authorized write set in the correction envelope above.

Allowed correction: replace only the obsolete `policy_sha256` value with the
SHA-256 of the current canonical strategy bytes. Matcher order, path mappings,
risk, gate definitions, authority suites, schemas, Rust, workflows, tests, and
all other policy fields are protected read-only boundaries.

Acceptance is observable when direct SHA reconstruction matches the JSON field,
the focused planner library target passes, and the three focused TESTGATE
integration contracts pass without a broad suite.

Branch-out boundaries: any mismatch involving another derived field, semantic
mapping, schema, or policy behavior requires prospective envelope amendment and
cannot be silently folded into this mechanical correction.

## Conversion Rule And HOLD Legitimacy

The defect is reproduced, its mechanism is the stale in-envelope digest field,
the canonical strategy bytes define the correct value, the correction is
mechanical and testable, and focused validation is available. The package must
land the correction and may not stop at HOLD for effort or further inspection.

HOLD is legitimate only if the canonical strategy bytes change concurrently,
the JSON field is not the sole stale binding, or required focused evidence is
unavailable. Any HOLD must include a legitimacy audit naming that boundary,
the attempted in-envelope correction, and the next defect owner.

## Intended Gate Plan

The initial focused-only plan was rejected by both independent reviewers.
`gate-policy/v1/impact-map.json` classifies every `gate-policy/` change as
`CRITICAL`, and the canonical strategy makes an impact-map change inherently
critical. The corrected plan is therefore the repository planner's exact
terminal selection over the scaffold base through the correction-and-review
head, executed once by `tools/local_ci/testgate.py`.

The terminal plan, rather than this prose, owns the final gate inventory. It
must include campaign-closure-strength workspace regression, global
adjudicated CRAP, and all specialized/prerequisite gates selected from
`gate-policy/v1/gate-definitions.json`. No manually narrowed substitute is
authorized. No workflow dispatch or runner action is selected; execution is
local and produces an explicitly untrusted local receipt.

## Execution Plan

1. Commit this scaffold before editing policy.
2. Reconstruct both digests and confirm only `policy_sha256` is stale.
3. Apply the one-field correction; prove the JSON diff is exactly one value.
4. Run the intended focused gates once and record commands, counts, and timing.
5. Complete dual independent review. Patch accepted findings and execute the
   mechanically selected critical plan once; do not repeat already-passing
   focused commands separately.
6. Complete terminal verification, archive the prompt, and close the defect. A
   fresh adversarial acceptance
   rerun owns live TESTGATE acceptance.

## Acceptance

- [x] Exact current strategy SHA equals `impact-map.json` `policy_sha256`.
- [x] Policy diff changes exactly the one authorized digest value.
- [x] Planner library and three focused TESTGATE contracts pass.
- [ ] Mechanically selected CRITICAL terminal plan passes with independently
  verified local receipt.
- [ ] No command outside the mechanical terminal plan runs.
- [ ] Dual review and dual terminal verification have no open finding.
- [x] No `.rs` file changes; line-count governance is `NOT_APPLICABLE`.
- [ ] Gate evidence non-deferral and HOLD legitimacy checks pass.

## Review And Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent reviewer/verifier roles for correction
envelope adequacy, exact-diff, digest reconstruction, anti-evasion,
test-economy, gate non-deferral, HOLD legitimacy, and terminal verification;
expected outputs are compact findings, commands, counts, timings, and
PASS/HOLD/FAIL evidence; write access is read-only.

Subagent requirement: two independent reviewer/verifier roles are required. No
heavy-run subagent is selected because no broad gate is authorized.

## Security Impact

The digest is a fail-closed policy identity. Review must prove the correction
restores currency without altering selection behavior or weakening policy.

## Surprises And Discoveries

- The first focused planner invocation ran 57 tests successfully before a
  receipt reconstruction test rejected the uncommitted correction with
  `GATE-COMMITTED-CHECKOUT-NOT-EXACT`; four tests were canceled by fail-fast.
  The correction bytes were already proven exact, so they were committed and
  only the failed plus four canceled cases were rerun. All five passed.
- The focused planner evidence consumed 782.617 seconds for the initial run
  and 933.146 seconds for the five-case clean-commit rerun. This is observed
  test-cost evidence, not authority to expand this mechanical repair into test
  architecture work.
- Both reviewers found that the initial plan contradicted the canonical
  `CRITICAL` classification. This is an accepted closure-blocking package
  defect. Because the required evidence remains producible, HOLD is not
  legitimate and the package continues with the planner-selected gate set.

## Decision Log

- Decision: correct the binding rather than revert queue governance.
  Rationale: the canonical strategy change is accepted authority; the defect is
  its stale derived identity.
  Date/author: 2026-07-19, parent agent.

## Outcomes And Retrospective

The one-field correction restores exact policy identity without altering gate
selection or weakening fail-closed behavior. Focused executable evidence is
complete but cannot close the critical increment. Mechanical critical-plan
execution and terminal closure remain before this package can move from the
active catalog.
