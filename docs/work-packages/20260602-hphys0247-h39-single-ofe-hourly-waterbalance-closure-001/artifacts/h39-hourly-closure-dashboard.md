# H39 Hourly Closure Dashboard

Status: hold

Evidence mode: static + ran

Static:
- HPHYS0247 disposition is `HOLD`, not `GO`.

Ran:
- Manifest hourly lane: pass.
- H39 execution row count: pass (`1461` days).
- Comparator row overlap: pass (`common_row_count=1461`).
- Comparator semantic pass: fail (`semantic_pass=false`).
- Winter sidecar-gate correction: pass; manifest reports
  `winter.active=true`, `snow_file_present=false`.
- WB19 lateral overshoot correction: partial pass; max `latqcc` residual
  improved to `8.13 mm`, but column still fails.
- Internal ledger: fail; WB18 day-1 to day-4 percolation is
  `22-24 mm/day` versus baseline `0.24 mm/day`.
- Targeted tests: pass.
- Workspace gates: pass; `fmt`, `clippy`, workspace tests, anti-evasion,
  `auth11`, diff check, and `cargo deny check` all ran.
- Review/verification: dual review performed and findings resolved; dual
  verification remains queued.
- Final disposition: `HOLD_PENDING_WB18_WB17_SNOWMELT_MIGRATION`.
