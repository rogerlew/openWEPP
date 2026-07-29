# Operand Lineage

Status: `PASS`

| Result operand | Units/basis | Source | Reconstruction and rejected aliases |
| --- | --- | --- | --- |
| Accepted-member count | members | CAL-04B accepted ensemble | Count accepted rows; do not use all finite or all searched candidates |
| Harvard aggregate-error range | days | CAL-04B Harvard rows for accepted IDs | Minimum/maximum `aggregate_score`; do not pool species RMSE or year medians |
| Harvard interval coverage | percent of named intervals | Same rows | Count exact zero fractions and scale maximum by 100; do not treat as probability of transfer |
| Litter ridge target/difference | kg dry mass m^-2 ground | CAL-05 terminal ridge | Common target and maximum absolute row difference; do not substitute equilibrium stock or daily SSE |
| CAL-06 run count | runs | Seven 37-member forest lanes plus two open controls | `7 × 37 + 2 = 261`; open controls are not multiplied by ensemble membership |
| Winter ordering count | members | CAL-06 forest strata | Require all 37 in each source-supplied gradient; ordering is not canopy-amplitude accuracy |
| Bezà complete members | members | CAL-07F two-product member rows | Require all 12 seasonal crossings in both products; wrong-season recovery is rejected |
| Bezà interval hits/errors | transitions and days | `GSI-4831` under each product | Use product-specific retained fields; do not mix `gcc_mean` hits with `gcc_90` error |
| Elliot contradicted count | targets | CAL-02 comparison rows | Count exact `CONTRADICTED` classifications; do not relabel missing exact historical inputs as openWEPP defects |

Mass and transformation claims in the manuscript remain inherited
contract/closure evidence and are not recalculated from a single producer
summary. The CAL-09 strict result intentionally reports only independently
recoverable synthesis operands.
