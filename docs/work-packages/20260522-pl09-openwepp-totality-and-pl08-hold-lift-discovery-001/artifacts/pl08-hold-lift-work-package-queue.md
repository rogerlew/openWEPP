# PL09 PL08 Hold-Lift Work-Package Queue

Status: `complete`
Evidence mode: `Static`

Static:
- Queue is dependency-ordered from PL09 gap register blockers and baseline
  representation decomposition.
- Queue execution is gated on PL09A pre-execution clearance for preconditions
  1/2/3 from `claude-pl09-pre-execution-review.md`.

## Proposed Combined Plant + Water-Balance Queue

| wp_id | lane | objective | depends_on | acceptance criteria | required evidence |
|---|---|---|---|---|---|
| `PL10-active-slot-authority` | `plant` | Replace first-slot dispatch constants with day-aware active slot/crop resolution per OFE and schedule slot state. | `PL09`, `PL09A` | scheduler dispatch no longer hard-codes `slot_0001/crop_0001`; multi-slot activation tests pass; typed ambiguity/empty-slot errors added | unit/integration tests demonstrating branch selection across rotation year boundaries |
| `PL10b-contract-blind-authority-and-conformance` | `governance+plant` | Author transition-control contract authority blind to openWEPP implementation, derive contract tests, run conformance against implementation, and reconcile gaps before PL11 execution. | `PL10` | canonical contract authority amended with algorithm detail; contract-derived tests authored and executed; gaps classified (`contract`, `implementation`, `authority`) with explicit dispositions | blind-authoring attestation, contract-test spec, execution evidence, and gap-reconciliation matrix |
| `PL11-pl-event-runtime-projection` | `plant` | Expand PL runtime projection to include annual extension controls and perennial event-day/cycle payload arrays (not just counts). | `PL10b` | runtime projection emits indexed symbols for cut/grazing/event controls with deterministic naming and bounds checks; typed errors extended; PL10b ignored conformance gates pass | fixture-backed projection tests including annual extension branches and perennial cycles; explicit pass of `pl10b_contract_conformance_*` ignored tests |
| `PL12-decomp-resup-transition-kernel` | `plant` | Implement production decomposition/residue transition execution against typed contexts and projected event controls. | `PL11` | decomposition/residue phases update required pool/state symbols with contract checks and typed failures; no placeholder no-op path | targeted kernel tests, invariants, and residue trajectory checks |
| `PL13-growth-transition-kernel` | `plant` | Implement production annual/perennial growth transition execution with senescence/harvest transition signaling. | `PL12` | annual/perennial growth transitions execute with day-window logic and state updates (`sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`, `rtd`, `hia`) | kernel transition tests plus parser/runtime/scheduler integration coverage for both branches |
| `PL13A-alias-continuity-closure` | `governance` | Close or explicitly disposition canonical symbol continuity for projected PL runtime naming (`GAP-007`) before hold-lift closeout. | `PL11` | either alias continuity gaps are closed in canonical tables/contracts, or a scoped exception is formally approved and recorded | alias table diff + contract update evidence, or approval artifact reference with rationale and owner |
| `WB10-hydrology-phase-kernel-skeleton` | `water-balance` | Add production hydrology kernel entry scaffolding for ET/perc/lateral/drainage/runoff/storage phase classes (non-probe implementation path). | `PL09`, `PL09A` | non-test production kernel path exists and is wired through scheduler phase-class dispatch | compile/test evidence for production kernel wiring and typed phase routing |
| `WB11-et-perc-lateral-drain-kernels` | `water-balance` | Implement ET, percolation/deep seepage, lateral transfer, and drainage phase kernels with typed invariant checks. | `WB10`, `PL13` | deterministic phase execution updates required state/flux symbols; contract checks enforce finite/bounds constraints | kernel unit/integration tests plus closure/invariant evidence (`SC-WATBAL-001` aligned) |
| `WB12-runoff-storage-reconciliation-kernels` | `water-balance` | Implement runoff reconciliation and storage reconciliation kernels with explicit closure diagnostics integration. | `WB11` | runoff/storage reconciliation phases produce typed statuses and closure checks without placeholder responses | integration tests proving closure-surface correctness and typed failure propagation |
| `WB13-daily-water-balance-output-surface` | `water-balance` | Emit comparator-ready daily water-balance output surface (`H5.wat.dat` equivalent contract surface) from openWEPP run path. | `WB12`, `PL13` | reproducible candidate daily output generated for the Tier-A fixture with documented schema/units ordering | run manifest, file checksums, output schema/field mapping, persisted candidate files |
| `INT10-plant-water-coupling-validation` | `integration` | Validate coupled daily execution ordering and state coupling (`decomp -> growth -> watbal`) across plant and hydrology phases. | `PL13`, `WB13` | integration suite proves ordering flags and coupled state-transfer semantics under fixture replay | coupled replay tests and ordering/state trace evidence |
| `PL14-tier-a-candidate-emission-and-replay` | `closeout` | Execute strict Tier-A comparator using direct openWEPP candidate output vs pinned legacy baseline. | `INT10`, `PL13A` | strict comparator replay completes for Tier-A lane with reproducible provenance | comparator JSON artifacts, command trace, provenance hashes |
| `PL15-tier-a-delta-closeout-and-hold-lift` | `closeout` | Disposition residual Tier-A deltas and issue PL08 hold-lift verdict. | `PL14` | blocker set empty or formally risk-accepted under policy; decision record updated with explicit risk-acceptance approval reference when applicable | updated comparator disposition, semantic parity assessment, PL08 hold-lift decision artifact, and risk-acceptance approval artifact reference (if used) |

