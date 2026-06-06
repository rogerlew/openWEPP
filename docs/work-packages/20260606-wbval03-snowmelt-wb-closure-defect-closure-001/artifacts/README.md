# WBVAL03 Artifacts

Status: executed-hold

Evidence mode: mixed `Static:` and `Ran:`

Static:

- WBVAL01 evidence still anchors the original WBVAL03 defects:
  - `p7`, `p11`, `p18`, and `p20` failed with
    `HKERNEL-WB11-PERC-E-003` at `sim_day_index=95`, calendar year `1990`,
    Julian day `95`.
  - Twelve prior WAT-emitting hillslopes had years `2..6` residuals above the
    `1.0 mm/year` tolerance.
- WBVAL03 completed the package-required balance identity audit against the
  saved WBVAL01 WAT parquet files.

Ran:

- Current `57eed35` release-binary reproduction after WBVAL02 shows all
  WBVAL03 target hillslopes fail earlier with `CLIM-RUNTIME-E-017`,
  source symbol `radly=486`.
- No production code or contract amendments were made under WBVAL03.

Closure checks:

- `HOLD` legitimacy: legitimate, because the current WBVAL03 acceptance
  surfaces are not reachable until upstream defect
  `WBVAL04-CLIMATE-RADLY-RAMAX-INPUT-BOUNDARY` is closed.
- Envelope adequacy: no snowmelt/percolation/WAT correction was shielded; the
  current blocker is outside the WBVAL03 authority envelope.
- Protected-boundary integrity: the blocker is named as an upstream climate
  source-bound defect, not a request for another diagnostic step.
