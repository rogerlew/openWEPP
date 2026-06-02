# HPHYS0250 Residual Ledger

Status: HOLD

Evidence mode: ran

Primary residuals after HPHYS0250 full 39 suite:

| Rank | Family | Status | Evidence | Next focus |
|---:|---|---|---|---|
| 1 | `Ep` | open | `0/39`, fail count `56230`, mean abs diff mean `1.683414`, max `7.778432` | Baseline-authoritative `swu.for` uptake/stress/storage availability magnitude. |
| 2 | `Total-Soil` / `SoilWaterTotal` | open | `0/39`, fail count `56955`, mean abs diff mean `168.130627`, max `619.184688` | Reassess after `Ep` uptake and snow/runoff timing; now affected by real Ep withdrawal. |
| 3 | `Snow-Water` | open | `0/39`, fail count `24137`, mean abs diff mean `58.195696`, max `562.47` | Snow timing/melt storage lineage. |
| 4 | `RM` / `Q` | open | `RM 0/39`, `Q 0/39`; worst H6 storm/runoff magnitude | Runoff/snow timing and WB14/WB16 coupling. |
| 5 | `Dp` | open | `0/39`, fail count `40512`, mean abs diff mean `0.171118`, max `0.24` | Continue WB18 restrictive-bottom/percolation after ET/storage state stabilizes. |
| 6 | `Es` | nearly closed | `38/39`, fail count `1165`, worst H6 max `1.89` | H6-specific storage/snow/runoff interaction, not first priority. |
| 7 | `Er`, `P`, `frozwt` | closed/pass | `39/39` pass for Er and frozwt; P pass with floating epsilon | Monitor only. |

Disposition:

- HOLD. HPHYS0250 removed the zero-`Ep` lineage blocker but exposed the next
  baseline-migration target: actual root uptake and water-stress magnitude.

Review-disposition residuals:

- PL active-slot logic is duplicated between runner sentinel preservation and
  scheduler slot resolution. Current tests cover HPHYS0250 single-OFE behavior;
  centralization/cross-checking is deferred to continuation if PL scope expands.
- Initial perennial `rtd` projection uses management-only `rdmax`; growth
  transition applies the merged soil-depth cap before ET/root uptake. A future
  initial-state migration package should add explicit `rdmax > solthk` evidence.
