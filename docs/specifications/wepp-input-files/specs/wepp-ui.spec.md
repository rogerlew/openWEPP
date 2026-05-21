# WEPP UI Sidecar Input Specification (`wepp_ui.txt`)

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-WEPPUI-001`
- `surface_id`: `infile-wepp-ui`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-21T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- [DIRECT][E-US-01] `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf` (WEPP usersum sidecar section for `wepp_ui.txt`; August 2024 edition near p.94).
- [DIRECT][E-US-02] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9391-9395` (`wepp_ui.txt` presence enables alternate hourly water-balance, recommends 7778 soils, and states file is empty).
- [DIRECT][E-WF-01] `/workdir/wepp-forest/src/main.for:187-194` (`ui_run` defaults `0`; existence/open of `wepp_ui.txt` sets `ui_run=1`; no value reads from file body).
- [DIRECT][E-WF-02] `/workdir/wepp-forest/src/watbal.for:268-276` (`ui_run=1` branches to `watbal_hourly`; otherwise daily path continues in `watbal`).
- [DIRECT][E-WF-03] `/workdir/wepp-forest/src/outfil.for:618-619` (water-balance header distinguishes daily vs `HOURLY SEEPAGE UPDATE FROM UI`).
- [DIRECT][E-WF-04] `/workdir/wepp-forest/src/wathour.inc:26-40` (`ui_run` declared as shared runtime control in `common /lfrtg/`).
- [DIRECT][E-WF-05] `/workdir/wepp-forest/src/input.for:532-536` and `:546-549` (soil/initial-saturation behavior changes under `ui_run`; 7778+ soil branch reads `ui_anisrt`).
- [DIRECT][E-WF-06] `/workdir/wepp-forest/tests/test_watbal_replay_acceptance_wb05.py:259-263` (sentinel presence/absence asserts and expected hourly/daily headers).
- [DIRECT][E-WP-01] `/workdir/wepppy/wepppy/nodb/core/wepp.py:1768-1775` (`_prep_wepp_ui` writes empty file; skips creation when any soil version contains `2006`).
- [DIRECT][E-WP-02] `/workdir/wepppy/wepppy/nodb/core/wepp_prep_service.py:97-100` (prep service toggles sentinel creation/removal from run flag).
- [DIRECT][E-WP-03] `/workdir/wepppy/wepppy/microservices/rq_engine/wepp_run_payload.py:25-32` (`checkbox_hourly_seepage` mapped to `_run_wepp_ui`).
- [DIRECT][E-WP-04] `/workdir/wepppy/docs/ui-docs/control-ui-styling/control-inventory.md:141` (UI control note: hourly seepage toggle affects projects with 7778 soils).
- [DIRECT][E-WP3-01] `/workdir/wepppyo3/README.md:66-71` (module registry does not declare `wepp_ui.txt` parser/writer ownership).

## 2. Surface Scope and Applicability
- [DIRECT][E-US-02] `wepp_ui.txt` is a sidecar sentinel for selecting alternate hourly water-balance behavior.
- [DIRECT][E-WF-01] Legacy parser behavior is presence-only: open success sets `ui_run=1`; missing/open failure leaves `ui_run=0`.
- [DIRECT][E-WF-01] File content is not parsed for fields in legacy code.
- [DIRECT][E-US-02] Usersum states this option should be used with 7778 soil format.
- [INFERENCE][E-WF-02] Applicability is hillslope/watershed runs that execute `watbal`; sentinel selection changes runtime kernel path and water-balance output labeling.

## 3. Version / `datver` Applicability Matrix

| Case | File state | Legacy `wepp-forest` behavior | openWEPP draft stance |
| --- | --- | --- | --- |
| A | `wepp_ui.txt` absent | [DIRECT][E-WF-01] `ui_run=0` (daily path). | [INFERENCE][E-WF-01] Treat as valid optional-surface absence with explicit default provenance (`ui_run=false`). |
| B | `wepp_ui.txt` present and empty | [DIRECT][E-US-02], [DIRECT][E-WF-01] `ui_run=1` (hourly path). | [INFERENCE][E-US-02] Canonical valid sentinel form. |
| C | `wepp_ui.txt` present and non-empty | [DIRECT][E-WF-01] Still `ui_run=1`; payload ignored because no reads occur. | [INFERENCE][E-WF-01] Compatibility mode may accept-ignored; strict canonical mode policy unresolved (`HOLD`). |
| D | `wepp_ui.txt` path exists but open fails (permissions/IO) | [DIRECT][E-WF-01] Open error branch sets `ui_run=0`. | [INFERENCE][E-WF-01] Distinguish `NotFound` vs `OpenError`; do not silently collapse operational IO faults into missing-file defaults. |

