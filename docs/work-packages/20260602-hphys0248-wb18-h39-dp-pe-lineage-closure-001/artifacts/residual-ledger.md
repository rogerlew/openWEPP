# Residual Ledger

Status: completed

Evidence mode: Ran

Ran:
- HPHYS0248 corrected the H39 early-season `Dp` burst from `~22..24 mm/day` to
  `0.246960 mm/day`, matching the baseline `0.240000 mm/day` scale.
- H39 `Dp` max residual improved from HPHYS0247 `23.809497` to HPHYS0248
  `0.240000`.
- Full 39 suite shows `Dp` still fails every hillslope by comparator tolerance,
  but the remaining max residual is bounded at `0.240000 mm`.
- Residual priority after HPHYS0248:
  1. WB17 `Ep`/`Es` partition: `0/39` pass, `56834` and `56973` fail-count sums.
  2. Snow/runoff timing: `Snow-Water`, `RM`, `Q` all `0/39` pass.
  3. Aggregate storage: `Total-Soil`/`SoilWaterTotal` `0/39` pass and remain
     strongly coupled to ET/snow/lateral timing.
  4. WB19 `latqcc`: `0/39` pass; now should be interpreted after WB18 no
     longer overdrains early H39.
