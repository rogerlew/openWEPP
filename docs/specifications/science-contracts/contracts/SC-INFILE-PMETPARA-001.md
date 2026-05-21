---
contract_id: SC-INFILE-PMETPARA-001
title: PMET Parameter Input Parser Contract (pmetpara.txt)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-PMETPARA-001 PMET Parameter Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-PMET-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md` (canonical sidecar grammar, symbol definitions, and policy gaps).
- `[DIRECT][E-SURVEY-PMET-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and ownership notes).
- `[DIRECT][E-WF-PMET-01]` `/workdir/wepp-forest/src/infile.for`, `/workdir/wepp-forest/src/pmetcoef.for`, `/workdir/wepp-forest/src/ccrpet.inc` (legacy PMET mode gate, parser behavior, symbol widths, and fallback).
- `[DIRECT][E-WP-PMET-01]` `/workdir/wepppy/wepppy/wepp/management/pmetpara.py`, `/workdir/wepppy/wepppy/nodb/core/wepp.py`, `/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py` (modern producer conventions and lifecycle behavior).
- `[INFERENCE][E-PHYS-PMET-01]` Process invariants: crop coefficient records must map deterministically to crop keys and must not silently mask missing crop parameterization in strict mode.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for optional sidecar surface `infile-pmetpara` (`pmetpara.txt`) and parse-to-runtime handoff of crop PMET coefficients.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | canonical count header + 5-field rows | Accept. | Canonical parse path. | `[DIRECT][E-SPEC-PMET-01]` |
| B | missing sidecar | Accept as optional absence branch. | Use non-PMET mode branch (`iflget=1`) with explicit provenance marker. | `[DIRECT][E-WF-PMET-01]` |
| C | datver-prefixed variant | Reject. | Emit typed unsupported-format error. | `[INFERENCE][E-SPEC-PMET-01]`, `[DIRECT][E-WF-PMET-01]` |

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
pmetpara_file    = irecord_line parameter_record{irecord} ;
irecord_line     = integer ;
parameter_record = names kcb rawp line actlnam ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves line count and ordered parameter records as parsed.
- Simulation model normalizes rows into typed PMET parameter records keyed by canonical crop name.
- Parser does not run ET calculations; runtime PMET kernels consume normalized parameters.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `irecord` | `header.irecord` | `pmetpara.record_count` | count | int | 1 | yes when file present | all | none | `pmetpara.record_count` |
| `names` | `records[i].names` | `pmetpara.records[i].crop_name` | none | string | irecord | yes | all | none | `pmetpara.records[i].crop_name` |
| `kcb` | `records[i].kcb` | `pmetpara.records[i].kcb` | none | real | irecord | yes | all | none | `pmetpara.records[i].kcb` |
| `rawp` | `records[i].rawp` | `pmetpara.records[i].rawp` | none | real | irecord | yes | all | none | `pmetpara.records[i].rawp` |
| `line` | `records[i].line` | `pmetpara.records[i].line_index` | count | int | irecord | yes | all | none | `pmetpara.records[i].line_index` |
| `actlnam` | `records[i].actlnam` | `pmetpara.records[i].description` | none | string | irecord | yes | all | none | `pmetpara.records[i].description` |
| derived `sidecar_present` | file presence branch | `pmetpara.mode.sidecar_present` | flag | bool | 1 | yes | all | derived: `true` when file exists and parsed, else `false` | `pmetpara.mode.sidecar_present` |
| derived `iflget` | file presence branch to ET mode selection | `pmetpara.mode.iflget` | enum int | 1 | yes | all | derived: `2` when sidecar present, `1` when sidecar absent | `pmetpara.mode.iflget` |
| derived `normalized_crop_key` | normalized `names` | `pmetpara.records[i].normalized_crop_key` | none | string | irecord | yes | all | strict: trim exact; compat: uppercase/truncate to width policy | `normalized_crop_key` |
| derived `fallback_first_row_used` | no-hit lookup branch | `pmetpara.lookup.fallback_first_row_used` | flag | bool | per lookup | conditional | all | compat-only derived branch | `fallback_first_row_used` |
| derived `line_count_closed` | `irecord == len(records)` | `pmetpara.line_count_closed` | flag | bool | 1 | yes | all | derived closure marker | `line_count_closed` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `irecord` | `header.irecord` | `pmetpara.record_count` | `input::sidecar::pmetpara` | init | immutable | row-count closure and lookup bounds | `G-PMET-001`, `G-PMET-002` |
| `names` | `records[*].names` | `pmetpara.records[*].crop_name` | `input::sidecar::pmetpara` | init | immutable | crop-key lookup and mapping | `G-PMET-003`, `G-PMET-005` |
| `kcb` | `records[*].kcb` | `pmetpara.records[*].kcb` | `input::sidecar::pmetpara` | init,daily | immutable | PMET ET coefficient selection | `G-PMET-004` |
| `rawp` | `records[*].rawp` | `pmetpara.records[*].rawp` | `input::sidecar::pmetpara` | init,daily | immutable | PMET ET coefficient selection | `G-PMET-004` |
| `line` | `records[*].line` | `pmetpara.records[*].line_index` | `input::sidecar::pmetpara` | init | immutable | diagnostics/provenance | `G-PMET-004` |
| `actlnam` | `records[*].actlnam` | `pmetpara.records[*].description` | `input::sidecar::pmetpara` | init | immutable | diagnostics/provenance | `G-PMET-004` |
| derived `sidecar_present` | `derived.sidecar_present` | `pmetpara.mode.sidecar_present` | `input::sidecar::pmetpara` | init | immutable | ET mode selection boundary | `G-PMET-009` |
| derived `iflget` | `derived.iflget` | `pmetpara.mode.iflget` | `input::sidecar::pmetpara` | init | immutable | ET branch selector (`iflget`) | `G-PMET-009` |
| derived `normalized_crop_key` | `derived.normalized_key[*]` | `pmetpara.records[*].normalized_crop_key` | `input::sidecar::pmetpara` | init | immutable | deterministic key matching | `G-PMET-005`, `G-PMET-006` |
| derived `fallback_first_row_used` | `derived.fallback_first_row_used` | `pmetpara.lookup.fallback_first_row_used` | `runtime::et::pmet_lookup` | lookup,daily | mutable | strict/compat observability and ET debugging | `G-PMET-007` |
| derived `line_count_closed` | `derived.line_count_closed` | `pmetpara.line_count_closed` | `input::sidecar::pmetpara` | init | immutable | parse closure diagnostics | `G-PMET-002` |

