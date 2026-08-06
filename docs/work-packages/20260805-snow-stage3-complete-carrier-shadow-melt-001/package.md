# Stage 3 Complete Carrier And Shadow Melt

Status: `executed / carrier plausibility FAIL / structural and authority HOLD`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.

## Purpose

Resume the CoE-to-Stage-3 cutover in the scientifically forced order: complete
and validate the Stage 3 surface-energy carrier first, then compute the
`INV-SNOWENERGY-029` cold-content-first melt chronology in shadow state while
CoE remains the sole authoritative melt owner. Do not cut over while the
carrier is incomplete, seasonal net energy is implausible, available ice is
exhausted with positive terminal energy, or any linked-ledger gate remains
open.

## User Authority And Prospective Freeze

Direct user instruction on 2026-08-05 authorizes resuming CoE cutover work with
carrier-first shadow sequencing. The following predictions are registered
before implementation or result inspection:

- mid-winter melt decreases;
- peak SWE increases;
- peak SWE date moves later;
- spring melt rate increases; and
- seasonal Stage 3 net energy approaches physical closure near zero rather
  than retaining approximately `+216.87 MJ m^-2` at Snowbird.

These are prospective directional tests, not tuning targets. Thresholds,
acceptance rules, or site windows may not be changed after result inspection
without a new package and explicit result-aware disclosure.

Follow-up user authority on 2026-08-05 grants explicit definition of the
turbulent virtual-instrument geometry and directs continuation of the work it
held up. Contract version 8 binds `z_T=z_q=z_u=5 m` above the instantaneous
modeled snow surface and exposed-snow `z_0,aero=0.005 m` for CLIGEN/openWEPP
forcing. These are fixed metadata, not calibration knobs.

## Frozen Gate Observation

For Snowbird, median per primary accumulation window in retained Stage 3
evidence:

| Quantity | Median |
| --- | ---: |
| Absorbed shortwave | `223.25 MJ m^-2` |
| Surface energy applied | `6.39 MJ m^-2` (`2.9%`) |
| Unused positive energy | `216.87 MJ m^-2` (`97.1%`) |
| Latent-fusion equivalent of unused energy | `0.649 m` SWE |
| Median snowfall | `0.769 m` SWE |
| Median pack loss | `0.530 m` SWE |

The unused positive energy is equivalent to about `84%` of median seasonal
snowfall. It is not a post-cutover melt forecast. It demonstrates that the
retained shortwave-dominated carrier was not physically complete and motivated
the terminal-energy gate. It does not predict the complete carrier's
`Q_unallocated_after_exhaustion`: the retained and new quantities have
different accounting lineages.

## Scientific Freeze

- `GAP-SNOWENERGY-011` remains open: complete sensible and
  precipitation-advected heat are absent from the carrier.
- Explicit longwave is required for the shadow carrier and cutover evidence;
  its current default-off compatibility selector does not authorize a
  shortwave-only melt carrier.
- Complete net radiation and the admitted sensible, latent, conduction, and
  precipitation-advection operands must be summed exactly once with the
  positive-toward-snow convention.
- Cold content is satisfied before latent-fusion conversion.
- `m_melt=min(Q_excess/L_f,m_ice_available)` is shadow-only in this package.
- `Q_unallocated_after_exhaustion` must be zero for every cutover-eligible
  substep. A positive value is not discarded or proxied and blocks cutover.
  A zero value above the unresolved thin-pack boundary proves only allocation
  over the evaluated resolved domain; it does not close the terminal event.
- CoE remains the sole authoritative mass-mutating melt owner throughout this
  package. Shadow Stage 3 melt cannot affect pack state, routing, public
  outputs, defaults, or downstream consumers.
- The principal mechanistic hypothesis is thermodynamic gating, not flux
  magnitude: cold content should suppress accumulation-season melt and defer
  melt toward an isothermal spring pack.
- Retained 21M evidence that `C_open` is net negative while the unvalidated
  2008 `C_canopy` branch is the sole net-positive CoE contributor is treated as
  evidence of formulation misspecification risk, not a calibration target.

## Included Scope

1. Freeze exact retained 21L/21M/21N energy and timing evidence and reconstruct
   every Snowbird operand independently.
2. Complete the typed Stage 3 shadow carrier with explicit net longwave,
   sensible heat, latent heat, conduction, and precipitation-advected heat.
3. Require complete-carrier shadow evaluation regardless of compatibility
   longwave selector while leaving authoritative compatibility behavior and
   defaults unchanged.
4. Add typed shadow cold-content, available-ice, melt, terminal-energy, and
   linked mass/energy ledgers derived from `INV-SNOWENERGY-029/030`.
