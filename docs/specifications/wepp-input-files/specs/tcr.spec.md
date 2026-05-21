# WEPP Channel Critical Shear Sidecar Input Specification (`tcr.txt`)

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-TCR-001`
- `surface_id`: `infile-channel-tcr`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-21T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- [DIRECT][E-US-01] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7309-7314` (watershed channel file documents canonical `chntcr` field and units `N/m2`).
- [DIRECT][E-US-02] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9537-9548` (`usersum2024` documents `tc.txt` and `chan.inp` in sidecar section; no `tcr.txt` format table appears in that section).
- [DIRECT][E-WF-01] `/workdir/wepp-forest/src/wshinp.for:182-194` (`tcr.txt` optional open/read path; reads `taumin`, `taumax`, `kch`, `nch`; sets `tcrflg=1` only when open/read path succeeds).
- [DIRECT][E-WF-02] `/workdir/wepp-forest/src/wshinp.for:403-404` (`chntcr(ichan)` first read from watershed channel file record).
- [DIRECT][E-WF-03] `/workdir/wepp-forest/src/wshinp.for:456-461` (`tcrflg=1` branch overwrites `chntcr(ichan)` using slope-shaped curve with `taumin`, `taumax`, `kch`, `nch`).
- [DIRECT][E-WF-04] `/workdir/wepp-forest/src/wshinp.for:441-445` (legacy in-file comment cites steep-stream critical Shields literature rationale for slope-to-critical-shear mapping).
- [DIRECT][E-WF-05] `/workdir/wepp-forest/src/chnrt.for:207` (runtime erosion/routing path consumes `chntcr` state downstream).
- [DIRECT][E-WF-06] `/workdir/wepp-forest/docs/config-defaults-and-overrides.md:12` (documents missing-sidecar default posture: `tcrflg=0`).
- [DIRECT][E-WP-01] `/workdir/wepppy/wepppy/nodb/core/wepp.py:587-626` (`TCROpts` model and emitted `tcr.txt` record order).
- [DIRECT][E-WP-02] `/workdir/wepppy/wepppy/nodb/core/wepp.py:813-824` (wepppy guard bounds/defaults for `taumin` and `taumax`).
- [DIRECT][E-WP-03] `/workdir/wepppy/wepppy/nodb/core/wepp.py:1815-1821` (producer writes `tcr.txt` when enabled).
- [DIRECT][E-WP-04] `/workdir/wepppy/wepppy/nodb/core/wepp_prep_service.py:773-774` (watershed prep only creates `tcr.txt` when TCR toggle path is active).
- [DIRECT][E-WP-05] `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py:34-35` (`tcr_opts_*` parsed from run inputs).
- [DIRECT][E-WP-06] `/workdir/wepppy/docs/ui-docs/control-ui-styling/control-inventory.md:131` (UI exposes `checkbox_wepp_tcr` and `tcr_opts_*` fields).
- [DIRECT][E-WP3-01] `/workdir/wepppyo3/README.md:66-78` (wepppyo3 module scope does not claim WEPP sidecar input parser ownership).

## 2. Surface Scope and Applicability
- [DIRECT][E-WF-01] `tcr.txt` is an optional watershed sidecar read by `wshinp.for` before channel-element parameterization.
- [DIRECT][E-WF-01] Legacy read order is four scalar records: `taumin`, `taumax`, `kch`, `nch`.
- [DIRECT][E-WF-03] When enabled (`tcrflg=1`), sidecar values replace channel-file `chntcr` via a slope-shaped curve per channel element.
- [DIRECT][E-WF-02] When disabled (`tcrflg=0`), channel-file `chntcr` values remain authoritative.
- [INFERENCE][E-WF-01] Applicability is watershed runs only (surface is consumed inside watershed input routine rather than hillslope input routine).

## 3. Version / `datver` Applicability Matrix

| Case | File state | Legacy `wepp-forest` behavior | openWEPP draft stance |
| --- | --- | --- | --- |
| A | `tcr.txt` absent | [DIRECT][E-WF-01], [DIRECT][E-WF-06] `tcrflg=0`; no slope-to-`chntcr` override from sidecar. | [INFERENCE][E-WF-01] Treat as valid optional-surface absence with explicit provenance event. |
| B | `tcr.txt` present with 4 parseable records | [DIRECT][E-WF-01] reads `taumin`, `taumax`, `kch`, `nch`; sets `tcrflg=1`. [DIRECT][E-WF-03] applies override formula to each channel element. | [INFERENCE][E-WF-01] Canonical parse path. |
| C | `tcr.txt` present but malformed/incomplete | [DIRECT][E-WF-01] reads have no `err=`/`end=` handlers for records; malformed content enters runtime I/O failure path behavior. | [INFERENCE][E-WF-01] Must raise typed parse/arity error; do not silently default on parse failure. |
| D | `tcr.txt` open fails for non-missing I/O reason | [DIRECT][E-WF-01] open `err=401` sets `tcrflg=0` without distinguishing error class. | [INFERENCE][E-WF-01] strict: `InputOpenError(surface_id=infile-channel-tcr, cause=...)`; compat: `OptionalSurfaceMissingDefaulted(surface_id=infile-channel-tcr, tcrflg=false)` plus `CompatibilityWarning(open_error_collapsed_with_missing=true)`. |

- [DIRECT][E-WF-01] No `datver` record is read from `tcr.txt`.
- [DIRECT][E-US-02] `usersum2024` sidecar section does not provide a dedicated `tcr.txt` format table.

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical grammar (draft)
```ebnf
tcr_file      = taumin_line taumax_line kch_line nch_line ;
taumin_line   = real [trailing_tokens] ;
taumax_line   = real [trailing_tokens] ;
kch_line      = real [trailing_tokens] ;
nch_line      = real [trailing_tokens] ;
```

- [DIRECT][E-WF-01] Legacy consumes exactly four sequential list-directed reads.
- [DIRECT][E-WP-01] Modern producer emits value-first lines with trailing labels (e.g., `taumin`, `taumax`), so trailing non-required tokens are present in current producer output.
- [INFERENCE][E-WP-01] Strict and compatibility modes both accept canonical numeric-leading records and preserve trailing annotation token provenance for observability.

### 4.2 Line definitions
- Line 1: `taumin` (minimum critical shear parameter for sidecar slope curve). [DIRECT][E-WF-01], [DIRECT][E-WF-03]
- Line 2: `taumax` (maximum critical shear parameter for sidecar slope curve). [DIRECT][E-WF-01], [DIRECT][E-WF-03]
- Line 3: `kch` (shape parameter in denominator/normalization term of slope curve). [DIRECT][E-WF-01], [DIRECT][E-WF-03]
- Line 4: `nch` (shape exponent for slope curve). [DIRECT][E-WF-01], [DIRECT][E-WF-03]

## 5. Field Dictionary With Canonical Symbols and openWEPP Alias Mapping

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Constraints (draft) | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `taumin` | lower asymptote/offset for sidecar `chntcr` curve | N/m^2 | real | 0..1 per file | optional (default branch when file missing) | finite, non-negative; relational guard `taumin <= taumax` | `tcr_opts.taumin` |
| `taumax` | upper scale term for sidecar `chntcr` curve | N/m^2 | real | 0..1 per file | optional (default branch when file missing) | finite, non-negative; relational guard `taumax >= taumin` | `tcr_opts.taumax` |
| `kch` | slope-scale curve parameter | slope-domain parameter | real | 0..1 per file | optional (default branch when file missing) | finite, strictly positive | `tcr_opts.kch` |
| `nch` | slope-curve exponent | dimensionless | real | 0..1 per file | optional (default branch when file missing) | finite, strictly positive | `tcr_opts.nch` |
| `chntcr` | per-channel critical shear used downstream | N/m^2 | real array | `nchan` runtime values | derived | from `.chn` unless sidecar override active | `channel_state.chntcr` |

### 5.1 Alias mapping notes
- [DIRECT][E-US-01], [DIRECT][E-WF-02] Canonical WEPP symbol continuity is anchored on `chntcr` from watershed channel input.
- [DIRECT][E-WF-03] Sidecar symbols `taumin`, `taumax`, `kch`, `nch` are legacy local names in watershed-input transformation logic.
- [DIRECT][E-WP-01], [DIRECT][E-WP-05] `wepppy` boundary names (`tcr_opts_*`) are aliases over legacy symbols and do not replace canonical naming.

## 6. Conditional Branches and Optional Sections
1. Presence branch.
- [DIRECT][E-WF-01] Missing/unopenable `tcr.txt` sets `tcrflg=0`.
- [DIRECT][E-WF-01] Successful open/read sets `tcrflg=1`.

2. Override branch.
- [DIRECT][E-WF-02] Channel file `chntcr` is always read first.
- [DIRECT][E-WF-03] If `tcrflg=1`, `chntcr` is overwritten by the slope curve for each channel element.

3. Runtime-consumption branch.
- [DIRECT][E-WF-05] Downstream channel routing/erosion routines consume resulting `chntcr` values.

4. No additional optional blocks.
- [DIRECT][E-WF-01] Legacy sidecar parse contract is fixed four-record scalar input with no further sections.

## 7. Cross-File Consistency Constraints and Coupling Dependencies
1. `.chn` coupling.
- [DIRECT][E-US-01], [DIRECT][E-WF-02] `.chn` provides base `chntcr` values and units; sidecar acts as optional override layer.

2. Channel-slope coupling.
- [DIRECT][E-WF-03] Sidecar override depends on `chnslp(ichan,ncsseg(ichan))`; valid slope state is prerequisite.

3. Downstream process coupling.
- [DIRECT][E-WF-05] Resulting `chntcr` propagates into channel hydraulic/erosion calculations.

4. Orchestration coupling.
- [DIRECT][E-WP-03], [DIRECT][E-WP-04], [DIRECT][E-WP-06] modern run orchestration gates sidecar materialization behind TCR toggle and option fields.

5. Cross-repo ownership coupling.
- [DIRECT][E-WP3-01] wepppyo3 does not currently claim sidecar parser ownership; this specification remains openWEPP/wepppy integration authority.

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)

| Condition | Legacy behavior | openWEPP typed expectation (draft) |
| --- | --- | --- |
| `tcr.txt` missing | [DIRECT][E-WF-01], [DIRECT][E-WF-06] `tcrflg=0`; no sidecar override. | [INFERENCE][E-WF-01] `OptionalSurfaceMissingDefaulted(surface_id=infile-channel-tcr, tcrflg=false)` |
| `tcr.txt` present, < 4 parseable records | [DIRECT][E-WF-01] no explicit read error handlers on record reads. | [INFERENCE][E-WF-01] `InputRecordCountError(surface_id=infile-channel-tcr, expected=4)` |
| Non-numeric token in required field | [DIRECT][E-WF-01] no explicit parse guards on reads. | [INFERENCE][E-WF-01] `TokenParseError(surface_id=infile-channel-tcr, line_no=...)` |
| Trailing token after required numeric token | [DIRECT][E-WP-01] modern producer emits trailing labels. | [INFERENCE][E-WP-01] strict and compatibility modes accept canonical numeric-leading records with trailing labels/comments and preserve line-level provenance diagnostics. |
| Open failure from permission/device error | [DIRECT][E-WF-01] collapsed to `tcrflg=0` via open `err=` branch. | [INFERENCE][E-WF-01] strict: `InputOpenError(surface_id=infile-channel-tcr, cause=...)`; compat: `OptionalSurfaceMissingDefaulted(surface_id=infile-channel-tcr, tcrflg=false)` plus `CompatibilityWarning(open_error_collapsed_with_missing=true)` |
| `taumin > taumax` after parse | [DIRECT][E-WF-03] both values feed sidecar override curve branch. | [INFERENCE][E-WF-03] strict: `RelationalInvariantError(lhs=taumin, op="<=", rhs=taumax)`; compat: `CompatibilityWarning(relational_invariant_violation="taumin<=taumax")` and preserve legacy value flow. |
| Parsed values violate curve-domain invariants (`kch<=0`, `nch<=0`, non-finite) | [DIRECT][E-WF-03] values feed power/denominator expression directly. | [INFERENCE][E-WF-03] `FieldRangeError(field=...)` / `FieldFiniteError(field=...)` before kernel mapping |
| Denominator degenerate near-zero (`kch**nch + slope**nch ~= 0`) | [DIRECT][E-WF-03] no legacy guard visible in formula branch. | [INFERENCE][E-WF-03] `InvariantViolation(curve_denominator_positive)` |

## 9. Example Snippets

### 9.1 Minimal valid canonical example
```text
35.0
70.0
0.02
1.0
```
- [DIRECT][E-WF-01] Matches required read arity/order.
- [DIRECT][E-WP-02] Aligns with common modern defaults for `taumin`/`taumax`.

### 9.2 Valid representative example with producer-style labels
```text
35.0	taumin
70.0	taumax
0.02	kch
1.0	nch
```
- [DIRECT][E-WP-01] Mirrors current wepppy producer formatting.

### 9.3 Invalid examples
1. Missing line 4:
```text
35.0
70.0
0.02
```
Reason: required `nch` record missing. [DIRECT][E-WF-01]

2. Non-numeric token:
```text
35.0
seventy
0.02
1.0
```
Reason: `taumax` must parse as real. [DIRECT][E-WF-01]

3. Curve-domain invalid:
```text
35.0
70.0
0.0
0.0
```
Reason: denominator/exponent invariants fail under strict numeric guard policy. [INFERENCE][E-WF-03]

## 10. Gap / Conflict Register and `HOLD` Conditions

| Gap ID | Provenance tags | Statement | Evidence | Disposition status |
| --- | --- | --- | --- | --- |
| `TCR-GAP-001` | `usersum2024` | `usersum2024` sidecar section does not provide a dedicated `tcr.txt` format table. | [DIRECT][E-US-02] | `HOLD` until source-authority disposition resolves normative documentation basis. |
| `TCR-GAP-002` | `legacy-code` | Legacy open error branch collapses missing and non-missing I/O failures into `tcrflg=0`. | [DIRECT][E-WF-01] | `HOLD` until typed open-error taxonomy is finalized. |
| `TCR-GAP-003` | `legacy-code` | Legacy formula branch has no explicit numeric-domain guards (`kch`, `nch`, denominator positivity). | [DIRECT][E-WF-03] | `HOLD` until invariant guard policy is encoded in `SC-INFILE-TCR-001`. |
| `TCR-GAP-004` | `legacy-code`, `wepppy` | Bound policy divergence: legacy sidecar path has no explicit hard bounds; wepppy applies selected bounds/defaults for `taumin`/`taumax`. | [DIRECT][E-WF-01], [DIRECT][E-WP-02] | `HOLD` until contract decides canonical range enforcement for all four fields. |
| `TCR-GAP-005` | `wepppy`, `legacy-code` | Modern producer may emit newline-only `tcr.txt` when `tcr_opts` are unset, but legacy read path expects 4 parseable values. | [DIRECT][E-WP-03], [DIRECT][E-WF-01] | `HOLD` until interoperability behavior is dispositioned (`error` vs producer hardening). |

`status` remains `draft-HOLD` until high-impact gaps above are dispositioned.

## 11. Parser-Contract Handoff Map (`SC-INFILE-TCR-001`)

| Contract area | Source spec requirement | Parser-contract expectation |
| --- | --- | --- |
| Presence/default branch | Section 3 Case A, Section 8 | Represent `tcr.txt` as optional sidecar with explicit default provenance when absent. |
| Grammar and arity | Section 4 | Parse exactly four real-valued records in fixed order. |
| Symbol continuity | Section 5 | Preserve canonical symbols (`taumin`, `taumax`, `kch`, `nch`, `chntcr`) with alias mapping. |
| Override semantics | Sections 6-7 | Apply sidecar override only when enabled; otherwise preserve `.chn` `chntcr`. |
| Numeric guards | Sections 8-10 | Enforce finite/range/domain invariants before mapping values into `chntcr` curve. |
| Error semantics | Section 8 | Typed errors for malformed/open-failure conditions; do not silently mask non-missing I/O faults. |
| Policy gaps | Section 10 | Carry unresolved gaps as `HOLD` until dispositioned. |

### Handoff ID
- `parser_contract_id`: `SC-INFILE-TCR-001`
- `canonical_contract_path`: `docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md`
- `handoff_status`: `ready-for-contract-authoring (with HOLD gaps carried forward)`
