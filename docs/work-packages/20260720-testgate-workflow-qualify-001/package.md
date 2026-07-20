# TESTGATE Black-Box Workflow Qualification

Package ID: `20260720-testgate-workflow-qualify-001`

Queue ID: `TESTGATE-WORKFLOW-QUALIFY-01`

Status: `SCAFFOLDED / REVIEW PENDING`

Authorization: Roger Lew's 2026-07-20 direction to scaffold and roadmap a
follow-up workflow verification package for the TESTGATE defects.

This ExecPlan is maintained under `docs/codex_exec_plans.md`. Keep its
progress, discoveries, decisions, and outcomes current throughout execution.

## Purpose

Independently prove that the repaired TESTGATE workflow prevents the observed
failures and avoids duplicate expensive work under realistic failure and
recovery sequences. This package is a black-box consumer of the frozen
`TESTGATE-CLOSURE-AUDIT-01` implementation. It cannot edit planner, executor,
verifier, helper, gate policy, workflow, cache, schema, test, or release-tool
bytes to obtain a passing result.

The earlier adversarial acceptance packages exercised one documentation-only
increment and stopped at the first admission failure. They were useful but too
narrow to prove stage ordering, mid-DAG recovery, runner loss, receipt reuse,
cache integrity, combined coverage, or trusted-workflow behavior. This package
qualifies those paths directly.

## Progress

- [x] (2026-07-20) Scaffold package authority, controller contract, qualification
  matrix, evidence contract, prospective gate plan, and kickoff prompt.
- [ ] Complete dual independent scaffold review and disposition.
- [ ] Admit the exact completed implementation head and freeze all subject
  roots.
- [ ] Execute the bounded local black-box matrix once.
- [ ] Execute or consume exactly one qualifying trusted-runner workflow.
- [ ] Complete dual result review, dual terminal verification, and disposition.

## Dependencies And Admission

Execution is blocked until all conditions hold:

1. `20260720-testgate-pre-heavy-closure-audit-001` has terminal `COMPLETE-PASS`
   evidence for an exact clean implementation commit.
2. `TGCA-001` through `TGCA-011` are individually mapped to passing
   implementation evidence with no open review finding or tooling defect.
3. A versioned black-box qualification interface exists at
   `tools/local_ci/testgate_qualification.py` or a reviewed successor path. It
   may be delivered by the implementation or an explicit prerequisite
   correction package. Qualification remains queued until the interface invokes
   the real `tools/local_ci/testgate.py`, planner, executor, verifier, schemas,
   and workflow contract while replacing only expensive gate payloads with
   bounded test-only probe executables.
4. The qualification package exists in the authenticated base commit before
   any qualification diff or TESTGATE execution.
5. The exact implementation source, policy, workflow, schema, toolchain, and
   binary roots are frozen in `artifacts/subject-freeze.md`.
6. GitHub credentials and provider availability exist for one ordinary
   trusted-runner qualification. If not, local cases may run but the package
   remains `HOLD-PROVIDER-EVIDENCE`; provider evidence is not simulated.

## Scope

Included:

- read-only inspection and independent hashing of the completed implementation
  package, plans, audit, per-node receipts, ledger, artifacts, and reviews;
- one controller-owned qualification session using the frozen versioned matrix;
- all bounded local cases through the real TESTGATE entry point and frozen
  test-only probe executables;
- process-spawn counters, argument traces, audit IDs, receipt IDs, cache events,
  timing, and exact retry/reuse decisions;
- forced process termination, worktree/runner-directory destruction, durable
  evidence re-ingestion, and cross-attempt resume inside disposable controlled
  environments;
- one real combined full-regression/LCOV/CRAP execution, ordinarily reused from
  the implementation's exact trusted TESTGATE run when current and eligible;
- exactly one ordinary trusted-runner TESTGATE execution when current provider
  evidence is not reusable, dispatched only after proving no run is queued or
  active;