- [DIRECT][E-WF-01] No `datver` record is read from `wepp_ui.txt`.
- [DIRECT][E-US-02] The source-authority rule is semantic (hourly toggle + empty sentinel) rather than a fielded line schema.

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical grammar (draft)
```ebnf
wepp_ui_file = byte_stream ;

# canonical form for openWEPP strict mode
canonical_wepp_ui_file = "" ;  # zero-byte file
```

- [DIRECT][E-WF-01] Legacy code uses file-open success only; there are no `read` statements for this surface.
- [DIRECT][E-US-02] Usersum describes `wepp_ui.txt` as an empty file.
- [INFERENCE][E-WF-01] Grammar is presence-only with no tokenized records.

### 4.2 Line definitions
- No required lines and no parseable fields in legacy behavior. [DIRECT][E-WF-01]
- Canonical authoring expectation is zero-byte file. [DIRECT][E-US-02]

## 5. Field Dictionary With Canonical Symbols and openWEPP Alias Mapping

`wepp_ui.txt` has no in-file scalar fields; it derives runtime state by file presence.

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Constraints (draft) | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ui_run` | runtime switch selecting hourly (`1`) vs daily (`0`) water-balance path | none | integer flag | 1 runtime value | derived | `0` when sentinel absent/open-fail; `1` when sentinel open succeeds | `run_options.wepp_ui_enabled` |
| `solwpv` | soil-file version coupled with `wepp_ui` recommendation | version id | numeric version token | per soil file | cross-file required | usersum recommends 7778 with `wepp_ui`; strict enforcement unresolved | `soil.version` |

### 5.1 Alias mapping notes
- [DIRECT][E-WF-04] Canonical runtime symbol is legacy `ui_run` from `wathour.inc`.
- [DIRECT][E-WP-03] UI/API boundary commonly exposes this via `checkbox_hourly_seepage` / `_run_wepp_ui`.
- [INFERENCE][E-WP-03] openWEPP boundary names should remain aliases over canonical WEPP symbol semantics.

## 6. Conditional Branches and Optional Sections
1. Presence branch.
- [DIRECT][E-WF-01] Sentinel open success sets `ui_run=1`.
- [DIRECT][E-WF-01] Missing/open-fail branch sets `ui_run=0`.

2. Water-balance kernel branch.
- [DIRECT][E-WF-02] `ui_run=1` calls `watbal_hourly`; otherwise execution continues in daily `watbal`.

3. Output labeling branch.
- [DIRECT][E-WF-03] Water-balance output header changes by `ui_run` state.

4. No optional section parsing branch.
- [DIRECT][E-WF-01] Legacy does not parse any records from `wepp_ui.txt` body.

## 7. Cross-File Consistency Constraints and Coupling Dependencies
1. Soil-version coupling.
- [DIRECT][E-US-02] Usersum states hourly option should use 7778 soil format.
- [DIRECT][E-WF-05] `ui_run` toggles behavior in soil/input processing, including saturation cap and 7778+ layer structure handling.

2. Runtime mode evidence coupling.
- [DIRECT][E-WF-06] Replay acceptance tests use sentinel presence plus output header to verify requested/effective mode behavior.

3. Orchestrator coupling.
- [DIRECT][E-WP-02], [DIRECT][E-WP-03] wepppy surfaces hourly seepage as a run-option toggle that materializes/removes `wepp_ui.txt`.

4. wepppyo3 scope coupling.
- [DIRECT][E-WP3-01] wepppyo3 module registry does not claim parser/writer responsibility for this sidecar; this remains an openWEPP/wepppy interoperability surface.

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)

| Condition | Legacy behavior | openWEPP typed expectation (draft) |
| --- | --- | --- |
| `wepp_ui.txt` missing | [DIRECT][E-WF-01] `ui_run=0` (daily mode) | [INFERENCE][E-WF-01] `OptionalSurfaceMissingDefaulted(surface_id=infile-wepp-ui, ui_run=false)` |
| `wepp_ui.txt` present, open succeeds | [DIRECT][E-WF-01] `ui_run=1` (hourly mode), no content parse | [INFERENCE][E-WF-01] `SentinelPresent(surface_id=infile-wepp-ui, ui_run=true)` |
| open fails for non-not-found IO reason | [DIRECT][E-WF-01] collapsed into err branch (`ui_run=0`) | [INFERENCE][E-WF-01] `InputOpenError(surface_id=infile-wepp-ui, cause=...)` (do not silently downgrade to missing) |
| non-empty file content | [DIRECT][E-WF-01] ignored | strict mode: `SentinelPayloadNotEmptyError(surface_id=infile-wepp-ui)`; compatibility mode: `SentinelPayloadIgnoredWarning(surface_id=infile-wepp-ui)` |

## 9. Example Snippets

### 9.1 Minimal valid canonical example
`wepp_ui.txt` is an empty file (0 bytes).

- [DIRECT][E-US-02] usersum states the file is empty.

### 9.2 Valid compatibility example
```text

