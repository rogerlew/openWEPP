# WEPP TC Sentinel Input Specification (`tc.txt`)

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-TC-001`
- `surface_id`: `infile-channel-tc`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-21T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- [DIRECT][E-US-01] `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf` (usersum 2024 section for `tc.txt`; page containing the `tc_out.txt` description).
- [DIRECT][E-US-02] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9518-9538` (`tc.txt` is presence-only; `tc_out.txt` is emitted for watershed simulation when file exists).
- [DIRECT][E-WF-01] `/workdir/wepp-forest/docs/work-packages/20260506-wb13-legacy-for-retirement-deletion/artifacts/legacy_reference_snapshot/src/wshdrv.for:352-356` (legacy sentinel latch: `open tc.txt` then `luntc=1` on open success).
- [DIRECT][E-WF-02] `/workdir/wepp-forest/docs/work-packages/20260506-wb13-legacy-for-retirement-deletion/artifacts/legacy_reference_snapshot/src/wshdrv.for:357-362` (`tc_out.txt` opened and header written only when `luntc != 0`).
- [DIRECT][E-WF-03] `/workdir/wepp-forest/docs/work-packages/20260506-wb13-legacy-for-retirement-deletion/artifacts/legacy_reference_snapshot/src/wshdrv.for:1503-1507` (canonical `tc_out.txt` header format lines).
- [DIRECT][E-WP-01] `/workdir/wepppy/wepppy/nodb/core/wepp.py:1810-1813` (`_prep_tc` writes an empty `tc.txt` sentinel).
- [DIRECT][E-WP-02] `/workdir/wepppy/wepppy/nodb/mods/omni/omni.py:239-246` (omni output trigger writes/removes `tc.txt` by feature flag).
- [DIRECT][E-WP-03] `/workdir/wepppy/tests/nodb/mods/test_omni.py:302-312` (tests assert `tc.txt` presence/absence behavior under trigger toggles).
- [DIRECT][E-WP-04] `/workdir/wepppy/wepppy/rq/wepp_rq_stage_post.py:112-121` (`tc_out.txt` is moved to output and passed to interchange stage when present).
- [DIRECT][E-WP-05] `/workdir/wepppy/wepppy/wepp/interchange/watershed_tc_out_interchange.py:17-18` (canonical downstream artifact names are `tc_out.txt` and `tc_out.parquet`).
- [DIRECT][E-WP-06] `/workdir/wepppy/wepppy/weppcloud/routes/usersum/weppcloud/wepp-usersum-2024.md:895-904` (vendored usersum rendering repeats presence-only `tc.txt` semantics).
- [DIRECT][E-WP3-01] `/workdir/wepppyo3/README.md:49-50` and `:68-70` (`wepppyo3` is scoped to selected output transforms/interchange; no `tc.txt` input-sentinel parser contract is declared).

## 2. Surface Scope and Applicability
- [DIRECT][E-US-02] `tc.txt` is a presence-only sentinel input surface; no parameter payload is required.
- [DIRECT][E-US-02] The behavior is specific to watershed simulation mode and controls creation of a supplemental watershed output (`tc_out.txt`).
- [DIRECT][E-WF-01], [DIRECT][E-WF-02] Legacy execution path checks file open success and toggles `luntc` accordingly; output creation is gated by that latch.
- [INFERENCE][E-WF-01], [DIRECT][E-WF-02] Interim source-authority rule: retirement-snapshot evidence anchors legacy-compat provenance for sentinel semantics, while openWEPP contracts remain authoritative for new strict-mode typed behavior until active-source parity trace is ratified.
- [INFERENCE][E-WF-01] Applicability is watershed CLI/run modes only; hillslope-only mode does not own this surface contract.

## 3. Version / `datver` Applicability Matrix

| Case | `tc.txt` state | Legacy behavior | openWEPP draft stance |
| --- | --- | --- | --- |
| A | File absent | [DIRECT][E-WF-01] open fails to `err=401`, `luntc` remains `0`, no `tc_out.txt` open. | [INFERENCE][E-WF-01] `OptionalSurfaceMissing(surface_id=infile-channel-tc)`; no fatal error. |
| B | File present and open succeeds | [DIRECT][E-WF-01], [DIRECT][E-WF-02] `luntc=1`; `tc_out.txt` opened and headers emitted. | [INFERENCE][E-WF-02] `SentinelPresent(surface_id=infile-channel-tc, enable_tc_out=true)` event. |
| C | File present but open fails (permissions/IO) | [DIRECT][E-WF-01] handled by `err=401`, same runtime path as missing file. | [INFERENCE][E-WF-01] strict: `InputOpenError(surface_id=infile-channel-tc, cause=...)`; compat: `OptionalSurfaceMissing(surface_id=infile-channel-tc)` plus `CompatibilityWarning(open_error_collapsed_with_missing=true)`. |
| D | File present with arbitrary contents | [DIRECT][E-US-02], [DIRECT][E-WF-01] content is not parsed; existence/open controls behavior. | [INFERENCE][E-US-02] preserve presence-only semantics; parser must not infer values from file body. |

