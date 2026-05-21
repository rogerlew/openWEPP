# WEPP Snow Sidecar Input Specification (`snow.txt`)

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-SNOW-001`
- `surface_id`: `infile-snow`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-20T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- [DIRECT][E-US-01] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:396-400` (winter-process description includes snowfall/snowmelt process context).
- [DIRECT][E-US-02] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9413-9453` and `:9546-9558` (sidecar section documents `pmetpara.txt`, `frost.txt`, `chan.inp`; no `snow.txt` format section appears there).
- [DIRECT][E-US-03] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:5261-5262` and `:5293-5294` (initial snow depth `snodpy` is documented in management initial conditions).
- [DIRECT][E-WF-01] `/workdir/wepp-forest/src/infile.for:1566-1575` (`snow.txt` optional read path: `rst`, `newsnw`, `ssd`; fallback defaults when file missing).
- [DIRECT][E-WF-02] `/workdir/wepp-forest/src/cclim.inc:11-18` and `:53` (legacy symbols `rst`, `newsnw`, `ssd`; `densnf` described as new-falling-snow density).
- [DIRECT][E-WF-03] `/workdir/wepp-forest/src/stmtim.for:74` and `:77` (`densnf = newsnw`; precipitation partition branch `hrtemp > rst`).
- [DIRECT][E-WF-04] `/workdir/wepp-forest/src/snowd.for:139-142` (`ssd` used as settling-threshold branch against snow density).
- [DIRECT][E-WF-05] `/workdir/wepp-forest/src/cwint.inc:43` (snowpack density `densg` units are `Kg/m^3`).
- [DIRECT][E-WF-06] `/workdir/wepp-forest/src/inidat.for:383` (legacy default initialization `densg(i)=100.0`).
- [DIRECT][E-WF-07] `/workdir/wepp-forest/src/infile.for:1367` and `:1472` (initial `snodpy` comes from management-side initial-condition records, not from `snow.txt`).
- [DIRECT][E-WP-01] `/workdir/wepppy/wepppy/nodb/core/wepp.py:320-363` (`SnowOpts` defaults and emitted `snow.txt` 3-line content).
- [DIRECT][E-WP-02] `/workdir/wepppy/wepppy/nodb/core/wepp.py:811-823` and `:1621-1637` (wepppy bounds/default guards for `newsnw` and `ssd`).
- [DIRECT][E-WP-03] `/workdir/wepppy/wepppy/nodb/core/wepp.py:1964-1967` and `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py:37-38` (wepppy writes `snow.txt` and parses `snow_opts_*` input fields).
- [DIRECT][E-WP-04] `/workdir/wepppy/wepppy/microservices/rq_engine/wepp_run_payload.py:60-65` (payload fields include `snow_opts_rst`, `snow_opts_newsnw`, `snow_opts_ssd`).
- [DIRECT][E-WP3-01] `/workdir/wepppyo3/README.md:70` (wepppyo3 module scope emphasizes WEPP output interchange, not sidecar input-file parsing).

## 2. Surface Scope and Applicability
- [DIRECT][E-WF-01] `snow.txt` is an optional sidecar file read during WEPP input initialization.
- [DIRECT][E-WF-01] The file contains exactly three scalar parameters in legacy read order: `rst`, `newsnw`, `ssd`.
- [DIRECT][E-WF-03] These parameters affect rain/snow partition and snowpack-density behavior in winter routines.
- [INFERENCE][E-WF-01] Applicability is hillslope and watershed runs that execute winter/snow routines, because values are stored in global climate/winter state rather than OFE-specific sidecar records.
- [DIRECT][E-US-03] Initial snowpack depth (`snodpy`) is sourced from management initial conditions, so this sidecar does not replace management initial snow state.

## 3. Version / `datver` Applicability Matrix

| Case | File state | Legacy `wepp-forest` behavior | openWEPP draft stance |
| --- | --- | --- | --- |
| A | `snow.txt` absent | [DIRECT][E-WF-01] No hard failure; defaults assigned: `rst=0.0`, `newsnw=100.0`, `ssd=250.0`. | [INFERENCE][E-WF-01] Support as explicit optional-surface default branch, with provenance event in diagnostics. |
| B | `snow.txt` present with three parseable records | [DIRECT][E-WF-01] Reads 3 list-directed values in fixed order (`rst`, `newsnw`, `ssd`). | [INFERENCE][E-WF-01] Treat as canonical parse path. |
| C | `snow.txt` present but malformed/incomplete | [DIRECT][E-WF-01] No explicit `err=` on the `read` statements; runtime behavior is implementation-dependent read failure. | [INFERENCE][E-WF-01] Must raise typed parse/record error; do not silently revert to defaults after parse failure. |

