# TESTGATE Assurance Historical Root Decoupling

Package ID:
`20260727-testgate-assurance-historical-root-decoupling-001`

Status: `COMPLETE`

Execution mode: `package-end-to-end`

## Objective

Close `TESTGATE-ASSURANCE-HISTORICAL-ROOT-001`: a valid assurance report
source/lifecycle transition regenerates the current DRAFT review lock, but gate
policy loading incorrectly requires that mutable current realization root to
equal the registry's immutable historical assessed realization. The mismatch
blocks impact planning before the planner can emit the required pending impact.

## Authority And Observed Failure

The exact full-workspace run
`9d17ef98-c121-4f18-b528-59d33b7afcce` at `2bf1a600` ran 2,299 tests:
2,278 passed, 21 failed, and 43 skipped. All 21 failures share
`GATE-ASSURANCE-ASSESSED-ROOT`, first for
`linear-groundwater-reservoir-recurrence`.

`docs/standards/testing-and-gate-strategy.md` section 13 and ADR-0039 require
ordinary source movement to create campaign impact without moving or
invalidating the historical assessed realization. DRAFT/held reports may
accumulate impact entries. The registry root is therefore the retained
historical assessment anchor; it must not be updated to the mutable current
DRAFT preapproval root.

## Correction Authority Envelope

Included:

- decouple historical registry roots from mutable current review-lock roots
  during policy loading;
- retain registry/catalog report-set equality, schema validation, lifecycle
  authority, principal-role digest, and all other fail-closed checks;
- add positive current-DRAFT divergence coverage and negative structural/root
  authority coverage;
- prove emitted impacts preserve the historical registry root;
- reconcile gate-policy documentation.

Protected:

- do not change `gate-policy/v1/assurance-registry.json` assessed roots;
- do not edit assurance reports, locks, identities, events, or receipts;
- do not weaken report-set, schema, watch, principal, role, currency, or
  release-transfer validation;
- do not fabricate scientific disposition or mark campaign/release currency
  current;
- do not run Harvard or calibration population work.

## Intended Write Set

- `crates/openwepp-gate-planner/src/policy.rs`
- the isolated policy fixture in
  `crates/openwepp-gate-planner/src/executor.rs` only as required to exercise
  the canonical generated review-lock and identity-lock contract
- adjacent policy unit/coverage tests in
  `crates/openwepp-gate-planner/src/pre_heavy_coverage_tests.rs` only if needed
- `tests/integration/testgate_assure_campaign_currency_contract.rs`
- `gate-policy/v1/README.md`
- this package and `docs/work-packages/README.md`
- predecessor assurance/CAL04B package artifacts only for terminal handoff and
  closure after this defect passes

## Implementation Intent

Classification: critical gate-policy correctness repair.

The registry continues to bind the historical `source_root` and
`assessed_realization_root`. Policy loading validates their shape and
association with the registered report but does not demand equality with the
current mutable review lock. Current report lifecycle and resolution authority
remain independently validated from current assurance state. Planner impact
records must continue to carry the historical registry root.

Selected gates:

- pre-fix focused reproducer;
- policy unit and TESTGATE assurance integration suites;
- warnings-denied gate-planner Clippy, fmt, deny, and docs lint;
- exact-head unfiltered full workspace profile;
- dual independent review, finding disposition, dual verification, exact
  write-set and line-count reconciliation.

Coverage/CRAP: `DEFERRED_TO_QUALITY_CI` per ADR-0041.

## Phase Plan

1. Commit this scaffold and pre-implementation intent.
2. Add contract-derived divergence and fail-closed tests.
3. Correct the policy loader and documentation without moving registry roots.
4. Run focused gates and independent review.
5. Run the comparator-owned exact-head full profile.
6. Verify and close this package, then resume assurance/CAL04B terminal
   disposition.

## Acceptance

- Current DRAFT lock divergence no longer prevents policy/planner loading.
- Planner impact records retain the historical assessed realization root.
- Registry/catalog equality and all authority validations remain fail-closed.
- The 21 exact failures pass without registry-root mutation.
- Full workspace, deny, selected Clippy, docs lint, dual review, and dual
  verification pass.

## Review And Delegation

Subagent requirement: REQUIRED. This package explicitly authorizes subagent
spawning/delegation to one bounded implementation worker, two independent
read-only gate-policy reviewers, the `comparator_suite_runner` for heavy gates,
and two read-only terminal verifiers. Expected outputs are compact findings,
test counts, run IDs, and artifact paths. The worker owns only the declared
source/test/doc write set; the primary executor owns package artifacts and
disposition.

## Progress

- [x] Diagnosed the shared historical/current-root equality defect.
- [x] Authored correction authority and intent.
- [x] Added contract-derived divergence and fail-closed coverage.
- [x] Corrected policy loading and documentation.
- [x] Passed focused gates and dual corrected review.
- [x] Passed the exact closure-candidate full workspace gate.
- [x] Completed dual terminal verification and closure.

## Decision Log

- Decision: preserve registry assessed roots and remove only the equality
  requirement against mutable current DRAFT locks.
  Rationale: moving the registry root would erase the baseline that impact
  planning is required to compare against.
  Date/Author: 2026-07-27 / Codex.
- Decision: accept the independent reviewers' fail-closed finding and validate
  current review locks against the canonical schema and current identity-lock
  digest binding. The isolated executor fixture is added to the write set so
  planner tests exercise the real generated-lock contract.
  Rationale: subset deserialization alone did not support the package's
  structural-validation claim.
  Date/Author: 2026-07-27 / Codex.

## Outcomes

The loader now preserves immutable historical assessment roots while
validating current locks against their canonical schemas, exact identity-lock
byte bindings, and report association. Focused gates, corrected dual review,
the exact closure-candidate 2,301-test full workspace profile, and dual
terminal verification pass.
