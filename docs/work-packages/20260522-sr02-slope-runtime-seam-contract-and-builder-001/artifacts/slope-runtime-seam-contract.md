# Slope Runtime Seam Contract

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Defined hillslope-owned slope parser-to-runtime seam in `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`.
- Preserved canonical symbol continuity for `slplen`, `nslpts`, `xinput`, `slpinp`, `avgslp` with explicit first-OFE alias mapping.
- Derived `avgslp` at runtime using legacy `profil.for` trapezoidal rule shape (`/workdir/wepp-forest_260430_baseline/src/profil.for:37-51`) with typed non-positive/non-finite rejection rather than silent clamping.

Ran:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Contract Boundary

- seam id: `HS-SLP-SEAM-001`
- owner: `openwepp-hillslope-orchestrator::runtime_inputs`
- input type: `openwepp_input_contract::parsers::slope::SlopeProfile`
- output type: `openwepp_hillslope_orchestrator::HillslopeWritebackSurface`
- projection function: `build_hillslope_runtime_surface_from_slope`

Boundary policy:
- No silent fallback/default behavior for missing or malformed required slope surfaces.
- Numeric and closure violations produce typed `HillslopeRuntimeInputError` with stable error codes.
- Canonical-first symbol continuity retained; runtime aliases are additive, not replacements.

## Symbol Projection Rules

| Canonical source symbol | Runtime symbol(s) | Rule |
|---|---|---|
| `nelem` / `nwsofe` | `nelem`, `nwsofe` | projected from `SlopeProfile.ofe_count` (lossless `u32` conversion required) |
| `nslpts` (OFE i) | `ofe{i}_nslpts` | projected from each OFE `nslpts`; typed mismatch guard against `points.len()` |
| `slplen` (OFE i) | `ofe{i}_slplen` | projected from each OFE `slplen`; finite and `>0` required |
| `xinput` (OFE i, point j) | `ofe{i}_xinput_{j:04}` | projected pointwise; finite and monotone non-decreasing required |
| `slpinp` (OFE i, point j) | `ofe{i}_slpinp_{j:04}` | projected pointwise; finite required |
| derived `avgslp` (OFE i) | `ofe{i}_avgslp` | computed as trapezoidal integral over `(xinput, slpinp)` divided by terminal `xinput` |
| first-OFE canonical continuity aliases | `nslpts`, `slplen`, `avgslp`, `xinput_{j:04}`, `slpinp_{j:04}` | explicit alias projection for current hillslope-kernel primary-OFE runtime path |

`avgslp` derivation rule (legacy-shape parity):
- `slen = xinput(last)`
- `top_elevation = Σ[(x_{k+1} - x_k) * (slpinp_k + slpinp_{k+1}) / 2]`
- `avgslp = top_elevation / slen`
- guard: `avgslp` must be finite and `>0`

## Error/Guard Taxonomy

All failures emit `HillslopeRuntimeInputError` with stable IDs:

- `HS-RUNTIME-E-011`: missing slope OFE blocks
- `HS-RUNTIME-E-012`: declared `ofe_count` mismatch with observed OFEs
- `HS-RUNTIME-E-013`: OFE count not losslessly representable
- `HS-RUNTIME-E-014`: per-OFE declared `nslpts` mismatch with observed points
- `HS-RUNTIME-E-015`: `nslpts` not losslessly representable
- `HS-RUNTIME-E-016`: insufficient slope points (`<2`)
- `HS-RUNTIME-E-017`: non-finite `slplen`
- `HS-RUNTIME-E-018`: non-positive `slplen`
- `HS-RUNTIME-E-019`: non-finite `xinput`
- `HS-RUNTIME-E-020`: non-finite `slpinp`
- `HS-RUNTIME-E-021`: non-monotone `xinput`
- `HS-RUNTIME-E-022`: non-finite derived `avgslp`
- `HS-RUNTIME-E-023`: non-positive derived `avgslp`
- `HS-RUNTIME-E-024`: non-finite derived terminal slope length (`xinput(last)`)
- `HS-RUNTIME-E-025`: non-positive derived terminal slope length (`xinput(last)`)

## Implementation Anchors

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:439`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:760`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:840`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:206`
