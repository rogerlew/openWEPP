# Review Agent A

Status: complete

Evidence mode: static

Reviewer: Kepler (`rust_code_reviewer`)

Summary: directionally sound. Contracts and runner correctly remap canonical
`hrsnow` to snowfall depth, not water equivalent, and stale `winter.for:410-412`
authority is corrected to `winter.for:296-300` calling `stmtim.for:43-95`.
No production migration from the old mismatch is present.

Findings:

- MEDIUM: negative-melt continuation routing was overstated for the H7
  first-2013 row. The row is `OPENWEPP-DEFECTIVE`, has
  `baseline_negative_raw_melt_sum_mm = 0.0`, raw melt within tolerance, and
  routed melt delta `11.698650 mm`; it supports follow-on post-raw routed-melt
  diagnosis, not acceptance of corrected negative-melt authority.
- MEDIUM: diagnostic runner had silent trace-default risk because HPHYS0298
  helpers return `0.0`/`[]` for missing or malformed trace data while the local
  required-field check only tested key presence/non-`None`.

Required fixes:

- Split `next_action_for` for `negative-melt-correction` so corrected
  negative-melt authority is retained only for proven `LEGACY-DEFECTIVE`
  material negative-melt cases.
- Add finite/type validation for required trace fields before classification,
  especially hourly maps, and fail closed as `trace-gap`.

Verification notes:

- Canonical mapping is correct: baseline `stmtim.for` line 94 is depth,
  openWEPP depth summary comes from `snow.hourly.snowfall_m_*`, and
  water-equivalent is derived later via density.
- Corrected ledger supports seven `raw-hourly-melt` windows and one remaining
  H39 first-2013 `hourly-forcing` window.