## 5. State Ownership and Mutability

- `input::sidecar::pmetpara` owns parsed source records and normalized PMET parameter state.
- Parsed PMET records are immutable after parse success.
- Runtime ET modules own mutable lookup/session state (`pmetpara.lookup.*`) including per-lookup fallback provenance markers, but may not mutate canonical PMET parameter records.
- Forbidden mutation path: runtime modules mutating parsed PMET coefficients (`kcb`, `rawp`) in place.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-PMET-001` | Derive normalized crop-key per row from `names` according to mode policy. | parse finalize | `C-PMET-001` |
| `D-PMET-002` | Derive line-count closure marker from `irecord` and parsed row count. | parse finalize | `C-PMET-002` |
| `D-PMET-003` | Derive sidecar presence and ET branch selector (`iflget`) from optional-surface branch. | parse preamble | `C-PMET-003` |
| `D-PMET-004` | Derive fallback usage marker when no crop-key match is found and compat fallback is enabled. | lookup time | `C-PMET-004` |

Closure hooks:
- `C-PMET-001`: normalized keys must be deterministic within selected policy mode.
- `C-PMET-002`: declared row count must exactly match parsed rows.
- `C-PMET-003`: optional-surface branch (`iflget=1` vs `iflget=2`) must be explicit and observable.
- `C-PMET-004`: fallback branch must remain explicit and observable; no hidden fallback in strict mode.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `PMET-E-000` | io | sidecar open/read failure when required |
| `PMET-E-001` | syntax | token parse failure for required numeric fields |
| `PMET-E-002` | syntax | row-count closure mismatch (`irecord` vs parsed rows) |
| `PMET-E-003` | semantic | invalid field domains (`irecord<=0`, duplicate keys in strict mode) |
| `PMET-E-004` | semantic | unsupported header variant (for example datver-prefixed format) |
| `PMET-E-005` | cross-file | crop-key lookup miss in strict mode |
| `PMET-E-006` | runtime-guard | post-parse lookup/normalization closure failure |
| `PMET-E-007` | cross-file | PMET-required ET mode selected while sidecar is absent in strict mode |
| `PMET-E-008` | syntax | unsupported quoted/multi-token `actlnam` tokenization form in strict mode |
| `PMET-W-001` | compat-warning | missing sidecar branch taken (`iflget=1`) |
| `PMET-W-002` | compat-warning | crop-key truncation/normalization applied |
| `PMET-W-003` | compat-warning | first-row fallback used for no-hit lookup |
| `PMET-W-004` | compat-warning | non-canonical `actlnam` tokenization normalized in compatibility mode |

No silent fallback/default masking is permitted in strict mode.

## 8. Cross-File Consistency Constraints

1. Crop keys (`names`) must couple to management canonical crop symbols (`iplane`/management row crop-name surface) with the same normalization-width policy used by PMET lookup (`normalized_crop_key`).
2. PMET sidecar presence/absence must couple to ET mode branch semantics by explicit mode surface (`pmetpara.mode.iflget`: absent `=1`, present `=2`).
3. Runtime crop-loop lookup behavior must be deterministic for all crop records; strict mode forbids first-row fallback for no-hit lookup.
4. Modern orchestration sidecar lifecycle must preserve parser assumptions (count header + canonical rows) and must not emit sidecar-absent + `iflget=2` inconsistent state.

Evidence: `[DIRECT][E-SPEC-PMET-01]`, `[DIRECT][E-WF-PMET-01]`, `[DIRECT][E-WP-PMET-01]`.

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `irecord` | `pmetpara.record_count` | `openwepp.boundary.parser.pmetpara.v1.header` | canonical `irecord` + alias `record_count` | required closure signal |
| row symbols (`names,kcb,rawp,line,actlnam`) | `pmetpara.records[*]` | `openwepp.boundary.et.pmetpara_parameters.v1.records` | canonical symbol continuity + aliases | preserves row order/provenance |
| optional-surface mode symbols (`sidecar_present,iflget`) | `pmetpara.mode.*` | `openwepp.boundary.et.mode_selection.v1` | `{sidecar_present,iflget}` | explicit missing-sidecar branch (`iflget=1`) |
| derived lookup markers (`normalized_crop_key,fallback_first_row_used,line_count_closed`) | `pmetpara.derived` / `pmetpara.lookup` | `openwepp.boundary.observability.parser_warnings.v1` | explicit derived fields + warning IDs | required for strict/compat auditability |

## 10. Compatibility Policy

- Strict mode:
  - requires canonical count-header format;
  - rejects datver-prefixed variants;
  - enforces exact row-count closure;
  - enforces exact crop-key lookup (no fallback);
  - rejects duplicate crop keys.
- Compatibility mode:
  - allows optional surface absence branch with typed warning (`PMET-W-001`);
  - allows crop-key normalization/truncation per legacy-width policy with `PMET-W-002`;
  - allows first-row fallback for no-hit lookup with `PMET-W-003`;
  - allows non-canonical `actlnam` tokenization normalization with `PMET-W-004`.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-PMET-001` | valid count header domain (`irecord>0`) | header parse | `PMET-E-003` |