5. Persist internal diagnostic evidence sufficient to compare Stage 3 shadow
   chronology with authoritative CoE on the same substeps and accumulation
   windows.
6. Expose a default-off, diagnostic-only runner environment switch and trace
   operands needed to execute and independently reconstruct the real Snowbird
   shadow; this surface cannot select authoritative melt behavior.
7. Prove exact noninterference: CoE remains the only mass-mutating melt owner
   and all established runtime/public outputs remain byte-identical.
8. Evaluate the frozen predictions and seasonal carrier plausibility without
   tuning to outcomes.

## Excluded Scope

- Stage 3 authoritative melt conversion or CoE retirement.
- Any dual-owner state mutation.
- Authoritative selector, public-output, calibration, site-window, observation,
  or release change. The version-8 typed CLIGEN geometry definitions are the
  sole default/input-contract change; the new shadow switch and fields are
  diagnostic-only and default off.
- Treating shadow results as cutover authority when terminal energy, thin-pack,
  same-substep liquid, or real-consumer gates remain open.

## Intended Write Set

- `docs/work-packages/20260805-snow-stage3-complete-carrier-shadow-melt-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-meteorology/src/surface_energy.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`
- `tests/integration/snow_surface_eb03_runtime.rs`
- `tests/integration/snow_surface_eb03_contract.rs`
- `tests/integration/snow_surface_eb04w_accumulation_melt_diagnostics_contract.rs`
- ignored evidence under `target/snow_stage3_complete_carrier_shadow_melt/`

Production edits may begin after the version-8 contract gate records the
explicit turbulent-transfer geometry amendment.

## Phase Plan

### Phase 1 — Freeze And Seam Inventory

Bind exact inputs, independently reconstruct the Snowbird gate number, map all
carrier producers/consumers and unit/sign/temporal boundaries, narrow the
write set, and establish byte-identical authoritative baselines.

### Phase 2 — Complete Shadow Carrier

Implement typed complete flux operands and exact-one integration. Longwave is
mandatory in shadow evidence. Preserve compatibility selectors and CoE state.

### Phase 3 — Shadow Melt Chronology

Compute cold-content-first bounded Stage 3 melt and linked ledgers without
mutating pack or routing state. Fail closed on incomplete operands, nonfinite
values, closure violations, or positive terminal unallocated energy.

### Phase 4 — Prospective Evaluation

Run real accumulation-window evidence, evaluate the frozen directions and
seasonal net-energy plausibility, and disposition rather than tune every
failure. A later atomic cutover package is authorized only if all holds close.

## Validation And Gates

- Contract invariants and exact typed operand reconstruction.
- Analytical sign/unit/time-integration vectors for every carrier component.
- Independent energy and mass closure reconstruction.
- Shadow/noninterference tests proving authoritative CoE state and outputs are
  unchanged.
- Real Snowbird and campaign-window consumer evidence.
- Focused, quick, frost, and immediate full-workspace correctness regression.
- Dual domain-science/Rust review, dual terminal verification, and heavy runner.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation to one
read-only domain-science reviewer, one read-only Rust correctness reviewer, one
read-only Rust QA reviewer, two read-only terminal verifiers, and one
`comparator_suite_runner` for heavy campaign/full-workspace gates. Subagents
may not edit tracked files or reinterpret frozen acceptance rules.

## Progress

- [x] (2026-08-05) User authorized carrier-first shadow resumption.
- [x] (2026-08-05) Froze directional predictions and the Snowbird retained
  energy gate before implementation.
- [x] (2026-08-05) Scaffolded this package before source edits.
- [x] (2026-08-05) Mapped and narrowed the exact carrier seams; confirmed
  existing typed longwave, turbulent, and precipitation-advection primitives.
- [x] (2026-08-05) User granted input authority; contract version 8 binds the
  typed `5 m` virtual heights and `0.005 m` exposed-snow roughness.
- [x] (2026-08-05) Implemented the explicit typed geometry through the real
  runner-to-hydrology boundary and passed its contract/focused gates.
- [x] (2026-08-05) Implemented and focused-tested an opt-in noninterfering
  complete-carrier shadow with longwave, turbulent, precipitation-advection,
  and conduction operands.
- [x] Reconstructed the retained Snowbird absorbed-shortwave median as
  `223.2500438 MJ m^-2` over the frozen 35-window support.
- [x] Implemented and focused-tested the default-off complete carrier.
- [x] Implemented a within-day sequential, non-mutating cold-content/melt
  shadow with explicit fusion and terminal-energy operands; persistent
  cross-day state and linked liquid disposition remain open.
