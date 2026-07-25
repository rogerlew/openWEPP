# Quality Observatory Merged Coverage

Package ID: `20260724-quality-observatory-merged-coverage-001`

Status: `ACTIVE / READY / ORDER-3`

## Objective

Produce one identity-bound coverage and adjudicated-CRAP report from sequential
`full` and `science-manual` Nextest profiles, proving manual science coverage
is represented rather than reported as false 0% debt.

## Included Scope

- Same-head, same-toolchain, same-feature, same-instrumented-build collection
  for both profiles.
- Separate profile inventories and LLVM profile inputs followed by deterministic
  merge.
- Global adjudicated CRAP from merged LCOV.
- Snowbench coverage regression fixtures.
- Canonical quality manifest, content-derived `quality_evidence_id`,
  provenance, validation, and compact outputs.

## Excluded Scope

- Moving science-manual tests back into routine TESTGATE.
- Individual exception/adjudication of the 18 snowbench rows from failed run
  `30113946779`.
- Production snowbench behavior changes.
- CQR refactoring.

## Declared Write Set

- `.config/nextest.toml`
- `Cargo.toml`
- `tools/local_ci/**`
- `tools/release/**`
- `tests/integration/quality_observatory_*`
- `tests/integration/testgate_*`
- `tests/integration/snowdensity03_physics_bulk_offline_contract.rs`
- `crates/openwepp-runner/tests/**`
- `docs/work-packages/20260724-quality-observatory-merged-coverage-001/**`
- `docs/work-packages/README.md`

## Dependencies

- Order 2 complete at closeout commit `1d7b457603942a15c0d89d66002f64dc32420934`.

## Pre-Implementation Intent

Risk: `CRITICAL`.

Reason: this package changes global coverage configuration and repository-owned
quality collection, inventory, evidence, and verification behavior.

Implementation is limited to:

1. one repository-owned collector/verifier under `tools/local_ci/`;
2. an observational mode in the canonical adjudicated-CRAP evaluator so debt
   remains visible without turning a valid QA observation into a transition
   failure;
3. focused integration/source-contract tests and the minimum Cargo/Nextest
   registration needed to run them;
4. package evidence and catalog disposition.

The collector must use one immutable admitted identity, independently enumerate
`full`, `science-manual`, and canonical nonignored workspace inventories, run
the profiles sequentially in one instrumented target, preserve per-profile raw
inputs locally, derive full-only, science-only, and merged LCOV views, and feed
only merged LCOV to the canonical CRAP evaluator. It must refuse identity,
inventory, JUnit, source, registry, toolchain, feature, coverage-mode, profile,
or instrumented-build drift.

Selected increment gates:

- focused quality-observatory integration and CRAP evaluator tests;
- exact profile/source guards and independent inventory/JUnit reconciliation;
- Rustfmt and warnings-denied Clippy for the registered integration contract;
- Python bytecode compilation and CLI negative-path probes;
- documentation lint for changed package/catalog evidence;
- diff hygiene, write-set reconciliation, prompt state, and line-count
  governance;
- repository-owned quality pre-heavy admission followed by the package-required
  delegated instrumented `full` plus `science-manual` collection;
- two independent read-only measurement reviews, finding disposition, and two
  independent read-only terminal verifications.

The instrumented full-profile execution is the selected exact-head
full-workspace correctness regression for this critical coverage-runner change.
Coverage/CRAP debt remains observational; execution-integrity, inventory, and
evidence-integrity failures are closure-blocking.

## Phase Plan

1. Freeze report identity and inventory/merge contracts.
2. Characterize the failed full-only snowbench observation.
3. Implement sequential profile collection into one confined instrumented root
   and deterministic coverage merge.
4. Run adjudicated CRAP from the merged LCOV and publish compact evidence.
5. Prove profile completeness, snowbench nonzero coverage, stale-input
   rejection, and exact report validation.
6. Reconcile, review, verify, and disposition.

## Exit Criteria

- Both profile inventories are independently enumerated and exactly match
  observed JUnit execution.
- An independently enumerated canonical nonignored workspace inventory is
  bound at the execution head. `full ∪ science-manual` equals that set and
  their intersection is empty unless a later versioned overlap policy
  explicitly replaces this rule. All three set digests and counts enter the
  canonical payload.
- Profile sets are combined by test identity without pretending they are one
  Nextest profile; duplicate execution is visible.
- Current characterization records 2,263 `full` and 36 `science-manual`
  nonignored tests, a disjoint intersection, and a 2,299-test union; final
  execution must recompute rather than assume those counts.
- Merge rejects different heads, toolchains, features, source manifests,
  coverage modes, or instrumented build identities.
