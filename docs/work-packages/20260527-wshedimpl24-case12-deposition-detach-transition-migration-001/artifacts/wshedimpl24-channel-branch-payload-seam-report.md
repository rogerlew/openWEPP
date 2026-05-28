# WSHEDIMPL24 Channel Branch Payload Seam Report

Status: complete
Evidence mode: static
Date: 2026-05-27

## Static
- WS24 transition seam depends on WS22 fail-closed class-fraction projection:
  - Required class-fraction symbols per active class:
    - `ws10_channel_{id}_crfrac_{class:04}`
- Seam validation behavior:
  - missing `crfrac` symbol -> typed hard failure
    (`WKERNEL-WS10-CHANNEL-E-001`, `MissingRequiredInput`)
  - non-finite/out-of-domain `crfrac` values -> typed hard failure
    (`WKERNEL-WS10-CHANNEL-E-002/003`)
- Active lane without `crfrac` projection remains fail-closed instead of
  unresolved continuation.
- WS24 seam closure:
  - WS20 `case12` branch now executes baseline-authoritative continuation into
    detach-capacity closure when `xdemax < x(i)`.
  - Transition publication is explicit via
    `ws10_channel_{id}_ws24_case2_detach_segment_count`.
- Residual program seam gap remains out-of-scope:
  - full `chnero/chnrt` parity families tracked by
    `GAP-ROUTE-009` / `GAP-SED-006` / `GAP-SYSTEM-008`.
