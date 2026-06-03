# Baseline Provenance Map

Status: completed
Evidence mode: static

Static:

- Baseline authority: `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- `src/cclim.inc`: declares `radly` as daily solar radiation in
  Langleys/day, `radmj` as daily solar radiation in `MJ m^-2 d^-1`, and
  `hradmj` as hourly radiation.
- `src/stmget.for:188-194`: reads daily climate `radly`.
- `src/winter.for:256-258`: converts `radmj = radly * langmj`, with
  `langmj = 0.04184`.
- `src/sunmap.for:99-260`: consumes `radly` for daily slope/aspect radiation
  and converts to `MJ m^-2 d^-1` for `estrad`/`rpoth`-derived products.
- `src/radcur.for:1-71`: computes hourly potential-radiation ratio/value for
  the hourly radiation path.
- `src/hr_tmp.for:41-47`: emits hourly `hradmj`; near-isothermal branch uses
  `radmj / 24`, otherwise slope/aspect radiation scaled by hourly potential
  ratio.

Inference:

- openWEPP climate parser/runtime daily `rad` is the legacy `radly` value, not
  `radmj`. SIMIMPL28 must therefore multiply by `0.04184` exactly once before
  passing daily radiation to the near-isothermal `hr_tmp` branch.

Ran: not applicable.
