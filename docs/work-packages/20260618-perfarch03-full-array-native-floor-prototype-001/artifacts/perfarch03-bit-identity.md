# PERFARCH03 Bit Identity

Evidence class: Ran.

Status: pass for the validated branch.

## Validation method

The prototype validates the array-native branch against the current public
production WB11 runoff kernel before timing is reported.

Validation flow:

1. Build the same logical input state and flux maps used by the production
   kernel request.
2. Run `Wb11HydrologyKernel::run` through `HillslopeKernelRequest`.
3. Resolve the production writeback payload symbols to `SymbolId` slots.
4. Run the array-native branch and dense writeback plan.
5. Compare every validated output value against the production payload by exact
   `f64::to_bits()` equality.

Validated output count:

| Output kind | Count |
|---|---:|
| State | 543 |
| Flux | 8 |
| Total | 551 |

## Result

The validation passed during the release timing run that produced
`perfarch03-floor-prototype.tsv`.

No absolute or relative tolerance was used for the numeric identity check. The
values matched exactly by `to_bits()`.

## Limitations

The identity check compares the numeric scalar value returned by
`BoundaryValue::as_f64()`. It does not assert variant-level identity for
`BoundaryValue` wrappers. The validated production outputs for this branch are
scalar values, so this is sufficient for the branch-floor measurement but should
not be reused as a general payload-identity contract without extension.

The identity claim applies to the warm-rain/no-snow/no-frost/no-irrigation/no-
MOFE branch only.
