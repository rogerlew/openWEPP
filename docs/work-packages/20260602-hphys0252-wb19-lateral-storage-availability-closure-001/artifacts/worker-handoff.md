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
- Post-commit Claude review adds an important strategy correction: the
  non-frozen "dominant WB19 withdrawal" premise is still unverified, and
  targeted `latqcc` is below baseline, so over-drainage should be demoted until
  a conservation audit localizes the deficit.
- Recommended next package: diagnostic-only H1 t=0/day-1 localization. Compare
  baseline and openWEPP initial/final `Total-Soil`, layer `st(i)`, and
  `watcon`, then compute water-balance conservation from inputs through
  `ET + Dp + latqcc + Q + delta-storage` before any further process-surface
  correction.
- Guardrail: do not revive the withdrawn `ProfileFCStore` producer-intermediate
  lead without direct t=0 state-surface authority.
