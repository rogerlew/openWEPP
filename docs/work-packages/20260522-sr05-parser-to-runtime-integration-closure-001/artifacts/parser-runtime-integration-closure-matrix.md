# Parser-to-Runtime Integration Closure Matrix (SR05)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Matrix defines integration-closure proof points for SR02 slope seam and SR03 expanded soil seam propagation through hillslope scheduler runtime surfaces.

Ran:
- All listed tests executed under `cargo test --workspace` and passed.

| closure_id | path_type | seam surface | test anchor | expected result | status |
|---|---|---|---|---|---|
| `SR05-INT-HS-SOIL-001` | happy path | SR03 soil parser -> runtime -> scheduler | `parser_to_hillslope_runtime_surface_closure` | scheduler receives required soil runtime symbols (`solthk`,`dg`,`thetdr`,`thetfc`,`nsl`,`ssc`,`ssc_0002`) | `pass` |
| `SR05-INT-HS-SLP-001` | happy path | SR02 slope parser -> runtime -> scheduler | `slope_parser_to_hillslope_runtime_surface_closure` | scheduler receives required slope runtime symbols (`nelem`,`nwsofe`,`nslpts`,`slplen`,`avgslp`, indexed OFE symbols) | `pass` |
| `SR05-INT-HS-SLP-SOIL-001` | happy path | combined SR02+SR03 runtime surfaces -> scheduler | `slope_and_soil_parser_outputs_propagate_to_hillslope_runtime_surface_closure` | scheduler consumes both slope and expanded-soil symbol families in one execution surface | `pass` |
| `SR05-INT-HS-SOIL-FAIL-001` | typed failure | SR03 soil required conductivity guard | `soil_runtime_surface_rejects_missing_saturated_conductivity_projection` | returns typed `HS-RUNTIME-E-033`; no default substitution | `pass` |
| `SR05-INT-HS-SOIL-FAIL-002` | typed failure | SR03 soil declared layer-count closure guard | `soil_runtime_surface_rejects_declared_nsl_mismatch_projection` | returns typed `HS-RUNTIME-E-028`; no fallback behavior | `pass` |
| `SR05-INT-HS-SLP-FAIL-001` | typed failure | SR02 slope declared point-count closure guard | `slope_runtime_surface_rejects_declared_nslpts_mismatch_projection` | returns typed `HS-RUNTIME-E-014`; no fallback behavior | `pass` |
| `SR05-INT-HS-SLP-FAIL-002` | typed failure | SR02 slope derived avg slope domain guard | `slope_runtime_surface_rejects_non_positive_avgslp_projection` | returns typed `HS-RUNTIME-E-023`; no clamped default | `pass` |
