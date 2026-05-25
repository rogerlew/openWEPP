# Erod13 implementation and test evidence

Status: completed
Evidence mode: mixed

## Static
- Production runtime implementation completed in:
  - `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- Core changes:
  - Added EROD13 symbol constants and tolerances.
  - Added EROD13 guard variants to `Wb11HydrologyKernelGuardError` with message IDs:
    - `HKERNEL-EROD13-CORE-E-001..003`
  - Added EROD13 helper guards for enabled-path symbol/domain validation.
  - Added `run_erod13_wave1_core` algorithm implementation.
  - Integrated EROD13 execution into `run_peak_runoff` post-WB16 compute path.
  - Added EROD13 success status emission on enabled path:
    - `HKERNEL-EROD13-CORE-OK-001`
- Contract-derived tests added:
  - `tests/integration/erod13_contract_authority_closure_contract.rs`
  - `tests/integration/erod13_wave1_core_kernel_contract.rs`
- Test target registrations added in `Cargo.toml`.

## Ran
- Pre-implementation baseline (expected fail for runtime vectors) was captured before production edits:
  - `cargo test --test erod13_contract_authority_closure_contract --test erod13_wave1_core_kernel_contract`
  - authority tests passed; runtime vectors failed (7/7).
- Post-implementation validation:
  - `cargo test --test erod13_contract_authority_closure_contract --test erod13_wave1_core_kernel_contract` -> pass.
  - `cargo test --test wb16_peak_runoff_kernel_contract --test erod13_wave1_core_kernel_contract` -> pass.
