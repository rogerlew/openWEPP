# Symbol Alias Consumer Coverage Matrix (SR04)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Matrix maps SR04 alias registry additions to contracted slope/soil consumer obligations and seam ownership boundaries.

Ran:
- Coverage entries validated against passing SR04 alias-registry integration tests.

| contract / consumer surface | canonical symbols | alias forms covered by SR04 | coverage status | notes |
|---|---|---|---|---|
| `SC-INFILE-SLOPE-001` -> SR02 slope runtime seam | `nslpts`, `slplen`, `xinput`, `slpinp`, `avgslp`, `nelem`, `nwsofe` | exact + indexed templates (`ofe{ofe}_*`, `{idx4}` point forms) | `covered` | closes registry continuity for SR02 projected surfaces |
| `SC-INFILE-SOIL-001` -> SR03 soil runtime seam | `ntemp`, `nsl`, `solthk`, `dg`, `thetdr`, `thetfc`, `ssc` | exact + OFE/layer indexed templates; ARCH03 semantic aliases preserved | `covered` | bridges canonical names and expanded SR03 runtime keys |
| `SC-SOIL-001` hillslope soil consumer substrate | `solthk`, `dg`, `thetdr`, `thetfc`, `ssc`, `nsl` | first-OFE aliases + indexed families | `covered (registry scope)` | SR04 handles naming continuity only; dynamic soil-state evolution remains downstream |
| `SC-WATBAL-001` hydrology coupling substrate | `solthk`, `dg`, `thetdr`, `thetfc`, `ssc`, `nsl` | first-OFE aliases + indexed families | `covered (registry scope)` | supports layer-wise water-balance lookup continuity |
| `SC-SUBHYD-001` subsurface coupling substrate | `solthk`, `dg`, `thetfc`, `ssc`, `nsl` | first-OFE aliases + indexed families | `covered (registry scope)` | supports drainable-layer lookup continuity |
