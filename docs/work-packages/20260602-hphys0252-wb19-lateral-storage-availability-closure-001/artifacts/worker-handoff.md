# Worker Handoff

Status: complete

Evidence mode: static + ran

Static:

- Implemented WB19 lateral frozen-adjusted capacity/withdrawal lineage.
- Runtime now uses:
  - `fzdrfc` for lateral capacity-active layers, `tdvv`, and top-down
    withdrawal floor,
  - `drfc` for hourly conductivity-active layers and `fffx`.

Ran:

- Full validation gates passed; see `artifacts/gate-results.md`.
- Full suite root: `/tmp/hphys0252_20260602T195147Z`.
- HPHYS0251 apples-to-apples semantic rerun root:
  `/tmp/hphys0252_hphys0251_semantic_rerun_20260602T200305Z`.

Handoff:

- The package is in `HOLD`, not because the scoped fix is suspect, but because
  the current 39-suite selected outputs do not move when the fix is present.
- This indicates the dominant H39/H1/H13 storage residuals are upstream or
  orthogonal to WB19 `frzw` thresholding in the current run surfaces.
- Recommended next package: diagnose WB11 seed/runtime `st(i)` and `watcon`
  scale continuity from runfile projection through WB18/WB19/WB17, with
  explicit checks for whether layer frozen-water `frzw(i)` is ever projected
  into WB19 for hourly frost-active days.
