# Fixed-Date Irrigation Input File Specification

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-IRRIGATION-FIXEDDATE-001`
- `surface_id`: `infile-irrigation-fixeddate`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-20T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- `[DIRECT][E-US-01]` `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf` (Table 19, pp. 63-64: fixed-date irrigation file format).
- `[DIRECT][E-US-02]` `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:6707-6781` (Table 19 line definitions; ordering and sentinel guidance).
- `[DIRECT][E-WF-01]` `/workdir/wepp-forest/src/infile.for:2133-2204` (unit 14 open/read path; `itemp/jtemp/ktemp` validation for fixed-date file header).
- `[DIRECT][E-WF-02]` `/workdir/wepp-forest/src/irinpt.for:108-120` and `/workdir/wepp-forest/src/irinpt.for:169-181` (initial fixed-date read of `line 3` for each OFE).
- `[DIRECT][E-WF-03]` `/workdir/wepp-forest/src/irrig.for:263-283` and `/workdir/wepp-forest/src/irrig.for:294-338` (event-time reads of sprinkler/furrow event blocks and next-event line 3).
- `[DIRECT][E-WF-04]` `/workdir/wepp-forest/src/irrig.for:161-163` and `/workdir/wepp-forest/src/irrig.for:283` and `/workdir/wepp-forest/src/irrig.for:338` (legacy warning path for incorrect ordering via format `2010`).
- `[DIRECT][E-WF-05]` `/workdir/wepp-forest/src/inidat.for:1167-1172` and `/workdir/wepp-forest/src/cdat.inc:26-37` (fixed-date version constants and storage variables `irfsch/irffch`, `ifsver/iffver`).
- `[DIRECT][E-WF-06]` `/workdir/wepp-forest/src/infile.for:2150-2167` (fixed-date `verchk` logic commented out in current legacy source; compatibility stop not enforced there).
- `[DIRECT][E-WF-07]` `/workdir/wepp-forest/src/infile.for:2156-2161` and `/workdir/wepp-forest/src/irrig.for:275-281` (pre-94.21 sprinkler compatibility behavior: warning path and `nozzle=1.0` default).
- `[DIRECT][E-WF-08]` `/workdir/wepp-forest/src/pmxsrg.inc:6-12` and `/workdir/wepp-forest/src/cirfur2.inc:69-73` (furrow surge count bound and `surges` meaning).
- `[DIRECT][E-WF-09]` `/workdir/wepp-forest/src/irrig.for:301` (legacy furrow line-5 read consumes `qspply,tstart,tend` only).
- `[DIRECT][E-WF-10]` `/workdir/wepp-forest/src/cirfixd.inc:17-20` (legacy inline definition of `irday/iryr`; `iryr` described as simulation-relative year).
- `[DIRECT][E-WF-11]` `/workdir/wepp-forest/src/infile.for:2003-2039` (run-option mapping to fixed-date-capable schedules/options 1,3,4,6).
- `[DIRECT][E-WF-12]` `/workdir/wepp-forest/src/irinpt.for:127-140` (furrow irrigation is disabled for contour rows and non-crop vegetation).
- `[DIRECT][E-WP-01]` `/workdir/wepppy/wepppy/weppcloud/routes/usersum/weppcloud/wepp-usersum-2024.md:485-520` (modern WEPPpy-hosted Table 19 restatement used for traceability).
- `[DIRECT][E-WP3-01]` `/workdir/wepppyo3/README.md:66-77` and `/workdir/wepppyo3/README.md:128-146` (`wepppyo3` module scope emphasizes output/interchange; no fixed-date irrigation input parser surface documented there).

## 2. Surface Scope and Applicability
- This specification defines the legacy fixed-date irrigation scheduling input file consumed on unit `14` in hillslope WEPP runs. `[DIRECT][E-WF-01]`
- It applies to both stationary sprinkler (`jtemp=1`) and furrow (`jtemp=2`) systems when schedule mode includes fixed-date operation (`ktemp=2`). `[DIRECT][E-US-02]`, `[DIRECT][E-WF-11]`
- It is only applicable when irrigation is enabled by the run option (`irrig` option 1/3/4/6 for fixed-date participation). `[DIRECT][E-WF-11]`
- Watershed `.str/.chn/.imp` routing inputs are out of scope for this file. `[INFERENCE][E-US-02]`

## 3. Version / `datver` Applicability Matrix

| Case | Header behavior | Legacy behavior | OpenWEPP draft stance |
|---|---|---|---|
| Canonical modern | Line 1 present, `datver` near `95.7` | Reads line 1 as `datver`, stores `ifsver/iffver`, validates line 2 tokens | `MUST` accept; treat as normative form |
| Legacy no-version-line | First token parsed as `itemp` (`datver <= 2` probe path) | Backspaces and treats first record as line 2 header | `SHOULD` support in compatibility mode |
| Pre-94.21 sprinkler payload | Sprinkler event line 4 has only `irint, irdept` | Legacy runtime defaults `nozzle=1.0` and prints warning | `SHOULD` support in compatibility mode with explicit default provenance |
| Version floor checks | `irfsch=94.21`, `irffch=91.5` constants exist | Current fixed-date `verchk` call is commented out; hard-stop check not active in this path | `HOLD`: openWEPP policy must explicitly decide enforced floor vs permissive parse |

- `[DIRECT][E-US-02]`, `[DIRECT][E-WF-05]`, `[DIRECT][E-WF-06]`, `[DIRECT][E-WF-07]`

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical grammar (normative draft)

```ebnf
fixeddate_file        = [datver_line] header_line initial_dates event_stream ;
datver_line           = real ;
header_line           = itemp jtemp ktemp ;
itemp                 = integer ;           (* number of OFEs *)
jtemp                 = integer ;           (* 1 sprinkler, 2 furrow *)
ktemp                 = integer ;           (* must equal 2 for fixed-date *)
initial_dates         = line3{itemp} ;
line3                 = ofeflg irday iryr ;