| `G-PMET-002` | row-count closure (`irecord == parsed_rows`) | parse finalize | `PMET-E-002` |
| `G-PMET-003` | canonical row arity/presence | row parse | `PMET-E-001` |
| `G-PMET-004` | numeric domain parse for `kcb/rawp/line` | row parse | `PMET-E-001`/`PMET-E-003` |
| `G-PMET-005` | deterministic crop-key normalization | parse finalize | `PMET-E-006` |
| `G-PMET-006` | strict width/uniqueness policy for crop keys | parse finalize | `PMET-E-003` |
| `G-PMET-007` | strict/compat lookup branch policy + warning emission | lookup policy gate | strict no-hit: `PMET-E-005`; compat: `PMET-W-002`/`PMET-W-003` |
| `G-PMET-008` | unsupported header variant rejection | preamble parse | `PMET-E-004` |
| `G-PMET-009` | sidecar-presence mode branch (`iflget=1` absent, `iflget=2` present) + warning emission | preamble/policy gate | strict required-missing: `PMET-E-007`; compat missing-sidecar: `PMET-W-001` |
| `G-PMET-010` | `actlnam` tokenization policy (strict canonical single-token/list-directed form vs compat normalization) | row parse/policy gate | strict: `PMET-E-008`; compat: `PMET-W-004` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`irecord`, `names`, `kcb`, `rawp`, `line`, `actlnam`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `PMET-GAP-001` | Legacy no-hit behavior falls back to first row; strict-vs-compat production policy needs fixture-backed migration coverage. | `[DIRECT][E-SPEC-PMET-01]`, `[DIRECT][E-WF-PMET-01]` | `HOLD` |
| `PMET-GAP-002` | Width/truncation policy for keys/descriptions across legacy fixed-width symbols and modern emitters requires finalized boundary contract limits. | `[DIRECT][E-SPEC-PMET-01]`, `[DIRECT][E-WF-PMET-01]`, `[DIRECT][E-WP-PMET-01]` | `HOLD` |
| `PMET-GAP-003` | Delimiter/quoting grammar for `actlnam` remains under-specified in literature and must be ratified for strict parsing. | `[DIRECT][E-SPEC-PMET-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE10. |
