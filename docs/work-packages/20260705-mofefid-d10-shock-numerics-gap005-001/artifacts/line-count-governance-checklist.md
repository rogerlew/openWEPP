# Line-Count Governance Checklist

Status: executed
Evidence mode: Static

Checklist:

- [x] List every touched `.rs` file and line count.
- [x] Record `WARN` disposition for files at or above 2000 lines.
- [x] Refactor or record approved exception for any non-exempt 3000+ line file.
- [x] Review artifacts check this explicitly.

Touched `.rs` files: none in the final diff.

Inspected-only line counts:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | 1002 | no final edit |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs` | 445 | no final edit |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs` | 445 | no final edit |
| `crates/openwepp-hillslope-orchestrator/examples/dval_case.rs` | 51 | no final edit |

No WARN/3000-line `.rs` exception is triggered.
