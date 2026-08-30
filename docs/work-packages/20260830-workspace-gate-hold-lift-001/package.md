# Workspace gate hold-lift

Status: `ACTIVE — EXECUTION`

Execution mode: `package-end-to-end`

This is a living ExecPlan maintained under `docs/codex_exec_plans.md`.

## Objective

Lift the mandatory workspace-gate HOLD recorded by
`20260830-snow-stage3-cold-content-fixed-point-optimization-001` by obtaining a
passing warnings-denied workspace Clippy command and a passing complete
full-workspace correctness profile on one exact clean terminal source identity.

## Progress

- [x] 2026-08-30: authorize and scaffold the hold-lift package.
- [x] 2026-08-30: freeze current source identity and classify retained current
  failures; delegated isolated-baseline profile remains in flight.
- [ ] 2026-08-30: correct warnings-denied and attributable correctness defects.
- [ ] 2026-08-30: execute exact-clean terminal mandatory gates.
- [ ] 2026-08-30: complete dual review, dual verification, disposition, and delivery.

## Observed defects

- `WGHL-CLIPPY-001`: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  exits 101 after `filter_map_bool_then` in `openwepp-coupled-time` and
  `similar_names` in `openwepp-biogeochemistry`; Cargo may expose further
  diagnostics after those are corrected.
- `WGHL-FULL-001`: `cargo nextest run --workspace --profile full` completes
  3,628 attempted tests with 3,503 pass, 96 fail, and 29 timeout. Three stale
  accepted-endpoint source scans are already corrected; 122 adverse outcomes
  remain unclassified against the fixed-point package baseline.
- `WGHL-FULL-001D`: the authentic exact-floor open-snow Picard map oscillates
  across the terminal one-volume snow enthalpy kink: a mixed `0 C` endpoint
  maps to a dry frozen `196.469 K` endpoint below the already-required LSE
  domain. Componentwise contraction correctly refuses the phase/posture
  crossing; the real `dff_ws2` fixture fails at `60 s`. The lawful v31 W/H
  projection then exposes a pure deposition-to-sublimation active-set reversal
  on `1860..1920 s`, which its required vapor-disposition guard correctly
  refuses.
- `WGHL-FULL-001E`: committed Stage-3 publication omits the requested accepted
  WAT5 producer projection and reconstructs accepted upstream runon from the
  LSE forcing field that correctly rejects routed parcels, masking transaction
  errors and collapsing `UpStrmQ` into local liquid.
- `WGHL-FULL-001F`: at an otherwise converged covered-column state, the full
  Newton trial may cross a closed phase bound while the first domain-valid
  halved trial has sub-tolerance prospective steps. The solver nevertheless
  requires strict residual decrease at floating-point roundoff and exhausts
  all 20 backtracking halvings in two interior terminal-event tests.
- `WGHL-FULL-001G`: valid `p61` and native-forest fixtures enter frozen forest
  litter, which current LSE and surface-liquid contracts explicitly reject.
  The distinct-authority successor
  `20260830-frozen-forest-litter-phase-authority-001` is scaffolded and active;
  its passing real-consumer evidence is a dependency of this hold-lift.
- `WGHL-FULL-001H`: after the corrected covered LSE solve, a terminal snow-free
  successor beginning at `600 s` is bounded only by adaptive cadence/parent end
  and crosses the accepted terminal child ending at `900 s`; qualification
  correctly rejects the successor chronology.
- `WGHL-FULL-001I`: a valid accepted infiltration enthalpy credit on the
  canonical WAT5 path is nonzero but only `1.10875e-7` ULP of the persistent
  soil-layer binary64 enthalpy. Scalar installation cannot change the owner
  bits, so strict independent closure correctly fails `SURFACELIQUID-E-003`.

## Rationale

Focused solver, contract, authority, conservation, and canonical performance
evidence passes, but package governance prohibits completion while mandatory
campaign-strength workspace gates fail. This package owns the cross-workspace
classification and repair needed to replace that HOLD with direct passing
evidence. It does not treat pre-existing failures, retry, or location outside a
prior write set as an implicit waiver.

