# SNOW-MASS-TRANSITION-LEDGER-PERSISTENCE

Status: `queued / scaffolded / not executed`

Date: `2026-08-03`

Plan class: `Contract-governed behavior-neutral architecture and diagnostic-cost containment`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
The `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` sections must remain current during execution.

## Purpose / Big Picture

Make the two mass-transition boundaries exposed by the Snowbird investigation
a durable part of openWEPP rather than campaign-only instrumentation:

1. solid snow and signed CoE melt become liquid offered to downstream snow
   handling; and
2. Stage-3 incoming liquid becomes routed liquid, signed retained-liquid
   change, refrozen liquid, and closure residual.

The package must retain one authoritative physical calculation and two linked,
compact scientific ledgers. It must not create a second snow-mass state, a
second calculation, or an independently mutable accounting authority. The
large hourly schema-v4 research payload remains available through the real
opt-in JSONL consumer, but normal trace-disabled production must not construct
or carry that verbose post-solve payload merely because the campaign once
needed it.

## Context And Orientation

`DirectSnowAccumulationMeltDiagnostics` currently embeds many 24-hour arrays in
every `DirectSnowLiquidPartition`. `DirectSnowStage3Diagnostics` combines
production-required outcome values, compact closure operands, and a large
hourly surface-energy diagnostic array. The runner writes those values only
when `OPENWEPP_R7H_SNOW_TRACE_PATH` is nonempty, but the orchestrator currently
constructs and carries the payload before the runner checks that opt-in.

The preceding package proved why the accounting separation matters. On the
real Snowbird trajectory, Stage 3 independently closes
`incoming - routed - retained_delta - refrozen = residual`, while the upstream
ledger distinguishes raw signed melt, applied pack withdrawal, and the liquid
handoff. Collapsing either boundary would again make plausible alias formulas
look correct. Keeping every hourly research field eagerly materialized is a
different decision and is not required for that scientific durability.

This package is behavior-neutral. It changes diagnostic ownership, capture
policy, and typed architecture only. It cannot adjudicate or correct pre-peak
liquid physics.

## Implementation Intent

- Intent: `behavior-neutral architecture hardening and observability-cost containment`.
- Science implementation status: `IMPLEMENTED`; no new process equation is in
  scope.
- Calibration evidence: `NOT_APPLICABLE`.
- Independent validation: `NOT_APPLICABLE`.
- Existing Snowbird output is diagnostic and regression evidence only.
- Serialization posture: schema-v4 JSONL field names, meanings, rows, and
  values remain unchanged when verbose tracing is enabled.
- Public-output posture: WAT, HBP, PASS, watershed, manifest, and ordinary CLI
  outputs remain byte-identical.

## Durable Architecture Invariant

The terminal implementation must satisfy all of these statements:

1. There is one authoritative snow calculation and one authoritative state
   mutation path.
2. Two linked compact ledgers expose the two distinct mass-transition
   boundaries without recomputing physics.
3. Every compact-ledger operand is copied or moved from an exact producer
   result with declared units and time basis; no operand is inferred from an
   adjacent output.
4. The downstream ledger consumes the exact liquid handoff identified by the
   upstream boundary. Shared values are linked, not independently editable.
5. Closure guards remain on the authoritative production path and do not
   disappear when verbose tracing is disabled.
6. Production-required values, including any meltwater-temperature handoff,
   are owned by a production outcome rather than hidden inside an optional
   diagnostic payload.
7. The verbose hourly payload is present only under an explicit typed capture
   request derived from the existing trace opt-in. Disabled, absent, and empty
   trace-path cases remain equivalent.
8. Opt-in capture observes the calculation; it cannot select equations,
   branches, tolerances, coefficients, or state mutations.

The package may rename the informal “double ledger,” but it may not erase the
two boundaries or introduce dual mass authority.

## Included Scope

1. Audit every field in `DirectSnowAccumulationMeltDiagnostics` and
   `DirectSnowStage3Diagnostics` as one of: production outcome, compact durable
   ledger, verbose hourly trace, or obsolete/duplicated diagnostic alias.
2. Freeze an operand-lineage table and compact ledger equations before
   production edits.
3. Amend `SC-SNOWFREEZE-001` to require the two linked ledger boundaries,
   single-authority ownership, and optional verbose-capture semantics.
4. Add contract-derived tests before production edits, including anti-alias,
   trace-disabled, trace-enabled, and public-output identity cases.
5. Separate production-required Stage-3 outcome state from optional diagnostic
   state and introduce typed linked ledger carriers.
