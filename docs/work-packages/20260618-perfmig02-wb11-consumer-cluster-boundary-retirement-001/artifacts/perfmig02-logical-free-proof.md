# PERFMIG02 Logical-Free Proof

Static: inspected scheduler skip-id resolution, writeback apply policy, and dense-first scalar helpers.

Ran: focused scheduler test proving indexed-only between-phase visibility for retired symbols.

## No Dual-Read For Retired Symbols

For the retired PERFMIG02 symbols, normal downstream scalar reads now use indexed authority:

- the scheduler carries an `IndexedWritebackSurface` beside the logical surfaces;
- `HillslopeKernelRequest` exposes hot state/flux symbol tables;
- `optional_state_scalar_for_symbol` / `require_state_scalar_for_symbol` check indexed hot state values first;
- `optional_flux_scalar_for_symbol` / `require_flux_scalar_for_symbol` check indexed hot flux values first.

The fallback to logical maps remains for unmigrated symbols and non-indexed execution. For the retired set,
the focused scheduler test proves the values are visible from indexed authority and absent from logical maps.

## Logical Materialization Removed/Moved

Scheduler-retired state materialization:

- `wb12_infiltration`
- `wb12_runoff_reconciled`
- `wb14_soil_conductivity_m_s`
- `wb14_effective_conductivity_m_s`
- `wb14_matric_potential_m`

Scheduler-retired flux materialization:

- `wb12_runoff_carryover`

The logical boundary is moved outward for these internal symbols: indexed authority remains updated at the
phase boundary, while public/reporting logical materialization remains for symbols that still need it.

## Stale Logical Protection

The apply path does not merely skip insertion. It removes skipped symbols from the logical maps after indexed
authority is updated. This matters because the same surface can contain seeded values or previous phase/day
values. Leaving those values in place would create a silent dual-authority hazard.

Focused kernel-contract test coverage:

- skipped state and flux ids update indexed authority;
- skipped symbols are absent from logical maps after apply;
- retained symbols still materialize logically;
- applied symbol lists report only materialized logical symbols.

Focused scheduler test coverage:

- first phase writes `wb12_infiltration` and `wb12_runoff_carryover` through indexed payload;
- later phases see the current values through indexed hot lookups;
- later phases do not see those symbols in the logical state/flux maps;
- final writeback surface does not contain those retired logical keys.

## Residual Logical Boundaries

PERFMIG02 did not remove logical maps globally. Logical materialization remains required for publication,
diagnostics, guard surfaces, public outputs, and unmigrated branches. This artifact proves the retired
internal six-symbol boundary only.