## Dependency Edges (Condensed)

1. `PL10 -> PL10b -> PL11 -> PL12 -> PL13`
2. `WB10 -> WB11 -> WB12 -> WB13`
3. `PL13 -> WB11` (water-balance kernels consume growth-updated state surfaces)
4. `PL11 -> PL13A -> PL14`
5. `PL13 + WB13 -> INT10`
6. `INT10 -> PL14 -> PL15`
7. `PL09A -> PL10` and `PL09A -> WB10` (pre-execution clearance gate)
8. `PL13A` may execute in parallel with `PL13` once both prerequisites are met
   (`PL11` complete for `PL13A`, `PL12` complete for `PL13`).
9. `WB14 + PL16 + PL17 + CLIM05 + CLIM06 + WB15 + IRRIG10 + WB16 + WS10 + ARCH22 + CLIM07 -> PL14R -> PL15R` (formal post-closeout recheck loop).

## Ordering Rationale

1. Plant lane (`PL10..PL13`) closes known PL representation and transition
   blockers before final Tier-A replay.
2. `PL10b` enforces contract-first blind authority and conformance testing so
   PL11 implementation is driven by ratified algorithm intent rather than
   implementation-backfit.
3. Water-balance lane (`WB10..WB13`) is explicit and separate so hydrology
   kernels are implemented as production code rather than probe placeholders.
4. `WB11` depends on `PL13` to ensure hydrology integration tests evaluate
   coupled post-growth state semantics.
5. `PL13A` enforces explicit naming-continuity governance closure (or formal
   scoped exception) before comparator closeout.
6. `INT10` is the cross-lane gate that verifies coupled execution ordering
   before comparator closeout.
7. `PL14` and `PL15` remain the authoritative hold-lift closure stages.
8. PL10/WB10/PL10b execution must preserve ARCH15/ARCH21 CRF-001/CRF-002 typed-seam
   closure posture as a non-regression constraint.
9. `PL13` and `PL13A` are intentionally parallelizable with disjoint ownership:
   runtime kernel implementation (`PL13`) vs alias continuity governance lane
   (`PL13A`).
10. `PL14R` and `PL15R` are reserved for evidence-driven recheck after the
    post-PL15 closure wave so hold-lift supersession, if any, is tied to fresh
    strict Tier-A replay artifacts.

## PL15 Post-Closeout Physics Gap Addendum (2026-05-23 UTC)

Disposition source:
- `docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/claude-pl15-pre-closeout-physics-review.md`
- `docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`

Purpose:
- Convert `KERNEL-GAP-001..012` into actionable follow-on queue rows with
  explicit acceptance evidence.

