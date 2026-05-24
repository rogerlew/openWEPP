# WB18 Percolation Physics Authority And Guard Map

Status: `completed`
Evidence mode: `Static`

## Physics Authority Summary
WB18 percolation execution authority is encoded in canonical `SC-PERC-001` and
consumed in hydrology lane authority in `SC-WATBAL-001`.

Per-layer runtime symbols:
- state: `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`,
  `wb18_perc_ssc_####`
- flux: `wb18_perc_pei_####`
- aggregate: `D`, `Pe`

Core equation forms (contract authority):
- `stz = theta / ul`
- `fx = max(stz^Bi, fx_min)` when `stz < 0.95`, else `1.0`
- `pei_pre = min(Delta_t * ssc * fx, theta - fc)`
- lower-layer restriction: `pei = pei_pre * sqrt(max(1 - stu_lower, 0))`
- deep percolation loss from bottom layer (`D`) and recharge closure (`Pe = D`).

## Guard Map
Typed guard IDs remain WB11-prefixed and now protect WB18 per-layer symbols.

- `HKERNEL-WB11-PERC-E-001`
  - Missing required WB18 per-layer symbol or required lane symbol.
- `HKERNEL-WB11-PERC-E-002`
  - Non-finite WB18 per-layer value.
- `HKERNEL-WB11-PERC-E-003`
  - Domain violation, including:
  - `nsl < 1`
  - `ul <= 0`
  - `ssc <= 0`
  - `fc > ul`
  - invalid saturation/state-ratio domains.

## Runtime Alias Continuity
Legacy WB11 seam symbols (`wb11_soil_water`, `wb11_field_capacity`,
`wb11_perc_fraction`) remain validated in percolation phase for mixed-lane
compatibility while WB18 per-layer symbols carry execution authority.
