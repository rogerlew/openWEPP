# PERFIDX06 bottleneck analysis

Evidence: Ran + Static.

## What Dominates Now

The PERFIDX04 endpoint no longer has the pre-indexed read-side profile where dynamic
symbol lookup and formatting were the dominant named cost. The PERFIDX06 profile is spread
across:

- scheduler/runtime-surface lifecycle (`98.40%` children);
- indexed scheduler execution (`76.17%` children);
- Wb11 hydrology (`41.62%` children);
- runoff/frost coupling (`21.12%` and `20.58%` children);
- writeback (`17.03%` children);
- decomposition dispatch and overflow guards (`11.73%`, `8.15%`);
- residual formatting (`9.53%` children);
- `BTreeMap` insert/remove, `memcmp`, allocation/free, and symbol-table access.

The direct sample list is consistent with that: `__memcmp_sse2`, BTreeMap operations,
malloc/free, hot-table symbol resolution, and state-access helpers all show up near the top.

## PERFIDX05 Constraint

Static: PERFIDX05 tested the tempting next write/guard id migration and was held because it
regressed H2637 by about `5.3-5.8%`. The independent review reproduced bit identity and
concluded the dual-write design is the ceiling: writing both the logical map and the indexed
mirror costs more than the id-side saving.

That finding blocks the obvious incremental path. The current read-mirror design won on
reads, but it cannot get legacy-scale performance by migrating writes one helper at a time
while preserving the logical map as the authoritative hot-path surface.

## Reachability

At `73.12x` no-UI, closing the package target requires about another `7.3x` improvement to
reach `10x`. Reaching the stretch `5x` target requires about `14.6x`.

The current profile does not expose a remaining single lever of that size. Even removing
one named subtree entirely would not close the gap:

- writeback children are `17.03%`;
- decomposition dispatch is `11.73%`;
- residual formatting is `9.53%`;
- the direct decomposition overflow guard is `7.70%`;
- malloc/free direct samples are visible but not dominant enough alone.

The required improvement is architectural: remove symbol-keyed map work, allocation churn,
formatting, and dual publication from the hot path as a class, not just by shaving one
helper. A fixed-index or otherwise array-authoritative runtime state path is the likely
shape, but it must solve the PERFIDX03 export seam and not reintroduce the PERFIDX05
dual-write cost.

## Practical Ceiling Of Current Approach

PERFOPT01 + PERFIDX03B + PERFIDX04 moved H2637 from `978.55s` to `666.82s`, a meaningful
roughly `31.9%` reduction from PERFHO01. The remaining gap is still dominated by the
runtime-surface representation. Continuing read-side/id-table work may produce incremental
single-digit gains, but PERFIDX06 finds no evidence that the current representation can
reach `10x`, let alone `5x`, without a deeper hot-path state redesign.
