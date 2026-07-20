# Independent Review A

Evidence mode: `Static`

Reviewed range: `49ff3138..53f47dba` (withdrawn prototype)

Status: `HOLD`

## Findings

| ID | Severity | Finding | Required disposition |
|---|---|---|---|
| A-01 | high | `00c_day_input_builder_impl.rs` seeded first-day foliar state from aggregate PL `vdmt`, violating the package's rejected-alias rule and potentially fabricating allocation/litter. | Add an explicit first-realization boundary with no synthetic transfer and prove `vdmt` cannot seed foliar mass. |
| A-02 | high | The three amended contracts remained `in_review` / `draft` while prototype production code consumed them. | Complete dual review, disposition, fixes, and verification before promotion or activation. |
| A-03 | high | The real-run test proved execution and file creation only; source-order guards did not dynamically prove snow, ET, WB15, erosion, residue/depth, and frost consumed the same realized values. | Add runtime-value evidence on the real direct run. |
| A-04 | medium | The SH test compared only 183 GSI pairs and did not exercise full-year wrap, canopy state, or leaf-on/off phase within one day. | Test the full wrapped phase transform and canopy/transfer limbs. |
| A-05 | medium | The kernel accepted `bb == 0` although CP-GSI02 required finite `bb > 0`. | Enforce positive finite `bb` through schema/projection/runtime with negative tests. |

Recommendation: `HOLD` until all five findings are fixed and verified.