## Included scope

- exact-source failure inventory and baseline/current comparison;
- narrow warnings-denied source-quality corrections;
- fixture, orchestration, runner, CLI, assurance, and test-support corrections
  proven necessary by the complete profile;
- a contract-first, exact-floor terminal-one-volume phase-aware unpublished
  contraction for the authentic open-snow fixed-point map, reconstructed in
  canonical total-water/enthalpy coordinates and preserving authentic-only
  acceptance;
- conservation-sensitive accepted Stage-3 WAT5 and upstream-runon publication
  reconstruction from sealed accepted producer/custody operands;
- a contract-first covered-column no-update termination witness evaluated on
  the first domain-valid halved Newton candidate when current residuals and
  prospective steps already satisfy every unchanged convergence threshold;
- producer-side terminal successor partitioning at the sealed accepted-child
  end, preserving the unchanged qualification validator and physical supports;
- contract-first exact soil-thermal enthalpy carry custody for accepted energy
  below scalar-state representability, with versioned owner/receipt/restart
  schemas, exact rollback, and unchanged constitutive physics;
- delegated execution and handback of the distinct contract-first frozen
  forest-litter phase successor before terminal workspace reruns;
- timeout diagnosis and deterministic resource/scheduling correction when the
  test's semantic obligation is preserved;
- direct focused tests followed by exact-clean full Clippy and full Nextest;
- review, verification, line-count, terminal-diff, and truthful disposition.

## Excluded scope

- numerical tolerance, conservation threshold, physical constitutive equation,
  event chronology, custody, topology, receipt, rollback, or adaptive temporal-
  policy changes beyond the exact phase-aware unpublished numerical iterate
  safeguard explicitly amended below;
- deleting, ignoring, weakening, filtering, reclassifying, or relaxing a test
  merely to make the workspace profile pass;
- TESTGATE/planner repair, coverage/CRAP campaigns, dependency changes, release,
  publication, or unrelated performance optimization;
- treating a baseline failure as waived without an explicit authoritative
  correction or accepted lifecycle disposition.

## Correction authority envelope

The package may make behavior-preserving source-quality edits and correct
production/test infrastructure when a retained failure plus focused evidence
demonstrates the defect. Every Rust implementation path must be entered in
`artifacts/failure-inventory.md` with failure ID, owner, exact source/test path,
classification, intended correction, and focused command before editing.

This package is amended to own the following classified exceptions. First,
`WGHL-FULL-001D` must follow contract-first sequencing and may admit only a
deterministic exact-floor terminal-one-volume phase-aware contraction. It must
derive the unpublished intermediate from immutable physical beginning state,
complete support water/energy operands, and the canonical SC-SNOWENERGY total-
water/enthalpy phase projection; it may not independently interpolate liquid,
cold content, melt, refreeze, or cumulative ledger fields. It must retain raw
authentic history and require a later fresh authentic image for convergence,
final replay, acceptance, and publication. It may not change the `60 s` floor,
convergence tolerances/cap, physical constitutive equations, event topology,
custody, receipts, rollback, or authentic-only publication. Second,
`WGHL-FULL-001E` may repair
public-output projection only from sealed accepted operands, with prospective
operand lineage, anti-tautology tests, explicit wrong-formula rejection,
independent reconstruction, and real closure evidence. Any further science,
numerical-policy, public schema, or serialization change remains outside the
envelope and requires another prospective amendment.

Third, `WGHL-FULL-001F` may extend only the existing no-update convergence
witness to the first domain-valid halved Newton candidate after the full trial
fails the existing no-update witness because it is domain-invalid or any
governed prospective step exceeds its unchanged threshold. It accepts the
current iterate without applying a step only when the current complete residual
vector and that first eligible halved candidate's exact component step norms
satisfy all existing thresholds. It changes no phase bound, residual/step tolerance,
strict-decrease rule for actual updates, iteration/backtracking limit, ledger,
receipt, event, custody, rollback, or 60-second floor.

