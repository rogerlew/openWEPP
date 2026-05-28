# WSHEDIMPL25 Channel Branch Payload Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Seam under review:
  - WS20-only opt-in (`ws20_case12_enable=1`, `ws21_case34_enable=0`) could
    previously route into residual unresolved-detachment fallback reporting lane
    (`ws20_detachment_unmigrated_segment_count > 0`).
- WS25 seam closure action:
  - Auto-activate WS21 migration lane whenever WS20 opt-in is enabled:
    `ws21_case34_enabled = ws20_case12_enabled || ws21_case34_opt_in`.
  - Preserve required fail-closed `crfrac` seam behavior when branch family is
    active.
- Expected seam behavior after WS25:
  1. WS20-only opt-in with missing `crfrac` fails closed.
  2. WS20-only opt-in with `crfrac` executes WS21 migrated branch path and no
     longer requires WS20 unresolved-detachment fallback accounting.

## Ran
- Verified by WS11 vectors:
  - `wshedimpl25_contract_ws20_only_opt_in_requires_crfrac_projection`
  - `wshedimpl25_contract_ws20_only_opt_in_auto_activates_ws21_with_crfrac_projection`
- Observed outcomes:
  - Missing-`crfrac` lane fails with `WKERNEL-WS10-CHANNEL-E-001`.
  - Seeded `crfrac` lane succeeds and reports
    `ws10_channel_1_ws20_detachment_unmigrated_segment_count == 0`.
