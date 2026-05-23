# WB10 Implementation and Test Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## Implementation Summary

Static:

- `crates/openwepp-kernel-contract/src/lib.rs`
  - Extended `HillslopeKernelPhaseClass` with explicit hydrology classes:
    - `HydrologyEvapotranspiration`
    - `HydrologyPercolationDeepSeepage`
    - `HydrologyLateralTransfer`
    - `HydrologyDrainage`
    - `HydrologyRunoffReconciliation`
    - `HydrologyStorageReconciliation`
  - Added `is_hydrology_phase()` predicate.
  - Added and expanded phase-class predicate tests for hydrology/growth/
    decomposition separation.

- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - Added `HydrologyPhaseDispatch` routing classification.
  - Added typed hydrology routing error surface:
    `HillslopeHydrologyRoutingError::UnsupportedPhaseClass`.
  - Added explicit phase-to-class mapping for WB10 hydrology phases in
    `hillslope_phase_class_for_phase`.
  - Added `hydrology_phase_dispatch_for_phase` guard function enforcing allowed
    scheduler phase-class pairs.
  - Wired hydrology routing guard into scheduler execution path before consumer
    boundary validation, with typed hard-fail status mapping (`HS-HYDRO-E-001`,
    `DomainViolation`).
  - Added WB10 conformance tests:
    - `wb10_contract_conformance_hydrology_phase_classes_are_not_generic`
    - `wb10_contract_conformance_rejects_unsupported_hydrology_phase_class`

- `tests/integration/hillslope_consumer_boundary_integration.rs`
  - Updated hydrology assertion from exact generic class match to
    `is_hydrology_phase()` to preserve typed-seam compatibility with explicit
    WB10 hydrology subclasses.

- Canonical contract and registry updates completed in:
  - `SC-WATBAL-001`
  - `SC-EVAP-001`
  - `SC-PERC-001`
  - `SC-SUBHYD-001`
  - `docs/specifications/science-contracts/index.md`

## Targeted WB10 Tests

Ran:

```bash
cargo test -p openwepp-hillslope-orchestrator wb10_contract_conformance -- --nocapture
cargo test -p openwepp-kernel-contract phase_class_hydrology_predicate_matches_contract -- --nocapture
cargo test -p openwepp --test hillslope_consumer_boundary_integration -- --nocapture
```

Result:

- WB10 orchestrator conformance tests: `2 passed`, `0 failed`.
- Kernel-contract hydrology predicate test: `1 passed`, `0 failed`.
- Hydrology consumer-boundary integration tests: `4 passed`, `0 failed`.

## Required Package Gates

Ran:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Result:

- `cargo fmt --check`: `ok` (after applying `cargo fmt` once for a single
  new match-arm formatting diff).
- `cargo clippy --workspace --all-targets -- -D warnings`: `ok`.
- `cargo test --workspace`: `ok`.
- `cargo deny check`: `ok` (`advisories ok, bans ok, licenses ok, sources ok`).

Note:
- `cargo deny check` emitted `license-not-encountered` warnings for unmatched
  allowlist entries in `deny.toml`; command exit status remained success.
