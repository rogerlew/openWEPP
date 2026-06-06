# Radiation-Bound Attribution Ledger

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

Ran:

- Before-state failures reproduced for all six WBVAL02 hillslopes.
- Source ledger computed from the shared DRIGGS climate file:
  `/wc1/runs/in/indispensable-presenter/wepp/runs/p2.cli`.

Static:

- All six wrappers reference identical climate records.
- Named mechanism: invalid upstream daily radiation exceeds baseline `sunmap`
  horizontal daily potential before SIMIMPL28 hourly radiation synthesis.
- Correction class: typed source-bound evidence in active SIMIMPL28 runtime
  projection.

Source-bound evidence:

| Field | Value |
|---|---|
| Shared source file | `/wc1/runs/in/indispensable-presenter/wepp/runs/p2.cli` |
| First violating row | `18 2 1990 ... rad 486` |
| Day of year | `49` |
| Latitude | `43.73` |
| Source `radly` | `486 Ly d^-1` |
| Baseline `sunmap` horizontal potential `r3` | `453.068716 Ly d^-1` |
| Ratio | `1.072685` |
| Contract boundary | `SC-CLIMATE-001#INV-CLIMATE-006`, `SC-CLIMATE-001#INV-CLIMATE-013` |

Per-hillslope attribution:

| Hillslope | Before error | Before value | Mechanism | Seven-gate result | Disposition |
|---|---|---:|---|---|---|
| `p2` | `winter.hourly.rad_mj_m2_0012` | 4.908100451183912 | Shared invalid `radly=486` exceeds `r3=453.068716` | Reproduction, mechanism, authority, safety, testability, and validation true; production ownership limited to typed evidence because source generation is upstream. | invalid upstream input |
| `p4` | `winter.hourly.rad_mj_m2_0012` | 4.915834085837891 | Shared invalid `radly=486` exceeds `r3=453.068716` | Same as `p2`. | invalid upstream input |
| `p6` | `winter.hourly.rad_mj_m2_0012` | 4.833738717329369 | Shared invalid `radly=486` exceeds `r3=453.068716` | Same as `p2`. | invalid upstream input |
| `p9` | `winter.hourly.rad_mj_m2_0012` | 4.912717106932223 | Shared invalid `radly=486` exceeds `r3=453.068716` | Same as `p2`. | invalid upstream input |
| `p14` | `winter.hourly.rad_mj_m2_0012` | 4.829383053400764 | Shared invalid `radly=486` exceeds `r3=453.068716` | Same as `p2`. | invalid upstream input |
| `p17` | `winter.hourly.rad_mj_m2_0012` | 4.857151679545786 | Shared invalid `radly=486` exceeds `r3=453.068716` | Same as `p2`. | invalid upstream input |

Gate conclusion:

- `HOLD` is not used for WBVAL02 closure.
- A valid-radiation physics correction was not identified. The source climate
  value is outside the contract boundary and must fail closed with typed
  evidence.
