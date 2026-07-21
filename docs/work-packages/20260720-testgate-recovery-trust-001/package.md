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
- [x] (2026-07-20) Correct the discovered verifier-fixture timeout without relaxing its limit; exact quick passed 94/94.
- [x] (2026-07-20) Complete dual implementation review and disposition every finding.
- [x] (2026-07-21) Correct and independently re-review the RTR-014 root-target gap exposed by exact workspace Clippy.
- [x] (2026-07-21) Correct and focus-qualify RTR-015 through RTR-017 from exact attempt 5.
- [x] (2026-07-21) Commit and independently review the RTR-015 through RTR-017 corrections; dual PASS at `a1c34412`.
- [x] (2026-07-21) Correct and independently review RTR-018 from exact attempt 6; dual PASS at `7ff552dc`.
- [x] (2026-07-21) Correct RTR-019/RTR-020: nested CRAP now consumes executor-qualified configuration and short TMPDIR, and signal/cleanup outcomes are fail-closed.
- [x] (2026-07-21) Mechanically split source-state observation from `executor.rs` for RTR-021; the file is 2,955 lines and focused planner/executor checks pass.
- [x] (2026-07-21) Mechanically split the 101-line CRAP source-contract test for RTR-022; targeted Clippy and the eight-case integration target pass.
- [x] (2026-07-21) Isolated standalone CRAP probes from executor-only inherited environment for RTR-023; regression probes pass under injected outer executor variables.
- [x] (2026-07-21) Corrected the blocked-receipt fixture's stale JUnit artifact set for RTR-024; its direct verifier regression passes.
- [x] (2026-07-21) Aligned CRAP executor TMPDIR validation with the hexadecimal executor namespace for RTR-025; nested contract coverage passes.
- [x] (2026-07-21) Rebound all three CRAP policy adapters to the current driver SHA for RTR-026; focused policy checks pending review.
- [ ] Reconcile the exact terminal diff and obtain a fresh `READY` audit.
- [ ] Delegate each selected HEAVY batch once.
- [ ] Complete dual terminal verification, prompt archival, and final disposition.
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

Audit-discovered extensions remain inside the same lifecycle authority envelope:

- `RTR-010`: the audit substitutes repository-wide Markdown lint for the terminal plan changed-path lint and can block on unrelated historical files after LIGHT passed;
- `RTR-011`: HEAVY `STARTED` mutates the admitted ledger before resume validation rebuilds an audit whose evidence hashes the pre-STARTED ledger; and
- `RTR-012`: `INVENTORY_AND_ARGUMENTS` checks an `EXACT` label and node shape but does not independently enumerate and compare the current inventory.

These defects were discovered by the package own first exact pre-heavy audit. They must be corrected, regression-tested, independently reviewed, and recorded closed before another audit attempt.

The third exact attempt extended the active envelope with two package-owned defects:

- `RTR-013`: independent audit inventory compilation shared the execution target, so HEAVY reused binaries whose compile-time fixture root belonged to a deleted source snapshot; and
- `RTR-014`: the selected workspace Clippy node found package-owned warnings in the gate-planner implementation.

RTR-013 passed focused verification, dual implementation review, and exact terminal Canopy/authority execution. RTR-014 was reopened when exact workspace Clippy found one package-owned root integration target omitted by package-scoped validation; its decomposed helper and stale source assertion correction now pass root-target Clippy, the owning 5-case integration target, and renewed dual review. Durable closure follows the correction commit before the next audit.

Attempt 5 extended the active envelope with three execution-efficiency defects:

- `RTR-015`: executor-injected long `TMPDIR` paths caused three Unix-socket fixtures to fail before their assertions with `SUN_LEN`;
- `RTR-016`: two-way assurance-publication scheduling caused four 720-second timeouts, while the exact four-case serial sweep passed in 267.751 seconds without changing the timeout; and
- `RTR-017`: the attempt finalizer traversed a 33 GB disposable Cargo target after durable closure, requiring operator cutoff after more than nine minutes.

