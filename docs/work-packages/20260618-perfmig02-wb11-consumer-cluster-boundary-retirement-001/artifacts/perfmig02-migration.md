# PERFMIG02 Migration

Static: inspected the changed Rust files and verified the migrated read/apply boundary is limited to the
WB11 warm-rain indexed path.

Ran: focused tests in `perfmig02-gate-results.md` exercised the indexed-only handoff and stale logical
removal.

## Code Changes

| File | Change |
|---|---|
| `crates/openwepp-kernel-contract/src/lib_mod/writeback.rs` | Added `IndexedWritebackLogicalMaterialization` and `apply_indexed_kernel_writeback_with_logical_materialization`. The old `apply_indexed_kernel_writeback` delegates to the new function with `materialize_all()`. |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | Resolves PERFMIG02 retired materialization ids once from the run-scoped registry and applies indexed writebacks with a skip policy. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs` | Central scalar helpers now prefer indexed hot state/flux values when an indexed surface exists and the symbol resolves in `HotSymbolTables`; logical maps remain the fallback. |
| `crates/openwepp-hillslope-orchestrator/src/lib.rs` | Re-exported the new materialization policy/apply function into the scheduler module surface. |
| `crates/openwepp-kernel-contract/src/lib.rs` | Added focused policy test for skipped logical materialization and stale-entry removal. |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs` | Added scheduler test proving retired symbols stay indexed-only between phases and are not visible in logical maps. |

## Materialization Policy

PERFMIG02 keeps indexed authority for every id in the payload. The skip policy only controls compatibility
logical map publication.

Skipped state ids:

- `wb12_infiltration`
- `wb12_runoff_reconciled`
- `wb14_soil_conductivity_m_s`
- `wb14_effective_conductivity_m_s`
- `wb14_matric_potential_m`

Skipped flux ids:

- `wb12_runoff_carryover`

For skipped ids, the apply path first resolves the id to a symbol, updates the indexed surface, then removes
any stale logical value for the symbol. This preserves fail-closed id validation and prevents prior-day or
seeded logical data from masquerading as current authority.

## Dense-First Reader Migration

The scalar helper migration is broader than the six-symbol materialization skip. Any caller that uses the
standard state/flux scalar helpers now reads the indexed hot slot first when available:

- `optional_state_scalar_for_symbol`
- `require_state_scalar_for_symbol`
- `optional_flux_scalar_for_symbol`
- `require_flux_scalar_for_symbol`
- `optional_state_scalar`
- `require_state_scalar`
- `optional_flux_scalar`
- `require_flux_scalar`

The fallback remains explicit and visible: if no indexed surface exists, or the symbol is not in the hot table,
the helper reads the logical map and preserves the same missing/non-finite typed guard behavior.

## Scope Boundaries

No science equations, output schemas, HBP serialization, or public publication boundaries were changed.
Logical materialization remains for public/reporting symbols and unmigrated diagnostic surfaces. This is not
a full array-native WB11 branch; kernel input reads and compute still pass through the existing request model.
