# Stage 3 Legacy Predecessor Bridge Reconciliation

Status: `executing / prospective protocol correction under dual review`

Date: `2026-08-06`

Package ID:
`20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001`

Plan class: `Critical characterization and evidence-custody reconciliation`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
Keep Progress, Surprises & Discoveries, Decision Log, and Outcomes &
Retrospective current throughout execution.

## Objective

Reconcile the frozen Snowbird schema-v4 predecessor estimand of
`+170.2536089 MJ m^-2` with the current schema-v6 legacy estimand of
`+188.8725288356066 MJ m^-2` without presuming that they used the same forcing.
Execute a two-source by two-forcing endpoint matrix first, reconstruct effects
per water year before any median, and prohibit source/state attribution until a
fixed-forcing lane closes. Only if a source effect remains may the package
localize its first source/term/state divergence before window reduction.

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
- Historical trace schema: `openwepp-r7h-direct-production-snow-trace-v4`.
- Historical canonical `p8.cli` SHA-256:
  `10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7`.
- Current development-precipitation `p8.cli` SHA-256:
  `c673145ee7fd41e71e3f2e21c529fba2d12691abd5f0f055444e621fb0b80afb`.
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
- Static custody correction: the two retained medians used different climate
  files. Their direct difference is not a same-forcing source-revision effect.
- Historical executable disposition: `HISTORICAL_BINARY_NOT_RETAINED`. Its
  reported hash remains documentary custody; a fresh exact-source build is a
  semantic replay, not a byte-identical reconstruction of that executable.

## Implementation Intent

Intent is `characterization and evidence-custody reconciliation`, not science
implementation, empirical calibration, or independent validation. The
calibration evidence and identifiability statuses are `NOT_APPLICABLE`.

Before implementation or result execution, amend SC-SNOWFREEZE-001
prospectively so predecessor-reproduction predicates bind forcing identity and
distinguish endpoint replay, forcing-stratified reconciliation, versioned
estimand reconciliation, and current-schema reproduction. The first execution
is then the frozen endpoint matrix. Additive Rust observability is permitted
only if fixed-forcing endpoint evidence leaves a source effect unresolved. Any
such diagnostic must be contract-first, default-off, evaluation-only,
consumer-forbidden outside the independent package analyzer, and exactly absent
from disabled/public output.

## Included Scope

- Verify immutable custody for historical trace, current v3 traces, source
  commits, fixture/runfile/forcing identities, observation windows, and binary
  provenance.
- Execute the frozen two-source by two-forcing endpoint matrix, including a
  same-source/same-forcing disabled control for every enabled arm.
- If and only if the fixed-forcing endpoints retain a source effect, execute
  the prospectively frozen build-input checkpoint groups from `2d035638a9`
  through the admitted v3 source on one fixed forcing.
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
  - `Cargo.toml`
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
  - `crates/openwepp-runner/src/hillslope/tests03/stage3_evaluation_publication_parity.rs`
  - focused module and integration tests under those same source modules.
- conditional DRAFT assurance-source adoption files selected by the canonical
  assurance plan, plus exactly the snow review-draft projection if required.

All retained executions, isolated local clones, Cargo caches, copied binaries,
generated runfiles, outputs, logs, and manifests write only below
`target/snow_stage3_legacy_predecessor_bridge_reconciliation/`. Historical
target namespaces are read-only. No `git worktree` is used. Any path expansion
requires a prospective package amendment and independent review before the new
write.

## Frozen Protocol

1. Freeze and prove all source, trace, binary, fixture, forcing, runfile,
   selector, date/window, unit, duration, schema-adapter, tolerance,
   aggregation-order, and operand-lineage identities before reading results.
2. Create isolated local clones below the target namespace at each exact source
   SHA. Build with a scrubbed environment, checkpoint-specific
   `CARGO_TARGET_DIR`, and `cargo build --locked --offline --release -p
   openwepp-runner --bin openwepp-cli-hill`. Record the build-input digest,
   `Cargo.lock`, toolchain/target/linker/OS identity, binary size/hash, argv,
   cwd, stdout/stderr hashes, exit status, and generated runfile/semantic-input
   manifest. Refuse overwrite; verification is read-only.
3. Execute endpoint cells `E00=old source/canonical forcing`, `E01=old
   source/development forcing`, `E10=current source/canonical forcing`, and
   `E11=current source/development forcing`. Every enabled legacy-selector arm
   has a same-source/same-forcing disabled control. The explicit evaluation
   selector is absent in legacy arms; sublimation is explicitly disabled and
   all other science selectors are frozen. At the current source, additionally
   prove legacy-selector and explicit-selector equivalence.
4. Independently reconstruct each cell from primitive trace fields without
   importing or reusing execution-runner reduction helpers. Require exact daily
   row identity, 35 water-year windows, term closure, mass/cold-content closure,
   internal-conduction cancellation, and protected-output identity.
5. Compute per-water-year effects before medians:
   `source_canonical=E10-E00`, `source_development=E11-E01`,
   `forcing_old=E01-E00`, `forcing_current=E11-E10`, and
   `interaction=(E11-E10)-(E01-E00)`. Do not algebraically combine medians of
   separately reduced distributions.
6. Only when at least one fixed-forcing source effect remains outside the
   frozen tolerance, traverse every prospectively frozen distinct binary
   build-input closure on one fixed forcing. Collapse commits only when the
   recorded closure digest is identical; never select a checkpoint from result
   values and never assume monotonicity.
7. Within a first divergent source interval, compare the earliest evaluated
   `(day, hour, substep)` and decompose:
   source/reset -> first projection -> surface terms -> active conduction ->
   state application/removal -> transition preparation -> next endpoint.
