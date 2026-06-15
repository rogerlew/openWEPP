# CQR15 Behavior Equivalence

Status: complete.

Static: production change is private helper extraction from
`seed_wb11_runtime_surface_inputs`.

Static: preserved behavior boundaries:

- Runtime symbols and aliases are unchanged.
- Lane policy for daily/hourly and multi-OFE carry arrays is unchanged.
- WB11 initial storage formulas preserve operand order:
  `((sat * por) * cpm)`, `(thetfc - thetdr) * dg`,
  `(por - thetdr) * dg`, `theta + thetdr * dg`.
- WB12 reconciliation seed symbols and scalar values are unchanged.
- ET demand seeding and WB16 ealpha compatibility calls remain in the same
  relative phase after WB12 seeding and before MOFE03 seeding.
- `HillslopeCliError::RuntimeSurfaceFailure` surface and guard detail strings
  are preserved for moved guard paths.

Ran: focused characterization before production refactor:

```bash
cargo test -p openwepp-runner publication_wb11_seed --lib
```

Result: `16 passed; 0 failed`.

Ran: focused characterization after production refactor and helper split:

```bash
cargo test -p openwepp-runner publication_wb11_seed --lib
```

Result: `16 passed; 0 failed`.

Ran: workspace after LCOV executed all workspace tests under coverage and
completed successfully.
