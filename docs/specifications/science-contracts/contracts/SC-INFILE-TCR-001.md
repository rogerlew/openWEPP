---
contract_id: SC-INFILE-TCR-001
title: Channel Critical Shear Sidecar Input Parser Contract (tcr.txt)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-TCR-001 Channel Critical Shear Sidecar Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-TCR-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md` (canonical input-surface specification and unresolved gaps).
- `[DIRECT][E-SURVEY-TCR-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and ownership context).
- `[DIRECT][E-WF-TCR-01]` `/workdir/wepp-forest/src/wshinp.for`, `/workdir/wepp-forest/src/cchrt.inc`, `/workdir/wepp-forest/src/chnrt.for` (legacy sidecar ingestion, symbols, and downstream `chntcr` use).
- `[DIRECT][E-WP-TCR-01]` `/workdir/wepppy/wepppy/nodb/core/wepp.py`, `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py` (modern sidecar emission and option ingress).
- `[INFERENCE][E-PHYS-TCR-01]` Process/common-sense invariants: critical shear parameters and curve denominator terms must remain finite and physically valid.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for optional sidecar surface `infile-channel-tcr` (`tcr.txt`) and parse-to-runtime handoff of channel critical-shear override parameters (`taumin`, `taumax`, `kch`, `nch`) that conditionally overwrite `chntcr`.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | sidecar absent | Accept optional absence branch. | `tcrflg=0`; preserve channel-file `chntcr` values. | `[DIRECT][E-WF-TCR-01]` |
| B | sidecar present with 4 numeric-leading records | Accept canonical parse path. | `tcrflg=1`; sidecar curve parameters available for `chntcr` override. | `[DIRECT][E-SPEC-TCR-01]`, `[DIRECT][E-WF-TCR-01]` |
| C | sidecar present malformed/incomplete | Strict reject. | Emit typed parse/count error; no silent fallback to missing branch. | `[INFERENCE][E-SPEC-TCR-01]` |
| D | sidecar open fails for non-ENOENT reason | Strict reject; compatibility may collapse to missing branch with warning. | Explicit strict/compat divergence required. | `[DIRECT][E-SPEC-TCR-01]`, `[DIRECT][E-WF-TCR-01]` |
| E | prefixed/datver-style variant | Provisional unsupported form. | Emit typed unsupported-format error in strict mode. | `[DIRECT][E-SPEC-TCR-01]`, `[INFERENCE][E-WF-TCR-01]` |

No datver/version line is defined for this sidecar.

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
tcr_file         = strict_tcr_file | compat_tcr_file ;
strict_tcr_file  = taumin_line taumax_line kch_line nch_line ;
compat_tcr_file  = taumin_line_compat taumax_line_compat kch_line_compat nch_line_compat ;

taumin_line      = real [trailing_tokens] ;
taumax_line      = real [trailing_tokens] ;
kch_line         = real [trailing_tokens] ;
nch_line         = real [trailing_tokens] ;

taumin_line_compat = real [trailing_tokens] ;
taumax_line_compat = real [trailing_tokens] ;
kch_line_compat    = real [trailing_tokens] ;
nch_line_compat    = real [trailing_tokens] ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves line order (`1..4`) plus line-level trailing-token provenance.
- Simulation model normalizes:
  - parsed curve parameters (`taumin`, `taumax`, `kch`, `nch`),
  - explicit cross-file dependency surfaces (`nchan`, `channel_element_ids`, `chnslp_terminal`),
  - branch/provenance state (`tcr_file_present`, `tcrflg`, `parse_outcome`),
  - derived override closure surfaces (`chntcr_override_applied`, `taumin_taumax_relational_warning_emitted`).
