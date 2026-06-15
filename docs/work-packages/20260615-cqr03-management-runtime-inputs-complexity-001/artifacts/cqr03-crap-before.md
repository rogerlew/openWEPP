# CRAP Before

Ran: `cargo crap --workspace --lcov docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/crap_before.json`

Exit code: `0`.

Raw artifact: `crap_before.json`.

Top target-module rows:

| Function | Line | CC | Coverage % | CRAP |
|---|---:|---:|---:|---:|
| `build_hillslope_pl_runtime_surfaces_from_management` | 39 | 128.0 | 79.33526011560693 | 272.5805153120575 |
| `apply_primary_initial_live_canopy_assimilation` | 961 | 35.0 | 73.02631578947368 | 59.041241719365075 |
| `legacy_initial_residue_depth_m` | 908 | 7.0 | 89.65517241379311 | 7.054245766534093 |
| `projection_usize_from_surface` | 1168 | 4.0 | 44.44444444444444 | 6.743484224965707 |
| `legacy_residue_depth_conversion_factor` | 944 | 5.0 | 72.72727272727273 | 5.507137490608565 |
| `projection_f64_from_surface` | 1142 | 2.0 | 50.0 | 2.5 |
| `build_hillslope_runtime_surface_from_management` | 1202 | 2.0 | 100.0 | 2.0 |
| `HillslopePlRuntimeSurfaces::merged_state_surface` | 22 | 1.0 | 100.0 | 1.0 |

Disposition: two eligible functions exceeded the package target before the
refactor.
