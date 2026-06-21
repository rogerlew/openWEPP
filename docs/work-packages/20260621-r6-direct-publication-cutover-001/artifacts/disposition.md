# Disposition

Status: executed-hold.
Evidence mode: Static + Ran.

R6 execution was resumed after R5E closed and stopped after ledger promotion.

Final disposition: `HOLD-R6-DIRECT-PUBLICATION-FRAME-ABSENT`.

## Evidence

Ran:

- R5E completion evidence was read from
  `docs/work-packages/20260621-r5e-full-ofe-day-endpoint-readiness-001/` after
  pushed commit `d8f6bbea`.
- `markdown-doc lint --path docs/architecture/array-native-runtime-specification.md --format json`
  passed after ledger promotion.
- `rg -n "PublicationFrame|publication\\.manifest|publication\\.runoff|publication\\.storage|publication\\.loss|DirectPublication|direct publication|PublicationOperands" crates docs/architecture/array-native-runtime-specification.md --glob '!target/**'`
  found canonical ledger/spec references plus the existing
  `DirectPublicationFrame`, but no production run-bound R6 publication frame
  carrying the promoted output ledger.
- `rg -n "fn build_hbp_output|runtime_surface: &HillslopeWritebackSurface|build_hillslope_wat_rows\\(&execution\\.wb13_rows|write_hillslope_pass_parquet\\(|build_loss_output_json\\(|write_hillslope_run_manifest\\(" crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
  confirmed the public output path still builds from compatibility WB13 rows
  and runtime surfaces.

Static:

- `docs/architecture/array-native-runtime-specification.md` section `5.2.1`
  is now canonical publication ledger authority.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` defines
  `DirectPublicationFrame` with only `runoff_m`, `infiltration_m`,
  `evapotranspiration_m`, `drainage_m`, and `lateral_flow_m`.
- `select_direct_runtime_skeleton_once` constructs `DirectRunFrame::skeleton`
  and records the declared compatibility edge; it does not bind a real
  direct publication frame to runner outputs.

## Outcome

No production Rust, output writer, schema, contract, or benchmark work occurred
after ledger promotion. R6 remains blocked until a direct publication frame is
constructed from typed direct run/lane/day state and can supply the promoted
ledger operands without reading compatibility WB13 rows, runtime symbols,
writeback payloads, stale logical state, or diagnostic compatibility ledgers.
