---
contract_id: SC-INFILE-CHANINP-001
title: Channel Routing Options Input Parser Contract (chan.inp)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.2
evidence_mode: Static
last_updated_utc: 2026-07-09T00:00:00Z
---

# SC-INFILE-CHANINP-001 Channel Routing Options Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-CHN-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md` (canonical `chan.inp` specification and unresolved gaps).
- `[DIRECT][E-SURVEY-CHN-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and ownership context).
- `[DIRECT][E-WF-CHN-01]` `/workdir/wepp-forest_260430_baseline/src/wshinp.for`, `/workdir/wepp-forest_260430_baseline/src/cchrt.inc`, `/workdir/wepp-forest_260430_baseline/src/pmxchr.inc`, `/workdir/wepp-forest_260430_baseline/src/chnrt.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` (legacy parse path, symbols, clamps/normalization, and downstream consumption).
- `[DIRECT][E-WP-CHN-01]` `/workdir/wepppy/wepppy/nodb/core/wepp.py`, `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py` (modern writer/input parser constraints and alias surfaces).
- `[INFERENCE][E-PHYS-CHN-01]` Process/common-sense invariants: timestep/count fields must remain finite and physically meaningful; selected channel IDs must close against loaded watershed topology.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for `chan.inp` (`infile-channel-contrast`) and parse-to-runtime handoff of channel-routing option fields (`ichout`, `dtchr`, `cbase`, `nchnum`, `ichnum[]`) when updated routing methods are active.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | `ipeak <= 2` (surface not applicable) | Accept as non-applicable branch. | Emit explicit `surface_not_applicable` outcome; no parser error. | `[DIRECT][E-SPEC-CHN-01]`, `[DIRECT][E-WF-CHN-01]` |
| B | `ipeak > 2` and full 4-line canonical payload | Accept canonical parse path. | Parse fields and apply deterministic normalization/closure rules. | `[DIRECT][E-SPEC-CHN-01]`, `[DIRECT][E-WF-CHN-01]` |
| C | `ipeak > 2` and file missing | Strict reject; compatibility may take legacy-default branch. | strict typed missing-surface error vs compat default/warning branch. | `[DIRECT][E-SPEC-CHN-01]`, `[DIRECT][E-WF-CHN-01]` |
| D | `ipeak > 2` and open fails (non-ENOENT) | Strict reject; compatibility may collapse to default branch. | Explicit strict/compat divergence required. | `[DIRECT][E-SPEC-CHN-01]`, `[DIRECT][E-WF-CHN-01]` |
| E | malformed/truncated payload | Strict reject. | Emit typed parse/count errors; no silent default in strict mode. | `[INFERENCE][E-SPEC-CHN-01]` |
| F | prefixed/datver-style variant | Unsupported by legacy positional parser design. | Reject as unsupported format in strict mode. | `[DIRECT][E-SPEC-CHN-01]`, `[INFERENCE][E-WF-CHN-01]` |

No datver/version line is defined for this surface.

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
chaninp_file           = strict_chaninp_file | compat_chaninp_file ;
strict_chaninp_file    = line1 line2 line3 line4 ;
compat_chaninp_file    = line1_compat line2_compat line3_compat line4_compat ;

line1                  = ichout dtchr [trailing_tokens] ;
line2                  = cbase [trailing_tokens] ;
line3                  = nchnum [trailing_tokens] ;
line4                  = ichnum { whitespace ichnum } [trailing_tokens] ;

line1_compat           = ichout dtchr [trailing_tokens] ;
line2_compat           = cbase [trailing_tokens] ;
line3_compat           = nchnum [trailing_tokens] ;
line4_compat           = ichnum { whitespace ichnum } [trailing_tokens] ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves raw parsed fields and line-level tokenization provenance.
- Simulation model normalizes:
  - applicability/branch state (`chaninp_required`, `parse_outcome`),
  - canonical routing fields,
  - normalized/clamped runtime fields (`dtchr_norm_s`, `ntchr`, `nchnum_norm`, `ichnum_norm`),
  - downstream output-selection closure (`chan_output_enabled`).
- Parser does not execute channel routing; downstream modules consume normalized fields.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `ipeak` | external `.chn` routing-mode field | `chaninp.context.ipeak` | enum/int | int | 1 | yes | all | sourced from channel-file parser context | `channel.routing_method_id` |
| `ichout` | `line1.ichout` | `chaninp.options.ichout` | mode enum | int | 0..1 | conditional (`ipeak>2`) | all | default `0` only in compat default branch | `channel_output_mode` |
| `dtchr` | `line1.dtchr` | `chaninp.options.dtchr_input_s` | s | real | 0..1 | conditional (`ipeak>2`) | all | default `60` only in compat default branch | `channel_routing_timestep_input_s` |
| `cbase` | `line2.cbase` | `chaninp.options.cbase_m3_s_m2` | m^3/s/m^2 | real | 0..1 | conditional (`ipeak>2`) | all | legacy default `0.0` only in compat default branch | `unit_area_baseflow_coefficient` |
| `nchnum` | `line3.nchnum` | `chaninp.options.nchnum_input` | count | int | 0..1 | conditional (`ipeak>2`) | all | default `0` only in compat default branch | `channel_output_count_input` |
| `ichnum(i)` | `line4.ichnum[i]` | `chaninp.options.ichnum_input[i]` | element id | int array | `nchnum` | conditional (`ipeak>2`, `nchnum>0`) | all | empty only in compat default branch where `nchnum=0` | `channel_output_element_ids_input` |
| derived `nchan` | external watershed topology context | `chaninp.context.nchan` | count | int | 1 | yes | all | sourced from watershed/channel topology inputs | `topology.channel_count` |
| derived `valid_channel_element_ids` | external watershed topology ID namespace | `chaninp.context.valid_channel_element_ids` | element id set | set<int> | 1 | yes | all | sourced from watershed structure/channel topology surfaces | `topology.valid_channel_ids` |
| derived `chaninp_required` | applicability branch | `chaninp.context.chaninp_required` | flag | bool | 1 | yes | all | `true` when `ipeak>2`; else `false` | `chaninp_required` |
| derived `parse_outcome` | parser branch outcome | `chaninp.context.parse_outcome` | enum | string | 1 | yes | all | `not_applicable`, `parsed_branch`, `defaulted_compat`, `open_error_collapsed_compat` | `parse_outcome` |
| derived `dtchr_norm_s` | normalized timestep | `chaninp.options.dtchr_norm_s` | s | int | 0..1 | conditional (`ipeak>2`) | all | bounded/renormalized from `dtchr` and `ntchr` closure rules | `channel_routing_timestep_s` |
| derived `ntchr` | routing steps per day | `chaninp.options.ntchr` | count/day | int | 0..1 | conditional (`ipeak>2`) | all | `min(mxtchr, floor(86400/dtchr_bound + 0.99))` | `channel_routing_steps_per_day` |
| derived `nchnum_norm` | normalized count | `chaninp.options.nchnum_norm` | count | int | 0..1 | conditional (`ipeak>2`) | all | strict exact value or compat clamped to `[0,nchan]` | `channel_output_count` |
| derived `ichnum_norm(i)` | normalized element IDs | `chaninp.options.ichnum_norm[i]` | element id | int array | `nchnum_norm` | conditional (`ipeak>2`, `nchnum_norm>0`) | all | strict validated against topology; compat retains with warning on unknown IDs | `channel_output_element_ids` |
| derived `line_count_closed` | record-count closure | `chaninp.context.line_count_closed` | flag | bool | 1 | yes | all | `true` when required line-set is complete | `line_count_closed` |
| derived `trailing_token_lines` | tokenization provenance | `chaninp.context.trailing_token_lines` | line-index set | list<int> | 0..4 | conditional | all | line-level provenance for canonical numeric-leading records | `trailing_token_lines` |
| derived `chan_output_enabled` | output gate closure | `chaninp.options.chan_output_enabled` | flag | bool | 1 | yes | all | `true` when `ichout>0 && nchnum_norm>0` | `channel_output_enabled` |
| derived `unknown_ichnum_retained_warning_emitted` | compatibility topology-warning surface | `chaninp.context.unknown_ichnum_retained_warning_emitted` | flag | bool | 1 | yes | all | `true` when compatibility mode retains unknown `ichnum` IDs and emits warning | `chaninp.unknown_id_warning_emitted` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ipeak` | `external.chn.ipeak` | `chaninp.context.ipeak` | `input::watershed::chn` | init,watershed | immutable | applicability gate for `chan.inp` parsing | `G-CHN-001` |
| `ichout` | `records.ichout` | `chaninp.options.ichout` | `input::sidecar::chaninp` | init,watershed | immutable | channel-output mode selection | `G-CHN-002`, `G-CHN-004` |
| `dtchr` | `records.dtchr` | `chaninp.options.dtchr_input_s` | `input::sidecar::chaninp` | init,watershed | immutable | timestep normalization and routing scheduler | `G-CHN-002`, `G-CHN-005` |
| `cbase` | `records.cbase` | `chaninp.options.cbase_m3_s_m2` | `input::sidecar::chaninp` | init,watershed,daily,event | immutable | baseflow routing parameter surfaces | `G-CHN-002`, `G-CHN-006` |
| `nchnum` | `records.nchnum` | `chaninp.options.nchnum_input` | `input::sidecar::chaninp` | init,watershed | immutable | channel output ID-list closure | `G-CHN-002`, `G-CHN-007` |
| `ichnum(i)` | `records.ichnum_input[i]` | `chaninp.options.ichnum_input[i]` | `input::sidecar::chaninp` | init,watershed,event | immutable | per-channel output selection matching | `G-CHN-003`, `G-CHN-008` |
| derived `nchan` | `external.topology.nchan` | `chaninp.context.nchan` | `input::watershed::topology` | init,watershed | immutable | topology-count closure for `nchnum` checks | `G-CHN-007` |
| derived `valid_channel_element_ids` | `external.topology.valid_channel_element_ids` | `chaninp.context.valid_channel_element_ids` | `input::watershed::topology` | init,watershed | immutable | topology ID-namespace closure for `ichnum` checks | `G-CHN-008` |
| derived `chaninp_required` | `derived.chaninp_required` | `chaninp.context.chaninp_required` | `input::sidecar::chaninp` | init | immutable | applicability observability | `G-CHN-001` |
| derived `parse_outcome` | `derived.parse_outcome` | `chaninp.context.parse_outcome` | `input::sidecar::chaninp` | init | immutable | strict/compat branch observability | `G-CHN-001`, `G-CHN-010` |
| derived `dtchr_norm_s` | `derived.dtchr_norm_s` | `chaninp.options.dtchr_norm_s` | `input::sidecar::chaninp` | init,watershed,event | immutable | runtime channel routing step size | `G-CHN-005` |
| derived `ntchr` | `derived.ntchr` | `chaninp.options.ntchr` | `input::sidecar::chaninp` | init,watershed,event | immutable | daily/event routing loop scheduling | `G-CHN-005` |
| derived `nchnum_norm` | `derived.nchnum_norm` | `chaninp.options.nchnum_norm` | `input::sidecar::chaninp` | init,watershed | immutable | output list cardinality closure | `G-CHN-007` |
| derived `ichnum_norm(i)` | `derived.ichnum_norm[i]` | `chaninp.options.ichnum_norm[i]` | `input::sidecar::chaninp` | init,watershed,event | immutable | runtime channel-ID output matching | `G-CHN-008` |
| derived `line_count_closed` | `derived.line_count_closed` | `chaninp.context.line_count_closed` | `input::sidecar::chaninp` | init | immutable | parse closure diagnostics | `G-CHN-003` |
| derived `trailing_token_lines` | `derived.trailing_token_lines` | `chaninp.context.trailing_token_lines` | `input::sidecar::chaninp` | init | immutable | tokenization diagnostics | `G-CHN-009` |
| derived `unknown_ichnum_retained_warning_emitted` | `derived.unknown_ichnum_retained_warning_emitted` | `chaninp.context.unknown_ichnum_retained_warning_emitted` | `input::sidecar::chaninp` | init | immutable | compatibility warning observability for unknown-ID retention branch | `G-CHN-008`, `G-CHN-010` |
| derived `chan_output_enabled` | `derived.chan_output_enabled` | `chaninp.options.chan_output_enabled` | `runtime::watershed::channel_outputs` | init,watershed,event | immutable | `chan.out`/`chanwb.out` gate closure | `G-CHN-004`, `G-CHN-011` |

