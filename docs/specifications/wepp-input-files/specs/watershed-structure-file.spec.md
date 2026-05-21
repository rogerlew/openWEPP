# Watershed Structure Input File Specification (`.str`)

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-WATERSHED-STRUCTURE-STR-001`
- `surface_id`: `infile-watershed-structure-str`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-21T06:52:27Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- [DIRECT] Primary format authority:
  `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf`
  (Structure file section around p.67-68, Table 21; structure restrictions around p.68-69, Table 23)
- [DIRECT] `usersum2024` watershed structure definition and ordering rules:
  `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:6876-6893`
- [DIRECT] `usersum2024` Table 21 field list and repeat rule:
  `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:6933-6953`
- [DIRECT] `usersum2024` watershed structure restrictions (Table 23 context):
  `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7093-7137`
- [DIRECT] `usersum2024` run-flow prompt includes watershed structure filename:
  `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9141-9154`
- [DIRECT] Legacy open/read/version-check path for `.str` (unit 17):
  `/workdir/wepp-forest/src/infile.for:373-399`
- [DIRECT] Legacy compatibility floor constant for structure file (`strchk = 94.301`):
  `/workdir/wepp-forest/src/inidat.for:1160-1162`
- [DIRECT] Legacy list-directed read of structure records (10 integer fields):
  `/workdir/wepp-forest/src/wshinp.for:244-248`
- [DIRECT] Legacy structural validity checks (non-isolated element, hillslope max, channel cross-check):
  `/workdir/wepp-forest/src/wshinp.for:265-367`
- [DIRECT] Legacy impoundment cross-checks with `.imp`:
  `/workdir/wepp-forest/src/wshini.for:339-361`
- [DIRECT] Comment/blank-line pre-scan behavior (`eatcom`) before first read:
  `/workdir/wepp-forest/src/eatcom.for:24-36` and `/workdir/wepp-forest/src/infile.for:382-386`
- [DIRECT] Modern `wepppy` `.str` writer examples (`94.301`, `99.1`) and 10-field rows:
  `/workdir/wepppy/wepppy/nodb/core/wepp.py:2369-2374` and
  `/workdir/wepppy/wepppy/nodb/core/wepp.py:2382-2394`
- [DIRECT] Modern watershed run template references `.str` alongside `.chn/.man/.slp/.cli/.sol`:
  `/workdir/wepppy/wepp_runner/templates/watershed.template:26-33`
- [DIRECT] `wepppyo3` scope emphasizes WEPP output interchange, not watershed input-file parsing:
  `/workdir/wepppyo3/README.md:70-73` and `/workdir/wepppyo3/README.md:128-146`

## 2. Surface Scope and Applicability
- [DIRECT] This file defines watershed connectivity for channels/impoundments and their hillslope/channel/impoundment contributors (`usersum2024` structure description).
- [DIRECT] Each structure record corresponds to one downstream element (channel or impoundment) and carries contributor IDs split by left/right/top slots (`usersum2024` Table 21 + `wshinp` 10-field read).
- [INFERENCE] Applicability is watershed simulation input assembly only; hillslope-only runs do not consume `.str`.
- [DIRECT] In run orchestration surfaces, `.str` is coupled with `.chn`, `.man`, `.slp`, `.cli`, `.sol` in watershed run templates.

## 3. Version / `datver` Applicability Matrix

| Case | First numeric token on file | Legacy `wepp-forest` behavior | OpenWEPP draft stance |
| --- | --- | --- | --- |
| A | `ver > 10` | [DIRECT] `infile.for` backspaces and calls `verchk`; `verchk` reads version and enforces `ver >= strchk` (`94.301`). | [INFERENCE] Treat as normative modern path. Require explicit version line and compatibility gate. |
| B | `ver <= 10` | [DIRECT] `verchk` is skipped in `infile.for`; parsing continues. | [INFERENCE] Support as legacy-compat parse mode behind explicit flag until behavior is fully characterized. |

- [DIRECT] `strchk` current legacy constant is `94.301` (`inidat.for`).
- [INFERENCE] `usersum2024` examples (`95.7`) and `wepppy` emitted values (`94.301`, `99.1`) are all in Case A.

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical Grammar (free-format tokens)
```text
line_1  := ver
line_n  := elmt nhleft nhrght nhtop ncleft ncrght nctop nileft nirght nitop
```

- [DIRECT] `line_1` is a real version number (`usersum2024` Table 21 line 1; legacy read path uses `datver` check).
- [DIRECT] `line_n` is read as 10 integers by `wshinp`.
- [DIRECT] `line_n` is repeated for each channel/impoundment in increasing element ID order (`usersum2024` note under Table 21).
- [INFERENCE] File uses list-directed Fortran parsing semantics (whitespace-delimited free format).

### 4.2 Element ID Model
- [INFERENCE] The element ID is implicit by record position plus hillslope count: `element_id = nhill + record_index` (because `wshinp` loops `i = nhill+1..` and reads one record per `i`).
- [DIRECT] Hillslope IDs occupy the lower ID range and structure rows begin after hillslopes (`usersum2024` ordering rules).

### 4.3 Comment and Blank Lines
- [DIRECT] Legacy `eatcom` consumes leading comment/blank lines before the first token read.
- [INFERENCE] After the first non-comment token stream begins, inline comments/trailing commentary are not part of the guaranteed grammar contract for `.str` rows.

## 5. Field Dictionary (Canonical WEPP Symbols + Alias Mapping)

| Field | Legacy symbol | Type | Units | Required | Allowed values | Meaning | OpenWEPP boundary alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Version | `ver` | real | none | yes | recommended `>= 94.301` | File format compatibility/version control number | `format_version` |
| Element type | `elmt` | integer | none | yes | `2` channel, `3` impoundment | Current downstream element class | `element_type_code` |
| Hillslope-left contributor ID | `nhleft` | integer | id | yes | `0` or valid hillslope ID | Hillslope feeding from left | `hillslope_left_id` |
| Hillslope-right contributor ID | `nhrght` | integer | id | yes | `0` or valid hillslope ID | Hillslope feeding from right | `hillslope_right_id` |
| Hillslope-top contributor ID | `nhtop` | integer | id | yes | `0` or valid hillslope ID | Hillslope feeding from top | `hillslope_top_id` |
| Channel-left contributor ID | `ncleft` | integer | id | yes | `0` or valid channel element ID | Channel feeding from left | `channel_left_id` |
| Channel-right contributor ID | `ncrght` | integer | id | yes | `0` or valid channel element ID | Channel feeding from right | `channel_right_id` |
| Channel-top contributor ID | `nctop` | integer | id | yes | `0` or valid channel element ID | Channel feeding from top | `channel_top_id` |
| Impoundment-left contributor ID | `nileft` | integer | id | yes | `0` or valid impoundment element ID | Impoundment feeding from left | `impoundment_left_id` |
| Impoundment-right contributor ID | `nirght` | integer | id | yes | `0` or valid impoundment element ID | Impoundment feeding from right | `impoundment_right_id` |
| Impoundment-top contributor ID | `nitop` | integer | id | yes | `0` or valid impoundment element ID | Impoundment feeding from top | `impoundment_top_id` |

### 5.1 Derived/Indexing Symbols Used by Legacy Logic
- [DIRECT] `nchan` is derived by counting rows with `elmt == 2`.
- [DIRECT] `npond` is derived by counting rows with `elmt == 3`.
- [DIRECT] `idelmt(i)` provides dense channel/impoundment numbering used in later routing/report logic.
- [INFERENCE] OpenWEPP should expose explicit derived fields (`channel_count`, `impoundment_count`, `element_local_index`) rather than hidden mutable globals.

## 6. Conditional Branches and Optional Sections
- [DIRECT] Branch by version handling:
  - Case A: version check via `verchk`.
  - Case B: legacy no-check parse path.
- [DIRECT] Branch by element type:
  - `elmt == 2`: row contributes to channel set (`nchan`).
  - `elmt == 3`: row contributes to impoundment set (`npond`).
- [INFERENCE] There are no optional trailing sections inside `.str`; variability is row count and token values only.

## 7. Cross-File Consistency Constraints and Coupling Dependencies
- [DIRECT] Non-isolated element constraint: every structure row must have at least one non-zero contributor across hillslope/channel/impoundment slots; otherwise stop (`wshinp` check #2).
- [DIRECT] Hillslope coverage constraint: `max(nhleft,nhrght,nhtop)` across rows must equal `nhill`; otherwise stop (`wshinp` check #3).
- [DIRECT] Channel cardinality constraint: `nchan` from `.str` must match channel count in `.chn` and management channel count (`jstruc`); otherwise stop (`wshinp` check #4).
- [DIRECT] Impoundment cardinality constraint: `npond` from `.str` must be `<=` impoundments declared in `.imp`, and both constrained by `mximp` (`wshini` checks).
- [DIRECT] Watershed run surfaces reference `.str` with `.chn/.man/.slp/.cli/.sol` in run templates.
- [INFERENCE] Parser contract must treat `.str` as a topology contract that is invalid unless dependent surfaces (`.chn`, `.man`, pass metadata, optional `.imp`) are mutually consistent.

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)
- [DIRECT] Legacy open path for old-status files (`istat=2`) fails hard when file cannot be opened (`open.for`).
- [INFERENCE] OpenWEPP shall not silently default missing `.str` content; return typed errors.

Expected typed error classes (draft):
- [INFERENCE] `InputFileMissing { path, surface_id }`
- [INFERENCE] `InputVersionIncompatible { observed_ver, required_ver, surface_id }`
- [INFERENCE] `InputRecordArityMismatch { expected_fields: 10, observed_fields, line_no }`
- [INFERENCE] `InputElementTypeInvalid { line_no, elmt }`
- [INFERENCE] `InputTopologyDisconnected { line_no, element_id }`
- [INFERENCE] `InputTopologyCoverageMismatch { nhmax, nhill }`
- [INFERENCE] `InputCrossFileChannelCountMismatch { str_nchan, chn_nchan, man_nchan }`
- [INFERENCE] `InputCrossFileImpoundmentCountMismatch { str_npond, imp_declared }`

## 9. Example Snippets

### 9.1 Minimal Valid (1 hillslope, 1 channel)
```text
94.301
2 0 0 1 0 0 0 0 0 0
```
- [DIRECT] Pattern matches `wepppy` minimal writer and 10-field row grammar.

### 9.2 Representative Valid (channels + impoundment)
```text
99.1
2 1 0 0 0 0 0 0 0 0
3 0 0 0 2 0 0 0 0 0
2 0 0 0 0 0 3 0 0 0
```
- [INFERENCE] Demonstrates mixed element types with increasing element IDs and explicit contributor slots.

### 9.3 Invalid Cases
```text
94.301
2 0 0 0 0 0 0 0 0 0
```
- [DIRECT] Invalid because the row has no hydrologic link; legacy stops (`wshinp` check #2).

```text
94.301
2 1 0 0 0 0 0 0 0 0
```
paired with `.chn` line-2 channel count `2` and management `jstruc=2`
- [DIRECT] Invalid cross-file channel mismatch; legacy stops (`wshinp` check #4).

## 10. Gap / Conflict Register (`HOLD` Conditions)

| Gap ID | Statement | Evidence | Disposition status |
| --- | --- | --- | --- |
| G1 | `usersum2024` Table 23 describes stronger structural rules than the explicit hard-stop checks observed in `wshinp/wshini`. | [DIRECT] `usersum2024:7093-7137` vs `wshinp:265-367`, `wshini:339-361` | `HOLD` until SC defines which rules are enforced at parse-time vs topology-validation stage. |
| G2 | Legacy Case B (`ver <= 10`) behavior is accepted by code path but historical corpus compatibility is not yet characterized in openWEPP. | [DIRECT] `infile.for:383-396` | `HOLD` until fixture-based compatibility decision is dispositioned. |
| G3 | `wepppy` structure emission path writes channel-only rows for non-minimal generated structures (`elmt` hardcoded `2`). | [DIRECT] `wepp.py:2383-2391` | `HOLD` as provenance note; does not redefine canonical `.str` grammar. |
| G4 | `wepppyo3` currently documents output/pass interchange surfaces; no dedicated `.str` parser contract is documented there. | [DIRECT] `wepppyo3/README.md:70-73`, `128-146` | `HOLD` until openWEPP parser contract and ownership boundary are finalized. |

## 11. Parser-Contract Handoff Map (`SC-INFILE-WATERSHED-STRUCTURE-001`)

| Contract area | Source spec requirement | Parser-contract expectation |
| --- | --- | --- |
| Version gate | Section 3 matrix | Must implement Case A and explicit policy for Case B. |
| Grammar | Section 4 | Enforce 1 version line + repeated 10-int records; reject arity drift. |
| Symbol continuity | Section 5 | Canonical field names remain legacy symbols with explicit alias table. |
| Structural validity | Section 7 | Enforce non-isolated element and hillslope coverage checks. |
| Cross-file closure | Section 7 | Validate `.str` counts against `.chn`, `.man`, optional `.imp`. |
| Error semantics | Section 8 | Emit typed errors; no silent defaulting/masking. |
| Correctness gate | Section 10 | Unresolved gaps keep status `HOLD` until disposition evidence exists. |

### Handoff ID
- `parser_contract_id`: `SC-INFILE-WATERSHED-STRUCTURE-001`
- `handoff_status`: `ready-for-contract-authoring (with HOLD gaps carried forward)`
