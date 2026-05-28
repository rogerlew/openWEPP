# WSHEDIMPL23 Channel Branch Payload Seam Report

Status: complete
Evidence mode: static
Date: 2026-05-27

## Static
- WS21 opt-in seam requirement remains explicit from WSHEDIMPL22:
  - Required class-fraction symbols per active class:
    - `ws10_channel_{id}_crfrac_{class:04}`
- Seam validation behavior:
  - missing `crfrac` symbol -> typed hard failure
    (`WKERNEL-WS10-CHANNEL-E-001`, `MissingRequiredInput`)
  - non-finite/out-of-domain `crfrac` values -> typed hard failure
    (`WKERNEL-WS10-CHANNEL-E-002/003`)
- Active lane without `crfrac` projection remains fail-closed instead of
  unresolved continuation.
- WS23 seam closure:
  - WS21 `case4 -> detach` iterative closure branch (`nt < cnpart`) now uses
    baseline-authoritative iterative closure execution and no longer requires
    unresolved-detachment diagnostics for this branch.
- Residual program seam gap remains out-of-scope:
  - full `chnero/chnrt` parity families tracked by
    `GAP-ROUTE-009` / `GAP-SED-006` / `GAP-SYSTEM-008`.
