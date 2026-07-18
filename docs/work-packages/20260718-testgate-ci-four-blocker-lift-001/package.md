# Close The Four TESTGATE-CI-01 Executor Blockers

Package ID: `20260718-testgate-ci-four-blocker-lift-001`

Queue ID: `TESTGATE-CI-01-HOLD-LIFT`

Status: `COMPLETE`

Execution date: 2026-07-18

Frozen base: `594f9a184b66ba228e0e81d379172639db3e55b7`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`. Keep its progress, decisions, discoveries, and
outcomes current.

## Purpose / Big Picture

Lift the four accepted high-severity blockers that prevented the TESTGATE
shadow executor from launching: nonpass evidence, adversarial execution proof,
terminal-plan covering-test closure, and external subprocess-output
confinement. A human can observe success through verifier-accepted FAIL and
BLOCKED receipts, the declared negative-case matrix, a terminal plan whose
affected-quality scope is complete or escalated globally, and an execution root
that leaves no subprocess evidence under repository authority.

The operator accepts the measured 48.8% projected wall-time reduction as a win.
That decision closes the package-local benchmark concern. It does not amend the
canonical 50% blocking-cutover threshold or authorize gate removal, protected
contexts, branch protection, or campaign/release certification.

## Progress

- [x] (2026-07-18) Froze base, authority, four-blocker scope, write set, and
  validation intent.
- [x] (2026-07-18) Emit verifier-valid FAIL/BLOCKED receipts with exact attempts, executed
  inventory, unavailable items, counts, artifacts, and mutation snapshots.
- [x] (2026-07-18) Implement the adversarial matrix, including zero-work dispatch and rollback
  consistency.
- [x] (2026-07-18) Bind affected production packages and complete covering-test inventory to
  the terminal plan; escalate unknown contribution to global measurement.
- [x] (2026-07-18) Relocate Cargo, Nextest, CRAP, and temporary subprocess outputs beneath
  the external execution root.
- [x] (2026-07-18) Complete focused adversarial checks and the fresh affected
  adjudicated-CRAP gate on the remediated tree.
- [x] (2026-07-18) Obtain dual independent implementation-review PASS verdicts.
- [x] (2026-07-18) Complete the terminal conservative sequence: fmt, workspace
  clippy, full Nextest, cargo-deny, and global adjudicated CRAP all PASS.
- [x] (2026-07-18) Obtain dual terminal-verifier PASS verdicts and disposition
  the package `COMPLETE`.

## Authority And Intent

Authority is `docs/standards/testing-and-gate-strategy.md` sections 8-10,
12, and 14-19; the prior package's accepted review disposition; and the user's
2026-07-18 direction to resolve the four blockers. This is a critical gate
executor/verifier change. Gate selection and receipt verification remain
fail-closed.

The pre-implementation intent is exactly the declared write set below. Terminal
reconciliation must retain global escalation for unknown coverage contribution
and must not weaken the unchanged conservative release workflow.

## Declared Write Set

- `crates/openwepp-gate-planner/src/executor.rs`
- `crates/openwepp-gate-planner/src/verifier.rs`
- `crates/openwepp-gate-planner/src/planner.rs`
- `crates/openwepp-gate-planner/src/repository.rs`
- `crates/openwepp-gate-planner/src/main.rs`
- `crates/openwepp-gate-planner/src/lib.rs`
- `gate-policy/v1/gate-definitions.json`
- `gate-policy/v1/impact-map.json`
- `gate-policy/v1/schemas/gate-plan.schema.json`
- `gate-policy/v1/schemas/gate-definitions.schema.json`
- `gate-policy/v1/schemas/gate-receipt.schema.json`
- `gate-policy/v1/fixtures/valid/gate-plan.json`
- `gate-policy/v1/fixtures/valid/gate-receipt.json`
- `tools/local_ci/testgate_shadow.py`
- `tools/release/run_adjudicated_crap_gate.sh`
- `tools/release/check_adjudicated_crap.py`
- `tools/release/README.md`
- `tools/local_ci/README.md`
- `.github/workflows/testgate-shadow.yml`
- `tests/integration/testgate_ci_*`
- `tests/python/test_adjudicated_crap_gate.py`
- `tests/python/test_testgate_shadow.py`
- `docs/work-packages/README.md`
- `docs/work-packages/20260718-testgate-ci-shadow-executor-001/**`
- `docs/work-packages/20260718-testgate-ci-four-blocker-lift-001/**`

Read-only discovery may inspect adjacent planner, verifier, schema, workflow,
and release-gate patterns. Writes outside this set require a recorded
pre-implementation amendment.

## Protected Boundaries

- Keep `.github/workflows/release-gates.yml` byte-unchanged and authoritative.
- Do not reduce, skip, deduplicate, or reuse a current gate.
- Keep shadow output nonblocking and explicitly non-certifying.
- Never execute a shell string assembled from plan content.
- Preserve typed errors, exact plan reconstruction, clean committed-checkout
  admission, atomic final artifacts, and process-group timeout termination.
- Unknown or incomplete affected coverage contribution escalates to global
  measurement; it never produces a narrow PASS.
- Subprocess-produced build, test, coverage, CRAP, and temporary evidence must
  remain under the caller-selected external execution root.

## Required Deliverables

1. Receipt construction for PASS, FAIL, BLOCKED, and mutation-invalid execution
   with mechanically verified attempts, inventory partition, unavailable
   reasons, result counts, artifacts, and before/after snapshots.
2. Executable negative cases for malformed identity, unknown executor, missing
   prerequisite, timeout, disallowed environment, working-directory/output
   escape, inventory mismatch, output collision, source mutation outside the
   prior filtered manifest roots, failing exit, blocked dependent, and
   zero-work aggregate behavior.
3. A terminal-plan quality scope that names affected production packages,
   covering gate nodes and exact covering inventory; bounded complete scope
   drives one affected measurement, while empty/unknown scope selects global.
4. Executor-owned external work paths for `CARGO_TARGET_DIR`, Nextest JUnit,
   CRAP output, and temporary files, with a source-tree negative assertion.
5. Focused evidence, dual independent review and remediation, exact-diff gate
   reconciliation, one terminal conservative closure sequence, dual terminal
   verification, line-count governance, and truthful disposition.

## Plan Of Work

Milestone 1 makes execution outcome a receipt value rather than an exception.
Preflight and plan-integrity errors still fail before process execution. Once a
node attempt begins, FAIL/BLOCKED outcomes remain observable, dependents receive
BLOCKED attempts, and the verifier reconstructs the exact inventory partition
and unavailable reasons.

Milestone 2 adds external executor work directories and an exact repository
mutation snapshot that covers tracked, staged, unstaged, and nonignored
untracked paths instead of relying only on selected manifest roots. The release
adapter honors the executor-selected external root without changing plan
arguments.

Milestone 3 projects affected-quality scope into the terminal plan. One
affected CRAP node receives the complete affected/reverse-dependent Cargo
package set. Its covering inventory is the exact union independently listed
for every package passed to that one instrumented Nextest execution. Any
unmapped or incomplete contribution selects the global node.

Milestone 4 runs the focused matrix and two independent reviews. Accepted
findings are remediated before the single conservative terminal sequence and
two independent exact-tree verifications.

## Validation And Acceptance

Focused development commands:

    cargo fmt --check
    cargo nextest run -p openwepp-gate-planner
    cargo nextest run --test testgate_ci_executor_contract
    python -m unittest tests.python.test_adjudicated_crap_gate
    cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings
    bash -n tools/release/run_adjudicated_crap_gate.sh
    python -m py_compile tools/local_ci/testgate_shadow.py tools/release/check_adjudicated_crap.py
    git diff --check

Acceptance requires direct assertions for every adversarial case; independent
verification of at least one FAIL and one BLOCKED receipt; exact affected
package/covering inventory agreement; global escalation on unknown scope; and
no subprocess-produced file under the repository after an externally rooted
execution.

After both implementation reviews pass, run exactly one terminal conservative
sequence on the stable tree:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check
    bash tools/release/run_adjudicated_crap_gate.sh --base-ref 594f9a184b66ba228e0e81d379172639db3e55b7

Do not repeat a successful heavy command for presentation. Any FAIL, BLOCKED,
or unjustified NOT RUN prevents package completion.

## Review, Verification, And Line Counts

Two independent reviewers inspect receipt/verifier consistency, adversarial
completeness, affected-quality soundness/economy, external confinement,
workflow zero-execution semantics, rollback truthfulness, and exact-diff gate
selection. Every finding is dispositioned as accepted, rejected, deferred, or
follow-up; accepted findings are fixed before closure.

Two terminal verifiers inspect the exact remediated tree and Gate Evidence
Non-Deferral compliance. Files at or above 2,000 lines are WARN with split
intent; non-generated 3,000+ files block closure absent an approved exception.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent reviewer roles and two independent
terminal-verifier roles for read-only inspection; expected outputs are compact
findings and verdicts delivered to the parent for package artifacts; write
access is read-only.

Subagent requirement: REQUIRED for the single terminal full-workspace Nextest,
cargo-deny, and global adjudicated-CRAP closure set. This package explicitly
authorizes subagent spawning/delegation to one closure-runner role for those
commands; expected outputs are exact commands, verdicts, counts, timing, and
artifact paths; write access is limited to generated build/coverage output.
The parent must not repeat successful heavy commands.

## Idempotence And Recovery

Every run uses a fresh external artifact root. Final receipt artifacts are
atomic; attempts and logs remain inspectable after failure. A rerun uses a new
root and cannot overwrite accepted evidence. Rollback disables only the
nonblocking shadow workflow; the conservative runner needs no restoration.

## Surprises & Discoveries

- Observation: the initial combined-path benchmark projected a 48.8% wall-time
  reduction with exact 26/26 inventory parity.
  Evidence: the operator accepted that reduction as a win on 2026-07-18.
- Observation: the first implementation smoke produced exact 31/31 Nextest
  execution but exposed repository-local JUnit output and 22 actionable CRAP
  rows concentrated in the previously unexecuted public executor path.
  Evidence: retained external report under
  `/tmp/openwepp-testgate-four-blocker-tXdBfr/target/affected-crap-smoke/`.
- Observation: the final affected package inventory expanded to 36 cases after
  adding executable end-to-end executor coverage; the fresh combined gate
  passed 36/36 with raw/actionable CRAP counts 0/0 and identical source
  manifests.
  Evidence: `artifacts/focused-gate-results.md` and the retained external root
  `/tmp/openwepp-testgate-four-blocker-final-lJfizstz/`.
- Observation: the first dual review held closure on four gaps: nonpass
  inventory inferred from process presence, reconstruction output outside the
  caller-selected root, source mutation detected only after the DAG, and an
  inline-shell admission mismatch. The review also requested behavioral proof
  for identity, drift, collision, zero-work, signal termination, and rollback.
  Evidence: accepted reviewer findings from `closure_final` and
  `terminal_governance`; all were remediated before re-review.
- Observation: externally confined verifier reconstruction materially raises
  cold-test duration because independent Nextest inventories compile beneath
  fresh external targets. This is a confirmed timing painpoint, not a reason to
  weaken confinement or repeat tests.
  Evidence: the focused truthful FAIL/BLOCKED verifier case passed once in
  275.58 seconds; no redundant standalone rerun is planned.
- Observation: dual re-review found remaining command-string option/wrapper
  bypasses, a rootless `ArtifactProvider` fallback, reconstruction child-path
  symlink escape, and missing behavioral global-escalation proof. All were
  accepted and remediated before terminal execution.
  Evidence: second HOLD verdicts from `closure_final` and
  `terminal_governance`; focused shell, rootless-provider, symlink, and global
  escalation cases pass.
- Observation: the third review passed governance/confinement and held only on
  additional shell families and option arguments before `-c`. The boundary was
  generalized to `*sh`/PowerShell interpreters in wrapper chains and scans up
  to an actual script path, with cited and env-wrapped variants covered.
  Evidence: focused structural shell-admission test and targeted clippy pass.
- Observation: the final governance review found an environment assignment
  ending in `sh` could shadow a later real interpreter token. Interpreter
  discovery now evaluates every non-assignment, non-script token and rejects
  when any such interpreter reaches command-string mode.
  Evidence: cited `env FOO=crash /bin/bash -c` case and targeted clippy pass.
- Observation: the first terminal sequence stopped after fmt PASS when
  workspace clippy found the integration contract test at 114 lines. The test
  was mechanically split into focused assertion helpers; targeted test clippy
  and both integration cases pass. Nextest, deny, and global CRAP were not run
  in that stopped sequence.
  Evidence: authorized closure-runner HOLD and focused remediation output.
- Observation: the second terminal sequence passed fmt and clippy, then full
  Nextest ran 2,141 tests with one failure: the authority contract expected an
  empty executed inventory to invalidate a non-zero-work PASS receipt. The
  receipt schema now requires nonempty execution only for non-zero-work PASS or
  PASS_WITH_RETRY, while preserving valid zero-execution FAIL/BLOCKED receipts.
  Evidence: terminal summary (2,140 pass, 1 fail, 5 skip) and focused authority
  case PASS after the precise conditional schema fix. Deny and global CRAP were
  not run in the stopped sequence.

## Decision Log

- Decision: close exactly four blockers and fold zero-work/rollback proof into
  the adversarial-workflow blocker.
  Rationale: those items are validation facets of execution behavior, not an
  independent implementation surface.
  Date/Author: 2026-07-18 / Codex.
- Decision: treat 48.8% as a positive package benchmark without rewriting the
  canonical 50% blocking-cutover threshold.
  Rationale: the user accepted the optimization outcome, while the canonical
  standard still controls protected cutover.
  Date/Author: 2026-07-18 / Codex.
- Decision: amend the write set before implementation to add
  `tests/python/test_testgate_shadow.py`.
  Rationale: nonpass CLI retention and failure-visible shadow exit semantics
  require executable Python-level proof, not source inspection alone.
  Date/Author: 2026-07-18 / Codex.
- Decision: extend the write set to the committed-source observer, definition
  schema, valid plan fixture, and local-CI README when their direct contract
  dependencies became concrete during integration.
  Rationale: mutation-invalid receipt reconstruction requires immutable commit
  observation, the combined inventory source is schema-controlled, the plan
  schema fixture must remain valid, and external-output behavior is an operator
  contract. The dependency was recorded immediately on discovery; no unrelated
  surface is admitted.
  Date/Author: 2026-07-18 / Codex.
- Decision: implement affected quality as one combined instrumented Nextest
  execution whose exact inventory is independently enumerated as the union of
  every terminal production/reverse-dependent package.
  Rationale: this preserves complete test contribution while realizing the
  accepted 48.8% savings; unknown or empty contribution still escalates to the
  unchanged global node.
  Date/Author: 2026-07-18 / Codex.
- Decision: amend the write set to include the valid receipt fixture.
  Rationale: adding signal termination to the closed receipt attempt schema
  requires the canonical valid fixture to carry the new nullable field.
  Date/Author: 2026-07-18 / Codex.
- Decision: accept both initial reviewer HOLD verdicts and remediate every
  actionable item before requesting re-review.
  Rationale: the findings identified real evidence-truth, mutation-order,
  shell-admission, confinement, and behavioral-proof gaps.
  Date/Author: 2026-07-18 / Codex.

## Outcomes & Retrospective

Complete. The four launch blockers are closed without reducing or reusing a
current gate:

- FAIL/BLOCKED/INVALID receipts retain real attempt, signal, partial-JUnit,
  unavailable-partition, artifact, and mutation evidence that the independent
  verifier reconstructs.
- Terminal quality scope selects one exact affected combined measurement when
  contribution is complete and escalates unknown contribution to the unchanged
  global gate.
- Cargo, Nextest, coverage, CRAP, temporary, snapshot, and verifier
  reconstruction outputs are confined beneath the caller-selected external
  root, including fail-closed provider and symlink handling.
- Inline command strings, source mutation continuation, identity/inventory
  drift, output collision, zero-work, rollback, termination signals, and other
  adversarial paths have executable coverage.

Dual implementation review and dual terminal verification passed. The final
conservative sequence passed fmt, workspace clippy, 2,141 full-profile tests,
cargo-deny, and fresh global adjudicated CRAP with zero actionable rows. The
accepted 48.8% projection remains a positive shadow benchmark only; this
package does not claim blocking cutover, branch-protection migration, or the
14-day/20-increment observation requirement.