Fourth, `WGHL-FULL-001I` must follow contract-first sequencing and may admit
only a receiver-owned exact dyadic carry for soil-layer enthalpy that cannot be
represented in the existing binary64 high term. The canonical total is
`E = exact(H_hi) + R`; `R` has one normalized signed-dyadic wire form, is
retained exactly, and is rejoined on every later solve and credit. Each accepted
candidate must aggregate the exact beginning total plus the canonical ordered
accepted soil, top-boundary, and infiltration energy operands, round the total
once to nearest-even finite `H_hi`, and retain exactly
`R = E - exact(H_hi)`. The amendment must freeze every V1 byte/digest, provide
explicit V1-to-V2 zero-carry migration with no production downgrade, bind the
carry through typed credit receipts and successor restart/checkpoint schemas,
and prove byte-exact rollback. It may not zero a nonzero carry, use tolerance,
`nextafter`, forced-ULP installation, a producer residual, a persisted
diagnostic, or change any constitutive equation, heat capacity, chronology,
event, custody, receipt, rollback, or temporal floor.

Fifth, `SC-SNOWENERGY-001` v32 supersedes the in-review v31 controller while
retaining its same-disposition W/H projection and vapor-disposition refusal.
When two independently closed, pure one-sided actual-vapor support images have
identical immutable beginning/support/identity/custody/receipt structure and
strictly opposite signed actual vapor, the covered solver may localize one
unpublished vapor-active-set interface. It derives the unique convex vapor-root
fraction from the signed endpoint masses, sets actual vapor, deposition,
sublimation, and their already-once latent-energy component to exact positive
zero, contracts external liquid and every nonlatent ordered energy component at
that fraction, recomputes complete energy, and applies the unchanged canonical
total-water/enthalpy phase projection. A subsequent zero-to-one-sided numerical
branch entry may preserve the fresh authentic endpoint's positive finite
specific latent heat under the existing bounded support-scaled Picard weight.
These numerical images are never authentic, converged, finalizable, replayable,
acceptable, or publishable; only a later fresh authentic map image may satisfy
those branches. Mixed deposition/sublimation, failed mass/latent/component
closure, or any identity/event/topology/custody/receipt change remains typed
failure. This private transition may apply on any exact covered support at or
above the unchanged 60-second floor solely when ordinary guarded contraction is
blocked by the terminal-one-volume phase/vapor active set; it changes no
adaptive temporal policy, tolerance, iteration cap, constitutive equation,
event, custody, receipt, rollback, public schema, persistence, or publication.

Sixth, `SC-SNOWENERGY-001` v33 supersedes the nonprogressing v32 controller on
the exact affected terminal-one-volume covered support. Versions 31 and 32 are
retained as diagnostic reconstruction/refusal oracles but must not remain in
production control flow: retained evidence proves the v32 root/branch map has
an exact period-two reset and no fresh-authentic fixed point. After an authentic
A/B/A phase/vapor active-set cycle with identical support, identity, event,
topology, custody, and receipt operands, v33 may invoke a private reduced
semismooth coupled solve of the actual physical residuals. Its unknowns are
ending total represented snow water, ending snow enthalpy, and the coupled soil
endpoint coordinates required by the unchanged CN snow-soil receipt; every
trial applies the unchanged canonical W/H phase projection and existing covered
LSE equations/ordered flux reconstruction. Residuals independently close exact
water, complete energy including signed actual-vapor latent energy, the
unchanged soil CN block, LSE components, and sealed receipts. A safeguarded
deterministic Newton/trust-region solve is private and unpublished; only an
exact fresh coupled-root replay that independently reseals every receipt may be
accepted and published. This is neither a convex event-time mixture nor
synthetic Picard acceptance. Existing discrete event/forcing/receipt boundaries
still force exact partition/refusal. No floor, tolerance, 96-iteration Picard
cap, constitutive equation, event, custody, receipt, rollback, public schema,
persistence, or diagnostic policy changes.

