# HPHYS0218 Worker Handoff

Status: completed
Evidence mode: Static + Ran

## Completed in HPHYS0218
- WB19 `drfc`-equivalent threshold lineage implemented in production kernels:
  `wb18_perc_fc_#### + (1-cpm_####)*dg_####`.
- Canonical contracts updated (`SC-WATBAL-001`, `SC-SUBHYD-001`,
  `SC-SYSTEM-001`).
- Contract-derived WB19 threshold tests added and passing.
- Workspace gates pass (`fmt`, `clippy`, `test`, `deny`).
- Fresh `unpalatable-rind` 39-hillslope rerun + semantic reports completed.

## Residual posture
- `latqcc`: fail-saturated (`39/39`) but mean residual improved.
- `Dp`: fail-saturated (`39/39`) and mean residual regressed.
- `Total-Soil` / `SoilWaterTotal`: fail-saturated (`39/39`) with slight mean
  improvement.
- Integrated disposition remains `HOLD`.

## Immediate next package recommendation
- **HPHYS0219** (follow-on):
  1. Isolate `Dp` regression mechanism under WB19 threshold migration with
     per-day/per-layer lineage diagnostics (`D`, `Pe`, `q`, `Qdd`, `Qd`,
     `wb11_drainable_storage`).
  2. Determine whether `cpm_####` is authoritative for WB19 `drfc` migration
     across all lanes or whether additional baseline symbol lineage mapping is
     required.
  3. Implement contract-first corrective closure for `Dp` without regressing
     `latqcc` improvements.
  4. Re-run the same 39-hillslope semantic lane as closure condition.
