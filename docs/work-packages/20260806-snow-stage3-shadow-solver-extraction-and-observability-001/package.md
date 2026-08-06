# Stage 3 Shadow Solver Extraction And Observability

Status: `queued / authorized / contract-first`

Date: `2026-08-06`

Package ID:
`20260806-snow-stage3-shadow-solver-extraction-and-observability-001`

Plan class: `Critical mixed mechanical refactor and behavior-neutral evaluation implementation`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
`Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` remain current throughout execution.

## Purpose

Make the Stage 3 evaluation shadow maintainable and scientifically auditable
without changing production snow behavior. The package first extracts the
Stage 3 solver from the 3,177-line runoff monolith. It then realizes the v127
evaluation operators behind a typed, default-off request and publishes the
component, support, fingerprint, mass, cold-content, and residual operands
needed for the four-site carrier audit.

The observable result is an opt-in internal schema-v5 evaluation row. Ordinary
and disabled runs retain exact schema-v4 bytes and one authoritative CoE-owned
production snow state.

## Authority And Rationale

The user authorized scaffolding and end-to-end execution after
`SNOW-STAGE3-EVALUATION-SHADOW-AUTHORITY` admitted exactly two evaluation
operators. `SC-SNOWFREEZE-001#INV-SNOWFREEZE-094` still says v127 authorizes no
runtime implementation, so production edits require a narrow v128 realization
invariant and contract-derived tests first.

This package implements evaluation machinery, not target melt ownership.
`INV-SNOWFREEZE-093` keeps CoE authoritative and Stage 3 on cutover hold.

## Implementation Intent

- Intent: mechanical solver extraction plus behavior-neutral evaluation-only
  implementation and internal observability.
- Production science: unchanged. No equation, coefficient, threshold, state
  mutation, forcing, selector default, or production owner changes.
- Evaluation science: implement the already admitted same-state paired-carrier
  and bounded sequential-response operators.
- Calibration and independent validation: `NOT_APPLICABLE`.
- Assurance: adopt v128 into the existing snow/frost report while preserving
  DRAFT and zero review, approval, release, or publication authority.
- Risk: `Critical` because canonical kernel authority, typed error taxonomy,
  orchestration structs, and an internal diagnostic schema are changed.

## Frozen Evaluation Surface

### Typed Request And Tag

The internal request is absent by default. When present it selects exactly one
of `same_state_paired_carrier_v1` or
`sequential_resolved_shadow_v1`. Constructors supply a mandatory typed tag with
the source snapshot, support, cadence, carrier/pair identity, unresolved
boundaries, and claim class. Conflicting legacy/new environment requests fail
without changing authoritative execution.

The existing `OPENWEPP_SNOW_STAGE3_COMPLETE_CARRIER_SHADOW` input remains an
exact compatibility spelling for the bounded sequential operator. A new
`OPENWEPP_SNOW_STAGE3_EVALUATION_OPERATOR` input admits explicit `disabled`,
`same_state_paired_carrier_v1`, or `sequential_resolved_shadow_v1`. Default is
disabled.

### Same-State Pair

The pair uses the immutable initial post-CoE daily snow snapshot. Both arms
receive identical forcing, `z_T/z_q/z_u/z_0`, cadence, support, and every
non-formulation operand. Arm A is `stage3_surface_energy_v1`; arm B is
`stage3_complete_carrier_v1`; pairing ID is `stage3_carrier_pair_v1`. Stable
FNV-1a fingerprints cover source state, forcing/geometry, and their combined
non-formulation support. The operator does not evolve its clone and claims only
carrier/component comparison.

### Sequential Resolved Shadow

The bounded sequential operator preserves the existing daily clone algorithm.
It evolves only inside the current 24-hour call and terminates there. Its claim
class is `bounded_response_experiment`, not physical seasonal chronology,
because daily initialization omits persistence and complete same-substep phase
and liquid chronology. It publishes evaluated versus requested seconds so
terminal or subresolution truncation is explicit.

### Schema And Operands

Disabled/default rows remain exact schema v4. An enabled evaluation row is
schema v5 and adds operator/tag IDs, arm IDs, fingerprints, component
applicability, hourly and daily shortwave, longwave, sensible, latent,
precipitation-advection, internal active/lower conduction, vapor exchange,
cold-content export, available ice, carrier total, melt, unallocated terminal
energy, closure residual, requested/evaluated seconds, and coverage fraction.

