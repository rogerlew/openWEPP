# Numeric Equivalence

Static, rejected attempt: the diff moved fallible column/value reads into helpers without moving
their order. It does not change an arithmetic expression, aggregation loop,
floating-point grouping, default, schema field, unit, or error construction.

Ran: the unchanged public CLI suite passed `6/6`; its numeric assertions verify
Area, runvol, Runoff, Q, lateral flow, QOFE, Interception, sediment class
deposition, and sediment delivery within `1e-12` on combined and per-hillslope
fixtures. Bit-identical behavior is supported statically by the unchanged
arithmetic expressions, accumulation order, and value-read order; the executed
suite is tolerance evidence, not a byte-identity claim.

Final hold: the implementation diff is rolled back to scaffold `e2ff321e`, so
the accepted production/test tree is identical to the pre-package baseline.
