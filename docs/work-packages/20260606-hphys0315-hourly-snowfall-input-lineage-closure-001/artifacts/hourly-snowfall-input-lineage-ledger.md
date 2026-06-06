# Hourly Snowfall Input Lineage Ledger

Status: complete

Evidence mode: Static

Static:

- Source package: HPHYS0313 settling-route continuation, reclassified through
  HPHYS0314 ADR0017 route ledger.
- Route: `hourly-snowfall-input-lineage-hold`.
- ADR0017 verdict: `UNRESOLVED`.
- owner: `HPHYS0317`.
- production_edit_authorized=false.
- Total carried rows: `24`.

| Hillslope | Window | Carried rows | Key | Baseline `hrsnow` depth (m) | OpenWEPP `snow.hourly.snowfall_m_0011` (m) | Delta openWEPP-baseline (m) | Classification | Owner | Production edit |
|---|---|---:|---|---:|---:|---:|---|---|---|
| H1 | spring-2014 | 8 | 2013 day 11 hour 11 | 0.0007454545120708644 | 0.0 | -0.0007454545120708644 | `forcing-input-surface-parity-hold` / ADR0017 verdict `UNRESOLVED` | `HPHYS0317` | `false` |
| H7 | spring-2014 | 7 | 2013 day 11 hour 11 | 0.0007454545120708644 | 0.0 | -0.0007454545120708644 | `forcing-input-surface-parity-hold` / ADR0017 verdict `UNRESOLVED` | `HPHYS0317` | `false` |
| H39 | spring-2014 | 9 | 2013 day 11 hour 11 | 0.0007454545120708644 | 0.0 | -0.0007454545120708644 | `forcing-input-surface-parity-hold` / ADR0017 verdict `UNRESOLVED` | `HPHYS0317` | `false` |

Source-line anchors:

- Baseline hourly snowfall producer: `stmtim.for:43-95`.
- Baseline snowing-branch consumer: `snowd.for:166-172`.
- Baseline winter caller/publication context: `winter.for:366-367` and
  `winter.for:379`.
- OpenWEPP hourly forcing helper:
  `06_simimpl28_hourly_forcing.rs:627-697`.
- OpenWEPP snowing-branch consumer:
  `03_kernel_support_00_support_helpers.rs:3914-3924`.
- Unit authority: `SC-CLIMATE-001#INV-CLIMATE-014` and
  `SC-SNOWFREEZE-001#INV-SNOWFREEZE-041`.

Disposition:

HPHYS0315 did not prove paired fixed-baseline/openWEPP input surfaces for
`rain`, `stmdur`, `wntdur`, `wnttim`, `hrtemp`, and `rst` at the key hour.
Therefore the rows remain `UNRESOLVED`; no snow-producer or water-balance edit
is authorized.