- read-only provider inspection of workflow identity, job order, artifacts,
  attestation, timestamps, and concurrency state;
- two independent result reviews, finding disposition, and two independent
  terminal verifications; and
- package-local evidence, roadmap/catalog updates, and prompt archival.

Excluded:

- any edit to production, tests, fixtures, planner, executor, verifier, helper,
  workflow, gate policy, schema, cache implementation, release tooling, or
  canonical testing standards;
- adding, changing, blessing, or repairing qualification hooks after subject
  freeze;
- changing gate selection, thresholds, test membership, trust, retry, reuse,
  or acceptance policy;
- repeated full workspace, coverage, CRAP, comparator, release, or population
  runs;
- concurrent or duplicate manual TESTGATE dispatch while any run is queued or
  active;
- intentional disruption of a production runner, provider queue, or unrelated
  workflow;
- simulation science, kernel, assurance, release qualification, or publication
  claims; and
- fixing a discovered defect inside this package.

## Declared Write Set

- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260720-testgate-workflow-qualify-001/**`

Temporary repositories, process probes, copied worktrees, and execution output
must remain under ignored `target/testgate-qualification/` or a fresh external
temporary root. Provider actions may dispatch and read exactly the admitted
TESTGATE workflow; they do not authorize repository-content edits.

## Qualification Invariants

1. Subject bytes are frozen before the first case. Any subject-root change
   invalidates the complete qualification and requires a new package attempt.
2. The controller invokes public/ordinary TESTGATE interfaces. Test-only probe
   executables replace cost, not orchestration, planning, identity, audit,
   receipt, verification, persistence, or state-machine behavior.
3. Every case has a predeclared injection, expected status, expected spawn
   count, expected reusable/rejected receipt set, and required artifacts.
4. Non-`READY` cases require a heavy-spawn count of zero.
5. A late failure must not rerun a successful current target-reusable node.
6. A rejected receipt retains the exact §10.4 trust, reuse-class, or execution-
   context reason; recovery does not relabel it reusable.
7. An observed failure is retained. The package does not retry an unexpected
   result, edit the subject, or narrow the matrix.
8. Producer-only unit results cannot satisfy a real-helper, crash/restart,
   fresh-environment, combined-heavy, or trusted-provider claim.
9. The only real expensive execution is the one current combined run required
   for functional and LCOV/CRAP proof. Every other heavy payload is a bounded
   probe.
10. A failed case opens a named correction package and leaves this package
    `FAIL` or `HOLD`; it is never fixed in place.

## Phase Plan

### Phase A: independent intake and subject freeze

1. Verify dependency completion, exact clean implementation commit, terminal
   audit/receipt signatures, and finding disposition.
2. Run instruction discovery for the declared write set and record the map.
3. Independently hash every subject path named by the implementation's
   execution, authority, documentation, toolchain, workflow, and policy roots.
4. Validate the qualification interface and probe binaries without running a
   case. Reject hooks that bypass the real helper or synthesize PASS receipts.
5. Freeze controller input, matrix version, case order, expected outputs,
   provider plan, and artifact root before delegating execution.

### Phase B: bounded local black-box matrix

1. Run `Q01` through `Q15` from
   [`artifacts/qualification-matrix.md`](artifacts/qualification-matrix.md) once
   in fresh case roots.
2. Capture process spawn/exit traces, exact command arguments, audit and receipt
   IDs, per-node counters, ledger chains, cache events, timings, and artifact
   hashes.
3. For crash/recovery, terminate after the declared checkpoint, delete the
   disposable worktree and runner-local execution directory, construct a fresh
   environment, re-ingest durable evidence, and resume.
4. Stop the matrix at the first unexpected result. Preserve it and create a
   defect-shaped handoff; do not patch or retry within this package.

### Phase C: real heavy and provider path

1. Determine whether the completed implementation's trusted combined-run
   receipt is current and reusable for the frozen execution root. Reuse it when
   eligible; rerunning for reassurance is forbidden.
2. If no eligible receipt exists, query provider state and prove no TESTGATE
   run is queued or active, then dispatch exactly one ordinary workflow against
   the admitted exact head. Never dispatch while another run is queued or
   active.
3. Require exactly one full Nextest process, one admitted inventory, functional
   and LCOV/CRAP artifacts derived from the same node/attempt, a verified
   receipt, independent hosted verification, durable artifact upload, and the
   expected attestation.
4. Reconstruct provider job order, timestamps, artifact digests, and run state.
   Validate single-active/newest-pending behavior through the implementation's
   hermetic workflow/concurrency harness and retained provider evidence; do not
   create overlapping live dispatches merely to test queuing.

### Phase D: independent verdict and disposition

1. Two independent reviewers compare all observed case results to the frozen
   matrix and inspect negative proof, consumer-path use, duplicate-process
   counts, evidence durability, and provider evidence.
2. Disposition every finding as `accepted`, `rejected`, `deferred`, or
   `follow-up`. A substantive accepted defect cannot be repaired here; it
   creates a correction package and blocks qualification.
3. Two independent terminal verifiers reconstruct subject hashes, case
   inventory, attempt ledger, spawn counts, receipt reuse/rejection, combined-
   run lineage, provider identity, and the no-subject-edit boundary.
4. Archive the prompt, update roadmap/catalog state, and write exact `PASS`,
   `FAIL`, or `HOLD` disposition.

## Qualification Interface Contract

Qualification requires the following stable, versioned interface before intake.
It may consume but not change it:

```text
.venv/bin/python tools/local_ci/testgate_qualification.py validate \
  --repo . --implementation-commit <sha> \
  --matrix-schema gate-policy/v1/schemas/workflow-qualification.schema.json \
  --output <interface-validation.json>
.venv/bin/python tools/local_ci/testgate_qualification.py run \
  --repo . --binary target/debug/openwepp-gate-plan \
  --subject-freeze <subject-freeze.json> \
  --controller-input <controller-input.json> \
  --artifact-root <fresh-root> --output <qualification-report.json>
.venv/bin/python tools/local_ci/testgate_qualification.py verify \
  --repo . --subject-freeze <subject-freeze.json> \
  --report <qualification-report.json> --artifact-root <fresh-root>
```

`validate` must prove that the harness calls the real helper and frozen
planner/executor/verifier rather than synthesizing results. `run` executes the
ordered matrix once and stops on unexpected output. `verify` independently
rehashes the subject, validates the ledger and artifacts, reconciles spawn
counts, and returns nonzero for any matrix mismatch. The implementation must
also provide the strict JSON Schema and valid/invalid fixtures for subject
freeze, controller input, per-case reports, and aggregate qualification report.

The expected local intake commands are:

```text
cargo build --locked -p openwepp-gate-planner --bin openwepp-gate-plan
git diff --quiet <implementation-commit> -- <frozen-subject-paths>
markdown-doc lint --path docs/work-packages/20260720-testgate-workflow-qualify-001
git diff --check
```

The planner build is permitted only when its exact frozen binary is not already
available and verified. It is not a test rerun. Package closure records every
actual command and observed output.

## Acceptance

- [ ] The frozen implementation and every subject root remain byte-identical
  from intake through final verification.
- [ ] All 15 matrix cases execute exactly once through the real helper and
  frozen planner/executor/verifier path.
- [ ] Every non-`READY` case records zero heavy probe spawns.
- [ ] Inventory omission, argument drift, output collision, mutation, unsafe
  cache reuse, environment mismatch, and package widening fail closed.
- [ ] The mid-DAG recovery case proves a previously successful current,
  target-reusable node has total spawn count one across attempts.
- [ ] Runner/worktree loss preserves and re-ingests digest-bound pre-receipt and
  per-node evidence in a fresh environment.
- [ ] Same-cause recurrence blocks another heavy spawn and links a typed tooling
  defect.
- [ ] Documentation-only and substantive post-review changes invalidate exactly
  the appropriate roots/nodes in separate disposable cases.
- [ ] Combined-run evidence proves exactly one full Nextest process and common
  inventory/node/attempt lineage for functional and LCOV/CRAP results.
- [ ] Combined-run timing meets the adopted performance thresholds, or the
  implementation truthfully retained separate nodes and this qualification
  makes no deduplication claim.
- [ ] Provider evidence comes from the exact admitted head, workflow, runner,
  jobs, artifacts, verifier, and attestation; no simulation is labeled live.
- [ ] No manual dispatch occurs while TESTGATE is queued or active, and no
  duplicate real heavy run is launched.
- [ ] The executor changes only package-local evidence and does not repair or
  weaken the frozen subject.
- [ ] Every matrix row, review finding, retry, reuse/rejection decision, skipped
  provider action, and gate has an explicit disposition.
- [ ] Dual review and dual terminal verification leave no finding open; `.rs`
  line-count governance is `NOT_APPLICABLE` because no Rust edit is authorized.

Any unexpected result, subject edit, missing artifact, unproven consumer path,
duplicate real heavy execution, unverifiable provider claim, or open finding
prevents `PASS`.

## Prospective Gate Plan

The package is a documentation/evidence increment whose qualification workload
is specified separately in
[`artifacts/prospective-gate-plan.md`](artifacts/prospective-gate-plan.md).
Ordinary package Markdown/path/diff checks run after evidence edits. Existing
implementation receipts are reused only when exact verification proves them
current. No broad gate runs solely because review or narrative changed.

## Review And Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one black-box executor with write access limited to fresh
ignored/external execution roots, two independent read-only result reviewers,
two independent read-only terminal verifiers, and one
`comparator_suite_runner` role for the single selected real heavy/provider run;
expected outputs are compact commands, case results, process counts, timings,
artifact paths/hashes, findings, and `PASS`/`HOLD`/`FAIL` verdicts.

Subagent requirement: REQUIRED. The parent owns subject freeze, controller
inputs, provider-state preflight, exact single-dispatch authorization, evidence
integration, roadmap/catalog edits, and final disposition. No subagent may edit
the subject, commit, push, or dispatch provider work.

## Security And Provider Controls

Qualification hooks are test-bound capability surfaces. Validate that they
cannot be enabled by ordinary push events, untrusted branches, untrusted input,
or unbound environment variables. Probe arguments use arrays without shell
interpolation. Temporary repositories and artifacts reject traversal, symlink
escape, aliasing, and pre-existing output. Provider tokens are never written to
artifacts. The controller queries queue state before any dispatch and records
the exact controller-created run ID; it never cancels or modifies unrelated
runs.

## Idempotence And Recovery

Each case and provider attempt uses an immutable new root. An unexpected result
stops execution and is not retried. Expected crash/recovery uses two linked
attempt IDs and proves evidence re-ingestion in a newly constructed environment.
The package can be restarted only from its frozen intake when no case has run;
after any case result, a complete restart requires a new package attempt and
preserves the prior evidence.

## Surprises And Discoveries

None at scaffold time.

## Decision Log

- Decision: separate implementation from qualification. Rationale: a package
  must not author the test surface and certify the same behavior as independent
  black-box evidence. Date/author: 2026-07-20, scaffold author.
- Decision: use bounded probes for failure cases and at most one real combined
  heavy run. Rationale: failure breadth should not recreate the repeated-test
  cost the tooling exists to prevent. Date/author: 2026-07-20, scaffold author.
- Decision: do not induce overlapping live provider runs. Rationale: repository
  governance forbids manual TESTGATE dispatch while a run is queued or active;
  concurrency failure injection belongs in the frozen hermetic harness.
  Date/author: 2026-07-20, scaffold author.

## Outcomes And Retrospective

Not executed. Scaffold review and dependency implementation are pending.
