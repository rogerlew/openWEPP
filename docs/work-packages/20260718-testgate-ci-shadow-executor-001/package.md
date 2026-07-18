# TESTGATE-CI-01 Shadow Executor And CI Observation

Package ID: `20260718-testgate-ci-shadow-executor-001`

Queue ID: `TESTGATE-CI-01`

Status: `EXECUTED-HOLD`

Execution date: 2026-07-18

Frozen base: `28daa0339cbc99ceb94b176a506ce0c213685d7c`

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`. Keep its progress, decisions, discoveries, and
outcomes current.

## Purpose / Big Picture

Implement the first repository-owned, fail-closed consumer of TESTGATE plans;
add affected-surface quality measurement and evidence receipts; and launch
distinct, nonblocking CI observation lanes beside the unchanged conservative
release runner. Benchmark a combined instrumented Nextest path without assuming
that it replaces the independent full and coverage runs.

This package executes the implementation and shadow-launch phase. It cannot
truthfully complete blocking cutover in one execution: the canonical standard
requires at least 14 consecutive days, 20 representative increments, retained
campaign replay, provider-side context evidence, and a dual-required interval.
Those operands remain current package acceptance rather than being fabricated,
waived, or silently deferred.

## Progress

- [x] (2026-07-18) Froze the clean base, authority, intent, write set, and
  non-cutover boundary.
- [ ] Implement the typed plan executor, receipt production, and failure
  injection coverage. The passing path is implemented; failure receipts and
  required adversarial tests remain blocking.
- [ ] Add affected-surface coverage/CRAP measurement while retaining the global
  closure path. Package measurement exists but covering-test closure remains
  blocking.
- [ ] Split lifecycle workflow lanes and launch stable nonblocking shadow
  contexts beside the conservative runner. Lanes are implemented but launch is
  held by executor/measurement findings.
- [x] (2026-07-18) Captured the initial combined-path benchmark, inventory
  comparison, rollback evidence, and scorecard seed.
- [x] (2026-07-18) Completed focused validation and two dual-review rounds;
  remediation closed both first-round critical findings, while four high
  findings remain.
- [ ] Run the single conservative terminal closure set on the stable tree and
  complete dual terminal verification.
- [ ] Observe at least 14 consecutive days and 20 representative increments,
  replay every eligible retained campaign, prove protected publication and
  provider contexts, and disposition the fixed scorecard.

## Authority And Intent

The `TESTGATE-CI-01` roadmap row, ADR-0039, the canonical testing-and-gate
strategy, and TESTGATE-ALIGN-01's implementation handoff authorize this package.
The accepted implementation intent is to add a real shadow consumer and
measurement path without reducing any current gate. This is a critical change
to gate execution, receipts, coverage, and workflow selection.

At terminal reconciliation, every changed path must be reclassified against the
canonical impact map. Unknown or unresolved production/authority impact is
critical and retains the full conservative terminal set.

## Declared Write Set

- `Cargo.toml`
- `Cargo.lock`
- `.config/nextest.toml`
- `.github/workflows/release-gates.yml`
- `.github/workflows/testgate-shadow.yml`
- `crates/openwepp-gate-planner/**`
- `gate-policy/v1/**`
- `tools/local_ci/**`
- `tools/release/run_adjudicated_crap_gate.sh`
- `tools/release/check_adjudicated_crap.py`
- `tools/release/run_release_candidate_gates.sh`
- `tools/release/README.md`
- `tests/integration/testgate_ci_*`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260717-test-gate-authority-001/artifacts/implementation-handoff.md`
- `docs/work-packages/20260718-testgate-ci-shadow-executor-001/**`

Read-only discovery may inspect Git history, schemas, manifests, workflow runs,
retained package evidence, and release tooling. Writes outside this set require
a recorded pre-implementation amendment.

## Protected Boundaries

- Keep the new workflow and every generated verdict nonblocking and labeled
  `SHADOW`; leave the existing conservative release runner unchanged and
  authoritative.
- Do not change branch protection, merge queues, required contexts, provider
  secrets, issuer authority, or repository rulesets in this package.
- Do not reduce, skip, deduplicate, or reuse a current gate based on prototype,
  shadow, retained, locally trusted, or benchmark-only evidence.
- Execute typed argument vectors directly. Never evaluate planner content
  through a shell or inherit undeclared environment variables.
- Confine working directories and outputs; reject path traversal, symlink
  escape, output collision, source-tree mutation, identity drift, incomplete
  inventories, missing prerequisites, timeout, and unsupported executors.
- Preserve global adjudicated CRAP closure unchanged. Affected measurement is
  increment feedback and cannot substitute for global critical closure before
  accepted cutover.
- Do not claim protected evidence, release certification, campaign
  certification, or cryptographic trust from local unsigned receipts.

## Required Deliverables

1. A typed executor that consumes a verified plan, runs its DAG without a
   shell, confines environment/work/output paths, enforces timeout and retry
   policy, blocks dependents after failure, and writes atomic logs and receipts.
2. Receipt construction that binds exact plan, roots, inventory, attempts,
   outcomes, artifacts, mutation checks, and local trust without bypassing the
   existing verifier.
3. Failure-injection tests for malformed identity, unknown executor, missing
   prerequisite, timeout, disallowed environment, working-directory/output
   escape, inventory mismatch, output collision, and source mutation.
4. Affected-surface coverage/CRAP measurement selected from the terminal plan,
   with explicit empty/unknown behavior and the retained global mode unchanged.
5. Distinct presubmit shadow, post-submit/backstop, campaign, and release lane
   definitions with stable aggregate names and a documented conservative
   rollback path.
6. An initial benchmark comparing separate full/coverage inventory, doctests,
   CRAP inputs, failure semantics, and wall time against the candidate combined
   instrumented path. Any discrepancy blocks deduplication, not shadow launch.
7. A machine-readable observation scorecard seed that records dates,
   increments, strata, selector misses, reuse defects, inventory mismatches,
   timing, and discrepancy dispositions without inventing unavailable history.
8. Focused evidence, dual independent review and remediation, one terminal
   conservative closure set, dual terminal verification, line-count governance,
   rollback evidence, and truthful disposition.

## Execution Plan

Milestone 1 implements execution and receipt construction in the planner crate.
The executor validates the plan before starting, performs deterministic
topological scheduling, and never converts typed arguments into a shell string.
Tests use purpose-built harmless commands and injected clocks/process outcomes
where practical; no test invokes the global release suite.

Milestone 2 adds affected-quality selection and a bounded measurement adapter.
The adapter derives candidate production files and covering test inventory from
the terminal plan, refuses unresolved or empty required scope, and preserves the
existing global adjudicated command byte-for-byte for critical closure.

Milestone 3 adds nonblocking lifecycle jobs. Presubmit emits the plan and shadow
execution result; post-submit/backstop collects broad comparison evidence;
campaign and release entry points retain their separate authority. The existing
`workspace-validation` context and release runner remain intact.

Milestone 4 captures the initial benchmark and scorecard seed, then completes
focused checks and dual exact-diff review. Accepted findings are fixed before a
single terminal conservative gate sequence and dual exact-tree verification.
Shadow launch can pass while cutover remains `OBSERVING`; the package stays
active until all time-, increment-, replay-, and provider-bound operands pass.

## Validation And Acceptance

Focused development commands:

    cargo fmt --check
    cargo nextest run -p openwepp-gate-planner
    cargo nextest run --test testgate_ci_executor_contract
    cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings
    bash tools/release/check_authority_suite_antievasion.sh
    git diff --check

Shadow-launch acceptance requires all executor and affected-quality negative
cases to fail closed; byte-identical plan/receipt replay where timestamps and
durations are normalized; stable aggregate context names; unchanged current
release execution; initial benchmark and rollback evidence; no unexplained
inventory mismatch; dual review; and the terminal conservative sequence:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check
    bash tools/release/run_adjudicated_crap_gate.sh --base-ref 28daa0339cbc99ceb94b176a506ce0c213685d7c

Blocking cutover additionally requires every operand in section 19 of the
canonical testing-and-gate strategy: at least 14 consecutive observation days,
20 representative increments across every required stratum, every eligible
retained campaign replay, zero unsafe misses/reuse/inventory defects, two clean
environment replays, planner p95 at or below 30 seconds, median ordinary
increment wall time at least 50% lower with p95 no worse, discrepancy closure,
protected publication, stable required contexts, and a dual-required interval.

Do not repeat a passing broad command merely to refresh presentation evidence.
Any failed or unavailable current-scope gate prevents package completion.

## Review, Verification, And Line Counts

Two independent reviews cover executor security, schema conformance, receipt
truthfulness, inventory completeness, affected-quality validity, workflow
semantics, rollback, test economy, and non-cutover scope. Every finding is
accepted, rejected, deferred, or follow-up with rationale. Two independent
terminal verifiers inspect the remediated exact tree and Gate Evidence
Non-Deferral compliance.

Files at or above 2,000 lines are `WARN` and need decomposition rationale;
non-generated files at or above 3,000 lines block closure without an approved
owner and sunset exception.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent reviewer roles and two independent
terminal-verifier roles for read-only inspection of the package diff and exact
terminal tree; expected outputs are compact findings/verdicts delivered to the
parent for package artifacts; write access is read-only.

Subagent requirement: REQUIRED for the single terminal full-workspace Nextest,
cargo-deny, and global adjudicated-CRAP closure set. This package explicitly
authorizes subagent spawning/delegation to one closure-runner role for those
commands; expected outputs are exact commands, verdicts, counts, timing, and
artifact paths; write access is limited to generated build/coverage output. The
parent must not repeat successful heavy commands.

## Idempotence And Recovery

Execution uses a caller-selected artifact root outside source authority and
atomic finalization. Interrupted nodes are failures; completed attempts remain
inspectable. Reruns create a new receipt and never overwrite accepted evidence.
Workflow rollback disables only the new nonblocking shadow workflow; the
conservative runner needs no restoration because it is never removed.

## Surprises & Discoveries

- None at intake.

## Decision Log

- Decision: Treat implementation launch and blocking cutover as two observable
  states of one package.
  Rationale: The fixed 14-day/20-increment scorecard is a real current-scope
  cutover gate, but waiting to implement until history exists is impossible.
  The package may record `SHADOW-LAUNCHED / OBSERVING`; it may not claim
  `COMPLETE` until those operands exist and pass.
  Date/Author: 2026-07-18 / Codex.
- Decision: Keep the conservative workflow authoritative and duplicative during
  observation.
  Rationale: The governing standard explicitly requires shadow and a
  dual-required interval before removal; benchmark evidence cannot authorize
  early economy changes.
  Date/Author: 2026-07-18 / Codex.
- Decision: Amend the write set to include `tools/release/README.md` before
  editing it.
  Rationale: The affected/global mode boundary is an operator-facing release
  contract and must be documented beside its canonical driver.
  Date/Author: 2026-07-18 / Codex.

## Outcomes & Retrospective

Executed to `HOLD`. The repository now has a typed executor passing-path,
real-artifact checks, exact committed-checkout admission, process-group timeout,
atomic external receipt artifacts, affected/global CRAP modes, lifecycle
profiles, and failure-visible shadow workflow lanes. The conservative release
runner is unchanged.

Dual review correctly prevented shadow launch. Failure receipts, executable
failure injection, terminal-plan covering-test closure, and external
subprocess-output confinement remain unmet. The focused combined benchmark ran
26/26 planner tests with byte-identical before/after inventory in 249.87 seconds
but missed the fixed 50% median-reduction threshold (48.8%). Planner time was
91.876 seconds, above the 30-second cutover threshold. No terminal broad gate
or global CRAP run was started because current acceptance was already on HOLD.
