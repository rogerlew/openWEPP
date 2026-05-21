# WEPP Groundwater Coefficients Sidecar Input Specification (`gwcoeff.txt`)

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-GWCOEFF-001`
- `surface_id`: `infile-gwcoeff`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-21T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- [DIRECT][E-US-01] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9413-9453` (`pmetpara.txt` and `frost.txt` sidecar format sections are present).
- [DIRECT][E-US-02] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9518-9557` (`tc.txt`, `wepp_ch.txt`, and `chan.inp` are documented in the sidecar section; no `gwcoeff.txt` section appears there).
- [DIRECT][E-US-03] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9555` (`chan.inp` line-2 "unit area baseflow coefficient" is documented as `m^3/s/m^2`, which is distinct from `gwcoeff.txt` coefficients).
- [DIRECT][E-WF-01] `/workdir/wepp-forest/src/main.for:140-160` (legacy reads `gwcoeff.txt` as four ordered records: `igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`; file-missing branch sets `lr_bf=0`).
- [DIRECT][E-WF-02] `/workdir/wepp-forest/src/main.for:474-489` (same 4-record parse contract in watershed prompt path).
- [DIRECT][E-WF-03] `/workdir/wepp-forest/src/cchrt1.inc:46-52` (canonical symbol meanings and units: `igwstrd` mm, `bfcoeff` per day, `dscoeff` per day, `bftharea` ha).
- [DIRECT][E-WF-04] `/workdir/wepp-forest/src/contin.for:1106-1131` (daily groundwater storage, baseflow, and deep seepage calculations using `bfcoeff` and `dscoeff` when `lr_bf=1`).
- [DIRECT][E-WF-05] `/workdir/wepp-forest/src/wshchr.f90:157-170` and `:229-236` (`lr_bf` branch switches baseflow source; `bftharea` suppresses routed baseflow below threshold area).
- [DIRECT][E-WF-06] `/workdir/wepp-forest/src/wshdrv.f90:457-462` (`lr_bf=1` emits `chntyp.txt` diagnostic output for channel type tagging).
- [DIRECT][E-WF-07] `/workdir/wepp-forest/tests/fixtures/up_pedantry_h5/runs/gwcoeff.txt:1-4` (observed 4-line data-plus-description file shape).
- [DIRECT][E-WP-01] `/workdir/wepppy/wepppy/nodb/core/wepp.py:460-510` (`BaseflowOpts` default values and serialized `gwcoeff.txt` line content).
- [DIRECT][E-WP-02] `/workdir/wepppy/wepppy/nodb/core/wepp.py:2013-2022` (`_prep_baseflow` writes `gwcoeff.txt`; single-storm path overrides to `gwstorage=0.0`, `bfcoeff=0.0`).
- [DIRECT][E-WP-03] `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py:30-33` (input parser routes request values into `baseflow_opts`).
- [DIRECT][E-WP-04] `/workdir/wepppy/tests/wepp/interchange/fixtures/deductive-futurist/wepp/runs/gwcoeff.txt:1-4` (modern fixture matches 4-line shape and labels).
- [DIRECT][E-WP3-01] `/workdir/wepppyo3/README.md:68-73` (`wepppyo3` module scope emphasizes climate/interchange modules; no owned `gwcoeff.txt` parser surface is listed).
- [DIRECT][E-WP3-02] `/workdir/wepppyo3/wepp_interchange/src/schema.rs:195-203` and `/workdir/wepppyo3/wepp_interchange/src/pass.rs:432-441` (`wepppyo3` handles baseflow/deep-seepage output semantics, not `gwcoeff.txt` input parsing).

