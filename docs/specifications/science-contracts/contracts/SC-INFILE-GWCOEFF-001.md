---
contract_id: SC-INFILE-GWCOEFF-001
title: Groundwater Coefficients Sidecar Input Parser Contract (gwcoeff.txt)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-GWCOEFF-001 Groundwater Coefficients Sidecar Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-GW-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md` (canonical sidecar grammar, symbol definitions, and unresolved policy gaps).
- `[DIRECT][E-SURVEY-GW-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and parser ownership context).
- `[DIRECT][E-WF-GW-01]` `/workdir/wepp-forest/src/main.for`, `/workdir/wepp-forest/src/cchrt1.inc`, `/workdir/wepp-forest/src/contin.for`, `/workdir/wepp-forest/src/wshchr.f90`, `/workdir/wepp-forest/src/wshdrv.f90` (legacy read sequence, symbols, and routing behavior).
- `[DIRECT][E-WP-GW-01]` `/workdir/wepppy/wepppy/nodb/core/wepp.py`, `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py` (modern sidecar authoring and payload ingress conventions).
- `[INFERENCE][E-PHYS-GW-01]` Process/common-sense invariants: depth and coefficients must be finite/non-negative; threshold area must not be negative.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for optional sidecar surface `infile-gwcoeff` (`gwcoeff.txt`) and parse-to-runtime handoff of groundwater/baseflow coefficients used by watershed routing and groundwater balance pathways.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | sidecar absent | Accept optional absence branch. | Set sidecar-disabled mode (`lr_bf=0`) with explicit provenance. | `[DIRECT][E-WF-GW-01]` |
| B | sidecar present with 4 numeric-leading records | Accept. | Canonical parse path (`igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`). | `[DIRECT][E-SPEC-GW-01]`, `[DIRECT][E-WF-GW-01]` |
| C | sidecar present malformed/incomplete | Reject in strict mode. | Emit typed parse/record-count errors; no silent fallback. | `[INFERENCE][E-SPEC-GW-01]` |
| D | prefixed/datver-style extra header line | Provisional policy: reject; no legacy authority for version preface. | Emit typed unsupported-format error. | `[DIRECT][E-SPEC-GW-01]`, `[INFERENCE][E-WF-GW-01]` |

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
gwcoeff_file         = strict_gwcoeff_file | compat_gwcoeff_file ;
strict_gwcoeff_file  = igwstrd_line bfcoeff_line dscoeff_line bftharea_line ;
compat_gwcoeff_file  = igwstrd_line_compat bfcoeff_line_compat dscoeff_line_compat bftharea_line_compat ;

igwstrd_line         = real [trailing_tokens] ;
bfcoeff_line         = real [trailing_tokens] ;
dscoeff_line         = real [trailing_tokens] ;
bftharea_line        = real [trailing_tokens] ;

igwstrd_line_compat  = real [trailing_tokens] ;
bfcoeff_line_compat  = real [trailing_tokens] ;
dscoeff_line_compat  = real [trailing_tokens] ;
bftharea_line_compat = real [trailing_tokens] ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves line order (`1..4`) and captures raw optional trailing token presence per line.
- Simulation model normalizes to typed runtime payload:
  - coefficients (`igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`),
  - sidecar branch state (`gwcoeff_file_present`, `lr_bf`),
  - strict/compat provenance markers.
