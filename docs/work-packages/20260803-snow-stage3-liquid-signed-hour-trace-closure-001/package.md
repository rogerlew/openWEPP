# SNOW-STAGE3-LIQUID-SIGNED-HOUR-TRACE-CLOSURE

Status: `complete / reviewed / verified / behavior-neutral evidence closure`

Date: `2026-08-03`

Plan class: `Behavior-neutral diagnostic publication and evidence-hold closure`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
The `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` sections must remain current during execution.

## Purpose / Big Picture

Close the evidence gap exposed by
`20260803-snow-prepeak-liquid-evacuation-physics-audit-001` without changing
snow physics. After this package, the real direct-production snow JSONL trace
must publish every Stage-3 liquid-closure operand and the existing hourly
forcing, state, routing, and cold-content diagnostics needed to audit mixed
signed CoE melt. An independent consumer must reconstruct Stage-3 liquid
closure and reject plausible alias formulas on a real Snowbird trajectory.

## Context And Orientation

`DirectSnowStage3Diagnostics` already owns incoming, routed, retained-delta,
refrozen, and closure-residual liquid in metres. The direct JSONL consumer in
`00c_day_input_builder_impl.rs` publishes only refrozen liquid, which forced the
predecessor audit to close `HOLD-EVIDENCE`. The runtime also owns hourly CoE
forcing and pack/liquid state, and Stage 3 owns hourly active/lower cold-content
diagnostics, but the current v3 JSONL surface omits enough of those values to
adjudicate mixed signed hours independently.

This package changes diagnostic publication only. CoE equations, signed-hour
redistribution, Stage-3 routing, density, phase, state mutation, selectors,
defaults, observations, calibration, and promotion remain unchanged.

## Implementation Intent

- Intent: `behavior-neutral implementation and diagnostic evidence closure`.
- Calibration evidence: `NOT_APPLICABLE`.
- Independent validation: `NOT_APPLICABLE`.
- The Snowbird trajectory is a real-consumer diagnostic fixture, not
  calibration or validation authority.
- The direct-production snow trace schema advances additively from v3 to v4.

## Included Scope

1. Amend `SC-SNOWFREEZE-001` to require additive v4 publication of exact
   Stage-3 liquid operands and exact existing signed-hour observability.
2. Add contract-derived anti-alias and real-consumer tests before production
   edits.
3. Publish Stage-3 incoming, routed, retained-delta, refrozen, and residual
   liquid with units and identity semantics.
4. Publish the existing CoE forcing and state lineage: hourly air temperature,
   radiation, cloud, raw/applied/routed melt, capacity, retained liquid before
   and after, released liquid, rain release, sublimation, and pack depth/density
   before and after, plus daily wind, dewpoint, and canopy scalars.
5. Publish existing hourly Stage-3 active/lower cold-content state through the
   real JSONL consumer.
6. Run an exact release CLI on Snowbird with Stage 3 enabled and independently
   reconstruct liquid closure from produced JSONL.
7. Prove behavior neutrality against a same-binary trace projection that
   ignores the new additive fields and against protected WAT output identity.

## Excluded Scope

- Changes to any snow, energy, phase, density, liquid-routing, or signed-hour
  equation, ordering, constant, guard, selector, default, or parameter.
- A signed-hour physics correction, negative-melt netting, cold-content
  coupling, wet-compaction correction, or legacy-routing promotion.
- Fixture or observation edits, fitting, calibration, validation, or provider
  admission.
- WAT, HBP, PASS, watershed, parser, runfile, or user-facing schema changes.
- Backfilling new fields into historical v3 traces.

## Intended Write Set

