# Stage 3 Legacy Predecessor Bridge Reconciliation

Status: `queued / result-blind protocol and scaffold review required`

Date: `2026-08-06`

Package ID:
`20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001`

Plan class: `Critical characterization and evidence-custody reconciliation`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
Keep Progress, Surprises & Discoveries, Decision Log, and Outcomes &
Retrospective current throughout execution.

## Objective

Reproduce or truthfully reject the frozen Snowbird schema-v5 predecessor
estimand of `+170.2536089 MJ m^-2` from the same daily source, forcing, water-
year windows, and reduction used by the current schema-v6 legacy estimator of
`+188.8725288356066 MJ m^-2`. Locate the first source/term/state divergence
before window reduction and partition it across source revision, daily reset,
first control-volume projection, same-hour transition preparation/cadence, and
active-to-lower internal-conduction lineage.

If the exact bridge closes, adjudicate only the already registered four-site
operator-mechanics classes. If it does not close, preserve
`PREDECESSOR_NOT_REPRODUCED`, identify the precise custody or implementation
boundary, and repair in-scope evidence custody. This package makes no physics,
persistence, promotion, ownership, or cutover claim.

## Frozen Intake

- Scaffold base: `5fa67643762146c15e622f5bb115d5117d2367d7`.
- Historical source: `2d035638a9819961a393207cd4813712d64bddcf`.
- Current admitted execution source:
  `5ebfc5135b80d250cb6b38d1b6241a7d2a72d6c5`.
- Historical trace SHA-256:
  `621bd3f91076403aac45737c387954e89c4760a12698e36952dec6dd72b94716`.
- Current v3 result SHA-256:
  `3b885fa0f04201744da5c24766d413cd2e74f1273021a1a35d6fd0f7227f691e`.
- Historical median: `+170.2536089 MJ m^-2`.
- Current legacy median: `+188.8725288356066 MJ m^-2`.
- Difference: `+18.618919935606613 MJ m^-2`, or
  `10.935991346029326%`.
- Frozen cohort: Snowbird WY1990--2024, October 1 through earliest maximum
  positive observed SWE date inclusive, Python `statistics.median` across the
  35 water-year window sums.
- Current production owner: CoE, unchanged and protected.

## Implementation Intent

Intent is `characterization and evidence-custody reconciliation`, not science
implementation, empirical calibration, or independent validation. The
calibration evidence and identifiability statuses are `NOT_APPLICABLE`.

The first execution path is model-free source-revision and retained-trace
reconciliation. Additive Rust observability is permitted only if retained
evidence cannot localize the first divergence. Any such diagnostic must be
contract-first, default-off, evaluation-only, consumer-forbidden outside the
independent package analyzer, and exactly absent from disabled/public output.

## Included Scope

- Verify immutable custody for historical trace, current v3 traces, source
  commits, fixture/runfile/forcing identities, observation windows, and binary
  provenance.
- Execute a result-blind source-revision checkpoint search from historical
  `2d035638a9` through the admitted v3 source on the exact Snowbird fixture.
- Reconstruct daily and water-year values independently before medians.
- At the first divergent source checkpoint, reconcile term totals, support,
  first effective state, substep cadence, transition preparation, endpoints,
  and internal conduction in temporal order.
- Add package-local deterministic tools/tests and retained result custody.
- If necessary, amend SC-SNOWFREEZE-001 and add narrow default-off diagnostic
  Rust/test surfaces after result-blind dual review and contract gate.
- Update roadmap, catalog, DRAFT assurance source identity when contract
  authority changes, reviews, validation, and terminal disposition.

## Excluded Scope

- Turbulent, radiative, advected-heat, conduction, or snow thermodynamic
  formula changes.
- Parameter fitting, calibration, observed-data acceptance, or transfer claims.
- Persistent shadow state, cross-day chronology authority, terminal meltout,
  land-surface energy, soil/frost coupling, selector/default changes, Stage 3
  promotion, CoE retirement, or cutover.
