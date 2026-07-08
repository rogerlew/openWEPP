# Authority Source Inventory

Status: queued placeholder.

Execution must complete this table before contract drafting.

| Source | Current evidence | Contract use |
|---|---|---|
| Srivastava (2013) dissertation | Local PDF exists at `/workdir/wepp-forest/references/Srivastava_Diss2013_14.pdf`. | Primary equations, state variables, calibration/lineage, WEPP integration narrative. |
| Srivastava et al. (2013) | Local PDF exists at `references/copyrighted/Srivastava2013.pdf`; verified as the ASABE paper, not the dissertation. | Peer-reviewed linear-reservoir WEPP baseflow companion authority. |
| Srivastava et al. (2017) | Local PDF exists at `references/copyrighted/Srivastava2017_ToASABE_wepp_streamflow.pdf`. | Later WEPP baseflow extension lineage and snow-dominated watershed context. |
| Dun et al. (2009) | Local PDF exists at `references/copyrighted/dun2009.pdf`. | Forest subsurface/deep-percolation/lateral-flow context; not groundwater reservoir authority by itself. |
| `/workdir/wepp-forest_260430_baseline` | Git SHA `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. | Code authority for Anurag Srivastava groundwater/baseflow surfaces. |

## Required Execution Notes

- `references/copyrighted/**` is git-ignored by repository policy; the three
  local openWEPP PDF paths are operator-provided authority inputs, not scaffold
  files to commit.
- Quote or paraphrase only short, necessary snippets from copyrighted papers.
- For each equation or runtime branch accepted into contract authority, cite both
  literature authority and baseline code authority when available.
- Keep `latqcc` lateral subsurface export, groundwater-reservoir baseflow,
  deep seepage, and `chan.inp` `cbase` in separate terminology rows.
