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
  crossing; the real `dff_ws2` fixture fails at `60 s`.
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

This package is amended to own two classified exceptions. First,
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
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_execution_tests.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/fixed_point.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_tests.rs`;
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

The same contract-first sequence applies to `WGHL-FULL-001F` under
`SC-LANDSURFACEENERGY-001`: amend authority, author the no-update witness and
refusal tests, record the expected pre-implementation failure, then edit the
covered solver. No convergence threshold or phase-domain amendment is allowed.

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
