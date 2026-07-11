# Numeric/behavior equivalence

Status: PASS
Evidence mode: Static and Ran

The correction changes typed classification only for invalid input recognized
by `INV-CHN-016`; it does not alter rating formulas or accepted typed data.
Nominal assertions cover every channel field, exact numeric comments, rating
values, optional frame projection, and disabled `None`. Post-safety-net helper
extraction preserves geometry→erosion→control→rating→effective-control order.
All 38 parser and 20 consumer tests pass.