- Public schema changes, WAT/HBP/PASS changes, assurance review/approval,
  release transfer, vendoring, or publication.
- Treating agreement with the legacy predecessor as correctness authority.

## Intended Write Set

- `docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/**`
- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/work-packages/README.md`
- conditional contract-first files:
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - `docs/specifications/science-contracts/index.md`
  - `tests/integration/snow_stage3_legacy_predecessor_bridge_contract.rs`
- conditional default-off diagnostic files:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00h_snow_stage3_evaluation_trace.rs`
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00i_snow_stage3_reconciliation_trace.rs`
  - focused module and integration tests under those same source modules.
- conditional DRAFT assurance-source adoption files selected by the canonical
  assurance plan, plus exactly the snow review-draft projection if required.

All retained executions write only below
`target/snow_stage3_legacy_predecessor_bridge_reconciliation/`. Historical
target namespaces are read-only. Any path expansion requires a prospective
package amendment and independent review before the new write.

## Frozen Protocol

1. Prove hashes, source ancestry, exact fixture/runfile/sidecar inputs, daily
   dates, observed windows, and output paths before reading result values.
2. Build the exact runner binary for each selected source checkpoint in an
   isolated local worktree/cache. Record commit, binary hash, run argv, runtime,
   and output manifest. Never overwrite a historical or current retained trace.
3. Run only the Snowbird sequential diagnostic lane with the frozen daily
   scheduler, source, forcing, and selector semantics applicable to that source.
4. Reduce every checkpoint with one independent standard-library consumer.
   Require exact daily row identity, 35 water-year windows, and term closure.
5. Use ordered source checkpoints to find the first commit interval whose
   legacy estimand differs. Agreement is evidence identity, not correctness.
6. Within the first divergent interval, compare the earliest evaluated
   `(day, hour, substep)` and decompose:
   source/reset -> first projection -> surface terms -> active conduction ->
   state application/removal -> transition preparation -> next endpoint.
7. Accept a causal bridge class only when one prospectively named transition
   changes from exact predecessor agreement to exact current agreement while
   all upstream operands close. Otherwise emit
   `MULTIFACTOR_OR_UNOBSERVED_PREDECESSOR_BOUNDARY`.
8. Preserve every contrary or failed execution. A deterministic semantic
   failure is not rerun into a pass.

## Protected Invariants

- CoE remains the sole production melt owner.
- Disabled evaluation is byte-exact and emits no new public field.
- Existing schema v4/v5/v6 semantics and retained evidence are immutable.
- HBP, PASS, and WAT remain byte-exact on any model execution.
- No observed data, fixtures, frozen traces, source history, or authority suite
  bindings are mutated.
- No legacy agreement is labeled physical correctness, validation, or fitness.

## Phase Plan

### Phase A -- Scaffold And Result-Blind Admission

Freeze the protocol, hashes, write set, claim taxonomy, failure policy,
required reading, line-count posture, security/custody analysis, and direct
validation selection. Commit the scaffold before implementation edits. Obtain
independent science/protocol and Rust/custody review; disposition all findings.

### Phase B -- Model-Free Bridge Search

Implement the independent package consumer and source-checkpoint runner. Test
hash, join, schema, window, exact reduction, source-checkpoint, first-divergence,
term closure, and false-attribution failures. Execute read-only reconciliation
against retained traces and exact checkpoint binaries.

### Phase C -- Conditional Contract-First Observability

Only if Phase B proves an unobserved boundary, amend SC-SNOWFREEZE-001 first,
add contract-derived tests, record the pre-implementation contract gate, then
add the smallest default-off diagnostic required to resolve that boundary.
Repeat focused result-blind review before any new result-bearing execution.

### Phase D -- Frozen Execution And Reconstruction

Execute the admitted Snowbird bridge, retain complete custody, independently
reconstruct daily/annual/median results and the earliest divergence, and emit
only frozen decision classes. Do not tune after result inspection.

### Phase E -- Review, Validation, And Closure

Complete independent science, Rust/custody, and consumer review; disposition
every finding; run the exact critical validation selected from the terminal
diff; adopt any amended DRAFT assurance source without creating lifecycle
authority; complete two independent terminal verifications; update roadmap and
catalog; archive the prompt; commit stable closure.

## Validation Requirements

- Package Python tests and real retained-consumer reconstruction.
- Rustfmt, affected and workspace warnings-denied Clippy, doctests, focused
  contract/module/runner tests, quick, frost, and full workspace Nextest if
  conditional Rust or canonical contract changes occur.
- Contract profile, Binding Exposure, SC unit compliance, schema/JSON,
  assurance validate/plan/export guards, generated review-draft drift check,
  Markdown lint, diff hygiene, line-count governance, and cargo-deny when the
  exact diff makes them applicable.
- Direct anti-evasion guards if any external-authority fixture, cohort, or
  required-case binding is touched.
- Exact terminal base-to-head write-set reconciliation and clean-worktree
  evidence.
- No TESTGATE use.

Coverage/CRAP is observational and not selected. This critical campaign
increment requires the comparator-suite runner for heavy checkpoint and
workspace closure commands.

## Review And Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to science/protocol reviewers, Rust/custody reviewers,
independent consumer reviewers, terminal verifiers, and the
`comparator_suite_runner` for checkpoint/cohort and heavy closure execution.
Expected outputs are compact findings, exact commands/results, hashes, and log
paths. Review and verification roles are read-only; the comparator role may
write only ignored target evidence. The primary agent owns all tracked edits
and finding disposition.

Two independent result-blind reviews must pass before result execution. After
results, independent science, Rust/custody, and consumer reviews must pass.
Two independent terminal verifiers must bind one exact clean closure candidate.
Every review and verifier checks gate legitimacy, claim limits, write-set
custody, line counts, and absence of undispositioned findings.

## Exit Criteria

- Exact historical/current custody and 35-window reductions reconstruct.
- The first divergent source interval and earliest divergent temporal/state
  operand are identified, or the retained evidence proves and names the exact
  unobserved boundary.
- Every emitted class follows the frozen decision predicates without tuning.
- All current-scope validation, review, assurance-impact, and verification
  requirements have direct evidence and no open finding.
- Production physics/output/defaults/CoE authority remain unchanged.
- Roadmap names the evidence-supported next action; no in-scope missing gate is
  relabeled future scope.

## Security And Data Impact

Local tracked source and ignored retained artifacts only. No network, secret,
credential, protected-data, remote mutation, or public export is required.
Subprocesses use explicit argv and working directories. Historical traces,
fixtures, and source commits are immutable inputs; new outputs are confined to
the package namespace and content-hashed.

## Progress

- [x] (2026-08-06) User authorized scaffold and end-to-end execution.
- [ ] Commit result-blind scaffold and obtain dual protocol review.
- [ ] Implement and test the independent bridge search.
- [ ] Complete any necessary contract-first diagnostic increment.
- [ ] Execute, reconstruct, review, validate, verify, roadmap, and close.

## Surprises & Discoveries

- Observation: the historical trace is schema v4/v5 custody from exact source
  `2d035638a9`; the current legacy estimator is reconstructed from schema-v6
  tuples at `5ebfc5135`. Their identical labels do not establish identical
  source revision or transition preparation.

## Decision Log

- Decision: treat predecessor agreement as evidence-custody reconciliation,
  not correctness authority. Rationale: legacy comparator evidence is A5 and
  the campaign already has independent physical holds. Date/Author:
  2026-08-06 / Codex.
- Decision: start with source-revision checkpointing before adding diagnostics.
  Rationale: retained history may localize the bridge without enlarging the
  kernel surface. Date/Author: 2026-08-06 / Codex.

## Outcomes & Retrospective

Pending execution.
