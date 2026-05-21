# WEPP Frost Sidecar Input Specification (`frost.txt`)

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-FROST-001`
- `surface_id`: `infile-frost`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-21T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- [DIRECT][E-US-01] `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf` (August 2024 usersum, sidecar section for `frost.txt`, page 94).
- [DIRECT][E-US-02] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9433-9453` (`frost.txt` purpose, 2-line format, example, and missing-file defaults).
- [DIRECT][E-US-03] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9454-9456` (Dun et al. citation context for freeze/thaw process updates).
- [DIRECT][E-WF-01] `/workdir/wepp-forest/src/infile.for:1585-1641` (legacy `frost.txt` read path, defaulting, and bounds clamping).
- [DIRECT][E-WF-02] `/workdir/wepp-forest/src/cflgfs.inc:7-16,30-34` (canonical symbols and common-block declarations for `wintRed`, `fineTop`, `fineBot`).
- [DIRECT][E-WF-03] `/workdir/wepp-forest/src/cwint.inc:19,88-95` (canonical winter conductivity symbols `ksnowf`, `kresf`, `ksoilf`, `kfactor(3)`).
- [DIRECT][E-WF-04] `/workdir/wepp-forest/src/getfreezecond.for:41-67` (runtime class-selection logic for `kfactor(1..3)`).
- [DIRECT][E-WF-05] `/workdir/wepp-forest/src/getfreezecond.for:7-9` and `/workdir/wepp-forest/src/cwint.inc:92-94` (legacy comment-level class-mapping conflicts for `kfactor` indices).
- [DIRECT][E-WP-01] `/workdir/wepppy/wepppy/nodb/core/wepp.py:381-457` (`FrostOpts` field set and serialized two-line `frost.txt` shape).
- [DIRECT][E-WP-02] `/workdir/wepppy/wepppy/nodb/core/wepp.py:815-833,1532-1605` (wepppy guard bounds/defaults aligned to legacy ranges/defaults).
- [DIRECT][E-WP-03] `/workdir/wepppy/wepppy/nodb/core/wepp.py:1782-1808` (`_prep_frost`, default-file minting, and remove lifecycle).
- [DIRECT][E-WP-04] `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py:40-47` and `/workdir/wepppy/wepppy/microservices/rq_engine/wepp_run_payload.py:66-74` (prefixed payload ingress for frost option fields).
- [DIRECT][E-WP3-01] `/workdir/wepppyo3/README.md:66-77,128-167` (`wepppyo3` published production modules do not claim `frost.txt` parsing/writing ownership).

## 2. Surface Scope and Applicability
- [DIRECT][E-US-02] `frost.txt` is an optional sidecar used to configure freeze/thaw sublayer controls and conductivity adjustment factors.
- [DIRECT][E-WF-01] Legacy parser expects line 1 (`wintRed`, `fineTop`, `fineBot`) and an optional line 2 (`ksnowf`, `kresf`, `ksoilf`, `kfactor(1..3)`).
- [DIRECT][E-WF-01] Missing file is non-fatal; legacy applies built-in defaults and continues.
- [INFERENCE][E-WF-01] Applicability is runs executing winter freeze/thaw processes (hillslope and watershed contexts), because symbols are stored in shared winter/frost state, not per-OFE sidecar records.
- [DIRECT][E-WP-03] Modern orchestration commonly mints `frost.txt` explicitly, but this does not negate optional-surface semantics from legacy WEPP.

## 3. Version / `datver` Applicability Matrix

| Case | File state | Legacy behavior | openWEPP draft stance |
| --- | --- | --- | --- |
| A | `frost.txt` absent | [DIRECT][E-US-02], [DIRECT][E-WF-01] Use defaults: `1 10 10` and `1.0 1.0 1.0 0.000010 0.000010 0.500000`. | [INFERENCE][E-WF-01] Treat as explicit optional-surface default branch with observable provenance event. |
| B | `frost.txt` present with 2 valid lines | [DIRECT][E-US-02], [DIRECT][E-WF-01] Read all fields then apply bounds clamping. | [INFERENCE][E-WF-01] Canonical parse path. |
| C | `frost.txt` present with only line 1 | [DIRECT][E-WF-01] Legacy falls through `err/end=300`, then defaults/clamps line-2 coefficients. | [INFERENCE][E-WF-01] Support as legacy-compat parse mode; strict mode policy remains unresolved (`HOLD`). |
| D | `frost.txt` present with malformed line 1 tokens | [DIRECT][E-WF-01] No explicit `err=` on first read; behavior is runtime-IO failure dependent. | [INFERENCE][E-WF-01] Must raise typed parse error; do not silently continue. |

- [DIRECT][E-WF-01] No `datver` header/line exists for this sidecar.

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical grammar (draft)
```ebnf
frost_file      = line1 [line2] ;
line1           = wintRed fineTop fineBot ;
line2           = ksnowf kresf ksoilf kfactor1 kfactor2 kfactor3 ;
wintRed         = integer ;
fineTop         = integer ;
fineBot         = integer ;
ksnowf          = real ;
kresf           = real ;
ksoilf          = real ;
kfactor1        = real ;
kfactor2        = real ;
kfactor3        = real ;
```

- [DIRECT][E-US-02] Usersum documents exactly two records with the listed field order.
- [DIRECT][E-WF-01] Legacy parser reads line 1 with list-directed input and attempts line 2 via `read(...,err=300,end=300)`.
- [INFERENCE][E-WF-01] `line2` is optional in compatibility mode due to explicit fallback branch.
- [INFERENCE][E-WF-01] Delimiters are list-directed compatible (whitespace and comma tokenization), but quoting/comment grammar is not formally documented and remains open.

### 4.2 Line definitions
- Line 1a `wintRed`: apply frost-zone water redistribution (`1=yes`, `0=no`). [DIRECT][E-US-02], [DIRECT][E-WF-02]
- Line 1b `fineTop`: number of freeze/thaw fine layers in each of top two 10 cm soil layers. [DIRECT][E-US-02], [DIRECT][E-WF-02]
- Line 1c `fineBot`: number of freeze/thaw fine layers in each remaining 20 cm soil layer. [DIRECT][E-US-02], [DIRECT][E-WF-02]
- Line 2a `ksnowf`: conductivity adjustment for snow. [DIRECT][E-US-02], [DIRECT][E-WF-03]
- Line 2b `kresf`: conductivity adjustment for residue. [DIRECT][E-US-02], [DIRECT][E-WF-03]
- Line 2c `ksoilf`: conductivity adjustment for soil. [DIRECT][E-US-02], [DIRECT][E-WF-03]
- Line 2d-2f `kfactor(1..3)`: lower bounds for frozen-soil conductivity by cover class. [DIRECT][E-US-02], [DIRECT][E-WF-04]

## 5. Field Dictionary With Canonical Symbols and openWEPP Alias Mapping

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Constraints (legacy clamp domain) | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `wintRed` | enable frost water redistribution | unitless flag | integer | 0..1 per file | optional (defaults if file missing) | values outside `{0,1}` are clamped to `1` | `frost_opts.wintRed` |
| `fineTop` | fine layers in each top 10 cm major layer | count | integer | 0..1 per file | optional | valid `1..10`; out-of-range clamped to `10` | `frost_opts.fineTop` |
| `fineBot` | fine layers in each deeper 20 cm major layer | count | integer | 0..1 per file | optional | valid `1..10`; out-of-range clamped to `10` | `frost_opts.fineBot` |
| `ksnowf` | snow conductivity adjustment factor | unitless multiplier | real | 0..1 per file | optional | valid `0.1..10.0`; out-of-range clamped to `1.0` | `frost_opts.ksnowf` |
| `kresf` | residue conductivity adjustment factor | unitless multiplier | real | 0..1 per file | optional | valid `0.1..10.0`; out-of-range clamped to `1.0` | `frost_opts.kresf` |
| `ksoilf` | soil conductivity adjustment factor | unitless multiplier | real | 0..1 per file | optional | valid `0.1..10.0`; out-of-range clamped to `1.0` | `frost_opts.ksoilf` |
| `kfactor(1)` | frozen-soil conductivity lower limit (annual/fallow branch per runtime logic) | unitless factor | real | 0..1 per file | optional | valid `(0,1]`; else `1e-5` | `frost_opts.kfactor1` |
| `kfactor(2)` | frozen-soil conductivity lower limit (pasture/perennial branch per runtime logic) | unitless factor | real | 0..1 per file | optional | valid `(0,1]`; else `1e-5` | `frost_opts.kfactor2` |
| `kfactor(3)` | frozen-soil conductivity lower limit (forest branch per runtime logic) | unitless factor | real | 0..1 per file | optional | valid `(0,1]`; else `0.5` | `frost_opts.kfactor3` |

### 5.1 Alias mapping notes
- [DIRECT][E-WF-02], [DIRECT][E-WF-03] Canonical symbols are the legacy WEPP names (`wintRed`, `fineTop`, `fineBot`, `ksnowf`, `kresf`, `ksoilf`, `kfactor(1..3)`).
- [DIRECT][E-WP-01] `frost_opts_*` names are alias/boundary names for orchestration payloads and emitted files.
- [INFERENCE][E-WF-04] openWEPP interfaces should preserve canonical symbols in specs and equation references while allowing alias mapping at API boundaries.

## 6. Conditional Branches and Optional Sections
1. File-presence branch.
- [DIRECT][E-WF-01] Missing `frost.txt` triggers full defaults without hard failure.

2. Optional second-line branch.
- [DIRECT][E-WF-01] Failure/end on line 2 enters fallback branch and then range-clamp logic.
- [INFERENCE][E-WF-01] Single-line files are legacy-compatible and should be policy-addressed explicitly in parser contracts.

3. Bounds clamping branch.
- [DIRECT][E-WF-01] All fields are normalized to bounded domains after read/default phases.

4. Land-cover class branch for `kfactor` application.
- [DIRECT][E-WF-04] Runtime class selection maps land-use state to `kfactor(1..3)` within `getFreezeCond`.

## 7. Cross-File Consistency Constraints and Coupling Dependencies
1. Winter kernel coupling.
- [DIRECT][E-WF-03], [DIRECT][E-WF-04] `frost.txt` coefficients feed winter thermal/flow behavior through `cwint` state and `getFreezeCond` class selection.

2. Soil-profile discretization coupling.
- [DIRECT][E-US-02], [DIRECT][E-WF-02] `fineTop`/`fineBot` control freeze/thaw discretization granularity and must remain consistent with soil layer interpretation.

3. Process-literature coupling.
- [DIRECT][E-US-03] Usersum ties frost process behavior to Dun et al. (2010); coefficients are process controls, not generic calibration placeholders.

4. Orchestration and payload coupling.
- [DIRECT][E-WP-04], [DIRECT][E-WP-03] wepppy exposes `frost_opts_*` payload ingress and writes/removes `frost.txt` in run-prep lifecycle.

5. Wepppyo3 scope boundary.
- [DIRECT][E-WP3-01] wepppyo3 module registry/API surface does not establish authoritative parsing/writing ownership for `frost.txt`.

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)

| Condition | Legacy behavior | openWEPP typed expectation (draft) |
| --- | --- | --- |
| `frost.txt` missing | [DIRECT][E-US-02], [DIRECT][E-WF-01] defaults applied and run continues | [INFERENCE][E-WF-01] `OptionalSurfaceMissingDefaulted(surface_id=infile-frost, defaults=...)` |
| `frost.txt` has line 1 only | [DIRECT][E-WF-01] line-2 values defaulted/clamped | [INFERENCE][E-WF-01] `OptionalRecordGroupMissingDefaulted(surface_id=infile-frost, record_group=line2)` in compatibility mode |
| line 1 non-numeric or wrong arity | [DIRECT][E-WF-01] no explicit `err=` handler on first read | [INFERENCE][E-WF-01] `TokenParseError` / `InputRecordArityError` |
| line 2 non-numeric/wrong arity | [DIRECT][E-WF-01] fallback branch then clamping | [INFERENCE][E-WF-01] strict-mode error vs compat defaulting is unresolved (`HOLD`) |
| out-of-range numeric values | [DIRECT][E-WF-01] clamp to legacy defaults/ranges | [INFERENCE][E-WF-01] strict rejection vs legacy-clamp mode must be explicit contract choice |
| non-finite numeric values | [DIRECT][E-WP-02] modern guards explicitly sanitize non-finite/unitized inputs | [INFERENCE][E-WP-02] must surface typed finite/range errors before kernel boundary in strict mode |

## 9. Example Snippets

### 9.1 Minimal valid canonical example (usersum)
```text
1 10 8
0.100000 0.200000 0.300000 0.000010 0.000020 0.500000
```
- [DIRECT][E-US-02] Matches documented example.

### 9.2 Valid default-equivalent explicit file
```text
1 10 10
1.0 1.0 1.0 0.000010 0.000010 0.500000
```
- [DIRECT][E-US-02] Matches documented missing-file defaults.

### 9.3 Legacy-compatible single-line file (line-2 omitted)
```text
1 10 10
```
- [DIRECT][E-WF-01] Legacy fallback branch defaults/clamps line-2 values.

### 9.4 Invalid examples
1. Invalid `wintRed` token:
```text
on 10 10
1.0 1.0 1.0 0.000010 0.000010 0.500000
```
Reason: canonical parser expects integer token on line 1 column 1. [INFERENCE][E-WF-01]

2. Invalid line-1 arity:
```text
1 10
1.0 1.0 1.0 0.000010 0.000010 0.500000
```
Reason: missing `fineBot`. [DIRECT][E-US-02]

3. Out-of-range coefficient values:
```text
1 12 0
20.0 -1.0 0.05 0.0 -0.2 2.0
```
Reason: violates documented/legacy clamp domains. [DIRECT][E-WF-01]

## 10. Gap / Conflict Register and `HOLD` Conditions

| Gap ID | Statement | Evidence | Disposition status |
| --- | --- | --- | --- |
| `FROST-GAP-001` | Legacy comments conflict on `kfactor(1..3)` class mapping (`infile.for`/usersum vs `cwint.inc` comments vs `getfreezecond.for` comments). Runtime code path appears authoritative but comments disagree. | [DIRECT][E-US-02], [DIRECT][E-WF-01], [DIRECT][E-WF-04], [DIRECT][E-WF-05] | `HOLD` until canonical class-index mapping is dispositioned and comment conflict handling rule is recorded. |
| `FROST-GAP-002` | Ambiguity on strict vs compatibility behavior for present-but-malformed line 2 (legacy defaults/clamps vs fail-fast parser policy). | [DIRECT][E-WF-01], [DIRECT][E-WP-02] | `HOLD` until `SC-INFILE-FROST-001` defines strict and compat modes explicitly. |
| `FROST-GAP-003` | `datver`/version line is absent for this sidecar; compatibility expectations for any version-prefixed variants are unspecified. | [DIRECT][E-WF-01], [DIRECT][E-US-02] | `HOLD` until parser contract states reject/accept policy for prefixed variants. |
| `FROST-GAP-004` | Delimiter/comment grammar beyond list-directed numeric reads (quoted strings, inline comments) is not specified by usersum for `frost.txt`. | [DIRECT][E-US-02], [DIRECT][E-WF-01] | `HOLD` until canonical tokenization rules are dispositioned. |

`status` remains `draft-HOLD` until high-impact gaps above are dispositioned.

## 11. Parser-Contract Handoff Map (`SC-INFILE-FROST-001`)

| Contract area | Source spec requirement | Parser-contract expectation |
| --- | --- | --- |
| Optional surface semantics | Section 3 Case A, Section 8 | Represent missing-file defaults explicitly with observable provenance. |
| Record grammar | Section 4 | Parse line 1 mandatory; line 2 strict/compat behavior explicitly mode-gated. |
| Symbol continuity and aliases | Section 5 | Keep legacy canonical symbols with boundary alias mapping (`frost_opts_*`). |
| Range policy | Sections 5 and 8 | Define strict rejection vs legacy clamping policy per mode. |
| Class-mapping consistency | Sections 5, 7, 10 | Disposition `kfactor` class-index mapping conflict and lock canonical semantics. |
| Typed error behavior | Section 8 | No silent parse corruption; typed errors for malformed present files in strict mode. |

### Handoff ID
- `parser_contract_id`: `SC-INFILE-FROST-001`
- `handoff_status`: `ready-for-contract-authoring (with HOLD gaps carried forward)`