- Parser does not execute channel erosion kernels; it prepares validated inputs and derived state used by downstream channel-routing modules.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `taumin` | `line1.taumin` | `tcr.options.taumin_n_m2` | N/m^2 | real | 0..1 | conditional | all | none when sidecar parse succeeds | `tcr_opts.taumin` |
| `taumax` | `line2.taumax` | `tcr.options.taumax_n_m2` | N/m^2 | real | 0..1 | conditional | all | none when sidecar parse succeeds | `tcr_opts.taumax` |
| `kch` | `line3.kch` | `tcr.options.kch` | slope-domain parameter | real | 0..1 | conditional | all | none when sidecar parse succeeds | `tcr_opts.kch` |
| `nch` | `line4.nch` | `tcr.options.nch` | exponent | real | 0..1 | conditional | all | none when sidecar parse succeeds | `tcr_opts.nch` |
| derived `nchan` | external watershed topology context | `tcr.context.nchan` | count | int | 1 | yes | all | sourced from watershed/channel topology inputs | `topology.channel_count` |
| derived `channel_element_ids` | external watershed topology context | `tcr.context.channel_element_ids` | element id set | set<int> | 1 | yes | all | sourced from watershed/channel topology inputs | `topology.channel_ids` |
| derived `chnslp_terminal(i)` | external channel slope context (`chnslp(i,ncsseg(i))`) | `tcr.context.chnslp_terminal[i]` | m/m | real array | `nchan` | conditional | all | sourced from watershed channel geometry context | `channel_state.terminal_slope` |
| `chntcr` | external channel state input/output surface | `channel.state.chntcr_n_m2` | N/m^2 | real array | `nchan` | yes | all | from `.chn` when `tcrflg=0`; overwritten by sidecar curve when `tcrflg=1` | `channel_state.chntcr` |
| derived `tcr_file_present` | sidecar presence/open branch | `tcr.options.tcr_file_present` | flag | bool | 1 | yes | all | `true` on parse-success branch; `false` on missing branch; strict open-failure branch emits typed error | `tcr.sidecar_present` |
| derived `tcrflg` | sidecar enable branch | `tcr.options.tcrflg` | flag | int | 1 | yes | all | `1` when sidecar parse succeeds; `0` only explicit missing branch (or compat open-error collapse branch) | `tcr.enabled` |
| derived `parse_outcome` | parser branch outcome | `tcr.options.parse_outcome` | enum | string | 1 | yes | all | `missing_branch`, `parsed_branch`, `open_error_collapsed_compat` | `parse_outcome` |
| derived `line_count_closed` | record-count closure | `tcr.options.line_count_closed` | flag | bool | 1 | yes | all | `true` when exactly 4 required records parsed | `line_count_closed` |
| derived `trailing_token_lines` | tokenization provenance | `tcr.options.trailing_token_lines` | line-index set | list<int> | 0..4 | conditional | all | records canonical numeric-leading lines with trailing text/comments | `trailing_token_lines` |
| derived `chntcr_override_applied` | override branch closure | `tcr.options.chntcr_override_applied` | flag | bool | 1 | yes | all | `true` when `tcrflg=1` and override mapping is applied across all channel elements | `tcr.override_applied` |
| derived `taumin_taumax_relational_warning_emitted` | compatibility relational-warning branch surface | `tcr.options.taumin_taumax_relational_warning_emitted` | flag | bool | 1 | yes | all | `true` only when compatibility mode accepts `taumin>taumax` and emits warning; else `false` | `tcr.relational_warning_emitted` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `taumin` | `records.taumin` | `tcr.options.taumin_n_m2` | `input::sidecar::tcr` | init,watershed | immutable | channel critical-shear override mapping | `G-TCR-001`, `G-TCR-003` |
| `taumax` | `records.taumax` | `tcr.options.taumax_n_m2` | `input::sidecar::tcr` | init,watershed | immutable | channel critical-shear override mapping | `G-TCR-001`, `G-TCR-003` |
| `kch` | `records.kch` | `tcr.options.kch` | `input::sidecar::tcr` | init,watershed | immutable | channel critical-shear override mapping | `G-TCR-001`, `G-TCR-003`, `G-TCR-009` |
| `nch` | `records.nch` | `tcr.options.nch` | `input::sidecar::tcr` | init,watershed | immutable | channel critical-shear override mapping | `G-TCR-001`, `G-TCR-003`, `G-TCR-009` |
| derived `nchan` | `external.topology.nchan` | `tcr.context.nchan` | `input::watershed::topology` | init,watershed | immutable | override dependency closure and bounds checks | `G-TCR-005` |
| derived `channel_element_ids` | `external.topology.channel_element_ids` | `tcr.context.channel_element_ids` | `input::watershed::topology` | init,watershed | immutable | override dependency closure and ID namespace checks | `G-TCR-005` |
| derived `chnslp_terminal(i)` | `external.channel.chnslp_terminal[i]` | `tcr.context.chnslp_terminal[i]` | `input::watershed::channel` | init,watershed,event | immutable | override curve denominator and per-channel mapping | `G-TCR-005`, `G-TCR-009` |
| `chntcr` | `external.channel.chntcr` | `channel.state.chntcr_n_m2` | `input::watershed::channel` | init,watershed,event | mutable | channel routing and erosion kernels | `G-TCR-004`, `G-TCR-007` |
| derived `tcr_file_present` | `derived.file_present` | `tcr.options.tcr_file_present` | `input::sidecar::tcr` | init | immutable | parser provenance/observability | `G-TCR-006` |
| derived `tcrflg` | `derived.tcrflg` | `tcr.options.tcrflg` | `input::sidecar::tcr` | init,watershed,event | immutable | override branch gate | `G-TCR-004`, `G-TCR-006` |
| derived `parse_outcome` | `derived.parse_outcome` | `tcr.options.parse_outcome` | `input::sidecar::tcr` | init | immutable | strict/compat branch observability | `G-TCR-006` |
| derived `line_count_closed` | `derived.line_count_closed` | `tcr.options.line_count_closed` | `input::sidecar::tcr` | init | immutable | parse closure diagnostics | `G-TCR-002` |
| derived `trailing_token_lines` | `derived.trailing_token_lines` | `tcr.options.trailing_token_lines` | `input::sidecar::tcr` | init | immutable | tokenization diagnostics | `G-TCR-008` |
| derived `chntcr_override_applied` | `derived.chntcr_override_applied` | `tcr.options.chntcr_override_applied` | `runtime::watershed::channel_routing` | init,watershed | immutable | override closure diagnostics | `G-TCR-004`, `G-TCR-005` |
| derived `taumin_taumax_relational_warning_emitted` | `derived.taumin_taumax_relational_warning_emitted` | `tcr.options.taumin_taumax_relational_warning_emitted` | `input::sidecar::tcr` | init | immutable | compatibility warning observability | `G-TCR-003` |

