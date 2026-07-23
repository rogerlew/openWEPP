# TESTGATE Runner Role And Closeout Correction

Package: `20260723-testgate-runner-role-closeout-correction-001`
Status: `COMPLETE`

## Objective

Correct the TESTGATE recovery and closure-audit closeout record so it
distinguishes the retired pre-pivot Omarchy runner from the active forest1
runner and distinguishes self-hosted HEAVY execution from GitHub-hosted
verification and attestation.

## Authority

Roger Lew's 2026-07-23 direction to correct the documentation and closeout
guidance after clarifying that Omarchy is defunct, forest1 is active, and
TESTGATE HEAVY always runs self-hosted.

## Declared Write Set

- `docs/standards/testing-and-gate-strategy.md`
- `gate-policy/v1/impact-map.json`
- `docs/work-packages/README.md`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/20260720-testgate-pre-heavy-closure-audit-001/**`
- `docs/work-packages/20260723-testgate-runner-role-closeout-correction-001/**`

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/work-packages/20260720-testgate-recovery-trust-001/package.md`

## Constraints

- Documentation and the bound policy digest only; no production-code change.
- Do not dispatch TESTGATE or rerun an unchanged expensive gate.
- Preserve historical evidence while explicitly superseding the incorrect
  runner-outage rationale.
- Do not promote a `LOCAL_UNTRUSTED` receipt or claim repository attestation.

## Review Authorization

Subagent authorization: this package explicitly authorizes two independent
read-only reviewers for the corrected runner-role, attempt, trust, and
closeout claims. Expected outputs are concise findings returned to the parent;
reviewers may not edit, commit, push, dispatch, or execute a gate.

## Progress

- [x] Scaffold prospective correction authority before committing corrections.
- [x] Correct the canonical closeout guidance and package evidence.
- [x] Rebind the policy digest and run narrow documentation/policy validation.
- [x] Obtain dual independent documentation review.
- [x] Record final disposition and archive the active prompt.

## Exit Criteria

- Every corrected record identifies Omarchy as the retired runner and forest1
  as the active self-hosted HEAVY runner.
- The guidance identifies GitHub-hosted jobs as verification/attestation
  consumers, not HEAVY executors.
- Run `30002884134` is recorded as canceled during forest1 content-gate
  execution, not blocked by forest1 unavailability.
- Engineering-package closeout remains distinct from receipt trust and
  repository certification.
- Markdown lint, policy binding, diff hygiene, and dual review pass.