## 2. Surface Scope and Applicability
- [DIRECT][E-WF-01] `gwcoeff.txt` is an optional sidecar: file presence sets `lr_bf=1`; absence leaves `lr_bf=0`.
- [DIRECT][E-WF-01] Legacy read order is fixed and positional with four records: `igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`.
- [DIRECT][E-WF-03] Canonical variable meanings and units are defined in the legacy common block include (`cchrt1.inc`).
- [INFERENCE][E-WF-04] Applicability is watershed and hillslope-pass groundwater/baseflow pathways because parsed values feed channel-routing/baseflow state via `contin.for` and watershed routing.
- [DIRECT][E-WP-02] In modern orchestration, this sidecar is actively written by prep logic and can be climate-mode conditioned (single-storm override).

## 3. Version / `datver` Applicability Matrix

| Case | File state | Legacy behavior | openWEPP draft interpretation |
| --- | --- | --- | --- |
| A | `gwcoeff.txt` absent | [DIRECT][E-WF-01] open `err=` branch sets `lr_bf=0` and continues. | [INFERENCE][E-WF-01] represent as explicit optional-surface absence branch (not parse failure). |
| B | `gwcoeff.txt` present with 4 numeric-leading records | [DIRECT][E-WF-01] sequential list-directed reads set `igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`. | [INFERENCE][E-WF-01] canonical parse path. |
| C | `gwcoeff.txt` present but malformed/incomplete | [DIRECT][E-WF-01] no per-read `err=` handlers; failure behavior is runtime-IO dependent. | [INFERENCE][E-WF-01] must raise typed parse/record errors; no silent fallback after malformed present file. |
| D | `datver`/version-prefixed variant | [DIRECT][E-WF-01] legacy parser does not read a version line for this sidecar. | [INFERENCE][E-WF-01] reject as format error unless explicit compatibility mode is approved. |

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical grammar (draft)

```ebnf
gwcoeff_file   = igwstrd_line bfcoeff_line dscoeff_line bftharea_line ;
igwstrd_line   = real [trailing_text] ;
bfcoeff_line   = real [trailing_text] ;
dscoeff_line   = real [trailing_text] ;
bftharea_line  = real [trailing_text] ;
```

- [DIRECT][E-WF-01] Legacy reads exactly four scalar records in fixed sequence.
- [DIRECT][E-WF-07] Observed runtime fixtures include descriptive trailing text after the numeric token.
- [INFERENCE][E-WF-01] Tokenization must accept list-directed numeric-leading records with optional trailing labels/comments.

### 4.2 Line definitions
- Line 1: `igwstrd` initial groundwater depth. [DIRECT][E-WF-01], [DIRECT][E-WF-03]
- Line 2: `bfcoeff` baseflow coefficient. [DIRECT][E-WF-01], [DIRECT][E-WF-03]
- Line 3: `dscoeff` deep seepage coefficient. [DIRECT][E-WF-01], [DIRECT][E-WF-03]
- Line 4: `bftharea` watershed baseflow threshold area. [DIRECT][E-WF-01], [DIRECT][E-WF-03]

## 5. Field Dictionary With Canonical Symbols and Alias Mapping

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Constraints (draft) | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `igwstrd` | initial groundwater depth | mm | real | 0..1 per file | yes when file present | finite, non-negative | `groundwater.initial_storage_depth_mm` |
| `bfcoeff` | baseflow coefficient | day^-1 | real | 0..1 per file | yes when file present | finite, non-negative | `groundwater.baseflow_coeff_per_day` |
| `dscoeff` | deep seepage coefficient | day^-1 | real | 0..1 per file | yes when file present | finite, non-negative | `groundwater.deep_seepage_coeff_per_day` |
| `bftharea` | watershed baseflow threshold area | ha | real | 0..1 per file | yes when file present | finite, non-negative | `groundwater.baseflow_threshold_area_ha` |
| `lr_bf` | linear-reservoir baseflow flag derived from file presence | flag | integer | runtime state | derived | `0` when file absent; `1` when present and parsed | `groundwater.linear_reservoir_enabled` |

### 5.1 Alias mapping notes
- [DIRECT][E-WF-03] Canonical symbols remain `igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`, `lr_bf`.
- [DIRECT][E-WP-01] Modern boundary names (`gwstorage`, `bfthreshold`) are aliases and must not replace canonical symbol provenance.
- [INFERENCE][E-WF-05] Alias consumers must preserve the threshold-area semantics used in channel routing (`wsarea/10000 < bftharea` tests).

