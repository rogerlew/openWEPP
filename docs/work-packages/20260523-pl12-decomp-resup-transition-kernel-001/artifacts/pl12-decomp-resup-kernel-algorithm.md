# PL12 Decomp/Resup Kernel Algorithm

Status: `complete`
Evidence mode: `Static`

## Runtime Algorithm Summary

Production decomposition-phase dispatch now executes this sequence:

1. Resolve active PL slot/crop selection for runtime `day/year`.
2. Validate decomposition ordering flags and required seed state symbols.
3. Classify management branch from `imngmt`.
4. For annual/fallow branch:
   - consume `resmgt` + annual transition controls
   - validate day/fraction domains and inactive-family zero semantics
   - derive deterministic active action:
     `none`, `herbicide`, `burn`, `silage`, `cut`, `remove`
5. For perennial branch:
   - consume `mgtopt`, `ncut`, `ncycle`, indexed `cutday`/grazing payload
     families
   - enforce contiguous cardinality closure and no overflow indexed symbols
   - enforce grazing window ordering (`gday < gend`) and positive payload
     domains
   - derive deterministic active action:
     `none`, `cut(event_index)`, `grazing(cycle_index)`
6. Assemble typed
   `HillslopeDecompositionTransitionPayload` and attach it to
   `HillslopeDecompositionKernelContext`.
7. Reject invalid domains as typed hard failures before downstream kernel
   execution.

## Guard/Error Map

- `HS-DECOMP-E-001`: missing required scalar state symbol
- `HS-DECOMP-E-002`: non-finite required scalar state symbol
- `HS-DECOMP-E-003`: invalid ordering flag value
- `HS-DECOMP-E-004`: unsupported management class
- `HS-DECOMP-E-005`: required integral symbol is non-integral
- `HS-DECOMP-E-006`: symbol value outside allowed range
- `HS-DECOMP-E-007`: missing required indexed symbol
- `HS-DECOMP-E-008`: unexpected overflow indexed symbol
- `HS-DECOMP-E-009`: invalid grazing window (`gday >= gend`)
- `HS-DECOMP-E-010`: invalid transition-payload state (branch incompatibility,
  invalid zero/positive/fraction semantics, or conflicting active transition)

## Typed Payload Surfaces

Static:

- `openwepp-kernel-contract` now provides typed decomposition transition payload
  types:
  - `HillslopeAnnualDecompositionControl`
  - `HillslopePerennialDecompositionControl`
  - `HillslopeDecompositionTransitionControl`
  - `HillslopeDecompositionTransitionPayload`
- `HillslopeDecompositionKernelContext` now carries optional typed transition
  payload via `.with_transition_payload(...)`.
