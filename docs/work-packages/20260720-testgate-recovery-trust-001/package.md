# TESTGATE Recovery Trust And Qualification Readiness

Package ID: `20260720-testgate-recovery-trust-001`

Queue ID: `TESTGATE-RECOVERY-TRUST-01`

Status: `EXECUTED-COMPLETE`

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
- [x] (2026-07-21) Bounded duplicate receipt-plan reconstruction in RTR-027's blocked-fixture test; direct regression passes.
- [x] (2026-07-22) Complete all seven one-module CQR packages and eliminate the 26 actionable attempt-15 CRAP rows.
- [x] (2026-07-22) Open RTR-031 when cheap final admission correctly rejected retroactive aggregate authority; activate its pre-implementation workflow correction package.
- [x] (2026-07-22) Close RTR-031 with the executable aggregate-admission validator, 17 focused regressions, `READY` package admission, and dual renewed PASS review.
- [x] (2026-07-22) Retain the zero-node pre-admission stop and open RTR-032 because the active intent package was not updated from its scaffold.
- [x] (2026-07-22) Close RTR-032 after exact intent authorization, `READY` package admission, and dual independent review.
- [x] (2026-07-22) Retain the changed-head receipt that exposed two overlong CQR characterization tests and open RTR-033; downstream full Nextest and CRAP did not run.
- [x] (2026-07-22) Close RTR-033 after mechanical 157->42 and 101->75 test splits, exact workspace Clippy, `READY` package admission, and dual review.
- [x] (2026-07-22) Retain the 2,271/2,272 full-Nextest result, open RTR-034 for its stale executor mutability sentinel, and preserve blocked CRAP without running it.
- [x] (2026-07-22) Close RTR-034 after the canonical immutable-binding sentinel, owning 8/8 target, Clippy, `READY` admission, and dual review.
- [x] (2026-07-22) Retain affected CRAP's 1,091/1,091 traversal and open RTR-035 when post-acquisition package admission rejected root `openwepp`; no retry.
- [x] (2026-07-22) Close RTR-035 through RTR-041 after exact-head owning tests,
  a `READY` aggregate audit, and dual independent implementation review.
- [x] (2026-07-22) Retain the zero-node broader-boundary rejection and open
  RTR-042 to bind recovery qualification to `INCREMENT`.
- [x] Reconciled the exact terminal diff and obtained READY audit
  `35729c88...009b8` with all ten checks passing.
- [x] Delegated every selected HEAVY gate through one comparator execution;
  receipt `c22fe3f...f06ca` sealed 15/15 PASS with zero retries.
- [x] Pushed exact head `ba6c1e1d...`; automatic trusted run `29978778150`
  stopped before gate execution because the live runner lacked the reviewed
  `/testgate-history` volume. Opened durable defect RTR-046 at
  `7c4dfaf1...`.
- [x] Activated and dual reviewed the runner contract: exact image, read-only
  root, dedicated writable history mount, ownership, isolation, provider
  identity, and residue checks pass.
- [x] Dispositioned three defunct pre-pivot Omarchy runner records as
  non-blocking historical metadata: zero jobs/artifacts/logs, no scheduler
  exclusion of the newer forest1 push, and no available
  cancel/force-cancel/delete control.
- [x] Obtained the bounded early-deletion exception and attempted only the
  three exact orphan IDs; GitHub refused each deletion with HTTP 403 and
  deleted nothing.
- [x] Retained automatic push run `29979508839`, which reached durable-history
  restore but stopped before gate execution because the runner image lacked
  the required `gh` executable; reopened RTR-046 at `a114c916...`.
- [x] Installed checksum-pinned GitHub CLI 2.96.0, enforced exact-version
  preflight, bound and activated exact image `sha256:8a551a87...`, and
  completed dual implementation review and dual terminal verification.
