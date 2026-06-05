# Cumulative Storage-Budget Localization

Status: executed-hold
Evidence mode: Ran

Ran:
- Full H1..H39 semantic suite plus H1/H7/H39 cumulative budget extraction under
  `/tmp/hphys0295_full_20260605T052422Z`.

Budget equation used:
- `known_flux_mm = ΔRM - ΔQ - ΔET - ΔD - Δlatqcc`
- `budget_gap_mm = Δcandidate_storage_error_mm - known_flux_mm`

Findings:
- `Q` remains closed across the full suite (`39/39` pass).
- WB18 local identity remains closed on targeted rows; cumulative `ΔD` is small
  in the diagnosed windows (`0.038399` to `0.129596 mm`).
- ET residuals are present, but cumulative `ΔET` is not the dominant storage
  owner in the diagnosed windows.
- WB19 lateral differences are material in some H39 windows, but they do not
  dominate after the snow/`RM` term is included.
- `RM` dominates every H1/H7/H39 diagnostic window, including the spring 2014
  storage-collapse windows.
- Residual budget gaps are smaller than the snow/`RM` term and do not justify
  a downstream compensating production edit.

Conclusion:
- The remaining H1/H7/H39 storage collapses are not proven WB17, WB18, WB19,
  or WB13 defects.
- Current evidence assigns next focus to the snow/`RM` producer authority seam:
  determine whether the residual is accepted corrected-negative-melt semantic
  divergence versus an additional baseline-authoritative snow/rain/melt
  producer migration gap.
- Do not compensate the snow/`RM` residual downstream in ET, percolation,
  lateral drainage, or aggregate storage publication.