## Intended write set

- `docs/work-packages/README.md`;
- this package tree;
- `crates/openwepp-coupled-time/src/event.rs`;
- `crates/openwepp-coupled-time/tests/authority.rs`;
- `crates/openwepp-biogeochemistry/src/lib.rs`;
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`;
- `crates/openwepp-runner/tests/test_fixture_authority_contract.rs`;
- `tests/fixtures/watershed/p102-sediment-active/README.md`;
- `tests/fixtures/watershed/p102-sediment-active/input-manifest.sha256`;
- `tests/fixtures/watershed/p102-sediment-active/runs/H1.source.run.snow_stage3_v11_owner_seed.json`;
- `tests/integration/vegetation_boundary_authority_contract.rs`;
- `crates/openwepp-vegetation/src/v11.rs`;
- `crates/openwepp-vegetation/src/v11/tests/v11_bgc_tests.rs`;
- `assurance/v2/identity.lock.json`;
- `assurance/v2/reports/linear-groundwater-reservoir-recurrence/review.lock.json`;
- `assurance/v2/reports/native-forest-canopy-phenology-evaluation/review.lock.json`;
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/report.yaml`;
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json`;
- the single typed `adopt-report-source` receipt generated under
  `assurance/v2/transactions/`;
- `tools/release/authority-policy/impact-map.json` for exact admission of newly
  touched science-bound test surfaces;
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`;
- `docs/specifications/science-contracts/index.md`;
- `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs`;
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_execution_tests.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/fixed_point.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_tests.rs`;
- new `crates/openwepp-hillslope-orchestrator/src/v11_covered/phase_consistent_coupled_solve.rs`;
- `crates/openwepp-land-surface-energy/src/solver_covered_evaluation.rs` and
  `crates/openwepp-land-surface-energy/src/transaction.rs` only if the v33
  nested trial requires a read-only private evaluation seam, with no equation,
  tolerance, or public-schema change;
- `tests/integration/dff_ws2_ksatadj_direct_runtime.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/stage3_committed_publication.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/stage3_committed_publication_wat5.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/stage3_committed_publication_tests.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/stage3_committed_publication_tests_tail.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_publication_retention.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_routing_tests.rs`;
- `crates/openwepp-runner/src/hillslope/tests03/wat5_output_transaction.rs`;
- `tests/integration/cli03_runner_contract_derived_tests.rs`;
- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`;
- `crates/openwepp-land-surface-energy/src/solver_covered_solve.rs`;
- exact existing/new covered-solver test modules under
  `crates/openwepp-land-surface-energy/src/` entered in the failure inventory;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs`
  as unchanged real-consumer evidence only;
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_execution.rs`;
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_execution_tests.rs`;
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_qualification_crossjoin_tests.rs`;
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_qualification_crossjoin_child_tests.rs`;
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_attachment_receipts.rs`
  as unchanged poison/validator evidence only;
- `tests/integration/land_surface_energy_balance_authority_contract.rs` for the
  exact v13/`INV-LANDSURFACEENERGY-139` source binding exposed by 001F;