## 6. Conditional Branches and Optional Sections
1. Presence/absence branch.
- [DIRECT][E-WF-01] Missing file sets `lr_bf=0` and bypasses linear-reservoir sidecar ingestion.

2. Routing-method branch (`lr_bf`).
- [DIRECT][E-WF-05] `lr_bf=0` path uses unit-area baseflow (`cbase * area`) behavior.
- [DIRECT][E-WF-05] `lr_bf=1` path uses hillslope-passed groundwater baseflow volumes (`tmpgwbfv(...)`).

3. Threshold-area suppression branch.
- [DIRECT][E-WF-05] When `(wsarea/10000) < bftharea`, routed baseflow contribution is set to zero for that element path.

4. Modern climate-mode branch.
- [DIRECT][E-WP-02] In single-storm mode, wepppy writes overridden baseflow settings (`gwstorage=0.0`, `bfcoeff=0.0`) before materializing `gwcoeff.txt`.

## 7. Cross-File Consistency Constraints and Coupling Dependencies
1. Watershed geometry coupling.
- [DIRECT][E-WF-05] `bftharea` is evaluated against `wsarea` (converted to ha), coupling this sidecar to watershed-structure-derived area state.

2. Groundwater/baseflow process coupling.
- [DIRECT][E-WF-04] `igwstrd`, `bfcoeff`, `dscoeff` drive daily groundwater storage, baseflow, and deep seepage volumes written into hillslope-pass/watershed-routing pathways.

3. Diagnostic/output coupling.
- [DIRECT][E-WF-06] `lr_bf=1` triggers `chntyp.txt` emission, so parsing decisions affect downstream diagnostics.

4. Configuration coupling in orchestration.
- [DIRECT][E-WP-01], [DIRECT][E-WP-02], [DIRECT][E-WP-03] wepppy request parsing and prep-writing are coupled to this sidecar schema (`baseflow_opts_*` -> `gwcoeff.txt`).

5. Channel-routing parameter coupling/conflict risk.
- [DIRECT][E-US-03] `chan.inp` also defines a "baseflow coefficient" with different units/meaning than `gwcoeff.txt` `bfcoeff`.
- [INFERENCE][E-US-03] Contracts must prohibit alias conflation between `chan.inp` line-2 coefficient and `gwcoeff.txt` line-2 coefficient.

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)

| Condition | Legacy behavior | openWEPP typed expectation (draft) |
| --- | --- | --- |
| `gwcoeff.txt` missing | [DIRECT][E-WF-01] set `lr_bf=0`; continue without sidecar parse. | [INFERENCE][E-WF-01] `OptionalSurfaceMissingDefaulted(surface_id=infile-gwcoeff)` with explicit observability event. |
| Present file with fewer than 4 records | [DIRECT][E-WF-01] no explicit read error branch for malformed present files. | [INFERENCE][E-WF-01] `InputRecordCountError(surface_id=infile-gwcoeff, expected=4)`. |
| Non-numeric leading token on required line | [DIRECT][E-WF-01] list-directed read failure path not explicitly handled. | [INFERENCE][E-WF-01] `TokenParseError(surface_id=infile-gwcoeff, line_no=...)`. |
| Non-finite numeric (`NaN`/`Inf`) | [INFERENCE][E-WP-03] modern request path accepts float coercions before writing. | [INFERENCE][E-WP-03] `FieldFiniteError(field=...)`. |
| Negative coefficient/depth/threshold | [DIRECT][E-WF-03] units imply physical non-negativity; legacy code does not clamp here. | [INFERENCE][E-WF-03] `FieldRangeError(field=...)` in strict mode; compat mode requires explicit disposition. |

## 9. Example Snippets

### 9.1 Minimal valid canonical example

```text
200.0
0.04
0.0
1.0
```

