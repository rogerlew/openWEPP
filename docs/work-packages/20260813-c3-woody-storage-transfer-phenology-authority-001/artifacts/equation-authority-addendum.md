# Equation Authority Addendum

| Rule | V7 selection | Authority |
|---|---|---|
| Preparation fraction | `f_stor_xfer=0.5` | CLM5 phenology |
| Preparation owners | Six tissue storage C/N to matching transfer C/N | CLM5 phenology/allocation |
| Deployment | `f_on=1` when remaining `<=dt`; otherwise `min(1,2dt/remaining)` | Imported E20/CLM5 |
| Preparation timing | One seasonal `Dormant -> Onset` edge | CLM5 mapped to openWEPP GSI owner |
| Same-interval growth | Excluded from preparation | openWEPP canonical ordering selection |
| Growth respiration | No additional debit | openWEPP E19 ownership selection |
| Evergreen | `f_cur=1`, zero storage/transfer | CLM5 evergreen branch |

V7 imports every other V6 physical and numerical rule unchanged.

