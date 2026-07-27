# CAL04B Native GSI Canopy-Height Coherence Hold Lift

This Defect-Closure ExecPlan is a living document governed by
`docs/codex_exec_plans.md` and `docs/defect_closure_execplans.md`. Keep
`Progress`, `Surprises & Discoveries`, `Decision Log`, and
`Outcomes & Retrospective` current throughout execution.

Package ID:
`20260727-cal04b-native-gsi-canopy-height-coherence-hold-lift-001`

Status: `ACTIVE / TERMINAL HOLD — PRE-EXISTING ASSURANCE IDENTITY DRIFT`

Execution mode: `package-end-to-end`

## Purpose

Close defect `CAL04B-NATIVE-001`: the generalized-GSI production override can
publish positive current LAI while retaining missing or non-positive
post-growth canopy height on a valid zero-to-positive GSI transition. Restore
one contract-coherent same-day canopy state through every real consumer without
weakening the existing rev-21/rev-36 fail-closed guard, then lift the production
prerequisite for a fresh CAL-04B calibration execution package.

## Rationale

CAL-04B cannot calibrate a parameter domain that the real production consumer
cannot consume. The failure is localized, reproducible, contract-facing, and
correctable only on production state-publication surfaces excluded from
CAL-04B. A dedicated hold-lift package keeps calibration evidence sealed while
giving the defect enough contract, source, test, consumer, and critical-gate
authority to close end-to-end.

## Included Scope

- Adjudicate and, if required, amend authoritative generalized-GSI
  current-day canopy-height and state-ordering law.
- Add contract-derived zero/positive transition and negative guard tests.
- Correct the centralized production post-phenology state projection.
- Prove Lane D active routing and every named downstream canopy consumer.
- Replay the entire frozen CAL-04B native-proof case plan.
- Complete critical correctness gates, review, verification, and hold lift.

## Excluded Scope

- Calibration-domain, forcing, observation, objective, acceptance, or
  identifiability changes.
- Hubbard population, later-stage calibration-readiness, freeze, or Harvard
  execution.
- New empirical canopy-height fitting or any surrogate/heuristic physics.
- Guard weakening, candidate filtering, fixture forcing, compatibility
  fallback, or unrelated plant-growth migration.

## Progress

- [x] 2026-07-27: Retain CAL-04B attempt 004 and exact `GSI-5557` reproducer.
- [x] 2026-07-27: Localize the incoherent generalized-GSI override and scaffold
  this hold-lift package before production edits.
- [x] 2026-07-27: Authenticate the declared write set and write the
  pre-implementation intent plan at base `f4b3db6c`.
- [x] 2026-07-27: Resolve the native height-basis authority gap: legacy PL16 height uses
  total above-ground biomass, while CP-GSI02 `Bf` is foliar-only.
- [x] 2026-07-27: Amend canonical authority if the current-day height law or transition
  ordering is not already explicit.
- [x] 2026-07-27: Add contract-derived zero-to-positive transition tests and run the
  pre-implementation contract gate.
- [x] Correct production projection and prove all named real consumers.
- [x] Reconcile the exact 44-path diff and pass two independent terminal-HOLD
  legitimacy reviews.
- [ ] Pass the blocked unfiltered critical gate and dual terminal
  verification.
- [ ] Lift the CAL-04B production prerequisite and close this package.

## Correction Authority Envelope

Defect ID: `CAL04B-NATIVE-001`.

Observed violation: retained CAL-04B attempt 004 completed the 16,437-day
native-default case, then failed the frozen central interior vector `GSI-5557`
at lane 1/day 11,186. The production trace published
`LAI=0.007687569841550092` with missing/non-positive post-growth `canhgt`.
The typed `laned_active_rev21_operands` guard stopped the run. The immutable
reproducer is under
`/home/workdir/cal04b-objects-native-proof-interior-failure-004`.

Localized mechanism: the production day builder computes baseline legacy
growth state, then the generalized-GSI override changes current foliar biomass,
interception biomass, LAI, and cover without recomputing current canopy height.
The resulting state is incoherent on the first zero-to-positive GSI day.

In-scope correction surfaces:

- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/index.md` only if lifecycle metadata
  must change
- `crates/openwepp-runner/src/hillslope/direct_publication/**`
- `crates/openwepp-runner/src/hillslope/tests03/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
  only to expose the exact post-growth height consumed by erosion and frost
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
  only to record the exact dynamic height consumed by active frost
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
  only for contract-derived tests or required integration, not guard weakening
- adjacent owning tests required to prove the same production state
- this package, the CAL-04B hold-lift status, and work-package catalogs

Allowed edit classes:

- clarify canonical same-day current canopy-height authority and aliases;
- add contract-derived transition and guard tests;
- centralize authoritative current-day canopy-height projection so the same
  post-phenology state carries foliar mass, LAI, cover, and height;
- update typed state transfer and production publication to consume that state;
- add exact real-consumer and negative regressions;
- update only directly affected documentation/evidence.

Protected boundaries:

- do not weaken, remove, bypass, or add a static fallback around the positive
  LAI/positive canopy-height guard;
- do not invent surrogate, provisional, proxy, empirical, clamped, or
  heuristic canopy-height physics;
- do not select a friendlier GSI vector, shorten the reproducer, filter
  candidates by downstream behavior, or force fixture height;
- do not change calibration domains, observations, objectives, acceptance,
  Harvard custody, or any CAL-04B result;
- do not revive skeleton, shadow-only, or compatibility publication paths.

## Conversion Rule And HOLD Legitimacy

The observed state is valid at the GSI input boundary, the production source
mechanism is localized, the real consumer fails reproducibly, and this package
owns the contract/source/test surfaces needed to correct it. Therefore this
package must convert the finding into a contract-first production fix when
canonical authority resolves the current-day height law.

`HOLD` is exceptional. Diagnostic uncertainty, implementation effort, edit
size, or a partially working compatibility path are not valid stop conditions.
A hold is legitimate only if canonical authority is proven missing or
contradictory, required evidence becomes unavailable, or the mechanism is
proven outside every declared surface. Any hold must include the audit required
by `docs/defect_closure_execplans.md`.

## Seven-Gate Conversion Bar

1. **Valid input:** `GSI-5557` is a frozen central interior member of the
   admitted 9,261-vector domain and passes native GSI validation.
2. **Expected behavior authority:** `SC-PLANT-001` requires one coherent
   post-phenology state and real-consumer ordering; execution must resolve any
   missing explicit height law contract-first.
3. **Localized source:** baseline growth computes height, then the GSI override
   replaces biomass/LAI/cover without replacing height.
4. **Executable reproducer:** attempt 004 deterministically fails at lane 1,
   day 11,186 after 11,185 retained interior trace rows.
5. **Correction authority:** this package owns the exact contract, production,
   and test surfaces needed for closure.
6. **Validation authority:** the frozen case plan, contract tests, downstream
   real consumers, and critical full-workspace correctness can directly prove
   the correction.
7. **Protected boundaries:** the fail-closed guard, calibration/Harvard
   custody, and no-surrogate-physics rule are explicit and reviewable.

All seven gates are presently met for conversion to an implementation package,
subject to the contract-law adjudication in phase 1. If that adjudication proves
authority missing or contradictory, the package may hold only with the required
legitimacy audit.

## Contract-First Sequence

1. Determine whether `SC-PLANT-001` already gives an unambiguous authoritative
   same-day `Hc/canhgt` law for generalized-GSI state transitions.
2. Amend the canonical contract and Binding Exposure Index before production
   edits if authority or ordering is incomplete.
3. Add contract-derived tests for zero-to-positive, positive-to-zero, evergreen
   floor, boundary, and invalid-state behavior.
4. Run and retain the pre-implementation contract gate.
5. Correct production state projection and publication.
6. Prove the same post-phenology state reaches every named real consumer.

Physics authority defaults to `/workdir/wepp-forest_260430_baseline` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, together with current
`SC-PLANT-001`. If those authorities do not define the required law, stop for a
contract decision; do not synthesize a formula.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/index.md`
- `tools/check_sc_binding_exposure.py` only to preserve canonical
  `OBL-<DOMAIN>-<ROLE>-<NNN>` IDs in the newly required Binding Exposure Index
- `tests/python/test_check_sc_binding_exposure.py` for valid role-qualified and
  absent-core-ID checker regressions
- `crates/openwepp-runner/src/hillslope/direct_publication/**`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/tests03/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- directly owning adjacent tests selected by the authenticated intent plan
- `docs/work-packages/20260727-cal04b-native-gsi-canopy-height-coherence-hold-lift-001/**`
- CAL-04B status/handoff artifacts and `docs/work-packages/README.md`
- the frozen CAL-04B `artifacts/native-consumer-proof.csv` output only, as
  current hold-lift replay evidence and not a calibration/result claim
- the frozen CAL-04B `tools/native-proof.py` invalid-case error recognizer and
  one owning regression test only if the unchanged case reaches the canonical
  typed parser error but the verifier rejects wording alone; its plan, inputs,
  semantic expectation, and production path remain immutable
- `docs/planning/canopy-phenology-assurance-roadmap.md`

Any wider production or contract surface requires a prospective package
amendment and renewed intent review before edits.

## Dependencies

- Closed CAL-04B incident 004, HOLD audit, and immutable attempt root.
- `SC-PLANT-001`, its registry row, Binding Exposure Index, and applicable
  downstream consumer contracts.
- `docs/codex_exec_plans.md`, `docs/defect_closure_execplans.md`,
  `docs/work-packages/AGENTS.md`, and kernel preparation/gate standards.
- Pinned baseline commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` for any legacy-authoritative
  canopy-height equation or ordering.
- Current production runner, orchestrator growth state, Lane D active router,
  and their owning tests.

## Milestones And Phase Plan

### Phase 1 — intent and authority

Authenticate the exact diff/write-set plan, reproduce the retained transition,
map the Binding Exposure Index, and decide whether current `SC-PLANT-001`
already supplies the complete generalized-GSI current-day height law. Amend the
contract before tests if it does not.

### Phase 2 — contract tests and pre-implementation gate

Add zero-to-positive, positive-to-zero, evergreen-floor, boundary, and invalid
state vectors. Run the applicable A0/A1/A3 contract gate and retain its current
receipt before production code changes.

### Phase 3 — production correction and focused proof

Implement one authoritative same-day state projection. Preserve error
precedence and the LAI/height guard. Prove the exact transition and the entire
frozen CAL-04B native-proof case plan through production.

### Phase 4 — consumers and critical gates

Prove Lane D active routing, snow, ET, WB15 interception, erosion,
residue/litter, and frost consume the corrected state. Run the terminal plan,
including cargo-deny and campaign-strength full-workspace correctness.

### Phase 5 — review, verification, and hold lift

Reconcile the exact diff, line counts, artifacts, and every gate. Complete dual
review, disposition every finding, complete dual verification, archive the
prompt, lift the CAL-04B prerequisite, and close.

Every phase must retain direct current evidence for its own required gates.
Nothing attempted, failed, or discovered as current-scope may be relabeled as a
future/campaign obligation to mark a phase complete.

## Required Acceptance

- The exact retained `GSI-5557` zero-to-positive transition publishes one
  finite, domain-valid, contract-coherent post-phenology state.
- Full real-production runs pass the entire frozen CAL-04B native-proof case
  plan: native-default, `GSI-5557`, `GSI-0001`, lowest saturated candidate,
  all-operands, and all six one-at-a-time operand perturbations; the invalid
  threshold-order case fails with its typed error.
- Snow canopy attenuation, ET LAI/height, WB15 interception, erosion-facing
  canopy, residue/litter handoff, frost publication, and Lane D active routing
  read the corrected same-day state. Source and runtime negative proofs reject
  static, stale, shadow, wrapper, and compatibility paths.
- The LAI/height guard remains fail-closed and its negative tests pass.
- Applicable `SC-PLANT-001` A0, A1, and A3 obligations and typed guards pass.
- The critical terminal plan runs campaign-strength full-workspace correctness
  through the canonical gate workflow; coverage/CRAP remains
  `DEFERRED_TO_QUALITY_CI`.
- Rustfmt, warnings-denied Clippy, cargo-deny, documentation lint,
  exact-diff/write-set reconciliation, placeholder scan, and `.rs` line-count
  governance pass.
- Two independent reviews, explicit finding disposition, and two independent
  terminal verifications pass with no undispositioned finding.
- CAL-04B remains held and Harvard sealed until this package closes; no
  calibration population or holdout execution belongs to this package.

## Review And Delegation

Subagent requirement: REQUIRED. This package explicitly authorizes subagent
spawning/delegation to two independent read-only contract/implementation
reviewers, the `comparator_suite_runner` for selected heavy full-workspace and
real-consumer execution, and two independent read-only terminal verifiers.
Expected outputs are compact findings, exact test counts, receipt identities,
and artifact paths. Production writes remain owned by the primary executor;
reviewers and verifiers are read-only.

## Security Impact

This change affects fail-closed production state publication and downstream
consumer routing. Review must prove the correction neither weakens guard/error
precedence nor introduces stale/static fallback state. If the terminal diff
touches external-authority suite posture, fixtures, or required-case bindings,
run both required authority-suite anti-evasion gates.

## Required Deliverables

The package-local `artifacts/` directory pre-creates the intent plan, authority
map, contract and test evidence, pre-implementation contract gate, production
evidence, kernel-profile checklist, owned-file manifest, gate results,
line-count governance, reviews, finding disposition, verifications, final
disposition, hold audit, and worker handoff.

## Surprises & Discoveries

- Native-default success did not exercise the interior vector's
  zero-to-positive transition.
- The correct runtime guard converted a latent incoherent state into an exact
  production-path failure before calibration population work.
- Active erosion had two additional stale-height seams: native erosion used an
  optional PMET zero fallback, while frost retained management-seed height.
  Both now consume the checked post-growth native height.
- The frozen invalid-case verifier recognized only runtime threshold wording,
  not the canonical typed parser wording. A bounded recognizer repair left the
  case plan, input, expectation, and production behavior unchanged.
- The exact native replay passes 12/12 and the non-assurance full profile
  passes 2,180/2,180, but the declared full-workspace gate is blocked by
  assurance identity drift predating this package: the generated lock binds
  the predecessor hash of `tests/fixtures/cancov_forest/README.md`.

## Decision Log

- Decision: keep the guard and move correction authority to a dedicated
  contract-first production package.
  Rationale: changing CAL-04B's selector or copied fixture would evade the
  required real-consumer proof.
  Date/Author: 2026-07-27 / Codex.

## Outcomes & Retrospective

The production defect and every named native consumer are corrected and
proven, including the retained `GSI-5557` transition and the complete frozen
12-case replay. No calibration population or Harvard command ran.

The package cannot close or lift the CAL-04B prerequisite because its mandatory
full-workspace gate remains red on pre-existing assurance dossier identity
drift. Canonical assurance governance forbids adopting report-evidence drift
through an implementation rebind. A separately authorized assurance lifecycle
correction must reconcile that evidence source, after which this package must
rerun the unfiltered full profile and terminal review/verification.

## Idempotence And Recovery

The retained attempt-004 root is read-only evidence. Every new production or
heavy execution uses a fresh durable attempt and never overwrites the
reproducer. An observed semantic failure is not retried unchanged.

## Defect-Shaped Handoff

First actionable item: close defect
`ASSURANCE-CANOPY-README-IDENTITY-001` under separately authorized assurance
lifecycle authority. The observable failure is the unfiltered full profile's
typed generated-identity drift for
`tests/fixtures/cancov_forest/README.md`. Commit `502dd745` changed that admitted
`IN_REVIEW` report dependency without a corresponding governed assurance
transaction, leaving `assurance/v2/identity.lock.json` bound to the predecessor
hash.

The new package must own the affected assurance report, review lock, generated
identity transaction, and directly required evidence surfaces; read
`assurance/v2/README.md`, the snow/frozen-soil report and review lock, and the
assurance amendment contracts before mutation. Its acceptance target is a
governed, review-valid identity reconciliation followed by a passing exact-head
unfiltered full profile. Manual hash editing, `rebind-implementation` adoption
of report evidence, reverting the prior research-documentation change, or
merely inspecting the next assurance function are forbidden relay routes.

After that separate defect closes, return to this package for its still-pending
full-profile pass, dual terminal verification, prompt archival, prerequisite
lift, and closure. No calibration population or Harvard execution is
authorized by this handoff.