- `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md`;
- `tests/integration/surface_liquid_hydrology_custody_authority_contract.rs`;
- `crates/openwepp-land-surface-energy/src/lib.rs`;
- `crates/openwepp-land-surface-energy/src/owner_envelope.rs`;
- `crates/openwepp-land-surface-energy/src/transaction.rs`;
- new `crates/openwepp-land-surface-energy/src/exact_dyadic_enthalpy.rs` and
  included focused tests;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_soil_thermal.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_serialization.rs`;
- `crates/openwepp-hillslope-orchestrator/src/canonical_owner_bytes.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/receiver_validation.rs`;
- `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/finalization_sealing.rs`;
- exact focused soil-thermal receiver/closure test modules entered under
  `WGHL-FULL-001I` in the failure inventory before edits;
- `crates/openwepp-persisted-restart-v1/src/lib.rs`;
- `crates/openwepp-persisted-restart-v1/src/scientific_owners.rs`;
- `crates/openwepp-persisted-restart-v1/src/checkpoint.rs`;
- `crates/openwepp-persisted-restart-v1/src/projection.rs`;
- `crates/openwepp-persisted-restart-v1/src/host.rs`;
- `crates/openwepp-persisted-restart-v1/src/transaction.rs`;
- new `crates/openwepp-persisted-restart-v1/src/soil_thermal_v2.rs`;
- new `crates/openwepp-persisted-restart-v1/src/scientific_owners_v2.rs`;
- new `crates/openwepp-persisted-restart-v1/src/checkpoint_v2.rs`;
- new `crates/openwepp-persisted-restart-v1/src/projection_v2.rs`;
- new `crates/openwepp-persisted-restart-v1/src/host_v2.rs`;
- new `crates/openwepp-persisted-restart-v1/src/transaction_v2.rs`;
- new `crates/openwepp-persisted-restart-v1/src/v2_tests.rs`;
- `crates/openwepp-runner/src/hillslope/snow_stage3_v11_production_seed.rs` and
  exact successor seed tests only if compile or real-consumer adoption proves
  the versioned seed handoff is required;
- exact production/test paths under `crates/openwepp-assurance/`,
  `crates/openwepp-runner/`, `crates/openwepp-hillslope-orchestrator/`, and
  `tests/integration/` entered prospectively in the failure inventory after
  baseline/current classification;
- `.config/nextest.toml` only if measured timeout evidence proves a scheduler
  defect and semantic test duration/coverage remains unchanged.

## Required evidence

- source identities and retained-log hashes;
- complete deduplicated baseline/current failure inventories;
- causality classification for every current adverse outcome;
- focused before/after evidence for each correction family;
- exact-clean warnings-denied Clippy and full-workspace Nextest results;
- no-test-weakening audit, line-count reconciliation, terminal diff, dual
  independent review, and dual independent verification.

## Gate selection

Risk is `CRITICAL`: this package owns workspace-wide correctness execution and
may touch cross-domain orchestration/test infrastructure. Mandatory terminal
commands are:

- `nix develop -c cargo fmt --all -- --check`;
- `nix develop -c cargo clippy --workspace --all-targets --all-features -- -D warnings`;
- `env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --workspace --profile full`;
- every focused command named by the prospective failure inventory;
- `git diff --check` and exact terminal write-set reconciliation.

Additional A0/A1/A3, anti-evasion, contract, conservation, consumer, restart,
or publication commands become mandatory if the exact diff touches their
owning surfaces. No faster profile substitutes for the selected full profile.

Contract-first sequence for `WGHL-FULL-001D` is mandatory: amend canonical
authority; author/adjust contract-derived tests; record a failing
pre-implementation gate on the unchanged production implementation; only then
edit production code. Kernel-profile compliance and dual contract review are
blocking.

The v32 successor repeats that contract-first sequence. Its retained pre-red
must include the exact `1860..1920 s` vapor/latent operands, the unique
`alpha=0.04393657257739406` root, proof that affine latent interpolation leaves
an inadmissible nonzero `45.77845449909091 J m^-2` at zero vapor, and direct
support above 60 seconds. Mandatory green evidence includes opposite-sign/root/
branch-entry and same-sign-v31 vectors; mixed-disposition, capacity, nonfinite,
component/identity/event, cap, rollback, and no-intermediate-publication
refusals; real DFF accepted/rejected counts, widths, runtime, limiting reasons,
and independent mass/energy/vapor closure.

The v33 successor repeats that contract-first sequence independently. Its
retained pre-red must reproduce the exact 60-second and 120-second v32
period-two/96-cap refusal, fail solely for the missing v33 coupled-solver seam,
and bind independent cold/phase/fusion-boundary residual vectors with known
roots and exact replay. Mandatory green evidence includes root results distinct
from every affine v31/v32 image; nonfinite/identity/event/active-set/singularity/
stagnation/domain/capacity/component/soil/LSE/receipt/iteration refusals; exact
rollback; real DFF accepted/rejected counts, width histogram, solver
invocations/residual evaluations, wall time, limiting reasons, and maximum
mass/energy/vapor/soil/receipt residuals. Closure requires zero repeated-96
failures and a material reduction from the retained ~1435 accepted/~1500
rejected one-day blocker; the package records a provisional 4x reduction gate
for both counts, plus accepted-time fraction at `>=900 s` and stable-support
maximum width, without weakening any event tick or ledger tolerance.

The same contract-first sequence applies to `WGHL-FULL-001F` under
`SC-LANDSURFACEENERGY-001`: amend authority, author the no-update witness and
refusal tests, record the expected pre-implementation failure, then edit the
covered solver. No convergence threshold or phase-domain amendment is allowed.

`WGHL-FULL-001I` independently requires `SC-SURFACELIQUID-001` v15
`INV-SURFACELIQUID-022` and `SC-LANDSURFACEENERGY-001` v15
`INV-LANDSURFACEENERGY-150`, contract-derived tests, and a retained isolated
expected-red on unchanged production before any carry/schema implementation.
Blocking evidence includes the exact canonical WAT5 sub-ULP operand, signed and
tie-even/crossing/cancellation/order/subnormal/overflow vectors, V1 byte locks,
V1-to-V2 migration and downgrade poisons, typed receipt identity/cardinality/
order/substitution poisons, restart split equivalence, exact rollback, and real
WAT5 plus unchanged p61/native-forest successor consumers.

`WGHL-FULL-001G` remains outside this package's direct write set because it
crosses a distinct canonical phase/state/restart authority. This package must
not waive it as historical: it owns executing the named successor and consuming
its stable passing handback before workspace closure.

Conservation/output acceptance for `WGHL-FULL-001E` requires the prospective
operand-lineage table, anti-tautology fixtures, explicit rejection of adjacent
aliases and known wrong formulas, independent output reconstruction, real
per-OFE/adjacent-transfer/hillslope cancellation and WAT5 magnitude/closure
evidence, and metadata/schema alignment. Producer-only or self-consistency
evidence cannot close the real consumer claim.

The mechanical vegetation Clippy correction retains
`SC-VEGETATION-001#INV-VEGETATION-123/124/127` and
`SC-VEGETATIONTRANSACTION-001#INV-VEGTRANSACTION-009/010/013`; affected V11
restart/custody authority tests and exact A0 admission are mandatory.

