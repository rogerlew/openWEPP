# PL09 openWEPP Totality Implementation Inventory

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL-relevant totality inventory was compiled from openWEPP parser contracts,
  runtime projection seams, scheduler/boundary wiring, kernel-contract
  surfaces, symbol alias registry, comparator metadata, and PL05-PL08
  disposition artifacts.
- Inventory scope is PL08 hold relevance only; this package does not implement
  new process physics.

Ran:
- Completed source/disposition discovery over runtime/orchestrator/test modules
  and baseline anchors under `/workdir/wepp-forest_260430_baseline/src`.

## Totality Matrix

| surface family | status | evidence | PL08 hold relevance |
|---|---|---|---|
| `.man` typed parser datamodel | `implemented` | `YearlyAnnualFallowData`, `YearlyPerennialData`, `cut_days`, `grazing_cycles` are typed and parsed | upstream structure is available for projection/dispatch |
| `PL-MAN-SEAM-001` runtime projection | `implemented` | `build_hillslope_pl_runtime_surfaces_from_management` produces `pl_schedule_surface`, `pl_growth_surface`, `pl_decomp_surface` and merged state | required adapter seam exists |
| PL seam typed reject paths | `implemented` | `HS-RUNTIME-E-036..045` mapped and integration-tested | strict seam failures are explicit (no silent defaults) |
| Scheduler phase graph and ordering | `implemented` | 13-phase canonical graph includes decomposition -> residue partition -> annual growth -> perennial growth -> ET chain | baseline ordering intent is represented |
| Growth/decomposition dispatch authority | `partial` | dispatch requires `pl_growth_slot_0001_crop_0001_*` and `pl_decomp_slot_0001_crop_0001_*` symbols | active-slot/day authority not yet generalized |
| Growth/decomposition kernel boundary typing | `implemented (interface)` | typed phase class and context dispatch exist in scheduler/kernel contract | scaffolding exists, but process kernels remain missing |
| Production growth/decomp/resup process implementation | `missing` | `HillslopeKernel` implementations found only in tests/integration probes | no production process execution for PL transitions |
| Canonical symbol alias continuity (PL) | `partial` | core PL canonical aliases (`lanuse`, `itype`, `imngmt`, `jdplt`, `jdharv`, `resmgt`, `mgtopt`) are present | naming/coverage gaps remain for some projected symbols |
| Comparator confidence-tier routing metadata | `implemented` | deterministic Tier-A/Tier-B routing and metadata propagation tests exist | policy wiring is ready |
| Tier-A direct openWEPP-vs-legacy candidate output lane | `missing in current workspace evidence` | PL08 disposition records this surface as unavailable | direct hold blocker |

## Implemented Strengths

1. Strict parser-to-runtime PL projection is present and deterministic, with
   typed failures for topology mismatch, slot mismatch, bad references,
   unsupported landuse/options, and non-finite controls.
2. Scheduler phase ordering explicitly encodes baseline decomp->growth->watbal
   ordering preconditions and enforces required ordering flags.
3. Comparator confidence-tier metadata semantics (higher-confidence for
   single-OFE daily, investigation for hourly/watershed) are typed and tested.

## Partial or Missing Surfaces

1. Active branch selection is placeholder-scoped to first slot/crop symbols,
   not to day-aware/multi-slot schedule resolution.
2. Runtime projection includes perennial cardinalities (`ncut`, `ncycle`) but
   does not project cut/grazing event-day arrays and associated cycle payloads.
3. Runtime projection carries annual `resmgt` selector but not full annual
   extension event payloads (herbicide/burn/silage/cut/remove controls).
4. Growth/decomposition/residue process semantics are not implemented in a
   production hillslope kernel path.

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:163`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:196`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:202`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:677`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:702`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:763`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:789`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:952`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1086`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:33`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:58`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:532`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:610`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:572`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:303`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:334`
- `/home/workdir/openWEPP/crates/openwepp-comparator-metadata/src/lib.rs:11`
- `/home/workdir/openWEPP/crates/openwepp-comparator-metadata/src/lib.rs:45`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:530`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:571`
- `/home/workdir/openWEPP/tests/integration/comparator_tier_routing_metadata.rs:13`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/comparator-confidence-tier-disposition.md:20`
