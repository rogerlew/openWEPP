# Implementation and Test Evidence

Status: complete

Evidence mode: ran

Ran:

- `cargo fmt`
- Focused post-patch tests listed in `contract-test-implementation-evidence.md`.
- Full gate chain listed in `gate-results.md`.
- HPHYS0254 diagnostics: `/workdir/wepppy/.venv/bin/python docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/hphys0254_diagnostics.py --run-root /tmp/hphys0254_20260602T220046Z`

Static:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs` now publishes normalized hydrology seed aliases (`wb11_nsl`, `wb19_dg_####`, `wb19_solthk_####`, `wb19_thetfc_####`, `wb19_thetdr_####`, `wb19_por_####`, `wb19_coca_####`, and WB18 percolation aliases) while preserving generic parser-layer symbols.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs` and `03_kernel_support_01_kernel_phases.rs` now prefer hydrology aliases with legacy fallback where required.
- `crates/openwepp-runner/src/hillslope/mod.rs` now derives WB11/WB13 seed storage from hydrology aliases.
- WB18 lower-layer saturation-ratio handling now uses the baseline `stu >= 0.95` cap path rather than hard failing finite over-UL lower-layer ratios.

Result:

- Candidate H1 WAT/trace day-1 accounting remains internally closed.
- Candidate post-seed WB11 storage now aligns with the baseline inferred t=0 proxy to within diagnostic tolerance.