8. Treat the earliest differing field as descriptive localization only. Accept
   a causal bridge class only after a controlled single-axis substitution
   changes predecessor agreement to current agreement and closes the tuple,
   day, water-year, and median deltas while excluding alternatives. Otherwise emit
   `MULTIFACTOR_OR_UNOBSERVED_PREDECESSOR_BOUNDARY`.
9. Preserve every contrary or failed execution, including stdout/stderr and
   manifests. A deterministic semantic
   failure is not rerun into a pass.

## Protected Invariants

- CoE remains the sole production melt owner.
- Disabled evaluation is byte-exact and emits no new public field.
- Existing schema v4/v5/v6 semantics and retained evidence are immutable.
- Within each source/forcing pair, enabled and disabled HBP, PASS, and WAT are
  byte-exact; disabled execution emits no diagnostic trace and leaves CoE state
  authoritative. Cross-revision byte identity is not required.
- No observed data, fixtures, frozen traces, source history, or authority suite
  bindings are mutated.
- No legacy agreement is labeled physical correctness, validation, or fitness.

## Phase Plan

### Phase A -- Scaffold And Result-Blind Admission

Freeze the protocol, hashes, 2x2 endpoint matrix, build-input checkpoint
groups, write set, claim taxonomy, failure policy,
required reading, line-count posture, security/custody analysis, and direct
validation selection. Commit the scaffold before implementation edits. Obtain
independent science/protocol and Rust/custody review; disposition all findings.

### Phase B -- Model-Free Bridge Search

Implement separate execution-custody and independent-consumer tools. The
consumer must not import runner reduction code. Test
hash, join, schema, window, exact reduction, source-checkpoint, first-divergence,
term closure, and false-attribution failures. Execute read-only reconciliation
against retained traces and exact checkpoint binaries.

### Phase C -- Conditional Contract-First Observability

Amend SC-SNOWFREEZE-001 prospectively to correct forcing identity before Phase
B results, add contract-derived tests, and record the pre-implementation
contract gate. Only if the forcing-matched endpoints then prove an unobserved
source boundary, add the smallest default-off diagnostic required to resolve it.
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

Quick, frost, and full workspace Nextest are unconditional because this package
changes canonical contract authority. The comparator-suite runner owns the
heavy checkpoint matrix and these terminal workspace runs.

## Frozen Decision State Machine

Outcome precedence is:

1. `INPUT_OR_ENDPOINT_REPLAY_FAILURE` when immutable source/input/output
   custody or either retained anchor cannot be reconstructed under its original
   source and forcing.
2. `FORCING_IDENTITY_DIFFERENCE` whenever the compared runs use different
   forcing hashes; this suppresses direct source/state attribution.
3. `FORCING_STRATIFIED_ENDPOINTS_RECONCILED` when all four endpoint cells and
   controls close under their exact identities.
4. `SOURCE_INVARIANT_WITHIN_FORCING` when both fixed-forcing source effects are
   within the frozen tolerance.
5. `VERSIONED_ESTIMANDS_RECONCILED` when the retained v4 and v6 estimands are
   explained by the prospectively frozen forcing/source decomposition.
6. `CURRENT_V6_FORCING_MATCHED_PREDECESSOR_REPRODUCED` only when current schema
   v6 under the predecessor forcing reproduces the predecessor reference under
   the corrected contract predicate.
7. A named causal source/state class only after controlled substitution closure.
8. `MULTIFACTOR_OR_UNOBSERVED_PREDECESSOR_BOUNDARY` otherwise.

The classes are multilabel where their predicates coexist. Technical package
PASS may coexist with a campaign causal HOLD. A HOLD is legitimate only for an
immutable missing input/executable, an exact historical source that cannot be
built or executed after the frozen compatibility procedure, or a genuinely
unobserved boundary after all in-scope instrumentation and tests are complete.
Missing package tools/tests, incomplete checkpointing, or required in-scope
instrumentation cannot justify HOLD.

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
- [x] (2026-08-06) Committed initial result-blind scaffold at `19e8c5cde`.
- [x] (2026-08-06) Dual static review found the forcing/schema/custody defects;
  no result-bearing execution had begun.
- [ ] Commit the prospective protocol correction and obtain dual PASS review.
- [ ] Implement and test the independent bridge search.
- [ ] Complete any necessary contract-first diagnostic increment.
- [ ] Execute, reconstruct, review, validate, verify, roadmap, and close.

## Surprises & Discoveries

- Observation: the historical trace is schema v4 custody from exact source
  `2d035638a9`; the current legacy estimator is reconstructed from schema-v6
  tuples at `5ebfc5135`. Their identical labels do not establish identical
  source revision or transition preparation.
- Observation: the predecessor used canonical forcing `10c1ede1...`; v3 used
  development forcing `c673145e...`. All other fixture inputs are byte-identical.

## Decision Log

- Decision: treat predecessor agreement as evidence-custody reconciliation,
  not correctness authority. Rationale: legacy comparator evidence is A5 and
  the campaign already has independent physical holds. Date/Author:
  2026-08-06 / Codex.
- Decision: start with source-revision checkpointing before adding diagnostics.
  Superseded prospectively before results by the 2x2 endpoint matrix after
  custody review proved the retained runs used different forcings.
  Date/Author: 2026-08-06 / Codex.
- Decision: correct SC-SNOWFREEZE-001 before execution so canonical authority
  binds forcing-matched predecessor reproduction rather than an unqualified
  scalar. Rationale: package evidence cannot override canonical policy.
  Date/Author: 2026-08-06 / Codex.

## Outcomes & Retrospective

Pending execution.