- [DIRECT][E-WF-01] Matches legacy four-line parse sequence.

### 9.2 Representative example with descriptive trailing text

```text
200.0	Initial groundwater storage (mm)
0.04	Baseflow coefficient (per day)
0.0	Deep seepage coefficient (per day)
1.0	Watershed groundwater baseflow threshold area (ha)
```

- [DIRECT][E-WF-07], [DIRECT][E-WP-04] Matches observed fixture conventions.

### 9.3 Invalid examples

1. Missing required record:
```text
200.0
0.04
0.0
```
Reason: line-4 `bftharea` missing. [DIRECT][E-WF-01]

2. Non-numeric leading token:
```text
abc
0.04
0.0
1.0
```
Reason: `igwstrd` must parse as real. [INFERENCE][E-WF-01]

3. Negative threshold:
```text
200.0
0.04
0.0
-1.0
```
Reason: threshold area must be non-negative in strict semantics. [INFERENCE][E-WF-03]

## 10. Gap / Conflict Register and `HOLD` Conditions

| Gap ID | Statement | Evidence | Disposition status |
| --- | --- | --- | --- |
| `GWCOEFF-GAP-001` | `usersum2024` sidecar section does not publish a dedicated `gwcoeff.txt` format definition. | [DIRECT][E-US-01], [DIRECT][E-US-02] | `HOLD` until formal source-authority disposition records the legacy-code-first basis for this surface. |
| `GWCOEFF-GAP-002` | Coefficient-name collision risk: `chan.inp` line-2 "unit area baseflow coefficient" differs semantically/units from `gwcoeff.txt` `bfcoeff`. | [DIRECT][E-US-03], [DIRECT][E-WF-03] | `HOLD` until parser contracts define explicit namespace separation and guard checks. |
| `GWCOEFF-GAP-003` | Legacy missing-file behavior toggles `lr_bf` but does not document normative default values for all four fields in core source comments. | [DIRECT][E-WF-01], [DIRECT][E-WF-03], [DIRECT][E-WP-01] | `HOLD` until openWEPP default-policy decision is dispositioned (strict optional absence vs explicit value defaults). |
| `GWCOEFF-GAP-004` | Legacy present-file parse failure semantics are implicit (no per-read `err=` branch). | [DIRECT][E-WF-01] | `HOLD` until typed error mapping and compat policy are codified in `SC-INFILE-GWCOEFF-001`. |
| `GWCOEFF-GAP-005` | `wepppyo3` provenance currently covers output/interchange baseflow fields, not `gwcoeff.txt` input parsing ownership. | [DIRECT][E-WP3-01], [DIRECT][E-WP3-02] | `HOLD` for ownership clarity only; not a blocker for openWEPP canonical spec authority. |

`status` remains `draft-HOLD` until high-impact gaps above are dispositioned.

## 11. Parser-Contract Handoff Map (`SC-INFILE-GWCOEFF-001`)

| Contract area | Source spec requirement | Parser-contract expectation |
| --- | --- | --- |
| Optional-surface behavior | Sections 2, 3, 8 | Model missing-file branch explicitly (`lr_bf` disabled) with observable provenance event. |
| Grammar and arity | Section 4 | Parse exactly four ordered numeric-leading records. |
| Canonical symbols and aliases | Section 5 | Preserve `igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`, `lr_bf` with alias mapping only at boundaries. |
| Routing semantics coupling | Sections 6, 7 | Enforce `bftharea` threshold semantics and `lr_bf` branch behavior in downstream routing state contracts. |
| Typed errors | Section 8 | No silent fallback on malformed present files; produce typed parse/range errors. |
| Known unresolved conflicts | Section 10 | Carry unresolved items as explicit `HOLD` obligations in `SC-INFILE-GWCOEFF-001`. |

### Handoff ID
- `parser_contract_id`: `SC-INFILE-GWCOEFF-001`
- `handoff_status`: `ready-for-contract-authoring (with HOLD gaps carried forward)`
