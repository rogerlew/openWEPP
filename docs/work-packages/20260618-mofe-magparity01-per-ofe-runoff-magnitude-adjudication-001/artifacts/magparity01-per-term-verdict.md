# MAGPARITY01 Per-Term Verdict

Evidence mode: **Ran** + **Static**.

| Term | Evidence | Verdict |
|---|---|---|
| Local surface runoff generation | Sum `QOFE-UpStrmQ-SubRIn` is `97,987 m³`, only `0.7%` of `runvol`; non-negative and bounded. | Expected Stage-2 magnitude context; no defect found. |
| Adjacent `UpStrmQ` carry | Downstream `UpStrmQ` equals upstream `QOFE` within `2.27e-13 mm`. | PASS; no `INV-RUNOFFPART-028` defect. |
| Adjacent `SubRIn` carry | Downstream `SubRIn` equals upstream `latqcc` exactly in emitted rows. | PASS; no `INV-RUNOFFPART-028` defect. |
| Area scaling | H2637 has equal OFE areas; `QOFE - Q*OFE` max residual is `6.82e-13 mm`; PASS `runvol` reconstructs from both valid area pairings. | PASS; no Q/QOFE area-duality defect. |
| Per-element and hillslope-total closure | Manifest residuals: transfer `0.0 mm`, per-element `7.96e-13 mm`, hillslope total `1.62e-13 mm`. | PASS; conservation closure remains accepted. |
| Export surfaces | PASS `runvol` equals outlet `Q` volume within `5.46e-12 m³`; PASS `sbrunv` equals OFE19 `latqcc` exactly. | PASS; no export-scaling defect. |
| Legacy `with_ui` comparator | Legacy `runvol = 127.7%` of precip; combined `runvol+sbrunv = 152.6%`. | `LEGACY-DEFECTIVE` / non-authoritative flag, not a target. |
| Legacy `without_ui` bounded delta | Legacy `runvol = 55.5%`; openWEPP `runvol = 71.0%`; both bounded. Including terminal lateral narrows the frame to legacy `72.2%` combined vs openWEPP `75.5%` combined. | `UNRESOLVED` Stage-2 lateral/subsurface magnitude flag; not `OPENWEPP-DEFECTIVE` under ADR-0017. |

## Final Verdict

MAGPARITY01 does **not** authorize a code or contract fix. The H2637 magnitude
difference is not caused by inter-OFE transfer, area scaling, or export duality.
It routes to Stage-2 lateral/subsurface magnitude adjudication, with legacy used
only as a flag.
