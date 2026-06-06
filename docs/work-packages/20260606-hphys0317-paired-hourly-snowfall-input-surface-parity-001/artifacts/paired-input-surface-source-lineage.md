# Paired Input Surface Source Lineage

Status: complete

Evidence mode: Static

Static:

| Lane | Source | HPHYS0317 use |
|---|---|---|
| Baseline winter caller | `winter.for:296-300` | Calls `stmtim` inside the 24-hour winter loop with `rain(iplane)`, `stmdur`, `hour`, `wnttim`, current snow depth, counters, and temperature context. |
| Baseline initialization | `stmtim.for:35-38` | Initializes `hrrain(hour)` and `hrsnow(hour)` to zero before active-storm checks. |
| Baseline active rainfall guard | `stmtim.for:43-95` | Owns positive-rain guard, duration rounding, active interval adjustment, phase split, and `hrsnow`/`hrrain` outputs. |
| Baseline phase branch | `stmtim.for:77-95` | Uses `hrtemp > rst` for rain; otherwise assigns `hrsnow(hour) = rain / wntdur * 10.0`. |
| Baseline snow consumer | `snowd.for:166-172` | Consumes positive `hrsnow(hour)` in the snowing branch. |
| OpenWEPP daily input selection | `06_simimpl28_hourly_forcing.rs:57-71` | Selects `rain_m`, `stmdur_s`, `tmax`, `tmin`, `radly`, and `wnttim` from no-breakpoint or breakpoint forcing. |
| OpenWEPP hourly partition helper | `06_simimpl28_hourly_forcing.rs:627-697` | Mirrors baseline duration rounding, active interval adjustment, and branch split into `snow.hourly.rain_m_####` and `snow.hourly.snowfall_m_####`. |
| OpenWEPP snow consumer | `03_kernel_support_00_support_helpers.rs:3914-3924` | Consumes positive hourly snowfall depth in the cold snowing branch. |
| Contract authority | `SC-CLIMATE-001#INV-CLIMATE-015` | Requires paired controlling input-surface values before production ownership. |
| Snow route authority | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-043` | Preserves the combined `57` carried-row route and no-compensation posture. |
| Water-balance gate | `SC-WATBAL-001#INV-WATBAL-091` | Blocks downstream water-balance edits until upstream source ownership is proven. |

Observed key mismatch:

- Key: `2013 day 11 hour 11`.
- Baseline `hrsnow`: `0.0007454545120708644 m`.
- OpenWEPP homologous snowfall-depth trace:
  `snow.hourly.snowfall_m_0011 = 0.0 m`.
- OpenWEPP minus baseline: `-0.0007454545120708644 m`.

Source-line classification:

Literal governance phrase: source-code resemblance is not parity proof.

The baseline and openWEPP source paths are homologous, but source-code
resemblance is not parity proof. The available HPHYS0313, HPHYS0315, and
HPHYS0316 artifacts do not publish paired fixed-baseline/openWEPP values for
`rain`, `stmdur`, `wntdur`, `wnttim`, `hrtemp`, `rst`, `hrrain`, active
interval membership, and branch choice at the key hour. The only paired value
is the same-unit snowfall-depth output mismatch. That is enough to preserve the
blocker but not enough to assign an openWEPP-owned production defect.

Conclusion: no production edit is authorized.

Follow-on requirement:

HPHYS0318 must add or recover paired observe/trace instrumentation for the
controlling input surfaces at the same year/day/hour/trace lane, then rerun the
classification under ADR0017.