## 5. State Ownership and Mutability

- `input::sidecar::tcr` owns sidecar-record and parser-provenance state surfaces.
- Parsed sidecar fields are immutable after parse finalization.
- `input::watershed::channel`/`runtime::watershed::channel_routing` own mutable per-channel runtime state such as `channel.state.chntcr_n_m2`.
- Forbidden mutation path: non-owner modules mutating parsed sidecar fields (`taumin`, `taumax`, `kch`, `nch`, `tcrflg`) after parser finalization.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-TCR-001` | Derive sidecar presence/enable branch (`tcr_file_present`, `tcrflg`). | parse preamble/finalize | `C-TCR-001` |
| `D-TCR-002` | Derive parse-outcome branch marker and line-count closure. | parse finalize | `C-TCR-002` |
| `D-TCR-003` | Derive trailing-token provenance lines. | parse finalize | `C-TCR-003` |
| `D-TCR-004` | Derive sidecar override application closure across channel elements (`chntcr_override_applied`). | watershed init | `C-TCR-004` |
| `D-TCR-005` | Derive compatibility relational-warning emission surface for `taumin>taumax` branch. | parse finalize | `C-TCR-005` |

Closure hooks:
- `C-TCR-001`: `tcrflg` must be deterministic and branch-consistent.
- `C-TCR-002`: present-file parse must close at exactly four required records.
- `C-TCR-003`: tokenization behavior must preserve line-level provenance.
- `C-TCR-004`: override branch must be all-or-nothing across eligible channel elements.
- `C-TCR-005`: compatibility relational-warning behavior for `taumin>taumax` must be explicitly emitted and observable.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `TCR-E-000` | io | sidecar open/read failure in strict mode (non-ENOENT branch) |
| `TCR-E-001` | syntax | numeric parse failure on required line |
| `TCR-E-002` | syntax | record-count mismatch (expected 4 required records) |
| `TCR-E-003` | semantic | non-finite sidecar values |
| `TCR-E-004` | semantic | domain violation (`taumin<0`, `taumax<0`, `kch<=0`, `nch<=0`) |
| `TCR-E-009` | semantic | relational invariant violation (`taumin>taumax`) in strict mode |
| `TCR-E-005` | cross-file | missing/incompatible downstream channel surfaces needed for override closure |
| `TCR-E-006` | runtime-guard | `tcrflg`/override closure mismatch (`chntcr_override_applied` inconsistent) |
| `TCR-E-007` | syntax | unsupported prefixed/datver-like variant |
| `TCR-E-008` | runtime-guard | override denominator/curve-domain degeneracy (`kch^nch + slope^nch <= 0`) |
| `TCR-W-001` | compat-warning | optional sidecar absence branch taken (`tcrflg=0`) |
| `TCR-W-002` | compat-warning | open-error branch collapsed with missing branch |
| `TCR-W-003` | compat-warning | relational invariant warning for `taumin>taumax` with legacy-flow preservation |

No silent parser-side fallback is permitted in strict mode for malformed present-file input.

## 8. Cross-File Consistency Constraints

1. `tcr.txt` applicability is watershed-side and must stay coupled to channel-file routing context and channel-state availability. `[DIRECT][E-WF-TCR-01]`
2. `chntcr` namespace continuity must preserve channel-file baseline values when `tcrflg=0`. `[DIRECT][E-SPEC-TCR-01]`, `[DIRECT][E-WF-TCR-01]`
3. Sidecar override mapping requires explicit valid cross-file dependency surfaces (`nchan`, `channel_element_ids`, `chnslp_terminal`) for all overridden channel elements. `[DIRECT][E-WF-TCR-01]`, `[INFERENCE][E-PHYS-TCR-01]`
4. Orchestrator-side aliases (`tcr_opts_*`) must map losslessly to canonical symbols and units. `[DIRECT][E-WP-TCR-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `taumin,taumax,kch,nch` | `tcr.options.*` | `openwepp.boundary.parser.tcr.v1.records` | canonical names + `tcr_opts.*` aliases | unit-preserving parser export |
| `tcr_file_present,tcrflg,parse_outcome` | `tcr.options.{tcr_file_present,tcrflg,parse_outcome}` | `openwepp.boundary.mode_selection.tcr.v1` | explicit branch/provenance fields | strict/compat branch observability |
| `line_count_closed,trailing_token_lines,taumin_taumax_relational_warning_emitted` | `tcr.options.{line_count_closed,trailing_token_lines,taumin_taumax_relational_warning_emitted}` | `openwepp.boundary.observability.parser_warnings.v1` | explicit closure/tokenization/compat-warning diagnostics | parser diagnostics |
| `nchan,channel_element_ids,chnslp_terminal(i)` | `tcr.context.{nchan,channel_element_ids,chnslp_terminal}` | `openwepp.boundary.crossfile.tcr_dependencies.v1` | explicit cross-file dependency surfaces | supports executable `G-TCR-005`/`G-TCR-009` closure |
| `chntcr,chntcr_override_applied` | `channel.state.chntcr_n_m2`, `tcr.options.chntcr_override_applied` | `openwepp.boundary.routing.channel_critical_shear.v1` | canonical state + override closure field | downstream kernel-facing state export |

## 10. Compatibility Policy

- Strict mode:
  - rejects malformed present-file payloads with typed errors;
  - rejects non-ENOENT open failures as typed IO errors;
  - enforces sidecar field-domain and relational invariants;
  - rejects unsupported prefixed/datver variants.
- Compatibility mode:
  - allows sidecar absence branch with explicit warning/provenance (`TCR-W-001`);
  - may collapse open errors with missing branch and emit `TCR-W-002`;
  - emits `TCR-W-003` and preserves legacy value flow when `taumin>taumax`;
  - accepts canonical numeric-leading records with optional trailing text/comments;
  - preserves canonical symbol mapping and override branch semantics.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-TCR-001` | required numeric parse for `taumin,taumax,kch,nch` | parse lines 1..4 | `TCR-E-001`/`TCR-E-003` |
| `G-TCR-002` | present-file record-count closure (`==4`) | parse finalize | `TCR-E-002` |
| `G-TCR-003` | sidecar domain and relational invariants | parse finalize/policy gate | strict: `TCR-E-004`/`TCR-E-009`; compat relational branch: `TCR-W-003` |
| `G-TCR-004` | deterministic override branch closure (`tcrflg` -> `chntcr` mapping behavior) | watershed init/branch validator | `TCR-E-006` |
| `G-TCR-005` | override requires complete channel-slope/state surfaces | cross-file validator | `TCR-E-005` |
| `G-TCR-006` | strict/compat open-branch policy closure | preamble open handler | strict: `TCR-E-000`; compat: `TCR-W-001`/`TCR-W-002` |
| `G-TCR-007` | `chntcr` namespace continuity across `.chn` baseline and sidecar override | cross-file validator | `TCR-E-005`/`TCR-E-006` |
| `G-TCR-008` | canonical numeric-leading tokenization policy | parse finalize/policy gate | `TCR-E-001` |
| `G-TCR-009` | override curve denominator positivity | derived mapping validator | `TCR-E-008` |
| `G-TCR-010` | prefixed/datver-style variant rejection | parse preamble | `TCR-E-007` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`taumin`, `taumax`, `kch`, `nch`, `tcrflg`, `chntcr`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `TCR-GAP-001` | `usersum2024` sidecar section does not publish a dedicated `tcr.txt` format table. | `[DIRECT][E-SPEC-TCR-01]` | `HOLD` |
| `TCR-GAP-002` | Legacy open-failure collapse behavior requires governance-ratified strict/compat policy closure. | `[DIRECT][E-SPEC-TCR-01]`, `[DIRECT][E-WF-TCR-01]` | `HOLD` |
| `TCR-GAP-003` | Curve-domain guard severity for legacy-compatible flows needs fixture-backed closure. | `[DIRECT][E-SPEC-TCR-01]`, `[INFERENCE][E-PHYS-TCR-01]` | `HOLD` |
| `TCR-GAP-004` | Bounds/default divergence between legacy path and modern producer policy remains unresolved for all four sidecar fields. | `[DIRECT][E-SPEC-TCR-01]`, `[DIRECT][E-WP-TCR-01]` | `HOLD` |
| `TCR-GAP-005` | Producer-side blank/newline `tcr.txt` edge-case behavior requires explicit cross-repo disposition. | `[DIRECT][E-SPEC-TCR-01]`, `[DIRECT][E-WP-TCR-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE17. |
