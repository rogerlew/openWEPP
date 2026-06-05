# Implementation/Test Evidence

Status: complete

Evidence mode: ran

Static:

- Primary generated artifacts:
  - `fixed-baseline-semantic-reports/H1.semantic.json` through
    `fixed-baseline-semantic-reports/H39.semantic.json`.
  - `fixed-baseline-semantic-summary.json`.
  - `fixed-baseline-semantic-metrics.md`.
  - `fixed-vs-previous-summary-delta.json`.
  - `snow-rm-window-reclassification.json`.
  - `snow-rm-window-reclassification.md`.
  - `continuation-decision.md`.
  - `fixed-baseline-semantic-suite-ledger.json`.

Ran:

- `/workdir/wepppy/.venv/bin/python artifacts/hphys0304_fixed_comparator_semantic_rerun.py --python /workdir/wepppy/.venv/bin/python`:
  pass.
- H1..H39 fixed-baseline semantic rerun: `0/39` hillslopes passed; structural
  row/key failures: `0`.
- Focus-column fail counts remained materially unchanged from HPHYS0302:
  `RM=7097`, `Snow-Water=10391`, `Total-Soil=52185`,
  `SoilWaterTotal=52185`, `Ep=42688`, `Dp=10961`, `Q=0`.
- Nine H1/H7/H39 target windows were reclassified as
  `fixed-baseline-unchanged-term-state-hold`.