## 5. State Ownership and Mutability

- `input::sidecar::chaninp` owns parsed raw fields and normalized option/provenance surfaces.
- Parser-owned `chaninp.options.*` and `chaninp.context.*` fields are immutable after parse finalization.
- `runtime::watershed::channel_outputs` owns mutable output-file lifecycle state but must not mutate parser-owned option fields.
- Forbidden mutation path: ad-hoc runtime rewriting of `ichout`, `dtchr_norm_s`, `nchnum_norm`, or `ichnum_norm` after parser finalization.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-CHN-001` | Derive applicability/branch surfaces (`chaninp_required`, `parse_outcome`) from `ipeak` and strict/compat branch outcomes. | parse preamble/finalize | `C-CHN-001` |
| `D-CHN-002` | Derive timestep normalization (`dtchr_norm_s`, `ntchr`) from legacy-bound normalization algorithm. | parse finalize | `C-CHN-002` |
| `D-CHN-003` | Derive normalized channel-count/list surfaces (`nchnum_norm`, `ichnum_norm`) from topology and policy gates. | parse/cross-file finalize | `C-CHN-003` |
| `D-CHN-004` | Derive line/tokenization closure surfaces (`line_count_closed`, `trailing_token_lines`). | parse finalize | `C-CHN-004` |
| `D-CHN-005` | Derive channel-output enable closure (`chan_output_enabled`). | parse/runtime finalize | `C-CHN-005` |
| `D-CHN-006` | Derive the WSHED-W10 compatibility default branch from pinned legacy open/read error behavior: `ichout=0`, `nchnum=0`, `ichnum=[]`, `cbase=0`, `dtchr_input_s=60`, `ntchr=1440`, `dtchr_norm_s=60`, `chan_output_enabled=false`. | compat default finalize | `C-CHN-006` |

Closure hooks:
- `C-CHN-001`: applicability and branch outcome must be deterministic.
- `C-CHN-002`: normalized timestep and step-count must satisfy bounds/closure.
- `C-CHN-003`: channel-count/list closure must be topology-consistent.
- `C-CHN-004`: required-line and tokenization closure must be explicitly observable.
- `C-CHN-005`: output-enable gate must close against normalized mode/count fields.
- `C-CHN-006`: defaulted compatibility output must be explicit and runtime-ready; watershed runtime must not replace it with an untyped `None` branch.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `CHN-E-000` | io | open failure in strict mode when `chan.inp` is required (`ipeak>2`) |
| `CHN-E-009` | io | missing required `chan.inp` surface in strict mode when `ipeak>2` |
| `CHN-E-001` | syntax | numeric parse failure on required fields |
| `CHN-E-002` | syntax | required line/cardinality mismatch (including line4 list arity vs `nchnum`) |
| `CHN-E-003` | semantic | non-finite numeric values |
| `CHN-E-004` | semantic | invalid field range/domain (`ichout` domain, timestep domain, negative counts) |
| `CHN-E-005` | cross-file | topology consistency failure (`nchnum`/`ichnum` vs `nchan`/valid structure IDs) |
| `CHN-E-006` | runtime-guard | normalized-timestep closure mismatch (`dtchr_norm_s`/`ntchr`) |
| `CHN-E-007` | runtime-guard | output-enable closure mismatch (`chan_output_enabled`) |
| `CHN-E-008` | syntax | unsupported prefixed/datver-style variant |
| `CHN-W-001` | compat-warning | required-surface missing/default branch applied in compatibility mode |
| `CHN-W-002` | compat-warning | open-error branch collapsed with default branch |
| `CHN-W-003` | compat-warning | parse/count failure collapsed to default branch |
| `CHN-W-004` | compat-warning | clamping/normalization applied for out-of-range fields |
| `CHN-W-005` | compat-warning | unknown `ichnum` IDs retained in compatibility mode with explicit topology-warning emission |

No silent fallback/default masking is permitted in strict mode for required `chan.inp` branches.

## 7a. WSHED-W10 Compatibility Default Branch

For `ipeak > 2`, pinned legacy `wshinp.for` initializes `cbase=0` before
opening `chan.inp`; if the open or first required read fails, the error-label
path sets `ichout=0`, then sets `nchnum=0`, and continues through the common
timestep normalization block. Because the legacy branch can reach that block
without a freshly read `dtchr`, openWEPP fixes the compatibility default to the
deterministic lower-bound/mxtchr normalization already used by the parser:
`dtchr_input_s=60`, `ntchr=1440`, `dtchr_norm_s=60`.

The complete defaulted compatibility state is:

| Field | Required default |
| --- | --- |
| `parse_outcome` | `defaulted_compat` or `open_error_collapsed_compat` |
| `ichout` | `0` |
| `dtchr_input_s` | `60` |
| `dtchr_norm_s` | `60` |
| `ntchr` | `1440` |
| `cbase_m3_s_m2` | `0.0` |
| `nchnum_input` / `nchnum_norm` | `0` / `0` |
| `ichnum_input` / `ichnum_norm` | empty / empty |
| `chan_output_enabled` | `false` |

Watershed runtime may consume this typed defaulted state directly. It must not
substitute separate hardcoded routing globals from an absent optional
`chan.inp` object.

## 8. Cross-File Consistency Constraints

1. `chan.inp` applicability is controlled by `.chn` routing method (`ipeak`) and must be explicit in parser outcomes. `[DIRECT][E-SPEC-CHN-01]`, `[DIRECT][E-WF-CHN-01]`
2. `ichnum` ID space must close against watershed structure/channel topology IDs and `nchan`. `[DIRECT][E-SPEC-CHN-01]`, `[DIRECT][E-WF-CHN-01]`
3. `chan_output_enabled` must be consistent with normalized `ichout` and normalized `nchnum` surfaces. `[DIRECT][E-WF-CHN-01]`
4. Alias surfaces written by modern orchestration (`ichout_override`, `dtchr_override`, `chn_topaz_ids_of_interest`) must map losslessly to canonical symbols. `[DIRECT][E-WP-CHN-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `ichout,dtchr,cbase,nchnum,ichnum(i)` | `chaninp.options.{ichout,dtchr_input_s,cbase_m3_s_m2,nchnum_input,ichnum_input}` | `openwepp.boundary.parser.chaninp.v1.records` | canonical names + writer-facing aliases | raw parse export |
| `dtchr_norm_s,ntchr,nchnum_norm,ichnum_norm(i)` | `chaninp.options.*` | `openwepp.boundary.parser.chaninp.v1.normalized` | normalized/clamped fields + closure flags | runtime-facing normalized export |
| `chaninp_required,parse_outcome,line_count_closed,trailing_token_lines,unknown_ichnum_retained_warning_emitted` | `chaninp.context.*` | `openwepp.boundary.observability.parser_warnings.v1` | explicit branch/closure/topology-warning diagnostics | strict/compat observability |
| `nchan,valid_channel_element_ids` | `chaninp.context.{nchan,valid_channel_element_ids}` | `openwepp.boundary.crossfile.chaninp_topology.v1` | explicit topology dependency surfaces | executable closure support for `G-CHN-007`/`G-CHN-008` |
| `chan_output_enabled` | `chaninp.options.chan_output_enabled` | `openwepp.boundary.outputs.channel_gate.v1` | output-enable closure field | output subsystem contract gate |

## 10. Compatibility Policy

- Strict mode:
  - when `ipeak<=2`, emits non-applicable branch (not an error);
  - when `ipeak>2`, treats `chan.inp` as required and rejects missing/open/parse failures;
  - enforces field-domain, timestep-closure, and topology-ID invariants;
  - rejects unsupported prefixed/datver-style variants.
- Compatibility mode:
  - preserves legacy-default/clamp behavior for missing/open/parse failures with explicit warnings;
  - preserves explicit missing-vs-open-failure warning distinction (`CHN-W-001` vs `CHN-W-002`);
  - allows legacy normalization/clamping outcomes with explicit closure surfaces;
  - allows unknown `ichnum` retention with explicit warning/provenance (`CHN-W-005`);
  - preserves canonical numeric-leading line handling with trailing-text provenance capture.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-CHN-001` | deterministic applicability/requiredness branch from `ipeak` | parse preamble | strict missing surface: `CHN-E-009`; strict open failure: `CHN-E-000`; strict unsupported format: `CHN-E-008`; compat: `CHN-W-001`/`CHN-W-002` |
| `G-CHN-002` | required numeric parse for line1..line3 fields | parse lines 1..3 | `CHN-E-001`/`CHN-E-003` |
| `G-CHN-003` | line4 list arity and record closure (`nchnum`-bound) | parse line4/finalize | `CHN-E-002` |
| `G-CHN-004` | `ichout` domain and output-gate semantic closure | parse/runtime validator | `CHN-E-004`/`CHN-E-007` |
| `G-CHN-005` | timestep normalization bounds/closure (`dtchr_norm_s`, `ntchr`) | parse normalize validator | `CHN-E-004`/`CHN-E-006` |
| `G-CHN-006` | `cbase` finite/domain semantics | parse finalize | `CHN-E-003`/`CHN-E-004` |
| `G-CHN-007` | `nchnum` bounds against topology size (`nchan`) | cross-file validator | `CHN-E-004`/`CHN-E-005` |
| `G-CHN-008` | `ichnum(i)` topology-ID closure | cross-file validator | strict: `CHN-E-005`; compat retain branch: `CHN-W-005` |
| `G-CHN-009` | canonical numeric-leading tokenization policy | parse finalize/policy gate | `CHN-E-001` |
| `G-CHN-010` | strict/compat default-branch observability closure | parse finalize | `CHN-E-007`/`CHN-W-001`/`CHN-W-003`/`CHN-W-004` |
| `G-CHN-011` | output-enable closure (`ichout>0 && nchnum_norm>0`) | runtime output gate validator | `CHN-E-007` |
| `G-CHN-012` | WSHED-W10 compatibility default branch is typed and runtime-ready; no untyped `None` fallback supplies `dtchr`, `ntchr`, `nchnum`, or `cbase` | parser default finalize + watershed frame intake | `CHN-E-007` or watershed runtime hard error |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`ichout`, `dtchr`, `cbase`, `nchnum`, `ichnum`, `ipeak`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `CHANINP-GAP-001` | Practical downstream semantics for `cbase` remain under-evidenced relative to usersum guidance. | `[DIRECT][E-SPEC-CHN-01]` | `RATIFIED-W4DR-006 (2026-05-22)` |
| `CHANINP-GAP-002` | Legacy error-path initialization/normalization semantics around `dtchr` require fixture-backed closure. | `[DIRECT][E-SPEC-CHN-01]`, `[DIRECT][E-WF-CHN-01]` | `RATIFIED-W4DR-005 (2026-05-22)` |
| `CHANINP-GAP-003` | `wepppy` `ichout` override domain (`{1,3}`) diverges from usersum/legacy domain (`0..3`) and requires governance closure. | `[DIRECT][E-SPEC-CHN-01]`, `[DIRECT][E-WP-CHN-01]` | `RATIFIED-W4DR-004 (2026-05-22)` |
| `CHANINP-GAP-004` | Program-level ownership for chaninp parsing vs interchange-only crates requires explicit ratification. | `[DIRECT][E-SPEC-CHN-01]`, `[DIRECT][E-SURVEY-CHN-01]` | `RATIFIED-W4DR-003 (2026-05-22)` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-07-09` | `0.1.2` | WSHED-W10 amendment: ratified explicit compatibility default values for absent/open-error/malformed `chan.inp`, required runtime consumption of typed defaulted parser state, and added `D-CHN-006` / `G-CHN-012`. |
| `2026-05-22` | `0.1.1` | Ratified HOLD gaps via ARCH13 decisions `W4DR-003/004/005/006`; kickoff HOLD removed for this contract surface. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE19. |
