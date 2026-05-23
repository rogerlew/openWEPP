# WB15 Canopy Interception Authority and Guard Map

Status: `completed`
Evidence mode: `Static`

## Canonical WB15 Runtime Inputs
- Plant canopy drivers: `cancov`, `lai`, `vdmt`
- Hyetograph forcing: `ninten`/`nbrkpt`, `timem_####`, `intsty_####`
- Runoff closure symbols: `wb12_rainfall_input`, `wb12_runon_input`,
  `wb12_depression_storage_delta`, `wb12_runoff_observed`,
  `wb12_runoff_closure_tolerance`
- Storage closure symbols: `wb12_storage_initial`, `wb12_precip_input`,
  `S`, `Q`, `I`, `ET`, `D`, `Qd`, `wb12_storage_observed`,
  `wb12_storage_closure_tolerance`

## WB15 Runtime Algorithm Map
1. Compute hyetograph rainfall depth from runtime breakpoints.
2. Require canopy drivers and enforce domains:
   - `0 <= cancov <= 0.999`
   - `lai >= 0`
   - `0 <= vdmt <= 0.8`
3. Compute interception potential with Eq. [5.1.2] lineage proxy:
   - `VE = vdmt * 10000`
   - `Ipot = cancov * ((0.000627 * VE - 3.73349e-8 * VE^2) / 1000)`
4. Apply runtime interception:
   - `I = 0` when `cancov <= 0` or `lai <= 0`
   - `I = min(Ipot, hyetograph_rainfall)` otherwise
5. Scale hyetograph intensities by remaining liquid fraction and compute
   infiltration from the coupled rainfall signal.
6. Reconcile runoff with interception and snow coupling:
   - `Q = (hyetograph_rainfall - I) + S + runon - infiltration - depression_storage_delta`
7. Reconcile storage with interception as an explicit closure term:
   - `wb12_storage_reconciled = wb12_storage_initial + wb12_precip_input + S - I - Q - ET - D - Qd`

## Guard-Family and Error Mapping
- Runoff phase (`WB14` family):
  - missing required symbol: `HKERNEL-WB14-RUNOFF-E-001`
  - non-finite symbol: `HKERNEL-WB14-RUNOFF-E-002`
  - domain/closure violation: `HKERNEL-WB14-RUNOFF-E-003`
- Storage phase (`WB12` family):
  - missing required symbol: `HKERNEL-WB12-STORAGE-E-001`
  - non-finite symbol: `HKERNEL-WB12-STORAGE-E-002`
  - domain/closure violation: `HKERNEL-WB12-STORAGE-E-003`
- Success statuses:
  - runoff: `HKERNEL-WB14-RUNOFF-OK-001`
  - storage: `HKERNEL-WB12-STORAGE-OK-001`