- [x] Corrected the first CLI closure record's mistyped activation-evidence
  SHA through an explicit append-only reopen/reclose pair; the 157-record
  ledger verifies with RTR-046 CLOSED at `1c36ef0c...` and zero effective
  open defects.
- [x] Retained automatic run `29981856347`, which passed runner/history
  recovery but stopped before planning because `--intent-package` was absent;
  its authenticated archive binds zero TESTGATE nodes.
- [x] Closed RTR-047/RTR-048 after authenticated event-bound package
  resolution, traversal/output-injection rejection, exact active-status
  vocabulary repair, focused validation, dual review, and dual verification.
  The 161-record ledger ends at `5cb57d17...` with zero effective open defects.
- [x] Retained automatic run `29983039718`, which passed runner/toolchain
  admission but stopped before any gate node when the zero-byte ledger produced
  by the prior pre-execution finalizer deadlocked authenticated history restore.
- [x] Close RTR-049 by admitting only the exact safe zero-byte ledger
  placeholder while preserving fail-closed nonempty and unsafe destinations.
- [x] Retain automatic run `29984179443`: LIGHT passed 6/6 and audit was
  `READY`; HEAVY admission stopped before any node on current-root provenance,
  and hosted re-ingest exposed hidden indexed-file omission.
- [x] Close RTR-050 by excluding only the exact current HEAVY STARTED record
  from prior-attempt resume discovery.
- [x] Close RTR-051 by preserving indexed hidden evidence through every
  TESTGATE artifact publication.
- [x] Retain automatic run `29985878363`: authenticated recovery publication
  passed with hidden evidence present, LIGHT passed 6/6, and pre-HEAVY blocked
  solely on the runner-local AUTO defect left open by the prior failed run.
- [x] Close RTR-052/RTR-054 through the reviewed ledger closure command and
  exact failed-root invalidation before one later changed-head attempt.
- [x] (2026-07-23) Accept both initial reviewers' HOLD findings: require exact
  OPEN/CLOSED states, real correction ancestry, nonblank review evidence,
  failed-record/root/cause association, safe-child confinement, and no-follow
  ledger paths.
- [x] (2026-07-23) Mechanically extract the inline `pre_heavy.rs` test module
  to `pre_heavy_tests.rs`; production authority remains unchanged and
  `pre_heavy.rs` falls from 3,116 to 1,863 lines.
- [x] (2026-07-23) Accept the renewed reviewer HOLD findings by requiring the
  resume consumer to mirror prior-OPEN/cause lifecycle and exact correction
  ancestry instead of trusting producer-only validation.
- [x] (2026-07-23) Open RTR-054 when the no-follow guard exposed a
  noncanonical pre-HEAVY coverage-fixture ledger path; construct the fixture
  from the canonical repository root and pass its exact regression.
- [x] (2026-07-23) Obtain dual renewed implementation PASS at exact head
  `94462c30`, close the runner-local AUTO defect with canonical ledger tip
  `2852c7ed`, and close RTR-052 through RTR-054 with campaign-ledger tip
  `50a59cc7`; both ledgers re-hash with zero effective open defects.
- [x] (2026-07-23) Retain automatic run `29990800842`, which stopped in
  trusted comparison admission because exact pushed tip `8e55cb91` omitted
  the mandatory intent-package trailer; no TESTGATE node or retry ran.
- [x] Close RTR-055 with an exact trailer-bearing tip, narrow resolver proof,
  and dual independent review before another changed-head push.
- [x] (2026-07-23) Restore the package to admitted `ACTIVE / REVIEW` after
  package-chain reconstruction correctly rejected the noncanonical temporary
  `ACTIVE / CORRECTION` label.
- [x] (2026-07-23) Close RTR-055 after dual PASS at exact trailer-bearing head
  `35e3d83b`; the campaign ledger now ends at `39c4ee9a` with zero effective
  open defects.
- [x] (2026-07-23) Retain automatic run `29991322951`: one documentation-lint
  node passed with no retry, but hosted verification failed on producer/consumer
  intent-authority field drift; no HEAVY or coverage ran.
