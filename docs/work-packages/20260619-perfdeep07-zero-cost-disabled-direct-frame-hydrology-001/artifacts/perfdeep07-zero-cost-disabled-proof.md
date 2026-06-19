# PERFDEEP07 Zero-Cost Disabled Proof

Status: HOLD.
Evidence mode: Static/Ran.

## Static Proof

The retained patch removes the measured dense-first compatibility tax in the
request access layer when dense slots are absent:

- `HillslopeKernelRequest::indexed_state_value` and `indexed_flux_value` now
  return directly from indexed surfaces when `has_dense_*_surface()` is false.
- `Wb11HydrologyKernel::state_value_for_symbol` and `flux_value_for_symbol`
  only attempt dense lookup when dense surfaces are present.
- `HotSymbolTables` uses hash lookup for hot scalar, series, and grid symbols.
- Runner frame-roundtrip diagnostics fail closed if explicitly requested
  without an active frame registry.

## Remaining Default Cost

P0 is not proven. The retained patch still measured `685.85 s`, above the
`676.67 s` threshold. Production indexed runtime surfaces remain part of the
current default execution path; removing them made the endpoint slower, so
that is not a valid zero-cost-disabled repair.

## Consequence

PERFDEEP07 must stay in `HOLD`. Direct-frame implementation and any default
activation remain blocked until the default-disabled timing gate passes.
