# Harness Closure Investigation

Evidence mode: **Ran + Static**.

## Finding

The eight-cell snowbench experiment completed, but both melt models lose the
same warm-mean, no-prior-pack snowfall event in every lane. The defect is not a
rounding tolerance and not an albedo effect.

| Lane | Date | Beginning SWE (m) | Snow input (m SWE) | Ending SWE (m) | Routed/lost/sublimated (m) | Residual (m) |
|---|---|---:|---:|---:|---:|---:|
| Mica Creek | 2018-11-23 | 0 | 0.037672727273 | 0 | 0 | 0.037672727273 |
| Niwot | 2005-10-09 | 0 | 0.045390000000 | 0 | 0 | 0.045390000000 |
| Paradise | 2001-11-22 | 0 | 0.059136000000 | 0 | 0 | 0.059136000000 |
| Snowbird | 2007-06-06 | 0 | 0.070800000000 | 0 | 0 | 0.070800000000 |

The exact residual repeats under `legacy_coe` and
`coe_shortwave_albedo_v1`. The forcing bridge contains positive hourly
snowfall on each date; the process returns zero accumulation and preserves the
zero SWE state.

## Mechanism

At frozen source HEAD `6be622ccfbef6bd563228c02d61095b8e05787c8`, the
public partition entry predicate activates snow coupling only when either
pre-existing runtime SWE is positive or the midpoint of daily maximum and
minimum temperature is below `0 °C`
(`runoff_reconciliation.rs:297-305`). It does not inspect positive typed hourly
snowfall.

When that predicate is false, the inactive outcome sets accumulation, melt,
loss, and routing to zero and returns the input SWE unchanged
(`runoff_reconciliation.rs:435-480`). The hourly active path would otherwise
sum snowfall and initialize a zero-depth pack
(`infiltration_reconciliation.rs:905-915,1313-1357`). The missing water in
snowbench is therefore explained by entry-path selection, not by the hourly
accumulator.

This diagnosis is bounded to the snowbench typed forcing. The retained direct-
production traces report zero typed snowfall and inactive coupling on these
four dates; their phase/input stream is not identical to the snowbench forcing
bridge. EB-04W2B must reconcile which phase representation is authoritative
before changing shared production activation behavior.

## Adjudication

The albedo comparison is withdrawn. Although the defect is identical in the
two model labels, removing an accumulation event changes all later pack state,
density, melt, and chronology. Common-mode input loss does not make the
counterfactual scientifically admissible.

The retained direct-production trace analysis is independently reconstructed
and remains admissible within its calibration-only role. W2A does not modify
the production predicate because Rust, contracts, and tests are protected by
its prospective write set.

## Hold-Lift Requirement

A successor must, before rerunning the contrast:

1. reconcile canonical phase/input authority for warm-mean days containing
   typed hourly snowfall across snowbench and direct production;
2. make positive typed snowfall enter an active, conserved snow path;
3. add a mixed rain/snow, zero-prior-pack regression and an explicit daily SWE
   closure guard;
4. prove the real snowbench consumer closes; and
5. rerun the unchanged W2A model pair and frozen hypothesis rule.
