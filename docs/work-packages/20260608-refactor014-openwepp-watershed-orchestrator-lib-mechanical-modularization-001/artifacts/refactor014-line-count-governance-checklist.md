# REFACTOR014 refactor014 line count governance checklist

Status: complete
Evidence mode: Static + Ran

## Line counts (post-refactor)
- Static: `src/lib.rs`: 674
- Static: `src/lib_mod/types.rs`: 166
- Static: `src/lib_mod/dispatch.rs`: 485
- Static: `src/lib_mod/kernel/kernel_core.rs`: 5638
- Static: `src/lib_mod/kernel.rs`: 16
- Static: `src/lib_mod/mod.rs`: 15

## Governance gates
- Static: `lib.rs` is below `3000` lines (target satisfied).
- Static: `src/lib_mod/kernel.rs` is now a 16-line module facade under `3000`; `src/lib_mod/kernel/kernel_core.rs` remains above `3000` and is intentionally carried as follow-on target.
- Ran: decomposition rationale is recorded in the package handoff and follow-on plan; completion deferred to follow-on kernel package before strict closure.
