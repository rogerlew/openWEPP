# Erod13 disposition

Status: completed
Evidence mode: mixed

## Static
- Package objective: implement Wave-1 core hillslope erosion kernel behavior with contract-first sequencing.
- Final state: completed.
- Sequencing validation:
  1. canonical contracts updated,
  2. contract-derived tests authored,
  3. pre-implementation gate captured,
  4. production runtime implementation completed.

## Ran
- Post-implementation gates:
  - `cargo fmt --check` pass
  - `cargo clippy --workspace --all-targets -- -D warnings` pass
  - `cargo test --workspace` pass
  - `cargo deny check` pass

## Exit criteria checklist
- [x] `INV-SED-001..007` Wave-1 scope implemented in canonical authority and runtime path.
- [x] Typed guard family implemented (`HKERNEL-EROD13-CORE-E-001..003`).
- [x] Contract-first evidence sequence complete.
- [x] Required repo gates completed.
- [x] Dual review and dual verification artifacts completed.
- [x] Explicit Wave-1 GO/HOLD verdict published (`GO`).
