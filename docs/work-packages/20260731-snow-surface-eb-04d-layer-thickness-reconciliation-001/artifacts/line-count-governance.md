# Line-Count Governance

Status: `PASS`

Evidence class: `Ran`

| File | Lines | Disposition |
| --- | ---: | --- |
| `09_snow_density.rs` | 1,308 | below WARN |
| `02_guard_errors.rs` | 725 | below WARN |
| `runoff_reconciliation.rs` | 2,443 | WARN: pre-existing cohesive WB11/Stage 3 implementation; this package routes lifecycle decisions through one predicate and adds local conservation tests. |
| `openwepp-unit-boundary/src/lib.rs` | 1,384 | below WARN |
| `boundary_catalog.rs` | 1,594 | below WARN |
| `00c_day_input_builder_impl.rs` | 2,406 | WARN: pre-existing cohesive direct-publication builder; this package adds one opt-in trace serializer only. |
| `snow_surface_eb03_contract.rs` | 256 | below WARN |
| `snow_surface_eb03_runtime.rs` | 889 | below WARN |

The two WARN files were already large, and extracting their tightly coupled
runtime/trace logic inside this correction would expand risk beyond EB-04D.
No touched file reaches the 3,000-line HOLD threshold.
