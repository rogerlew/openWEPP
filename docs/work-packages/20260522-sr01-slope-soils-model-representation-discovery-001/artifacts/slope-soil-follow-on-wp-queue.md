# Slope/Soil Follow-On Work-Package Queue

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Queue ordering is driven by ownership closure dependencies discovered in SR01.

Ran:
- Derived queue from audited baseline/openWEPP seams and symbol gaps.

| wp_id | title | objective | dependencies |
|---|---|---|---|
| `SR02` | Slope Runtime Seam Contract + Builder | Add typed hillslope runtime projection for canonical slope surfaces (`slplen`, `nslpts`, `xinput/slpinp` or derived equivalents, `avgslp`, segment coefficients where required). | SR01 decision record |
| `SR03` | Soil Runtime Seam Expansion | Extend soil runtime export from minimal 4-symbol seed to full layer/profile surfaces needed by `SC-SOIL-001`, `SC-WATBAL-001`, `SC-SUBHYD-001` consumers. | SR01, SR02 (shared seam patterns) |
| `SR04` | Symbol Alias Continuity Completion | Expand `openwepp-sim-contract` alias registry to include slope + expanded soil symbols with canonical continuity tables. | SR02, SR03 |
| `SR05` | Parser-to-Runtime Integration Closure | Add integration tests validating slope and expanded soil parser outputs reach runtime scheduler surfaces without silent defaults. | SR02, SR03, SR04 |
| `SR06` | Consumer Ownership Wiring (Hillslope Kernels) | Wire slope/soil runtime surfaces into hillslope consumer boundaries (runoff/soil/watbal/perc adapters) with typed error propagation only. | SR05 |
| `SR07` | Comparator Confidence-Tier Delta Review | Run legacy comparator review for single-OFE daily water-balance confidence tier after SR06 to validate semantic parity direction. | SR06 |

## Queue Notes

- `SR02` and `SR03` can be developed in parallel only if write ownership is disjoint.
- `SR04` should land before consumer wiring to prevent alias drift.
- `SR07` is acceptance signal, not bitwise parity gate.
