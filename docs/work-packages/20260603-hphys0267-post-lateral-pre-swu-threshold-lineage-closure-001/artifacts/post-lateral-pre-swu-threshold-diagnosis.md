# Post-Lateral Pre-SWU Threshold Diagnosis

Status: completed
Evidence mode: Static + Ran

Ran:

- Report:
  `/tmp/hphys0267_20260603T162040Z/reports/hphys0267_threshold_lineage_classification.md`.
- Threshold: first `|candidate Ep - baseline Ep| > 0.05 mm`.
- H1 first divergence: 2013 day 15, `Ep` diff `-0.052129 mm`.
- H7 first divergence: 2013 day 11, `Ep` diff `-0.057740 mm`.
- H39 first divergence: 2013 day 22, `Ep` diff `-0.050136 mm`.

Static:

- WB17 identity closed at all three first-divergence rows.
- WB19 realized lateral identity closed at all three first-divergence rows.
- Pre/post-lateral withdrawal deltas closed for all withdrawal layers.
- H7 withdrawal from layer `0007` is outside the capacity-active layer set
  `0008,0009`, but it is authorized by pinned baseline top-down realized
  withdrawal because pre-lateral storage is above `fzdrfc`.
- H1 and H39 show no inactive-withdrawal case; their threshold identities
  close as context-only evidence.

Conclusion: HPHYS0267 does not identify a production defect in the
post-lateral/pre-SWU threshold seam. Remaining residuals are materially tied to
upstream storage magnitude and snow/runoff context, not to WB19 threshold
withdrawal eligibility.