- [DIRECT][E-WF-01] No `datver`/version line is used for this sidecar.
- [DIRECT][E-US-02] `usersum2024` does not publish a dedicated `snow.txt` format table in the documented sidecar section.

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical grammar (draft)
```ebnf
snow_file      = canonical_snow_file | compat_snow_file ;
canonical_snow_file = rst_line_strict newsnw_line_strict ssd_line_strict ;
compat_snow_file = rst_line_compat newsnw_line_compat ssd_line_compat ;
rst_line_strict    = real ;
newsnw_line_strict = real ;
ssd_line_strict    = real ;
rst_line_compat    = real [trailing_tokens] ;
newsnw_line_compat = real [trailing_tokens] ;
ssd_line_compat    = real [trailing_tokens] ;
```

- [DIRECT][E-WF-01] Legacy reads exactly three list-directed records in sequence.
- [INFERENCE][E-WP-01] Compatibility mode may allow trailing tokens/comments after the first numeric value per line because modern producer practice emits inline annotations.

### 4.2 Line definitions
- Line 1: `rst` (rain-snow threshold temperature). [DIRECT][E-WF-01], [DIRECT][E-WF-03]
- Line 2: `newsnw` (new-falling-snow density parameter). [DIRECT][E-WF-01], [DIRECT][E-WF-03]
- Line 3: `ssd` (snow settling threshold density parameter). [DIRECT][E-WF-01], [DIRECT][E-WF-04]

## 5. Field Dictionary With Canonical Symbols and openWEPP Alias Mapping

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Constraints (draft) | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `rst` | rain/snow partition threshold applied to hourly temperature comparison (`hrtemp > rst` => rain branch) | deg C | real | 0..1 per file | optional (defaulted if file absent) | finite numeric; no canonical hard bounds yet | `snow_opts.rst` |
| `newsnw` | density of new falling snow (`densnf = newsnw`) | kg/m^3 | real | 0..1 per file | optional (defaulted if file absent) | finite positive numeric; legacy default `100.0` | `snow_opts.newsnw` |
| `ssd` | density threshold controlling settling-factor branch | kg/m^3 | real | 0..1 per file | optional (defaulted if file absent) | finite positive numeric; legacy default `250.0` | `snow_opts.ssd` |

### 5.1 Alias mapping notes
- [DIRECT][E-WF-02] Canonical symbols are the legacy WEPP names (`rst`, `newsnw`, `ssd`) from climate common state.
- [DIRECT][E-WP-01] `wepppy` boundary naming (`snow_opts_*`) is an alias layer and not canonical symbol replacement.
- [INFERENCE][E-WF-03] `newsnw` and `ssd` units are treated as `kg/m^3` by legacy winter-state computations; see conflict register for modern doc/comment inconsistencies.

## 6. Conditional Branches and Optional Sections
1. Presence branch.
- [DIRECT][E-WF-01] If `snow.txt` is missing, legacy assigns defaults and continues.

2. Precipitation partition branch.
- [DIRECT][E-WF-03] During storm-time processing, `hrtemp > rst` routes water to rain branch; otherwise snowfall branch.

3. Settling-factor branch.
- [DIRECT][E-WF-04] Snow settling update checks `densgy > ssd`; if true, settling factor is clamped (`setf=1`) for that step.

4. No optional trailing sections.
- [DIRECT][E-WF-01] Legacy parse contract is fixed three-record scalar input; no additional structured blocks are read.

## 7. Cross-File Consistency Constraints and Coupling Dependencies
1. Management initial-state coupling.
- [DIRECT][E-US-03], [DIRECT][E-WF-07] `snodpy` initial snow depth is provided by management initial-condition records; `snow.txt` coefficients do not define initial snow depth.

2. Climate coupling.
- [DIRECT][E-WF-03] `rst` logic uses hourly temperature (`hrtemp`), which is produced from climate forcing and storm disaggregation.

3. Winter-state coupling.
- [DIRECT][E-WF-05], [DIRECT][E-WF-04] `newsnw` and `ssd` interact with snow-density state (`densnf`, `densg`, `densgy`) used by snow settling/melt routines.

4. Orchestration coupling in modern toolchain.
- [DIRECT][E-WP-03], [DIRECT][E-WP-04] wepppy run prep and payload parsing expose this sidecar as a first-class optional input surface.
- [DIRECT][E-WP3-01] wepppyo3 currently does not claim ownership of sidecar input parsing in its top-level module scope.

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)