- `docs/work-packages/20260803-snow-stage3-liquid-signed-hour-trace-closure-001/`
- `docs/work-packages/README.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`
- `assurance/v2/identity.lock.json`
- `assurance/v2/reports/linear-groundwater-reservoir-recurrence/review.lock.json`
- `assurance/v2/reports/native-forest-canopy-phenology-evaluation/review.lock.json`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/report.yaml`
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json`
- one generated `assurance/v2/transactions/*.json` receipt from the typed
  `adopt-report-source` transaction for `SC-SNOWFREEZE-001`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00f_snow_accumulation_melt_trace.rs`
- `tests/integration/*.rs`, limited to the new closure test, contract-derived
  assertions, and mechanical `SC-SNOWFREEZE-001` version-pin reconciliation.
- `target/snow_stage3_liquid_signed_hour_trace_closure/`

Everything else is protected. No fixture, observation, production output, or
historical audit evidence may be modified.

## Authority And Dependencies

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/work-packages/20260803-snow-prepeak-liquid-evacuation-physics-audit-001/artifacts/disposition.md`
- `docs/work-packages/20260803-snow-prepeak-liquid-evacuation-physics-audit-001/artifacts/worker-handoff.md`
- Existing typed diagnostics and direct-production JSONL consumer named above.

Package artifacts remain evidence; canonical science authority remains in the
`SC-*` contracts.

## Contract-First Sequence

Production edits are prohibited until these steps complete in order:

1. amend canonical contract authority and version;
2. add contract-derived tests and rejected-formula fixtures;
3. record the pre-implementation contract gate and operand-lineage table; and
4. edit production diagnostic carriers and the real JSONL consumer.

No surrogate physics is authorized. This package only publishes exact values
already produced by the contract-governed runtime.

## Conservation / Output Acceptance

Before production edits, `artifacts/operand-lineage.md` must record every field,
units, time basis, source object, consumer name, and diagnostic-versus-
authoritative role. Anti-tautology tests must distinguish the accepted Stage-3
identity from at least these rejected formulas:

- omitting retained-liquid change;
- substituting top-level CoE routed melt for Stage-3 routed liquid;
- substituting the CoE retained store for Stage-3 retained change;
- double-counting refrozen liquid; and
- accepting the producer-reported residual without reconstructing it.

Acceptance requires an independent parser over real JSONL, a row population
with non-aliased operands, two-sided closure within the contract tolerance, and
protected WAT identity. Self-consistency and source-string checks are supporting
evidence only.

## Real Consumer Proof

The release `openwepp-cli-hill` binary must write schema-v4 JSONL. The
independent package tool must read that file, not runtime memory or a test-only
adapter. Static evidence must map typed producer fields through the direct
partition, runner handoff, formatter, file writer, and parser, and confirm no
wrapper, shadow, skeleton, or v3 compatibility formatter carries the closure
claim.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent read-only Rust/science reviewers, two independent read-only
terminal verifiers, and a `comparator_suite_runner` for heavy quick/frost/full
workspace gates. Expected outputs are compact findings, metrics, and artifact
or log paths. Reviewers and verifiers have read-only access; the comparator may
write only normal build/test outputs under `target/`. The orchestrator alone
edits package, contract, production, and test files.

## Phase Plan

### Phase A: Authority And Scaffold

Freeze the source/write set, complete required reading, amend the contract,
author contract-derived tests, and record operand lineage plus the
pre-implementation contract gate.

### Phase B: Behavior-Neutral Publication

Extend existing diagnostic carriers without changing arithmetic or branches,
publish additive schema-v4 fields through the real JSONL consumer, and add
formatter/parser tests with deliberately distinct operands.

### Phase C: Real-Consumer Reconstruction

Build the exact release CLI, execute the bounded Snowbird fixture with Stage 3
enabled into a non-overwriting target namespace, parse JSONL independently,
reconstruct every liquid row, and compare protected output identities.

### Phase D: Direct Validation

Run formatting, warnings-denied Clippy, focused contract/runner tests, direct
reconstruction, quick and frost profiles, full critical workspace regression,
doctests, Markdown/JSON checks, line-count governance, and exact-diff/write-set
reconciliation. Record exact commands and outcomes.

### Phase E: Review, Verification, And Disposition

Complete two independent reviews, disposition every finding, remediate accepted
findings, then obtain two fresh terminal verifications. Close only when every
current-scope criterion has direct evidence; otherwise record a truthful hold.

## Validation And Exit Criteria

- Contract version, invariant, obligation, tolerance, binding-exposure entry,
  and revision history are internally consistent.
- Contract-derived tests fail without the new authority/publication and pass
  after implementation.
- Schema v4 contains all named exact operands with unambiguous units and
  retained-delta semantics.
- A real Snowbird trace contains a non-aliased row population and independently
  closes `incoming - routed - retained_delta - refrozen = residual` within the
  contract tolerance.
- Every rejected formula differs materially on at least one retained row.
- Existing v3 scientific evidence is not rewritten; the new result is a fresh
  behavior-neutral publication run.
- Protected WAT output and a same-binary projection of all pre-v4 trace fields
  are byte/value identical.
- No production arithmetic, branch, selector, default, parameter, fixture, or
  observation changes appear in the terminal diff.
- Applicable focused, quick, frost, full, Clippy, format, doctest, documentation,
  syntax, source-identity, and real-consumer gates pass.
- Dual review, complete finding disposition, dual terminal verification, and
  line-count governance pass with no deferred current-scope gate.

This serialization/output change is classified `critical`; the full workspace
correctness profile is required and must be executed by the authorized
`comparator_suite_runner` when available.

## Security And Data Impact

Security impact is `none expected`. The package uses local repository fixtures
and writes disposable target evidence only. It must not read credentials,
contact external services, expose environment secrets, or serialize absolute
credential paths.

## Progress

- [x] (2026-08-03) User authorized commit/push of the predecessor audit and
  end-to-end execution of this follow-up.
- [x] (2026-08-03) Scaffolded package, prompt, authority map, artifacts,
  catalog, and roadmap; required pre-read is `509317` bytes (`WARN`).
- [x] (2026-08-03) Committed the scaffold as `48d89081` before production
  edits.
- [x] (2026-08-03) Completed v123 contract authority, contract-derived
  anti-alias tests, exact operand lineage, and a `7 passed / 2 expected failed`
  pre-implementation RED gate.
- [x] (2026-08-03) Implemented additive diagnostic publication and completed
  the independent real-Snowbird reconstruction and neutrality proof.
- [x] (2026-08-03) Amended the write set before assurance mutation to include
  the typed generated consequences of adopting the changed declared contract
  dependency; no assurance source prose, lifecycle decision, or approval is
  authorized.
- [x] (2026-08-03) Completed focused, format, Clippy, doctest, documentation,
  assurance, quick `2160/2160`, frost `345/345`, and Critical full `2209/2209`
  gates.
- [x] (2026-08-03) Completed two independent reviews with exact-diff assurance
  supplements, two fresh terminal verifications, prompt archival, handoff, and
  final disposition.

## Surprises & Discoveries

- The first quick profile correctly failed closed because the v123 contract
  bytes no longer matched the generated assurance identity. The snow assurance
  report is a `DRAFT`, declares this contract as an external `local_content`
  dependency, and has no active review events or approvals; the repository's
  typed `adopt-report-source` transaction is therefore the bounded closure
  route. Hand-editing generated locks remains prohibited.

## Decision Log

- Decision: combine Stage-3 liquid closure and signed-hour observability in one
  behavior-neutral publication package.
  Rationale: both consume the same typed snow diagnostics, JSONL formatter,
  contract authority, real-consumer fixture, and critical serialization gates;
  splitting them would repeat the same production/publication boundary.
  Date/Author: 2026-08-03 / Codex.
- Decision: advance the additive diagnostic schema to v4 rather than silently
  changing v3.
  Rationale: downstream consumers can distinguish the new closure-capable
  surface, while historical v3 evidence remains immutable.
  Date/Author: 2026-08-03 / Codex.
- Decision: include only the machine-generated assurance identity files and
  transaction receipt emitted by the typed contract-dependency adoption.
  Rationale: the contract is already a declared report dependency and the
  fail-closed transaction is required to reconcile that intentional source
  change; it makes no human lifecycle decision and invalidates no authority.
  Date/Author: 2026-08-03 / Codex.

## Outcomes & Retrospective

The real internal snow trace now has additive schema v4 with exact Stage-3
liquid closure and existing signed-hour forcing/state/thermal observability.
An independent parser closed `14245` Snowbird rows to `1.2272e-17 m`, found
`227` mixed rows with every Stage-3 operand nonzero, rejected four adjacent
aliases, and proved WAT, HBP/PASS, and every pre-v4 trace value unchanged.

The predecessor's missing-operand hold is closed. The resulting trajectory
shows Stage 3 routes most incoming liquid (`39.5692/40.3463 m`), with much
smaller retained change (`0.7230 m`) and refreeze (`0.0541 m`). That is a
mechanism observation, not authority for a physics correction. The next work
must prospectively separate forcing and empirical `B/C` error from signed-hour
thermal/export policy and separately establish wet-compaction operand
authority.

The v123 contract edit correctly triggered stale-assurance failure. The typed
source-adoption transaction reconciled generated identity without invalidating
active authority. All focused and broad gates, both reviews, and both terminal
verifications pass.
