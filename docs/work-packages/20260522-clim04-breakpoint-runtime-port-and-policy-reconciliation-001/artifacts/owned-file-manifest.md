# Owned File Manifest (CLIM04)

Evidence mode: `Static`
Status: `complete`

## CLIM04 Write Set
1. `crates/openwepp-input-contract/src/parsers/climate.rs`
- breakpoint strict policy alignment (`1500`, strict breakpoint-time monotonicity)
- compatibility controls extension (`allow_legacy_zero_drain_non_positive_dtime`)

2. `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- breakpoint runtime projection (`stmstr`, elapsed `timem`, `mxint`)
- breakpoint tests for WC1 fixtures and symbol/event-shape parity

3. `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- per-hillslope breakpoint runtime projection (`hs{ID}_stmstr`, elapsed `timem`, `mxint`)
- watershed breakpoint tests for WC1 fixtures and symbol/event-shape parity

4. `tests/integration/infile_climate_parser_contract.rs`
- parser policy boundary tests (`1500`/`1501`)
- strict duplicate-time rejection test
- explicit legacy compatibility-control test

5. `tests/fixtures/infile/climate/breakpoint_duplicate_timem.cli`
- duplicate-time compatibility-control fixture

6. `tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_stmstr_nonzero.cli`
- curated WC1 breakpoint fixture with non-zero first breakpoint hour

7. `tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_nbrkpt_42.cli`
- curated WC1 breakpoint fixture with `nbrkpt=42`

8. `docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- breakpoint policy text reconciliation to CLIM04 implementation

9. `docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/artifacts/*`
- CLIM04 contract, evidence, review, verification, and disposition records

## Explicitly Not Owned by This Package
- `docs/work-packages/README.md` (pre-existing local modification)
- `docs/work-packages/20260522-sr01-slope-soils-model-representation-discovery-001/` (separate package)