Internal conduction is labeled as active/lower exchange, not an external
snow-ground flux. Absent ground-to-snow-base energy remains an unresolved
boundary and may not be inferred from the residual.

## Included Scope

- Advance `SC-SNOWFREEZE-001` to v128 with evaluation-realization authority,
  guard/error obligations, schema-v5 lineage, and exact holds.
- Add contract-derived guards and mechanically advance exact version pins.
- Extract Stage 3 solver code into bounded modules without changing arithmetic
  order or existing production results.
- Add typed request/tag/operator/claim types and typed turbulent-transfer error
  preservation.
- Implement both evaluation operators and schema-v5 internal JSONL projection.
- Prove disabled-path v4 byte identity, evaluator-only custody, production
  noninterference, per-term reconstruction, coverage, fingerprint equality,
  typed failures, and real consumer reads.
- Adopt the v128 source into the governed DRAFT assurance report.
- Update package/campaign roadmaps and the catalog on closure.

## Excluded Scope

- Cross-day persistence, restart, reappearance, accumulation-season chronology,
  terminal event location, post-meltout or receiving-surface energy.
- Ground-to-snow-base flux implementation, land-surface energy, soil enthalpy,
  parameter fitting, literature-range adjudication, or four-site result runs.
- CoE retirement, Stage 3 production melt, dual ownership, defaults, public
  WAT/HBP/PASS schemas, fixtures, observations, or promotion.
- Changing any energy, mass, density, phase, liquid-routing, or frost equation.

## Intended Write Set