6. Thread one explicit internal capture request from the runner to the snow
   calculation. Reuse the current environment opt-in; add no runfile setting or
   user coefficient.
7. Preserve schema-v4 JSONL exactly when tracing is enabled and prove the real
   writer consumes the optional payload.
8. Preserve compact closure guards and real-consumer independent
   reconstruction when verbose tracing is disabled and enabled.
9. Measure trace-disabled runtime, peak RSS, result-carrier footprint, and
   trace-enabled output cost against an immutable scaffold-baseline binary.
10. Keep new ledger/capture-policy code in bounded cohesive modules rather than
    extending existing 2000-line WARN hosts.

## Excluded Scope

- Any change to snow accumulation, phase, melt, density, sublimation,
  longwave, refreeze, liquid routing, cold-content, or runoff equations.
- Changes to arithmetic order, constants, thresholds, guards, selectors,
  defaults, calibration parameters, observations, fixtures, or provider
  authority.
- Re-adjudication of the Snowbird physics hypotheses, legacy-routing
  promotion, negative-melt correction, wet-compaction correction, or a new
  factorial.
- A second mass-balance calculation, shadow mass state, or diagnostic value
  that can disagree with the authoritative production result by construction.
- New required output, new user-facing schema, runfile/API setting, or network
  dependency.
- Removal, reinterpretation, or backfill of schema-v4 fields or historical
  traces.
- Opportunistic refactors outside the diagnostic-ownership and capture seam.

## Intended Write Set

