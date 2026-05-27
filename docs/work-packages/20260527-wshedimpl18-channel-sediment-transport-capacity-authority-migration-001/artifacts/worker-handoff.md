# WSHEDIMPL18 Worker Handoff

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- WSHEDIMPL18 scope is complete:
  - WS10 channel sediment `tc` publication migrated from surrogate identity
    (`tc = qsed`) to class-aware transport-capacity computation using baseline
    `shield`/`trncap` lineage and hydraulic coupling.
  - Runtime contributor payload ingress now uses structured class payload
    aggregation (`mass_kg`, `particle_flow_fraction`, `particle_diameter_m`) to
    drive `tc` branch calculations.
  - WS11 contract vectors now enforce transport-capacity behavior:
    - `tc` no longer collapses to `qsed`,
    - `tc` responds to class-diameter perturbations.
- Canonical contract/index posture updated for WSHEDIMPL18 closure:
  - `SC-ROUTE-001` revision `20`
  - `SC-SED-001` revision `19`
  - `SC-SYSTEM-001` revision `41`
- Residual blockers remain open and non-promotable:
  - `GAP-SYSTEM-008`
  - `GAP-ROUTE-009`
  - `GAP-SED-006`

### Immediate next actions
- Prepare and execute follow-on package for remaining channel segment-loop
  families (`case12/case34/detach/dcap/enddet`) and full `chnero/chnrt`
  inflow-partition parity closure using landed WS15/WS16/WS17/WS18 seams.
- Add contract-derived WS11 vectors for remaining detachment/deposition branch
  families once migrated.
- Rerun watershed baseline-authoritative comparator lane after remaining
  process-family migration and publish updated GO/HOLD disposition.

## Ran
- Full workspace validation gates executed and passed (see `gate-results.md`).