- [DIRECT][E-US-02] No `datver` line exists for `tc.txt`.

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical grammar (draft)
```ebnf
tc_file = { byte } ;
```

- [DIRECT][E-US-02] `tc.txt` does not need to contain data.
- [DIRECT][E-WF-01] Legacy code performs open/close only and never issues a `read` on `tc.txt`.
- [INFERENCE][E-WF-01] Any byte content (including empty file) is semantically equivalent under legacy-compatible mode.

### 4.2 Line definitions
- No line records are consumed from `tc.txt`. [DIRECT][E-US-02], [DIRECT][E-WF-01]
- The specification is sentinel-only: parse contract is file presence/openability, not token extraction. [INFERENCE][E-WF-01]

## 5. Field Dictionary With Canonical Symbols and openWEPP Alias Mapping

`tc.txt` defines no parsed scalar fields. The canonical dictionary therefore documents runtime control symbols derived from sentinel detection.

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Constraints | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `luntc` | legacy runtime latch set to `1` when `tc.txt` opens successfully | none | integer flag | 1 per watershed run | derived (from sentinel open) | domain `{0,1}` in observed path | `observability.channel_tc.enabled` |
| `tc_out.txt` | supplemental watershed output stream/file emitted when `luntc != 0` | file artifact | path/output artifact | 0..1 per watershed run | conditional on sentinel | header format per legacy `3000/3001/3002` | `outputs.tc_out.path` |

### 5.1 Alias mapping notes
- [DIRECT][E-WF-01] `luntc` is the legacy WEPP symbol controlling whether TC output is emitted.
- [DIRECT][E-WF-03] Canonical output header shape is defined by legacy format records rather than input-file fields.
- [INFERENCE][E-WP-01] openWEPP API/boundary naming can differ, but canonical references must preserve `luntc`/`tc_out.txt` provenance.

## 6. Conditional Branches and Optional Sections
1. Sentinel presence branch.
- [DIRECT][E-WF-01] open success sets `luntc=1`; open failure retains `luntc=0`.

2. Output activation branch.
- [DIRECT][E-WF-02] `tc_out.txt` is created only when `luntc != 0`.

3. Content-insensitive branch.
- [DIRECT][E-US-02], [DIRECT][E-WF-01] no records are read from `tc.txt`; content does not affect activation.

4. Optionality.
- [DIRECT][E-US-02] `tc.txt` is optional and presence-triggered.

## 7. Cross-File Consistency Constraints and Coupling Dependencies
1. Watershed-only coupling.
- [DIRECT][E-US-02] `tc.txt` behavior is defined in watershed simulation context.

2. Output contract coupling.
- [DIRECT][E-WF-03] When activated, `tc_out.txt` headers follow canonical legacy column labels.

3. Orchestration coupling in WEPPpy.
- [DIRECT][E-WP-04], [DIRECT][E-WP-05] post-run flow expects optional `tc_out.txt` and can emit `tc_out.parquet` interchange artifacts.

4. Trigger UX coupling.
- [DIRECT][E-WP-02], [DIRECT][E-WP-03] modern tooling frequently toggles sentinel creation/removal via option flags, so contract must preserve deterministic on/off semantics.

5. Substrate boundary coupling.
- [DIRECT][E-WP3-01] `wepppyo3` scope emphasizes output transforms/interchange; no explicit `tc.txt` sentinel-parser ownership is declared.

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)