| Condition | Legacy behavior | openWEPP typed expectation (draft) |
| --- | --- | --- |
| `snow.txt` missing | [DIRECT][E-WF-01] assign defaults and continue | [INFERENCE][E-WF-01] `OptionalSurfaceMissingDefaulted(surface_id=infile-snow, defaults={rst:0.0,newsnw:100.0,ssd:250.0})` |
| File present, fewer than 3 records | [DIRECT][E-WF-01] read failure path is not explicitly handled | [INFERENCE][E-WF-01] `InputRecordCountError(surface_id=infile-snow, expected=3)` |
| File present, more than 3 records | [DIRECT][E-WF-01] legacy reads only first three records and ignores surplus | strict mode: `InputRecordCountError(surface_id=infile-snow, expected=3)`; compatibility mode: ignore extras with `SurplusRecordWarning` |
| Trailing tokens after primary numeric on a line | [INFERENCE][E-WP-01] modern emitters may include inline annotations | strict mode: `TrailingTokenError(surface_id=infile-snow)`; compatibility mode: allow with `TrailingTokenCompatibilityWarning` |
| Non-numeric token on required line | [DIRECT][E-WF-01] read failure path is not explicitly handled | [INFERENCE][E-WF-01] `TokenParseError(surface_id=infile-snow, line_no=...)` |
| Non-finite numeric (`NaN`/`Inf`) | [INFERENCE][E-WF-01] sidecar records must parse as numeric reals; non-finite values are non-physical parse payloads | [INFERENCE][E-WF-01] `FieldFiniteError(field=...)` |
| Non-positive density values (`newsnw<=0` or `ssd<=0`) | [DIRECT][E-WF-05] densities are physically represented in `kg/m^3` | strict/compat baseline invariant: `FieldRangeError(field=newsnw|ssd)` |
| Positive but extreme density/rst values outside unresolved policy bounds | [DIRECT][E-WP-02] modern toolchain applies additional bounds not yet canonically ratified | strict mode: policy pending `HOLD`; compatibility mode may emit `FieldRangePolicyPendingWarning` |

- [INFERENCE][E-WF-01] Missing-file defaulting is part of legacy behavior and should be explicit in diagnostics, not silent parser mutation.

## 9. Example Snippets

### 9.1 Minimal valid canonical example
```text
0.0
100.0
250.0
```
- [DIRECT][E-WF-01] Matches legacy defaults and read order.

### 9.2 Valid representative example with trailing annotations
```text
-1.5  # rain-snow threshold (deg C)
120.0  # density of new snow
275.0  # settling density threshold
```
- [DIRECT][E-WP-01] Mirrors wepppy emitter style for inline annotations.

### 9.3 Invalid examples
1. Non-numeric token on line 2:
```text
0.0
abc
250.0
```
Reason: `newsnw` must parse as real. [INFERENCE][E-WF-01]

2. Missing line 3:
```text
0.0
100.0
```
Reason: required third record (`ssd`) missing. [DIRECT][E-WF-01]

3. Non-physical density values:
```text
0.0
0.0
-5.0
```
Reason: density parameters must be positive for physically meaningful snow-density behavior. [INFERENCE][E-WF-05]

## 10. Gap / Conflict Register and `HOLD` Conditions

| Gap ID | Statement | Evidence | Provenance tags | Disposition status |
| --- | --- | --- | --- | --- |
| `SNOW-GAP-001` | `usersum2024` does not provide a dedicated `snow.txt` format table, while other sidecars are documented. | [DIRECT][E-US-02] | `usersum2024`, `legacy-code` | `HOLD` until explicit source-authority decision is dispositioned (legacy-code vs supplemental docs). |
| `SNOW-GAP-002` | Unit labeling conflict: wepppy comments describe `newsnw`/`ssd` as `g/cm^3`, while legacy winter-state symbols operate with `kg/m^3` semantics and defaults (`100`, `250`). | [DIRECT][E-WF-02], [DIRECT][E-WF-05], [DIRECT][E-WP-01] | `legacy-code`, `wepppy` | `HOLD` until canonical unit declaration and any conversion policy are dispositioned. |
| `SNOW-GAP-003` | Bounds policy divergence: legacy code uses defaults but no explicit hard bounds; wepppy applies guard bounds for `newsnw` and `ssd`. | [DIRECT][E-WF-01], [DIRECT][E-WP-02] | `legacy-code`, `wepppy` | `HOLD` until openWEPP parser-contract decides normative range enforcement. |
| `SNOW-GAP-004` | `rst` lacks documented canonical bounds in both legacy sidecar read path and modern guard tables. | [DIRECT][E-WF-01], [DIRECT][E-WP-02] | `legacy-code`, `wepppy` | `HOLD` until `SC-INFILE-SNOW-001` defines validated range policy. |

`status` remains `draft-HOLD` until high-impact gaps above are dispositioned.

## 11. Parser-Contract Handoff Map (`SC-INFILE-SNOW-001`)

| Contract area | Source spec requirement | Parser-contract expectation |
| --- | --- | --- |
| Presence/default branch | Section 3 Case A, Section 8 | Implement explicit optional-file defaulting with observable default provenance. |
| Grammar and arity | Section 4 | Parse exactly three real-valued records in fixed order. |
| Symbol continuity | Section 5 | Preserve canonical symbols (`rst`, `newsnw`, `ssd`) with alias mapping. |
| Process coupling | Sections 6-7 | Validate and expose coupling to `hrtemp`, `densnf`, and settling branch behavior. |
| Error semantics | Section 8 | Typed errors for malformed present files; no silent fallback on parse failure. |
| Policy gaps | Section 10 | Carry unresolved gaps as `HOLD` until dispositioned by contract governance. |

### Handoff ID
- `parser_contract_id`: `SC-INFILE-SNOW-001`
- `handoff_status`: `ready-for-contract-authoring (with HOLD gaps carried forward)`
