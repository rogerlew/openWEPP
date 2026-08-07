# Stage 3 Evolving-State Carrier Plausibility Reconciliation

Status: `queued / scaffolded / result-blind admission required`

Date: `2026-08-07`

Package ID:
`20260807-snow-stage3-evolving-state-carrier-plausibility-reconciliation-001`

Plan class: `Critical characterization and science-authority reconciliation`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
Keep Progress, Surprises & Discoveries, Decision Log, and Outcomes &
Retrospective current throughout execution.

## Purpose / Big Picture

Determine whether the currently admitted evolving Stage 3 evaluation carrier
is physically plausible before any snow state is persisted across days. The
predecessor bridge proved an operator-mechanics fact: at all four sites the
immutable same-state and frozen-active carriers are negative while the
within-day sequential carrier is positive. It did not prove that the
sequential state feedback, turbulent exchange, or vapor loss is physically
correct.

This package must identify whether that crossing is explained by the actual
wind-forcing exposure, missing canopy aerodynamics, raw-versus-bounded vapor
mass semantics, the Monin-Obukhov stability geometry, active-layer state
evolution, or a non-identifiable combination. It must then issue a
prospectively governed four-site plausibility disposition. It does not persist
the shadow, correct production physics, promote Stage 3, retire CoE, or cut
over any owner.

## Progress

- [x] (2026-08-07) Scaffolded the package and made it the canonical next snow
  campaign increment.
- [x] (2026-08-07 14:12Z) Completed result-blind authority, exact trace/cohort,
  estimator, mass/energy reconstruction, predicate, and zero-arm
  counterfactual freeze; independent admission reviews dispositioned.
- [x] (2026-08-07 15:06Z) Amended Snow Energy to v9 and Snow/Freeze to v131,
  added independent contract-derived arithmetic/adversarial vectors, and passed
  the clean pre-implementation gate at `5e353b8c8`.
- [x] (2026-08-07 16:03Z) Implemented and independently reviewed the
  package-local schema-v6 consumer; `28/28` analyzer tests pass and existing
  observability is sufficient, so no Rust/schema expansion occurred.
- [ ] Execute the frozen four-site audit and independent reconstruction once.
- [ ] Complete dual review, finding disposition, dual verification, and final
  package disposition.

## Surprises & Discoveries

- Observation: Canonical `SC-SNOWENERGY-001` uses signed `m_v` as bounded
  transfer, while schema-v6 exposes raw vapor opportunity and separate bounded
  deposition/sublimation fields.
  Evidence: independent science review against contract v8 and current Stage 3
  evaluation chronology; Phase B must reconcile the names and latent/mass
  semantics before analysis.
- Observation: Existing schema-v6 is sufficient for independent turbulent,
  bounded-vapor, operator-order cold/melt, endpoint, and support
  reconstruction; no Rust observability expansion is justified.
  Evidence: independent Rust/consumer reviews and exact tuple inventory.
- Observation: Locally retained libsnobal and current Rust both use `z/L`, but
  parity does not independently select `z/L` over `(z-d)/L` as science
  authority.
  Evidence: `hle1.c` and `surface_energy.rs`; physical geometry disposition is
  `NOT_EVALUABLE`.

- Observation: The retained operator evidence reports exact shortwave
  invariance, so shortwave cannot explain the `S/F < 0` to `Q > 0` crossing.
  Evidence: predecessor package and schema-v6 result named under Frozen Intake.
- Observation: The current Stage 3 turbulent call consumes the supplied wind
  directly and has no canopy operand, while sub-canopy longwave consumes canopy
  cover and sky view.
  Evidence: `openwepp-meteorology/src/surface_energy.rs` and the Stage 3
  evaluation caller named under Dependencies.
- Observation: The four fixtures use GRIDMET wind embedded in observed-mode
  climate inputs; calling it physically open-site or sub-canopy wind requires
  provenance that has not yet been established.
  Evidence: `tests/fixtures/snotel_observed/README.md`.
