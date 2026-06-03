# Contract-Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

Static:

- Added HPHYS0268 trace fields to `Hphys0245TraceRow` in `crates/openwepp-runner/src/hillslope/mod.rs`.
- Added `hphys0268_trace_row_captures_spring_snowpack_lineage` to assert runtime SWE/depth/density, snowfall water equivalent, signed-`S` closure, and WB13 `P`/`RM`/`Snow-Water` capture.
- Added `hphys0268_diagnostics.py` to classify H1/H7/H39 first material `|Ep diff| > 1 mm` snowpack lineage.

Ran:

- `python3 -m py_compile docs/work-packages/20260603-hphys0268-spring-snowpack-melt-wiring-closure-001/artifacts/hphys0268_diagnostics.py` passed.
- `cargo test -p openwepp-runner hphys0268_trace_row_captures_spring_snowpack_lineage --lib` passed.
- `cargo test --test clim05_snow_runtime_kernel_contract` passed: 6 passed.
