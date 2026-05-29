# hillstab05-implementation-and-test-evidence

Status: complete  
Evidence mode: Ran

## Production Code Changes
- `crates/openwepp-input-contract/src/parsers/slope.rs`
  - added compatibility endpoint tolerance (`1e-3`) for near-terminal closure,
  - scoped cross-OFE continuity hard-fail branch to strict mode,
  - threaded parser mode through distance-mode derivation.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`
  - extended `derive_avgslp` with optional compatibility floor handling and
    floor-applied signaling.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`
  - added `SlopeRuntimeSurfaceOptions` (`strict`, `compatibility`),
  - added `build_hillslope_runtime_surface_from_slope_with_options`,
  - compatibility lane applies `avgslp` floor `0.000001` and emits explicit
    floor-applied symbols.
- `crates/openwepp-runner/src/hillslope/mod.rs`
  - switched compatibility execution path to slope runtime surface builder with
    compatibility options.

## Commands
```bash
cargo test --test infile_slope_parser_contract compatibility_mode_accepts_near_endpoint_terminal_distance
cargo test --test infile_slope_parser_contract compatibility_mode_accepts_cross_ofe_boundary_discontinuity
cargo test --test parser_runtime_seam_integration slope_runtime_surface_compatibility_floor_accepts_non_positive_avgslp_projection
cargo test --test infile_slope_parser_contract
cargo test --test parser_runtime_seam_integration
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
python docs/work-packages/20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/artifacts/hillstab01_stability_cohort.py \
  --openwepp-binary /home/workdir/openWEPP/target/release/openwepp-cli-hill \
  --cohort-seeds-csv /home/workdir/wepp-forest/docs/work-packages/20260503-wb05b-forest-hillslope-closure-sweep/artifacts/audits/_meta/defect_seeds.csv \
  --watchlist-csv /home/workdir/wepp-forest/docs/ablation/hillslope_watchlist.csv \
  --scratch-root /tmp/hillstab05 \
  --output-json /home/workdir/openWEPP/docs/work-packages/20260528-hillstab05-slope-residual-family-closure-001/artifacts/hillstab05-rerun-results.json
```

## Results
- Targeted new slope vectors: pass (`3/3`).
- Full updated integration files: pass
  - `infile_slope_parser_contract`: `20/20`,
  - `parser_runtime_seam_integration`: `48/48`.
- Required workspace gates: pass (`fmt`, `clippy`, `test`, `deny`).
- Release hillslope binary build: pass.
- Cohort rerun completed and emitted:
  - `artifacts/hillstab05-rerun-results.json`.
