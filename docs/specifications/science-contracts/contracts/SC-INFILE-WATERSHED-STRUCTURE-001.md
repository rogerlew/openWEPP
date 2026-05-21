---
contract_id: SC-INFILE-WATERSHED-STRUCTURE-001
title: Watershed Structure Input Parser Contract (.str)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.1
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-WATERSHED-STRUCTURE-001 Watershed Structure Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-STR-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-structure-file.spec.md` (canonical openWEPP `.str` grammar, fields, cross-file closure rules, and HOLD gaps).
- `[DIRECT][E-SURVEY-STR-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (`.str` parser surface provenance and ownership notes).
- `[DIRECT][E-WF-STR-01]` `/workdir/wepp-forest/src/infile.for`, `/workdir/wepp-forest/src/wshinp.for`, `/workdir/wepp-forest/src/wshini.for`, `/workdir/wepp-forest/src/inidat.for` (legacy version gate, row reads, and topology/closure checks reflected in spec).
- `[DIRECT][E-WP-STR-01]` `/workdir/wepppy/wepppy/nodb/core/wepp.py` (current `.str` writer behavior cited by the spec).
- `[INFERENCE][E-PHYS-STR-01]` Topology/common-sense invariants: each downstream element must have at least one upstream contributor and graph references must resolve to valid prior elements.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for surface `infile-watershed-structure-str` (`.str`) and parse-to-runtime topology handoff for watershed channel/impoundment connectivity.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | explicit `datver > 10` | Accept and version-check. | Use canonical modern parse path. | `[DIRECT][E-SPEC-STR-01]` |
| B | first token `<= 10` legacy no-datver form | Strict reject. Compat optional with explicit flag. | Emit compatibility warning when accepted. | `[DIRECT][E-SPEC-STR-01]`, `[DIRECT][E-WF-STR-01]` |
| C | explicit `datver >= 94.301` | Accept. | Canonical compatibility floor per legacy constant. | `[DIRECT][E-SPEC-STR-01]`, `[DIRECT][E-WF-STR-01]` |
| D | explicit `datver < 94.301` | Reject. | Emit typed `UnsupportedDatver`. | `[DIRECT][E-WF-STR-01]` |

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
str_file = [datver_line] structure_row{n_rows} ;

datver_line = real ;
structure_row = elmt nhleft nhrght nhtop ncleft ncrght nctop nileft nirght nitop ;
```

`n_rows` is resolved from watershed topology authority (`nchan + npond`) during cross-file closure.
Strict-mode parse requires exact logical record closure: one optional `datver` record plus exactly `n_rows` structure rows (no deficit/surplus rows).

### 2.2 Two-Layer Model Contract

- Source model is file-faithful and preserves each 10-integer row in encountered order.
- Simulation model normalizes rows into typed downstream-element topology records with explicit `element_id`, `element_type`, and contributor sets.
- Parser performs structural decoding only; full topology closure requiring `.chn/.man/.imp` is a post-parse cross-file validation phase.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `watershed.structure.version.datver` | none | real | 0..1 | conditional | see Section 1 | none | `structure_version` |
| `nhill` (external authority) | `topology_context.nhill` | `watershed.structure.context.nhill` | count | int | 1 | yes | all | provided by watershed topology authority | `hillslope_count` |
| `elmt` | `rows[r].elmt` | `watershed.structure.rows[r].element_type_code` | code | int | n_rows | yes | all | none | `element_type_code` |
| `nhleft` | `rows[r].nhleft` | `watershed.structure.rows[r].hillslope_left_id` | id | int | n_rows | yes | all | none | `hillslope_left_id` |
| `nhrght` | `rows[r].nhrght` | `watershed.structure.rows[r].hillslope_right_id` | id | int | n_rows | yes | all | none | `hillslope_right_id` |
| `nhtop` | `rows[r].nhtop` | `watershed.structure.rows[r].hillslope_top_id` | id | int | n_rows | yes | all | none | `hillslope_top_id` |
| `ncleft` | `rows[r].ncleft` | `watershed.structure.rows[r].channel_left_id` | id | int | n_rows | yes | all | none | `channel_left_id` |
| `ncrght` | `rows[r].ncrght` | `watershed.structure.rows[r].channel_right_id` | id | int | n_rows | yes | all | none | `channel_right_id` |
| `nctop` | `rows[r].nctop` | `watershed.structure.rows[r].channel_top_id` | id | int | n_rows | yes | all | none | `channel_top_id` |
| `nileft` | `rows[r].nileft` | `watershed.structure.rows[r].impoundment_left_id` | id | int | n_rows | yes | all | none | `impoundment_left_id` |
| `nirght` | `rows[r].nirght` | `watershed.structure.rows[r].impoundment_right_id` | id | int | n_rows | yes | all | none | `impoundment_right_id` |
| `nitop` | `rows[r].nitop` | `watershed.structure.rows[r].impoundment_top_id` | id | int | n_rows | yes | all | none | `impoundment_top_id` |
| derived row index | `rows[r].position` | `watershed.structure.rows[r].record_index` | index | int | n_rows | yes | all | `1..n_rows` by file order | `record_index` |
| derived `element_id` | `rows[r] + nhill` | `watershed.structure.rows[r].element_id` | id | int | n_rows | yes | all | `nhill + record_index` | `element_id` |
| derived `nchan` | `count(elmt==2)` | `watershed.structure.summary.channel_count` | count | int | 1 | yes | all | derived from rows | `channel_count` |
| derived `npond` | `count(elmt==3)` | `watershed.structure.summary.impoundment_count` | count | int | 1 | yes | all | derived from rows | `impoundment_count` |
| derived `nhmax` | `max(nhleft,nhrght,nhtop)` | `watershed.structure.summary.max_hillslope_ref` | id | int | 1 | yes | all | derived from hillslope contributor fields | `max_hillslope_ref` |
| derived `idelmt` | dense local index | `watershed.structure.rows[r].element_local_index` | index | int | n_rows | yes | all | derived by `elmt` class order | `element_local_index` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `watershed.structure.version` | `input::watershed::structure` | init | immutable | version-compat gate and provenance manifest | `G-STR-001` |
| `nhill` | `topology_context.nhill` | `watershed.structure.context.nhill` | `input::watershed::structure` | init,watershed | immutable | `element_id` derivation, hillslope-coverage closure | `G-STR-010` |
| `elmt` | `rows[*].elmt` | `watershed.structure.rows[*].element_type_code` | `input::watershed::structure` | init,watershed | immutable | channel/impoundment branch dispatch | `G-STR-002` |
| `nhleft` | `rows[*].nhleft` | `watershed.structure.rows[*].hillslope_left_id` | `input::watershed::structure` | init,watershed | immutable | runoff routing graph assembler | `G-STR-003`, `G-STR-004` |
| `nhrght` | `rows[*].nhrght` | `watershed.structure.rows[*].hillslope_right_id` | `input::watershed::structure` | init,watershed | immutable | runoff routing graph assembler | `G-STR-003`, `G-STR-004` |
| `nhtop` | `rows[*].nhtop` | `watershed.structure.rows[*].hillslope_top_id` | `input::watershed::structure` | init,watershed | immutable | runoff routing graph assembler | `G-STR-003`, `G-STR-004` |
| `ncleft` | `rows[*].ncleft` | `watershed.structure.rows[*].channel_left_id` | `input::watershed::structure` | init,watershed | immutable | runoff routing graph assembler | `G-STR-003`, `G-STR-004` |
| `ncrght` | `rows[*].ncrght` | `watershed.structure.rows[*].channel_right_id` | `input::watershed::structure` | init,watershed | immutable | runoff routing graph assembler | `G-STR-003`, `G-STR-004` |
| `nctop` | `rows[*].nctop` | `watershed.structure.rows[*].channel_top_id` | `input::watershed::structure` | init,watershed | immutable | runoff routing graph assembler | `G-STR-003`, `G-STR-004` |
| `nileft` | `rows[*].nileft` | `watershed.structure.rows[*].impoundment_left_id` | `input::watershed::structure` | init,watershed | immutable | runoff routing graph assembler | `G-STR-003`, `G-STR-004` |
| `nirght` | `rows[*].nirght` | `watershed.structure.rows[*].impoundment_right_id` | `input::watershed::structure` | init,watershed | immutable | runoff routing graph assembler | `G-STR-003`, `G-STR-004` |
| `nitop` | `rows[*].nitop` | `watershed.structure.rows[*].impoundment_top_id` | `input::watershed::structure` | init,watershed | immutable | runoff routing graph assembler | `G-STR-003`, `G-STR-004` |
| row order | `rows[*].position` | `watershed.structure.rows[*].record_index` | `input::watershed::structure` | init | immutable | deterministic element-id derivation | `G-STR-005`, `G-STR-011` |
| derived `element_id` | `derived.element_id` | `watershed.structure.rows[*].element_id` | `input::watershed::structure` | init,watershed | immutable | channel/impoundment table joins | `G-STR-006` |
| derived `idelmt` | `derived.idelmt` | `watershed.structure.rows[*].element_local_index` | `input::watershed::structure` | init,watershed | immutable | channel/impoundment table joins | `G-STR-006` |
| derived `nchan` | `derived.nchan` | `watershed.structure.summary.channel_count` | `input::watershed::structure` | init,watershed | immutable | cross-file closure vs `.chn/.man/.imp` | `G-STR-007`, `G-STR-008` |
| derived `npond` | `derived.npond` | `watershed.structure.summary.impoundment_count` | `input::watershed::structure` | init,watershed | immutable | cross-file closure vs `.chn/.man/.imp` | `G-STR-007`, `G-STR-008` |
| derived `nhmax` | `derived.nhmax` | `watershed.structure.summary.max_hillslope_ref` | `input::watershed::structure` | init,watershed | immutable | hillslope coverage closure | `G-STR-008` |

## 5. State Ownership and Mutability

- `input::watershed::structure` owns parsed `.str` source records and normalized topology graph state.
- Parsed rows and normalized contributor edges are immutable after parse success.
- Routing runtime modules may maintain mutable flow state, but may not mutate canonical structure graph definitions.
- Forbidden mutation path: any non-input module rewriting contributor IDs, element class, or derived counts after cross-file closure.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-STR-001` | Derive `element_id = nhill + record_index` for each row. | parse finalize | `C-STR-001` |
| `D-STR-002` | Derive `nchan` and `npond` counts by `elmt` class. | parse finalize | `C-STR-002` |
| `D-STR-003` | Derive class-local `idelmt` indexing. | parse finalize | `C-STR-003` |

Closure hooks:
- `C-STR-001`: record-index determinism and monotone element-ID assignment.
- `C-STR-002`: class-count closure and allowed `elmt` domain (`2` or `3`).
- `C-STR-003`: graph-connectivity minimum (no isolated downstream element).

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `STR-E-000` | io | missing/unopenable `.str` file |
| `STR-E-001` | syntax | token parse failure in required numeric fields |
| `STR-E-002` | syntax | row arity mismatch (expected 10 integer fields) |
| `STR-E-003` | semantic | unsupported/invalid datver policy result |
| `STR-E-004` | semantic | invalid element type (`elmt` not in `{2,3}`) |
| `STR-E-005` | semantic | disconnected downstream element (all contributors zero) |
| `STR-E-006` | semantic | invalid contributor ID domain/reference |
| `STR-E-007` | cross-file | channel count mismatch (`.str` vs `.chn` / management channel count) |
| `STR-E-008` | cross-file | impoundment count mismatch (`.str` vs `.imp`) |
| `STR-E-009` | cross-file | hillslope coverage mismatch (`nhmax` vs `nhill`) |
| `STR-E-010` | runtime-guard | post-parse topology closure failure |
| `STR-E-011` | syntax | file-level row-count closure mismatch (surplus/deficit rows vs expected `n_rows`) |
| `STR-W-001` | compat-warning | legacy no-datver acceptance path used in compatibility mode |

No silent fallback/default masking is permitted for malformed required rows.

## 8. Cross-File Consistency Constraints

1. Derived `nchan` from `.str` must equal channel count in `.chn` and management-derived channel count where applicable. `[DIRECT][E-SPEC-STR-01]`
2. Derived `npond` from `.str` must satisfy impoundment-file closure policy (`strict`: exact match; `compat`: may allow `.imp` surplus with warning). `[DIRECT][E-SPEC-STR-01]`, `[INFERENCE][E-WF-STR-01]`
3. Maximum referenced hillslope contributor ID must equal `nhill` from watershed topology context. `[DIRECT][E-SPEC-STR-01]`
4. Contributor IDs must resolve to valid previously defined/hydrologically valid upstream elements under topology rules from usersum and legacy checks. `[DIRECT][E-SPEC-STR-01]`, `[INFERENCE][E-PHYS-STR-01]`
5. Structure surface is invalid in isolation; `.str` closure requires at least `.chn` and `.man` plus optional `.imp` if impoundments are present. `[DIRECT][E-SPEC-STR-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `datver` | `watershed.structure.version.datver` | parser output manifest | `datver` (canonical) + `structure_version` alias | explicit when present in input; compat mode annotates no-datver acceptance |
| `nhill` | `watershed.structure.context.nhill` | parser output manifest | `nhill` (canonical dependency) + `hillslope_count` alias | imported from watershed topology authority |
| row fields (`elmt`, `nhleft`, `nhrght`, `nhtop`, `ncleft`, `ncrght`, `nctop`, `nileft`, `nirght`, `nitop`) | `watershed.structure.rows[*]` | watershed topology interchange | canonical field names preserved; alias names emitted in typed payload schema | no unit conversion |
| derived `element_id`, `record_index`, `element_local_index` | `watershed.structure.rows[*].ids` | routing graph assembly boundary | exported as explicit derived metadata fields | derived values are immutable after parse |
| derived `nchan`,`npond`,`nhmax` | `watershed.structure.summary` | cross-file validator boundary | `channel_count`, `impoundment_count`, `max_hillslope_ref` aliases with canonical derivation provenance | consumed by `.chn/.imp` closure gate and hillslope coverage checks |

## 10. Compatibility Policy

- Strict mode:
  - requires explicit datver line;
  - accepts only `datver >= 94.301`;
  - rejects legacy no-datver form;
  - enforces full row arity/type and topology closure checks.
- Compatibility mode:
  - may accept no-datver legacy form when explicitly enabled;
  - retains `datver >= 94.301` lower bound for explicit datver entries;
  - still rejects disconnected/invalid topology rows;
  - emits `STR-W-001` on accepted no-datver path.

Compatibility acceptance must be observable via typed warnings (`STR-W-001`); no silent normalization.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-STR-001` | datver policy gate | header parse | `STR-E-003` |
| `G-STR-002` | `elmt` enum domain | row parse | `STR-E-004` |
| `G-STR-003` | row arity closure (10 ints) | row parse | `STR-E-002` |
| `G-STR-004` | contributor ID domain | row parse | `STR-E-006` |
| `G-STR-005` | deterministic record ordering | parse finalize | `STR-E-010` |
| `G-STR-006` | derived `element_id` closure | parse finalize | `STR-E-010` |
| `G-STR-007` | derived count closure (`nchan`,`npond`) | parse finalize | `STR-E-010` |
| `G-STR-008` | cross-file count/topology closure (`.chn/.man/.imp`) | cross-surface validator | `STR-E-007`/`STR-E-008`/`STR-E-009` |
| `G-STR-009` | non-isolated downstream element (at least one contributor non-zero) | row semantic validation | `STR-E-005` |
| `G-STR-010` | `nhill` context availability and positive domain | topology context binding | `STR-E-009` |
| `G-STR-011` | strict row-count closure (`n_rows` expected, no surplus/deficit) | file finalize | `STR-E-011` |
| `G-STR-012` | compatibility no-datver observability | preamble policy gate | `STR-W-001` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical variable names remain authoritative and unchanged:
`datver`, `elmt`, `nhleft`, `nhrght`, `nhtop`, `ncleft`, `ncrght`, `nctop`, `nileft`, `nirght`, `nitop`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `STR-GAP-001` | Usersum topology restrictions beyond legacy hard-stop checks need explicit parse-vs-topology-validator placement in executable architecture docs. | `[DIRECT][E-SPEC-STR-01]` | `HOLD` |
| `STR-GAP-002` | Legacy no-datver compatibility corpus is not yet benchmarked; strict/compat enablement policy needs fixture evidence. | `[DIRECT][E-SPEC-STR-01]` | `HOLD` |
| `STR-GAP-003` | wepppy emitted channel-only patterns in some generated structures require explicit compatibility disposition (accepted vs rejected strict mode). | `[DIRECT][E-SPEC-STR-01]`, `[DIRECT][E-WP-STR-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.1` | Added symbol-level propagation rows, `nhill`/`nhmax` modeling, strict row-count closure error/guard, and explicit compatibility warning typing. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE05. |
