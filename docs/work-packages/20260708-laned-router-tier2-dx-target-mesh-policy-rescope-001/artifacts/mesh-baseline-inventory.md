# Mesh Baseline Inventory

Status: EXECUTED-COMPLETE
Evidence mode: Static + Ran.

## Active Mesh Surfaces

- Active production default: fixed `10 cells/OFE`.
- Diagnostic selector: `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M`.
- Diagnostic trace selector: `OPENWEPP_LANED_ACTIVE_TRACE=1`.
- Cell policy: `ceil(slplen_m / target_dx_m)`, bounded by
  `min_cells = 10`, `max_cells = 4096`.
- Time caps fixed across the ladder:
  `LANED_ACTIVE_SAMPLE_DT_S = 900`, `LANED_ACTIVE_MAX_DT_S = 300`.
- Shadow mesh remains separate fixed `LANED_SHADOW_CELLS = 10`; no shadow
  policy change landed.

## OFE Lengths And Candidate Counts

| Member | OFE shape | Baseline | `dx20` | `dx10` | `dx5` | `dx2p5` | `dx1p25` |
|--------|-----------|---------:|-------:|-------:|------:|--------:|---------:|
| `h2637` | 19 x 26.11 m | 10 | 10 | 10 | 10 | 11 | 21 |
| `mn_corn_h4` | 1 x 81.2 m | 10 | 10 | 10 | 17 | 33 | 65 |
| `n_idaho_forest_h1` | 1 x 300.0 m | 10 | 15 | 30 | 60 | 120 | 240 |
| `wa_cascades_forest_h1` | 5 x 108.34 m | 10 | 10 | 11 | 22 | 44 | 87 |

## Release Binary Provenance

From `artifacts/mesh-ladder-summary.md`:
- Build: `cargo build --release -p openwepp-runner --bins`
- Binary: `target/release/openwepp-cli-hill`
- SHA256:
  `9a4f9c2755723c2e312dea460ed714bb183e283968fef2f003cf7690a71d48b8`
- Git HEAD at run start: `ec82f061a18db352ce5efab52e1b04eed5de3701`

The ladder used package-local copied run directories and rewrote each runfile's
`[outputs]` paths into the package tree, preventing mutation of source fixture
outputs.