- Parser does not execute groundwater routing equations; downstream hydrology modules consume normalized state.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `igwstrd` | `line1.igwstrd` | `groundwater.options.igwstrd_mm` | mm | real | 0..1 | conditional | all | none when file present | `groundwater.initial_storage_depth_mm` |
| `bfcoeff` | `line2.bfcoeff` | `groundwater.options.bfcoeff_per_day` | day^-1 | real | 0..1 | conditional | all | none when file present | `groundwater.baseflow_coeff_per_day` |
| `dscoeff` | `line3.dscoeff` | `groundwater.options.dscoeff_per_day` | day^-1 | real | 0..1 | conditional | all | none when file present | `groundwater.deep_seepage_coeff_per_day` |
| `bftharea` | `line4.bftharea` | `groundwater.options.bftharea_ha` | ha | real | 0..1 | conditional | all | none when file present | `groundwater.baseflow_threshold_area_ha` |
| derived `gwcoeff_file_present` | sidecar presence branch | `groundwater.options.gwcoeff_file_present` | flag | bool | 1 | yes | all | `true` when parse success branch is taken, `false` only for explicit missing-file branch; malformed present-file paths terminate with typed error | `groundwater.sidecar_present` |
| derived `lr_bf` | sidecar mode branch | `groundwater.options.lr_bf` | flag | int | 1 | yes | all | `1` when sidecar parse succeeds; `0` only for explicit missing-file branch; malformed present-file branch emits error and no normalized state | `groundwater.linear_reservoir_enabled` |
| derived `parse_outcome` | parser branch outcome | `groundwater.options.parse_outcome` | enum | string | 1 | yes | all | values: `missing_branch`, `parsed_branch`; malformed present-file path is represented as typed error rather than model emission | `parse_outcome` |
| derived `line_count_closed` | record-count closure | `groundwater.options.line_count_closed` | flag | bool | 1 | yes | all | `true` when exactly 4 records parsed | `line_count_closed` |
| derived `trailing_token_lines` | tokenization provenance | `groundwater.options.trailing_token_lines` | line-index set | list<int> | 0..4 | conditional | all | line-level provenance for canonical numeric-leading records with optional trailing text/comments | `trailing_token_lines` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `igwstrd` | `records.igwstrd` | `groundwater.options.igwstrd_mm` | `input::sidecar::gwcoeff` | init,daily,watershed | immutable | groundwater storage initialization and daily update | `G-GW-001`, `G-GW-003` |
| `bfcoeff` | `records.bfcoeff` | `groundwater.options.bfcoeff_per_day` | `input::sidecar::gwcoeff` | init,daily,watershed | immutable | baseflow flux computation | `G-GW-001`, `G-GW-003` |
| `dscoeff` | `records.dscoeff` | `groundwater.options.dscoeff_per_day` | `input::sidecar::gwcoeff` | init,daily,watershed | immutable | deep seepage computation | `G-GW-001`, `G-GW-003` |
| `bftharea` | `records.bftharea` | `groundwater.options.bftharea_ha` | `input::sidecar::gwcoeff` | init,watershed | immutable | threshold-area suppression branch | `G-GW-001`, `G-GW-004` |
| derived `gwcoeff_file_present` | `derived.file_present` | `groundwater.options.gwcoeff_file_present` | `input::sidecar::gwcoeff` | init | immutable | mode/provenance diagnostics | `G-GW-005` |
| derived `lr_bf` | `derived.lr_bf` | `groundwater.options.lr_bf` | `input::sidecar::gwcoeff` | init,watershed | immutable | routing baseflow branch selection | `G-GW-005`, `G-GW-007` |
| derived `parse_outcome` | `derived.parse_outcome` | `groundwater.options.parse_outcome` | `input::sidecar::gwcoeff` | init | immutable | strict/compat branch observability | `G-GW-005` |
| derived `line_count_closed` | `derived.line_count_closed` | `groundwater.options.line_count_closed` | `input::sidecar::gwcoeff` | init | immutable | parse closure diagnostics | `G-GW-002` |
| derived `trailing_token_lines` | `derived.trailing_token_lines` | `groundwater.options.trailing_token_lines` | `input::sidecar::gwcoeff` | init | immutable | strict/compat observability | `G-GW-006` |

## 5. State Ownership and Mutability

