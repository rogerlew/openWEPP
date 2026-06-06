# Hourly Snowfall Source Lineage

Status: complete

Evidence mode: Static

Static:

| Lane | Source | HPHYS0315 use |
|---|---|---|
| Baseline winter caller | `winter.for:366-367` | Calls `snowd` for each winter hour with current `hour`. |
| Baseline hourly publication context | `winter.for:379` | Converts `hrsnow(hour)` to millimeter output context after winter processing. |
| Baseline initialization | `stmtim.for:35-38` | Initializes `hrrain(hour)` and `hrsnow(hour)` to zero each call. |
| Baseline active rainfall guard | `stmtim.for:43-95` | Owns active-storm duration, start-hour adjustment, phase split, and snowfall depth. |
| Baseline phase branch | `stmtim.for:77-95` | Uses `hrtemp > rst` for rain; otherwise assigns snowfall as `rain / wntdur * 10.0`. |
| Baseline snow consumer | `snowd.for:166-172` | Snowing branch adds `hrsnow(hour)` to `snodep` and density mixing. |
| OpenWEPP hourly forcing | `06_simimpl28_hourly_forcing.rs:627-697` | `simimpl28_stmtim_hourly_partition` mirrors the baseline active interval, duration, and rain/snow branch semantics. |
| OpenWEPP snow consumer | `03_kernel_support_00_support_helpers.rs:3914-3924` | Cold snowing branch consumes hourly snowfall depth and updates depth/density. |
| Unit authority | `SC-CLIMATE-001#INV-CLIMATE-014` | Canonical parity comparison is baseline `hrsnow` depth vs openWEPP `snow.hourly.snowfall_m_####`, not water-equivalent snowfall. |

Observed key mismatch:

- Baseline key: 2013 day 11 hour 11.
- Baseline `hrsnow`: `0.0007454545120708644 m`.
- OpenWEPP homologous snowfall-depth trace:
  `snow.hourly.snowfall_m_0011 = 0.0 m`.
- OpenWEPP minus baseline: `-0.0007454545120708644 m`.

Remaining gap:

HPHYS0315 inspected source-line structure and confirmed that openWEPP has a
homologous helper for the pinned baseline `stmtim.for` branch. That is not
enough to prove openWEPP ownership. The package did not produce paired
fixed-baseline/openWEPP values for `rain`, `stmdur`, `wntdur`, `wnttim`,
`hrtemp`, `rst`, `hrsnow`, `hrrain`, active interval, and branch choice at the
key hour. Because those inputs control whether `hrsnow` is positive, no
production edit is authorized.

Conclusion: no production edit is authorized.

Follow-on requirement:

HPHYS0317 must capture or reconstruct the paired forcing input surfaces at the
same year/day/hour/trace lane and then classify parser forcing, daily climate
input, phase partition, hourly distribution, or harness-surface ownership under
ADR0017.
