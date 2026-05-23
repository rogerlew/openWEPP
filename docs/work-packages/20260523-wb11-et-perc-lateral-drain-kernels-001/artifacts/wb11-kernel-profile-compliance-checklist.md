# WB11 Kernel Profile Compliance Checklist

Status: `completed`
Evidence mode: `Static + Ran`

## Procedure Dependencies
- [x] `docs/specifications/science-contract-authoring-procedure.md` referenced in package dependencies.
- [x] `docs/specifications/science-contracts/kernel-process-contract-profile.md` referenced in package dependencies.

## Contract-First Requirements
- [x] Canonical SC contract amendments implemented (`SC-WATBAL-001`, `SC-EVAP-001`, `SC-PERC-001`, `SC-SUBHYD-001`, registry note updates).
- [x] Contract-derived WB11 tests implemented before production kernel availability.
- [x] Pre-implementation contract gate evidence recorded (missing `Wb11HydrologyKernel` failure).

## Kernel Behavior Requirements
- [x] Deterministic ET/percolation/lateral/drain execution implemented.
- [x] Typed guard propagation for missing/non-finite/domain states implemented.
- [x] No silent defaults/clamping for invalid routing/domain states.

## Validation Gates
- [x] `cargo fmt --check`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] `cargo deny check`

## Disposition Constraint
- [x] Package-level kernel-profile compliance evidence present in artifacts.
