# REFACTOR013 refactor013 public api surface parity report

Status: complete  
Evidence mode: Static: completed; Ran: not-run

## Scope
Static:
- Scope target: preserve the `Wb11HydrologyKernel` helper/public symbol behavior after
  module extraction.
- No publicly visible API was broadened or narrowed by this package.
- `impl Wb11HydrologyKernel` methods were moved into `support_helpers_mod/*.rs` with
  signature-preserving relocation.  
- Internal helper methods were re-keyed to `pub(super)` visibility to satisfy
  current module-local call paths.
- `03_kernel_support_00_support_helpers.rs` remains the facade layer for state,
  constants, and tests.

Ran:
- Commanded API surface validation not executed in this pass.
