---
contract_id: SC-INFILE-CLIMATE-001
title: Climate Input Parser Contract (.cli)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.1
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-CLIMATE-001 Climate Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-CLI-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/climate-file.spec.md:1-176` (canonical openWEPP climate input shape and field semantics).
- `[DIRECT][E-SURVEY-CLI-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md:20-21` (`.cli` coverage and legacy parser provenance references).
- `[DIRECT][E-WF-CLI-01]` `/home/workdir/wepp-forest/src/infile.for:1707-1827` (legacy climate header/state parse path references cited by survey).
- `[DIRECT][E-WF-CLI-02]` `/home/workdir/wepp-forest/src/stmget.for:11-246` and `/home/workdir/wepp-forest/src/idat.for:1-60` (breakpoint/storm-path consumption references cited by survey).
- `[DIRECT][E-WP-CLI-01]` `/workdir/wepppy/wepppy/climates/cligen/cligen.py` (`ClimateFile` parser surface cited by survey).
- `[DIRECT][E-WP3-CLI-01]` `/workdir/wepppyo3/cli_revision/src/lib.rs` (Rust climate transform readers cited by survey).
- `[INFERENCE][E-PHYS-CLI-01]` Physical/common-sense invariants: day/month/date validity, non-negative precipitation/duration, monotone cumulative precipitation within a day.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for surface `infile-climate-cli` (`.cli`) and the handoff from file-faithful parsed structures to simulation forcing state.

### 1.2 Version/Datver Applicability Matrix

| Case | `datver` / mode | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | `0.0` | Accept. | Preserve source values without CLIGEN `ip` scaling interpretation in parser layer. | `[DIRECT][E-SPEC-CLI-01]` |
| B | `4.0` | Accept. | Preserve raw parsed values; scaling policy is runtime/model logic, not parser mutation. | `[DIRECT][E-SPEC-CLI-01]`, `[INFERENCE][E-WF-CLI-01]` |
| C | `4.30`, `5.30` | Accept. | Preserve version tag and records as canonical modern forms. | `[DIRECT][E-SPEC-CLI-01]` |
| D | unrecognized numeric `datver` | Strict: reject. Compat: reject unless explicit allowlist extension is configured. | Emit typed `UnsupportedDatver`. | `[INFERENCE][E-SPEC-CLI-01]` |
| E | `itemp=1` continuous | Accept. | Build continuous daily forcing stream. | `[DIRECT][E-SPEC-CLI-01]` |
| F | `itemp=2` single-storm | Strict: reject by policy. Compat: accepted only when explicit legacy mode is enabled. | Default contract posture is unsupported in openWEPP startup policy. | `[DIRECT][E-SPEC-CLI-01]`, `[INFERENCE][E-SURVEY-CLI-01]` |

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
cli_file = datver_line flags_line station_line headers metadata_line
           month_block daily_header daily_units daily_records ;

datver_line = real ;
flags_line = itemp ibrkpt iwind ;
metadata_line = deglat deglon elev obsyrs ibyear numyr [generator_cmd] ;
month_block = maxt_hdr maxt_vals mint_hdr mint_vals rad_hdr rad_vals rain_hdr rain_vals ;

daily_records =
  (daily_line_no_break){n_days}      when ibrkpt=0
  | ((daily_line_break, breakpoint_line{nbrkpt})){n_days} when ibrkpt=1 ;
```

### 2.2 Two-Layer Model Contract

- Source model is file-faithful:
  - preserves `datver`, flags, metadata, monthly vectors, and daily row tokens as parsed.
  - does not infer or mutate scientific quantities beyond type conversion and structural decoding.
- Simulation model is normalized:
  - expands daily records into typed `ClimateDailyRecord` items.
  - for breakpoint mode, normalizes each day into a `BreakpointDay` with ordered breakpoint points.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `climate.version.datver` | none | real | 1 | yes | all | none | `climate.datver` |
| `itemp` | `header.itemp` | `climate.mode.itemp` | enum | int | 1 | yes | all | none | `climate.mode` |
| `ibrkpt` | `header.ibrkpt` | `climate.mode.breakpoint_enabled` | flag | int/bool | 1 | yes | all | bool derived from int | `climate.breakpoint_flag` |
| `iwind` | `header.iwind` | `climate.mode.wind_et_mode` | enum | int | 1 | yes | all | none | `climate.wind_mode` |
| `stmid` | `header.stmid` | `climate.station.id_text` | text | string | 1 | yes | all | trim trailing spaces | `climate.station_id` |
| `deglat` | `meta.deglat` | `climate.station.lat_deg` | degrees | real | 1 | yes | all | none | `climate.lat_deg` |
| `deglon` | `meta.deglon` | `climate.station.lon_deg` | degrees | real | 1 | yes | all | none | `climate.lon_deg` |
| `elev` | `meta.elev` | `climate.station.elev_m` | m | real | 1 | yes | all | none | `climate.elev_m` |
| `obsyrs` | `meta.obsyrs` | `climate.meta.obs_years` | years | int | 1 | yes | all | none | `climate.obs_years` |
| `ibyear` | `meta.ibyear` | `climate.meta.start_year` | year | int | 1 | yes | all | none | `climate.start_year` |
| `numyr` | `meta.numyr` | `climate.meta.num_years` | years | int | 1 | yes | all | none | `climate.num_years` |
| `generator_cmd` | `meta.generator_cmd` | `climate.meta.generator_cmd` | text | string | 0..1 | no | present when provided by CLIGEN 5.1+ style outputs | preserve verbatim token payload | `climate.generator_cmd` |
| `obmaxt(1..12)` | `monthly.obmaxt[]` | `climate.monthly.obmaxt[]` | degC | real[12] | 12 | yes | all | none | `climate.monthly.tmax_avg` |
| `obmint(1..12)` | `monthly.obmint[]` | `climate.monthly.obmint[]` | degC | real[12] | 12 | yes | all | none | `climate.monthly.tmin_avg` |
| `radave(1..12)` | `monthly.radave[]` | `climate.monthly.radave[]` | langleys/day | real[12] | 12 | yes | all | none | `climate.monthly.rad_avg` |
| `obrain(1..12)` | `monthly.obrain[]` | `climate.monthly.obrain[]` | mm | real[12] | 12 | yes | all | none | `climate.monthly.pcp_avg` |
| `day,mon,year` | `daily[i].date` | `forcing.daily[i].date` | calendar | tuple(int,int,int) | n_days | yes | all | none | `forcing.date` |
| `prcp` | `daily[i].prcp` | `forcing.daily[i].prcp_mm` | mm | real | n_days (ibrkpt=0) | conditional | all | none | `forcing.prcp_mm` |
| `stmdur` | `daily[i].stmdur` | `forcing.daily[i].storm_duration_h` | h | real | n_days (ibrkpt=0) | conditional | all | none | `forcing.storm_duration_h` |
| `timep` | `daily[i].timep` | `forcing.daily[i].time_to_peak_ratio` | fraction | real | n_days (ibrkpt=0) | conditional | all | none | `forcing.timep` |
| `ip` | `daily[i].ip` | `forcing.daily[i].ip_ratio` | fraction | real | n_days (ibrkpt=0) | conditional | all | none | `forcing.ip_ratio` |
| `nbrkpt` | `daily[i].nbrkpt` | `forcing.daily[i].breakpoint_count` | count | int | n_days (ibrkpt=1) | conditional | all | none | `forcing.breakpoint_count` |
| `timem,pptcum` | `daily[i].breakpoints[j]` | `forcing.daily[i].breakpoints[j]` | h,mm | pair(real,real) | sum(nbrkpt) | conditional | all | none | `forcing.breakpoints` |
| `tmax,tmin,rad,vwind,wind,tdpt` | `daily[i].met` | `forcing.daily[i].met` | mixed | record | n_days | yes | all | none | `forcing.met` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `climate.version.datver` | `input::climate` | init | immutable | parser compatibility gate | `G-CLI-001` |
| `itemp,ibrkpt,iwind` | `header.flags` | `forcing.mode` | `input::climate` | init | immutable | forcing scheduler, ET mode selector | `G-CLI-002`, `G-CLI-003` |
| `generator_cmd` | `meta.generator_cmd` | `climate.meta.generator_cmd` | `input::climate` | init | immutable | provenance/audit export only | `G-CLI-004` |
| monthly vectors | `monthly.*` | `climate.monthly_stats` | `input::climate` | init,daily | immutable | stochastic/consistency checks | `G-CLI-005` |
| daily date fields | `daily[i].date` | `forcing.daily[i].date` | `input::climate` | init,daily | immutable | snow/freeze, watbal, evap | `G-CLI-006` |
| `prcp,stmdur,timep,ip` | `daily[i].storm` | `forcing.daily[i].storm` | `input::climate` | init,event | immutable | runoff partition, storm kernel | `G-CLI-007` |
| `nbrkpt,timem,pptcum` | `daily[i].breakpoints` | `forcing.daily[i].breakpoints` | `input::climate` | init,event | immutable | breakpoint event builder | `G-CLI-008`, `G-CLI-009` |
| `tmax,tmin,rad,vwind,wind,tdpt` | `daily[i].met` | `forcing.daily[i].met` | `input::climate` | init,daily,event | immutable | snowfreeze, evap, watbal | `G-CLI-010` |

## 5. State Ownership and Mutability

- `input::climate` owns parsed source and normalized simulation climate state. `[INFERENCE][E-SURVEY-CLI-01]`
- Parsed source model is immutable after successful parse. `[INFERENCE][E-SPEC-CLI-01]`
- Runtime forcing consumers may derive transient working values but must not mutate canonical parsed records in place. `[INFERENCE][E-PHYS-CLI-01]`
- Forbidden mutation path: any non-`input::climate` module rewriting `forcing.daily[*]` primary parsed values. `[INFERENCE][E-SURVEY-CLI-01]`

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-CLI-001` | Derive expected day record count from `numyr` and parsed date span policy. | parse finalize | `C-CLI-001` |
| `D-CLI-002` | For breakpoint mode, derive daily storm depth from final `pptcum` and compare to monotone accumulation constraints. | per day parse finalize | `C-CLI-002` |
| `D-CLI-003` | Normalize wind/ET mode enum from `iwind` token. | header parse | `C-CLI-003` |

Closure hooks:
- `C-CLI-001`: record-count closure check (no silent truncation/overflow).
- `C-CLI-002`: monotone/non-negative cumulative rainfall closure.
- `C-CLI-003`: enum-domain closure for `itemp`, `ibrkpt`, `iwind`.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `CLI-E-001` | syntax | token parse failure on required numeric fields (`TokenParseError`) |
| `CLI-E-002` | syntax | missing required records (`UnexpectedEof` / `RecordCountError`) |
| `CLI-E-003` | semantic | unsupported `datver` (`UnsupportedDatver`) |
| `CLI-E-004` | semantic | unsupported or invalid mode flags (`EnumDomainError`) |
| `CLI-E-005` | semantic | negative precipitation/duration or invalid ratio domain (`FieldRangeError`) |
| `CLI-E-006` | semantic | invalid date tuple (`DateDomainError`) |
| `CLI-E-007` | cross-file | climate years incompatible with management/run schedule (`CrossFileConsistencyError`) |
| `CLI-E-008` | cross-file | breakpoint mode declared but breakpoint payload malformed (`BreakpointShapeError`) |
| `CLI-E-009` | runtime-guard | post-parse monotonicity/closure hook failure (`InvariantViolation`) |
| `CLI-E-010` | semantic | `nbrkpt` exceeds parser-cardinality policy (`BreakpointCardinalityError`) |

No silent fallback/default masking is permitted for required invalid inputs. `[DIRECT][E-SPEC-CLI-01]`, `[INFERENCE][E-SURVEY-CLI-01]`

## 8. Cross-File Consistency Constraints

1. `climate.meta.num_years` and parsed date coverage must be consistent with management/run schedule years (`nyears * nrots`) where coupled. `[INFERENCE][E-SPEC-CLI-01]`, `[INFERENCE][E-SURVEY-CLI-01]`
2. For `ibrkpt=1`, each day must provide exactly `nbrkpt` breakpoint pairs. `[DIRECT][E-SPEC-CLI-01]`
3. ET/wind mode compatibility branch from `iwind` must be preserved for downstream ET subsystem mode selection. `[DIRECT][E-SPEC-CLI-01]`
4. Climate forcing fields needed by snow/freeze, watbal, evap, and runoff consumers must be complete for every parsed day. `[INFERENCE][E-SURVEY-CLI-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `datver,itemp,ibrkpt,iwind,stmid,deglat,deglon,elev,obsyrs,ibyear,numyr` | `climate.version`, `climate.mode`, `climate.station`, `climate.meta` | hillslope/watershed CLI process handoff payload | same canonical names in metadata block; units preserved (`deg`, `m`, `years`) | no unit conversion at export boundary |
| `generator_cmd` | `climate.meta.generator_cmd` | provenance/manifest export | `generator_cmd` text field | optional; omitted when absent in source |
| `day,mon,year,prcp,stmdur,timep,ip,tmax,tmin,rad,vwind,wind,tdpt` | `forcing.daily[i]` | forcing interchange (HBP/parquet/runtime payload) | date tuple + canonical weather/storm field names; units preserved from Section 3 | parser contract owns naming and unit continuity through this boundary |
| `nbrkpt,timem,pptcum` | `forcing.daily[i].breakpoints` | event forcing interchange | repeated `breakpoints[*]` records preserving `(timem,pptcum)` | strict mode enforces declared cardinality and max count policy |

## 10. Compatibility Policy

- Strict mode:
  - accept allowlisted `datver` only;
  - reject `itemp=2` legacy single-storm mode;
  - reject `nbrkpt > 1500` (`CLI-POL-003`) for breakpoint days;
  - reject duplicate/decreasing breakpoint `timem` (`dtime<=0`) for all intervals;
  - reject malformed breakpoint days and invalid cumulative rainfall structure.
- Compatibility mode:
  - `itemp=2` may be accepted only under explicit legacy flag;
  - unrecognized but numerically parseable `datver` remains rejected by default pending explicit policy extension;
  - `nbrkpt > 1500` remains rejected unless explicit `allow_breakpoint_cardinality_override` is enabled for controlled investigations;
  - legacy zero-drain non-positive `dtime` acceptance is disabled by default and must be explicitly enabled with `allow_legacy_zero_drain_non_positive_dtime` for controlled investigations;
  - no silent data repair.

Unsupported forms are hard errors with typed taxonomy entries from Section 7.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-CLI-001` | datver allowlist/compat policy | header parse | `CLI-E-003` |
| `G-CLI-002` | mode enums in domain | header parse | `CLI-E-004` |
| `G-CLI-003` | strict single-storm rejection | mode policy gate | `CLI-E-004` |
| `G-CLI-004` | optional metadata payload (`generator_cmd`) must remain UTF-8/text-safe | metadata parse | `CLI-E-001` |
| `G-CLI-005` | 12-value monthly vector closure | monthly parse | `CLI-E-002` |
| `G-CLI-006` | valid calendar date | daily parse | `CLI-E-006` |
| `G-CLI-007` | non-negative precipitation/duration and valid ratios | daily parse | `CLI-E-005` |
| `G-CLI-008` | breakpoint arity closure (`nbrkpt`) | breakpoint parse | `CLI-E-008` |
| `G-CLI-009` | breakpoint cardinality policy (`nbrkpt <= 1500` unless compat override) | breakpoint parse policy gate | `CLI-E-010` |
| `G-CLI-010` | met field completeness per day + monotone non-decreasing `pptcum` in breakpoint mode + strict breakpoint `timem` monotonicity (`dtime>0`) unless explicit legacy compat control is enabled | daily parse + breakpoint closure hook | `CLI-E-002`/`CLI-E-009` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical variable names follow legacy WEPP/spec symbols and are not replaced:
`datver`, `itemp`, `ibrkpt`, `iwind`, `stmid`, `deglat`, `deglon`, `elev`,
`obsyrs`, `ibyear`, `numyr`, `day`, `mon`, `year`, `prcp`, `stmdur`, `timep`,
`ip`, `nbrkpt`, `timem`, `pptcum`, `tmax`, `tmin`, `rad`, `vwind`, `wind`, `tdpt`.

openWEPP names are explicit aliases only (Section 3 table). `[DIRECT][E-SPEC-CLI-01]`, `[INFERENCE][E-SURVEY-CLI-01]`

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `CLI-GAP-001` | Final openWEPP policy for `itemp=2` legacy single-storm acceptance is not ratified. | `[DIRECT][E-SPEC-CLI-01]`, `[INFERENCE][E-SURVEY-CLI-01]` | `HOLD` |
| `CLI-GAP-002` | Exact parser-vs-runtime responsibility boundary for historical `datver=4.0` `ip` handling is not yet encoded in executable architecture docs. | `[DIRECT][E-SPEC-CLI-01]`, `[INFERENCE][E-WF-CLI-01]` | `HOLD` |
| `CLI-GAP-003` | Parser/runtime breakpoint cardinality policy is aligned to `1500` in openWEPP; cross-port comparator limits outside openWEPP remain investigative only. | `[DIRECT][E-SPEC-CLI-01]`, `[INFERENCE][E-WF-CLI-02]` | `RESOLVED-IN-OPENWEPP` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-22` | `0.1.2` | Updated breakpoint policy to `1500`, added strict breakpoint-time monotonicity policy text, and documented explicit legacy timing compat control. |
| `2026-05-21` | `0.1.1` | Added boundary export mapping, generator command propagation, and explicit strict/compat breakpoint cardinality guard policy. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE01. |
