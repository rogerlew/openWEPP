# CLIM07 Breakpoint Comparator Evidence

Status: `completed`
Evidence mode: `Ran`

## Vector
- Fixture: `tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_stmstr_nonzero.cli`
- Mode: `ibrkpt=1`
- Test: `clim07_breakpoint_vector_projects_expected_runtime_surface`

## Deterministic Assertions
- Hillslope seam:
  - `ibrkpt=1`, `nbrkpt=5`
  - `stmstr=4.8667 h`
  - `prcp=0.00735 m`
  - `stmdur=(23.9833-4.8667)*3600 s`
  - `timem_0001=0`
  - `timem_0002=(17.2667-4.8667)*3600 s`
  - `intsty_0005=0`
  - `mxint` equals max interval intensity reconstructed from breakpoint deltas.
- Comparator closure check:
  - reconstructed depth from `timem/intsty` series = `0.00735 m`.
- Watershed seam parity:
  - assignment `{21 -> climate}`
  - required `hs21_*` fields match hillslope breakpoint projections.

## Typed Guard Vector
- Test: `clim07_breakpoint_domain_violation_remains_typed_hard_fail`
- Mutation: duplicate breakpoint time (`timem[1] = timem[0]`)
- Expected and observed:
  - hillslope seam: `CLIM-RUNTIME-E-009`
  - watershed seam: `CLIM-RUNTIME-E-009`.

## Run Evidence
- `cargo test --test clim07_climate_comparator_and_closure_contract`
  - `clim07_breakpoint_vector_projects_expected_runtime_surface ... ok`
  - `clim07_breakpoint_domain_violation_remains_typed_hard_fail ... ok`
