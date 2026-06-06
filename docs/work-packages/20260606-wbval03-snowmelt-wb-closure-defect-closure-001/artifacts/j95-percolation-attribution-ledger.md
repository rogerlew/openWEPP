# J-95 Percolation Attribution Ledger

Status: executed-hold

Evidence mode: mixed `Static:` and `Ran:`

Static:

- Historical WBVAL01 evidence:
  - `p7`, `p11`, `p18`, and `p20` failed with
    `HKERNEL-WB11-PERC-E-003`.
  - Failure surface:
    `last_phase=percolation_deep_seepage`,
    `last_decision_message_id=HKERNEL-WB11-PERC-E-003`,
    `wb18_guard_terms={layer_count=8,invalid_ratio_layers=none}`,
    `sim_day_index=95`, calendar year `1990`, Julian day `95`.
- This statically anchors the original WBVAL03 J-95 defect class.

Ran:

- Current post-WBVAL02 reproduction:

  | Hillslope | Historical WBVAL01 blocker | Current blocker | WBVAL03 attribution status |
  |---|---|---|---|
  | `p7` | `HKERNEL-WB11-PERC-E-003`, J-95 | `CLIM-RUNTIME-E-017`, `radly=486` | held behind upstream climate source boundary |
  | `p11` | `HKERNEL-WB11-PERC-E-003`, J-95 | `CLIM-RUNTIME-E-017`, `radly=486` | held behind upstream climate source boundary |
  | `p18` | `HKERNEL-WB11-PERC-E-003`, J-95 | `CLIM-RUNTIME-E-017`, `radly=486` | held behind upstream climate source boundary |
  | `p20` | `HKERNEL-WB11-PERC-E-003`, J-95 | `CLIM-RUNTIME-E-017`, `radly=486` | held behind upstream climate source boundary |

Seven-gate result:

| Gate | Result |
|---|---|
| Reproduction | True by WBVAL01 static evidence; current reproduction reaches an earlier source-radiation blocker. |
| Mechanism | Not reduced beyond historical percolation-domain failure; current executable mechanism is upstream invalid daily radiation. |
| Ownership | Current blocker is outside WBVAL03 authority. |
| Authority | Climate source boundary is governed by `SC-CLIMATE-001`; percolation authority is not currently reachable. |
| Safety | No guard loosening or compensation attempted. |
| Testability | WBVAL03 percolation regression is not valid until J-95 is reachable again. |
| Validation | False for WBVAL03 surfaces under current inputs; all targets fail earlier at `radly=486`. |

Disposition:

- Legitimate `HOLD` behind `WBVAL04-CLIMATE-RADLY-RAMAX-INPUT-BOUNDARY`.
