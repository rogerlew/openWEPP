# Implementation/Test Evidence

Status: completed/HOLD
Evidence mode: ran

Static: production changes implemented in the HPHYS0269 slice.

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - Added `rain_retained_m` and `melt_raw_m` hourly snow state.
  - Preserved signed raw melt from the melt equation instead of clamping at the equation boundary.
  - Added rain-on-snow holding-capacity accounting for sub-`350 kg m^-3` snowpack.
  - Added mass-closed daily negative-melt redistribution for routed melt and runtime SWE closure.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - Subtracts retained rain from direct liquid forcing.
  - Publishes `snow.hourly.rain_retained_m_####` and `snow.hourly.melt_raw_m_####`.
- `crates/openwepp-runner/src/hillslope/mod.rs`
  - Bumped HPHYS trace schema to `v8`.
  - Adds retained-rain/raw-melt sums and updates snow closure error to `S - (melt - snowfall - rain_retained)`.

Ran:

- `cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture`
- Result: pass, `8 passed; 0 failed`.
- `cargo test -p openwepp-runner hphys0268_trace_row_captures_spring_snowpack_lineage --lib -- --nocapture`
- Result: pass, `1 passed; 0 failed`.
