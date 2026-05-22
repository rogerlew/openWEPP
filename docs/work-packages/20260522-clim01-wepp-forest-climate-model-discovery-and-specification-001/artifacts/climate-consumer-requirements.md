# Climate Consumer Requirements

Status: `complete`
Evidence mode: `Ran + Static`

Static:
- Consumer requirements derived from baseline runtime call/variable coupling and existing openWEPP climate/parser contracts.

Ran:
- Executed line-level source tracing commands over baseline WEPP and openWEPP contract/parser files.

## Downstream Consumers

| consumer surface | consumed climate fields | timing | requirement class |
|---|---|---|---|
| `winter` + `stmtim` | `rain`, `stmdur`, `stmstr` (breakpoint), `tmin`, `tmax`, `radly`, `tdpt` | daily -> hourly expansion | hard runtime dependency |
| `idat` + `disag` + `dblex/const` | `prcp/rain`, `stmdur`, `timep`, `ip`, `timem`, `intsty`, `nbrkpt` | event | hard runtime dependency |
| `irs` / `wshirs` runoff path | `ninten`, `timem`, `intsty`, `mxint`, `dur`, `rain` | event | hard runtime dependency |
| `watbal` | `rain`, `runoff`, `wmelt`, climate-derived ET terms | daily | hard runtime dependency |
| `evap` / `evappm` | `tave`, `radly`, `tdpt`, `vwind`, `iwind` | daily | hard runtime dependency |
| `irrig` scheduling | `rain`, `stmdur`, `ip`, `norain` | daily/event | hard runtime dependency |
| climate summaries (`sumrnf`) | `prcp` wet-day events | daily | reporting dependency |

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/winter.for:296-299`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-104`
- `/workdir/wepp-forest_260430_baseline/src/idat.for:152-260`
- `/workdir/wepp-forest_260430_baseline/src/disag.for:228-330`
- `/workdir/wepp-forest_260430_baseline/src/irs.for:261-286`
- `/workdir/wepp-forest_260430_baseline/src/wshirs.for:185-196`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:137-139`
- `/workdir/wepp-forest_260430_baseline/src/evap.for:98-100`
- `/workdir/wepp-forest_260430_baseline/src/evap.for:200-213`
- `/workdir/wepp-forest_260430_baseline/src/irrig.for:344-351`
- `/workdir/wepp-forest_260430_baseline/src/sumrnf.for:36-43`

## Required Input Surfaces

1. Header/mode surface:
- `datver`, `itemp`, `ibrkpt`, `iwind`, `stmid`, station metadata.

2. Monthly climatology surface:
- `obmaxt(12)`, `obmint(12)`, `radave(12)`, `obrain(12)`.

3. Daily no-breakpoint surface (`ibrkpt=0`):
- `day,mon,year,prcp,stmdur,timep,ip,tmax,tmin,rad,vwind,wind,tdpt`.

4. Daily breakpoint surface (`ibrkpt=1`):
- `day,mon,year,nbrkpt,tmax,tmin,rad,vwind,wind,tdpt` and
- repeated `timem,pptcum` pairs.

5. Runtime-derived climate surfaces required by consumers:
- `tave`, `tmnavg`, `tmxavg`, `mxint`, `dur`, `ninten`, `timem`, `intsty`, `stmstr`.

## Required Output Surfaces

### Runtime climate model must emit

| output surface | semantics | primary consumers |
|---|---|---|
| `forcing.daily.met` | daily met state (`tmax,tmin,tave,rad,tdpt,vwind,wind`) | ET, snow/freeze, water balance |
| `forcing.event.summary` | `prcp_m`, `stmdur_s`, `mxint_m_s`, `timep`, `ip` (non-breakpoint) | runoff/infiltration, irrigation guards |
| `forcing.event.shape` | `ninten`, `timem[]`, `intsty[]`, cumulative `rr[]` | infiltration/runoff solvers |
| `forcing.breakpoint.start` | `stmstr` for winter event timing | winter/stmtim |
| `forcing.daily.flags` | wet-day/dry-day marker equivalent to `norain` semantics | irrigation and event gating |

### Consumer-side expectations

1. Consumers must treat climate forcing as immutable per day/event after parse-to-runtime conversion.
2. Consumers may derive transient values (for example ET subterms), but cannot rewrite canonical climate forcing arrays.
3. Consumers must preserve units and not reinterpret mm/hr/h values implicitly.

## Failure/Guard Requirements

| requirement | reason |
|---|---|
| Reject unsupported mode combinations (`itemp`, `ibrkpt`, `iwind`) with typed failures. | Prevent silent mode drift. |
| Reject invalid dates and non-monotone daily sequence. | Prevent state chronology corruption. |
| Reject negative or out-of-domain rainfall/storm parameters. | Prevent non-physical forcing payloads. |
| Reject breakpoint non-monotone cumulative depth and invalid interval timings per runtime rules. | Prevent invalid event intensities and duration closure breaks. |
| Surface explicit error for parser/runtime seam gaps (missing climate runtime adapter). | Prevent implicit fallback/default climate inputs. |

## OpenWEPP Consumer Integration Constraints

1. `SC-INFILE-CLIMATE-001` is active for parser governance, but climate parser output is not yet promoted through a dedicated runtime adapter seam equivalent to soil/chaninp seams.
2. Climate consumers currently exist as contract/runtime expectations, not fully wired orchestrator runtime input surfaces.
3. Until climate seam ownership is implemented, climate-driven scheduler phases remain governance `HOLD` for full closure evidence.

Evidence:
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:98-183`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:86-170`
