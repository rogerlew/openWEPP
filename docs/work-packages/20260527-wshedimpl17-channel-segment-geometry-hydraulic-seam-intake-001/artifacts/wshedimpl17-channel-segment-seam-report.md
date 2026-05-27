# WSHEDIMPL17 Channel Segment Seam Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- WS17 seam families projected at runtime:
  - `ws10_channel_{id}_nslpts`
  - `ws10_channel_{id}_x_{point:04}`
  - `ws10_channel_{id}_slope_{point:04}`
  - `ws10_channel_{id}_depa_{point:04}`
  - `ws10_channel_{id}_depb_{point:04}`
  - `ws10_channel_{id}_wida_{point:04}`
  - `ws10_channel_{id}_widb_{point:04}`
- Projection behavior:
  - slope OFE profile selection is channel-id ordered (`channel_id -> ofe[id-1]`).
  - normalized slope `xinput` values are converted with `slplen`.
  - segment hydraulic depth scaffold uses channel `chnedm` converted to feet.
  - segment hydraulic width scaffold uses slope `fwidth` converted to feet.
- Fail-closed guards:
  - missing profile coverage, undersized profile cardinality, non-monotone
    segment `x`, and domain violations reject seeding in runtime-input path.
  - WS10 kernel rejects missing/non-finite/out-of-domain WS17 scaffold symbols
    before channel execution using existing typed guard-family IDs.
- Residual blockers remain unchanged:
  - `GAP-SYSTEM-008`
  - `GAP-ROUTE-009`
  - `GAP-SED-006`

## Ran
- `cargo test --workspace` passed and includes WS17 runtime-input unit vectors
  and WS11 fail-closed scaffold guard vector.