| wp_id | lane | gap ids | objective | depends_on | acceptance criteria | required evidence |
|---|---|---|---|---|---|---|
| `WB14-infiltration-and-subdaily-hyetograph-kernel` | `water-balance` | `KERNEL-GAP-001`, `KERNEL-GAP-004` | Implement production infiltration kernel (Green-Ampt lineage authority) and within-day hyetograph integration, replacing externally-seeded infiltration bookkeeping posture. | `PL15` | infiltration is computed by openWEPP kernel (not fixture-seeded), sub-daily forcing is consumed by hydrology execution loop, and strict replay provenance documents infiltration-source lineage. | contract amendments (`SC-RUNOFFPART-001`, `SC-WATBAL-001`), kernel tests, replay traces proving computed infiltration path |
| `PL16-growth-physics-kernelization` | `plant` | `KERNEL-GAP-002` | Replace PL13 growth plumbing-only path with production growth equations (GDD, biomass, canopy, phenology, senescence/harvest dynamics). | `PL15` | growth transition updates are equation-driven and no default skip/zero-reset fallback remains for active growth branches. | growth contract-test vectors, state-trajectory evidence, regression parity traces |
| `PL17-decomposition-physics-kernelization` | `plant` | `KERNEL-GAP-003` | Replace PL12 decomposition plumbing-only path with production residue/decomposition kinetics and transition pool transfers. | `PL15` | decomposition outputs are equation-driven and transition payloads drive real residue/pool updates under typed guards. | decomposition contract updates, kinetic tests, residue trajectory evidence |
| `CLIM05-snow-runtime-kernel-port` | `climate+hydrology` | `KERNEL-GAP-005` | Implement runtime snow accumulation/melt kernel coupling from parsed snow controls into hydrology boundary surfaces. | `WB14` | snow forcing is no longer orphan parser output; snow accumulation/melt affects water-balance terms with typed invariants. | snow contract/test vectors, fixture replay with snow scenarios |
| `CLIM06-frost-frozen-soil-kernel-port` | `climate+hydrology` | `KERNEL-GAP-006` | Implement frozen-soil/frost runtime kernel and infiltration coupling effects under typed failure semantics. | `CLIM05`, `WB14` | frozen-soil state surfaces drive infiltration/runoff branch behavior in runtime execution. | frost contract/test vectors, cold-season replay evidence |
| `WB15-canopy-interception-kernel-coupling` | `water-balance+plant` | `KERNEL-GAP-007` | Implement canopy interception kernel consuming plant state (`lai`, `cancov`, biomass context) before soil-water accounting. | `PL16`, `WB14` | interception is computed in production path and explicitly coupled into runoff/infiltration/watbal closure semantics. | interception contract updates, integration tests, daily closure evidence |
| `IRRIG10-irrigation-runtime-kernel-port` | `irrigation` | `KERNEL-GAP-008` | Implement irrigation runtime kernels consuming parsed depletion/fixed-date surfaces with typed scheduling and hydrology coupling. | `WB14` | irrigation parsers are no longer orphan surfaces; irrigation events alter runtime water-balance/forcing surfaces deterministically. | irrigation contract/test vectors, replay evidence for irrigated fixtures |
| `WB16-peak-runoff-kernel` | `water-balance+routing` | `KERNEL-GAP-009` | Implement peak runoff calculation required for downstream sediment/routing coupling. | `WB14`, `WB15` | peak runoff outputs are produced in canonical runtime path with documented method branches and typed guards. | peak-flow contract authority, kernel tests, trace outputs |
| `EROD10-sediment-kernelization-intake` | `erosion` | `KERNEL-GAP-010` | Convert acknowledged erosion-kernel deferral into an executable intake/phase plan with gated package wave ownership. | `WB16` | sediment kernelization roadmap is ratified with explicit package IDs, ownership, and acceptance gates. | intake decision artifact, dependency graph, contract-authority mapping |
| `WS10-channel-impoundment-production-kernels` | `watershed` | `KERNEL-GAP-011` | Replace watershed test/probe kernel posture with production channel/impoundment kernels and typed boundary integration. | `WB16` | at least one production `impl WatershedKernel` path exists for channel/impoundment execution under typed guards. | watershed kernel tests, routing/impoundment contract evidence |
| `ARCH22-typed-state-surface-closure` | `architecture` | `KERNEL-GAP-012` | Close CRF-001 carry-forward by migrating stringly `BoundarySymbol(String)` kernel surfaces to typed state surfaces. | `PL16`, `PL17`, `WB14` | runtime kernel interfaces no longer rely on stringly symbol keys for production state surfaces. | typed-surface contract updates, migration proof tests, ARCH closure artifact |
| `PL14R-tier-a-candidate-emission-and-replay-rerun` | `closeout-recheck` | `Tier-A replay blockers` | Re-run strict Tier-A comparator using direct openWEPP candidate output after post-PL15 kernel-closure wave completion. | `WB14`, `PL16`, `PL17`, `CLIM05`, `CLIM06`, `WB15`, `IRRIG10`, `WB16`, `WS10`, `ARCH22`, `CLIM07` | strict Tier-A replay executes with reproducible provenance and both required include surfaces (`H5.wat.dat`, `H5.plot.dat`) present in candidate-vs-baseline comparison artifacts. | refreshed comparator JSON artifacts, replay command trace, provenance hashes, updated Tier-A delta report |
| `PL15R-tier-a-delta-recloseout-and-hold-lift-rerun` | `closeout-recheck` | `Tier-A replay blockers` | Re-disposition residual Tier-A deltas from PL14R and issue refreshed PL08 hold-lift verdict with explicit risk-acceptance references when required. | `PL14R` | blocker set is empty or explicitly risk-accepted under policy, and PL08 hold-lift decision record is superseded with explicit criteria outcomes and approval references. | updated comparator confidence-tier disposition, updated semantic parity direction assessment, refreshed PL08 hold-lift decision artifact, risk-acceptance approval reference artifact (if used) |

