# SNOW-SURFACE-EB-03B Terminal Validation Hold Lift

Status: `executed / pass`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md` and `docs/defect_closure_execplans.md`.

## Purpose / Big Picture

SNOW-SURFACE-EB-03A implemented and independently verified the authoritative
active-layer snow thermal correction, but its critical terminal validation
could not finish cleanly. One quick-profile CQR self-test rejected its own
exact-head fixture, and two assurance publication negative matrices exceeded
their configured per-test timeout. This package closes those two validation
defects without changing snow physics, then reruns the complete required
profiles. A complete pass lifts EB-03A's validation hold and admits EB-04;
anything less leaves EB-04 held.

## Objective

Close defects `EB03B-CQR-001` and `EB03B-ASSURE-001` end-to-end, preserve all
fail-closed and exact-identity semantics, obtain complete exact-tree quick and
Critical full results, reconcile EB-03A's terminal disposition, and update the
campaign roadmap/catalog to state whether EB-04 is admitted.

## Correction Authority Envelope

Included:

- reproduce and root-cause the exact-head CQR fixture rejection;
- correct test-fixture identity construction or comparison when the expected
  behavior is already established by quality-observatory authority;
- reproduce and profile the two assurance publication-matrix timeouts;
- decompose or share test fixtures so each negative case remains fail-closed
  while fitting the configured per-test budget;
- preserve complete case inventory through explicit tests or a case manifest;
- run focused regressions, quick, frost if the inherited snow diff remains in
  the terminal tree, and the complete Critical full profile;
- update EB-03A evidence and disposition only from new direct evidence;
- update `docs/ROADMAP.md`, the snow campaign roadmap, and package catalog.

Excluded:

- snow equations, coefficients, selectors, active-layer behavior, or other
  kernel semantics;
- weakening, deleting, ignoring, or increasing timeouts for a failing test;
- CQR metric thresholds, production filters, exception registry, or quality
  debt adjudication;
- assurance trust roots, approval rules, protected-data custody, release
  semantics, or positive publication authority;
- EB-04 factorial execution.

The seven-gate correction bar is satisfied when a defect is reproduced,
reduced to a named in-envelope fixture/test mechanism, backed by existing
authority, preserves fail-closed behavior, has a before/after regression, and
can be validated through the declared profiles. `HOLD` is exceptional and is
not allowed while an in-envelope repair or required validation remains
possible.

## Implementation Intent

Intent: `behavior-preserving validation-infrastructure defect closure`.

This package does not implement, calibrate, or validate new process science.
The inherited EB-03/03A snow implementation remains part of the exact terminal
tree and therefore retains its already-declared critical validation boundary.

## Dependencies

- `docs/work-packages/20260730-snow-surface-eb-03a-active-layer-thermal-coupling-001/`
- `docs/standards/testing-and-gate-strategy.md`
- `tools/local_ci/cqr_quality_evidence.py`
- `tools/local_ci/quality_observatory.py`
- `tests/integration/cqr_quality_evidence_handoff_contract.rs`
- `tests/integration/assurance_v2_publication_contract.rs`
- `crates/openwepp-assurance/`
- `.config/nextest.toml`

## Intended Write Set

- this package directory;
- `tools/local_ci/cqr_quality_evidence.py` and directly coupled quality
  fixture/test files, if required;
- `tests/integration/cqr_quality_evidence_handoff_contract.rs`;
- `tests/integration/assurance_v2_publication_contract.rs` and directly coupled
  assurance test-fixture helpers, if required;
- EB-03A terminal evidence/status artifacts;
- `docs/planning/snow-surface-energy-balance-roadmap.md`;
- `docs/ROADMAP.md`;
- `docs/work-packages/README.md`.

Any production assurance or snow-runtime edit requires a package amendment
before the edit and conservative reclassification.

## Security Impact

Assurance publication is a protected fail-closed boundary. Every refactor must
retain all negative cases and prove that no invalid fixture publishes or
mutates the public destination. Any change to trust roots, protected custody,
approval authority, release transfer, or publication acceptance is forbidden
in this package.

## Phase Plan

1. Record pre-implementation intent, exact inherited dirty-tree identity,
   required reading, failing commands, and named mechanisms.
2. Correct `EB03B-CQR-001` with a focused regression.
3. Correct `EB03B-ASSURE-001` by test-only decomposition or fixture reuse that
   retains the full fail-closed matrix and configured timeout.
4. Run focused tests and source-quality/document checks.
5. Run complete quick and Critical full profiles against the unchanged exact
   terminal tree; run frost because the tree still contains EB-03A snow work.
6. Complete exact-diff reconciliation, dual independent review, finding
   disposition, dual verification, line-count governance, and final
   disposition.
7. If and only if all required evidence passes, close EB-03A's validation hold
   and mark EB-04 admitted but not executed.

## Validation And Acceptance

Required direct evidence:

- both original failing tests reproduce before repair;
- focused CQR handoff test binary passes;
- the complete assurance publication contract binary passes with every
  negative case retained and each test below the configured timeout;
- `cargo fmt --all -- --check` passes;
- warnings-denied workspace Clippy passes;
- `cargo nextest run --workspace --profile quick` completes and passes;
- `cargo nextest run --workspace --profile frost` completes and passes;
- `cargo nextest run --workspace --profile full` completes and passes;
- assurance catalog validation/render checks selected by the terminal diff pass;
- affected Markdown lint and `git diff --check` pass;
- no production physics or assurance authority changed;
- dual reviews and dual verifications accept the terminal evidence.

The full profile is mandatory because this package modifies test
decomposition/execution at a critical campaign boundary and because EB-03A
already selected that profile. A partial inventory, fail-fast cancellation,
timeout increase, filtered run, or isolated pass does not satisfy it.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation to two
independent read-only reviewers and two independent read-only verifiers for
technical, security, evidence, and gate-legitimacy review; expected outputs are
compact findings written to the package review/verification artifacts by the
primary agent.

Subagent requirement: REQUIRED for the complete quick, frost, and Critical full
batch runs. Spawn/delegate a `comparator_suite_runner` for those heavy commands;
expected output is compact metrics plus retained log paths; write access is
read-only except for test-generated temporary files. The primary agent must not
run those heavy batches while the role is available.

## Progress

- [x] (2026-07-30) User authorized scaffold and end-to-end execution.
- [x] (2026-07-30) Pre-scaffold hold and applicable governance inspected.
- [x] (2026-07-30) Required-reading budget measured at 477141 bytes (`WARN`);
  the large package catalog remains Core because package governance requires it.
- [x] (2026-07-30) Reproduced both defects and reduced them to ambient
  optional-QA leakage in a synthetic CQR fixture and oversized serial
  assurance negative matrices.
- [x] (2026-07-30) Implemented bounded corrections; CQR focused tests pass 4/4,
  the 14 decomposed cases pass 14/14, and the complete assurance publication
  binary passes 37/37.
- [x] (2026-07-30) Terminal validation complete: quick 2109/2109, frost
  324/324, and Critical full 2158/2158 pass.
- [x] (2026-07-30) Dual review, finding disposition, dual verification, prompt
  archival, and final disposition complete.

## Surprises & Discoveries

- Observation: EB-03A's science and snow-domain gates already pass; the only
  terminal blockers are validation infrastructure outside snow physics.
  Evidence: EB-03A `artifacts/final-disposition.md` and `artifacts/gate-results.md`.

- Observation: The CQR fixture failed because current production edits
  invalidate one or more adjudication records, which is legitimate optional-QA
  state but unrelated to the synthetic handoff protocol being self-tested.
  Evidence: retained diagnostic receipt reason
  `canonical CRAP registry has invalid adjudications`.

- Observation: Splitting the two assurance matrices into 14 independently
  scheduled cases preserved every assertion and completed all cases in
  557.596 seconds under the unchanged timeout and two-way publication cap.
  Evidence: nextest run `0c91b1f7-b15d-4cc1-afeb-d039d02ff342`.

## Decision Log

- Decision: Name this hold-lift `SNOW-SURFACE-EB-03B` and keep EB-04
  result-bearing factorial work out of scope.
  Rationale: The failures are prerequisites to truthful EB-03A closure, not
  snow experiment results.
  Date/Author: 2026-07-30 / Codex.

- Decision: Do not satisfy assurance timeouts by increasing the timeout.
  Rationale: The configured limit exposed oversized serial matrices; retaining
  each case while decomposing execution preserves stronger localization.
  Date/Author: 2026-07-30 / Codex.

- Decision: Keep the CQR test seam private to the self-test and leave the
  public `intake(args)` path responsible for loading canonical modules.
  Rationale: Production inspection must not accept dependency injection; only
  the explicitly synthetic self-test should isolate ambient adjudication debt.
  Date/Author: 2026-07-30 / Codex.

## Outcomes & Retrospective

Both observed validation-infrastructure defects are corrected without changing
snow physics, assurance authority, quality thresholds, test timeouts, or
publication concurrency. All focused and terminal profiles, dual reviews,
finding disposition, and dual verifications pass. EB-03A's hold is lifted and
EB-04 is admitted for scaffolding.
