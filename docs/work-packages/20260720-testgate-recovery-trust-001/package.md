# TESTGATE Recovery Trust And Qualification Readiness

Package ID: `20260720-testgate-recovery-trust-001`

Queue ID: `TESTGATE-RECOVERY-TRUST-01`

Status: `ACTIVE / SCAFFOLD`

Authorization: Roger Lew's 2026-07-20 direction to scaffold and execute
`TESTGATE-RECOVERY-TRUST-01` after independent review held the predecessor.

This defect-closure ExecPlan is maintained under `docs/codex_exec_plans.md` and
`docs/defect_closure_execplans.md`. Its progress, discoveries, decisions, and
outcomes remain current throughout execution.

## Purpose

Close the trust, persistence, reporting, and combined-execution defects that
prevent `TESTGATE-CLOSURE-AUDIT-01` from landing and make frozen qualification
case Q12 impossible. After this package, a HEAVY transition can reuse only
independently admissible evidence, runner-reset evidence has a real durable
consumer path, every admission failure is typed and recorded, and the
qualification controller can exercise both proven-combined and typed-separate
quality paths without editing its frozen subject.

## Progress

- [x] (2026-07-20) Reproduce and independently review the predecessor defects.
- [x] (2026-07-20) Commit scaffold `ee4d9946` before implementation adoption.
- [x] (2026-07-20) Freeze and pass focused tests for RTR-001 through RTR-009.
- [x] (2026-07-20) Implement durable attested recovery, exact audits, lifecycle reconciliation, and combined-selection corrections.
- [ ] Commit and reconcile the exact terminal diff; obtain a fresh `READY` audit.
- [ ] Delegate each selected HEAVY batch once.
- [ ] Complete dual implementation review, finding disposition, dual terminal verification, prompt archival, and final disposition.
## Correction Authority Envelope

This package owns these observed violations end-to-end:

- `RTR-001`: self-hashed ledger/checkpoint files can suppress a HEAVY node
  without an independently admissible receipt or provenance root;
- `RTR-002`: the trusted workflow names a nonexistent path inside a read-only
  runner and restores a ledger without the checkpoint outputs it references;
- `RTR-003`: admission/resume failures can occur before Rust records a HEAVY
  `STARTED` and terminal failure;
- `RTR-004`: LIGHT and HEAVY workflow, job, runner, and attempt claims can drift;
- `RTR-005`: `PACKAGE_ADMISSION` does not reconstruct exact package authority;
- `RTR-006`: early invalid input can escape as a CLI error instead of one
  schema-valid ten-check `BLOCKED` or `INVALID` audit;
- `RTR-007`: the frozen qualification cannot select a proven combined
  full-regression plus LCOV/CRAP path;
- `RTR-008`: Python and Rust ledger hashing disagree for non-ASCII JSON; and
- `RTR-009`: resume copy can follow a dangling final-component symlink.

If the package establishes a reproducible root cause inside this envelope, it
must implement and validate the correction. It may not close as `HOLD` merely
because further implementation or testing remains possible in scope.

## Declared Write Set

