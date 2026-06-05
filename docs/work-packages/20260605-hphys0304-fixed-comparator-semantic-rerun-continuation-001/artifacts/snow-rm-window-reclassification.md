# Snow/RM Window Reclassification

Status: complete

Evidence mode: ran

Static:

- ADR-0011 treats single-OFE daily water-balance deltas as higher-confidence investigation signals.
- Aggregate daily WAT residuals still do not identify `amelt`/`bmelt`/`cmelt`/`dmelt` term ownership.

Ran:

- Reclassified `9` H1/H7/H39 target windows against the fixed baseline.
- Production edit authorized: `false` for every row.

| H | Window | Year | J Range | RM Residual | Snow-Water Residual | Total-Soil Residual | Classification |
| ---: | --- | ---: | --- | ---: | ---: | ---: | --- |
| H1 | first-abs-storage-ge-10mm | 2013 | 112-127 | 14.672576 | 47.080388 | 207.526646 | fixed-baseline-unchanged-term-state-hold |
| H1 | spring-2014 | 2014 | 120-146 | 63.562583 | 567.967857 | -1546.726174 | fixed-baseline-unchanged-term-state-hold |
| H1 | spring-2016 | 2016 | 104-111 | 15.276407 | 69.169930 | -857.225502 | fixed-baseline-unchanged-term-state-hold |
| H7 | first-abs-storage-ge-10mm | 2013 | 112-127 | 11.427268 | 5.890809 | 186.596716 | fixed-baseline-unchanged-term-state-hold |
| H7 | spring-2014 | 2014 | 120-146 | 61.799024 | 488.240644 | 294.808536 | fixed-baseline-unchanged-term-state-hold |
| H7 | spring-2016 | 2016 | 104-111 | 16.885426 | 110.855194 | -46.727989 | fixed-baseline-unchanged-term-state-hold |
| H39 | first-abs-storage-ge-10mm | 2013 | 97-112 | 10.689298 | 33.599154 | 163.029542 | fixed-baseline-unchanged-term-state-hold |
| H39 | spring-2014 | 2014 | 120-146 | 65.755222 | 592.358693 | 973.943367 | fixed-baseline-unchanged-term-state-hold |
| H39 | spring-2016 | 2016 | 104-111 | 15.940163 | 74.530948 | 49.593766 | fixed-baseline-unchanged-term-state-hold |

Continuation: HPHYS0305 is the required paired melt-term/state instrumentation package. No snow, forcing, WB13, WB17, WB18, WB19, or WB12 production edit is authorized by HPHYS0304.
