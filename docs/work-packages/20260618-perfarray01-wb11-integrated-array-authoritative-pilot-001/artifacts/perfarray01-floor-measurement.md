# PERFARRAY01 Floor Measurement

Evidence class: Not run.

## Result

No H2637 floor measurement was produced.

## Why

The package requires a real integrated WB11 pilot that satisfies both structural
proofs:

- no per-day full `BTreeMap` export at the kernel seam;
- no normal-path logical + array dual-write.

Static inspection showed the existing production seam cannot satisfy either
proof for WB11 runoff reconciliation without first introducing an
array-capable request/accessor/scheduler authority path. Timing the current
logical path, or a path that exports logical maps from `ArrayHotState` per
phase, would not measure the requested floor.

## Budget Context

PERFIDX06 remains the current floor evidence:

- openWEPP H2637 no-UI endpoint: 666.82 seconds;
- legacy H2637 no-UI median: 9.12 seconds;
- current ratio: 73.12x;
- <=10x budget: about 386 microseconds per OFE-day;
- <=5x budget: about 193 microseconds per OFE-day.

PERFARRAY01 does not change these numbers.

## Ratification Input

ADR-0023 should not be ratified from PERFARRAY01. Stage A supports the
contract-shell direction, but the real integrated floor remains unmeasured.
