# TESTGATE Pre-Heavy Closure Audit And Structural Repair

Package ID: `20260720-testgate-pre-heavy-closure-audit-001`

Queue ID: `TESTGATE-CLOSURE-AUDIT-01`

Status: `ACTIVE / IMPLEMENTATION`

Authorization: Roger Lew's 2026-07-20 direction to make tooling correction
canonical, create one pre-heavy closure audit, scaffold the repair package, and
dispatch dual review and disposition.

This ExecPlan is maintained under `docs/codex_exec_plans.md`. Its progress,
discoveries, decisions, and outcomes must remain current during execution.

## Purpose

Make TESTGATE materially reduce elapsed time. An executor will be able to run
one cheap, canonical audit after assembling its intended closure state and
before launching any expensive gate. The audit either returns `READY` with the
exact execution DAG and reusable evidence, or stops with typed defects before
hours of work are spent. Mechanically detectable workflow gaps become tool
checks, and recurring infrastructure workarounds become blocking tooling
defects rather than repeated retries.

## Progress

- [x] (2026-07-20) Record canonical pre-heavy and tooling-correction rules.
- [x] Scaffold the implementation package and acceptance inventory.
- [x] (2026-07-20) Complete dual independent scaffold review and disposition;
  both final re-reviews pass with no open finding.
- [x] (2026-07-20) Freeze implementation intake at
  `fc514188651b3bb3353e2cab247f5112a0c324f6`, confirm a clean worktree, and
  record the applicable instruction chain and intended implementation diff.
- [x] (2026-07-20) Implement typed cost classes, staged light/heavy execution,
  package validation, the ten-check audit, per-node checkpoints, durable
  attempt indexing, and trusted-workflow re-ingestion.
- [x] (2026-07-20) Retain separate full-regression and global-coverage nodes
  with typed `COMBINATION_NOT_ADOPTED_INSUFFICIENT_COMPATIBLE_HISTORY`; no
  compatible three-receipt same-host baseline exists for safe adoption.
- [ ] Run the exact current light plan and pre-heavy audit, then delegate
  selected heavy nodes once.
- [ ] Complete dual implementation review, dual terminal verification, and
  closeout.

## Context

The canopy-phenology closure exposed a system problem rather than an isolated
operator mistake. Expensive work began before package schema, closure paths,
line counts, environment identity, output namespaces, and cache behavior were
fully proven. Planning, execution, and verification could rediscover the same
inventory independently. Failed attempts lived partly in ephemeral locations,
and the ordinary full suite could be repeated under coverage. The later
adversarial TESTGATE rerun also proved that a newly scaffolded package could not
authorize itself from the selected base and that the helper depended on one
exact write-set heading not used by that package.

The implementation must correct these causes at their owning layers. It must
not merely add another checklist, wrapper, or package-specific workaround.

## Defect Closure Envelope

The canonical defect inventory is
[`artifacts/defect-inventory.md`](artifacts/defect-inventory.md). The package
owns these defect classes end-to-end:

- `TGCA-001`: no single mandatory pre-heavy closure audit;
- `TGCA-002`: scaffold admission sequencing and write-set schema mismatch;
- `TGCA-003`: closure-path and cheap static blockers discovered after heavy
  execution;
- `TGCA-004`: planner, executor, and verifier inventory/argument drift;
- `TGCA-005`: late environment/tool/binary identity failure;
- `TGCA-006`: mutable or colliding attempt outputs and unsafe cache reuse;
- `TGCA-007`: duplicate full regression and coverage execution;
- `TGCA-008`: overbroad invalidation of executable receipts by documentation;
- `TGCA-009`: ephemeral-only attempt, timing, and cost evidence; and
- `TGCA-010`: recurring infrastructure workarounds without a blocking defect
  lifecycle; and
- `TGCA-011`: late-node failure restarts already successful, still-current
  nodes instead of resuming from verified per-node receipts.

## Scope

Included:

- a versioned pre-heavy audit schema, typed Rust model, CLI subcommand, and
  stable human rendering in `openwepp-gate-planner`;
- integration in `tools/local_ci/testgate.py` and trusted TESTGATE workflows so
  heavy nodes cannot launch without the exact `READY` audit report;
