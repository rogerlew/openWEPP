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

## Release Rule

- PL08 hold is not eligible for lift before `PL15` closure and Tier-A blocker
  clearance under policy.
- After PL15 retained hold, lift remains ineligible until:
  1. Tier-A strict replay blockers are closed or explicitly risk-accepted, and
  2. critical kernel gaps (`KERNEL-GAP-001..004`) are closed or explicitly
     risk-accepted under recorded approval authority.
