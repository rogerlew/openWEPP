# Worker Handoff

Status: completed

Evidence mode: Static + Ran

## Summary

HPHYS0265 completed the HPHYS0264 next-focus diagnostic slice.

Run root:

- `/tmp/hphys0265_20260603T151958Z`

Key result:

- First large H1/H7/H39 `Ep` divergences are SWU stress-limited with closed
  PMET/WB17 identities and material storage/snow/lateral context.

## Next Work Package

Recommended objective:

- Diagnose and correct the layer storage/stress-threshold lineage that causes
  first-season SWU stress under closed PMET demand, including snow/runoff timing
  and lateral-flow coupling.

Required starting evidence:

- `artifacts/targeted-h1-h7-h39-first-ep-divergence-classification.md`
- `artifacts/full-39-suite-metrics.md`
- `/tmp/hphys0265_20260603T151958Z/reports/hphys0265_first_ep_divergence_classification.json`

Suggested contract scope:

- `SC-WATBAL-001` for layer/aggregate storage and WAT context.
- `SC-SUBHYD-001` for lateral-flow coupling.
- Snow/runoff contract if one already owns `Snow-Water`, `RM`, and `Q` timing;
  otherwise add a governance gap instead of inventing process math.

Do not patch:

- WB17/SWU publication identities.
- PMET seam wiring.
- Aggregate WB13 publication only.

Reason:

- At first divergence, `pmet_ep_m = Etp`, `Ep = ΣUi`, and `Ws = Ep/Etp` close;
  the residual follows stress-limited water availability and layer/storage
  context.