## PL15R Reversal and Physics-Parity Recovery Addendum (2026-05-23 UTC)

Disposition source:
- `docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r-pl08-hold-lift-decision-record.md` (reversal update)
- `docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r_disposition.md` (reversal update)

Purpose:
- Revert PL15R hold-lift supersession posture.
- Define the required executable path to real openWEPP-vs-legacy Tier-A parity
  evidence for any future PL08 hold-lift claim.

| wp_id | lane | objective | depends_on | acceptance criteria | required evidence |
|---|---|---|---|---|---|
| `CLI01-open-wepp-runner-and-hillslope-driver-bootstrap` | `runtime-foundation` | Implement in-repo `open_wepp_runner` and executable hillslope run driver (`openwepp-cli-hill`) that produce comparator-ready outputs from openWEPP runtime execution (not legacy lane substitution), with blind run-directory sidecar discovery and typed strict/compat adapter behavior. | `PL15R` | in-repo runner binary exists; `[[bin]]` hillslope target exists; deterministic fixture run produces candidate `H5.wat.dat` and `H5.plot.dat`; run provenance manifest includes binary SHA, command line, sidecar resolution posture, and output checksums; release metadata sidecars are schema-valid. | cargo metadata showing runner + hillslope binary targets, fixture run trace, sidecar-resolution evidence, run-manifest sample, candidate output checksums, release-sidecar validation evidence |
| `WB17-et-physics-equivalence-port` | `water-balance-physics` | Replace WB11 ET demand-consumption surrogate with legacy-equivalent ET physics authority (including plant/soil/residue partition semantics). | `CLI01` | ET path no longer reduces to `min(soil_water, et_demand)` surrogate; ET partition variables are equation-driven and contract-vectored. | contract amendments (`SC-EVAP-001`, `SC-WATBAL-001`), equation vectors, fixture trajectory parity traces |
| `WB18-percolation-physics-equivalence-port` | `water-balance-physics` | Replace WB11 scalar excess*fraction percolation surrogate with layer-aware percolation authority and conductivity-domain behavior. | `WB17` | percolation path no longer uses `excess * perc_fraction` surrogate as production authority; per-layer transport state/flux surfaces execute with typed guards. | contract amendments (`SC-PERC-001`, `SC-WATBAL-001`), per-layer vectors, parity traces |
| `WB19-lateral-drainage-physics-equivalence-port` | `water-balance-physics` | Replace WB11 fraction-only lateral/drainage surrogates with legacy-equivalent subsurface/drain physics authority. | `WB18` | lateral/drainage path no longer relies on scalar fraction split as production authority; drain/lateral terms are equation-driven with typed guards. | contract amendments (`SC-SUBHYD-001`, `SC-WATBAL-001`), hydraulic vectors, parity traces |
| `WB20-forward-water-balance-solver-lane` | `water-balance-parity` | Establish parity comparator lane that is forward-solved by openWEPP kernels and does not consume observed closure targets as acceptance inputs. | `WB14`, `WB15`, `CLIM05`, `CLIM06`, `IRRIG10`, `WB17`, `WB18`, `WB19` | Tier-A parity lane runtime inputs exclude `wb12_runoff_observed` and `wb12_storage_observed` as acceptance-driving closure targets; closure is solver-output-derived. | lane input manifest, contract/test evidence proving no observed-target substitution, forward-solver replay traces |
| `PL14S-tier-a-openwepp-candidate-emission-and-replay` | `closeout-parity` | Run strict Tier-A comparator using openWEPP-emitted candidate outputs from CLI01/WB20 lane only. | `WB20`, `PL16`, `PL17`, `CLIM07` | strict replay executes with required include surfaces present from openWEPP lane; no legacy candidate substitution or schema-fallback upcast in candidate lane. | comparator JSON artifacts, command trace, binary/tool/output hashes, candidate provenance attestation |
| `PL15S-tier-a-final-hold-lift-closeout` | `closeout-parity` | Re-disposition PL08 Tier-A deltas from PL14S and issue final hold-lift verdict. | `PL14S` | hold-lift permitted only when active Tier-A blockers are closed on openWEPP-vs-legacy evidence, or formally risk-accepted under explicit approval authority. | updated confidence-tier disposition, semantic parity assessment, final PL08 decision record, risk-acceptance artifact (if used) |
| `WS11-channel-routing-physics-equivalence-port` | `watershed-physics` | Replace WS10 channel gain-factor surrogate with legacy-equivalent routing physics authority for production claims. | `WB16` | channel routing is no longer governed by `(1+slope)/(1+roughness)` surrogate as production parity claim basis. | contract amendments (`SC-ROUTE-001`, `SC-HYDRAULICS-001`), routing vectors, parity traces |
| `WS12-impoundment-physics-equivalence-port` | `watershed-physics` | Replace WS10 impoundment algebraic retention surrogate with legacy-equivalent impoundment hydraulics authority for production claims. | `WS11` | impoundment routing is no longer governed by simple headroom ratio surrogate as production parity claim basis. | contract amendments (`SC-IMPOUND-001`, `SC-HYDRAULICS-001`), impoundment vectors, parity traces |

