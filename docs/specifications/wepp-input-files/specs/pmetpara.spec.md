# WEPP PMET Parameter Input File Specification (`pmetpara.txt`)

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-PMETPARA-001`
- `surface_id`: `infile-pmetpara`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-20T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- [DIRECT][E-US-01] `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf` (sidecar section for `pmetpara.txt`, p.94 in August 2024 usersum pagination).
- [DIRECT][E-US-02] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9413-9432` (`pmetpara.txt` purpose and line-by-line fields).
- [DIRECT][E-WF-01] `/workdir/wepp-forest/src/infile.for:1538-1557` (presence of `pmetpara.txt` switches ET mode to FAO Penman-Monteith via `iflget=2`; missing file falls back to `iflget=1`).
- [DIRECT][E-WF-02] `/workdir/wepp-forest/src/pmetcoef.for:56-64` (legacy parser reads line-1 record count, scans crop names, then reads `names,kcb,rawp,line,actlnam`).
- [DIRECT][E-WF-03] `/workdir/wepp-forest/src/pmetcoef.for:69-79` (if crop not found, legacy rewinds and uses first record as fallback).
- [DIRECT][E-WF-04] `/workdir/wepp-forest/src/pmetcoef.for:20,35-37` (legacy symbol widths: `cropname`/`names` are `character*8`, `actlnam` is `character*20`, `line` is integer).
- [DIRECT][E-WF-05] `/workdir/wepp-forest/src/ccrpet.inc:7,15-18` (legacy storage symbols `kcb(mxcrop)` and `rawp(mxcrop)`).
- [DIRECT][E-WF-06] `/workdir/wepp-forest/tests/fixtures/up_pedantry_h5/runs/pmetpara.txt:1-12` (real corpus uses line-1 count then comma-delimited 5-field records).
- [DIRECT][E-WP-01] `/workdir/wepppy/wepppy/wepp/management/pmetpara.py:15-42` (modern writer emits line-1 count and comma-delimited `plant,kcb,rawp,index,description` rows).
- [DIRECT][E-WP-02] `/workdir/wepppy/wepppy/nodb/core/wepp.py:1850-1897` (sidecar lifecycle in orchestration: prepare/remove `pmetpara.txt`).
- [DIRECT][E-WP-03] `/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py:1648-1740` (disturbed path writes `pmetpara.txt` row-per-domain and default coefficients).
- [DIRECT][E-WP-04] `/workdir/wepppy/tests/nodb/mods/disturbed/live_e2e/runbook.py:92-110` (consumer expectation of comma-delimited 5-column payload after count line).
- [DIRECT][E-WP3-01] `/workdir/wepppyo3/README.md:68-78` and `:128-167` (`wepppyo3` documented API surface does not currently claim `pmetpara.txt` parsing/writing ownership).

## 2. Surface Scope and Applicability
- [DIRECT][E-US-02] `pmetpara.txt` is an optional sidecar that signals use of FAO Penman-Monteith dual-coefficient ET.
- [DIRECT][E-WF-01] Legacy behavior is file-presence gated: present -> `iflget=2`, absent -> `iflget=1`.
- [INFERENCE][E-WF-01] Applicability is hillslope/watershed runs that include crop ET evaluation; this file is a mode override, not a required baseline input.
- [INFERENCE][E-WP-02] In modern orchestration, this sidecar is controlled by runtime flags and prep flows rather than mandatory static project assets.

## 3. Version/datver Applicability Matrix

| Input form | First non-comment line | Legacy interpretation | openWEPP draft interpretation | Evidence |
| --- | --- | --- | --- | --- |
| Canonical current | Integer record count (`irecord`) | Parsed by `pmetcoef` as line-1 count | `MUST` support | [DIRECT][E-US-02], [DIRECT][E-WF-02] |
| Missing sidecar | File absent | ET mode reverts to original Penman (`iflget=1`) | `MUST` represent as explicit optional-mode branch | [DIRECT][E-WF-01] |
| Datver-prefixed variant | Version token line before count | Not documented for this sidecar; not parsed in legacy code | `MUST` reject as format error unless future disposition adds compatibility mode | [INFERENCE][E-WF-02] |

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical Grammar (normative draft)

```ebnf
pmetpara_file   = irecord_line parameter_record{irecord} ;
irecord_line    = integer ;
parameter_record = crop_name kcb rawp line actlnam ;
```

- [DIRECT][E-US-02] Line 1 is number of records.
- [DIRECT][E-US-02] Each data record contains `crop_name`, `kcb`, `rawp`, `line`, `logical name/comment`.
- [DIRECT][E-WF-02] Legacy parser reads these fields with list-directed reads.
- [INFERENCE][E-WF-02] Token delimiters should be treated as list-directed compatible (comma and/or whitespace).
- [DIRECT][E-WF-06] Field corpora in legacy fixture files are comma-delimited.