All three package-owned tooling defects passed focused verification and dual review, then closed in the durable ledger at correction commit `a1c34412`. The direct/manual full-profile schedule is explicitly outside the current TESTGATE execution claim and is a blocking pre-freeze intake item for `TESTGATE-WORKFLOW-QUALIFY-01`.

Attempt 6 opened `RTR-018`: standalone package audit correctly admitted the explicit recovery package, but pre-heavy audit rejected the same exact path set merely because it contained two changed `package.md` files. The audit must independently validate every candidate against the exact base and complete path set, admit the unique `READY` authority, and continue to reject zero or multiple admitted authorities.

Attempt 7 opened two execution-envelope defects. `RTR-019` (`GATE-CRAP-NESTED-EXECUTION-CONTRACT-BYPASS`) occurred when nested CRAP replaced the executor short `TMPDIR` and regenerated Nextest configuration, causing three socket-contract failures and bypassing the qualified publication schedule. `RTR-020` (`GATE-CRAP-SIGNAL-ENVELOPE-FALSE-PASS`) occurred when signal termination left CRAP `run-status.json` at PASS/0 despite the authoritative receipt recording signal 15. The correction injects and validates the qualified config/TMPDIR, restricts standalone cleanup to owned `/tmp/owg-crap-*` roots, and records signal/cleanup failures as nonzero FAIL statuses. Focused regressions pass; durable closure awaits dual review and correction commit.

Attempt 8 opened `RTR-021` (`GATE-EXECUTOR-RUST-FILE-OVER-3000`): pre-HEAVY audit blocked the changed-head attempt because `executor.rs` reached 3,023 lines. The correction is a behavior-preserving source-state helper extraction to `executor_source.rs`; public executor APIs, error codes, and execution logic remain unchanged. The main file is now 2,955 lines, below the hard 3,000-line limit; the remaining 2,000-line WARN is accepted with this extraction as the first decomposition seam and a follow-on test-module split retained for future readability work.

Attempt 9 reached a READY pre-HEAVY audit, then workspace Clippy opened `RTR-022` (`GATE-TEST-CONTRACT-FUNCTION-OVER-100`): `blocking_executor_and_affected_quality_preserve_manual_rollback` was 101 lines, exceeding the enforced 100-line test-function limit. The correction mechanically extracts gate-definition and CRAP-driver assertion helpers without changing assertions or coverage; targeted Clippy and the complete eight-case integration target pass. Doctests, full Nextest, and CRAP did not run in attempt 9 because its terminal plan correctly blocked them after Clippy.

Attempt 10 reached a READY pre-HEAVY audit and passed 13 HEAVY nodes before full Nextest exposed `RTR-023` (`GATE-STANDALONE-CRAP-PROBE-INHERITS-EXECUTOR-ENVIRONMENT`): standalone CRAP tests inherited `OPENWEPP_GATE_ARTIFACT_ROOT` and `OPENWEPP_GATE_NEXTEST_CONFIG` from the executor, unintentionally entering executor mode and failing before their fake Cargo probes. The correction explicitly removes those executor-only variables in standalone test subprocesses and proves the probes pass even when outer executor values are injected. The comparator process then terminated externally before sealing its receipt; retained evidence is preserved at `/home/workdir/testgate-recovery-trust-01-final9.p5rh73`, its ledger is intentionally unreconciled, and CRAP was never started.

Attempt 11 reconciled the interrupted predecessor, then reached a READY audit and passed 13 HEAVY nodes before ordinary full Nextest exposed `RTR-024` (`GATE-VERIFIER-BLOCKED-FIXTURE-STALE-JUNIT`): a blocked-receipt fixture changed the final attempt to `BLOCKED` but retained its original JUnit artifact bytes. The verifier correctly reconstructed that JUnit inventory as executed and rejected the inconsistent receipt. The correction retains the required artifact manifest but replaces the blocked JUnit payload with a valid empty suite and updates its receipt digest before verification. The direct verifier regression passes; CRAP was not run because full Nextest was nonpass.

