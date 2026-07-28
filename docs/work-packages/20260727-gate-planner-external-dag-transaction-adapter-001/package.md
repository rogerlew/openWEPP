# Gate Planner External DAG Transaction Adapter

Package ID: `20260727-gate-planner-external-dag-transaction-adapter-001`

Queue ID: `GATE-EXTERNAL-DAG-01`

Status: `FROZEN / SUPERSEDED BY ADR-0043`

ADR-0043 freezes this incomplete planner prerequisite as historical evidence.
Resuming it requires explicit user authorization; no prospective work may infer
authority from its former `ACTIVE` status.

Authorization: the user's 2026-07-27 direction to complete CAL-04B, including
strategy, scaffold rework, delegated review, and finding disposition.

This ExecPlan is maintained under `docs/codex_exec_plans.md`.

## Progress

- [x] Identify the CAL-04B pre-heavy trust defect before population execution.
- [x] Freeze the defect inventory and initial adversarial matrix.
- [x] Commit the prerequisite scaffold at `9bf3f344`.
- [x] Complete first dual scaffold review; both reviewers returned `HOLD`.
- [x] Close every scaffold finding and obtain dual `GO` at `edc102bc`.
- [x] Freeze adversarial fixtures and implement the adapter.
- [ ] Execute focused and campaign-strength gates through canonical admission;
  focused gates pass, canonical heavy admission remains pending review.
- [ ] Complete dual implementation review, dual terminal verification, and
  package closeout.

## Context And Orientation

`crates/openwepp-gate-planner/src/pre_heavy.rs` owns the only valid
`openwepp-pre-heavy-audit-v1` construction. `executor.rs` owns the trusted
same-process transition and source-mutation check. `verifier.rs` owns receipt
verification. Policy schemas live under `gate-policy/v1/`.

CAL-04B's frozen command authority remains
`artifacts/executor-command-plan.csv` and
`artifacts/observed-command-contract.csv` in its package. The adapter imports
those committed controls as data; it does not reinterpret calibration science.
The exact transaction, output-remap, publication, and custody contract is
`artifacts/transaction-contract.md`.

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
  outputs outside the repository and publishes them only through the separate
  authenticated publication protocol;
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
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/execute-prefix.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/observe.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/publish-results.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/prepare.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/native-proof.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/synthetic-gsi.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/retain.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/summarize.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze-verify.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/custody.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/holdout.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/validate.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_execute_prefix.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_external_paths.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_freeze_custody.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_native_proof.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_observe.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_publish_results.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_validate_calibration.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/test_validate_scaffold.py`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/executor/src/bin/reconstruct.rs`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/executor/src/bin/verify_reconstruct.rs`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/executor/src/bin/readiness.rs`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/external-dag-transaction-plan.json`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/executor-command-plan.csv`
- `docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/observed-command-contract.csv`
- `docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/**`

No other path is writable without a prospective amendment and scaffold review.
The three package-local Rust binaries are authorized only to replace hard-coded
derived output roots with required, plan-bound output-root operands; algorithms,
observations, objectives, acceptance, and reconstruction math remain frozen.
The two CSV authority files are authorized only to replace semantic or
incomplete output descriptions with the exact existing produced filenames and
directories required for exhaustive manifests, and to enumerate already-built
package binaries. Command order, source paths, argv, environment, working
directory, inputs, Harvard access, cost class, prerequisites, and all scientific
semantics remain frozen. Every CSV change requires producer-source proof and
dual review before execution.

## Security And Trust Invariants

- The Rust planner is the sole audit authority; package Python never derives,
  validates, or upgrades READY.
- LIGHT execution, audit construction, first HEAVY admission, and HEAVY STARTED
  persistence occur in one trusted process.
- Every STARTED record has exactly one typed terminal record, including
  preflight and audit failures.
- HEAVY appends its audit-, plan-, claims-, attempt-, and root-bound STARTED
  record before audit validation, resume admission, executable checks, or
  subprocess preparation. Admission occurs only after the ledger equals the
  audited head followed by that exact STARTED record.
- The durable ledger must equal the audited head plus the exact current STARTED
  record before subprocess preparation.
- Source checkout bytes remain unchanged throughout a transaction.
- Declared external outputs are regular files below one no-follow attempt root,
  exhaustively inventoried and hash-bound; undeclared bytes fail.
- A later HEAVY stage after an independent process handoff requires a new
  same-process transition and cannot reuse an earlier audit as standalone
  authority.
- No BLOCKED, INVALID, audit-failure, or pre-spawn path may create the Harvard
  opening token or read Harvard content. The second HEAVY transaction creates
  the exclusive token immediately before its first Harvard open, records
  `OPENED_ONCE`, and is never retryable after that state.

