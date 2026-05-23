# CLIM07 Comparator Vector Manifest

Status: `completed`
Evidence mode: `Static + Ran`

## Vector Inventory

| Vector ID | Mode / Surface | Fixture / Input | Deterministic checks | Executed by |
|---|---|---|---|---|
| `CLIM07-VEC-CD-001` | Continuous-daily (`ibrkpt=0`) hillslope runtime seam | `tests/fixtures/infile/climate/strict_valid.cli` day index `0` | `datver=5.3`, `iclig=1`, `prcp=0.01 m`, `stmdur=7200 s`, `ninten=11`, series closure depth `0.01 m` | `tests/integration/clim07_climate_comparator_and_closure_contract.rs` |
| `CLIM07-VEC-CD-002` | Continuous-daily watershed assignment seam (`hs{id}_*`) | `strict_valid.cli` assignment map `{1 -> climate}` | `nclimhs=1`, `hs1_*` projection parity with hillslope seam, depth closure `0.01 m` | `clim07_climate_comparator_and_closure_contract.rs` |
| `CLIM07-VEC-BP-001` | Breakpoint (`ibrkpt=1`) hillslope runtime seam | `tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_stmstr_nonzero.cli` | `stmstr=4.8667 h`, `nbrkpt=5`, elapsed `timem`, `mxint` closure, depth closure `0.00735 m` | `clim07_climate_comparator_and_closure_contract.rs` |
| `CLIM07-VEC-BP-002` | Breakpoint watershed assignment seam (`hs{id}_*`) | same breakpoint fixture assignment `{21 -> climate}` | `hs21_*` projection parity with hillslope breakpoint seam, depth closure `0.00735 m` | `clim07_climate_comparator_and_closure_contract.rs` |
| `CLIM07-VEC-BP-003` | Breakpoint typed-domain failure (duplicate `timem`) | runtime-mutated breakpoint day (`timem[1] = timem[0]`) | typed hard-fail `CLIM-RUNTIME-E-009` at hillslope and watershed seams | `clim07_climate_comparator_and_closure_contract.rs` |
| `CLIM07-VEC-TIER-001` | Confidence-tier routing metadata | comparator routing requests | daily -> `higher_confidence`; hourly/watershed -> `investigation`; missing metadata -> typed failure | `clim07_climate_comparator_and_closure_contract.rs`, `tests/integration/comparator_tier_routing_metadata.rs` |

## Execution Summary
Ran:
- `cargo test --test clim07_climate_comparator_and_closure_contract` -> pass (`4/4`).
- `cargo test --test comparator_tier_routing_metadata` -> pass (`5/5`).
