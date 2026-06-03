# Implementation and Test Evidence

Status: completed

Evidence mode: mixed

## Implementation

- Static: `Hphys0245TraceRow` now includes:
  `wb17_upi_layers_m`, `wb17_ui_layers_m`, `wb18_thetdr_layers`,
  `wb18_dg_layers_m`, `wb18_frozen_depth_layers_m`,
  `wb18_recomputed_soil_water_m`, and
  `wb18_recomputed_minus_wb11_m`.
- Static: trace schema is bumped to
  `openwepp-hphys0245-wb11-wb18-wb19-wb17-storage-trace-v4`.
- Static: trace population is additive and opt-in through existing
  `OPENWEPP_HPHYS0245_TRACE_*` controls; no hydrology equations changed.
- Static: aggregate trace recomputation follows
  `Σ(wb18_perc_theta_i + thetdr_i*(dg_i - frozen_i))`, using preferred
  `wb19_*` layer aliases with legacy fallback aliases for diagnostic
  compatibility.

## Focused Evidence

- Ran: HPHYS0260 trace-row test passed.
- Ran: HPHYS0245 trace-writer JSON serialization test passed and includes the
  new schema fields.
- Ran: `python -m py_compile` passed for `hphys0260_diagnostics.py`.
