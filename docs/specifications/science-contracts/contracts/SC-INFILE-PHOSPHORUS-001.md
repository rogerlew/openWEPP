---
contract_id: SC-INFILE-PHOSPHORUS-001
title: Phosphorus Sidecar Input Parser Contract (phosphorus.txt)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-PHOSPHORUS-001 Phosphorus Sidecar Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-PHOS-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/phosphorus.spec.md` (canonical grammar, symbols, and policy gaps).
- `[DIRECT][E-SURVEY-PHOS-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and parser ownership context).
- `[DIRECT][E-WF-PHOS-01]` `/workdir/wepp-forest/src/main.for`, `/workdir/wepp-forest/src/cchrt1.inc`, `/workdir/wepp-forest/src/wshdrv.f90`, `/workdir/wepp-forest/src/wshcqi.f90` (legacy parse, enable flag, and routing propagation).
- `[DIRECT][E-WP-PHOS-01]` `/workdir/wepppy/wepppy/nodb/core/wepp.py`, `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py` (modern emission and validation behavior).
- `[INFERENCE][E-PHYS-PHOS-01]` Process/common-sense invariants: concentration values must be finite and non-negative; enabled-state must propagate deterministically.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for optional sidecar surface `infile-phosphorus` (`phosphorus.txt`) and parse-to-runtime handoff of channel-routing phosphorus concentration controls.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | sidecar absent | Accept optional absence branch. | Set `p_flag=0` with explicit provenance. | `[DIRECT][E-WF-PHOS-01]` |
| B | sidecar present with header + 4 concentration records | Accept. | Canonical parse path and propagation fanout. | `[DIRECT][E-SPEC-PHOS-01]`, `[DIRECT][E-WF-PHOS-01]` |
| C | sidecar present malformed/incomplete | Reject strict. | Emit typed parse/count errors; no silent disablement fallback. | `[INFERENCE][E-SPEC-PHOS-01]` |
| D | header literal differs from `Phosphorus concentration` | Strict reject; compatibility may allow ignored header. | Strict: typed header mismatch error; compat: warning + continue. | `[DIRECT][E-WF-PHOS-01]`, `[DIRECT][E-WP-PHOS-01]` |

No datver/version line is defined for this sidecar.

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
phosphorus_file         = strict_phosphorus_file | compat_phosphorus_file ;
strict_phosphorus_file  = header_line_strict srp_line slfp_line bfp_line scp_line ;
compat_phosphorus_file  = header_line_compat srp_line_compat slfp_line_compat bfp_line_compat scp_line_compat ;

header_line_strict      = "Phosphorus concentration" ;
header_line_compat      = text ;

srp_line                = real [trailing_tokens] ;
slfp_line               = real [trailing_tokens] ;
bfp_line                = real [trailing_tokens] ;
scp_line                = real [trailing_tokens] ;