- Observation: Immutable-operator vapor totals are exchange opportunities, not
  conserved snow loss, because immutable state is not debited. Sequential
  physical loss must be read from the bounded sublimation ledger.
  Evidence: schema-v6 operand lineage and the Stage 3 sequential evaluator.

## Decision Log

- Decision: Admit no equation counterfactual and keep all physical magnitude
  gates that lack comparable authority `NOT_EVALUABLE`.
  Rationale: result-blind governance forbids inventing a canopy operator,
  geometry correction, or numeric envelope; this blocks plausibility passage
  while allowing truthful mechanism characterization.
  Date/Author: 2026-08-07 / Codex.
- Decision: Independently reconstruct bounded vapor from raw opportunity and
  pre-transfer active ice, and reconstruct melt in operator order before using
  endpoints.
  Rationale: producer transfer columns and endpoint identities alone are
  tautological and can hide compensating aliases.
  Date/Author: 2026-08-07 / Codex.
- Decision: Treat any nonfinite numeric tuple evidence as invalid package
  evidence, while retaining finite reconstructable domain violations as the
  nonexclusive active-state physical class.
  Rationale: nonfinite operands cannot support independent mass/energy
  reconstruction; classifying them as physical would suppress invalid-evidence
  precedence or corrupt common support.
  Date/Author: 2026-08-07 / Codex.

- Decision: Treat canopy aerodynamic sheltering as a named high-priority
  hypothesis, not as an authorized correction.
  Rationale: Static asymmetry is established, but forcing exposure, canopy
  input authority, and the coupled energy response are unresolved. A scalar
  wind multiplier would be an unauthorized fitted surrogate.
  Date/Author: 2026-08-07 / Codex.
- Decision: Keep the current shortwave formulation inside the audit as a
  protected invariant across operator/counterfactual arms, while making no
  claim that its absolute magnitude is validated.
  Rationale: Exact invariance excludes it from the operator crossing but not
  from broader physical-validation questions.
  Date/Author: 2026-08-07 / Codex.
- Decision: Separate raw signed vapor exchange from actual bounded deposition
  and sublimation in every result and claim.
  Rationale: Treating immutable flux opportunity as snow mass loss would
  manufacture a conservation claim the operator does not make.
  Date/Author: 2026-08-07 / Codex.

## Outcomes & Retrospective

Queued. No result-bearing execution, physics change, persistence, promotion,
or cutover claim has occurred.

## Frozen Intake

- Current source and scaffold base: record exact clean Git identity in
  `artifacts/protocol-freeze.json` before result-bearing work.
- Governing contract versions at scaffold: `SC-SNOWFREEZE-001` v130 and
  `SC-SNOWENERGY-001` v8.
- Retained operator result:
  `target/snow_stage3_operator_reconciliation_v3/results/operator-reconciliation-results.json`.
- Retained compact result:
  `docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/artifacts/operator-reconciliation-results.json`.
- Predecessor bridge:
  `docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/`.
- Four-site medians on identical common support, in `MJ m^-2`:
  - Mica Creek: `S=-706.2097`, `F=-644.2316`, `Q=+129.7417`;
  - Niwot: `S=-1709.7291`, `F=-1353.8158`, `Q=+106.2273`;
  - Paradise: `S=-887.0556`, `F=-503.4318`, `Q=+379.6567`; and
  - Snowbird: `S=-1086.4748`, `F=-742.0217`, `Q=+179.5949`.
- Snowbird same-state term medians, reduced separately and therefore not an
  additive median identity: shortwave `+231.4479`, longwave `-25.7704`,
  sensible `-407.1946`, latent `-852.3982`, and advected `-5.4811 MJ m^-2`.
- Retained signed vapor-exchange medians span approximately `-205` to
  `-435 kg m^-2` for same-state and `-91` to `-209 kg m^-2` for sequential.
  These values are raw exchange equivalents until reconciled to the operator's
  actual bounded transfer semantics.
- Shortwave invariance, primitive reconstruction, delta closure, lineage
  identity, and protected HBP/PASS/WAT byte identity pass in the predecessor.
