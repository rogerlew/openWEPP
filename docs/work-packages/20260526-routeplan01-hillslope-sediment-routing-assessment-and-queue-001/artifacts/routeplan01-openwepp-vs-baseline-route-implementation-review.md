# ROUTEPLAN01 openWEPP vs baseline route implementation review

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Baseline authority target is `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Assessment focus is the audit-flagged row for `route.for` upper-end
  detach-vs-deposit routing behavior.

## Ran
- `nl -ba /workdir/wepp-forest_260430_baseline/src/contin.for | sed -n '1190,1248p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/route.for | sed -n '140,420p'`
- `sed -n '320,760p' /workdir/wepp-forest_260430_baseline/src/route.for`
- `sed -n '1,320p' /workdir/wepp-forest_260430_baseline/src/rtpart.for`
- `rg -n -i "rtpart" /workdir/wepp-forest_260430_baseline/src`
- `rg -n "\\bxcrit\\b|\\bdepc\\b|\\bdepend\\b|\\bdepos\\b|\\berod\\b|\\benrich\\b|mshear" crates/openwepp-hillslope-orchestrator/src -g '*.rs'`
- `nl -ba crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs | sed -n '5800,6660p'`
- `rg -n "0\\.005|1\\.0e-8|Vec::with_capacity\\(5 \\+ \\(class_count \\* 6\\)\\)|\\(1\\.\\.=4\\)|phi \\+ 2\\.0|phi \\+ 1\\.0" crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`

## Findings

### 1. Baseline `route.for` algorithm shape (authoritative)
- `contin.for` calls `route` in the event-erosion path (`call route` at line
  1218).
- `route.for` implements:
  - per-segment loop across `k = 2..nslpts(iplane)`,
  - flow-end bypass logic for case-4 style planes,
  - `xcrit` shear classification into `mshear` cases 1..5,
  - upper-end deposition branch (`du < 0`) with `depc/depend/depos`,
  - detachment-after-deposition sub-branches by `mshear`,
  - upper-end detachment branch (`du >= 0`) with separate `mshear` 1..5
    dispatch,
  - deposition follow-up when `ndep != 0`,
  - final OFE-end enrichment call.

### 2. openWEPP current shape
- `run_erod14_wave2` in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
  computes class-wise enrichment/load closure from scalar boundary surfaces and
  case-1..4 classification, then emits `erod14_*`/`EROD15` payload surfaces.
- It does not contain a segment loop over slope-point arrays, does not
  implement `mshear`-branch dispatch, and does not expose explicit runtime
  analogs of `xcrit`, `depc`, `depend`, `depos`, or route-local `erod` call
  sequencing.

### 3. Gap classification
| gap_id | gap | baseline evidence | openWEPP status | closure need |
|---|---|---|---|---|
| ROUTE-GAP-001 | Per-segment loop routing (`k=2..nslpts`) | `route.for` loop body | missing | implement segment-state topology + loop kernel path |
| ROUTE-GAP-002 | Upper-end deposition (`du<0`) branch with deposition-end solve | `route.for` L1/L2 deposition logic | missing | implement `depc/depend/depos`-equivalent path |
| ROUTE-GAP-003 | MSHEAR 1..5 branch dispatch and split `erod` call ranges | `route.for` computed-GOTO families (both deposition/detachment sections) | missing | implement branch dispatcher + typed guards |
| ROUTE-GAP-004 | Post-detachment deposition closure when `ndep != 0` | `route.for` post-branch deposition block | missing | implement follow-up closure path |
| ROUTE-GAP-005 | OFE-end enrichment finalization semantics | `route.for` final `call enrich(...,1.0,1.0,...)` | partial | map to explicit Wave-2/Wave-3 finalization contract |
| ROUTE-GAP-006 | Audit routine reference accuracy (`rtpart.for`) | `rtpart.for` root partitioning and `grow.for` usage | incorrect audit row coupling | correct provenance mapping in follow-on docs updates |

### 4. `rtpart.for` classification correction
- The audit row currently cites `route.for, rtpart.for` for sediment routing.
- Baseline inspection shows `rtpart.for` is root-mass partitioning, and usage
  is in growth paths (`grow.for`), not `CONTIN` erosion routing.
- Sediment-routing companions for `route.for` are `xcrit.for`, `depc.for`,
  `depend.for`, `depos.for`, `erod.for`, and `enrich.for`.

### 5. Sediment-routing magic-number inventory (current Rust)
| magic_id | location | literal | observed role | recommended normalization |
|---|---|---|---|---|
| MAGIC-ROUTE-001 | `03_kernel_support.rs` line ~6241 | `1.0e-8` | attenuation-factor floor in Wave-2 class update | promote to named constant in `constants.rs` with contract citation |
| MAGIC-ROUTE-002 | `03_kernel_support.rs` line ~6387 | `+ 0.005` | enrichment-ratio additive offset | promote to named constant (for example `EROD14_ENRICHMENT_OFFSET`) |
| MAGIC-ROUTE-003 | `03_kernel_support.rs` lines ~5985/5995 | `1..=4` | case-range literal for branch classification | replace with named case min/max constants |
| MAGIC-ROUTE-004 | `03_kernel_support.rs` lines ~6093/6400 | `5 + class_count * 6` | payload vector capacity literal | replace with named structural constants |
| MAGIC-ROUTE-005 | `03_kernel_support.rs` lines ~6220-6221 | `phi + 2.0`, `phi + 1.0` | denominator coefficients in class update formula | centralize as named coefficient constants or equation helper with provenance note |
| MAGIC-ROUTE-006 | baseline not yet ported | `abs(qostar) < .0011` (`route.for`) | near-zero inflow threshold controlling upper-bound deposition branch | introduce explicit constant during route migration; document in canonical contract |

## Conclusion
- The audit's `Partial` classification for route support is upheld.
- Current openWEPP implements Wave-2 class enrichment closure, but does not
  yet implement baseline `route.for` segment-level MSHEAR branch routing.
- A dedicated closure queue is required to migrate route physics and remove
  routing magic numbers under contract-first governance.