srp_line_compat         = real [trailing_tokens] ;
slfp_line_compat        = real [trailing_tokens] ;
bfp_line_compat         = real [trailing_tokens] ;
scp_line_compat         = real [trailing_tokens] ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves header text and ordered concentration records (`srp`, `slfp`, `bfp`, `scp`), including optional trailing tokens in compatibility mode.
- Simulation model normalizes to typed scalar concentrations and derived runtime toggles (`phosphorus_file_present`, `p_flag`) plus propagation provenance.
- Parser does not execute routing mass-balance calculations.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| derived `header_text` | `line1.header_text` | `phosphorus.options.header_text` | text | string | 0..1 | conditional | all | preserved exactly as read for strict/compat policy evaluation | `phosphorus.header_text` |
| `srp` | `line2.srp` | `phosphorus.options.srp_mg_l` | mg/L | real | 0..1 | conditional | all | none when file present | `phosphorus.surface_runoff_mg_l` |
| `slfp` | `line3.slfp` | `phosphorus.options.slfp_mg_l` | mg/L | real | 0..1 | conditional | all | none when file present | `phosphorus.lateral_flow_mg_l` |
| `bfp` | `line4.bfp` | `phosphorus.options.bfp_mg_l` | mg/L | real | 0..1 | conditional | all | none when file present | `phosphorus.baseflow_mg_l` |
| `scp` | `line5.scp` | `phosphorus.options.scp_mg_kg` | mg/kg | real | 0..1 | conditional | all | none when file present | `phosphorus.sediment_mg_kg` |
| `p_flag` | derived from sidecar branch | `phosphorus.options.p_flag` | flag | int | 1 | yes | all | `0` when absent, `1` when present+parsed | `phosphorus.enabled` |
| derived `phosphorus_file_present` | sidecar presence branch | `phosphorus.options.phosphorus_file_present` | flag | bool | 1 | yes | all | `true` on parse success, else `false` | `phosphorus.sidecar_present` |
| derived `header_literal_match` | strict header policy | `phosphorus.options.header_literal_match` | flag | bool | 1 | yes | all | `true` when header matches strict literal | `header_literal_match` |
| derived `line_count_closed` | record-count closure | `phosphorus.options.line_count_closed` | flag | bool | 1 | yes | all | `true` when exactly 5 records parsed | `line_count_closed` |
| derived `trailing_token_lines` | tokenization provenance | `phosphorus.options.trailing_token_lines` | line-index set | list<int> | 0..4 | conditional | all | line-level provenance for canonical numeric-leading records with optional trailing text/comments | `trailing_token_lines` |
| `tmpsrp` | fanout from scalar `srp` | `phosphorus.routing.by_hillslope.tmpsrp_mg_l` | mg/L | real array | per hillslope | conditional | all | copied from `srp` at init when enabled | `phosphorus.by_hillslope.tmpsrp_mg_l` |
| `tmpslfp` | fanout from scalar `slfp` | `phosphorus.routing.by_hillslope.tmpslfp_mg_l` | mg/L | real array | per hillslope | conditional | all | copied from `slfp` at init when enabled | `phosphorus.by_hillslope.tmpslfp_mg_l` |
| `tmpbfp` | fanout from scalar `bfp` | `phosphorus.routing.by_hillslope.tmpbfp_mg_l` | mg/L | real array | per hillslope | conditional | all | copied from `bfp` at init when enabled | `phosphorus.by_hillslope.tmpbfp_mg_l` |
| `tmpscp` | fanout from scalar `scp` | `phosphorus.routing.by_hillslope.tmpscp_mg_kg` | mg/kg | real array | per hillslope | conditional | all | copied from `scp` at init when enabled | `phosphorus.by_hillslope.tmpscp_mg_kg` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| derived `header_text` | `records.header_text` | `phosphorus.options.header_text` | `input::sidecar::phosphorus` | init | immutable | strict/compat header policy and observability | `G-PHOS-006` |
| `srp` | `records.srp` | `phosphorus.options.srp_mg_l` | `input::sidecar::phosphorus` | init,watershed | immutable | runoff phosphorus routing | `G-PHOS-001`, `G-PHOS-003`, `G-PHOS-004` |
| `slfp` | `records.slfp` | `phosphorus.options.slfp_mg_l` | `input::sidecar::phosphorus` | init,watershed | immutable | lateral-flow phosphorus routing | `G-PHOS-001`, `G-PHOS-003`, `G-PHOS-004` |
| `bfp` | `records.bfp` | `phosphorus.options.bfp_mg_l` | `input::sidecar::phosphorus` | init,watershed | immutable | baseflow phosphorus routing | `G-PHOS-001`, `G-PHOS-003`, `G-PHOS-004` |
| `scp` | `records.scp` | `phosphorus.options.scp_mg_kg` | `input::sidecar::phosphorus` | init,watershed | immutable | sediment phosphorus routing | `G-PHOS-001`, `G-PHOS-003`, `G-PHOS-004` |
| `p_flag` | `derived.p_flag` | `phosphorus.options.p_flag` | `input::sidecar::phosphorus` | init,watershed | immutable | enable/disable routing fanout | `G-PHOS-005` |
| derived `phosphorus_file_present` | `derived.file_present` | `phosphorus.options.phosphorus_file_present` | `input::sidecar::phosphorus` | init | immutable | parser provenance | `G-PHOS-005` |
| derived `header_literal_match` | `derived.header_literal_match` | `phosphorus.options.header_literal_match` | `input::sidecar::phosphorus` | init | immutable | strict/compat policy observability | `G-PHOS-006` |
| derived `line_count_closed` | `derived.line_count_closed` | `phosphorus.options.line_count_closed` | `input::sidecar::phosphorus` | init | immutable | parse closure diagnostics | `G-PHOS-002` |
| derived `trailing_token_lines` | `derived.trailing_token_lines` | `phosphorus.options.trailing_token_lines` | `input::sidecar::phosphorus` | init | immutable | strict/compat diagnostics | `G-PHOS-007` |
| `tmpsrp` | `derived.propagation_fanout.tmpsrp` | `phosphorus.routing.by_hillslope.tmpsrp_mg_l` | `runtime::watershed::phosphorus_routing` | init,watershed | immutable | channel routing and report kernels | `G-PHOS-008` |
| `tmpslfp` | `derived.propagation_fanout.tmpslfp` | `phosphorus.routing.by_hillslope.tmpslfp_mg_l` | `runtime::watershed::phosphorus_routing` | init,watershed | immutable | channel routing and report kernels | `G-PHOS-008` |
| `tmpbfp` | `derived.propagation_fanout.tmpbfp` | `phosphorus.routing.by_hillslope.tmpbfp_mg_l` | `runtime::watershed::phosphorus_routing` | init,watershed | immutable | channel routing and report kernels | `G-PHOS-008` |
| `tmpscp` | `derived.propagation_fanout.tmpscp` | `phosphorus.routing.by_hillslope.tmpscp_mg_kg` | `runtime::watershed::phosphorus_routing` | init,watershed | immutable | channel routing and report kernels | `G-PHOS-008` |

