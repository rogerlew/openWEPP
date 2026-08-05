# Turbulent Input Implementation

Ran: contract-first and typed-boundary increment on 2026-08-05.

- `DirectSnowTurbulentGeometry::CLIGEN_V1` publishes distinct temperature,
  vapor-pressure, wind, and aerodynamic-roughness fields.
- `DirectSnowSurfaceEnergyOptions` carries that typed geometry across the real
  runner-to-hydrology boundary.
- The production runner binds the canonical CLIGEN geometry explicitly.
- No CoE ownership, melt, cold-content, routing, selector, or public-output
  behavior changes in this increment.

Ran:

- `tools/check_sc_binding_exposure.py --strict .../SC-SNOWENERGY-001.md` — PASS.
- `tools/release/check_sc_unit_compliance.py --path .../SC-SNOWENERGY-001.md` — PASS.
- `cargo check -p openwepp-hillslope-orchestrator -p openwepp-runner` — PASS.
- focused `cligen_virtual_instrument_geometry_is_contract_bound` — PASS.

The broader complete-flux computation and non-mutating shadow melt remain the
next implementation increment; this artifact does not claim carrier closure or
CoE retirement.