- `artifacts/snowbench-full-only-row-ledger.json` reconstructs the exact 18
  snowbench rows from run `30113946779`. Every row is either nonzero in merged
  coverage with a `science-manual` contribution proof or explicitly proven
  legitimately uncovered and retained as real observational debt. No row may
  remain actionable solely because full-only coverage was used.
- Global CRAP consumes only the merged LCOV and reports raw, adjudicated, and
  actionable rows without reclassifying false full-only gaps.
- The canonical payload binds ordered profiles, per-profile inventory
  hashes/counts and JUnit hashes/results, union identity, source/toolchain,
  coverage/CRAP/registry/report digests, runner, workflow, run, and attempt;
  it excludes `quality_evidence_id` and any digest of itself or its containing
  envelope. `quality_evidence_id` is SHA-256 of canonical JSON payload bytes.
  An outer envelope stores the ID, payload, and publication metadata;
  verification reserializes the payload canonically and recomputes the ID.
- Compact adjudicated evidence preserves every raw, adjudicated, and actionable
  row with exact symbol identity, metrics, classification, and source/report
  hashes so downstream filtering and ranking are independently reproducible.
- Published files are limited to `quality-envelope.json`,
  `quality-payload.json`, `run-status.json`, three inventory-summary JSON files,
  two compact JUnit XML files, `adjudicated-crap-report.json`,
  `adjudicated-crap-report.md`, and `coverage-summary.json`. The indexed total
  must be at most 100 MiB. Raw LCOV, `.profraw`, target/build/reconstruction/
  temp trees, and caches remain local-only even when compressed.
- Compact report artifacts bind all inputs and pass independent verification.
- Focused tests, profile source guards, dual review, dual verification, and
  security-impact review pass.

## Security Impact

The merge is fail-closed on identity mismatch or missing profiles. It cannot
silently fall back to full-only coverage.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only measurement reviewers and two read-only terminal verifiers;
expected outputs are profile/inventory, provenance, and report-integrity
artifacts; write access is read-only. Any selected heavy collection must be
delegated to `comparator_suite_runner`.

## Hold Lift

Attempt 3 remains retained failure evidence. Prerequisite package
`20260724-gate-planner-quality-deferral-hold-lift-001` corrected the seven
gate-planner regressions and passed the exact seven, complete coverage-
configured owning crate, dual review, and selected full workspace on clean
committed head `0342b9f8`.

See `artifacts/gate-planner-hold-lift.md`. The predecessor hold is lifted and a
fresh one-process transition was authorized after this evidence was committed.
Attempt 5 then exposed a separate execution-clone compatibility defect: the
required, identity-bound `.venv` symlink was not locally Git-excluded, so three
exact-checkout tests rejected the clone. See
`artifacts/heavy-attempt-05-venv-git-exclude.md`. The scoped correction and
both re-reviews now pass with zero open findings; fresh admission is authorized
after committing the reviewed correction.

Attempt 7 subsequently passed the complete 2,279-test full profile, including
all exact-checkout regressions, then correctly rejected 11 runtime Cargo
artifacts added by assurance publication tests after admission. See
`artifacts/heavy-attempt-07-runtime-cargo-artifacts.md`. Collection is blocked
until that exact runtime consumer is primed and identity-bound before manifest
sealing. The correction and both re-reviews now pass with zero open findings;
attempt 10 then passed the committed admitted-snapshot downstream-consumer
probe with 3/3 tests and preserved all 291 admitted executable rows and the
working-tree identity. See
`artifacts/heavy-attempt-10-runtime-consumer-probe.md`. Fresh collection is
authorized after committing this evidence.

Attempt 11 passed the complete 2,279-test full profile, then ran all 36
science-manual tests and exposed one stale exact-path confinement allowlist.
The Order-2 TESTGATE source contract legitimately names the science binary to
prove profile partitioning; that governance-only test path is not a runtime
consumer. See `artifacts/heavy-attempt-11-science-confinement.md`. This package
amends its write set before implementation for the exact confinement test and
authorizes only the narrow allowlist correction plus negative-path proof. The
correction passes both reviews after closing a security-review `HIGH`
suffix-collision bypass in the older matcher. Exact repository-relative
equality now admits the one governance path and rejects ordinary and nested
unlisted paths. Fresh collection is authorized after committing this evidence.

Attempt 12 then ran all 2,279 full tests and failed only the coverage-only
gate-planner public-audit consumer because it observed transient dirt in the
shared execution checkout. Final observatory identity remained exact. The
gate-planner prerequisite is reopened to move that content-independence proof
onto its own committed no-hardlink clone. See the prerequisite artifact
`artifacts/attempt-12-repository-snapshot-isolation.md`.

The isolated real consumer subsequently passed twice from a clean source and
once while deliberate ambient untracked dirt existed. Both independent
reviews passed with no findings. Fresh collection is authorized after
committing the reviewed correction.
