# CQR15 Kernel Profile Compliance Checklist

Status: pending.

Static: CQR15 is kernel-affecting because runtime seeding controls hydrology
kernel inputs and branch activation. Behavior-preserving decomposition must not
change science-contract semantics.

Status: complete.

Static: kernel-profile checks:

- No science-contract authority text was changed.
- No runtime symbol name, alias, unit, or parser compatibility behavior was
  changed.
- No new provisional, surrogate, or heuristic process-physics math was added.
- No canonicalization threshold was introduced or changed.
- No `unsafe` block was introduced.
- No new dependency or fallback wrapper was introduced.
- Existing guard details and `HillslopeCliError::RuntimeSurfaceFailure`
  behavior were preserved for moved code.
- Float expression order for moved formulas was preserved by moving statements
  into private helpers without algebraic rewriting.

Ran: focused characterization and final workspace coverage pass both succeeded.
