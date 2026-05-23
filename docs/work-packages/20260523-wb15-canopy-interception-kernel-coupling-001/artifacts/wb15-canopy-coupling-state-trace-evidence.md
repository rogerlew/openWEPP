# WB15 Canopy-Coupling State-Trace Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Nominal Coupled Trace (from WB15 contract vector)
Input highlights:
- `cancov=0.65`, `lai=2.1`, `vdmt=0.6`
- hyetograph rainfall = `3.0`
- `runon=0.2`, `depression_storage_delta=0.2`, `S=0.0`

Derived interception:
- `VE = vdmt * 10000 = 6000`
- `Ipot = cancov * ((0.000627*VE - 3.73349e-8*VE^2)/1000)`
- `I = 0.00157166334`

Coupled runoff/infiltration outputs:
- liquid after interception = `2.99842833666`
- `wb12_infiltration = 2.99842833666`
- `Q = 0.0`

Observed runtime outputs (test assertions):
- `I` present in flux surface and equals `0.00157166334`
- `wb12_infiltration` equals `2.99842833666`
- `Q` equals `0.0`