### 4.2 Line definitions
- Line 1: `irecord` integer, declared number of data rows. [DIRECT][E-US-02]
- Lines 2..N: `names, kcb, rawp, line, actlnam`. [DIRECT][E-WF-02]

## 5. Field Dictionary With Canonical Symbols and Alias Mapping

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Constraints | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `irecord` | number of parameter rows in file | count | integer | 1 per file | yes when file present | `>= 1`; must equal actual row count | `pmetpara.record_count` |
| `names` | crop name key | none | string | 1 per row | yes | must correspond to management crop name intent | `pmetpara.records[i].crop_name` |
| `kcb` | mid-season basal crop coefficient | none | real | 1 per row | yes | finite numeric; physically plausible range policy pending disposition | `pmetpara.records[i].kcb` |
| `rawp` | readily available root-zone soil-water coefficient `p` | none | real | 1 per row | yes | finite numeric; physically plausible range policy pending disposition | `pmetpara.records[i].rawp` |
| `line` | row index token (legacy not used) | count | integer | 1 per row | yes (legacy row shape) | value is informational; legacy runtime does not use it | `pmetpara.records[i].line_index` |
| `actlnam` | logical name/comment text | none | string | 1 per row | yes (legacy row shape) | legacy local width is `character*20` | `pmetpara.records[i].description` |

### 5.1 Alias Mapping Notes
- [DIRECT][E-WF-02] Legacy symbols in read path are `irecord`, `names`, `kcb`, `rawp`, `line`, `actlnam`.
- [INFERENCE][E-WF-02] openWEPP aliases are transport names only; canonical provenance stays with legacy symbol names.
- [DIRECT][E-WF-04] Symbol width continuity matters for compatibility: `names` and lookup key are `character*8`, `actlnam` is `character*20` in legacy parser.

### 5.2 Deterministic crop-key normalization and match policy (draft)
- Strict mode:
  - Use trimmed keys without implicit truncation.
  - Use case-sensitive matching on trimmed keys (no implicit case folding).
  - Require `names` and lookup crop key widths `<= 8` characters; otherwise `CropNameWidthError`.
  - Match policy is exact-key match on normalized key.
- Compatibility mode:
  - Apply uppercase normalization before width handling to emulate legacy corpus conventions.
  - Emulate fixed-width compatibility by truncating keys to 8 characters for lookup parity with legacy `character*8` symbols.
  - Emit `CropNameTruncationWarning` when truncation occurs.
- [DIRECT][E-WF-04] Width constraints are anchored in legacy symbol declarations.
- [INFERENCE][E-WF-03] Explicit match policy is required to avoid ambiguous first-row fallback behavior.

## 6. Conditional Branches and Optional Sections
1. Sidecar presence branch.
- Present `pmetpara.txt`: Penman-Monteith mode selected (`iflget=2`).
- Missing `pmetpara.txt`: original Penman mode (`iflget=1`).
- Parser contract boundary must export both `sidecar_present` and resolved `iflget` mode value; this branch is not implicit.
- [DIRECT][E-WF-01]

2. Crop-name lookup branch per crop.
- Exact name hit: use matching row for that crop.
- No name hit: fallback to first row and emit notice.
- [DIRECT][E-WF-02], [DIRECT][E-WF-03]

3. Authoring branch in modern pipeline.
- Default/scalar, map-derived, and disturbed-derived row generation paths all materialize canonical 5-field rows.
- [DIRECT][E-WP-01], [DIRECT][E-WP-02], [DIRECT][E-WP-03]

## 7. Cross-File Consistency Constraints and Coupling Dependencies
1. Management crop-name coupling.
- `names` should align with crop names used by management input.
- [DIRECT][E-US-02]

2. Crop-loop coverage coupling.
- Legacy runtime calls `pmetcoef` once per crop (`1..ncrop`), so row coverage or fallback behavior affects each crop ET parameterization.
- [DIRECT][E-WF-01], [DIRECT][E-WF-03]

3. ET mode coupling.
- Presence/absence of `pmetpara.txt` changes the ET formulation branch globally through `iflget`.
- [DIRECT][E-WF-01]

4. Sidecar lifecycle coupling in orchestration.
- Runtime prep/removal of `pmetpara.txt` must remain synchronized with run option flags.
- [DIRECT][E-WP-02]

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)

Legacy behavior includes fallback semantics that are not strict-format safe for openWEPP by default.