## Deliverables And Interfaces

- `gate-policy/v1/schemas/external-dag-plan.schema.json`
- `gate-policy/v1/schemas/external-output-manifest.schema.json`
- `gate-policy/v1/schemas/publication-receipt.schema.json`
- `gate-policy/v1/schemas/external-verifier-attestation.schema.json`
- typed Rust modules `external_dag.rs`, `external_outputs.rs`, and
  `publication.rs`; existing 2,000+ line modules receive only narrow hooks;
- CLI `openwepp-gate-plan run-external-transition` accepting committed plan,
  transaction ID, fresh attempt root, durable ledger, output paths, and
  execution claims;
- CLI `openwepp-gate-plan publish-external-results` accepting a passing
  transaction receipt, exact source/destination manifest, clean destination
  baseline, and publication receipt path;
- independent verifier support for external transaction, output manifest, and
  publication receipts; and
- verifier dispatch capabilities and attestations binding distinct parent
  dispatch IDs, agent task IDs, principals, execution claims, receipt bytes,
  freeze digest, and one non-reusable verifier capability per actor;
- package evidence `implementation-gates.md`, `review-a.md`, `review-b.md`,
  `finding-disposition.md`, `verification-a.md`, `verification-b.md`,
  `line-count-governance.md`, `worker-handoff.md`, and
  `final-disposition.md`.

## Implementation Plan

1. Freeze schema, lifecycle, artifact, publication, and two-transaction
   acceptance fixtures.
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
- fresh exact-head `cargo nextest run --workspace --profile full`;
- `bash tools/release/check_authority_suite_antievasion.sh` and
  `cargo nextest run --test auth11_required_suite_obligation_guards_contract`;
- exact-diff terminal reconciliation;
- two independent implementation reviews and two terminal verifications.

The prior full-workspace receipt is prerequisite history only. Gate-planner,
receipt, verifier, and anti-evasion edits are critical and require a fresh
exact-head campaign-strength full-workspace run.

## Concrete Execution And Expected Results

All commands run from `/home/workdir/openWEPP`.

1. Run `git diff --check` and
   `markdown-doc lint --path
   docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001`;
   expect zero findings.
2. Run `.venv/bin/python -m unittest discover -s
   docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools
   -p 'test_*.py'`; before implementation the newly frozen GED tests must fail
   for their expected typed reasons, and after implementation all tests pass.
3. Run `cargo nextest run -p openwepp-gate-planner` and
   `cargo clippy -p openwepp-gate-planner --all-targets --all-features --
   -D warnings`; expect the exact current focused inventory to pass and zero
   warnings.
4. Run `cargo run -p openwepp-gate-planner --bin openwepp-gate-plan --
   validate-package --repo /home/workdir/openWEPP --base 9bf3f344
   --package
   docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/package.md
   --output <attempt-root>/package-validation.json`; expect `PASS`.
5. Commit the complete intended closure state, construct canonical READY, and
   delegate selected heavy gates once; expect READY plus passing receipts.
6. Run the fresh exact-head full workspace and anti-evasion gates; expect zero
   failures.
7. Obtain two independent implementation reviews, close findings, then obtain
   two independent terminal verifications at one exact commit.

## Dependencies

- authenticated scaffold base `9bf3f344`;
- canonical audit, ledger, receipt, verifier, and package-admission code already
  present in `openwepp-gate-planner`;
- CAL-04B's committed frozen plan/contract and sealed Harvard controls; and
- current Rust/Python toolchains recorded by the terminal plan.

## Decisions And Discoveries

- Decision: a Python-only READY report is prohibited because it duplicates
  policy and cannot authenticate standalone HEAVY.
- Decision: CAL-04B uses two transactions separated by independent freeze
  verification; the second audit is fresh.
- Decision: all scientific execution output remains external until a separate
  publication receipt installs it.
- Discovery: the canonical executor rejects checkout mutation, so repository
  result paths must be remapped, not exempted.
- Discovery: `executor.rs` is already near 3,000 lines; implementation must add
  modules instead of growing it through that threshold.

## Line-Count Governance

Every touched production file at or above 2,000 lines requires explicit
decomposition review. No touched file may end above 3,000 lines. New adapter
modules target at most 1,000 lines each. The package must split a touched
over-limit module before closure rather than recording warning-only debt.

## Outcomes And Retrospective

Populate at closeout with achieved behavior, gate receipts, residual risks,
and reusable lessons. This section is not evidence until terminal disposition.

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
