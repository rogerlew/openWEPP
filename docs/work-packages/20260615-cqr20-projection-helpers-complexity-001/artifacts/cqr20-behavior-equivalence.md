# CQR20 Behavior Equivalence

Status: complete.

Static: production edits are limited to private helper extraction below
`project_annual_extension_controls`.

Static: preserved behavior:

- public API unchanged; no new `pub` items or exported type changes.
- stable error variants and IDs preserved:
  `HS-RUNTIME-E-042`, `HS-RUNTIME-E-043`, `HS-RUNTIME-E-046`,
  `HS-RUNTIME-E-047`, and `HS-RUNTIME-E-051`.
- annual extension expected/observed labels preserved:
  `herbicide`, `burn`, `silage`, `cut`, `remove`, and `none`.
- runtime projection symbols, aliases, units, parser compatibility, and formula
  behavior unchanged.
- validation call order for burn, cut, and remove branches remains day first
  and fraction fields in the original order.

Ran: focused characterization passed before production refactor:

```bash
cargo test -p openwepp-hillslope-orchestrator cqr20_project_annual_extension_controls
```

Ran: the same focused characterization passed after production refactor:

```bash
cargo test -p openwepp-hillslope-orchestrator cqr20_project_annual_extension_controls
```
