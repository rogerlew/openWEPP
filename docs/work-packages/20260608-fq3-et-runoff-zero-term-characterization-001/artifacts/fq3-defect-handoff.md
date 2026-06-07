# FQ3 Defect Handoff

Evidence mode: Static (routing from Ran characterization)

## Routed Follow-On 1

- Proposed ID: `FQ3-DC-ET-CORN-ENGAGEMENT-001`
- Defect family: annual-crop ET engagement missing
- Trigger evidence:
  - Corn `Ep`: 36/36 prefixes `defect-openwepp-zero-legacy-nonzero`
  - Tah_4899 `Ep`: 6/6 prefixes `both-nonzero`
- Authority envelope:
  - Contracts: `SC-EVAP-001` (+ canopy/interception contract surfaces as applicable)
  - Input family: `/wc1/runs/al/algebraic-radium` Corn management set
  - In-scope source surfaces: crop-growth / canopy / transpiration path engagement and ET partition wiring
  - Protected boundaries: snow magnitude, p11 percolation package ownership
- Acceptance target:
  - Comparator-flagged Corn `Ep` zero-term defect eliminated without regressing Tah_4899 ET behavior

## Routed Follow-On 2

- Proposed ID: `FQ3-DC-RUNOFFPART-QQOFE-001`
- Defect family: runoff partition/output underproduction (Q/QOFE)
- Trigger evidence:
  - `Q`: 35/42 prefixes `defect-openwepp-zero-legacy-nonzero`
  - `QOFE`: 35/42 prefixes `defect-openwepp-zero-legacy-nonzero`
  - All Tah_4899 cases are defective for both `Q` and `QOFE`
  - Seven Corn cases (`p4,p25,p33,p35,p38,p40,p42`) are nonzero but still materially below legacy
- Authority envelope:
  - Contract: `SC-RUNOFFPART-001`
  - In-scope source surfaces: runoff partitioning/production pathway and Q/QOFE publication coupling
  - Protected boundaries: no comparator-oracle acceptance; enforce contract-first closure
- Acceptance target:
  - Remove openWEPP zero/near-zero runoff defect shape while preserving ET and water-balance invariants

## Follow-On Dependency

- Interception comparator availability gap:
  - Legacy WAT comparator surface omits interception term.
  - Before an interception-specific DC close/fail decision, select an authority-approved comparator surface that includes interception, or define contract-first acceptance independent of legacy WAT term parity.
