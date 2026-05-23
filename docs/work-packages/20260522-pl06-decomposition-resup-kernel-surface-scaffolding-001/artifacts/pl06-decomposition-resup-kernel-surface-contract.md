# PL06 Decomposition/Resup Kernel Surface Contract

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Added typed decomposition scheduler metadata at the hillslope kernel seam.
- Added explicit decomposition-boundary error taxonomy and typed boundary-class mapping.
- Preserved hydrology request compatibility while extending request construction to dual transition contexts.

Ran:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Implemented Interface Surface

1. Kernel contract typed decomposition metadata:
- `HillslopeDecompositionManagementClass` (`AnnualOrFallow`, `Perennial`)
- `HillslopeKernelPhaseClass` additions (`DecompositionTransition`, `ResiduePartitionTransition`)
- `HillslopeDecompositionKernelContext`
- `HillslopeConsumerAdapter::Decomposition`
- `HillslopeKernelRequest` additions:
  - `decomposition_context`
  - `with_transition_context(...)`

2. Scheduler decomposition boundary failure IDs:
- `HS-DECOMP-E-001` missing required decomposition symbol
- `HS-DECOMP-E-002` non-finite required value
- `HS-DECOMP-E-003` ordering flag mismatch
- `HS-DECOMP-E-004` unsupported management class

3. Typed boundary-class mapping:
- missing -> `BoundaryClass::MissingRequiredInput`
- non-finite -> `BoundaryClass::NonFinite`
- ordering/domain -> `BoundaryClass::DomainViolation`

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:342`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:350`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:414`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:465`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:495`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:290`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:532`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:1500`
