# PL07 Typed Reject Path Catalog

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL07 requires explicit typed reject-path assertions for required PL seam inputs with no silent defaults.

Ran:
- All PL seam reject paths `HS-RUNTIME-E-036..045` are asserted at integration level via fixture mutation tests.

| code | variant | mutation/assertion anchor | status |
|---|---|---|---|
| `HS-RUNTIME-E-036` | `ManagementTopologyCountMismatch` | `management_runtime_surface_rejects_topology_count_mismatch_projection` | `pass` |
| `HS-RUNTIME-E-037` | `ManagementScheduleSlotCountMismatch` | `management_runtime_surface_rejects_slot_count_mismatch_projection` | `pass` |
| `HS-RUNTIME-E-038` | `ManagementScheduleSlotArityMismatch` | `management_runtime_surface_rejects_slot_arity_mismatch_projection` | `pass` |
| `HS-RUNTIME-E-039` | `ManagementInitialReferenceOutOfRange` | `management_runtime_surface_rejects_out_of_range_initial_reference_projection` | `pass` |
| `HS-RUNTIME-E-040` | `ManagementYearlyReferenceOutOfRange` | `management_runtime_surface_rejects_out_of_range_yearly_reference_projection` | `pass` |
| `HS-RUNTIME-E-041` | `UnsupportedPlLanduse` | `management_runtime_surface_rejects_unsupported_landuse_projection` | `pass` |
| `HS-RUNTIME-E-042` | `UnsupportedPlManagementOption` | `management_runtime_surface_rejects_unsupported_perennial_option_projection` | `pass` |
| `HS-RUNTIME-E-043` | `NonFinitePlProjectionField` | `management_runtime_surface_rejects_non_finite_required_growth_projection` | `pass` |
| `HS-RUNTIME-E-044` | `PlProjectionCountOutOfRange` | `management_runtime_surface_rejects_overflowed_projection_count` | `pass` |
| `HS-RUNTIME-E-045` | `ManagementScheduleOfeIndexOutOfRange` | `management_runtime_surface_rejects_schedule_ofe_index_out_of_range_projection` | `pass` |

Reference block:
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:571`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:588`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:605`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:623`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:641`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:659`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:678`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:695`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:719`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:737`
