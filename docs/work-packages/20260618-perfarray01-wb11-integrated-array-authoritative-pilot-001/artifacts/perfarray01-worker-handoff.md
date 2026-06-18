# PERFARRAY01 Worker Handoff

Evidence class: Static.

## Current State

PERFARRAY01 landed Stage A in `openwepp-kernel-contract` and stopped before
Stage B timing. The valid WB11 array-authoritative execution path does not
exist yet.

## Next Action

Scaffold a narrower follow-on, proposed as `PERFARRAY02`:

**Array request/accessor authority split for WB11 runoff reconciliation.**

Minimum scope:

- add an array-capable request/view that can satisfy WB11 scalar reads without
  logical maps;
- port core state/flux accessors used by runoff reconciliation to that view;
- add a scheduler pilot path where `ArrayHotState` is the only mutable hot-path
  authority;
- keep publication materialization explicit and out of the per-phase kernel
  seam;
- then run H2637/OFE-ladder identity and timing.

## Do Not Do

- Do not benchmark a path that exports `ArrayHotState` to `BTreeMap` before
  every phase.
- Do not maintain logical maps and array slots as co-authorities in normal
  timing.
- Do not ratify ADR-0023 until the integrated floor is measured on a valid path.