- [x] Close RTR-056 by consuming `intent_package_path` in hosted verification.
- [x] Close RTR-057 with a fail-closed exact-head comparison-base declaration
  that can only expand the push diff backward to an ancestor.
- [x] Close RTR-058 by invoking canonical hosted intent-authority
  reconstruction with the exact binary, head, changed paths, package, and
  retained output.
- [x] (2026-07-23) Obtain dual implementation PASS at exact correction head
  `22d3db03`, close RTR-056 through RTR-058 with ledger tip `6246459e`, and
  retain zero effective open defects.
- [x] (2026-07-23) Retain automatic run `29993134526`: LIGHT sealed four PASS,
  formatting FAIL, and one dependency-BLOCKED placeholder without launch;
  pre-HEAVY blocked, and no HEAVY or CRAP node started.
- [x] Cancel the orphaned GitHub scheduler record after the self-hosted runner
  stopped progressing; preserve unsigned artifact `8558284504` and classify
  the canceled record as non-gate infrastructure history.
- [x] Close RTR-059 after canonical formatting, narrow validation, corrected
  retained-attempt accounting, and dual independent renewed PASS review; the
  184-record ledger ends at `dd312eb7` with zero effective open defects.
- [x] Delegate one changed-head comparator attempt at `a2446adc`: LIGHT passed,
  audit was READY 10/10, and seven HEAVY nodes passed before full Nextest
  exposed two duplicated noncanonical durable-ledger fixtures; CRAP did not
  launch and no retry ran.
- [x] Close RTR-060 after canonicalizing both fixture repository roots, exact
  2/2 qualification, dual independent PASS review, and durable closure at
  ledger tip `46777931`; zero effective defects remain open.
- [x] Apply the operator-authorized engineering-closeout exception to the
  missing repository attestation, complete dual terminal verification from
  retained exact evidence, archive the prompt, and record final disposition.
- [x] Correct the closeout record after operator clarification: Omarchy is the
  defunct historical runner; forest1 is the active self-hosted HEAVY runner.
  Automatic run `30002884134` reached forest1 content-gate execution and was
  canceled there, so it produced no passing repository attestation.

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

Attempt 14 passed LIGHT, audit, ordinary full Nextest, and entered CRAP, whose instrumented traversal timed out `verifier_accepts_truthful_fail_and_blocked_receipts` after 720 seconds. `RTR-027` (`GATE-VERIFIER-BLOCKED-FIXTURE-DUPLICATE-RECONSTRUCTION`) corrects that test to retain one public receipt reconstruction for the FAIL assertion while exercising the BLOCKED fixture at the already-admitted invariant seam. This preserves production verification behavior and avoids a second isolated full plan reconstruction.

After all seven CQR packages completed, the cheap final package audit exposed
RTR-031 (`GATE-CQR-AGGREGATE-AUTHORITY-NOT-SCAFFOLDED`): the master CQR plan
was not a base-commit aggregate authority package, and the original recovery
package could not be widened retroactively. No LIGHT or HEAVY gate started. The
correction package establishes immutable authority before its own correction
diff. The bounded validator package now mechanically binds the reusable CQR
standard, process, and template to a unique aggregate scaffold, immutable batch
manifest, complete master/module/path inventory, committed module scaffold, and
retained PASS before the first implementation edit. RTR-031 closed at correction
commit `5f47695e` with durable ledger entry `cb46f832`.

The next delegated attempt stopped before intent planning because the validator
package itself remained byte-identical to its scaffold. No LIGHT or HEAVY node
started. RTR-032 (`GATE-INTENT-PACKAGE-NOT-UPDATED`) owns the bounded lifecycle
correction: update package status/progress without changing its immutable write
set, prove exact admission, and obtain dual review before another changed-head
attempt.

