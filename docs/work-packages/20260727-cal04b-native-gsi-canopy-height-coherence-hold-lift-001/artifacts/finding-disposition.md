# Finding Disposition

Status: `ACTIVE / CONTRACT FINDINGS CLOSED`

Evidence class: `Static + Ran`

| Finding | Disposition | Closure |
|---|---|---|
| A-01 `Bt` publication could replace WB15 foliar physics | accepted | `Bt` is internal to height; `Bf` remains foliar/interception state |
| A-02 missing native branch/guard row | accepted | added `BR-PL-NATIVE-GSI-HEIGHT` |
| A-03 primary equation citation wrong | accepted | Chapter 8 Eq. 8.2.8 is primary; historical source label disclosed |
| A-04 arithmetic guards/vectors incomplete | accepted | checked sum/product/result, overflow, underflow, and saturation bound |
| A-05 YAML versus PL projection enforcement stale | accepted | `bb/xmxlai` remain YAML/runtime; `bbb/hmax` are selected-crop runtime projections |
| A-06 BEI omitted obligations | accepted | all GSI producer/consumer obligations are canonical IDs in the BEI |
| A-R1 structural-only underflow unguarded | accepted | `Bt>0 => Hc>0` or typed failure everywhere |
| A-R2 top-level `Bt` consumer wording stale | accepted | sole `Bt` consumer is internal height projection |
| B-01 consumer height overclaim | accepted | all consumers share the realization; ET/erosion/Lane D specifically share `Hc` |
| B-02 BEI checker self-authenticated missing IDs | accepted | core IDs exclude BEI text; three regression vectors pass |

No contract-review finding remains open. Implementation and terminal findings
will be appended and dispositioned before closure.
