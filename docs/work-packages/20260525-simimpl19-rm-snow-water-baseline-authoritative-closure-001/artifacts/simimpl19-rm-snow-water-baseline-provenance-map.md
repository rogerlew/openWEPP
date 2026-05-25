# simimpl19-rm-snow-water-baseline-provenance-map

Status: complete
Evidence mode: static
Date: 2026-05-25

## Static
- Baseline authority anchors:
  - `RM = rain + wmelt + irdept (+ irrigation terms)` lineage from legacy
    water-balance pathways (`idat.for`, `watbal_hourly.for`).
  - `Snow-Water` published from runtime SWE state (`snodpy * densg` lineage)
    and not from static controls (`ssd`).
  - `Total-Soil` corresponds to full-profile unfrozen water (`watcon`), while
    top-layer 0.1 m metric is separate (`TSW`) and not the WB13 `Total-Soil`
    column.

## Ran
- not run