- Snowbird remains `DEVELOPMENT_ONLY_NON_DECISIVE_DIAGNOSTIC`. Mica Creek,
  Niwot, and Paradise are the canonical screen sites.
- CoE remains the sole authoritative production melt and snow-mass owner.

## Implementation Intent

Intent is `characterization plus science-authority reconciliation`. It is not
science implementation, empirical calibration, independent validation, or
production correction. Existing measured SWE selects diagnostic windows only
and remains `DIAGNOSTIC_ONLY`.

The package may add package-local independent analysis and, only when existing
schema-v6 evidence cannot express a required primitive, additive default-off
internal observability. Any contract or Rust change follows the mandatory
sequence: canonical contract, contract-derived tests, recorded
pre-implementation contract gate, then implementation. No result-bearing
counterfactual may run until its equation, inputs, applicability domain,
cohort, estimators, and quantitative predicates are frozen without reading
that counterfactual's results.

## Included Scope

- Prove the wind lineage for every site: provider, temporal cadence, spatial
  support, units, transformation history, reference-height/exposure metadata,
  and the exact value consumed by Stage 3.
- Distinguish the `5 m` virtual-instrument projection from a claim that the
  forcing is physically measured at `5 m` above the sub-canopy snow surface.
- Inventory every current canopy/wind/aerodynamic path, including Stage 3,
  sub-canopy longwave, direct-runtime canopy state, and the separate frost
  `tmpadj` path. Do not reuse frost equations as snow-energy authority.
- Reconstruct current Monin-Obukhov sensible heat, latent heat, vapor mass,
  friction velocity, exchange velocities, density, humidity gradients,
  stability corrections, and termination status independently from schema-v6
  primitives.
- Reconcile neutral logarithms using `ln((z-d)/z_0)` with the current stability
  arguments using `z/L`; determine from canonical equation authority whether
  the latter must use `(z-d)/L`. Quantify an isolated consequence only if
  canonical equation authority prospectively selects the alternative;
  otherwise record implementation/reference parity and `NOT_EVALUABLE` without
  silently correcting production behavior.
- Separate raw signed vapor exchange, bounded sublimation, bounded deposition,
  actual mass debit/credit, and mass censored at unresolved support. Label the
  same-state and frozen-active transfer fields N/A where state mutation is
  prohibited.
- Attribute the `F -> Q` evolution delta temporally to surface temperature,
  cold content, active volume, vapor-pressure gradient, stability, exchange
  velocity, longwave, sensible, latent, precipitation advection, and support.
- Establish whether direct-runtime canopy cover, canopy height, LAI or another
  structural state has science and consumer authority for a snow-surface
  aerodynamic resistance or within-canopy wind operator.
- If and only if canonical authority supplies a complete equation and every
  required input, execute a diagnostic-only, no-fit canopy-aerodynamic or
  geometry counterfactual from the same source states and forcing. Preserve
  current shortwave exactly and report the coupled response of all terms.
- Run the frozen audit across Mica Creek, Niwot, Paradise, and non-decisive
  Snowbird with water-year distributions, site medians, support, censoring,
  and independently reconstructed energy and mass ledgers.
- Update canonical contracts, roadmap, catalog, DRAFT assurance source impact,
  reviews, verification, and worker handoff as applicable.

## Excluded Scope

- A fixed `0.1`, `0.2`, `0.3`, or other scalar canopy-wind multiplier; fitted
  attenuation; site-specific coefficients; or any result-selected equation.
- Treating GRIDMET, CLIGEN-station, reanalysis/grid, open-site, above-canopy,
  within-canopy, or sub-canopy wind as aliases without proven metadata.
- Changing shortwave, albedo, longwave, turbulent, canopy, phase, density,
  melt, liquid-routing, frost, ET, soil, or land-surface production physics.
- Cross-day persistence, restart/reappearance, terminal meltout, receiving-
  surface energy, snow-free land-surface energy, soil heat receipt, promotion,
  defaults, parser/runfile/user selectors, CoE retirement, or cutover.
