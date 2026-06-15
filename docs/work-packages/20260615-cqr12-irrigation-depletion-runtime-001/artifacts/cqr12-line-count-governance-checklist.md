# CQR12 Line-Count Governance Checklist

Status: complete.

Static: line counts after package edits:

| File | Before | After | Result |
| --- | ---: | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs` | 978 | 1042 | below 2000 |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs` | 1257 | 1260 | below 2000 |
| `tests/integration/irrig10_irrigation_runtime_kernel_contract.rs` | 586 | 1034 | below 2000 |
| `docs/work-packages/README.md` | 524 | 528 | documentation log |
| `package.md` | new | 213 | package-local |

Static: no touched `.rs` file is at or above `2000` lines. No touched
non-exempt `.rs` file is at or above `3000` lines.