## Exit criteria

- warnings-denied workspace Clippy exits zero on the exact clean terminal source;
- every test started by the full profile passes on that same source, with
  skipped inventory separately reconciled against profile configuration;
- no test assertion, timeout, fixture, authority binding, or failure policy is
  weakened to obtain the pass;
- all attributable failures have focused regression evidence;
- the terminal diff stays inside the amended prospective write set and every
  applicable instruction/line-count requirement is dispositioned;
- dual review and dual verification accept the direct gate evidence and final
  package status.

## Review, verification, and security

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to `comparator_suite_runner` subagents for baseline/current
full-workspace and Clippy execution, triage/worker subagents for bounded failure
families, two independent review subagents, and two independent verification
subagents. Comparator outputs are compact metrics, exact identities, failure
inventories, and retained log paths with read-only source access. Triage outputs
are bounded diagnoses or prospective inventory entries; worker write access is
limited to explicitly assigned files. Review and verification write access is
limited to the assigned package artifact. Standing user/session authorization
was supplied on 2026-08-30.

Security impact is `NONE` unless exact triage discovers a security/protected-data
surface; such a discovery requires explicit package amendment before edits.

## Gate non-deferral

The two mandatory failing commands are the package objective and cannot be
deferred, waived, or converted to a narrative handoff. Continue through safe
in-envelope repair while any classified correction remains available.
