# WEPP Phosphorus Sidecar Input Specification (`phosphorus.txt`)

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-PHOSPHORUS-001`
- `surface_id`: `infile-phosphorus`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-21T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- [DIRECT][E-US-01] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9413-9558` (sidecar section documents `pmetpara.txt`, `frost.txt`, `tc.txt`, `chan.inp`; no `phosphorus.txt` format appears).
- [DIRECT][E-US-02] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9391-9395` (`wepp_ui.txt` presence-only sidecar documented in same section).
- [DIRECT][E-WF-01] `/workdir/wepp-forest/src/main.for:169-182` (first read path: optional open of `phosphorus.txt`, `p_flag` set to `1` on success, reads header + 4 concentration lines).
- [DIRECT][E-WF-02] `/workdir/wepp-forest/src/main.for:498-510` (second read path in watershed branch, same format and missing-file fallback to `p_flag=0`).
- [DIRECT][E-WF-03] `/workdir/wepp-forest/src/cchrt1.inc:18-21,33,37-39` (canonical legacy symbols `p_flag`, `srp`, `slfp`, `bfp`, `scp`, plus propagated arrays).
- [DIRECT][E-WF-04] `/workdir/wepp-forest/src/wshdrv.f90:470-477` (when `p_flag=1`, scalar concentrations are copied to `tmpsrp/tmpslfp/tmpbfp/tmpscp` for all hillslopes).
- [DIRECT][E-WF-05] `/workdir/wepp-forest/src/wshcqi.f90:108-110,177-181,212-218` (baseflow and runoff phosphorus routing uses propagated concentration arrays).
- [DIRECT][E-WF-06] `/workdir/wepp-forest/docs/config-defaults-and-overrides.md:7-10` (`phosphorus.txt` is optional runtime file; missing => `p_flag=0`).
- [DIRECT][E-WF-07] `/workdir/wepp-forest/src/annchn.for:136-145`, `/workdir/wepp-forest/src/monchn.for:146-156`, `/workdir/wepp-forest/src/endchn.for:133-144` (annual/monthly/average channel reports consume concentration symbols for SRP/PP/TP outputs).
- [DIRECT][E-WP-01] `/workdir/wepppy/wepppy/nodb/core/wepp.py:515-530` (`validate_phosphorus_txt` requires first non-empty line exactly `Phosphorus concentration`, then exactly 4 numeric lines).
- [DIRECT][E-WP-02] `/workdir/wepppy/wepppy/nodb/core/wepp.py:570-577` (`PhosphorusOpts.contents` writes header + 4 lines with units: mg/L, mg/L, mg/L, mg/kg).
- [DIRECT][E-WP-03] `/workdir/wepppy/wepppy/nodb/core/wepp.py:1948-1957` (`_prep_phosphorus` writes `phosphorus.txt`, validates, and deletes file when invalid).
- [DIRECT][E-WP-04] `/workdir/wepppy/wepppy/nodb/core/wepp.py:844-853,1905-1939` (inputs may come from configured values and optional outlet maps before file emission).
- [DIRECT][E-WP-05] `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py:31-34` (web/input payload parses into `phosphorus_opts_*` fields).
- [DIRECT][E-WP3-01] `/workdir/wepppyo3/README.md:66-71,128-146` (`wepppyo3` module scope is climate/raster/interchange; no `phosphorus.txt` sidecar parser surface is declared).
- [DIRECT][E-WP3-02] `/workdir/wepppyo3/docs/wepp-hill-pass-to-swat-rec-spec.md:138` (nutrient fields are currently zero-filled in SWAT recall conversion until explicitly supported).

## 2. Surface Scope and Applicability
- [DIRECT][E-WF-01] `phosphorus.txt` is an optional sidecar file consumed at startup.
- [DIRECT][E-WF-01] Legacy parser shape is five records: one header line (discarded), then four scalar concentrations (`srp`, `slfp`, `bfp`, `scp`).
- [DIRECT][E-WF-04] When enabled (`p_flag=1`), concentrations are propagated to per-hillslope arrays used by watershed routing/reporting.
- [INFERENCE][E-WF-04] This surface is primarily watershed-impacting because routing/reporting consumption shown in evidence is in watershed/channel code paths.
- [DIRECT][E-WP-03] In modern orchestration (`wepppy`), sidecar creation is controlled by option validity and prep lifecycle.

## 3. Version / `datver` Applicability Matrix

| Case | File state | Legacy `wepp-forest` behavior | openWEPP draft stance |
| --- | --- | --- | --- |
| A | `phosphorus.txt` absent | [DIRECT][E-WF-01], [DIRECT][E-WF-02] `open(...,err=...)` falls through to `p_flag=0`; run continues. | [INFERENCE][E-WF-01] Treat as optional sidecar absence with explicit disabled-state signal. |
| B | `phosphorus.txt` present and parseable | [DIRECT][E-WF-01] Reads 1 discarded line + 4 concentration values; sets `p_flag=1`. | [INFERENCE][E-WF-01] Canonical parse path. |
| C | `phosphorus.txt` present but malformed | [DIRECT][E-WF-01] No explicit `err=` on `read`; runtime read failure behavior is implementation-dependent. | [INFERENCE][E-WF-01] Must raise typed parse/record errors; do not silently continue with partial state. |

- [DIRECT][E-WF-01] No `datver` token is read for this sidecar.
- [DIRECT][E-US-01] `usersum2024` does not define a `phosphorus.txt` format section.

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical grammar (draft)
```ebnf
phosphorus_file = header_line srp_line slfp_line bfp_line scp_line ;
header_line     = text ;
srp_line        = real [trailing_tokens] ;
slfp_line       = real [trailing_tokens] ;
bfp_line        = real [trailing_tokens] ;
scp_line        = real [trailing_tokens] ;
```

- [DIRECT][E-WF-01] Legacy parser consumes one first record with `read(22,*)` and does not bind it to a variable.
- [DIRECT][E-WF-01] Concentration records are read in fixed order into `srp`, `slfp`, `bfp`, `scp`.
- [DIRECT][E-WP-02] Producer formatting commonly appends descriptive trailing text after first numeric token per line.

### 4.2 Line definitions
- Line 1: header/title text. [DIRECT][E-WF-01], [DIRECT][E-WP-01]
- Line 2: `srp` surface runoff concentration (mg/L). [DIRECT][E-WF-01]
- Line 3: `slfp` subsurface lateral-flow concentration (mg/L). [DIRECT][E-WF-01]
- Line 4: `bfp` baseflow concentration (mg/L). [DIRECT][E-WF-01]
- Line 5: `scp` sediment concentration (mg/kg). [DIRECT][E-WF-01]

## 5. Field Dictionary With Canonical Symbols and Alias Mapping

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Constraints (draft) | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `p_flag` | phosphorus sidecar enable flag | none | integer | 1 per run | derived | `0` when missing/unreadable sidecar; `1` when sidecar read succeeds | `phosphorus.enabled` |
| `srp` | surface runoff concentration | mg/L | real | 0..1 per file | yes when file present | finite numeric; range policy pending disposition | `phosphorus.surface_runoff_mg_l` |
| `slfp` | subsurface lateral-flow concentration | mg/L | real | 0..1 per file | yes when file present | finite numeric; range policy pending disposition | `phosphorus.lateral_flow_mg_l` |
| `bfp` | baseflow concentration | mg/L | real | 0..1 per file | yes when file present | finite numeric; range policy pending disposition | `phosphorus.baseflow_mg_l` |
| `scp` | sediment concentration | mg/kg | real | 0..1 per file | yes when file present | finite numeric; range policy pending disposition | `phosphorus.sediment_mg_kg` |
| `tmpsrp/tmpslfp/tmpbfp/tmpscp` | propagated per-hillslope concentrations | mixed | real arrays | one per hillslope element | derived when `p_flag=1` | copied from scalars at startup | `phosphorus.by_hillslope.*` |

### 5.1 Alias Mapping Notes
- [DIRECT][E-WF-03] Legacy symbols (`srp`, `slfp`, `bfp`, `scp`) are canonical for provenance continuity.
- [INFERENCE][E-WF-03] openWEPP alias names are boundary-layer transport names only; they do not replace canonical symbol naming in contracts.

## 6. Conditional Branches and Optional Sections
1. Presence branch.
- [DIRECT][E-WF-01] File missing => `p_flag=0`; run continues.
- [DIRECT][E-WF-01] File present and readable => `p_flag=1`.

2. Run-mode read-callsite branch.
- [DIRECT][E-WF-01], [DIRECT][E-WF-02] Two startup callsites read the same 5-record shape.

3. Routing/reporting branch.
- [DIRECT][E-WF-04] `p_flag=1` gates propagation into `tmps*` concentration arrays.
- [DIRECT][E-WF-05], [DIRECT][E-WF-07] Routing and reporting consume these propagated concentrations for SRP/PP/TP computations.

4. Optional sidecar behavior.
- [INFERENCE][E-WF-01] No additional structured blocks are defined; sidecar contract is fixed 5-line scalar payload.

## 7. Cross-File Consistency Constraints and Coupling Dependencies
1. Channel routing coupling.
- [DIRECT][E-WF-05] `tmpbfp`, `tmpsrp`, `tmpslfp` feed watershed phosphorus flux terms; parser unit/type fidelity propagates directly to mass outputs.

2. Reporting coupling.
- [DIRECT][E-WF-07] Annual/monthly/end-of-run channel summaries use the same concentration symbols for soluble/particulate phosphorus totals.

3. Orchestration-to-sidecar coupling.
- [DIRECT][E-WP-04], [DIRECT][E-WP-03] `wepppy` may source values from config/maps, then writes/validates `phosphorus.txt` before execution.

4. Surface registry/usersum coupling.
- [DIRECT][E-US-01] Because `usersum2024` sidecar section omits `phosphorus.txt`, canonical behavior relies on legacy/static provenance until a higher-order source is identified.

5. Ecosystem scope coupling.
- [DIRECT][E-WP3-01], [DIRECT][E-WP3-02] `wepppyo3` currently documents interchange/output behavior, not `phosphorus.txt` parsing authority.

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)

| Condition | Legacy behavior | openWEPP typed expectation (draft) |
| --- | --- | --- |
| `phosphorus.txt` missing | [DIRECT][E-WF-01], [DIRECT][E-WF-06] `p_flag=0`, continue run | [INFERENCE][E-WF-01] `OptionalSurfaceMissingDefaulted(surface_id=infile-phosphorus, enabled=false)` |
| File present, wrong record count | [DIRECT][E-WF-01] no explicit record-count guard | [INFERENCE][E-WF-01] `InputRecordCountError(surface_id=infile-phosphorus, expected=5)` |
| Non-numeric concentration token | [DIRECT][E-WF-01] Fortran list-directed read failure behavior | [INFERENCE][E-WF-01] `TokenParseError(surface_id=infile-phosphorus, field=...)` |
| Non-finite numeric (`NaN`/`Inf`) | [INFERENCE][E-WP-01] modern layers parse first tokens as float | [INFERENCE][E-WP-01] `FieldFiniteError(field=...)` |
| Header mismatch (`Phosphorus concentration` vs other text) | [DIRECT][E-WF-01] legacy ignores header contents; [DIRECT][E-WP-01] wepppy validator requires exact string | [INFERENCE][E-WP-01] policy conflict; keep `HOLD` until strict/compat mode disposition |

- [INFERENCE][E-WF-01] Parser failures on present files should be explicit typed errors rather than silent disablement.

## 9. Example Snippets

### 9.1 Minimal valid canonical example
```text
Phosphorus concentration
0.01
0.005
0.002
250.0
```
- [DIRECT][E-WP-02] Matches modern writer shape and units.

### 9.2 Representative example with trailing descriptions
```text
Phosphorus concentration
0.01    Surface runoff concentration (mg/l)
0.005   Subsurface lateral flow concentration (mg/l)
0.002   Baseflow concentration (mg/l)
250.0   Sediment concentration (mg/kg)
```
- [DIRECT][E-WP-02] Mirrors emitted line style with trailing comments.

### 9.3 Invalid examples
1. Too few concentration records:
```text
Phosphorus concentration
0.01
0.005
0.002
```
Reason: missing `scp` record. [INFERENCE][E-WF-01]

2. Non-numeric concentration:
```text
Phosphorus concentration
0.01
abc
0.002
250.0
```
Reason: `slfp` must parse as real. [INFERENCE][E-WF-01]

3. Non-finite value:
```text
Phosphorus concentration
0.01
0.005
NaN
250.0
```
Reason: concentration fields must be finite numeric values. [INFERENCE][E-WP-01]

## 10. Gap / Conflict Register and `HOLD` Conditions

| Gap ID | Statement | Evidence | Disposition status |
| --- | --- | --- | --- |
| `PHOS-GAP-001` | `usersum2024` does not provide a `phosphorus.txt` format specification, even though it documents nearby sidecars. | [DIRECT][E-US-01] | `HOLD` until higher-rank format authority is identified or legacy-source authority is formally ratified. |
| `PHOS-GAP-002` | Header policy conflict: legacy ignores first-line contents while wepppy validator requires exact literal `Phosphorus concentration`. | [DIRECT][E-WF-01], [DIRECT][E-WP-01] | `HOLD` until strict-vs-compat parse mode is dispositioned. |
| `PHOS-GAP-003` | Concentration range bounds are not explicitly defined in legacy parser path; modern validity checks are type-only for file shape. | [DIRECT][E-WF-01], [DIRECT][E-WP-01] | `HOLD` until contract-level physical/risk bounds are set. |
| `PHOS-GAP-004` | `wepppyo3` currently provides interchange surfaces but no declared ownership for phosphorus input parsing. | [DIRECT][E-WP3-01], [DIRECT][E-WP3-02] | `HOLD` for provenance-completeness tracking; does not block openWEPP spec authority. |
| `PHOS-GAP-005` | Primary demonstrated consumption is watershed/channel routing/reporting; hillslope-only applicability semantics are not explicitly documented. | [DIRECT][E-WF-04], [DIRECT][E-WF-05], [DIRECT][E-WF-07] | `HOLD` until applicability matrix is finalized in parser contract. |

`status` remains `draft-HOLD` until high-impact gaps are dispositioned.

## 11. Parser-Contract Handoff Map (`SC-INFILE-PHOSPHORUS-001`)

| Contract area | Source spec requirement | Parser-contract expectation |
| --- | --- | --- |
| Optional-file semantics | Sections 3, 6, 8 | Explicit missing-file disabled state (`enabled=false`) with observable provenance. |
| Grammar/arity | Section 4 | Parse exactly 5 records (header + 4 concentrations) in fixed order. |
| Symbol continuity | Section 5 | Preserve canonical symbols (`srp`, `slfp`, `bfp`, `scp`, `p_flag`) with alias mapping. |
| Coupling/propagation | Section 7 | Preserve unit-consistent propagation into watershed routing/reporting state surfaces. |
| Error handling | Section 8 | Typed parse/count/finite errors for malformed present files; no silent mutation. |
| Conflict carry-forward | Section 10 | Carry unresolved gaps as `HOLD` until disposition closes policy conflicts. |

### Handoff ID
- `parser_contract_id`: `SC-INFILE-PHOSPHORUS-001`
- `handoff_status`: `ready-for-contract-authoring (with HOLD gaps carried forward)`
- `linked_work_package`: `docs/work-packages/20260520-infile11-author-sc-infile-phosphorus-001/`
