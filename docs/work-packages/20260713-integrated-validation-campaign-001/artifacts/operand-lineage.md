# Operand Lineage

Status: `PARTIAL-PRE-FIX-BINDING`

Evidence class: **Ran + Static** test binding; not terminal closure evidence.

| Surface | Units/basis | Producer -> real consumer | Independent reconstruction | Rejected aliases |
| --- | --- | --- | --- | --- |
| H2637 precipitation/runoff/storage | m3 per lane/day | direct day frame -> active route/public WAT and manifest | source, routed outlet, cascade and identity residuals from distinct manifest ledgers | precipitation magnitude as runoff; compatibility shadow state |
| Groundwater | m3/day | deep-percolation recharge -> reservoir -> HBP `gwbfv/gwdsv` -> watershed | recharge/storage/baseflow/deep-seepage recurrence and external-source-once network ledger | `latqcc`, `cbase`, or upstream cumulative baseflow re-added locally |
| p61/p102 runoff | m3/event day | direct hydrology -> HBP `V_h` and pass `runvol` | `sum(V_h) = runvol` | peak discharge or area-normalized depth |
| p61/p102 sediment | kg/event day | Wave-1 per-OFE chain -> HBP `S_h` and pass/public rows | `sum(S_h) = total_detachment - total_deposition`; p61 also `sum(sedcon_k) * runvol = tdet` on zero-deposition day | zero-fill, unit-width seed, adjacent concentration, raw `tdet` as yield |
| Snow forcing/partition | mm/day and hourly fraction | production day-input precipitation/canopy -> winter state and WAT | forcing CSV precipitation and snow-fraction sums reconstruct independent audit totals | daily radiation as hourly flux; ground temperature as surface temperature; raw rain as melt |
| Snow/frost storage | mm SWE/liquid and m frost depth | winter column/direct frost state -> WAT snow/frozen surfaces and next-day carry | prior + input - melt/release = terminal storage over selected interval | physical depth as SWE; stale coarse frost projection; compatibility sidecar carry |
| Watershed interval water | m3/interval | HBP `V_h`, upstream `q1`, local baseflow -> dependency channel -> EBE/chanwb | external hourly sum = terminal outflow + storage; published balance = 0 | event peak, scalar daily runoff, dependency baseflow re-addition |
| Watershed interval sediment | kg/class/interval | HBP `S_h` and class blend -> upstream interval egress -> downstream same-grid inlet -> EBE | per-class ingress + detachment - deposition - egress = delta; network yield equals terminal egress | daily smearing, scalar event sediment, `tdet-tdep` as routed yield |

Commands 06, 07, 10, 11, and 13 passed the named p61, p102, W7R, W11B, and
watershed-hourly bindings with separate readers and arithmetic rather than
production aggregation helpers. Command 13, not H2637 command 02, binds the
explicit two-channel baseflow-external-once assertion. The campaign did not
archive complete H2637 groundwater or snow numeric operands and output hashes;
therefore this map is partial and cannot be reused by the post-fix campaign.
