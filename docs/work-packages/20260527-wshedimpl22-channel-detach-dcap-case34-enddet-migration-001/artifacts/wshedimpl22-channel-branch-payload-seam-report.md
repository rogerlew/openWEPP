# WSHEDIMPL22 Channel Branch Payload Seam Report

Status: complete
Evidence mode: static
Date: 2026-05-27

## Static
- WS22 introduces an explicit WS21 opt-in seam requirement:
  - Required class-fraction symbols per active class:
    - `ws10_channel_{id}_crfrac_{class:04}`
- Seam validation behavior:
  - missing `crfrac` symbol -> typed hard failure
    (`WKERNEL-WS10-CHANNEL-E-001`, `MissingRequiredInput`)
  - non-finite/out-of-domain `crfrac` values -> typed hard failure
    (`WKERNEL-WS10-CHANNEL-E-002/003`)
- Active lane without `crfrac` projection is now fail-closed instead of
  silently falling back to unresolved continuation.
- Residual seam gap remains:
  - WS21 `case4 -> detach` iterative closure branch is still explicitly
    unresolved and emits diagnostics ownership in hold posture.
