# CLIM07 Continuous-Daily Comparator Evidence

Status: `completed`
Evidence mode: `Ran`

## Vector
- Fixture: `tests/fixtures/infile/climate/strict_valid.cli`
- Mode: `ibrkpt=0`
- Test: `clim07_continuous_daily_vector_projects_expected_runtime_surface`

## Deterministic Assertions
- Hillslope seam:
  - `datver=5.3`
  - `iclig=1`
  - `itemp=1`
  - `ibrkpt=0`
  - `prcp=0.01 m`
  - `stmdur=7200 s`
  - `timep=0.25`
  - `ip=2.1`
  - `ninten=11`
  - `timem_0001=0`, `timem_0011=7200`, `intsty_0011=0`
- Comparator closure check:
  - reconstructed depth from `timem/intsty` series = `0.01 m`.
- Watershed seam parity:
  - assignment `{1 -> climate}`
  - `nclimhs=1`
  - required `hs1_*` fields project with the same closure depth `0.01 m`.

## Run Evidence
- `cargo test --test clim07_climate_comparator_and_closure_contract`
  - `clim07_continuous_daily_vector_projects_expected_runtime_surface ... ok`