- a scaffold-only package validator and explicit scaffold-commit admission
  sequence, followed by terminal reconciliation against the current package
  and exact Git diff;
- one canonical inventory/argument/DAG snapshot consumed by execution and
  checked against the verifier's independent current enumeration;
- cheap package-schema, path, diff, documentation/schema, artifact, prompt,
  and line-count checks before heavy work;
- preflight identity for toolchain, environment allowlist, binaries, fixtures,
  policy, features, configuration, runner, and concurrency ownership;
- fresh immutable attempt roots, collision rejection, cache-key isolation, and
  source/index/measurement mutation guards;
- separate execution, authority, and documentation roots with explicit
  evidence-reuse decisions;
- append-only repository-local ignored run history for attempts, timing, cost,
  failure classification, and tooling-defect linkage;
- durable trusted-run upload, digest indexing, and re-ingestion even when an
  attempt fails before its aggregate receipt exists;
- verified cross-attempt import of successful per-node receipts that are current
  and reusable in the target attempt, so a late failure resumes only missing,
  invalidated, or context-ineligible nodes;
- one instrumented full Nextest path that satisfies regression and LCOV/CRAP
  when parity is proven, with a typed separation reason otherwise;
- focused unit, schema, integration, failure-injection, and workflow contract
  tests reproducing every defect class; and
- documentation, package templates, prompt language, and adoption guidance.

Excluded:

- simulation science, kernel, fixtures, numerical tolerances, and scientific
  assurance conclusions;
- weakening, skipping, or reclassifying any selected gate;
- changing CRAP thresholds, test membership, authority semantics, or protected
  evidence trust merely to obtain a pass;
- live GitHub dispatch, runner registration, forest1 mutation, release
  qualification, and publication;
- deleting historical failed attempts; and
- retrying the canopy package or adversarial acceptance package inside this
  implementation package.

## Declared Write Set

