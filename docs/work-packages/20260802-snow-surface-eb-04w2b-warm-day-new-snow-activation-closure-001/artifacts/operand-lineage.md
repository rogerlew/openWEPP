# Operand Lineage

Status: passed

Evidence mode: **Static + Ran**

| Closure operand | Independent source | Units/sign |
|---|---|---|
| `SWE_before` | pre-call snow state | m, positive input |
| `typed_snow` | sum of each typed `snowfall_m * 0.1` | m SWE, positive input |
| `rain_retained` | public partition result | m, positive input |
| `snowpack_loss` | public partition result | m, negative output |
| `sublimation` | public partition result | m, negative output |
| `SWE_after` | post-call snow state | m, negative storage |

The `0.1` conversion is the canonical fixed `100 kg m^-3` new-snow-density
mapping retained by the shared boundary; EB-04W2B did not invent or calibrate
it. The validator does not reuse a producer residual. Focused mixed-event and
snow-only vectors exercise nonzero typed-snow reconstruction, while an
intentional source-density mismatch demonstrates fail-closed behavior.