RTR-032 closed at correction commit `68701b05` with durable ledger entry
`4eab64c9`. The immutable declared write set is unchanged, while the package
lifecycle now identifies the authorized increment.

The next changed-head attempt sealed receipt `20038867...fd4`: LIGHT passed 6/6,
the pre-HEAVY audit was READY 10/10, five HEAVY nodes passed, and workspace
Clippy rejected two CQR characterization tests at 157 and 101 lines. RTR-033
(`GATE-CQR-TEST-FUNCTION-LENGTH-REGRESSION`) owns their mechanical split in the
fresh `20260722-testgate-clippy-test-length-recovery-001` package. Downstream
doctest, full Nextest, and CRAP were blocked and did not run; there was no retry.

RTR-033 closed at correction commit `8b26689c` with durable ledger entry
`1a40c57e`. Both exact focused tests, package Clippy, and the previously failing
workspace Clippy command pass; dual independent reviews found no behavior or
assertion drift.

The following changed-head attempt passed corrected workspace Clippy and
doctests, then full Nextest passed 2,271/2,272 in 1,002.897 seconds. Its sole
failure was a stale integration source-contract search for
`let mut execution = execute_nodes_for(` after the executor binding became
immutable; the substantive final-context-before-spawn ordering remains present.
RTR-034 owns the one-line sentinel correction. CRAP was blocked and did not run;
there was no retry.

RTR-034 closed at correction commit `eeb858b2` with durable ledger entry
`f01d2e9e`. The exact failed test and all eight owning integration tests pass;
the final-context-before-spawn guard remains substantive and unchanged.

The following changed-head attempt passed LIGHT 5/5, audit 10/10, and its first
seven HEAVY nodes. Affected CRAP's instrumented Nextest passed 1,091/1,091 in
689.242 seconds, but post-acquisition adjudication rejected root measurement
package `openwepp` because production ownership is restricted to `crates/*`.
RTR-035 (`GATE-AFFECTED-CRAP-ROOT-PACKAGE-ADMISSION-MISMATCH`) owns early
measurement-package rejection plus planner escalation to global quality. No
retry ran.

RTR-035 through RTR-041 closed at exact reviewed correction commit `85d706ed`.
The final classifier admits only exact direct `Cargo.toml` production members
with plain existing lowercase-`.rs` targets beneath plain source roots and
rejects traversal, missing, symlink, root, test-only, uppercase, and out-of-tree
shapes before quality-node construction. The final planner target passed
155/155 with two skipped in 1,862.964 seconds, package audit was `READY`, and
both independent reviews passed. Durable closure digests are RTR-035
`aa0db31e`, RTR-036 `0915a278`, RTR-037 `9c5a5f69`, RTR-038 `fc01953c`, RTR-039
`f7f4c2ba`, RTR-040 `e10b42ae`, and RTR-041 `6a481692`.

The next delegated invocation stopped before planning because the package did
not explicitly constrain the recovery qualification to the planner-supported
`INCREMENT` boundary. No gate node or retry ran. RTR-042
(`GATE-QUALIFICATION-BOUNDARY-UNBOUND`) corrects the package and active prompt;
broader `CAMPAIGN`/`RELEASE` certification remains on the conservative lane.
Dual independent review passed, and RTR-042 closed at `dcb43397` with durable
ledger digest `1b65f07f`.
## Declared Write Set

- `.github/workflows/testgate-shadow.yml`
- `crates/openwepp-gate-planner/**`
- `gate-policy/v1/**`
- `tests/integration/testgate_*`
- `tests/python/test_testgate.py`
- `tests/python/test_resolve_testgate_comparison_base.py`
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

An exact failed recovery root that can never acquire hosted provenance remains
in history. It may be excluded only by a canonical `CLOSED` tooling-defect
record binding the exact root, reviewed correction commit, and closure
evidence. Reopening that defect revokes the exclusion; unrelated explicit
recovery roots remain fail-closed.

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