- this package tree;
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` and
  its lifecycle index;
- mechanically affected exact v127-to-v128 contract-version tests;
- one new focused authority/implementation contract test and `Cargo.toml`
  registration;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`;
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`;
- new files below
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/`;
- the direct runner authority/options builder and internal trace formatter under
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`;
- focused Stage 3 runtime and direct-consumer tests;
- the root roadmap, snow campaign roadmap, and package catalog; and
- typed assurance source-adoption paths and receipts selected by check mode.

No fixture, observation, public schema, production default, or unrelated crate
is authorized. Any additional path requires a prospective package amendment.

## Protected Boundaries

- CoE remains the sole current melt owner and Stage 3 cannot mutate production
  solid mass through evaluation.
- Evaluation errors fail only the explicitly enabled evaluation request; the
  disabled path remains byte-identical.
- Schema v4 remains exact for disabled/default rows. Schema v5 is internal and
  exists only when a tagged evaluator is selected.
- The evaluator clone and its ledgers cannot reach production state, runoff,
  WAT, HBP, PASS, defaults, calibration, or public output.
- The package cannot claim seasonal chronology, physical carrier plausibility,
  efficacy, promotion, retirement, or cutover.

## Mechanical Refactor Seam

Move the Stage 3 routing/thermal solver from `runoff_reconciliation.rs` into a
core solver module and a nested evaluation/carrier module. Preserve item bodies,
signatures, visibility to existing callers, comments, constants, arithmetic
order, and error precedence except for the separately declared typed evaluation
error improvement. The original file must finish below 3,000 lines; every new
or touched 2,000-line file receives explicit WARN disposition, and no 3,000+
nonexempt file may remain.

## Conservation And Output Acceptance

Before production edits, `artifacts/operand-lineage.md` freezes every new field,
units, time/area basis, source, applicability, and rejected aliases. Acceptance
requires independent reconstruction from the real schema-v5 consumer, paired
fixtures where wrong aliases differ numerically, explicit component-total and
coverage identities, production-state identity against disabled execution, and
schema-v4 byte identity. Producer self-consistency alone cannot close this
package.

## Phase Plan

### Phase 1 — Scaffold And Contract Freeze

Commit this autonomous plan, typed surface freeze, operand lineage, module seam,
reading map, write set, and pre-implementation test plan before authority or
Rust changes.

### Phase 2 — v128 Authority And Tests

Amend the canonical contract, add the focused contract guard, mechanically
advance exact pins, run strict contract/profile checks, and record the
pre-implementation gate. No production Rust changes precede this phase.

### Phase 3 — Mechanical Extraction

Move the Stage 3 core and evaluation carrier blocks into the declared modules.
Run format, check, focused pre-existing Stage 3 tests, export parity, and
line-count checks. Reconcile the extraction as behavior-preserving before
semantic evaluation additions.

### Phase 4 — Typed Evaluation And Observability

Implement request/tag types, paired and bounded sequential evaluators, typed
turbulent error custody, per-term diagnostics, schema-v5 projection, and real
consumer tests. Preserve exact disabled/v4 and production-state identity.

### Phase 5 — Assurance, Review, And Closure

Adopt v128 through the typed assurance workflow while retaining DRAFT. Complete
dual independent reviews, independent QA, finding disposition, direct critical
validation, exact-diff reconciliation, prompt archival, dual verification, and
a stable clean commit.

## Validation And Exit Criteria

- Contract v128 and its focused guards pass before production edits.
- `runoff_reconciliation.rs` is below 3,000 lines and no new nonexempt file is
  3,000+ lines.
- Mechanical extraction preserves existing focused Stage 3 results exactly.
- Disabled/default direct rows remain byte-identical schema v4 and allocate no
  evaluation clone/payload.
- Both typed operators have correct tags, fingerprints, support, coverage, and
  claim class; conflicting/invalid requests fail closed.
- Schema-v5 per-term totals independently reconstruct both arms and the
  sequential energy ledger; rejected alias formulas fail the fixture.
- Evaluation-enabled and disabled runs have bit-identical production state,
  compact ledgers, WAT/HBP/PASS outputs, and defaults.
- The real internal JSONL consumer reads every new field; producer-only proof is
  insufficient.
- Typed turbulent failures retain source category and do not mutate production.
- Formatting, warnings-denied Clippy, doctests, focused/contract/consumer tests,
  quick, frost, full workspace, assurance validation, Markdown, line-count,
  security, review, and dual verification pass at their declared boundaries.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation to two
read-only Rust/science reviewers, one read-only QA/consumer reviewer, two
read-only terminal verifiers, and one read-only heavy-gate runner. Expected
outputs are compact Markdown findings, dispositions, verification results, and
exact command/count evidence. Delegates may inspect and run commands but may
not edit tracked files, change authority, tune results, or activate assurance
review/approval/publication.

## Security And Data Impact

No network, credential, secret, external provider, protected fixture, or public
publication access is required. New environment parsing is bounded to an
internal evaluation selector with explicit conflict and UTF-8 errors. The
package adds no dependency and does not write outside existing opt-in trace
custody.

## Progress

- [x] (2026-08-06) User authorized scaffolding and execution.
- [x] (2026-08-06) Mapped the v127 authority boundary, 3,177-line seam,
  existing consumer, and missing operand surface.
- [x] Commit the scaffold and frozen pre-implementation artifacts.
- [x] Amend v128 and pass the pre-implementation contract gate.
- [ ] Complete mechanical extraction and parity checks.
- [ ] Implement typed evaluation and schema-v5 observability.
- [ ] Adopt assurance source, review, validate, verify, archive, and close.

## Surprises & Discoveries

- Observation: v127 intentionally admits evaluation semantics but closes with
  “no runtime implementation,” so this cannot truthfully be only a mechanical
  refactor. Evidence: `INV-SNOWFREEZE-094`.
- Observation: the existing carrier already computes complete sensible, latent,
  and precipitation-advection terms but publishes only a partial shadow subset;
  shortwave/longwave and internal conduction are aliased to production fields.
  Evidence: `stage3_hourly_surface_energy` and schema-v4 formatter.
- Observation: `runoff_reconciliation.rs` is 3,177 lines; moving the complete
  Stage 3 seam can reduce it below 3,000 without touching unrelated frost,
  density, or runoff logic.

## Decision Log

- Decision: use schema v5 only for explicitly enabled evaluation rows while
  retaining exact schema v4 for disabled/default runs. Rationale: additive
  evaluation evidence must not silently mutate the established ordinary trace
  contract. Date/Author: 2026-08-06 / Codex.
- Decision: represent fingerprints as deterministic FNV-1a hashes over exact
  input bits and stable enum IDs, not platform-default hashes. Rationale:
  evidence requires reproducibility without a new dependency. Date/Author:
  2026-08-06 / Codex.
- Decision: label the current sequential claim `bounded_response_experiment`.
  Rationale: this package does not add cross-day persistence or complete
  same-substep phase/liquid chronology. Date/Author: 2026-08-06 / Codex.

## Outcomes & Retrospective

Pending execution.

## Revision Note

2026-08-06: Initial contract-first scaffold created from the v127 worker
handoff and active roadmap.