Attempt 12 passed ordinary full Nextest (2,225/2,225) but opened `RTR-025` (`GATE-CRAP-TMPDIR-HEX-SEQUENCE-REJECTED`): the executor creates `/tmp/owg-<pid>-<hex>` paths while CRAP accepted decimal-only sequence text. The script rejected its own executor's qualified temporary root before coverage began. The correction accepts lowercase hexadecimal sequences, retains ownership/mode/path-budget guards, and proves a hexadecimal temporary path reaches the subsequent config validation seam. CRAP was not rerun.

Attempt 13 opened `RTR-026` (`GATE-CRAP-ADAPTER-DIGEST-DRIFT`) before LIGHT: after the RTR-025 script correction, all three CRAP adapters still bound its older SHA. The package audit was READY, but terminal-plan construction correctly rejected the stale binding without admitting an attempt or running any gate. The correction updates each adapter definition to the current script SHA; no expensive execution was consumed.
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

- Independent re-review caught that the first bounded rewrite no longer invoked public `verify_receipt_envelope` and supplied the plan itself to the extracted equality check. That made the test-local reconstruction comparison tautological even though production behavior was unchanged. The accepted correction makes the single expensive call the public envelope verifier; existing truthful FAIL/BLOCKED tests retain live `verify_receipt` coverage, and cheap negatives remain at their owning invariant seams.

- The first exact pre-heavy audit correctly blocked before HEAVY, but exposed three more tooling defects: its Markdown check ignored the selected node paths, its inventory check trusted labels instead of enumerating, and the HEAVY STARTED append would self-invalidate ledger-content-bound audit reconstruction. The failed audit and original LIGHT receipt are retained under `/home/workdir/testgate-recovery-trust-01.FFQVyI`.

- Dual review of the first RTR-010 through RTR-012 correction found that audit reconstruction compared only nodes, HEAVY did not cheaply revalidate all execution-context identity breakers, and local post-HEAVY verification reconstructed the same plan a third time. The accepted correction binds the complete plan digest, rechecks current context without inventory enumeration, and consumes the READY proof for local receipt validation while retaining independent reconstruction at the external verification boundary.

- Review proved that a self-hashed READY audit was not provenance: a caller could synthesize PASS checks unless audit construction and HEAVY admission shared an unforgeable boundary. Production now carries an opaque constructed-audit capability through one in-process LIGHT-to-audit-to-HEAVY transition; standalone HEAVY rejects audit JSON.

- The in-process transition initially discovered required output paths too late and did not reserve the durable ledger identity. Canonical preflight now validates every required path and rejects canonical aliases among the plan, ledger, LIGHT receipt, audit, and aggregate receipt before LIGHT begins.

- The gate-receipt schema duplicated the embedded audit contract and initially omitted the new ledger-head field. A non-null READY-audit receipt test now keeps the duplicated boundary synchronized.

- Canonical-parent collision checks still missed a final-component symlink on the plan or durable ledger. Transition input preflight now requires existing regular non-symlink inputs and covers both parent traversal aliases and a ledger symlink targeting an output.

- The first in-process exact attempt reached a genuine READY audit, then stopped before HEAVY because recovery scanning treated the current LIGHT/STARTED artifact root as an explicit historical recovery archive. The correction skips ordinary artifact-root records outside the durable recovery namespace while retaining fail-closed rejection for an explicit invalid `recovery_root`. Evidence remains under `/home/workdir/testgate-recovery-trust-01-final.9vt9qp`; the ledger opened `AUTO-6ec4b6897533dd60`.