- Using immutable raw vapor opportunity as actual snow loss, using sequential
  raw opportunity in place of bounded transfer, or routing sublimation as
  melt/liquid.
- Retroactively changing the prior `[-5,+5] W m^-2` screen or using the known
  S/F/Q result to choose new thresholds.
- Public schema, WAT, HBP, PASS, fixture, observation, or frozen-result edits.
- Assurance approval, release transfer, publication, or application-fitness
  claims.

## Dependencies And Authority

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/`
- `docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/`
- `docs/work-packages/20260806-snow-stage3-four-site-carrier-term-audit-001/`
- `tests/fixtures/snotel_observed/README.md` and the four site manifests
- `crates/openwepp-meteorology/src/surface_energy.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`
- `docs/decisions/0042-science-data-and-calibration-authority-separation.md`

Use locally retained primary equation sources and permissively licensed
reference implementations. If a complete canopy-aerodynamic equation or its
required inputs lack authority, record `AUTHORITY_MISSING`; do not invent a
surrogate. The pinned legacy WEPP baseline is provenance for legacy behavior,
not automatic authority for a modern sub-canopy snow exchange operator.

## Intended Write Set

- `docs/work-packages/20260807-snow-stage3-evolving-state-carrier-plausibility-reconciliation-001/**`
- `docs/ROADMAP.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/work-packages/README.md`
- mandatory or conditional contract-first authority files:
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
  - `docs/specifications/science-contracts/index.md`
  - focused integration contract tests under `tests/integration/`
- conditional behavior-neutral observability only after a proven missing-
  primitive gate:
  - `crates/openwepp-meteorology/src/surface_energy.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00h_snow_stage3_evaluation_trace.rs`
  - `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00i_snow_stage3_reconciliation_trace.rs`
  - focused module and integration tests for those exact surfaces;
- conditional DRAFT assurance-source adoption files selected by the canonical
  assurance plan, with no review, approval, release, or public output; and
- ignored execution outputs under
  `target/snow_stage3_evolving_state_carrier_plausibility_reconciliation/`.

Existing retained target results are read-only. No fixture, observation, prior
package artifact, frozen result, or public output is writable. Any write-set
expansion requires a prospective package amendment and independent review
before the new write.

## Prospective Protocol And Decision Discipline

Before any new result-bearing calculation, `artifacts/protocol-freeze.json`
must bind exact source/binary/trace hashes, site and water-year inventories,
observation roles, support/censoring, forcing custody, operator IDs, units,
aggregation order, tolerances, quantitative physical envelopes, decision
precedence, and the permitted counterfactual budget.

The freeze must distinguish:

1. Known retained evidence, which may be used as intake but not hidden as
   prospective evidence.
2. Independent reconstruction of known evidence, which proves consumer
   correctness but is not a new physical result.
3. New attribution summaries from existing primitives, whose estimators and
   predicates must be frozen before calculation.
4. New equation counterfactuals, which require canonical equation/input
   authority and may be executed once after result-blind review.

The quantitative physical gate must be sourced from site/season/forest-
comparable authority and frozen before results. It must separately assess net
external energy, sensible and latent exchange, raw vapor opportunity, actual
bounded sublimation/deposition, surface temperature, stability/exchange
behavior, support, and absence of compensating term errors. If a defensible
numeric envelope is unavailable, the relevant physical gate is
`NOT_EVALUABLE`, which blocks persistence; it is not replaced with an assumed
range.

The three canonical sites decide the screen. Snowbird is reported in full but
cannot turn a canonical failure into a pass or carry a pass alone. Medians of
separately reduced distributions are never added as if they were a joint
median. All sign and magnitude decisions use per-water-year reconstructed
totals before site reduction.

Allowed terminal classes are nonexclusive and evidence-limited:

- `EVOLVING_CARRIER_PLAUSIBILITY_PASS`;
- `WIND_FORCING_EXPOSURE_UNRESOLVED`;
- `CANOPY_AERODYNAMIC_AUTHORITY_REQUIRED`;
- `VAPOR_OPPORTUNITY_TRANSFER_MISMATCH`;
- `TURBULENT_STABILITY_GEOMETRY_DEFECT`;
- `ACTIVE_STATE_EVOLUTION_PLAUSIBILITY_FAIL`;
- `COMPENSATING_TERM_ERROR`;
- `MULTIFACTOR_OR_INCONCLUSIVE`.

Persistence is unblocked only by the first class with every required canonical
site, reconstruction, provenance, mass/energy, support, and physical-envelope
gate passing. Any other class keeps persistence held and must name the first
actionable correction or authority package. A failed physical screen is a
valid completed scientific result for this characterization package; it does
not make the package's execution gates fail when the protocol and evidence
are complete.

## Protected Invariants

- CoE remains the sole authoritative production melt and snow-mass owner.
- Evaluation remains typed, default-off, current-call bounded, and forbidden
  to production consumers.
- Disabled schema, production state, WAT, HBP, PASS, defaults, and public
  outputs remain exact.
- Existing shortwave is bit-identical across audit arms. This is a protected
  comparison invariant, not an absolute-validation claim.
- No raw exchange opportunity is relabeled as actual transfer.
- No canopy, geometry, turbulent, or state correction enters production.
- No site-specific tuning, observation leakage, or post-result threshold
  change is permitted.
- Assurance remains governed DRAFT with zero publication claim.

## Phase Plan

### Phase A -- Result-Blind Admission

Freeze intake hashes, write set, data roles, wind-forcing custody questions,
operator and transfer semantics, candidate authority requirements,
counterfactual budget, quantitative predicates, decision precedence, and
failure policy. Record the required-reading map, security/data impact,
calibration-readiness posture, operand lineage, direct validation selection,
and scaffold commit. Obtain independent science/authority and Rust/consumer
review and disposition every finding before result execution.

### Phase B -- Contract And Test Authority

Amend `SC-SNOWFREEZE-001` to bind the audit, claim limits, raw-versus-bounded
vapor semantics, and persistence gate. Amend `SC-SNOWENERGY-001` only if the
authority review admits a specific diagnostic equation or corrects a canonical
geometry statement. Add contract-derived tests and record a passing
pre-implementation contract gate before any implementation edit. If complete
equation or input authority is absent, freeze that absence and prohibit the
corresponding counterfactual.

### Phase C -- Independent Consumer And Minimal Observability

Build package-local analysis that reads retained schema-v6 tuples without
importing producer calculation helpers. First prove existing schema suffices.
Only a demonstrated missing primitive may trigger the conditional default-off
observability write set. Independently reconstruct every turbulent, radiation,
advected, mass-transfer, support, and endpoint operand; reject malformed N/A,
duration, identity, or alias states.

### Phase D -- Frozen Four-Site Execution

Build the exact release CLI only if a new trace is required; otherwise consume
the immutable retained traces. Execute the frozen current-operator audit and
at most the prospectively admitted counterfactual budget. Preserve every
result, failure, binary/source identity, stdout/stderr, and manifest. Do not
retry a scientific failure into a pass.

### Phase E -- Reconstruction And Interpretation

Publish per-site and per-water-year term, state, stability, raw-vapor,
bounded-transfer, mass, energy, support, and censoring results. Reconstruct
independently, apply the frozen decision precedence, distinguish mechanism
from physical validation, and state whether persistence remains held.

### Phase F -- Closure

Run all applicable direct validation, assurance-impact analysis, exact-diff
reconciliation, line-count governance, dual review with explicit finding
disposition, and dual terminal verification. Update roadmaps, catalog,
disposition, and a defect- or authority-shaped worker handoff. Archive the
kickoff prompt byte-identically. Do not mark the package complete while any
current-scope execution gate or review finding remains unresolved.

## Validation And Acceptance

Before edits, declare exact implementation intent and validation selection in
the package artifacts. At minimum, execution must record:

- focused package-local analyzer tests, including malformed identity, unit,
  N/A, raw/bounded-mass alias, median-of-medians, and result-selected threshold
  rejection;
- contract schema/profile and focused contract-derived tests for every amended
  invariant or obligation;
- shared turbulent solver parity and primitive reconstruction if Rust or
  schema changes;
- exact disabled-path and protected WAT/HBP/PASS output identity if a new trace
  is emitted;
- independent energy and mass reconstruction on the exact four-site cohort;
- source-level anti-evasion guards if fixture/authority-suite binding is
  touched;
- warnings-denied Clippy for affected Rust packages and applicable reverse
  dependents when Rust changes;
- the appropriate focused, quick, frost, and critical/full correctness
  profiles selected from the exact terminal diff;
- scoped Markdown lint/validation, local-link checks, and `git diff --check`;
- DRAFT assurance impact with no lifecycle promotion; and
- exact terminal clean-source identity for every result-bearing execution.

Heavy four-site/release/comparator and critical/full runs are delegated to the
required comparator runner. Exact commands and expected inventories are frozen
in `artifacts/gate-results.md` before execution rather than delegated to a
planning tool.

## Exit Criteria

The package may close as executed only when:

1. The protocol and physical predicates were frozen before new results and
   independently reviewed.
2. Wind provider/exposure/reference-height custody has an explicit per-site
   disposition; unknowns remain named and block persistence.
3. Raw vapor opportunity and actual bounded transfer are independently
   reconstructed and never aliased.
4. Current geometry and any admitted counterfactual equations reconstruct from
   independent operands with explicit applicability and units.
5. All canonical-site and Snowbird inventories, support, censoring, failures,
   and physical outcomes are reported without sample laundering.
6. Every required execution gate has direct current evidence; physical
   `DIVERGES` is allowed, but missing or invalid execution evidence is not.
7. Protected production authority, defaults, outputs, fixtures, and DRAFT
   assurance posture are unchanged.
8. Dual reviews, finding disposition, dual verification, exact terminal diff,
   and line-count governance pass with no undispositioned finding.
9. The roadmap and worker handoff either admit persistent accumulation shadow
   after a full plausibility pass or keep it held behind one concrete next
   correction/authority boundary.

## Security And Data Impact

The package uses local source, retained traces, fixture metadata, and locally
retained references. It must not modify observations or fetch unpinned data.
No secret, credential, personal-data, external-message, deployment, or public-
release surface is in scope. Release binaries and copied inputs remain in the
ignored target namespace. Environment selectors are scrubbed and recorded for
result-bearing execution. The security/data gate is `PASS` only after exact
input/output manifests prove these boundaries.

## Line-Count Governance

Record touched `.rs` line counts before implementation and at terminal diff.
Files at or above 2000 lines are `WARN` and require decomposition rationale and
follow-up intent. Any nonexempt 3000+ line file blocks closure until refactored.
Generated/fixture exceptions require explicit owner and sunset metadata.

## Review, Verification, And Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent read-only science/Rust reviewers, one independent read-only
consumer/reconstruction reviewer, the `comparator_suite_runner` for heavy
four-site, release, comparator, and critical/full commands, and two independent
read-only terminal verifiers. Expected outputs are compact findings, reproduced
metrics, exact commands/counts, and artifact/log paths. Reviewer and verifier
write access is read-only; the comparator may write only ignored package target
outputs. Higher-precedence session authorization remains required when the
execution environment requires it.

Reviews must assess scientific authority, forcing and geometry custody,
raw-versus-bounded mass semantics, anti-tautology, gate legitimacy, claim
limits, protected boundaries, and line-count governance. Every finding is
dispositioned as `accepted`, `rejected`, `deferred`, or `follow-up`, with
rationale. Accepted findings are fixed and reverified. Closure is blocked by
an undispositioned finding or by a current-scope gate relabeled as future work.

## Idempotence And Recovery

All analysis writes to a package-specific ignored target namespace and refuses
to overwrite an admitted result without a new immutable attempt directory.
Protocol and result hashes make reruns detectable. Failed scientific runs are
retained; they are not deleted or retried into acceptance. Existing retained
artifacts and fixtures remain read-only. Git branches and worktrees are not
created by this package.
