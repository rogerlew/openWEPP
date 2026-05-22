# WEPP-Forest Climate Model Behavior Map

Status: `complete`
Evidence mode: `Ran + Static`

Static:
- Baseline source inspection only (no legacy binary execution) from `/workdir/wepp-forest_260430_baseline`.

Ran:
- Executed repository inspection commands (`rg`, `nl`, `sed`) to trace climate call paths, guards, units, and consumer coupling.

## Scope

Included:
- Continuous-daily climate execution (`itemp=1`) with non-breakpoint records (`ibrkpt=0`).
- Breakpoint climate execution (`ibrkpt=1`) with explicit `timem,pptcum` rows.

Excluded:
- Single-storm modeling (`itemp=2`) and single-storm climate support.

## Entry and Mode Selection

1. Runtime mode is selected in `main.for` (`imodel`), then `contin` is used for hillslope/hillslope-watershed paths (`ivers != 3`).
2. Climate header is parsed in `infile.for` (`open` unit `13`), reading `datver`, `itemp`, `ibrkpt`, `iwind`, station/meta blocks, and monthly normals.
3. `stmget` performs per-day climate ingestion and branches by `ibrkpt`.

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/main.for:236-246`
- `/workdir/wepp-forest_260430_baseline/src/main.for:316-382`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1694-1858`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:707-716`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:113-258`

## Continuous-Daily Behavior (`ibrkpt=0`)

### Read and normalization behavior

1. `stmget` chunk-reads 10 daily records at a time (date, `prcp`, `stmdur`, `timep`, `ip`, met variables).
2. `iclig` governs legacy CLIGEN corrections:
- `imodel=1` and `iclig=2`: `stmdur *= 2.06`, `ip *= 1.44`.
- `iclig=1`: `ip *= 0.70`.
3. Storm duration is capped to `23.999` hours before conversion.
4. Units are converted for simulation:
- `prcp` and `rain(*)`: mm -> m.
- `stmdur`: hr -> s.
5. `mxint = ip * (rain/stmdur)` is set in m/s.
6. If `prcp > 0` but `stmdur <= 0`, WEPP warns and zeros precipitation/rainfall for that day.

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:117-124`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:156-184`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:164`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:197-210`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:211-219`

### Per-day derived climate state

1. `tave=(tmin+tmax)/2` is computed each day.
2. For continuous mode, moving averages (`tmnavg`, `tmxavg`) are maintained with a 5-day window.
3. For wet days (`prcp > 0`), rainfall event counters/totals are incremented via `sumrnf`.

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:264-287`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:297`
- `/workdir/wepp-forest_260430_baseline/src/sumrnf.for:36-43`

### Disaggregation path into event intensities

1. `irs`/`wshirs` call `idat` when rain/irrigation/snowmelt triggers event processing.
2. For non-breakpoint days, `idat` calls `disag`.
3. `disag` produces dimensionless storm shape using `timep` and `ip`:
- `const` for constant-intensity case.
- `dblex` for double-exponential shape with `eqroot` solve.
4. `disag` enforces minimum sub-interval width target (`>=300 s`) by reducing interval count if needed.
5. `idat` converts disaggregated series into infiltration arrays (`tr`, `r`, `rr`) using adaptive inserted `dt` values.

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/irs.for:261-286`
- `/workdir/wepp-forest_260430_baseline/src/wshirs.for:185-196`
- `/workdir/wepp-forest_260430_baseline/src/idat.for:152-153`
- `/workdir/wepp-forest_260430_baseline/src/disag.for:213-232`
- `/workdir/wepp-forest_260430_baseline/src/disag.for:239-270`
- `/workdir/wepp-forest_260430_baseline/src/idat.for:205-215`
- `/workdir/wepp-forest_260430_baseline/src/idat.for:226-260`
- `/workdir/wepp-forest_260430_baseline/src/dblex.for:92-119`
- `/workdir/wepp-forest_260430_baseline/src/eqroot.for:147-214`

## Breakpoint Behavior (`ibrkpt=1`)

### Read and conversion behavior