## CLI02/CLI03 Runner Realignment Addendum (2026-05-24 UTC)

Disposition source:
- user-directed runner scope realignment after CLI01 bootstrap review

Purpose:
- authorize removal of CLI01 bootstrap-synthesizer output posture from
  production acceptance criteria;
- split execution into:
  - CLI02 planning/governance authority closeout, and
  - CLI03 implementation execution under contract-first sequencing.

| wp_id | lane | objective | depends_on | acceptance criteria | required evidence |
|---|---|---|---|---|---|
| `CLI02-hillslope-simulation-and-interchange-emission` | `runtime-foundation` | Planning/governance package that ratifies schema-versioned hillslope `.run` contract simplification (required `pass`/`loss`, optional parquet outputs), metric-only discoverability, and legacy-sidecar precedence semantics, then hands off implementation scope. | `CLI01`, `WB20` | canonical runner/CLI authority is updated and traceable; implementation tasks are explicitly deferred with prepared CLI03 execution package. | contract/spec amendment evidence, authority/guard mapping, handoff/disposition artifacts, queue/index updates |
| `CLI03-hillslope-runner-interchange-implementation` | `runtime-foundation` | Implement runner/CLI behavior from ratified CLI02 authority: `.run` validation, metric-only enforcement, legacy sidecar discovery precedence, required pass/loss outputs, optional parquet outputs, and manifest checksum behavior; move output contracts/serializers/tests into dedicated crate `crates/openwepp-hillslope-output/`. | `CLI02`, `WB20` | production run path enforces CLI03 contract surfaces with deterministic fixture evidence and required validation gates; output logic is crate-organized (not embedded in runner orchestration); no bootstrap-synthesized acceptance semantics. | contract-derived tests, pre-implementation gate evidence, output-crate organization evidence, implementation/test evidence, fixture output/manifest evidence |

## Release Rule

- PL08 hold remains `RETAIN HOLD` after PL15R reversal.
- PL08 hold-lift is ineligible until `PL14S -> PL15S` completes on provenance-
  valid openWEPP-vs-legacy Tier-A evidence.
- Provenance-valid Tier-A evidence requires:
  1. candidate lane outputs emitted by openWEPP executable(s) with recorded
     binary SHA and command trace;
  2. no legacy candidate-lane substitution; and
  3. no schema-only fallback/upcast used to satisfy required include surfaces.
- Physics-valid Tier-A evidence requires closure of WB physics parity packages:
  `WB17`, `WB18`, `WB19`, and `WB20` before PL14S execution.
- `CLI02` is planning authority only and does not, by itself, alter PL14S
  include-surface requirements for final hold-lift closeout evidence.