## 5. State Ownership and Mutability

- `input::sidecar::phosphorus` owns parsed scalar records and parser provenance state.
- Parsed scalar concentrations and enable flags are immutable after parse finalization.
- `runtime::watershed::phosphorus_routing` owns mutable route-time accumulator states but not parser-owned canonical concentrations.
- Forbidden mutation path: downstream routing modules mutating parser-owned concentration scalars in place.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-PHOS-001` | Derive sidecar presence marker from open/parse branch. | parse preamble/finalize | `C-PHOS-001` |
| `D-PHOS-002` | Derive `p_flag` from sidecar presence and parse closure. | parse finalize | `C-PHOS-002` |
| `D-PHOS-003` | Derive header literal match marker for strict policy. | parse line1 | `C-PHOS-003` |
| `D-PHOS-004` | Derive line-count closure marker (`exactly 5 records`). | parse finalize | `C-PHOS-004` |
| `D-PHOS-005` | Derive per-line trailing-token provenance in compatibility mode. | parse finalize | `C-PHOS-005` |
| `D-PHOS-006` | Derive propagation fanout closure across hillslope arrays when enabled. | init fanout | `C-PHOS-006` |

Closure hooks:
- `C-PHOS-001`: optional-sidecar branch must be explicit.
- `C-PHOS-002`: `p_flag` must be deterministic and aligned with branch state.
- `C-PHOS-003`: strict header-literal policy must be enforced.
- `C-PHOS-004`: present-file parse must close at exactly five records.
- `C-PHOS-005`: compatibility trailing-token acceptance must preserve line-level provenance.
- `C-PHOS-006`: when enabled, concentration fanout to routing arrays must be complete and lossless.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `PHOS-E-000` | io | sidecar open/read error when present/required |
| `PHOS-E-001` | syntax | numeric parse failure on required concentration line |
| `PHOS-E-002` | syntax | record-count mismatch (expected header + 4 concentration lines) |
| `PHOS-E-003` | semantic | non-finite concentration values |
| `PHOS-E-004` | semantic | out-of-domain concentration (negative) |
| `PHOS-E-005` | cross-file | propagation fanout mismatch to per-hillslope routing arrays |
| `PHOS-E-006` | runtime-guard | post-parse enable/branch closure failure (`p_flag` mismatch) |
| `PHOS-E-007` | syntax | strict header literal mismatch |
| `PHOS-W-001` | compat-warning | optional sidecar absence branch taken (`p_flag=0`) |
| `PHOS-W-002` | compat-warning | non-canonical header accepted/ignored in compatibility mode |

No silent parser-side fallback is permitted for malformed present-file input in strict mode.

## 8. Cross-File Consistency Constraints

1. `p_flag` enable state must be consistent with routing-branch activation and fanout of `tmps*` concentration arrays. `[DIRECT][E-WF-PHOS-01]`
2. Phosphorus concentration units must remain stable across parser/routing/report boundaries (`mg/L` for `srp/slfp/bfp`, `mg/kg` for `scp`). `[DIRECT][E-SPEC-PHOS-01]`
3. Per-hillslope array propagation must close for all active hillslopes when `p_flag=1`. `[DIRECT][E-WF-PHOS-01]`
4. Modern orchestration payload aliases (`phosphorus_opts_*`) must map losslessly to canonical symbols. `[DIRECT][E-WP-PHOS-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `srp,slfp,bfp,scp` | `phosphorus.options.*` | `openwepp.boundary.parser.phosphorus.v1.records` | canonical symbols + aliases `phosphorus.*` | unit-preserving scalar export |
| `phosphorus_file_present,p_flag` | `phosphorus.options.{phosphorus_file_present,p_flag}` | `openwepp.boundary.routing.mode_selection.v1` | `{phosphorus_file_present,p_flag}` | controls routing enable branch |
| `header_literal_match,line_count_closed,trailing_token_lines` | `phosphorus.options.{header_literal_match,line_count_closed,trailing_token_lines}` | `openwepp.boundary.observability.parser_warnings.v1` | explicit derived diagnostics | strict/compat observability |
| `tmpsrp` | `phosphorus.routing.by_hillslope.tmpsrp_mg_l` | `openwepp.boundary.routing.phosphorus_inputs.v1` | `tmpsrp` + alias `phosphorus.by_hillslope.tmpsrp_mg_l` | fanout closure required |
| `tmpslfp` | `phosphorus.routing.by_hillslope.tmpslfp_mg_l` | `openwepp.boundary.routing.phosphorus_inputs.v1` | `tmpslfp` + alias `phosphorus.by_hillslope.tmpslfp_mg_l` | fanout closure required |
| `tmpbfp` | `phosphorus.routing.by_hillslope.tmpbfp_mg_l` | `openwepp.boundary.routing.phosphorus_inputs.v1` | `tmpbfp` + alias `phosphorus.by_hillslope.tmpbfp_mg_l` | fanout closure required |
| `tmpscp` | `phosphorus.routing.by_hillslope.tmpscp_mg_kg` | `openwepp.boundary.routing.phosphorus_inputs.v1` | `tmpscp` + alias `phosphorus.by_hillslope.tmpscp_mg_kg` | fanout closure required |

## 10. Compatibility Policy

- Strict mode:
  - requires strict header literal and exactly 5 records;
  - accepts canonical numeric-leading concentration records with optional trailing text/comments;
  - rejects malformed present-file payloads;
  - rejects negative or non-finite concentration values.
- Compatibility mode:
  - allows optional sidecar absence branch with explicit warning/provenance (`PHOS-W-001`);
  - allows non-canonical header text with `PHOS-W-002`;
  - accepts canonical numeric-leading concentration records with optional trailing text/comments.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-PHOS-001` | required concentration-line numeric parsing | parse lines 2..5 | `PHOS-E-001`/`PHOS-E-003` |
| `G-PHOS-002` | present-file record-count closure (`==5`) | parse finalize | `PHOS-E-002` |
| `G-PHOS-003` | non-negative concentration domains | parse finalize | `PHOS-E-004` |
| `G-PHOS-004` | unit-preserving concentration field mapping | boundary validator | `PHOS-E-006` |
| `G-PHOS-005` | sidecar presence and `p_flag` derivation closure | preamble/finalize | `PHOS-E-006`/`PHOS-W-001` |
| `G-PHOS-006` | strict header literal policy | parse line1/policy gate | strict: `PHOS-E-007`; compat: `PHOS-W-002` |
| `G-PHOS-007` | canonical numeric-leading tokenization policy with optional trailing text/comments | parse finalize/policy gate | `PHOS-E-001` when required numeric-leading token parse fails |
| `G-PHOS-008` | routing fanout completeness to `tmps*` arrays | init fanout validator | `PHOS-E-005` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`p_flag`, `srp`, `slfp`, `bfp`, `scp`, `tmpsrp`, `tmpslfp`, `tmpbfp`, `tmpscp`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `PHOS-GAP-001` | `usersum2024` does not publish a dedicated `phosphorus.txt` format specification. | `[DIRECT][E-SPEC-PHOS-01]` | `RATIFIED-W4DR-001 (2026-05-22)` |
| `PHOS-GAP-002` | Final concentration-range policy bounds remain unresolved beyond non-negative and finite checks. | `[DIRECT][E-SPEC-PHOS-01]` | `RATIFIED-W4DR-009 (2026-05-22)` |
| `PHOS-GAP-003` | Hillslope-only applicability semantics remain unresolved relative to watershed-centric routing evidence. | `[DIRECT][E-SPEC-PHOS-01]` | `RATIFIED-W4DR-009 (2026-05-22)` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-22` | `0.1.1` | Ratified HOLD gaps via ARCH13 decisions `W4DR-001/009`; watershed-coupled applicability and bounded-range governance adopted. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE14. |
