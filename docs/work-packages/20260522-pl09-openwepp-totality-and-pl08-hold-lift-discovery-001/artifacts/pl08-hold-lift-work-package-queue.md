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

## Release Rule

- PL08 hold is not eligible for lift before `PL15` closure and Tier-A blocker
  clearance under policy.