- `docs/work-packages/20260803-snow-mass-transition-ledger-persistence-001/`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`
- `assurance/v2/identity.lock.json`
- `assurance/v2/reports/linear-groundwater-reservoir-recurrence/review.lock.json`
- `assurance/v2/reports/native-forest-canopy-phenology-evaluation/review.lock.json`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/report.yaml`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json`
- one generated `assurance/v2/transactions/*.json` receipt from the typed
  `adopt-report-source` transaction if the contract edit invalidates the
  declared report dependency;
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- one new bounded orchestrator module under
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/`
  for ledger/capture types and exact projections;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r3_r4.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4pqz.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00f_snow_accumulation_melt_trace.rs`
- at most one new bounded runner include under
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`
  for trace-request or formatting ownership;
- `tests/integration/*.rs`, limited to new ledger/capture/consumer contracts and
  mechanical `SC-SNOWFREEZE-001` version-pin reconciliation;
- `target/snow_mass_transition_ledger_persistence/`.

Everything else is protected. Fixture, observation, retained campaign result,
and historical trace edits are prohibited.

## Authority And Dependencies

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/work-packages/20260803-snow-prepeak-liquid-evacuation-physics-audit-001/`
- `docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/`
- Existing typed snow outcomes, diagnostics, guards, direct-runtime handoffs,
  and schema-v4 formatter named in `artifacts/architecture-boundary.md`.

Package artifacts remain evidence. Canonical science authority remains in the
`SC-*` contracts.

## Contract-First And Baseline Sequence

Production edits are prohibited until these steps complete in order:

1. commit this scaffold;
2. build the exact release runner binaries from the scaffold commit and retain
   immutable baseline binary hashes;
3. run baseline trace-disabled and trace-enabled output/performance captures;
4. complete the field-ownership, public-API, architecture, and operand-lineage
   audits;
5. amend canonical contract authority and version;
6. add contract-derived tests and deliberately separated anti-alias fixtures;
7. record the pre-implementation contract gate; and
8. edit production carriers, capture policy, and the real consumer.

No surrogate physics is authorized. Diagnostic values must be exact existing
producer operands, and the compact ledgers cannot become independent
calculators.

## Conservation / Output Acceptance

Before production edits, `artifacts/operand-lineage.md` must name every compact
ledger operand, its units, time basis, sign convention, source object, source
calculation, authority role, and linked downstream consumer. It must state the
two accepted identities and reject at least these aliases:

- raw signed CoE melt substituted for applied solid-pack withdrawal;
- positive hourly melt sum substituted without the existing redistribution
  semantics;
- top-level CoE routed melt substituted for Stage-3 routed liquid;
- CoE retained store substituted for signed Stage-3 retained change;
- retained change omitted;
- refrozen liquid double-counted; and
- producer residual accepted without operand reconstruction.

Acceptance requires independent reconstruction from real schema-v4 JSONL and
from the compact typed ledger. The two consumers must agree on the shared
operands, close within canonical tolerances, and contain rows where rejected
aliases differ materially. Producer self-consistency and one-sided bounds are
supporting evidence only.

## Capture And Compatibility Contract

The existing environment selector remains the sole trace request. The runner
must translate absent, empty, and nonempty path state into a typed internal
capture request before calling the snow calculation. The snow kernel may use
temporary hourly working state required by the authoritative solve, but after
an hour's values have served the solve it must not assemble or carry the full
verbose result payload when capture is disabled.

When capture is enabled, the real release CLI must produce byte- or
value-identical schema-v4 content relative to the scaffold baseline. If byte
identity is prevented solely by deterministic JSON formatting movement, every
row, key, ordering, numeric value, null, and string must still be identical and
the exact reason must be reviewed. No schema-v5 bump is authorized.

The execution must inventory exported Rust symbols before edits. Existing
user-facing CLI, runfile, and output surfaces are strictly preserved. An
exported Rust diagnostic type may be decomposed only after documenting whether
it is a supported external surface and selecting an additive or migration-safe
route. A breaking external API change is a hold boundary requiring separate
explicit authority; it cannot be hidden in this refactor.

## Performance And Storage Acceptance

The executor must preserve both scaffold and candidate release binaries and
record exact command, hash, size, and mtime. The frozen performance and
compatibility fixture is the retained local
`target/snow_prepeak_liquid_evacuation_physics_audit_v3/fixtures/baseline_replay/snotel_snowbird_ut`
source. Copy it into the package target namespace and hash every input before
execution; do not mutate the retained source. After one unmeasured warm-up, run
at least seven paired alternating repetitions per binary with tracing disabled.
Record all wall-time and peak-RSS samples.

- Candidate trace-disabled median wall time and median peak RSS may not exceed
  the scaffold baseline by more than 5%. An observed excess requires controlled
  rerun and diagnosis; a confirmed excess is closure-blocking.
- Ordinary trace-disabled WAT/HBP/PASS and other protected outputs must be
  byte-identical.
- Source/type evidence must prove the verbose post-solve payload is absent on
  the disabled path. A producer-only counter is insufficient.
- Trace-enabled schema-v4 output must retain baseline row count, key set,
  values, and ordering. Its bytes may not grow by more than 1%; any nonzero
  change requires exact explanation and review.
- Compact ledgers remain always available and closure-guarded. Their concrete
  type footprint and copies across production handoffs must be reported, not
  silently traded for heap churn.

These thresholds are engineering non-regression bounds, not claims that a
short benchmark proves universal performance improvement.

## Structural And Line-Count Governance

At scaffold, the directly implicated WARN hosts are:

- `infiltration_reconciliation.rs`: 2392 lines;
- `runoff_reconciliation.rs`: 2632 lines; and
- `00c_day_input_builder_impl.rs`: 2575 lines.

The package may move only the ledger/capture seam. It must record pre/post
symbol and export inventories, preserve comments and contract citations, and
avoid unrelated cleanup. New ledger/capture implementation belongs in bounded
cohesive modules. No touched WARN host may grow unless review documents why a
smaller extraction would harm cohesion; no nonexempt `.rs` file may reach 3000
lines. Every touched `.rs` file at 2000+ lines remains `WARN` and requires an
explicit disposition.

## Real Consumer Proof

Static evidence must map:

1. authoritative snow computation;
2. production outcome and state mutation;
3. upstream compact ledger;
4. exact linked handoff;
5. downstream compact ledger;
6. optional verbose payload;
7. runner trace request and handoff;
8. schema-v4 formatter and file writer; and
9. independent parser.

The real release CLI must exercise both trace-disabled and trace-enabled paths.
Wrappers, shadows, skeletons, test-only adapters, or a payload that no real
writer reads cannot close the consumer claim.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent read-only Rust/science reviewers, two independent read-only
terminal verifiers, and a `comparator_suite_runner` for heavy quick/frost/full
workspace and paired performance/comparator gates. Expected outputs are compact
findings, metrics, and artifact or log paths. Reviewers and verifiers have
read-only access; the comparator may write only normal build/test outputs under
`target/`. The orchestrator alone edits package, contract, production, and test
files.

## Phase Plan

### Phase A: Baseline, Ownership, And Authority

Commit the scaffold, retain exact scaffold release binaries, collect bounded
baseline output/performance evidence, complete the field/public-API/operand
inventory, amend the contract, author contract-derived tests, and record the
pre-implementation gate.

### Phase B: Typed Ledger And Capture Architecture

Separate production outcomes, compact linked ledgers, and optional verbose
payloads. Thread the typed capture request from the existing runner selector.
Keep the calculation, state mutation, closures, selectors, and defaults
unchanged.

### Phase C: Consumer And Compatibility Closure

Prove real trace-disabled and trace-enabled CLI paths, schema-v4 compatibility,
compact-ledger independent reconstruction, rejected-alias separation, and
protected output identity.

### Phase D: Performance And Direct Validation

Build the exact candidate release binaries; execute the paired performance
protocol; run format, warnings-denied Clippy, focused contract and consumer
tests, quick, frost, Critical full, doctests, documentation/syntax checks,
assurance identity reconciliation when triggered, line-count governance, and
exact-diff/write-set reconciliation.

### Phase E: Review, Verification, And Disposition

Complete two independent reviews, disposition every finding, remediate accepted
findings, then obtain two fresh terminal verifications. Close only when every
current-scope gate has direct evidence; otherwise record a truthful hold with a
named blocker and first actionable follow-up.

## Validation And Exit Criteria

- The contract version, invariants, obligations, binding exposure, and revision
  history consistently require one authority, two linked ledgers, and optional
  verbose capture.
- Contract-derived tests fail against the scaffold production architecture and
  pass after implementation.
- The field-ownership inventory accounts for every prior diagnostic field and
  identifies every production-required value.
- Compact ledger operands have explicit units, signs, time basis, source, and
  exact linkage; no duplicate mass authority exists.
- Independent compact-ledger and real JSONL consumers close the two boundaries
  and reject every named alias on non-aliased rows.
- Trace-disabled execution retains closure guards but neither constructs nor
  carries the verbose post-solve payload.
- Trace-enabled schema-v4 rows, keys, values, nulls, strings, and ordering match
  the immutable scaffold baseline; historical traces remain untouched.
- Protected ordinary outputs are byte-identical and the trace selector cannot
  affect physics, state, or output values.
- Paired runtime/RSS and trace-size thresholds pass with complete raw samples
  and exact binary provenance.
- Public API inventory shows parity or an explicitly reviewed additive,
  migration-safe diagnostic transition; no unauthorized breaking surface is
  present.
- No snow formula, arithmetic order, branch, threshold, selector, default,
  fixture, observation, or calibration change appears in the terminal diff.
- Applicable focused, quick, frost, Critical full, Clippy, format, doctest,
  documentation, syntax, assurance, source-identity, and real-consumer gates
  pass.
- Dual review, complete finding disposition, dual terminal verification,
  prompt archival, worker handoff, line-count governance, and exact write-set
  reconciliation pass with no deferred current-scope gate.

This diagnostic-ownership, runtime projection, and serialization change is
classified `critical`; the full workspace correctness profile is required and
must be executed by the authorized `comparator_suite_runner` when available.

## Security And Data Impact

Security impact is `none expected`. The package uses local repository fixtures
and disposable target evidence only. It must not read credentials, contact
external services, expose environment secrets, or serialize absolute
credential paths. Environment handling is limited to the existing snow-trace
selector and must preserve empty/absent equivalence.

## Progress

- [x] (2026-08-03) User directed scaffolding of the durable two-ledger package.
- [x] (2026-08-03) Inspected current typed diagnostics, producer ownership,
  direct-runtime handoffs, and opt-in JSONL writer before freezing scope.
- [x] (2026-08-03) Prepared this exact scaffold as its own local commit before
  contract or production edits.
- [ ] Execute Phases A-E end-to-end.

## Surprises & Discoveries

- The JSONL writer is opt-in already, but the capture policy is not: the large
  hourly diagnostic carriers are assembled in the orchestrator before the
  runner checks `OPENWEPP_R7H_SNOW_TRACE_PATH`.
- `DirectSnowStage3Diagnostics` currently mixes production-required handoff
  state and optional research observations, so simply wrapping the whole value
  in `Option` would risk disabling real production behavior.

## Decision Log

- Decision: persist two compact linked ledgers, not the campaign's entire eager
  hourly payload.
  Rationale: the Snowbird audit demonstrated that both mass boundaries are
  scientifically necessary, while file emission and detailed hourly context
  are research observability concerns.
  Date/Author: 2026-08-03 / Codex.
- Decision: treat the package as critical behavior-neutral architecture, not a
  physics correction or ordinary mechanical split.
  Rationale: capture request plumbing crosses runtime projection and a real
  serialization consumer, and the current diagnostic type contains values used
  outside reporting.
  Date/Author: 2026-08-03 / Codex.
- Decision: freeze quantitative performance non-regression before execution.
  Rationale: the package should not exchange eager stack copies for hidden heap
  churn or claim an optimization without comparable baseline evidence.
  Date/Author: 2026-08-03 / Codex.

## Outcomes & Retrospective

Queued. No contract, production, test, fixture, output, or assurance mutation
has occurred in this scaffold.