- `input::sidecar::gwcoeff` owns parsed sidecar records and derived parser provenance surfaces.
- Parsed coefficient values are immutable after parse success.
- Runtime hydrology modules own mutable simulation state (`gwatst`, routed daily flux terms), but must not mutate parsed coefficient surfaces.
- Forbidden mutation path: runtime modules rewriting `igwstrd/bfcoeff/dscoeff/bftharea` in place after parser finalization.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-GW-001` | Derive sidecar presence marker from open/parse branch. | parse preamble/finalize | `C-GW-001` |
| `D-GW-002` | Derive `lr_bf` from sidecar presence marker. | parse finalize | `C-GW-002` |
| `D-GW-003` | Derive line-count closure marker (`exactly 4 records`). | parse finalize | `C-GW-003` |
| `D-GW-004` | Derive per-line trailing-token provenance for canonical numeric-leading line shape. | parse finalize | `C-GW-004` |
| `D-GW-005` | Derive parse-outcome marker for successful model-emission branches (`missing_branch` vs `parsed_branch`). | parse finalize | `C-GW-005` |

Closure hooks:
- `C-GW-001`: missing-file branch must be explicit and observable.
- `C-GW-002`: `lr_bf` branch derivation must be deterministic (`0|1`) and aligned with successful parse branches only.
- `C-GW-003`: present-file parse must close exactly at four records.
- `C-GW-004`: canonical trailing-token handling must preserve line-level provenance.
- `C-GW-005`: malformed present-file branch must emit typed failure and must not emit normalized `lr_bf` model state.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `GW-E-000` | io | sidecar open/read error when present/required |
| `GW-E-001` | syntax | numeric token parse failure on required line |
| `GW-E-002` | syntax | record-count/arity mismatch (expected 4 records) |
| `GW-E-003` | semantic | non-finite numeric values |
| `GW-E-004` | semantic | invalid domain (`igwstrd<0`, `bfcoeff<0`, `dscoeff<0`, `bftharea<0`) |
| `GW-E-005` | cross-file | coeff-namespace conflation with `chan.inp` unit-area baseflow coefficient surface |
| `GW-E-006` | runtime-guard | post-parse closure mismatch (`lr_bf` vs state export/routing branch) |
| `GW-E-007` | syntax | unsupported prefixed/datver-like variant |
| `GW-W-001` | compat-warning | optional-surface absence branch taken (`lr_bf=0`) |

No silent parser-side fallback is permitted for malformed present-file input in strict mode.

## 8. Cross-File Consistency Constraints

1. `bftharea` must be interpreted with watershed-area coupling (`wsarea/10000` in ha) before threshold branch checks. `[DIRECT][E-WF-GW-01]`
2. `lr_bf` branch selection must remain consistent with routing-mode usage (linear-reservoir vs unit-area baseflow path). `[DIRECT][E-WF-GW-01]`
3. `gwcoeff.txt` `bfcoeff` must remain namespace-separated from `chan.inp` baseflow coefficient semantics/units. `[DIRECT][E-SPEC-GW-01]`
4. Modern orchestration sidecar emission (`baseflow_opts_*`) must map losslessly to canonical symbols and units. `[DIRECT][E-WP-GW-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `igwstrd,bfcoeff,dscoeff,bftharea` | `groundwater.options.*` | `openwepp.boundary.parser.gwcoeff.v1.records` | canonical symbols + aliases `groundwater.*` | parser emits unit-preserving values |
| `gwcoeff_file_present,lr_bf` | `groundwater.options.{gwcoeff_file_present,lr_bf}` | `openwepp.boundary.hydrology.mode_selection.v1` | `{gwcoeff_file_present,lr_bf}` | controls downstream routing branch |
| `line_count_closed,trailing_token_lines` | `groundwater.options.{line_count_closed,trailing_token_lines}` | `openwepp.boundary.observability.parser_warnings.v1` | explicit derived fields + warning IDs | supports strict/compat diagnostics |

## 10. Compatibility Policy

- Strict mode:
  - requires exactly four numeric-leading records;
  - accepts canonical trailing text/comments after numeric-leading tokens;
  - rejects prefixed/datver-like variants;
  - rejects malformed present-file payloads with typed errors.
- Compatibility mode:
  - allows optional-file absence branch with explicit warning/provenance (`GW-W-001`);
  - accepts canonical trailing text/comments after numeric-leading tokens;
  - preserves `lr_bf` derivation from sidecar presence only; does not infer from line payload content.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-GW-001` | line-wise numeric parse for `igwstrd,bfcoeff,dscoeff,bftharea` | parse lines 1..4 | `GW-E-001`/`GW-E-003` |
| `G-GW-002` | present-file record-count closure (`==4`) | parse finalize | `GW-E-002` |
| `G-GW-003` | non-negative domains for coefficient/depth fields | parse finalize | `GW-E-004` |
| `G-GW-004` | threshold-area semantics (`bftharea` interpreted in ha) | cross-surface validator | `GW-E-004`/`GW-E-006` |
| `G-GW-005` | sidecar presence branch and `lr_bf` derivation closure | preamble/finalize | `GW-E-006`/`GW-W-001` |
| `G-GW-006` | canonical numeric-leading tokenization policy with optional trailing text/comments | parse finalize/policy gate | `GW-E-001` |
| `G-GW-007` | routing branch consistency (`lr_bf` selection) | runtime branch validator | `GW-E-006` |
| `G-GW-008` | coefficient namespace separation from `chan.inp` baseflow coefficient | cross-file validator | `GW-E-005` |
| `G-GW-009` | prefixed/datver-like variant rejection | parse preamble | `GW-E-007` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`igwstrd`, `bfcoeff`, `dscoeff`, `bftharea`, `lr_bf`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `GWCOEFF-GAP-001` | `usersum2024` sidecar section does not publish a dedicated `gwcoeff.txt` format definition. | `[DIRECT][E-SPEC-GW-01]` | `HOLD` |
| `GWCOEFF-GAP-002` | Coefficient-name collision risk with `chan.inp` baseflow-coefficient semantics requires governance-locked namespace guard tests. | `[DIRECT][E-SPEC-GW-01]` | `HOLD` |
| `GWCOEFF-GAP-003` | Missing-file branch behavior and explicit default-value publication policy remain unresolved. | `[DIRECT][E-SPEC-GW-01]`, `[DIRECT][E-WF-GW-01]` | `HOLD` |
| `GWCOEFF-GAP-004` | Present-file parse-failure behavior in legacy path is implicit and requires fixture-backed strict/compat validation policy closure. | `[DIRECT][E-SPEC-GW-01]`, `[DIRECT][E-WF-GW-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE13. |