- The third exact transition passed all six LIGHT nodes and obtained `READY`, then exposed cache contamination in three HEAVY suites: audit reconstruction and execution shared `.work/cargo-target`, so cached binaries retained compile-time `CARGO_MANIFEST_DIR` paths into deleted inventory snapshots. The same run also surfaced 18 package-owned Clippy findings. Evidence remains under `/home/workdir/testgate-recovery-trust-01-final2.ALfL49`; the durable ledger opened `RTR-013` and `RTR-014`. Audit compilation is now isolated under a disposable reconstruction root, and the implementation is Clippy-clean under the selected package command.

- The fourth exact transition confirmed RTR-013 end-to-end: all three Canopy suites and required authority passed from the isolated execution cache. Workspace Clippy then found one remaining 105-line package-owned root integration helper, which the prior package-only Clippy command could not see; doctest, full Nextest, and CRAP were correctly prerequisite-blocked. Evidence remains under `/home/workdir/testgate-recovery-trust-01-final3.4vP6Es`; RTR-014 was reopened rather than treating the incomplete closure as final.

- The fifth exact transition passed every gate through workspace Clippy and doctests, then full Nextest exposed three `SUN_LEN` setup failures and four assurance-publication timeouts. The already-nonpass fail-fast-false batch was cut off after 2,197/2,218 tests ran to avoid roughly another hour on an obsolete subject; its receipt and durable lifecycle were already sealed. The retained attempt root is `/home/workdir/testgate-recovery-trust-01-final4.3vbZS2`. A serial temporary-config sweep passed all four timeout cases in 267.751 seconds under the unchanged ceiling, and the three socket cases passed with a short temp root.

- Attempt-5 finalization then spent more than nine minutes hashing disposable build caches after the receipt and ATTEMPT-CLOSED record existed. The indexing pass was stopped, the new no-follow cache-pruning boundary reduced the retained root from 33 GB to 2.8 MB, and a complete 60-file evidence index was regenerated in 16.93 seconds with receipts and node logs retained.

- The sixth exact transition passed all six LIGHT nodes, then pre-heavy returned `INVALID / GATE-AUDIT-PACKAGE-AMBIGUOUS` while standalone package audit was `READY`. It counted the recovery and workflow-qualification package documents instead of independently checking which one admitted the complete exact diff. No HEAVY lifecycle or node started. The 29-file, 860 KB retained evidence root is `/home/workdir/testgate-recovery-trust-01-final5.mbHSTh`; durable defect `RTR-018` is `69a447f9...`.
## Decision Log

- Decision: use a new authenticated package rather than widen the predecessor retroactively. Rationale: package validation correctly rejects authority added after its base. Date/author: 2026-07-20, execution agent.
- Decision: require hosted archive-index provenance for every cross-attempt checkpoint, and additionally verify an aggregate receipt when present. Rationale: self-authored plans, receipts, checkpoints, and ledgers cannot establish authorship. Date/author: 2026-07-20, execution agent.
- Decision: copy the terminal plan before HEAVY, snapshot all referenced roots, and re-attest carried roots in every newest archive. Rationale: this makes pre-receipt and A→B→C recovery self-contained across volume loss. Date/author: 2026-07-20, execution agent.
- Decision: keep `active_combined_quality_proof_id` null until real three-baseline protected-CI evidence is reviewed. Rationale: a fabricated proof would defeat the qualification; the selector and active-policy path land now, while proof collection is a pre-freeze qualification input. Date/author: 2026-07-20, execution agent.
## Outcomes And Retrospective

Implementation is active. Six retained exact attempts exposed and drove corrections without an unchanged HEAVY rerun. Attempt 5 exposed path-budget, publication-concurrency, and cache-index amplification defects; those corrections pass focused qualification and dual review and are durably closed. Attempt 6 stopped before HEAVY on inconsistent multi-package admission reconstruction; RTR-018 now passes a real Git/schema regression and dual review and is durably closed. A changed-subject terminal attempt, dual terminal verification, and final disposition remain pending.
