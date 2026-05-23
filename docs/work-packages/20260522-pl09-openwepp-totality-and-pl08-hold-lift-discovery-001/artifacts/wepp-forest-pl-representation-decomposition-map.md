# PL09 WEPP-Forest PL Representation Decomposition Map

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline PL representation/transition authority is the pinned
  `/workdir/wepp-forest_260430_baseline` code path.
- Mapping compares baseline representation semantics to currently implemented
  openWEPP ownership and runtime surfaces.

Ran:
- Decomposed baseline PL control flow and state surfaces across
  `tilage.for`, `contin.for`, `watbal.for`, `ptgra.for`, `ptgrp.for`,
  `grow.for`, `decomp.for`, and `resup.for`.

## Coverage Classes

- `mapped`: represented with direct openWEPP surface ownership.
- `partial`: represented only as scaffolding/seed/counts or with incomplete
  activation/event semantics.
- `missing`: no production representation of the baseline behavior.

## Baseline-to-openWEPP Representation Map

| baseline surface | baseline representation anchor | openWEPP representation | coverage | notes |
|---|---|---|---|---|
| Yearly schedule expansion | `tilage.for` reads `manndx` into `itype/imngmt/tilseq/conseq/drseq` | `pl_schedule_slot_*_{itype,imngmt,tilseq,conset,drset}` projection | `partial` | schedule continuity present, but naming drift (`conseq->conset`, `drseq->drset`) remains |
| Annual/perennial controls | `tilage.for` sets `jdplt/jdharv/rw/resmgt/mgtopt/ncut/ncycle` by branch | growth/decomp slot projections include these scalar controls | `partial` | scalar controls projected; event arrays/details not fully projected |
| Perennial event-day arrays | `tilage.for` sets `cutday`, `gday`, `gend`, and cycle payload fields | runtime projection currently emits `ncut`/`ncycle` counts only | `partial` | missing day-indexed event and cycle payload symbol families |
| Decomp-before-soil ordering | `contin.for` calls `decomp` before `soil`; `watbal.for` notes move for same-day management effect | order flags (`pl_order_decomp_before_soil`, `pl_order_growth_after_decomp`, `pl_order_watbal_after_growth`) and scheduler dependencies | `mapped` | ordering intent represented explicitly |
| PTGRA/PTGRP branch selection | `watbal.for` dispatches by `imngmt` to `ptgra` or `ptgrp` | growth dispatch selects annual/perennial by `imngmt` | `partial` | dispatch currently keyed to first slot/crop symbol, not active slot/day |
| Growth state evolution (`sumgdd`, `vdmt`, `lai`, `rtmass`, `rtd`, `hia`) | `grow.for` computes daily growth/senescence and calls `resup` | alias continuity exists for many symbols; no production growth kernel implementation | `missing` | interface/scaffold only |
| Decomposition kinetics and residue/root pools (`senvin/fenvin/benvin`, `rmagt/rmogt/rilrm/rigrm/smrm/rtm`) | `decomp.for` updates pool dynamics and tillage/residue transforms | decomposition scheduler phases exist; no production kinetics implementation | `missing` | process semantics not executed in production |
| RESUP transition semantics (`isenes` modes, residue slot shifting, root transfer) | `resup.for` handles senescence/harvest/kill transitions and slot shifts | no production `resup` equivalent in openWEPP kernel path | `missing` | critical transition behavior absent |

## Dimensional/Index Representation Notes

| concept | baseline shape | openWEPP shape | status |
|---|---|---|---|
| OFE seed surfaces | `(ofe)` arrays | `pl_*_ofe{index}_*` symbols | `mapped` |
| Rotation slot/crop schedule | `(slot,crop)` arrays | `pl_schedule_slot_{slot}_crop_{crop}_*` | `mapped` |
| Growth/decomp branch controls | `(slot,crop)` arrays | `pl_growth_slot_*`, `pl_decomp_slot_*` | `mapped` |
| Perennial event arrays (`cutday`, `gday`, `gend`) | `(event,slot,crop)` arrays | not projected (only counts) | `missing` |
| Annual extension events (`jdherb/jdburn/jdslge/jdcut/jdmove`, fractions) | branch extension structs | not projected into runtime surface | `missing` |

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/tilage.for:230`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:268`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:397`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:442`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:811`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:883`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:894`
- `/workdir/wepp-forest_260430_baseline/src/ptgra.for:257`
- `/workdir/wepp-forest_260430_baseline/src/ptgrp.for:341`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:284`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:693`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:538`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:580`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:829`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:201`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:255`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:371`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:968`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:983`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1086`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:996`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:532`
