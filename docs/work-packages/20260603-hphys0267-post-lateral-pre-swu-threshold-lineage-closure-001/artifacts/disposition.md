# Disposition

Status: completed/HOLD
Evidence mode: Static + Ran

Ran:

- H1/H7/H39 targeted trace classification used
  `/tmp/hphys0267_20260603T162040Z/reports/hphys0267_threshold_lineage_classification.md`.
- Full 39 hillslope semantic metrics used
  `/tmp/hphys0267_20260603T162040Z/reports/hillslope_semantic_summary.md`.
- Focused trace tests and diagnostic compile passed as recorded in
  `gate-results.md`.

Static:

- H1 classification:
  `THRESHOLD_LINEAGE_IDENTITIES_CLOSED_CONTEXT_ONLY`.
- H7 classification:
  `BASELINE_TOPDOWN_WITHDRAWAL_FROM_NONACTIVE_CAPACITY_LAYER`.
- H39 classification:
  `THRESHOLD_LINEAGE_IDENTITIES_CLOSED_CONTEXT_ONLY`.
- Pinned baseline `watbal_hourly.for:774-824` confirms realized `latqcc`
  withdrawal is top-down from any layer with `st(jj)>fzdrfc`, after potential
  and active-layer capacity calculation.

Disposition: HOLD. HPHYS0267 closes the post-lateral/pre-SWU threshold
lineage seam as non-actionable for production physics. No kernel physics patch
is justified by this package. Continue with upstream material storage,
snow/runoff partition timing, and layer redistribution lineage.
