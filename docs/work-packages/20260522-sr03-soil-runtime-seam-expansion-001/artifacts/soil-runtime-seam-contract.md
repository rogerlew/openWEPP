# Soil Runtime Seam Contract

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Defined a first-class hillslope-owned soil parser-to-runtime seam in `openwepp-hillslope-orchestrator::runtime_inputs`.
- Expanded projection from minimal seed (`solthk`, `dg`, `thetdr`, `thetfc`) to OFE/layer indexed runtime surfaces plus canonical first-OFE aliases.
- Added strict typed guards for OFE/layer closure, depth monotonicity, and required hydraulic inputs (`ksat_mm_h`).

Ran:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Contract Boundary

- seam id: `HS-SOIL-SEAM-001`
- owner: `openwepp-hillslope-orchestrator::runtime_inputs`
- input type: `openwepp_input_contract::parsers::soil::SoilProfile`
- output type: `openwepp_hillslope_orchestrator::HillslopeWritebackSurface`
- projection function: `build_hillslope_runtime_surface_from_soil`

Boundary policy:
- No silent defaults for required parser fields (`theta_r_rosetta`, `fc_rosetta`, `ksat_mm_h`).
- Shape closure is explicit (`ntemp == ofes.len()`, `nsl == layers.len()`).
- Numeric/domain violations fail with typed `HillslopeRuntimeInputError` and stable error codes.
- Canonical continuity is preserved with additive aliases (indexed symbols + first-OFE aliases).

## Symbol Projection Rules

| Canonical source symbol | Runtime symbol(s) | Rule |
|---|---|---|
| `ntemp` | `ntemp` | Lossless count projection from `SoilProfile.ntemp` after closure check against observed OFEs. |
| `nsl` (OFE `i`) | `ofe{i}_nsl`; first-OFE alias `nsl` | Per-OFE layer count projection after `nsl == layers.len()` guard. |
| `solthk` profile depth (OFE `i`) | `ofe{i}_solthk`; first-OFE alias `solthk` | Last cumulative layer depth (`depth_mm`) converted to meters. |
| `solthk` layer cumulative depth (OFE `i`, layer `j`) | `ofe{i}_solthk_{j:04}`; first-OFE alias `solthk_{j:04}` | Layer cumulative depth in meters; requires positive strictly increasing depth sequence. |
| `dg` layer thickness (OFE `i`, layer `j`) | `ofe{i}_dg_{j:04}`; first-OFE alias `dg_{j:04}` and top-layer alias `dg` | Differential thickness `(depth_j - depth_{j-1})/1000`. |
| `theta_r_rosetta` (OFE `i`, layer `j`) | `ofe{i}_thetdr_{j:04}`; first-OFE alias `thetdr_{j:04}` and top-layer alias `thetdr` | Required, finite. |
| `fc_rosetta` (OFE `i`, layer `j`) | `ofe{i}_thetfc_{j:04}`; first-OFE alias `thetfc_{j:04}` and top-layer alias `thetfc` | Required, finite. |
| `ksat_mm_h` (OFE `i`, layer `j`) | `ofe{i}_ssc_{j:04}`; first-OFE alias `ssc_{j:04}` and top-layer alias `ssc` | Required, finite, positive; converted `mm/h -> m/s` by `/3.6e6`. |

## Error/Guard Taxonomy

Legacy soil-seed guards retained:
- `HS-RUNTIME-E-001..010` (missing primary OFE/layer, missing theta fields, primary-depth/domain guards).

New SR03 seam expansion guards:
- `HS-RUNTIME-E-026`: declared `ntemp` vs observed OFE mismatch
- `HS-RUNTIME-E-027`: OFE count conversion out-of-range
- `HS-RUNTIME-E-028`: declared `nsl` vs observed layer-row mismatch
- `HS-RUNTIME-E-029`: layer-count conversion out-of-range
- `HS-RUNTIME-E-030`: non-finite layer depth
- `HS-RUNTIME-E-031`: non-positive layer depth
- `HS-RUNTIME-E-032`: non-monotone layer depth
- `HS-RUNTIME-E-033`: missing saturated conductivity (`ksat_mm_h`)
- `HS-RUNTIME-E-034`: non-finite saturated conductivity
- `HS-RUNTIME-E-035`: non-positive saturated conductivity

## Implementation Anchors

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:514`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:573`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:681`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1245`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:52`