event_stream          = sprinkler_stream | furrow_stream ;
sprinkler_stream      = { sprinkler_line4 line3 } ;
sprinkler_line4       = irint irdept [nozzle] ;

furrow_stream         = { furrow_line4 furrow_line5{surges} line3 } ;
furrow_line4          = surges ;
furrow_line5          = qspply tstart tend [tdepl] ;
```

- First `itemp` occurrences of `line3` are required and are read at initialization in increasing OFE order. `[DIRECT][E-US-02]`, `[DIRECT][E-WF-02]`
- During simulation, each fixed-date event consumes event payload (`line4[/line5]`) followed by a next-event `line3` record for the same OFE. `[DIRECT][E-WF-03]`
- `irday=0` on a `line3` next-event record is the sentinel for "no additional fixed-date irrigation" on that OFE. `[DIRECT][E-US-02]`, `[DIRECT][E-WF-02]`, `[DIRECT][E-WF-03]`

### 4.2 Line-by-line definitions
- **Line 1 (optional in compatibility mode):** `datver` (real), canonical baseline `95.7`. `[DIRECT][E-US-02]`
- **Line 2:** `itemp jtemp ktemp` (integer triple). `[DIRECT][E-US-02]`
- **Line 3:** `ofeflg irday iryr` (integer triple). `[DIRECT][E-US-02]`
- **Sprinkler Line 4:** `irint irdept nozzle` (real triplet), with legacy pre-94.21 two-field form allowed (`nozzle` implied `1.0`). `[DIRECT][E-US-02]`, `[DIRECT][E-WF-07]`
- **Furrow Line 4:** `surges` (integer, max 20). `[DIRECT][E-US-02]`, `[DIRECT][E-WF-08]`
- **Furrow Line 5:** usersum specifies `qspply tstart tend tdepl`; current legacy read path consumes `qspply tstart tend` only. `[DIRECT][E-US-02]`, `[DIRECT][E-WF-09]`

## 5. Field Dictionary With Canonical Symbols and Alias Mapping

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Constraints | openWEPP alias |
|---|---|---|---|---|---|---|---|
| `datver` | fixed-date irrigation file version | none | real | 0..1 per file | yes in canonical mode; optional in compatibility mode | canonical source value `95.7`; compatibility no-version path exists | `fixeddate_irrigation.datver` |
| `itemp` | number of OFEs in file | count | integer | 1 per file | yes | must equal hillslope OFE count (`jstruc`) | `fixeddate_irrigation.ofe_count` |
| `jtemp` | irrigation system type | enum int | integer | 1 per file | yes | `1` sprinkler, `2` furrow; must match run-selected `irsyst` | `fixeddate_irrigation.system_flag` |
| `ktemp` | scheduling type flag | enum int | integer | 1 per file | yes | must equal `2` for fixed-date file | `fixeddate_irrigation.schedule_flag` |
| `ofeflg` | OFE identifier for current record | id | integer | many | yes | first `itemp` lines must be OFE 1..`itemp` order | `event.ofe_id` |
| `irday` | Julian day of event | day-of-year | integer | many | yes | `0` sentinel indicates no additional fixed-date events | `event.day_of_year` |
| `iryr` | irrigation event year | year | integer | many | yes | semantics conflict (calendar vs simulation-relative); see gap register | `event.year` |
| `irint` | sprinkler application rate | m/s | real | per sprinkler event | conditional | finite; `> 0` when event active | `sprinkler_event.application_rate_mps` |
| `irdept` | sprinkler depth | m | real | per sprinkler event | conditional | finite; `>= 0` | `sprinkler_event.depth_m` |
| `nozzle` | sprinkler nozzle factor | none | real | per sprinkler event | conditional | pre-94.21 compatibility may imply `1.0` | `sprinkler_event.nozzle_factor` |
| `surges` | number of furrow surge rows | count | integer | per furrow event | conditional | `1..20` | `furrow_event.surge_count` |
| `qspply` | furrow supply rate | m^3/s | real | per surge row | conditional | finite; non-negative | `furrow_event.surges[i].supply_rate_m3ps` |
| `tstart` | surge start time from midnight | s | real | per surge row | conditional | finite; non-negative | `furrow_event.surges[i].start_s` |
| `tend` | surge end time from midnight | s | real | per surge row | conditional | finite; `>= tstart` | `furrow_event.surges[i].end_s` |
| `tdepl` | depletion-phase duration | s | real | per surge row | optional | documented in usersum; not consumed by current legacy read path | `furrow_event.surges[i].depletion_duration_s` |

### 5.1 Alias mapping policy
- Canonical symbols remain legacy WEPP/wepp-forest names (`datver`, `itemp`, `jtemp`, `ktemp`, `ofeflg`, `irday`, `iryr`, etc.). `[DIRECT][E-US-02]`
- openWEPP names above are alias projections only and must preserve round-trip traceability to canonical symbols. `[INFERENCE][E-US-02]`

## 6. Conditional Branches and Optional Sections
1. **System branch:** `jtemp=1` uses sprinkler event format; `jtemp=2` uses furrow event format. `[DIRECT][E-US-02]`, `[DIRECT][E-WF-01]`
2. **Version branch:** `datver<=2` probe path allows omitted version line in legacy compatibility mode. `[DIRECT][E-WF-01]`
3. **Sprinkler old-format branch:** if `ifsver<94.21`, runtime reads `irint,irdept` and sets `nozzle=1.0`. `[DIRECT][E-WF-07]`
4. **Scheduling branch (`irschd`):** fixed-date-only (`2`) vs combination (`3`) affects whether depletion paths are also active and how `irday=0` changes schedule state. `[DIRECT][E-WF-02]`, `[DIRECT][E-WF-03]`
5. **Furrow availability branch:** furrow fixed-date is disabled for contour practices or non-crop vegetation in legacy initialization. `[DIRECT][E-WF-12]`

## 7. Cross-File Consistency Constraints and Coupling Dependencies
1. Header coupling: `itemp` must equal hillslope OFE count (`jstruc`/`nplane`). `[DIRECT][E-WF-01]`
2. Run-option coupling: file is only meaningful when run options select fixed-date scheduling participation. `[DIRECT][E-WF-11]`
3. System coupling: `jtemp` must equal selected irrigation system (`irsyst`), else legacy rejects and re-prompts file selection. `[DIRECT][E-WF-01]`
4. Scheduling coupling: `ktemp` must equal `2`; mismatch is rejected. `[DIRECT][E-WF-01]`
5. Sequence coupling: first `itemp` `line3` records initialize next event date per OFE; subsequent records must maintain OFE/date ordering discipline from usersum for deterministic behavior. `[DIRECT][E-US-02]`, `[DIRECT][E-WF-02]`, `[DIRECT][E-WF-04]`
6. Simulation-date coupling: event is triggered when simulation date/year equals `irday/iryr` (or single-event mode). `[DIRECT][E-WF-03]`

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)

Draft openWEPP parser outcomes:

| Condition | Expected behavior |
|---|---|
| Missing file | `InputFileMissing(surface_id=infile-irrigation-fixeddate)` |
| Non-numeric or malformed header tokens | `TokenParseError` |
| `jtemp` not in `{1,2}` | `EnumDomainError(field=jtemp)` |
| `ktemp != 2` | `EnumDomainError(field=ktemp)` |
| `itemp` mismatch with hillslope OFE count | `CrossFileMismatch(field=itemp)` |
| `ofeflg` not expected OFE index in required initialization phase | `OrderingConstraintError` |
| `surges < 1` or `surges > 20` | `FieldRangeError(field=surges)` |
| Furrow surge rows missing or arity mismatch | `RecordCountError` / `RecordArityError` |
| Sprinkler pre-94.21 record missing `nozzle` | compatibility mode: apply `nozzle=1.0` with provenance flag |
| Legacy-order warning condition (`ofeflg` mismatch at event read) | treat as hard `OrderingConstraintError` (not warning-only) |

- Legacy runtime frequently warns and continues in some malformed-order cases; openWEPP contract draft prefers typed failures for correctness. `[DIRECT][E-WF-04]`, `[INFERENCE][E-WF-04]`

## 9. Example Snippets

### 9.1 Minimal valid sprinkler fixed-date file (single OFE, one event)
```text
95.7
1 1 2
1 150 1
0.000020 0.0120 1.00
1 0 0
```

### 9.2 Representative valid furrow fixed-date file (single OFE, two surges)
```text
95.7
1 2 2
1 120 1
2
0.00030 0.0 1800.0
0.00015 1800.0 3600.0
1 0 0
```

### 9.3 Invalid examples
1. Invalid scheduling flag:
```text
95.7
1 1 1
1 120 1
```
Reason: `ktemp` must be `2` for fixed-date file. `[DIRECT][E-US-02]`, `[DIRECT][E-WF-01]`

2. Invalid furrow surge count:
```text
95.7
1 2 2
1 120 1
21
```
Reason: `surges` exceeds maximum 20. `[DIRECT][E-US-02]`, `[DIRECT][E-WF-08]`

3. Invalid initialization OFE ordering:
```text
95.7
2 1 2
2 150 1
1 150 1
```
Reason: first `itemp` line-3 entries must be OFE-ordered initialization records. `[DIRECT][E-US-02]`, `[DIRECT][E-WF-02]`

## 10. Gap / Conflict Register (`HOLD` Conditions)

| Gap ID | Issue | Evidence | Status |
|---|---|---|---|
| `FDIRR-GAP-001` | Usersum furrow line 5 includes `tdepl`; current legacy read path only consumes `qspply,tstart,tend`. | `[DIRECT][E-US-02]`, `[DIRECT][E-WF-09]` | `HOLD` |
| `FDIRR-GAP-002` | `iryr` semantics conflict: usersum describes event year; `cirfixd.inc` describes simulation-relative year numbering. | `[DIRECT][E-US-02]`, `[DIRECT][E-WF-10]` | `HOLD` |
| `FDIRR-GAP-003` | Fixed-date compatibility floor constants exist, but fixed-date `verchk` call is commented out in current legacy path. | `[DIRECT][E-WF-05]`, `[DIRECT][E-WF-06]` | `HOLD` |
| `FDIRR-GAP-004` | Usersum prescribes strict cross-OFE chronological organization; legacy code mainly emits warnings for OFE-order mismatch during event reads. | `[DIRECT][E-US-02]`, `[DIRECT][E-WF-04]` | `HOLD` |
| `FDIRR-GAP-005` | wepppyo3 does not currently expose a fixed-date input parser surface, so no modern rust parser behavior baseline exists there. | `[DIRECT][E-WP3-01]` | `HOLD` |

## 11. Parser-Contract Handoff Map

Target parser contract: `SC-INFILE-IRRIGATION-FIXEDDATE-001`

| Contract area | Spec source | Required parser behavior |
|---|---|---|
| Header parse | Sections 3-4 | Parse optional `datver`, required `itemp/jtemp/ktemp`, and enforce `ktemp=2`. |
| Branching | Sections 4 and 6 | Select sprinkler vs furrow event grammar from `jtemp`; support documented compatibility branches. |
| Symbol continuity | Section 5 | Preserve canonical legacy symbols; emit explicit alias mapping in model structs. |
| Cross-file closure | Section 7 | Validate OFE-count and system/schedule coherence against run context. |
| Error surface | Section 8 | Return typed errors; avoid silent mutation except documented compatibility defaulting (`nozzle=1.0`). |
| Unresolved conflicts | Section 10 | Carry `HOLD` gaps into contract disposition; do not claim completion while unresolved. |

Handoff status: `ready-for-contract-authoring (with HOLD gaps)`.
