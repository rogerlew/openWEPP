# Slope/Soil Symbol Alias Continuity Table (SR04)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Table captures canonical slope/soil symbols surfaced by SR02/SR03 and their boundary alias forms in `openwepp-sim-contract`.
- Indexed families use explicit template aliases (`{ofe}`, `{idx4}`) to preserve continuity without silent fallback.

Ran:
- Reverse lookup verified with passing integration tests for representative indexed aliases.

## Slope Runtime Surfaces (SR02)

| canonical symbol | boundary alias form(s) | source seam | notes |
|---|---|---|---|
| `nelem` | `nelem` | SR02 | exact identity alias |
| `nwsofe` | `nwsofe` | SR02 | exact identity alias |
| `nslpts` | `nslpts`, `ofe{ofe}_nslpts` | SR02 | first-OFE identity + per-OFE indexed family |
| `slplen` | `slplen`, `ofe{ofe}_slplen` | SR02 | first-OFE identity + per-OFE indexed family |
| `avgslp` | `avgslp`, `ofe{ofe}_avgslp` | SR02 | first-OFE identity + per-OFE indexed family |
| `xinput` | `xinput_{idx4}`, `ofe{ofe}_xinput_{idx4}` | SR02 | first-OFE point family + per-OFE point family |
| `slpinp` | `slpinp_{idx4}`, `ofe{ofe}_slpinp_{idx4}` | SR02 | first-OFE point family + per-OFE point family |

## Soil Runtime Surfaces (SR03)

| canonical symbol | boundary alias form(s) | source seam | notes |
|---|---|---|---|
| `ntemp` | `ntemp` | SR03 | exact identity alias |
| `nsl` | `nsl`, `ofe{ofe}_nsl` | SR03 | first-OFE identity + per-OFE indexed family |
| `solthk` | `soil_profile_depth_m`, `solthk`, `ofe{ofe}_solthk`, `solthk_{idx4}`, `ofe{ofe}_solthk_{idx4}` | ARCH03 + SR03 | keeps ARCH03 semantic alias and adds SR03 runtime families |
| `dg` | `layer_thickness_m`, `dg`, `dg_{idx4}`, `ofe{ofe}_dg_{idx4}` | ARCH03 + SR03 | keeps ARCH03 semantic alias and adds SR03 runtime families |
| `thetdr` | `layer_theta_residual`, `thetdr`, `thetdr_{idx4}`, `ofe{ofe}_thetdr_{idx4}` | ARCH03 + SR03 | keeps ARCH03 semantic alias and adds SR03 runtime families |
| `thetfc` | `layer_theta_field_capacity`, `thetfc`, `thetfc_{idx4}`, `ofe{ofe}_thetfc_{idx4}` | ARCH03 + SR03 | keeps ARCH03 semantic alias and adds SR03 runtime families |
| `ssc` | `ssc`, `ssc_{idx4}`, `ofe{ofe}_ssc_{idx4}` | SR03 | saturated-conductivity runtime family |

## Template Token Policy

- `{ofe}`: positive integer OFE index (`>=1`).
- `{idx4}`: fixed-width 4-digit index segment (e.g. `0001`, `0002`).
- Unsupported template tokens are rejected with typed error `InvalidBoundaryAliasTemplate`.
