# First Ep Divergence Diagnosis

Status: completed/HOLD

Evidence mode: Ran

Run root:

- `/tmp/hphys0265_20260603T151958Z`

Diagnosis:

- H1 first `|Ep diff| > 0.05 mm`: 2013 day 15, candidate `0.207871 mm`,
  baseline `0.260000 mm`, diff `-0.052129 mm`.
- H7 first `|Ep diff| > 0.05 mm`: 2013 day 11, candidate `0.282260 mm`,
  baseline `0.340000 mm`, diff `-0.057740 mm`.
- H39 first `|Ep diff| > 0.05 mm`: 2013 day 22, candidate `0.439864 mm`,
  baseline `0.490000 mm`, diff `-0.050136 mm`.

Interpretation:

- PMET demand is close to baseline at the first divergence in all three hills:
  H1 `pmet_ep_m = Etp = 0.258109 mm`, H7 `0.344900 mm`, H39 `0.488521 mm`.
- WB17/SWU identities close: `Ep = ΣUi`, aggregate `Ui = ΣUi`, and
  `Ws = Ep/Etp` within diagnostic tolerance.
- Final `Ep` is lower than `Etp` because SWU stress is active, with four
  stress-limited layers in each traced hillslope.
- Same-day context already has material storage, snow/runoff, lateral-flow, or
  runoff-melt residuals.

Conclusion:

- The first large longer-season `Ep` divergence is not a PMET seam regression
  and not a WB17 publication identity defect.
- Focus continuation on layer storage/stress-threshold lineage and coupled
  snow/runoff/lateral storage context.
