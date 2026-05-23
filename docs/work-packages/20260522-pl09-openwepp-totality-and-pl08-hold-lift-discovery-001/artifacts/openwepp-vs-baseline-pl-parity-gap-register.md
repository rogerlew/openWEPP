# PL09 openWEPP vs Baseline PL Parity Gap Register

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Gap classes below are scoped to PL08 hold-lift decision authority and
  confidence-tier policy semantics.

Ran:
- Gaps were identified by cross-reading openWEPP implementation surfaces,
  PL08 disposition evidence, and baseline PL representation anchors.

## Gap Register

| gap_id | gap statement | confidence-tier impact | severity | blocker class | evidence |
|---|---|---|---|---|---|
| `PL09-GAP-001` | Direct openWEPP-vs-legacy Tier-A candidate output surface is unavailable in current workspace evidence. | `Tier-A` | `critical` | `block` | PL08 disposition record marks this as explicit blocker |
| `PL09-GAP-002` | Strict Tier-A comparator still reports unresolved `H5.wat.dat` structure delta (`line_count_mismatch`, `numeric_arity_mismatch`). | `Tier-A` | `critical` | `block` | PL08 comparator delta/disposition artifacts |
| `PL09-GAP-003` | Growth/decomp activation authority is fixed to `slot_0001/crop_0001` symbols rather than active slot/day resolution. | `Tier-A relevance` | `high` | `block` | Scheduler dispatch constants and branch selection code |
| `PL09-GAP-004` | Perennial event-day arrays and cycle payloads (e.g., `cutday`, `gday`, `gend`, cycle payload fields) are parsed but not projected into runtime surfaces. | `Tier-A relevance` | `high` | `block` | Parser types include arrays; runtime projection emits only `ncut`/`ncycle` counts |
| `PL09-GAP-005` | Annual extension event payloads (`jdherb/jdburn/jdslge/jdcut/jdmove` and fractions) are parsed but not projected to PL runtime surfaces. | `Tier-A relevance` | `high` | `block` | parser branch extension types vs projection symbol set |
| `PL09-GAP-006` | Production growth/decomp/resup process execution is not implemented; kernel boundary is interface scaffolding with test probe implementations only. | `Tier-A` | `critical` | `block` | `HillslopeKernel` trait exists; `impl` search results are test/integration-local |
| `PL09-GAP-007` | Canonical alias continuity remains partial for projected PL symbols (e.g., schedule naming drift `conset/drset` vs legacy `conseq/drseq`; projected `ncut/ncycle` not in canonical registry). | `Tier-B/Tier-C` | `medium` | `investigate` | runtime projection + canonical registry comparison |
| `PL09-GAP-008` | Runtime projection rejects non-cropland landuse (`landuse != 1`) while baseline includes additional landuse branches. | `Tier-C` | `medium` | `investigate` | typed `UnsupportedPlLanduse` guard and baseline branch coverage |

## Blocker Set Keeping PL08 in HOLD

1. `PL09-GAP-001`: no direct Tier-A openWEPP candidate output.
2. `PL09-GAP-002`: unresolved strict Tier-A structure mismatch.
3. `PL09-GAP-003`: unresolved active-branch authority model.
4. `PL09-GAP-004` and `PL09-GAP-005`: missing event-level runtime projection
   needed for branch-faithful transition execution.
5. `PL09-GAP-006`: no production growth/decomposition/residue process kernel.

## Investigation-Only Gaps (Non-Blocking by Policy Today)

1. `PL09-GAP-007`: alias/table continuity refinements.
   This gap is release-coupled to hold-lift closeout and must be explicitly
   closed or formally exceptioned in `PL13A` / `PL15` governance evidence.
2. `PL09-GAP-008`: non-cropland extension path coverage outside current
   single-OFE cropland Tier-A fixture focus.

## Evidence Links

- `/home/workdir/openWEPP/docs/work-packages/20260520-arch01-subsystem-map-and-contract-spine/artifacts/comparator-confidence-tier-policy.md:14`
- `/home/workdir/openWEPP/docs/work-packages/20260520-arch01-subsystem-map-and-contract-spine/artifacts/comparator-confidence-tier-policy.md:48`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/comparator-confidence-tier-disposition.md:18`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl08-comparator-confidence-tier-review-001/artifacts/comparator-confidence-tier-disposition.md:20`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:33`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:532`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:610`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:789`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1086`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1090`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:170`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:196`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:202`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:572`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:322`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:334`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:380`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:397`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:416`
