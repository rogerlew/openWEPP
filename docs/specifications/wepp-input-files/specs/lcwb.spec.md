# WEPP Channel Last-OFE Water-Balance Sidecar Specification (`lcwb.txt`)

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-LCWB-001`
- `surface_id`: `infile-channel-lcwb`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-21T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- [DIRECT][E-US-01] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9413-9453` (sidecar section documents `pmetpara.txt` and `frost.txt`; no `lcwb.txt` section appears there).
- [DIRECT][E-US-02] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9518-9557` (sidecar section documents `tc.txt`, `wepp_ch.txt`, and `chan.inp`; no `lcwb.txt` section appears there).
- [DIRECT][E-WF-01] `/workdir/wepp-forest/src/wshinp.for:197-204` (`lcwb.txt` is opened as presence-only sentinel; `lcwbflg` set to `1` on open success and `0` on `err=` branch; no records are read from file contents).
- [DIRECT][E-WF-02] `/workdir/wepp-forest/src/cchrt1.inc:26` and `:33` (`lcwbflg` is persisted in channel-routing common state as legacy canonical symbol).
- [DIRECT][E-WF-03] `/workdir/wepp-forest/src/wshinp.for:502-507` (`chanwb.out` file open/header write is gated by `ichout > 0` and `nchnum > 0`, not by `lcwbflg`).
- [DIRECT][E-WF-04] `/workdir/wepp-forest/docs/config-defaults-and-overrides.md:13` (`lcwb.txt` missing-file default documented as `lcwbflg=0`).
- [DIRECT][E-WF-05] `/workdir/wepp-forest/docs/glossary.md:17` (`lcwb` documented as channel water-balance output toggle).
- [DIRECT][E-WF-06] `/workdir/wepp-forest/docs/work-packages/20260506-wb13-legacy-for-retirement-deletion/artifacts/legacy_reference_snapshot/src/watbalprint.for:98-130` (historical write path uses `lcwbflg`: when `1`, only last OFE (`iplane.eq.nplane`) is written to unit `35`; when `0`, all OFEs are written).
- [DIRECT][E-WP-01] `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py:31-41` (modern request parser includes `baseflow`, `phosphorus`, `tcr`, `snow`, `frost`; no `lcwb` input parser surface is present).
- [DIRECT][E-WP-02] `/workdir/wepppy/wepp_runner/templates/watershed.template:12-13` (watershed prompt contract routes water-balance output to `../output/chnwb.txt`).
- [DIRECT][E-WP-03] `/workdir/wepppy/wepp_runner/wepp_runner.py:1452-1475` (`chnwb` output is an explicit run option; when disabled, `../output/chnwb.txt` line is removed from generated run script).
- [DIRECT][E-WP-04] `/workdir/wepppy/wepppy/wepp/interchange/README.md:142-143` (modern interchange consumes `chanwb.out` and `chnwb.txt` as distinct output sources).
- [DIRECT][E-WP3-01] `/workdir/wepppyo3/wepp_interchange/src/lib.rs:25-26` and `:91-133` (native interchange exports cover `chanwb` and `chnwb`; no `lcwb` input parser entrypoint is present).
- [DIRECT][E-WP3-02] `/workdir/wepppyo3/wepp_interchange/src/chanwb.rs:16-33` and `/workdir/wepppyo3/wepp_interchange/src/chnwb.rs:16-33` (native Rust interchange reads channel water-balance outputs, not `lcwb.txt` sidecar input).

## 2. Surface Scope and Applicability
- [DIRECT][E-WF-01] `lcwb.txt` is an optional watershed-sidecar sentinel file evaluated during watershed input setup.
- [DIRECT][E-WF-01] The legacy contract is presence-based: open success sets `lcwbflg=1`; open failure sets `lcwbflg=0`; file contents are not parsed.
- [DIRECT][E-WF-02] Canonical symbol continuity is `lcwbflg` (legacy common-block variable).
- [INFERENCE][E-WF-06] Historical snapshot provenance indicates channel OFE row-selection behavior (last OFE only vs all OFEs), but this remains compatibility provenance rather than active-source authority until `LCWB-GAP-002` is closed.
- [DIRECT][E-WP-04] Modern consumers treat resulting output files (`chnwb.txt`, `chanwb.out`) as downstream interchange sources.
- [INFERENCE][E-WF-01], [DIRECT][E-WF-06] Interim authority rule: openWEPP parser contract is normative for strict/compat typed handling; historical `watbalprint.for` behavior is cited as legacy-compat evidence only.

## 3. Version / `datver` Applicability Matrix

| Case | File state | Legacy behavior | openWEPP draft interpretation |
| --- | --- | --- | --- |
| A | `lcwb.txt` absent | [DIRECT][E-WF-01] `lcwbflg=0` via `err=` branch. | [INFERENCE][E-WF-01] optional-surface default branch; feature disabled. |
| B | `lcwb.txt` present and readable | [DIRECT][E-WF-01] `lcwbflg=1`; no payload read. | [INFERENCE][E-WF-01] canonical enable branch. |
| C | `lcwb.txt` present but unreadable/open-fails | [DIRECT][E-WF-01] falls through same `err=` branch (`lcwbflg=0`). | [INFERENCE][E-WF-01] strict: `InputOpenError(surface_id=infile-channel-lcwb, cause=...)`; compat: `OptionalSurfaceMissingDefaulted(surface_id=infile-channel-lcwb, enabled=false)` plus `CompatibilityWarning(open_error_collapsed_with_missing=true)`. |
| D | Versioned payload (`datver`/header line) present | [DIRECT][E-WF-01] no reads are attempted from file body. | [INFERENCE][E-WF-01] strict: `SentinelPayloadNotEmptyError(surface_id=infile-channel-lcwb)`; compat: treated as opaque payload and ignored. |

- [DIRECT][E-WF-01] No `datver` record is defined or parsed for this sidecar.
- [DIRECT][E-US-01], [DIRECT][E-US-02] No `usersum2024` format table exists for `lcwb.txt`.

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical grammar (draft)

```ebnf
lcwb_file = opaque_text_stream ;
opaque_text_stream = { any_character } ;
```

- [DIRECT][E-WF-01] Legacy code performs open/close only and does not issue `read` statements for `lcwb.txt`.
- [INFERENCE][E-WF-01] Content bytes are non-authoritative for legacy behavior; existence/readability is the effective contract.
- [INFERENCE][E-WF-01] Strict mode constrains sentinel body to empty/whitespace-only payload (`SentinelPayloadNotEmptyError` on non-empty body); compat mode preserves legacy content-insensitive behavior.

### 4.2 Line definitions
- [DIRECT][E-WF-01] No line-level payload fields are defined or consumed.
- [INFERENCE][E-WF-01] Canonical openWEPP parser surface should model this file as a sentinel input with zero required records.

## 5. Field Dictionary With Canonical Symbols and Alias Mapping

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Constraints (draft) | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `lcwbflg` | Channel last-OFE water-balance toggle in legacy runtime state | flag | integer | runtime scalar | derived | `0` or `1` | `channel.last_ofe_watbal_enabled` |
| `lcwb_present` | Sentinel open-success condition inferred from `lcwb.txt` presence/readability | flag | boolean | 0..1 per run | derived | `false` if open fails | `inputs.sidecars.lcwb_present` |

### 5.1 Alias mapping notes
- [DIRECT][E-WF-02] Canonical symbol for this surface is `lcwbflg`; openWEPP names are boundary aliases only.
- [DIRECT][E-WF-06] Historical writer behavior keys off `lcwbflg` directly; alias mappings must preserve binary toggle semantics.

## 6. Conditional Branches and Optional Sections
1. Sentinel presence branch.
- [DIRECT][E-WF-01] `open(...,err=403)` success sets `lcwbflg=1`; failure sets `lcwbflg=0`.

2. Historical last-OFE write branch (compatibility provenance).
- [DIRECT][E-WF-06] Retirement-snapshot `watbalPrint` shows `lunw=1` and `lcwbflg=1` writing only when `iplane.eq.nplane`; treat as historical behavior pending active-source trace closure.

3. Historical all-OFE write branch (compatibility provenance).
- [DIRECT][E-WF-06] Retirement-snapshot `watbalPrint` shows `lunw=1` and `lcwbflg=0` writing all OFE rows; treat as historical behavior pending active-source trace closure.

4. Channel-routing output branch independent of `lcwb`.
- [DIRECT][E-WF-03] `chanwb.out` output file creation is gated by `ichout`/`nchnum`, not by `lcwbflg`.

5. Modern run-script output gating branch.
- [DIRECT][E-WP-02], [DIRECT][E-WP-03] `chnwb.txt` path emission is controlled by run-template `water_balance_output` option.

## 7. Cross-File Consistency Constraints and Coupling Dependencies
1. Watershed-run prompt contract coupling.
- [DIRECT][E-WP-02] If water-balance output is disabled in run script, `chnwb.txt` is not requested, regardless of `lcwb.txt`.

2. Output-option coupling in orchestrator.
- [DIRECT][E-WP-03] `chnwb` output option governs whether `../output/chnwb.txt` is retained in generated run files.

3. Legacy runtime state coupling.
- [DIRECT][E-WF-01], [DIRECT][E-WF-02] `lcwb.txt` only affects derived runtime toggle (`lcwbflg`), not channel-routing parameter ingestion.

4. Historical water-balance row selection coupling.
- [DIRECT][E-WF-06] Historical writer uses `lcwbflg` to switch between last-OFE-only vs all-OFE water-balance row writes.

5. Downstream consumer coupling.
- [DIRECT][E-WP-04], [DIRECT][E-WP3-01], [DIRECT][E-WP3-02] downstream parsers consume produced `chnwb.txt`/`chanwb.out`; they do not parse `lcwb.txt` directly.

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)

| Condition | Legacy behavior | openWEPP typed expectation (draft) |
| --- | --- | --- |
| `lcwb.txt` missing | [DIRECT][E-WF-01] `lcwbflg=0`; continue. | [INFERENCE][E-WF-01] `OptionalSurfaceMissingDefaulted(surface_id=infile-channel-lcwb, enabled=false)`. |
| `lcwb.txt` present with non-empty payload text | [DIRECT][E-WF-01] payload is ignored; `lcwbflg=1` if open succeeds. | [INFERENCE][E-WF-01] strict: `SentinelPayloadNotEmptyError(surface_id=infile-channel-lcwb)`; compat: `SentinelPayloadIgnoredWarning(surface_id=infile-channel-lcwb)` and `SentinelPresent(...)`. |
| `lcwb.txt` open failure (`err=` path) | [DIRECT][E-WF-01] treated same as missing (`lcwbflg=0`). | [INFERENCE][E-WF-01] strict: `InputOpenError(surface_id=infile-channel-lcwb, cause=...)`; compat: `OptionalSurfaceMissingDefaulted(surface_id=infile-channel-lcwb, enabled=false)` plus `CompatibilityWarning(open_error_collapsed_with_missing=true)`. |
| `chnwb` output prompt disabled | [DIRECT][E-WP-03] `../output/chnwb.txt` line removed from run script. | [INFERENCE][E-WP-03] treat as orthogonal output contract condition, not `lcwb` parse failure. |

## 9. Example Snippets

### 9.1 Minimal valid canonical example (empty sentinel file)
```text