```
(single newline only)

- [INFERENCE][E-WF-01] Legacy presence-only branch will still activate hourly mode.

### 9.3 Invalid examples (strict canonical mode draft)
1. Non-empty payload token:
```text
1
```
Reason: canonical strict form is empty sentinel; tokenized content has no defined semantics. [INFERENCE][E-US-02]

2. Sentinel requested but unreadable file permissions:
(Example: file exists but process cannot open)
Reason: runtime intent cannot be satisfied; should produce explicit typed IO error. [INFERENCE][E-WF-01]

3. Sentinel present with non-7778 soil version under strict interoperability policy:
(Example: 2006 soil profiles with hourly toggle forced)
Reason: usersum compatibility rule and modern orchestrator behavior diverge from permissive legacy handling. [INFERENCE][E-US-02], [DIRECT][E-WP-01]

## 10. Gap / Conflict Register and `HOLD` Conditions

| Gap ID | Statement | Evidence | Provenance tags | Disposition status |
| --- | --- | --- | --- | --- |
| `WEPPUI-GAP-001` | `usersum2024` recommends 7778 soils for hourly mode, but legacy does not hard-fail non-7778 at sentinel gate. | [DIRECT][E-US-02], [DIRECT][E-WF-01], [DIRECT][E-WF-05] | `usersum2024`, `legacy-code` | `HOLD` until enforcement severity (`error` vs `warning`) is fixed in `SC-INFILE-WEPPUI-001`. |
| `WEPPUI-GAP-002` | Legacy open-error branch collapses missing and non-missing IO faults into `ui_run=0`, obscuring operational failures. | [DIRECT][E-WF-01] | `legacy-code` | `HOLD` until typed error taxonomy is finalized for openWEPP input consumers. |
| `WEPPUI-NOTE-001` | `usersum2024` says `wepp_ui.txt` should be empty, but legacy runtime ignores file content and checks presence only; this now has explicit strict/compat typed outcomes. | [DIRECT][E-US-02], [DIRECT][E-WF-01] | `usersum2024`, `legacy-code` | `NOTE` policy provenance retained; non-blocking. |
| `WEPPUI-NOTE-002` | wepppyo3 does not currently claim `wepp_ui` parser ownership; cross-repo interoperability boundary is procedural rather than contract-bound. | [DIRECT][E-WP3-01] | `wepppyo3` | `NOTE` governance alignment; non-blocking. |

`status` remains `draft-HOLD` until gaps above are dispositioned.

## 11. Parser-Contract Handoff Map (`SC-INFILE-WEPPUI-001`)

| Contract area | Source spec requirement | Parser-contract expectation |
| --- | --- | --- |
| Presence semantics | Sections 3 and 6 | Represent `wepp_ui` as presence-only optional sidecar that derives `ui_run`. |
| Canonical form | Sections 4 and 9 | Define strict canonical zero-byte sentinel and explicit compatibility posture for non-empty payloads. |
| Soil compatibility coupling | Sections 5 and 7 | Encode 7778 compatibility rule with explicit enforcement level and diagnostics. |
| Error behavior | Section 8 | Distinguish missing sentinel defaults from non-not-found open failures via typed errors. |
| Runtime observability | Sections 6 and 7 | Emit mode-selection provenance usable by scheduler/replay diagnostics. |
| Gap carry-forward | Section 10 | Carry unresolved policy gaps as `HOLD` conditions until formally dispositioned. |

### Handoff ID
- `parser_contract_id`: `SC-INFILE-WEPPUI-001`
- `handoff_status`: `ready-for-contract-authoring (with HOLD gaps carried forward)`
