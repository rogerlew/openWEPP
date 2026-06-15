# CQR16 Line-Count Governance Checklist

Status: complete.

Line counts:

| Path | Before | After | Status |
| --- | ---: | ---: | --- |
| `crates/openwepp-sim-contract/src/units_mod/registries.rs` | `954` | `1013` | Below 2000 advisory and 3000 hard cap |
| `tests/integration/sim_contract_boundary_unit_registry.rs` | Not captured before | `1120` | Test file, below 2000 advisory and 3000 hard cap |
| `docs/work-packages/README.md` | `570` | `574` | Documentation catalog |

Suppression census:

| Item | Before | After |
| --- | ---: | ---: |
| `#[allow(clippy::too_many_lines)]` in `registries.rs` | `2` | `1` |

Static: CQR16 removed the formatter suppression at the target function. The
remaining suppression is pre-existing on `validate_entry`, which is outside the
CQR16 target/helper closure scope.
