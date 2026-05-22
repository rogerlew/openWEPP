# PL05 Growth Kernel Surface Contract

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Added typed growth scheduler metadata at the hillslope kernel seam.
- Added explicit growth-boundary error taxonomy and typed boundary-class mapping.

Ran:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Implemented Interface Surface

1. Kernel contract typed growth metadata:
- `HillslopeGrowthManagementClass` (`AnnualOrFallow`, `Perennial`)
- `HillslopeKernelPhaseClass` (`Hydrology`, `GrowthAnnualTransition`, `GrowthPerennialTransition`)
- `HillslopeGrowthKernelContext`
- `HillslopeConsumerAdapter::Growth`
- `HillslopeKernelRequest` fields:
  - `phase_class`
  - `growth_context`
  - `with_phase_context(...)` constructor for scheduler wiring

2. Scheduler growth boundary failure IDs:
- `HS-GROWTH-E-001` missing required growth input
- `HS-GROWTH-E-002` non-finite required growth input
- `HS-GROWTH-E-003` invalid ordering flag value
- `HS-GROWTH-E-004` unsupported management class value

3. Typed boundary-class mapping:
- missing -> `BoundaryClass::MissingRequiredInput`
- non-finite -> `BoundaryClass::NonFinite`
- ordering/domain -> `BoundaryClass::DomainViolation`

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:335`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:342`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:369`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:392`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:420`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:166`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:386`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:1018`
