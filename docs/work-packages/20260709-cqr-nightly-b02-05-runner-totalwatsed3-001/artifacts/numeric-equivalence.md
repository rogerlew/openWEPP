# Output and Numeric Equivalence

Static: the refactor moved no numeric expression. `write_totalwatsed3` receives
the same `Totalwatsed3Config` fields in the same order after the same required
and optional input resolution. No unit conversion, formula, row construction,
or output schema changed.

Ran: public successful aggregate/per-hillslope cases assert totalwatsed3 output
values; characterisation also proves input precedence and exact hard failures.
The new test cases passed against scaffold `98243c6d` before decomposition.