- [x] Reconstructed the real Snowbird primary windows and dispositioned the
  prescribed-state carrier plausibility screen as failed without tuning, then
  reran the frozen evaluation after correcting snowfall-mass and fusion-heat
  arithmetic.
- [x] Dispositioned resolved-domain terminal allocation as numerically closed
  while leaving the thin-pack terminal event open and not evaluable.
- [x] Dispositioned all four chronology predictions as not evaluable because
  the shadow reinitializes daily from the post-CoE pack and lacks coherent
  snowfall/liquid chronology.
- [x] Recorded independent domain, Rust, and QA review findings and retained
  the package in HOLD rather than claiming cutover or completion.
- [x] Retained the `3,177`-line solver extraction as a structural successor
  gate.

## Decision Log

- Decision: longwave is mandatory for shadow cutover evidence even though it
  remains a compatibility default-off selector. Rationale: a shortwave-only
  carrier cannot satisfy seasonal physical plausibility or the terminal-energy
  cutover gate. Date/Author: 2026-08-05 / Codex.
- Decision: separate shadow computation from authoritative deployment.
  Rationale: this preserves exact-one melt ownership while allowing the
  complete carrier and cold-content mechanism to be tested prospectively.
  Date/Author: 2026-08-05 / Codex.
- Decision: close the implementation as an executed HOLD rather than extend it
  after inspecting the Snowbird result. Rationale: the frozen carrier screen
  failed at corrected `+170.2536 MJ m^-2`; current trace observability cannot
  isolate the responsible term, and persistence would require additional state
  authority.
  Date/Author: 2026-08-06 / Codex.
- Decision: route evaluation-shadow authority and per-term multi-site carrier
  observability ahead of terminal land-surface work. Rationale: a downstream
  recipient cannot repair an upstream carrier plausibility failure.
  Date/Author: 2026-08-06 / Codex.

## Outcomes

The user granted the missing turbulent-input authority on 2026-08-05.
Contract version 8 defines the CLIGEN/openWEPP virtual-instrument geometry, and
the real runner carries those typed values into a default-off complete-energy
shadow. The shadow adds explicit net longwave, Monin-Obukhov sensible and
latent heat, precipitation-advection, and active/lower conduction, then applies
cold-content-first bounded fusion on cloned snow state. CoE remains the only
authoritative melt and mass owner.

The frozen Snowbird absorbed-shortwave median was independently recovered.
The complete-carrier prescribed-state screen then produced median seasonal
  energy of `+170.2536 MJ m^-2`, positive excess of `196.4733 MJ m^-2`, shadow
  melt of `0.5889 m` SWE, and authoritative CoE raw melt of `0.4101 m`. The
strongly positive carrier result fails the prospective plausibility screen. It
does not identify a defective flux term and is not a coherent post-cutover
seasonal balance.

Resolved evaluated substeps satisfied the pre-vapor-debit allocation identity
within `1.521e-9 J m^-2`, and their median
`Q_unallocated_after_exhaustion` was numerically zero. The shadow does not emit
cold content exported with sublimated mass, so this is not whole-state energy
closure. It also stops before the unresolved `1 kg m^-2` terminal event;
residual-snow exhaustion, post-snow energy, and the actual terminal gate remain
open. The legacy
`unused_positive_energy` quantity and new `Q_unallocated_after_exhaustion` are
not interchangeable.

The four frozen chronology directions are not evaluable. The shadow advances
sequentially only within each daily call, reinitializes from the post-CoE pack,
and lacks persistent snowfall, liquid, restart, and receiving-state ownership.
The adverse `0.5889 m` versus `0.4101 m` melt comparison is a diagnostic signal,
not evidence that mid-winter melt increased in a coherent Stage 3 simulation.

The package closes as an executed HOLD. It does not authorize persistence,
terminal meltout, a snow-to-soil energy handoff, Stage 3 publication, CoE
retirement, or a default change. The next result-bearing work must first add
contract-scoped evaluation-shadow authority and shadow-specific per-term
observability, freeze the paired-window operator, and audit all four canonical
sites. A behavior-neutral solver extraction must also reduce
`runoff_reconciliation.rs` below the `3,000`-line closure threshold before
further feature work in that module.

Historical execution deviations are retained rather than normalized away.
Commits `122c88af` and `24676c6d` expanded the declared write set in the same
increment that first edited the added paths. Contract version 8 landed at
`478fa788`, while the source-binding integration guard remained pinned to
version 7 until `24676c6d`. The terminal source is consistent and the current
guard passes, but later packages must amend write sets before edits and update
contract authority plus its binding guard in one stable increment.
