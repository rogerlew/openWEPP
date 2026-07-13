# Medium Raw-To-Actionable Ledger

Evidence class: **Ran + Static**

Source commit: `ec2f197e1f3248d14fb1232dc1ebb8766d5dc6b5`.
Fresh production census: 32 rows / 25 modules. Fixed Medium cohort: 19 raw
rows / 13 modules. Actionable cohort: 19 rows / 13 modules.

The plan's discovery count was stale: High-B removed
`GwcoeffParseError::fmt`, so M-02 has one live row. Review A proposed
`R-OBSERVABILITY` for three formatter rows; Review B rejected those exceptions.
The binding disagreement rule defaults all three to `E-PRODUCTION`. No `R-*`
or `X-*` disposition is retained.

| ID | Function | Line | CC | Coverage % | CRAP | Class | Eligibility reason |
| --- | --- | ---: | ---: | ---: | ---: | --- | --- |
| M-01 | `parse_climate_from_str` | 354 | 41 | 74.074 | 70.293 | `E-PRODUCTION` | grammar, flags, metadata, record order |
| M-01 | `parse_no_breakpoint_day` | 574 | 21 | 69.231 | 33.847 | `E-PRODUCTION` | daily cardinality and range guards |
| M-01 | `parse_breakpoint_day` | 644 | 31 | 81.373 | 37.211 | `E-PRODUCTION` | breakpoint order, cardinality, compatibility |
| M-02 | `enforce_result_invariants` | 432 | 16 | 53.333 | 42.017 | `E-SCIENCE` | groundwater numerical/domain invariants |
| M-03 | `SnowParseError::fmt` | 156 | 10 | 0.000 | 110.000 | `E-PRODUCTION` | stable public `SNOW-E-*` error text |
| M-03 | `enforce_invariants` | 445 | 10 | 39.394 | 32.261 | `E-SCIENCE` | snow numerical/domain invariants |
| M-04 | `HbpFormatErrorCode::as_str` | 25 | 15 | 35.294 | 75.956 | `E-PRODUCTION` | machine-readable contract error identity |
| M-05 | `ManagementYamlError::fmt` | 415 | 9 | 23.077 | 45.868 | `E-PRODUCTION` | stable public `MAN-YAML-E-*` error text |
| M-05 | `validate_management_yaml_document` | 498 | 24 | 73.333 | 34.923 | `E-PRODUCTION` | schema and validation priority |
| M-05 | `validate_schedule` | 788 | 20 | 70.000 | 30.800 | `E-PRODUCTION` | schedule order/cardinality/domain |
| M-06 | `validate_entry` | 783 | 19 | 50.617 | 62.474 | `E-SCIENCE` | canonical unit/dimension registry admission |
| M-07 | `parse_runfile_execution_config` | 11 | 25 | 78.333 | 31.357 | `E-PRODUCTION` | execution-lane configuration authority |
| M-08 | `build_laned_shadow_lane_day_operands` | 1054 | 16 | 56.322 | 37.332 | `E-SCIENCE` | production operand and state handoff |
| M-09 | `validate_release_sidecar_unlocked` | 343 | 19 | 67.442 | 31.459 | `E-PRODUCTION` | release authority and fail-closed ordering |
| M-10 | `run_oracle` | 177 | 30 | 95.313 | 30.093 | `E-SCIENCE` | Iwagaki numerical oracle behavior |
| M-11 | `LanduseMigrationError::fmt` | 520 | 17 | 44.776 | 65.672 | `E-PRODUCTION` | stable public migration CLI error text |
| M-11 | `authority_from_files` | 703 | 7 | 0.000 | 56.000 | `E-PRODUCTION` | migration source-authority selection |
| M-12 | `yearly_extension_to_yaml` | 500 | 6 | 0.000 | 42.000 | `E-PRODUCTION` | serialized YAML schema/value/order mapping |
| M-13 | `run_cli_args` | 15 | 54 | 78.146 | 84.437 | `E-PRODUCTION` | CLI grammar, dispatch, overwrite, output |

Coordinates and metrics come from the fresh Medium CRAP artifact. The filtered
census SHA-256, `292c40b7d9eb3dd757c6d4d8cf3e4656bb9bfea00f845e5d065ba37e3fa37118`,
is identical to High-B final. Every row remains in the denominator and must
reach CRAP at most 30.