| Condition | Legacy behavior | openWEPP typed expectation (draft) |
| --- | --- | --- |
| `tc.txt` missing | [DIRECT][E-WF-01] no `tc_out` activation; run continues | [INFERENCE][E-WF-01] `OptionalSurfaceMissing(surface_id=infile-channel-tc)` |
| `tc.txt` present, open succeeds | [DIRECT][E-WF-01], [DIRECT][E-WF-02] activate `tc_out` creation | [INFERENCE][E-WF-02] `SentinelPresent(surface_id=infile-channel-tc, enable_tc_out=true)` |
| `tc.txt` present, open fails for non-ENOENT reason | [DIRECT][E-WF-01] merged into `err=401` branch | [INFERENCE][E-WF-01] strict: `InputOpenError(surface_id=infile-channel-tc, cause=...)`; compat: `OptionalSurfaceMissing(surface_id=infile-channel-tc)` plus `CompatibilityWarning(open_error_collapsed_with_missing=true)` |
| `tc.txt` contains malformed/non-text bytes | [DIRECT][E-US-02], [DIRECT][E-WF-01] ignored (not parsed) | [INFERENCE][E-US-02] do not raise parse errors based on body content |
| `tc_out.txt` absent despite sentinel present | [DIRECT][E-WP-04] downstream tooling treats file as optional artifact | [INFERENCE][E-WP-04] emit `OutputExpectationWarning(output=tc_out.txt)` for diagnostics |

## 9. Example Snippets

### 9.1 Minimal valid canonical example
```text

```
- [DIRECT][E-US-02] Empty file is valid because only existence is checked.

### 9.2 Valid representative example with non-empty contents
```text
this text is ignored by legacy tc sentinel logic
123 456 789
```
- [DIRECT][E-WF-01] Legacy does not read body tokens from `tc.txt`.

### 9.3 Invalid examples (strict-mode policy candidates)
1. Directory path named `tc.txt` (not a regular file).
Reason: sentinel open is expected to target a regular file path; behavior otherwise OS-dependent. [INFERENCE][E-WF-01]

2. Permission-denied `tc.txt` under strict mode.
Reason: legacy merges with missing branch, but strict policy may require explicit IO error surfacing. [INFERENCE][E-WF-01]

3. `tc.txt` placed outside active run working directory.
Reason: legacy open uses relative path in run cwd; out-of-dir file will not activate sentinel. [DIRECT][E-WF-01], [INFERENCE][E-WF-01]

## 10. Gap / Conflict Register and `HOLD` Conditions

| Gap ID | Provenance tags | Statement | Evidence | Disposition status |
| --- | --- | --- | --- | --- |
| `TC-GAP-001` | `legacy-code`, `usersum2024` | Active legacy production source tree no longer exposes `tc.txt` handling in current `src/`; authoritative implementation is presently captured in retirement snapshot provenance. | [DIRECT][E-WF-01], [DIRECT][E-WF-02] | `HOLD` until source-authority order is ratified for `tc` (snapshot vs archival branch vs current binary behavior). |
| `TC-GAP-002` | `legacy-code` | Legacy behavior collapses missing-file and open-failure branches; strict-mode diagnostics likely need split typed outcomes. | [DIRECT][E-WF-01] | `HOLD` until `SC-INFILE-TC-001` defines strict vs compatibility error policy. |
| `TC-GAP-003` | `usersum2024`, `legacy-code` | Usersum states watershed output shape textually, but no formal machine grammar for `tc_out.txt` rows is provided in input spec context. | [DIRECT][E-US-02], [DIRECT][E-WF-03] | `HOLD` until `SC-INFILE-TC-001` and downstream output contract align on row grammar authority. |
| `TC-NOTE-001` | `wepppy` | WEPPpy omni option key `tcr_out` currently drives `tc.txt` sentinel creation, which is naming-inconsistent and may obscure intent. | [DIRECT][E-WP-02], [DIRECT][E-WP-03] | `NOTE` non-blocking naming-governance item; carry to UX alias backlog without blocking parser promotion. |

`status` remains `draft-HOLD` until high-impact gaps above are dispositioned.

## 11. Parser-Contract Handoff Map (`SC-INFILE-TC-001`)

| Contract area | Source spec requirement | Parser-contract expectation |
| --- | --- | --- |
| Sentinel semantics | Sections 2, 4, 6 | Presence/openability only; no token parsing from file body. |
| Optional-surface behavior | Sections 3, 8 | Missing sentinel is non-fatal and explicit in diagnostics. |
| Strict vs compat IO handling | Sections 3, 8, 10 | Separate missing vs open-error in strict mode while preserving compat branch behavior. |
| Symbol continuity | Section 5 | Preserve legacy canonical symbol `luntc` and output artifact naming provenance. |
| Output coupling | Sections 7, 8 | Define expected activation relationship between sentinel and `tc_out` downstream pipeline. |
| Governance gaps | Section 10 | Carry unresolved authority/policy conflicts as blocking `HOLD` items until dispositioned. |

### Handoff ID
- `parser_contract_id`: `SC-INFILE-TC-001`
- `canonical_contract_path`: `docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md`
- `handoff_status`: `ready-for-contract-authoring (with HOLD gaps carried forward)`