- `CLI03` is the implementation lane for runner/CLI execution semantics and
  likewise does not supersede PL14S/PL15S release-rule requirements.

## EROD Queue Reassessment Addendum (2026-05-23 UTC)

Disposition source:
- `docs/work-packages/20260523-erod10-sediment-kernelization-intake-001/artifacts/erod10-wave-execution-plan.md`
- `docs/work-packages/20260523-erod11-alias-and-boundary-ownership-closure-001/artifacts/erod11-wave0-gate-verdict.md`
- `docs/work-packages/20260523-erod12-cross-domain-contract-closure-001/artifacts/erod12-wave0-release-verdict.md`
- `docs/work-packages/20260523-erod12-cross-domain-contract-closure-001/artifacts/erod12_disposition.md`
- package status headers under `docs/work-packages/20260523-*/package.md`

Purpose:
- Reassess erosion-lane sequencing after confirmed execution through `WB20`
  and completion of `EROD12`.

### Prerequisite Snapshot for Erosion Wave Entry

| dependency package | state | relevance to EROD wave entry |
|---|---|---|
| `WB14` | `completed` | EROD13 intake dependency from EROD10 scope plan |
| `WB15` | `completed` | EROD13 intake dependency from EROD10 scope plan |
| `WB16` | `completed` | Required upstream hydrologic forcing for erosion lane |
| `WB17` | `completed` | PL15R parity lane prerequisite now closed |
| `WB18` | `completed` | PL15R parity lane prerequisite now closed |
| `WB19` | `completed` | PL15R parity lane prerequisite now closed |
| `WB20` | `completed` | Forward-solver parity lane prerequisite now closed |
| `WS10` | `completed` | Removes previously queued upstream gate for `EROD15` |
| `EROD10` | `completed` | Wave plan authority is ratified |
| `EROD11` | `completed` | Wave-0 alias ambiguity closure is complete (`GO`) |
| `EROD12` | `completed` | Wave-0 cross-domain closure is complete (`GO` for EROD13 entry) |

### Reassessed EROD Execution Queue

| wp_id | wave | current state | dependency posture | queue decision |
|---|---|---|---|---|
| `EROD13-hillslope-core-erosion-kernel-001` | Wave 1 | `completed` | Entry dependencies satisfied and Wave-1 contract-first execution closed (`GO`) | `DONE` |
| `EROD14-multiofe-and-enrichment-kernel-001` | Wave 2 | `completed` | Wave-2 contract-first execution closed (`GO`) | `DONE` |
| `EROD15-routing-boundary-coupling-001` | Wave 3 | `queued` (package scaffolded; execution not started) | EROD14 completion satisfied; `WS10` upstream dependency remains satisfied | `NEXT` |
| `EROD16-sediment-closeout-and-comparator-001` | Wave 4 | `not-started` | Blocked on `EROD15` completion | `QUEUE` |

### Hold/Promotability Posture

- `EROD12` authorizes **EROD13 package entry** only; erosion-kernel physics
  edits still require EROD13 contract-first sequencing and gates.
- Non-Wave-0 non-promotable holds remain active and unchanged:
  `GAP-ROUTE-005`, `GAP-RUNOFFPART-003`, `GAP-WATBAL-002`, `GAP-SYSTEM-001`.
- Queue reassessment does not supersede PL08 release-rule constraints for
  `PL14S -> PL15S`.

## EROD13 Scaffold Update (2026-05-25 UTC)

Status: `completed`
Evidence mode: `Mixed`

Static:
- `EROD13` package scaffold is now present at:
  `docs/work-packages/20260525-erod13-hillslope-core-erosion-kernel-001/`.
- Queue execution note: `EROD13` has been executed end-to-end with Wave-1
  `GO` verdict and required gates passing.

Ran:
- Wave-1 package gates completed:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

## EROD14 Completion Update (2026-05-25 UTC)

Status: `completed`
Evidence mode: `Mixed`

Static:
- `EROD14` execution is complete with Wave-2 `GO` verdict for EROD15 entry.

Ran:
- Wave-2 package gates completed:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

## EROD15 Scaffold Update (2026-05-25 UTC)

Status: `queued`
Evidence mode: `Static`

Static:
- `EROD15` package scaffold is now present at:
  `docs/work-packages/20260525-erod15-routing-boundary-coupling-001/`.
- Queue progression update: Wave-3 is promoted to `NEXT` after EROD14
  completion; EROD16 remains blocked on EROD15 completion.
