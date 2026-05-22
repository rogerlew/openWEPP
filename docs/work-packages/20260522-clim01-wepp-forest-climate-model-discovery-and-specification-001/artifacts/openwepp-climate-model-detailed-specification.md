# openWEPP Climate Model Detailed Specification

Status: `complete`
Evidence mode: `Ran + Static`

Static:
- Specification derived from baseline legacy runtime behavior in `/workdir/wepp-forest_260430_baseline` plus existing openWEPP climate/parser contracts.

Ran:
- Evidence gathered by executed source-inspection commands only.

## Model Boundary

This specification defines openWEPP-owned behavior for climate ingestion and event forcing preparation for:
- continuous-daily no-breakpoint inputs (`itemp=1`, `ibrkpt=0`), and
- breakpoint inputs (`itemp=1`, `ibrkpt=1`).

Explicit exclusion:
- single-storm modeling/climate (`itemp=2`) is out of scope and remains unsupported in strict parser mode.

Authoritative sources:
- Legacy behavior: `/workdir/wepp-forest_260430_baseline/src/infile.for`, `src/stmget.for`, `src/brkpt.for`, `src/idat.for`, `src/disag.for`, `src/dblex.for`, `src/const.for`, `src/winter.for`, `src/stmtim.for`.
- openWEPP contracts/specs: `SC-CLIMATE-001`, `SC-INFILE-CLIMATE-001`, `climate-file.spec.md`.

## Variable and Alias Table

| Canonical symbol | Legacy meaning | openWEPP parser field | Runtime climate model alias | Units |
|---|---|---|---|---|
| `datver` | climate file version / CLIGEN behavior selector | `ClimateFile.datver` | `climate.datver` | none |
| `itemp` | simulation mode | `ClimateModeFlags.itemp` | `climate.mode.itemp` | enum |
| `ibrkpt` | breakpoint mode flag | `ClimateModeFlags.breakpoint_enabled` | `climate.mode.breakpoint_enabled` | bool |
| `iwind` | wind/ET mode flag | `ClimateModeFlags.iwind` | `climate.mode.iwind` | enum |
| `stmid` | station identifier | `ClimateFile.station_id` | `climate.station_id` | text |
| `deglat,deglon,elev,obsyrs,ibyear,numyr` | station metadata | `ClimateMetadata.*` | same canonical names | mixed |
| `obmaxt,obmint,radave,obrain` | monthly normals | `ClimateMonthlyStats.*` | `climate.monthly.*` | mixed |
| `prcp` | daily precip depth | `NoBreakpointDay.prcp` | `forcing.daily.prcp_mm` (source) and `forcing.daily.prcp_m` (runtime) | mm -> m |
| `stmdur` | storm duration | `NoBreakpointDay.stmdur` | `forcing.daily.stmdur_h` and `forcing.event.stmdur_s` | hr -> s |
| `timep` | normalized time-to-peak | `NoBreakpointDay.timep` | `forcing.event.timep` | fraction |
| `ip` | peak-to-average intensity ratio | `NoBreakpointDay.ip` | `forcing.event.ip` | fraction |
| `nbrkpt` | breakpoint count | `BreakpointDay.nbrkpt` | `forcing.breakpoint.nbrkpt` | count |
| `timem,pptcum` | breakpoint time/cumulative depth pairs | `BreakpointPoint.timem,pptcum` | `forcing.breakpoint.points` | hr/mm -> s/m |
| `tmax,tmin,rad,vwind,wind,tdpt` | met forcing | day record met fields | `forcing.daily.met.*` | mixed |
| `tmnavg,tmxavg,tave` | derived daily temperature stats | runtime-derived | same canonical names | degC |
| `mxint` | max rainfall intensity | runtime-derived | `forcing.event.mxint` | m/s |
| `stmstr` | breakpoint storm start hour | runtime-derived (breakpoint path) | `forcing.breakpoint.storm_start_hr` | hr |

Notes:
- Variable naming continuity is preserved; openWEPP aliases are additive and do not replace canonical symbols.
- Parser layer is file-faithful; runtime model layer performs unit conversion and event-shape derivation.

## Process Specification

### P0. Climate header/metadata parse

1. Parse `datver`, flags (`itemp,ibrkpt,iwind`), station line, metadata, monthly vectors, and daily payload.
2. Enforce mode policy:
- strict mode rejects single-storm (`itemp=2`);
- compatibility mode may allow single-storm only when explicitly enabled.
3. Preserve optional `generator_cmd` without semantic mutation.

Evidence:
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs:330-360`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs:383-415`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs:417-459`

### P1. Continuous daily ingestion (`ibrkpt=0`)

1. Read per-day row: `day,mon,year,prcp,stmdur,timep,ip,tmax,tmin,rad,vwind,wind,tdpt`.
2. Apply ratified `iclig` policy:
- support explicit `datver=0.0` override branch (`iclig=0`) with no duration/intensity correction factors;
- support CLIGEN `4.0+` branch (`iclig=1`) and apply `ip *= 0.70`;
- hard-fail on pre-4 nonzero branch requests (`0.0<datver<4.0`, `iclig=2`; no carry-forward of `stmdur *= 2.06`, `ip *= 1.44` behavior).
3. Cap `stmdur <= 23.999 h` before conversion.
4. Convert `prcp` and `rain(*)` mm -> m; convert `stmdur` hr -> s.
5. Compute `avrint = rain/stmdur` and `mxint = ip * avrint` for event forcing.
6. If `prcp > 0` and `stmdur <= 0`, zero precipitation event and emit warning/error policy event.

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:156-219`

