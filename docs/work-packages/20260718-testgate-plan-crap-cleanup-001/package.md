# Close TESTGATE-PLAN-01 CRAP Debt

Package ID: `20260718-testgate-plan-crap-cleanup-001`

Queue ID: `TESTGATE-PLAN-CRAP-01`

Status: `COMPLETE`

Frozen base: `5613bb4d63b38a5c64cca08be6f089999f03987d`

This ExecPlan is a living document maintained under `docs/codex_exec_plans.md`.
Its `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` sections must remain current.

## Purpose / Big Picture

Remove the 12 actionable CRAP rows introduced with the shadow gate planner so
TESTGATE-PLAN-01 can close without weakening its verifier, adding metric
exceptions, or creating unnecessary broad test execution. The work combines
essential adversarial characterization with behavior-preserving decomposition.
Afterward, the fresh adjudicated report must contain zero actionable rows and
the predecessor package may advance from `EXECUTED-BLOCKED-CRAP`.

## Progress

- [x] (2026-07-18) Froze base, scope, protected boundaries, and the retained 12-row baseline.
- [x] (2026-07-18) Classified all 12 rows as production and bound each to essential characterization or decomposition.
- [x] (2026-07-18) Added direct output-confinement, reuse-class, authority-outcome, and consolidated typed-error ledger characterization; rejected branch-matrix inflation.
- [x] (2026-07-18) Reduced every affected function to CRAP at most 30 without semantic drift; fresh global closure has zero actionable rows.
- [x] (2026-07-18) Ran focused checks, dual review/remediation, one terminal fallback sequence, dual verification, and final disposition.

## Authority And Scope

User direction authorizes this package. ADR-0021, the code-quality refactor
guide, the mechanical-refactor guide, and TESTGATE-PLAN-01's final disposition
govern execution. All 12 rows default to `E-PRODUCTION`; this package does not
seek an adjudication or denominator exclusion.

The usual one-module CQR preference is intentionally widened to the single
`openwepp-gate-planner` crate because all rows were introduced by one package,
share planner/receipt identity tests, and form one predecessor closure gate.
Splitting by file would repeat full-workspace and global-CRAP execution four or
five times, contrary to the governing test-economy objective.

## Declared Write Set

- `crates/openwepp-gate-planner/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260717-testgate-plan-shadow-planner-001/**`
- `docs/work-packages/20260718-testgate-plan-crap-cleanup-001/**`

Any other write requires a pre-edit amendment.

## Protected Boundaries

- Preserve every public API, typed error code, schema, identity derivation,
  canonical byte sequence, plan node, trust rule, and fail-closed outcome.
- Preserve exact statement, predicate, short-circuit, and side-effect order.
- Do not change gate policy, thresholds, fixtures, adjudication registry, CI,
  evidence publication, or assurance/campaign state.
- Do not add inline/wildcard exceptions, lint allowances, fallback wrappers, or
  tests whose only assertion is metric inflation.
- Prefer branch/helper extraction for CC reduction. Characterization must assert
  observable typed results and security invariants.

## Baseline

The fresh predecessor report measured 14 raw rows, two existing adjudications,
and 12 actionable rows. They are:

- `ledger.rs`: `verify_predecessor`, `verify_authorizations`,
  `verify_certification_references`, `verify_assurance_replacements`;
- `main.rs`: `write_plan_confined`;
- `planner.rs`: `reconcile_semantics`, `manifest_object_identity`,
  `cargo_configuration_manifest`; and
- `verifier.rs`: `authority_outcome_accepted`, `verify_envelope`, `verify_reuse`,
  `verify_node_reuse`.

The immutable baseline evidence is the predecessor's
[`gate-results.md`](../20260717-testgate-plan-shadow-planner-001/artifacts/gate-results.md),
which records the base `0873bdae960f7f8c76401845acb476750fdd020e`, 14 raw / 2
adjudicated / 12 actionable counts, and historical report SHA-256
`d52a7bb7ec11f6db563b094fab95aaac53e5ee5815c5fef3e968dd4f3e91d8ff`.
The mutable `target/adjudicated-crap/` path now contains this cleanup package's
fresh report against its frozen base.

## Plan Of Work

First, inventory existing tests and identify which security/error branches are
not reached. Add only tests that distinguish accepted from rejected ledger,
artifact, plan, trust, reuse, or configuration behavior. Run the focused crate
suite after the characterization layer is stable.

Second, extract cohesive validation branches into private typed helpers without
changing evaluation order. `verify_envelope` must be decomposed because its CC
exceeds 30 even at full coverage. Planner manifest and execution-context seams
are preferred extraction targets because `planner.rs` also exceeds the
2,000-line warning threshold.

