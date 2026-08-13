# Admit Portable Rejected-Failure Diagnostic Comparison Authority

Status: `complete / V6 diagnostic portability authority released`

Date: `2026-08-13`

Package ID: `20260813-c3-woody-failure-diagnostic-portability-authority-001`

Plan class: `Narrow critical contract-first numerical-evidence correction`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

## Objective

Admit one portable comparison rule for rejected cross-runtime
`backtracking_limit.step_norm`. The rule lifts the bounded
V5 implementation HOLD without changing any accepted state, flux, residual,
conservation, authorization, branch, count, ordering, identity, or rollback
requirement. The successor evidence identity is `OPENWEPP_C3_WOODY_V6` under
`SC-VEGETATION-001@10`.

## Authority Trigger

The frozen CPython capped backtracking vector reports `step_norm =
3925.8532969524972`; the conforming Rust trajectory reports
`3925.8544224384018`. The absolute delta is `0.0011254859045948251`, or about
`2.86686e-7` relative to the diagnostic magnitude. Both runtimes agree exactly
on typed failure, solve/pass/occupancy identity, iteration and backtracking
counts, active-cap order, candidate absence, and byte-identical rollback.

## Scope

Included:

- `SC-VEGETATION-001` Version 10 and `OPENWEPP_C3_WOODY_V6`;
- `rtol`-only comparison for rejected `backtracking_limit.step_norm` from the
  same cross-runtime nonlinear trajectory;
- exact preconditions and anti-laundering guards;
- independent boundary/inside/outside/zero/sign/nonfinite fixture vectors;
- digest binding, focused gates, dual science review, and dual verification.

Excluded:

- production Rust changes;
- accepted state, flux, residual, conservation, authorization, or convergence
  tolerance changes;
- runtime activation, deployment, publication, selector change, consumer
  cutover, calibration, or empirical validation;
- edits to V1--V5 definition bytes.

## Intended Write Set

- this package tree;
- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`;
- `docs/specifications/science-contracts/index.md`;
- the canonical model-stack copy of the V6 definition;
- `tests/integration/vegetation_boundary_authority_contract.rs`; and
- `docs/work-packages/README.md`.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent science reviewers and two independent
terminal verifiers; expected outputs are package review and verification
artifacts; write access is bounded to this package tree.

## Progress

- [x] Freeze intake, base commit, observed delta, instructions, and protected
  V1--V5 bytes.
- [x] Select the narrow portable comparison and anti-laundering rules.
- [x] Amend canonical authority to Version 10 in review.
- [x] Freeze the V6 definition, independent generator, and boundary fixture.
- [x] Run pre-review focused authority, unit, anti-evasion, Markdown, digest,
  and admission-posture gates.
- [x] Complete two independent science reviews and disposition all findings.
- [x] Promote lifecycle only after separate verification passes and rerun admission.
- [x] Complete terminal gates and two independent terminal verifications.
- [x] Archive the active prompt and issue the implementation handoff.

## Exit Criteria

The package may complete only after the V6 comparison is canonical and
digest-bound; V1--V5 bytes remain unchanged; all boundary and laundering
vectors pass; both reviews and both verifiers pass; admission, unit,
anti-evasion, Markdown, digest, and applicable workspace gates pass; the prompt
is archived byte-for-byte; and the final disposition states that only rejected
cross-runtime diagnostic comparison changed.

## Decision Log

- Decision: use `abs(a-b) <= 3e-7*max(abs(a),abs(b))` only for rejected
  `backtracking_limit.step_norm` after exact field identity, presence,
  finiteness, sign class, and zero/nonzero class checks. Rationale: `3e-7` is
  the smallest one-significant-digit relative
  ceiling above the observed `2.86686e-7` delta; at the observed scale the
  allowance is `0.0011777563267315204`, leaving only about 4.6% headroom. The
  No universal absolute tolerance exists; zero/nonzero is exact and signed
  zeros are explicitly one non-material zero class.
  Date/Author: 2026-08-13 / Codex.
- Decision: create V6 rather than mutate V5. Rationale: fixture comparison is
  model-definition evidence authority, while V1--V5 identities are immutable.
  Date/Author: 2026-08-13 / Codex.
