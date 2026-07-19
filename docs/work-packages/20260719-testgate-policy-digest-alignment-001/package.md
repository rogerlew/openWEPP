# Close TESTGATE Policy Digest Drift

Package ID: `20260719-testgate-policy-digest-alignment-001`

Queue ID: `TESTGATE-POLICY-DIGEST-01`

Status: `READY`

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
- [ ] Update the one stale policy digest inside the correction envelope.
- [ ] Run the focused policy/planner and TESTGATE contract gates once.
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

Run exactly:

1. pre-edit and post-edit SHA-256 reconstruction plus JSON parsing;
2. `cargo nextest run -p openwepp-gate-planner --lib`;
3. `cargo nextest run --test testgate_align_authority_contract --test testgate_assure_campaign_currency_contract --test testgate_ci_executor_contract`;
4. package/catalog Markdown lint and `git diff --check`.

No full workspace Nextest, Clippy, coverage, CRAP, cargo-deny, campaign,
release, comparator, workflow dispatch, or runner action is selected. Rust and
test bytes do not change. A validation failure is fixed at cause and only its
invalidated focused family reruns.

## Execution Plan

1. Commit this scaffold before editing policy.
2. Reconstruct both digests and confirm only `policy_sha256` is stale.
3. Apply the one-field correction; prove the JSON diff is exactly one value.
4. Run the intended focused gates once and record commands, counts, and timing.
5. Complete dual independent review and terminal verification. Patch accepted
   findings and rerun only invalidated focused checks.
6. Archive the prompt and close the defect. A fresh adversarial acceptance
   rerun owns live TESTGATE acceptance.

## Acceptance

- [ ] Exact current strategy SHA equals `impact-map.json` `policy_sha256`.
- [ ] Policy diff changes exactly the one authorized digest value.
- [ ] Planner library and three focused TESTGATE contracts pass.
- [ ] No broad or unauthorized gate runs.
- [ ] Dual review and dual terminal verification have no open finding.
- [ ] No `.rs` file changes; line-count governance is `NOT_APPLICABLE`.
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

Pending execution.

## Decision Log

- Decision: correct the binding rather than revert queue governance.
  Rationale: the canonical strategy change is accepted authority; the defect is
  its stale derived identity.
  Date/author: 2026-07-19, parent agent.

## Outcomes And Retrospective

Pending execution.
