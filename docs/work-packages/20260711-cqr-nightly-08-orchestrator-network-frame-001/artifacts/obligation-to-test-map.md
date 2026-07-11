# Obligation-To-Test Map

| Authority / family | Characterization tests | Status |
|---|---|---|
| `INV-SYSTEM-003` A/C/D/E/F/H: valid duration max, zero boundary, non-finite/domain and missing operands, fail closed without repair | `duration_contract_valid_maxima_and_dtchr_floor`; `duration_contract_reachable_input_guards`; `duration_contract_reachable_dependency_guards` | Bound |
| `INV-SYSTEM-036`, `INV-ROUTE-005(e)` A/C/G/H: terminal-only extensive sediment, hourly active span, event fallback, impoundment boundary | `wshedw11d_terminal_selector_and_extensive_sediment_sum_exclude_internal_channel`; `wshedw11d_terminal_selector_follows_serial_impoundment_path`; `sediment_duration_contract_covers_hourly_fallback_and_guards` | Bound |
| `INV-ROUTE-005(c/e)`, `INV-ROUTE-011` C/D/E/F/H: hourly cardinality, missing contributor, non-finite sediment and typed failure | `sediment_duration_contract_covers_hourly_fallback_and_guards` | Bound |
| `INV-ROUTE-021`, `INV-SYSTEM-033` A/G/H: terminal water authority, diagnostic channel operands, area-normalized runoff, missing routed states | `terminal_publication_contract_covers_wrapper_success_and_missing_states`; real W11D runner consumer suite | Bound |
| `INV-SYSTEM-035`, `G-CHN-012` A/C/D/E/H: runtime-ready globals, defaults, applicability, mismatch, missing options and typed conversion | `routing_global_contract_covers_authorized_branches_and_failures` | Bound |
| Typed value/error surfaces and area closure A/C/D/E/H | `contributing_area_contract_covers_complete_and_rejected_operands`; `small_surface_contracts_cover_conversions_helpers_and_error_sources` | Bound |

Families B/F are enforced at typed parser/kernel producers when the frame receives
already-normalized integer options; no duplicate parse/conversion policy is owned
here. The only eligible-surface exclusion is observability-only
`WatershedNetworkFrameError::fmt`. Construction-impossible postconditions are
reviewed as covered by their dominating positive-finite/max invariants.
