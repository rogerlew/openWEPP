# Erod13 typed error guard surface map

Status: completed
Evidence mode: mixed

## Static
- Guard family implemented in `Wb11HydrologyKernelGuardError`:
  - `HKERNEL-EROD13-CORE-E-001`: missing required enabled-path symbol.
  - `HKERNEL-EROD13-CORE-E-002`: non-finite required enabled-path symbol.
  - `HKERNEL-EROD13-CORE-E-003`: domain/closure violation on enabled path.
- Guard emission boundary: closure diagnostics (`HydrologyPeakRunoff`) when `erod13_core_enabled = 1`.
- Boundary-class mapping:
  - `-E-001` -> `MissingRequiredInput`
  - `-E-002` -> `NonFinite`
  - `-E-003` -> `DomainViolation`

## Ran
- Verified guard-code behavior via integration vectors:
  - `erod13_contract_vector_rejects_missing_required_symbol` -> `HKERNEL-EROD13-CORE-E-001`
  - `erod13_contract_vector_rejects_non_finite_required_symbol` -> `HKERNEL-EROD13-CORE-E-002`
  - `erod13_contract_vector_rejects_domain_violation` -> `HKERNEL-EROD13-CORE-E-003`
  - `erod13_contract_vector_rejects_continuity_residual_violation` -> `HKERNEL-EROD13-CORE-E-003`