| Condition | Legacy behavior | openWEPP parser-contract draft expectation |
| --- | --- | --- |
| `pmetpara.txt` missing | ET reverts to non-PMET branch (`iflget=1`) | represent as optional sidecar absence, not parse failure |
| PMET-required strict mode + `pmetpara.txt` missing | legacy often falls back to `iflget=1` | strict mode: `PMETRequiredSidecarMissingError(surface_id=infile-pmetpara)` |
| Datver-prefixed header line present before `irecord` | not parsed by legacy PMET reader path | `FormatVersionLineUnsupportedError(surface_id=infile-pmetpara)` |
| `irecord` row count mismatch | undefined/IO-failure-prone | `RecordCountError(surface_id=infile-pmetpara)` |
| crop name not found | silently uses first row after notice | `CropNameMissingError` in strict mode; optional legacy-compat fallback mode must be explicit |
| crop key width exceeds legacy-compatible width (`>8`) | legacy fixed-width symbols may truncate/alter match behavior | strict mode: `CropNameWidthError`; compatibility mode: truncate with `CropNameTruncationWarning` |
| non-numeric `kcb`/`rawp`/`line` | list-directed read failure | `TokenParseError(field=...)` |
| zero/negative `irecord` | undefined behavior | `FieldRangeError(field=irecord)` |
| duplicate crop-name rows | first encountered match in scan order | `DuplicateCropKeyError` (strict) or deterministic first-row policy (compat), pending disposition |

- [DIRECT][E-WF-03] legacy no-hit fallback is observable behavior.
- [INFERENCE][E-WF-03] openWEPP should expose fallback choice as explicit contract policy rather than implicit mutation.

## 9. Example Snippets

### 9.1 Minimal valid canonical example

```text
1
WHEAT,1.05,0.45,1,default
```

- [DIRECT][E-US-02] Matches 1-count + 5-field row structure.

### 9.2 Representative multi-row example

```text
3
CORN,1.20,0.55,1,loam-annual
WHEAT,1.05,0.45,2,silt_loam-cover
ALFALFA,1.15,0.50,3,perennial
```

- [DIRECT][E-WP-01] Mirrors modern comma-delimited writer form.

### 9.3 Invalid examples

1. Invalid row-count mismatch
```text
2
CORN,1.20,0.55,1,loam-annual
```
Reason: `irecord` does not match actual records. [INFERENCE][E-WF-02]

2. Invalid numeric token
```text
1
CORN,abc,0.55,1,loam-annual
```
Reason: `kcb` must parse as real. [INFERENCE][E-WF-02]

3. Invalid missing required field
```text
1
CORN,1.20,0.55,1
```
Reason: canonical row requires 5 fields. [DIRECT][E-US-02]

## 10. Gap/Conflict Register and HOLD Conditions

| ID | Issue | Evidence | Provenance tags | Draft disposition |
| --- | --- | --- | --- | --- |
| `PMET-GAP-001` | `usersum2024` says crop name should match management crop name, but legacy runtime silently falls back to first row when not found. | [DIRECT][E-US-02], [DIRECT][E-WF-03] | `usersum2024`, `legacy-code` | `HOLD` until strict-vs-compat policy is dispositioned for `SC-INFILE-PMETPARA-001`. |
| `PMET-GAP-002` | Legacy parser uses fixed-width string symbols (`names` 8 chars, `actlnam` 20 chars), but modern pipelines emit potentially longer strings/descriptions. | [DIRECT][E-WF-04], [DIRECT][E-WP-01], [DIRECT][E-WP-03] | `legacy-code`, `wepppy` | `HOLD` until canonical string-length and truncation policy is set. |
| `PMET-GAP-003` | `usersum2024` does not define delimiter/quoting rules for `actlnam` with embedded spaces/commas; modern writers mostly use comma-delimited single-token descriptions. | [DIRECT][E-US-02], [DIRECT][E-WP-01], [DIRECT][E-WF-06] | `usersum2024`, `wepppy`, `legacy-code` | `HOLD` pending explicit grammar decision for quoted strings and allowed character set. |
| `PMET-NOTE-001` | `wepppyo3` does not currently document an owned parser/writer surface for `pmetpara.txt`. | [DIRECT][E-WP3-01] | `wepppyo3` | `NOTE` provenance completeness only; non-blocking. |

`status` remains `draft-HOLD` until gaps above are dispositioned.

## 11. Parser-Contract Handoff Map
- Target parser contract ID: `SC-INFILE-PMETPARA-001`.
- This specification governs:
  - optional sidecar mode semantics,
  - record grammar and tokenization,
  - symbol continuity and alias mapping,
  - cross-file coupling with management crop names,
  - typed error semantics and explicit strict/compat branching.
- Contract authoring linkage: `docs/work-packages/20260520-infile10-author-sc-infile-pmetpara-001/`.

Handoff linkage:
- `parser_contract_id`: `SC-INFILE-PMETPARA-001`
- `canonical_contract_path`: `docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md`
- `handoff_status`: `contract-authored-draft (HOLD gaps carried forward to review/disposition)`