```
- [DIRECT][E-WF-01] No reads are performed, so zero-byte file is valid sentinel.

### 9.2 Compat-mode representative example with ignored payload
```text
lcwb sentinel present; payload ignored by legacy reader
```
- [DIRECT][E-WF-01] Legacy behavior depends only on open success, not payload parsing.
- [INFERENCE][E-WF-01] Under strict mode this example is invalid (`SentinelPayloadNotEmptyError`).

### 9.3 Invalid examples
1. Path exists but cannot be opened (permission denied / non-regular inode in strict mode).
Reason: open failure branch ambiguity requires policy; strict mode should surface typed open error. [INFERENCE][E-WF-01]

2. `chnwb` output disabled while expecting channel OFE water-balance file.
Reason: run-template contract removed `../output/chnwb.txt`; missing output is expected by configuration. [DIRECT][E-WP-03]

3. Non-empty `lcwb.txt` body under strict mode.
Reason: strict sentinel policy requires empty/whitespace-only payload. [INFERENCE][E-WF-01]

## 10. Gap / Conflict Register and `HOLD` Conditions

| Gap ID | Provenance tags | Statement | Evidence | Disposition status |
| --- | --- | --- | --- | --- |
| `LCWB-GAP-001` | `usersum2024`, `legacy-code` | `usersum2024` does not publish a dedicated `lcwb.txt` format section. | [DIRECT][E-US-01], [DIRECT][E-US-02] | `HOLD` until source-authority disposition accepts legacy-code provenance for this surface. |
| `LCWB-GAP-002` | `legacy-code` | Active `wepp-forest/src` snapshot declares/sets `lcwbflg` but does not expose a current in-tree `lcwbflg` consumer site; historical snapshot does. | [DIRECT][E-WF-01], [DIRECT][E-WF-02], [DIRECT][E-WF-06] | `HOLD` until current-release consumer path is fully traced and documented. |
| `LCWB-GAP-003` | `legacy-code`, `wepppy` | `lcwb.txt` semantics overlap with `chnwb.txt` output selection and may be confused with `chanwb.out` routing output controls. | [DIRECT][E-WF-03], [DIRECT][E-WP-02], [DIRECT][E-WP-04] | `HOLD` until parser/output contracts codify separation of concerns. |
| `LCWB-GAP-004` | `wepppy`, `wepppyo3` | Modern input parsing surfaces (`wepppy`, `wepppyo3`) do not expose direct `lcwb` input parsing; behavior is mediated by run-template/output contracts. | [DIRECT][E-WP-01], [DIRECT][E-WP-03], [DIRECT][E-WP3-01] | `HOLD` until openWEPP decides whether to model `lcwb` as explicit input-surface entity or derived run-option compatibility flag. |
| `LCWB-NOTE-001` | `legacy-code` | Strict/compat open-failure and payload policy is now codified in Section 8 and needs downstream implementation verification. | [DIRECT][E-WF-01] | `NOTE` non-blocking specification-closure item; carry to parser verification backlog. |

`status` remains `draft-HOLD` until high-impact gaps above are dispositioned.

## 11. Parser-Contract Handoff Map (`SC-INFILE-LCWB-001`)

| Contract area | Source spec requirement | Parser-contract expectation |
| --- | --- | --- |
| Sentinel semantics | Sections 2, 4, 8 | Model `lcwb` as existence/readability sentinel; no payload token grammar. |
| Symbol continuity | Section 5 | Preserve canonical `lcwbflg` symbol with explicit alias mapping only at boundaries. |
| Branching behavior | Section 6 | Preserve enable/disable toggle semantics and separate from channel-routing `chanwb.out` gating. |
| Cross-file coupling | Section 7 | Bind to run-template `chnwb` output option and downstream interchange expectations. |
| Typed errors/defaulting | Section 8 | Define strict-vs-compat policy for open failures; missing file defaults must be explicit/observable. |
| Outstanding conflicts | Section 10 | Carry unresolved provenance and behavior ambiguities as `HOLD` obligations. |

### Handoff ID
- `parser_contract_id`: `SC-INFILE-LCWB-001`
- `canonical_contract_path`: `docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md`
- `handoff_status`: `ready-for-contract-authoring (with HOLD gaps carried forward)`