- `crates/openwepp-gate-planner/**`
- `tools/local_ci/**`
- `tools/release/README.md`
- `tools/release/run_adjudicated_crap_gate.sh`
- `tools/release/check_adjudicated_crap.py`
- `tools/release/run_release_candidate_gates.sh`
- `gate-policy/v1/**`
- `tests/python/test_testgate.py`
- `tests/integration/testgate_*`
- `tests/fixtures/testgate/**`
- `.config/nextest.toml`
- `.github/workflows/testgate-conservative.yml`
- `.github/workflows/testgate-shadow.yml`
- `.github/workflows/release-gates.yml`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/codex_exec_plans.md`
- `docs/prompt_templates/**`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/templates/**`
- `docs/ROADMAP.md`
- `docs/work-packages/20260720-testgate-pre-heavy-closure-audit-001/**`
- `Cargo.lock`

No other tracked path is writable without amending this package before the
edit and obtaining review of the widened authority.

## Architecture Contract

The pre-heavy audit is a planner-owned typed artifact, not a shell checklist.
Its versioned schema and acceptance rules are summarized in
[`artifacts/pre-heavy-audit-contract.md`](artifacts/pre-heavy-audit-contract.md).
The local helper orchestrates it but cannot reinterpret `BLOCKED` or `INVALID`
as ready. The executor consumes the audit's node IDs, argument arrays,
inventory, identities, prerequisite edges, and output namespaces verbatim. The
verifier independently recalculates inputs and enumerates the current expected
inventory, compares both to the admitted values, and confirms that exact audit
ID was consumed. Independent verification cannot substitute a new inventory
into execution.

Package bootstrap is validation, not execution authority. It validates a new
working-tree package against the canonical schema and reports
`SCAFFOLD_COMMIT_REQUIRED`; the package must then exist in the authenticated
base commit before an increment can be admitted. It cannot let uncommitted
package bytes retroactively authorize changed paths. The terminal plan must
bind the current package digest and reject every changed path not authorized by
both the base package and current declared write set.

Attempt directories are immutable after terminal status. Cache entries are
content-addressed and scoped by every identity that can change semantics.
Source, Git index, measurement output, receipt, and evidence paths never share
mutable cache keys. A cache hit is recorded and independently verifiable.
Every gate definition has a policy-owned `LIGHT` or `HEAVY` execution cost
class. The executor exposes separate light and heavy transitions and cannot
reach a heavy node from its initial loop. On recovery, it imports successful
per-node receipts that are current and target-reusable under §10.4 by verified
receipt ID, and records the exact trust/reuse/context reason for every rejected
receipt before rerun. Trusted workflows upload and index
attempt records before aggregate receipt creation and can re-ingest them after
a runner or job reset.

## Implementation Plan

### Phase A: freeze contracts and failing fixtures

1. Reconcile this defect inventory against the current planner, helper,
   schemas, workflows, and retained canopy/adversarial failure evidence.
2. Add schema fixtures and focused failing tests for all eleven defect classes.
3. Freeze report status, reason codes, bootstrap constraints, attempt state
   transitions, reuse decisions, and duplicate-execution rules before
   production edits.
4. Freeze the complete adversarial cases in
   `artifacts/acceptance-matrix.md`; producer-only unit evidence cannot close a
   real helper or trusted-workflow case.

### Phase B: implement audit and lifecycle controls

1. Add the typed audit model, canonical serialization, stable identifier,
   planner command, validation, and human renderer.
2. Implement scaffold-only validation, base-commit admission, and
   current-package terminal reconciliation.
3. Implement cheap prerequisite aggregation, final-path audit, inventory
   freezing, identity preflight, immutable attempt allocation, cache guards,
   and persistent run/defect ledger.
4. Make the executor and verifier require and consume the exact audit artifact.
5. Integrate the command into the local helper and trusted workflows ahead of
   every heavy node.
6. Split executor orchestration into enforced `LIGHT` and `HEAVY` stages; add
   durable per-node receipt checkpoints, trusted-run upload/re-ingestion, and
   invalidation-aware cross-attempt resume.
7. Add new modules instead of growing `planner.rs`, `executor.rs`, or
   `verifier.rs` past 3000 lines; record decomposition rationale for every
   touched 2000+ line file and finish any split required by line-count policy.

### Phase C: eliminate duplicate work

1. Instrument one full Nextest execution to emit both functional results and
   LCOV input.
2. Prove exact test-inventory parity, required coverage completeness, semantic
   equivalence, acceptable runtime, and CRAP input compatibility.
3. When proven, select the combined node and reject duplicate full/coverage
   nodes. If not proven for a platform or gate definition, retain separate
   nodes with a typed, report-visible separation reason; never silently claim
   deduplication.
4. Prove documentation-only changes invalidate documentation roots without
   invalidating unchanged executable roots.

The performance baseline uses current compatible same-host receipts; do not
rerun separate heavy paths solely to manufacture a baseline. Record full-only
and coverage-only medians from at least three retained compatible receipts when
available, plus setup, compilation, test, instrumentation, and report time. The
single combined candidate passes economy only when its wall time is no more
than 120% of the coverage-only median and no more than 80% of the summed
full-plus-coverage medians. If compatible history is insufficient or either
threshold fails, retain separate nodes with a typed `COMBINATION_NOT_ADOPTED`
reason and a measured follow-up; parity or speed is never assumed.

### Phase D: acceptance and disposition

1. Run the focused unit, schema, integration, failure-injection, and workflow
   contract tests.
2. Execute the adversarial fixture through the real local helper. It must stop
   each defect before heavy launch and produce one `READY` audit after all
   defects are corrected.
3. Generate the exact terminal plan. Run every selected gate once, using the
   required heavy-run subagent for heavy nodes, and retain all attempt/timing
   records.
4. Complete dual independent review, disposition every finding, fix accepted
   findings, and rerun only invalidated nodes.
5. Complete dual terminal verification, archive the prompt, update catalogs,
   and record the final disposition.

## Command And Interface Contract

The implementation may refine option names only through a reviewed package
amendment. The intended CLI transaction from the repository root is:

```text
target/debug/openwepp-gate-plan validate-package --repo . \
  --base <base> --package <package.md> --output <package-audit.json>
target/debug/openwepp-gate-plan run --repo . --plan <terminal-plan.json> \
  --stage light --artifact-root <attempt> --output <light-receipts.json>
target/debug/openwepp-gate-plan pre-heavy-audit --repo . \
  --plan <terminal-plan.json> --light-receipts <light-receipts.json> \
  --artifact-root <attempt> --ledger <ledger.jsonl> \
  --output <pre-heavy-audit.json>
target/debug/openwepp-gate-plan run --repo . --plan <terminal-plan.json> \
  --stage heavy --audit <pre-heavy-audit.json> --resume <ledger.jsonl> \
  --artifact-root <attempt> --output <receipt.json>
```

`tools/local_ci/testgate.py --execute` remains the ordinary operator entry
point and performs this transaction without shell interpolation. The new schema
is `gate-policy/v1/schemas/pre-heavy-audit.schema.json`; valid and invalid
fixtures live under `gate-policy/v1/fixtures/{valid,invalid}/`. Per-node receipt
schemas may be added alongside it.

Focused commands expected during implementation are:

```text
cargo nextest run -p openwepp-gate-planner --profile quick
.venv/bin/python -m unittest tests/python/test_testgate.py
markdown-doc lint --path docs/standards --path docs/work-packages/20260720-testgate-pre-heavy-closure-audit-001 --path tools/local_ci/README.md --path tools/release/README.md
git diff --check
```

Each command must report zero failures. The authenticated terminal plan may add
nodes; it may not remove these focused obligations when their inputs changed.

## Acceptance

- [ ] One canonical command produces a schema-valid `READY`, `BLOCKED`, or
  `INVALID` pre-heavy report and no heavy node starts unless it is `READY`.
- [ ] The report covers the exact ten checks in the canonical standard and is
  consumed by the executor; the verifier independently enumerates and compares
  the current inventory without replacing the admitted inventory.
- [ ] Machine-owned `LIGHT`/`HEAVY` classes and an enforced two-stage executor
  keep a heavy-spawn sentinel at zero for every non-`READY` audit state.
- [ ] A new package receives a useful scaffold-only validation result and must
  be committed before execution admission; stale-base authorization, malformed
  headings, undeclared paths, and retroactive widening fail closed.
- [ ] Diff hygiene, docs/schema checks, artifact/prompt completeness, and `.rs`
  line-count governance fail before any heavy process is spawned.
- [ ] Toolchain, environment, binary, fixture, policy, feature, runner, and
  concurrency mismatches fail during preflight.
- [ ] Attempt roots are fresh and immutable; collisions, output aliasing,
  source/index mutation, and cache poisoning fail closed.
- [ ] Receipt verification proves safe reuse across documentation-only edits
  and rejects changed executable or authority inputs.
- [ ] Combined full regression plus LCOV/CRAP proves inventory and semantic
  parity, or the report records a typed reason for separate execution. Proven
  combinations cannot schedule the same full inventory twice.
- [ ] Attempts, timings, cost classes, failures, retries, cache hits, and linked
  tooling defects persist in append-only ignored history outside `/tmp`.
- [ ] Trusted-run attempt records, including pre-receipt failures, survive a
  runner/job reset through digest-bound upload, indexing, and re-ingestion.
- [ ] After a late-node failure, a new attempt imports every successful
  per-node receipt that is current and target-reusable under §10.4. Every
  rejected receipt records its exact trust/reuse/context reason, including
  `SAME_EXECUTION` after runner, job, or workflow-attempt change.
- [ ] One infrastructure retry is retained; recurrence of the same cause blocks
  another heavy retry until the linked tooling defect is resolved or a bounded
  external-outage authority is recorded.
- [ ] The adversarial acceptance matrix covers non-`READY` spawn counters,
  mid-DAG fail/resume, active-run/newest-pending/concurrency timeout, runner
  reset, parity-proven and parity-unproven paths, and selective post-review
  invalidation in addition to the historical failure chain.
- [ ] No science, threshold, authority, trust, or test-membership weakening is
  used to close the package.
- [ ] Dual review and dual terminal verification leave no finding
  undispositioned; `.rs` line-count governance passes.

## Prospective Gate Plan

The authenticated intent plan created during execution owns exact selection.
The minimum expected nodes are described in
[`artifacts/prospective-gate-plan.md`](artifacts/prospective-gate-plan.md).
Focused edit-loop tests run before heavy work. The new pre-heavy audit must be
`READY` before any selected critical/campaign node. Successful nodes are reused
unless their bound inputs change; reassurance reruns are forbidden.

The scaffold line-count baseline is
[`artifacts/line-count-baseline.md`](artifacts/line-count-baseline.md).
`planner.rs`, `executor.rs`, and `verifier.rs` already require warning-level
decomposition treatment; new audit behavior belongs in cohesive new modules.

## Review And Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only reviewers for scaffold and
implementation review, two independent read-only terminal verifiers, and one
`comparator_suite_runner` heavy-run role with write access limited to ignored
execution/evidence roots; expected outputs are compact findings, dispositions,
commands, timings, artifact paths, and `PASS`/`HOLD`/`FAIL` verdicts.

Subagent requirement: REQUIRED. Reviewers must not read each other's draft
before submitting. The parent must dispatch `comparator_suite_runner` for every
selected heavy batch/closure/comparator node and must not duplicate those runs.
No role may commit, push, dispatch GitHub workflows, or edit outside its bounded
write set.

## Security Impact

This package changes the mechanism that authorizes commands and evidence.
Malformed package authority, shell interpolation, path traversal, symlink
escape, environment injection, artifact replacement, cache poisoning, receipt
replay, inventory omission, and concurrency collision must fail closed. Run all
applicable source-level anti-evasion and required-suite obligation guards
selected by the terminal plan.

## Idempotence And Recovery

Every attempt receives a new content-addressed directory and append-only ledger
entry. A terminal attempt is never overwritten. Recovery verifies the prior
record, allocates a new attempt, and links it to the cause and tooling defect.
Only current verified receipts may be reused. A failed migration must leave the
existing conservative TESTGATE entry point usable; it must not silently fall
back around the new audit.

## Surprises And Discoveries

- The adversarial rerun proved that the local helper reads package authority
  only from the selected base commit and requires an exact
  `## Declared Write Set` heading. That is incompatible with a newly scaffolded
  package using the previously accepted `## Intended Write Set` spelling.
- The first staged-light attempt stopped during preflight before any node spawn
  because `CARGO_HOME`, `RUSTUP_HOME`, and `RUSTUP_TOOLCHAIN` were treated as
  required even though Cargo and rustup define standard defaults. The durable
  ledger records `TGCA-ENV-OPTIONAL-001`; the executor now projects optional
  allowlisted variables when present while continuing to require `PATH`.
- Planner/executor reconstruction takes roughly 60-90 seconds on this
  worktree, materially more than any individual light node. This is retained
  as timing evidence for follow-up qualification rather than hidden as test
  time.
- The first complete audit report correctly returned `BLOCKED` with nine
  passing checks because the unchanged package path was absent from the exact
  diff. Updating this progress record makes the package part of the terminal
  authorization instead of weakening package admission.
- The first exact-head audit reached `READY`, but static inspection before
  delegation found that heavy preflight would reject the audit-bound light
  outputs as ordinary collisions. No heavy process was dispatched. The
  handoff now admits only existing light outputs whose node checkpoint and
  artifact digests match the exact audit; mutation remains fail-closed.

## Decision Log

- Decision: make the audit one typed planner artifact shared by execution and
  verification. Rationale: independently authored checklists and inventories
  recreate drift and cannot mechanically block heavy launch. Date/author:
  2026-07-20, scaffold author.
- Decision: implement tooling defects in this package rather than record them
  only as process lessons. Rationale: the failures are reproducible and owned
  by the gate lifecycle. Date/author: 2026-07-20, scaffold author.
- Decision: require combined full/coverage execution only after measured parity.
  Rationale: avoiding duplicate work cannot weaken regression or coverage
  semantics. Date/author: 2026-07-20, scaffold author.

## Outcomes And Retrospective

The scaffold and canonical governance amendment are reviewed and ready for an
implementation kickoff. Implementation and its execution gates have not
started. Both independent scaffold reviews initially held; all findings were
accepted, patched, and re-reviewed to `PASS`. No finding is deferred or open.
