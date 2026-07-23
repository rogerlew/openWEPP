# Behavior Equivalence

Static: the target path contains identity/control operations and no floating-point arithmetic. Whole guards were extracted with statement and short-circuit order unchanged.

- Required `base`, `head`, and `package` options are still read in that order before validation and persistence.
- Authorized paths, observed source, and package authority are still produced in that order; stage validation still precedes authority-field conversion.
- Authority JSON read, intent-package conversion, committed-head rejection, independent chain reconstruction, and byte-structural exact comparison retain their original order and error types/messages.
- Response JSON keys and values, confined persistence, `PlanRequest` fields, and exact reconstruction inputs are unchanged.

Ran: pre/post characterization and the complete 8-test binary suite pass unchanged. The committed-head negative case retains `GATE-PLAN-PACKAGE-AUTHORITY` and `package authority requires a committed head`.
