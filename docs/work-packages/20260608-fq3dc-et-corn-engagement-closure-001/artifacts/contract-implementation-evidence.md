# Contract Implementation Evidence

Status: complete

Evidence mode: Static.

## Contract Edits

- `SC-PLANT-001` v18:
  - Added `INV-PLANT-026`, requiring annual pre-plant skips to be day-local and
    forbidding deletion of PL schedule/runtime sentinels needed for later
    `jdplt` activation.
  - Added guard-map and vector-family coverage for annual PL activation
    persistence.
- `SC-EVAP-001` v26:
  - Tightened `INV-EVAP-016` so annual pre-plant ET skips cannot erase PL
    sentinels needed for later plant/canopy/transpiration engagement.
- `SC-WATBAL-001` v147:
  - Added pinned-baseline `idat.for` interception provenance.
  - Specified `VE_raw = vdmt * 10000` and `VE = min(VE_raw, 8000)` for the
    WB15 interception equation input only.
  - Required finite non-negative `vdmt`; no upper cap on plant live-mass state.
- `SC-RUNOFFPART-001` v40:
  - Mirrored WB15 interception biomass input semantics where runoff closure
    consumes the interception result.

## Authority Notes

The WATBAL/RUNOFFPART contract changes are guard-domain corrections for a
plant-interception consumer. They do not tune runoff partitioning, alter Q
acceptance, or cross the package's protected runoff boundary.

The package objective mentioned `Er`, but upstream FQ-3 evidence classifies
`Er=0` as expected-config-zero with legacy `Er=0`. No `Er` production change was
authorized or needed for closure.
