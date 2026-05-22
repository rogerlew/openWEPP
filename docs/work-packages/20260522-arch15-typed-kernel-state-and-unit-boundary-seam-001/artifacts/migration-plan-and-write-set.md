# Migration Plan and Write Set

Static: implemented migration sequence and touched-file inventory.
Ran: all required ARCH15 gates executed after migration.
Status: complete.

## Migration Sequence Executed

1. Introduced typed seam core in `openwepp-kernel-contract`.
2. Added `openwepp-unit-boundary` dependency to kernel seam crate.
3. Migrated hillslope orchestrator writeback surface and tests to typed maps.
4. Migrated watershed orchestrator writeback surface and tests to typed maps.
5. Migrated integration test `kernel_writeback_contract` to typed assertions.
6. Ran full workspace gates.

## Write Set (Implemented)

- `crates/openwepp-kernel-contract/Cargo.toml`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/kernel_writeback_contract.rs`
- `Cargo.lock` (workspace dependency graph update)
- `docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/**`
- `docs/work-packages/README.md` (package registry entry)

## Out-of-Scope Carryover

- Scheduler hot-path clone optimization (`CRF-003`) not addressed in ARCH15.
- Parser-to-simulation seam ownership (`CRF-005`/`CRF-010`) not addressed in ARCH15.
- Kernel trait mutability/purity enforcement (`CRF-004`) not changed in code;
  remains queued for explicit contract decision follow-on.
