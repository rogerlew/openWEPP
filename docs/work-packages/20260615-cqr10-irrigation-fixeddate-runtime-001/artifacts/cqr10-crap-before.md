# CQR10 CRAP Before

Status: complete.

Static: target file:
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`.

Static: before source line count was `902`.

Static: before suppression census in the target file:

- line 51: `#[allow(clippy::too_many_lines)]` on depletion projection.
- line 341: `#[allow(clippy::too_many_lines)]` on fixed-date projection.
- line 725: `#[allow(clippy::too_many_lines)]` on frost projection.

Ran: before CRAP was generated with:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001/artifacts/crap_before.json
```

Ran: command exited `0` with the known `125 source files had no matching entry
in the LCOV report` warning.

Target-file rows before refactor:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `seed_hillslope_runtime_surface_from_irrigation_fixeddate` | 342 | 38.0 | 0.0 | 1482.0 |
| `seed_hillslope_runtime_surface_from_irrigation_depletion` | 52 | 33.0 | 0.0 | 1122.0 |
| `seed_hillslope_runtime_surface_from_frost` | 726 | 18.0 | 86.11111111111111 | 18.868055555555554 |
| `seed_hillslope_runtime_surface_from_snow` | 615 | 11.0 | 78.68852459016394 | 12.171186134522273 |
| `build_hillslope_runtime_surface_from_irrigation_depletion` | 36 | 2.0 | 0.0 | 6.0 |
| `build_hillslope_runtime_surface_from_irrigation_fixeddate` | 326 | 2.0 | 0.0 | 6.0 |
| `build_hillslope_runtime_surface_from_snow` | 7 | 2.0 | 100.0 | 2.0 |
| `build_hillslope_runtime_surface_from_frost` | 21 | 2.0 | 100.0 | 2.0 |
| `snow_runtime_boundary_value` | 697 | 1.0 | 42.10526315789473 | 1.194051611022015 |

Live target identity: `seed_hillslope_runtime_surface_from_irrigation_fixeddate`
at line 342, CRAP `1482.0`, CC `38.0`, coverage `0.0%`.
