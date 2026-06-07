# Complete Balance Identity Audit

Status: corrected

Evidence mode: executed

Purpose: verify the complete WBVAL06 closure identity before attributing the
18-emitter annual residual.

Declared identity:

- Annual closure for years `2..6`, in `mm/year`:
  `R = P + Irr + UpStrmQ + SubRIn - S - I - Q - Ep - Es - Er - D - latqcc - Tile - delta(SoilWaterTotal + Snow-Water)`.
- `P`, `Irr`, `UpStrmQ`, `SubRIn`, `S`, `I`, `Q`, `Ep`, `Es`, `Er`, `D`,
  `latqcc`, and `Tile` are daily fluxes in `mm`.
- `SoilWaterTotal` and `Snow-Water` are end-of-day storage terms in `mm`.
- Acceptance tolerance: `abs(R) <= 1.0 mm/year` for years `2..6`.
- Year `1` remains outside full-calendar annual closure because initial
  storage is not a full-year boundary in this package.

Static:

- `SC-WATBAL-001` v146 declares daily storage closure with explicit
  interception flux `I` and maps it to `hillslope_wat.Interception:mm`.
- `SC-EVAP-001` keeps interception distinct from ET; `I` is not folded into
  `Ep`, `Es`, or `Er`.
- Post-SNOWSCI WAT schema audit found `UpStrmQ`, `SubRIn`, `Tile`, and
  `frozwt` zero for the single-OFE validation set; `InterceptionStorage` was
  present but all null and is not the daily flux term.
- The old identity, without `I`, produced a max annual residual of
  `26.79080937662684 mm`.
- Daily residuals on the old identity occurred on precipitation/interception
  days and summed by year to the omitted annual interception total.

Ran:

- Revalidated corrected WAT outputs in
  `/tmp/wbval06_interception_after_20260607T000000Z/outputs/`.
- Report:
  `/tmp/wbval06_interception_after_20260607T000000Z/reports/wbval06_interception_rollup.json`.
- Result: `wat_emitters=22`, `clean_with_interception=22`,
  `break_with_interception=0`,
  `max_abs_with_interception=1.0364184390709852e-06 mm`.
