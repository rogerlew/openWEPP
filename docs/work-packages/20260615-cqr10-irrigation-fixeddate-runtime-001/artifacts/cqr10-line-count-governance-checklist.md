# CQR10 Line-Count Governance Checklist

Status: complete.

Static: line counts after refactor:

| File | Lines |
| --- | ---: |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs` | 1257 |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs` | 978 |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs` | 11 |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/irrigation_fixeddate.rs` | 230 |
| `docs/work-packages/README.md` | 505 |

Static: target-file suppression census after refactor:

- line 51: `#[allow(clippy::too_many_lines)]` on depletion projection,
  out-of-scope for CQR10.
- line 801: `#[allow(clippy::too_many_lines)]` on frost projection,
  out-of-scope for CQR10.

Closure:

- PASS: no touched `.rs` file is at or above `2000` lines.
- PASS: no touched non-exempt `.rs` file is at or above `3000` lines.
- PASS: scoped fixed-date `too_many_lines` suppression was removed.
