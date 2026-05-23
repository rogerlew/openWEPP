# WB14 Infiltration and Hyetograph Kernel Authority and Guard Map

Status: `completed`
Evidence mode: `Static`

## Canonical WB14 Runtime Inputs
- Hyetograph cardinality/state: `ninten` and/or `nbrkpt`
- Hyetograph breakpoints: `timem_0001..` and `intsty_0001..`
- Soil/infiltration controls: `ssc`, `dg`, `thetdr`, `thetfc`
- Runoff reconciliation coupling: `wb12_rainfall_input`, `wb12_runon_input`, `wb12_depression_storage_delta`, `wb12_runoff_observed`, `wb12_runoff_closure_tolerance`

## Runtime Algorithm Map
1. Resolve hyetograph point count from `ninten`/`nbrkpt` and enforce cardinality consistency.
2. Load `timem_####`/`intsty_####` pairs and enforce finite, non-negative, strictly monotone time breakpoints.
3. Compute moisture deficit and Green-Ampt capillary term:
- `delta_theta = max(thetfc - thetdr, 0)`
- `psi_f = dg * delta_theta`
4. Integrate each hyetograph interval:
- `delta_t = timem[i+1] - timem[i]`
- `rain_depth_i = intsty[i] * delta_t`
- Compute interval infiltration using deterministic branch rules:
  - no ponding branch (`intsty <= ssc`): infiltration equals interval rainfall depth
  - ponding branch (`intsty > ssc`): use Green-Ampt threshold and implicit cumulative solve
5. Sum interval infiltration to cumulative infiltration `F`.
6. Reconcile runoff:
- `Q = rainfall + runon - F - depression_storage_delta`
- enforce closure delta against tolerance and emit typed status.

## Guard-Family and Error Mapping
- Runoff phase class maps to `WB14` guard family.
- Missing required symbols: `HKERNEL-WB14-RUNOFF-E-001`
- Non-finite symbols: `HKERNEL-WB14-RUNOFF-E-002`
- Domain/invariant violations (including malformed hyetograph ordering and closure violations): `HKERNEL-WB14-RUNOFF-E-003`
- Success: `HKERNEL-WB14-RUNOFF-OK-001`
