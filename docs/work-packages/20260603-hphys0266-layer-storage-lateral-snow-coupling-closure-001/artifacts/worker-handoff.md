# Worker Handoff

Status: completed/HOLD
Evidence mode: Static + Ran

Ran:

- `/tmp/hphys0266_20260603T155434Z/reports/hphys0266_layer_storage_lateral_classification.md`
- `/tmp/hphys0266_20260603T155434Z/reports/hillslope_semantic_summary.md`

Current facts:

- Full semantic pass remains `0/39`.
- H1/H7/H39 first residual days still match HPHYS0265 day IDs:
  H1 day 15, H7 day 11, H39 day 22.
- WB17 identities close on all three first-divergence days.
- WB11/WB18 aggregate recompute closes on all three first-divergence days.
- WB19 lateral potential/target/realized/withdrawal and `Qd` identities close
  on all three first-divergence days.
- H1 and H39 show stress layers above lateral/withdrawal layers.
- H7 is the most actionable seam: withdrawal occurs in layer `0007`, which also
  appears in the SWU-stressed layer set, while capacity/conductivity active
  counts remain concentrated in layers `0008` and `0009`.

Recommended next package:

- Contract-first diagnostic package for post-lateral/pre-SWU layer-threshold
  lineage.
- Required trace additions: `drfc_i`, `fzdrfc_i`, pre/post lateral `st/theta`,
  lateral withdrawal eligibility by layer/substep, WB17 `ul_i`, `pltol*ul_i`,
  and stress threshold inputs.
- Patch production only if those fields prove a baseline-authoritative defect
  against `/workdir/wepp-forest_260430_baseline`.