Third, stabilize the source, run focused warnings-denied Clippy and the crate
suite, then obtain two independent static reviews. Accepted findings are fixed
before the terminal sequence. Run the conservative pre-cutover fallback once on
the stable tree: format, workspace Clippy, full Nextest, deny, and fresh global
adjudicated CRAP. Do not repeat a successful broad command for presentation.

## Validation And Acceptance

Focused development gates are:

    cargo fmt --check
    cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings
    cargo nextest run -p openwepp-gate-planner
    git diff --check

Terminal fallback gates are:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check
    bash tools/release/run_adjudicated_crap_gate.sh --base-ref 5613bb4d63b38a5c64cca08be6f089999f03987d

Acceptance requires zero actionable workspace rows, CRAP at most 30 for each
target and extracted helper, unchanged public behavior, no source mutation
during measurement, all terminal gates passing, and complete dual review and
verification evidence.

## Review, Verification, And Delegation

Two independent reviews cover behavior preservation, anti-evasion, test
necessity, branch/side-effect order, eligibility, CRAP closure, and line counts.
Two independent terminal verifiers inspect the final exact tree and Gate
Evidence Non-Deferral Rule. Every finding is dispositioned as `accepted`,
`rejected`, `deferred`, or `follow-up`; accepted findings are fixed.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two read-only reviewer roles, two read-only terminal
verifier roles, and one terminal heavy-gate runner. Expected outputs are compact
findings/verdicts and exact command results for package artifacts. The heavy
runner may write only generated `target/` evidence; reviewers/verifiers have
read-only access.

Files at or above 2,000 lines are WARN and require decomposition rationale and
split intent. Non-generated files at or above 3,000 lines block closure.

## Idempotence And Recovery

Edits are local and behavior-preserving. Focused commands may be repeated only
after affected source changes. The fresh global CRAP run occurs once after
source freeze; if it exposes residual in-scope debt, correct that debt and rerun
only the invalidated gates. No exception or rollback may hide an actionable row.

## Surprises & Discoveries

- Observation: The predecessor's exact source passed full functional testing
  while the metric still found 12 under-covered/high-complexity verifier paths.
  Evidence: predecessor full Nextest passed 2,115 tests; CRAP reported 12
  actionable rows.
- Observation: A zero-coverage static `cargo crap` assessment was a cheap
  complexity ceiling check; helpers at CC 5 or below cannot exceed the
  threshold even before coverage is considered.
- Observation: Boolean aggregation preserved truth values but not the protected
  short-circuit contract. Static review caught the eager evaluation, and the
  implementation returned to ordered checks before terminal measurement.

## Decision Log

- Decision: Treat every row as eligible production behavior and seek no
  adjudication.
  Rationale: The functions govern validation, state transitions, filesystem
  confinement, identity, trust, and reuse; none is pure observability.
  Date/Author: 2026-07-18 / Codex.
- Decision: Use one crate-level package for one quality dimension.
  Rationale: The rows share one closure blocker and splitting would multiply
  expensive global evidence without improving authority separation.
  Date/Author: 2026-07-18 / Codex.
- Decision: Add direct output-confinement, reuse-class, authority-outcome, and
  consolidated typed-error ledger characterization; reject broader branch
  matrices after helpers reached the static CRAP ceiling.
  Rationale: These tests reach meaningful security outcomes without restoring
  the unnecessary-test posture that this campaign is reducing.
  Date/Author: 2026-07-18 / Codex.
- Decision: Keep the 2,250-line `planner.rs` intact in this behavior-preserving
  cleanup and bind the next structural planner package to split manifest/root
  and execution-context collection before adding planner behavior.
  Rationale: The file is WARN, not BLOCK, and an in-package module move adds
  privacy/import churn unrelated to the retained rows.
  Date/Author: 2026-07-18 / Codex.

## Outcomes & Retrospective

TESTGATE-PLAN-CRAP-01 completed without an adjudication, exclusion, policy
change, or broad-test duplication. Three new essential security tests, a
strengthened existing reuse test, and ordered private-helper extraction closed
all 12 retained production rows.
Dual review passed after exact-order remediation. Terminal format, workspace
Clippy, full Nextest (2,118/2,118), cargo-deny, and fresh adjudicated CRAP pass;
CRAP reports 2 raw / 2 existing adjudications / 0 actionable.

TESTGATE-PLAN-01 is complete and TESTGATE-CI-01 is next. The conservative
pre-cutover gates remain authoritative; this package does not authorize gate
reduction, publication, certification, assurance mutation, or cutover.

Revision note (2026-07-18): initial executable scaffold created from direct user
authorization and TESTGATE-PLAN-01's CRAP blocker.
