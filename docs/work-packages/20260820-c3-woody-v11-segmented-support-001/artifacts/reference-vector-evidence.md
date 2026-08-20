# Reference Vector Evidence

Status: authority candidate / Ran

`reference_calculator.py` imports no Rust or subprocess and independently
executes 8 migration and 14 chronology/resource cases. Direct run PASS. The
separately authored Rust contract test invokes it and checks all 22 results.
The population separates exact cadence roundtrip, one-nanosecond boundaries,
unequal order, gap/overlap, overbooking, double increment, rejection rollback,
scheduled replay, and restart replay. Full V10 physical compatibility vectors
remain an implementation-phase generated ledger backed by direct V10/V11
execution and independent field enumeration; Python does not reproduce V10
constitutive physics.