### P2. Breakpoint ingestion (`ibrkpt=1`)

1. Read per-day row: `day,mon,year,nbrkpt,tmax,tmin,radly,vwind,wind,tdpt`.
2. For each breakpoint pair:
- read `timem,pptcum` (hour, mm),
- store `stmstr = first timem`,
- normalize `timem = (timem-stmstr)*3600`,
- convert `pptcum = pptcum/1000`.
3. Build interval intensities:
- `drain = pptcum(i+1)-pptcum(i)`;
- `dtime = timem(i+1)-timem(i)`;
- if `dtime<=0` fail hard (all intervals);
- if `drain<0` fail hard;
- if `drain==0` set `intsty(i)=0`;
- else `intsty(i)=drain/dtime`.
4. Set `stmdur = sum(dtime)`, `mxint=max(intsty)`, terminal `intsty(nbrkpt)=0`, and `prcp=pptcum(nbrkpt)`.

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:223-257`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for:61-117`

### P3. Event-shape construction for infiltration/runoff

1. If `ibrkpt=0` (or rain absent with snowmelt present), call `disag` to generate event shape from `timep` and `ip`.
2. `disag` chooses `const` when constant-intensity case holds, else `dblex` double-exponential form.
3. Enforce minimum interval spacing behavior by reducing `ninten` if generated spacing is under 300 s.
4. Convert dimensionless shape to dimensional `timem`/`intsty` using `p` and `dur`.
5. Build infiltration arrays (`tr`,`r`,`rr`) with inserted `dt` cadence.

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/idat.for:152-260`
- `/workdir/wepp-forest_260430_baseline/src/disag.for:228-330`
- `/workdir/wepp-forest_260430_baseline/src/dblex.for:109-133`
- `/workdir/wepp-forest_260430_baseline/src/const.for:57-61`

### P4. Winter timing coupling

1. Non-breakpoint path: winter storm start hour uses pseudo-random draw from day seed.
2. Breakpoint path: winter storm start hour uses breakpoint storm start (`stmstr`).

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/winter.for:205-233`

## Invariants and Guards

| Guard ID | Invariant statement | Enforcement surface | Disposition |
|---|---|---|---|
| `CLIM-G-001` | `itemp` domain is constrained to supported policy (`1` strict; `2` only explicit compat). | parser mode gate | hard-fail |
| `CLIM-G-002` | `ibrkpt` in `{0,1}` and `iwind` in `{0,1}`. | parser | hard-fail |
| `CLIM-G-003` | Date tuple is valid and strictly increasing across daily records. | parser | hard-fail |
| `CLIM-G-004` | `prcp>=0`, `stmdur>=0`, `timep in [0,1]`, `ip>=0` on non-breakpoint rows. | parser | hard-fail |
| `CLIM-G-005` | Breakpoint `pptcum` is monotone nondecreasing. | parser | hard-fail |
| `CLIM-G-006` | Breakpoint runtime checks: no cumulative decrease; no positive-rain interval with non-positive elapsed time. | runtime climate model | hard-fail |
| `CLIM-G-007` | Runtime unit conversion and event-shape closure must preserve non-negative storm depth/duration semantics. | runtime climate model | hard-fail |
| `CLIM-G-008` | No silent fallback on invalid required climate inputs or invalid event shape. | parser + runtime | hard-fail |
| `CLIM-G-009` | Supported climate-version policy is explicit `datver=0.0` override (`iclig=0`) plus `datver>=4.0` (`iclig=1`); pre-4 nonzero branch (`iclig=2`) is rejected by explicit guard. | parser + runtime policy gate | hard-fail |
| `CLIM-G-010` | Breakpoint times must be strictly increasing for all intervals (`dtime>0`), including zero-drain intervals. | parser + runtime policy gate | hard-fail |

## Decision and HOLD Register

1. `DECISION-CLIM01-SPEC-001`: breakpoint cardinality target should match legacy runtime capacity (`1500`); parser/runtime alignment implementation remains open.
2. `DECISION-CLIM01-SPEC-002`: do not carry forward the disabled dewpoint partition branch; retain active temperature-threshold path unless superseding authority is adopted.
3. `DECISION-CLIM01-SPEC-003`: openWEPP supports explicit `datver=0.0` override (`iclig=0`) and `datver>=4.0` (`iclig=1`), and must guard/reject pre-4 nonzero inputs (`iclig=2` branch).
- Clarification: baseline factors are `stmdur*2.06`, `ip*1.44`, and `ip*0.70` (not `0.8`); carry-forward policy keeps `iclig=0` and `iclig=1` only.
4. `DECISION-CLIM01-SPEC-004`: treat legacy zero-drain non-increasing-time acceptance as a bug and enforce strict `dtime>0` guard for all breakpoint intervals.

Evidence:
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs:9`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs:629-634`
- `/workdir/wepp-forest_260430_baseline/src/cdiss12.inc:7`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1743-1797`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for:67-136`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:161-184`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for:76-83`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for:88-92`
