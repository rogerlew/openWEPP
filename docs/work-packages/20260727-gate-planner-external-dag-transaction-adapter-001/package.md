# Gate Planner External DAG Transaction Adapter

Package ID: `20260727-gate-planner-external-dag-transaction-adapter-001`

Queue ID: `GATE-EXTERNAL-DAG-01`

Status: `SCAFFOLD / REVIEW REQUIRED`

Authorization: the user's 2026-07-27 direction to complete CAL-04B, including
strategy, scaffold rework, delegated review, and finding disposition.

This ExecPlan is maintained under `docs/codex_exec_plans.md`.

## Objective

Extend the canonical Rust gate planner so a frozen, repository-owned external
command DAG can execute through authenticated `LIGHT -> READY -> HEAVY`
transactions without mutating the source checkout. Preserve complete external
scientific outputs by manifest and receipt, and support a second authenticated
transition after independent custody handoffs such as CAL-04B's freeze barrier.

## Defect

CAL-04B's package-local Python observer predates the canonical pre-heavy
contract. It can reach population work without a planner-owned READY audit,
cannot produce canonical gate receipts or balanced durable lifecycle records,
and writes result artifacts into the source checkout. A Python audit or
self-hashed audit transport would duplicate policy and remain unauthenticated.

The exact retained reproducer and disposition are in
`artifacts/defect-inventory.md`.

## Included Scope

- a policy-owned external-DAG plan node and versioned schema;
- planner reconstruction of exact command arrays, cost classes, prerequisite
  edges, declared output manifests, timeouts, retry policy, and handoffs;
- same-process LIGHT execution, canonical READY construction, HEAVY STARTED
  admission, execution, and balanced terminal lifecycle;
- no-follow external attempt roots, collision rejection, immutable output
  manifests, binary/toolchain/environment identity, and source-mutation proof;
- exact prerequisite receipt IDs, output digests, DAG position, execution
  claims, and unchanged full READY audit in HEAVY receipts;
- a second authenticated transaction whose LIGHT stage consumes independently
  produced custody receipts before a later HEAVY node;
- CAL-04B package-local integration that writes result-bearing execution
  outputs outside the repository and imports them only after authenticated
  execution;
- focused unit, schema, integration, adversarial, and source anti-evasion
  tests; and
- documentation, dual review, dual verification, and CAL-04B handoff evidence.

## Excluded Scope

- simulation physics, science contracts, calibration domains, observations,
  objectives, acceptance rules, or Harvard data;
- weakening the canonical ten-check audit, trust model, package admission,
  ledger ordering, source mutation, or receipt verification;
- general shell evaluation, caller-synthesized READY documents, or unmanifested
  output preservation;
- running the CAL-04B population or Harvard holdout inside this prerequisite;
- GitHub dispatch, release qualification, or production deployment.

## Declared Write Set

- `crates/openwepp-gate-planner/**`
- `gate-policy/v1/**`
- `Cargo.lock`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/**`
- `docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/**`

No other path is writable without a prospective amendment and scaffold review.

## Security And Trust Invariants

- The Rust planner is the sole audit authority; package Python never derives,
  validates, or upgrades READY.
- LIGHT execution, audit construction, first HEAVY admission, and HEAVY STARTED
  persistence occur in one trusted process.
- Every STARTED record has exactly one typed terminal record, including
  preflight and audit failures.
- The durable ledger must equal the audited head plus the exact current STARTED
  record before subprocess preparation.
- Source checkout bytes remain unchanged throughout a transaction.
- Declared external outputs are regular files below one no-follow attempt root,
  exhaustively inventoried and hash-bound; undeclared bytes fail.
- A later HEAVY stage after an independent process handoff requires a new
  same-process transition and cannot reuse an earlier audit as standalone
  authority.

## Implementation Plan

1. Freeze schema, lifecycle, artifact, and two-transaction acceptance fixtures.
2. Add external-DAG typed plan reconstruction and schema validation.
3. Add confined execution and exhaustive external-output preservation.
4. Integrate canonical audit, ledger, receipts, verifier, and recovery rules.
5. Add the second-transition custody-receipt admission.
6. Rework CAL-04B's coordinator and result publication boundary against the
   new generic interface.
7. Run focused gates, canonical pre-heavy audit, selected heavy correctness
   gates once, dual review, dual terminal verification, and closeout.

## Prospective Gates

- package/schema/diff/documentation/line-count checks;
- red-before-green adversarial fixtures for forged audit, direct HEAVY,
  intervening ledger append, unbalanced STARTED, source mutation, symlink,
  output escape, undeclared output, prerequisite drift, and stale custody;
- gate-planner focused Nextest and warnings-denied Clippy;
- relevant integration and anti-evasion tests;
- canonical pre-heavy READY audit for this package's own selected heavy gates;
- exact-diff terminal reconciliation;
- two independent implementation reviews and two terminal verifications.

Full workspace correctness is campaign evidence already current at
`7e79049d-0871-4142-a9f7-86ac7ac714be`; the authenticated terminal plan decides
whether the implementation diff invalidates and reruns it.

## Exit Criteria

- External DAGs cannot enter HEAVY except through the canonical same-process
  transition.
- Receipts and ledgers satisfy every invariant above and independent
  verification rejects all frozen adversarial cases.
- CAL-04B can represent both its pre-freeze population transaction and its
  post-verifier holdout transaction without source mutation or Harvard custody
  weakening.
- All selected gates pass, findings are closed, dual verification passes, and
  the active prompt is archived.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to bounded implementation workers, two independent
read-only reviewers, two independent read-only terminal verifiers, and the
`comparator_suite_runner` for selected heavy gate execution. Expected outputs
are code/evidence within the declared write set and compact read-only findings,
receipts, artifact paths, and verdicts. Workers receive disjoint ownership and
must not revert concurrent edits.