1. `stmget` reads daily header rows: `day mon year nbrkpt tmax tmin radly vwind wind tdpt`.
2. If `nbrkpt > 0`, `brkpt` reads `nbrkpt` `(timem,pptcum)` pairs.
3. `brkpt` stores first breakpoint hour as `stmstr`, then converts times to elapsed seconds from storm start.
4. `pptcum` is converted mm -> m.
5. Interval intensities are computed as `drain/dtime` (m/s), `stmdur` is summed, `mxint` is tracked, and final interval intensity is set to zero.
6. `prcp` and `p` are set from terminal cumulative breakpoint depth.

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:223-247`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for:61-66`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for:73-103`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for:115-117`

### Validation/failure behavior in breakpoint parsing

1. Decreasing cumulative precipitation triggers fatal stop.
2. Positive rain interval with non-positive `dtime` triggers fatal stop.
3. `timep`/`ip` are intentionally not recomputed from breakpoint rows (commented out).

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for:84-92`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for:107-113`

### Downstream event handling differences

1. With `ibrkpt=1` and `prain>0`, `idat` does not call `disag`; breakpoint intensities are treated as authoritative event shape.
2. `idat` still builds infiltration arrays and can add snowmelt intensity onto breakpoint intensity (`rf(i)=intsty(i)+pwmelt/dur`).
3. Winter storm start uses breakpoint storm start time (`wnttim = stmstr`) instead of random draw.

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/idat.for:152-153`
- `/workdir/wepp-forest_260430_baseline/src/idat.for:190-197`
- `/workdir/wepp-forest_260430_baseline/src/winter.for:206-233`

## Consumer Coupling Observed in Baseline

1. Runoff/infiltration path consumes event arrays from `idat`/`disag`/`brkpt` via `irs`/`wshirs`.
2. Winter hourly partition path consumes `rain`, `stmdur`, `stmstr`, temperatures, radiation, and dewpoint-related state.
3. ET path consumes `tave`, `radly`, `tdpt`, `iwind`, `vwind` and selects equation branch by `iwind`.
4. Water balance consumes `rain`, `runoff`, and winter/snowmelt coupled surfaces.
5. Irrigation scheduling references rain-event peak intensity derived from `rain/stmdur*ip`.

Evidence:
- `/workdir/wepp-forest_260430_baseline/src/irs.for:261-286`
- `/workdir/wepp-forest_260430_baseline/src/winter.for:296-299`
- `/workdir/wepp-forest_260430_baseline/src/evap.for:98-100`
- `/workdir/wepp-forest_260430_baseline/src/evap.for:200-213`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:137-139`
- `/workdir/wepp-forest_260430_baseline/src/irrig.for:344-351`

## Decisions and Remaining Holds

1. `DECISION-CLIM01-001`: Breakpoint cardinality target should match legacy capacity (`1500`).
- Legacy runtime arrays are sized to `1500` (`timem`, `intsty`); openWEPP parser currently caps strict mode at `50`.
- Evidence: `/workdir/wepp-forest_260430_baseline/src/cdiss12.inc:7`, `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs:9`, `:629-634`.

2. `DECISION-CLIM01-002`: Do not carry forward the disabled dewpoint-based winter partition branch as an active path.
- Legacy code keeps the dewpoint partition logic commented out and uses the active temperature-threshold branch.
- Evidence: `/workdir/wepp-forest_260430_baseline/src/stmtim.for:67-136`.

3. `DECISION-CLIM01-003`: openWEPP supports CLIGEN `4.0+` only (`iclig=1`) and must hard-guard `datver<4.0` inputs.
- Clarification: this is not a `0.8` factor rule. Baseline legacy factors are `stmdur*2.06`, `ip*1.44` (`iclig=2`) and `ip*0.70` (`iclig=1`); openWEPP carry-forward policy is `iclig=1` only with explicit guard on pre-4.0 data versions.
- Evidence: `/workdir/wepp-forest_260430_baseline/src/infile.for:1743-1765`, `/workdir/wepp-forest_260430_baseline/src/stmget.for:161-184`.

4. `HOLD-CLIM01-004`: For `drain==0`, breakpoint branch does not explicitly reject non-increasing time; `dtime` is still accumulated.
- Evidence: `/workdir/wepp-forest_260430_baseline/src/brkpt.for:76-83`, `:96-99`.