- `.github/workflows/testgate-shadow.yml`
- `crates/openwepp-gate-planner/**`
- `gate-policy/v1/**`
- `tests/integration/testgate_*`
- `tests/python/test_testgate.py`
- `tests/fixtures/testgate/**`
- `tools/ci/omarchy-runner/Dockerfile`
- `tools/ci/omarchy-runner/manage.sh`
- `tools/ci/omarchy-runner/README.md`
- `tools/local_ci/**`
- `tools/release/run_adjudicated_crap_gate.sh`
- `tools/release/check_adjudicated_crap.py`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/work-packages/20260720-testgate-pre-heavy-closure-audit-001/**`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/20260720-testgate-workflow-qualify-001/**`
- `Cargo.lock`

No other tracked path is writable without a reviewed package amendment.

## Architecture And Trust Contract

A checkpoint is diagnostic evidence until a verifier can bind it to the exact
node, non-documentation roots, execution context, claims, artifacts, and an
accepted provenance envelope. Pre-receipt evidence must survive reset but may
not suppress execution merely because its JSON self-hashes. Rejection records
must state the precise missing trust, root, reuse, or context reason.

The trusted runner receives a dedicated writable named volume mounted at
`/testgate-history`; registration state stays read-only and ordinary removal
preserves history. The executor mirrors each completed checkpoint and its
declared outputs before continuing. An always-run finalizer indexes the ledger
and recovery tree for upload. Restore independently verifies every indexed byte
before installing it at the original stable recovery paths.

The pre-heavy command always emits its versioned report once it can identify
the submitted transaction. Identity/schema substitution is `INVALID`; an
unmet external prerequisite is `BLOCKED`. The report reconstructs package
validation, exact paths, canonical ten checks, current executor identity,
LIGHT/HEAVY claims, retry ownership, and the immutable execution DAG.

Combined quality is adopted only through an authenticated proof record that
binds three compatible same-host baselines, exact inventory/result parity,
LCOV/CRAP completeness, and both economy thresholds. Without that proof the
planner emits separate nodes and the typed reason. With it, the DAG contains
one instrumented full node and cannot schedule the same full inventory twice.

## Implementation Plan

First commit this package scaffold alone. Then turn each retained predecessor
finding into a failing focused test. Adopt the preserved working-tree prototype
only after comparing it to those tests. Correct checkpoint provenance and
invalidation in `resume.rs` and the verifier; correct durable mirroring in the
executor, helper, workflow, and runner source; make Rust and Python share exact
UTF-8 canonical ledger bytes; bind LIGHT/HEAVY claims and package authority;
and implement the authenticated combined-quality decision.

Run fast exact tests after each correction. Before HEAVY work, commit the
intended closure diff, rebuild and hash the exact binary, generate one fresh
terminal plan, run its LIGHT stage once, and require a reconstructed `READY`
audit. The parent does not run selected HEAVY nodes. The delegated runner runs
them once and retains every checkpoint, receipt, ledger record, and timing.

## Acceptance

- A forged or root-stale checkpoint never suppresses HEAVY; a current accepted
  receipt imports exactly eligible nodes and records every rejection reason.
- Process termination before aggregate receipt leaves durable, indexed
  checkpoint evidence; fresh-run restoration verifies both ledger and outputs.
- The deployed runner source exposes a writable persistent history mount while
  retaining its read-only root and registration state.
- Rust records every admitted HEAVY attempt and terminal outcome; a first
  tooling defect blocks immediately, while one infrastructure retry remains.
- LIGHT and HEAVY claims, executor bytes, plan, roots, package audit, and exact
  paths are inseparable at admission.
- Every audit invocation returns a schema-valid `READY`, `BLOCKED`, or `INVALID`
  artifact with the exact canonical check set whenever transaction identity is
  representable.
- Python-produced non-ASCII ledger records verify in Rust.
- Resume rejects existing files, ordinary symlinks, and dangling symlinks at
  every destination component.
- Authenticated parity/economy proof selects one combined instrumented node;
  absent or failed proof selects separate nodes with a typed reason.
- Focused tests, exact terminal gates, dual review, dual verification, diff
  hygiene, documentation lint, and `.rs` line-count governance all pass.

## Validation And Gates

Focused implementation commands are:

    cargo nextest run -p openwepp-gate-planner --profile quick
    cargo nextest run --test testgate_align_authority_contract --test testgate_ci_executor_contract --profile quick
    .venv/bin/python -m unittest tests/python/test_testgate.py
    bash -n tools/ci/omarchy-runner/manage.sh
    markdown-doc lint --path docs/work-packages/20260720-testgate-recovery-trust-001 --path tools/local_ci/README.md --path tools/ci/omarchy-runner/README.md
    cargo fmt --all -- --check
    git diff --check

The authenticated terminal plan owns final selection. Critical classification
retains full regression, global CRAP, Clippy/deny, authority, and anti-evasion
obligations. Successful current receipts are reused only under the corrected
trust rules; reassurance reruns are forbidden.

## Review And Delegation

Subagent requirement: REQUIRED. This package explicitly authorizes subagent
spawning/delegation to two independent read-only implementation reviewers, one
`comparator_suite_runner` for every selected HEAVY batch, and two independent
read-only terminal verifiers. Reviewers and verifiers return compact findings,
commands, timings, artifact paths, and `PASS`/`HOLD`; the heavy runner may write
only ignored execution/evidence roots. No role may commit, push, dispatch a
workflow, deploy the runner, or edit outside the package.

Every finding is dispositioned as accepted, rejected, deferred, or follow-up.
Accepted findings are fixed and reverified. No undispositioned finding, failed
current-scope gate, 3000+ nonexempt Rust file, or open tooling defect permits
closure.

## Idempotence And Recovery

No live GitHub dispatch or runner deployment is authorized. Source definition
changes are validated hermetically. Every execution attempt uses a fresh
external root and append-only durable ledger. One infrastructure-only retry is
allowed; the same cause recurring opens a blocking defect before another HEAVY
spawn. Failed and invalidated evidence is retained, never overwritten.

## Surprises And Discoveries

- The predecessor persistence fix named a path absent from the read-only container and restored references to outputs already purged from `/t`.
- A digest chain proves mutation, not authorship. Reusable recovery requires a hosted attestation of the exact archive index plus prior-plan/node binding.
- One newest archive must carry every ledger-referenced root; otherwise a later failed attempt silently drops an older accepted candidate.
- A killed Rust process leaves an unmatched admission unless the always-run finalizer or next admission reconciles it.
- Aggregate receipts and pre-receipt checkpoints need distinct lineage fields; forcing both into `prior_receipt_id` made provenance-only recovery schema-invalid.
- The frozen Q12 case revealed two independent controls: the ordinary workflow needs a selector, and policy needs an active reviewed proof chosen before subject freeze.

- The first exact-head quick gate ran 89 of 94 tests before `receipt_verification_reconstructs_identity_dag_inventory_and_artifacts` reached the unchanged 720-second timeout and canceled four tests. The repository-snapshot cohort was already serial, so concurrency was not the cause: the test rebuilt the same immutable normalized plan, complete Nextest inventory, receipt, and artifact bytes twice in one process. The fixture now caches that immutable template while retaining a fresh no-follow artifact workspace for each verification call; no unchanged rerun was attempted.

- Caching the immutable fixture alone did not close the timeout: the exact focused rerun again reached 720 seconds because the acceptance case also performed three identical full planner reconstructions. Production envelope verification still reconstructs independently, but the unit test now invokes the extracted post-reconstruction verifier after one real live reconstruction and checks dishonest attempts at the owning invariant seam. This preserves production fail-closed behavior while bounding repeated test setup.
## Decision Log

- Decision: use a new authenticated package rather than widen the predecessor retroactively. Rationale: package validation correctly rejects authority added after its base. Date/author: 2026-07-20, execution agent.
- Decision: require hosted archive-index provenance for every cross-attempt checkpoint, and additionally verify an aggregate receipt when present. Rationale: self-authored plans, receipts, checkpoints, and ledgers cannot establish authorship. Date/author: 2026-07-20, execution agent.
- Decision: copy the terminal plan before HEAVY, snapshot all referenced roots, and re-attest carried roots in every newest archive. Rationale: this makes pre-receipt and A→B→C recovery self-contained across volume loss. Date/author: 2026-07-20, execution agent.
- Decision: keep `active_combined_quality_proof_id` null until real three-baseline protected-CI evidence is reviewed. Rationale: a fabricated proof would defeat the qualification; the selector and active-policy path land now, while proof collection is a pre-freeze qualification input. Date/author: 2026-07-20, execution agent.
## Outcomes And Retrospective

Implementation is active. Focused Rust, integration, Python, shell, formatting, and diff-hygiene checks are passing. Both independent reviews confirmed the recovery trust corrections; their retained HOLD is limited to an aggregate-receipt black-box case assigned to Q07 and the intentionally absent real Q12 calibration proof, which must be produced and pinned before qualification freeze rather than synthesized in this defect package. Terminal planning and delegated HEAVY evidence remain pending.
