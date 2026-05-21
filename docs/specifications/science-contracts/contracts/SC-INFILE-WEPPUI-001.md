---
contract_id: SC-INFILE-WEPPUI-001
title: WEPP UI Sentinel Sidecar Input Parser Contract (wepp_ui.txt)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-WEPPUI-001 WEPP UI Sentinel Sidecar Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-WUI-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/wepp-ui.spec.md` (canonical sentinel semantics and gap register).
- `[DIRECT][E-SURVEY-WUI-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and ownership context).
- `[DIRECT][E-WF-WUI-01]` `/workdir/wepp-forest/src/main.for`, `/workdir/wepp-forest/src/watbal.for`, `/workdir/wepp-forest/src/outfil.for`, `/workdir/wepp-forest/src/input.for` (legacy `ui_run` branching and soil coupling).
- `[DIRECT][E-WP-WUI-01]` `/workdir/wepppy/wepppy/nodb/core/wepp.py`, `/workdir/wepppy/wepppy/nodb/core/wepp_prep_service.py`, `/workdir/wepppy/wepppy/microservices/rq_engine/wepp_run_payload.py` (modern toggle and sentinel lifecycle).
- `[INFERENCE][E-PHYS-WUI-01]` Process/common-sense invariants: sentinel toggles must be deterministic and observable; requested hourly mode should not silently degrade under IO faults.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for sentinel sidecar surface `infile-wepp-ui` (`wepp_ui.txt`) and parse-to-runtime handoff of `ui_run` mode selection for daily vs hourly seepage update behavior.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | sentinel absent | Accept optional absence branch. | `ui_run=0` with explicit provenance marker. | `[DIRECT][E-WF-WUI-01]` |
| B | sentinel present and open succeeds | Accept sentinel presence. | `ui_run=1` branch enabled. | `[DIRECT][E-SPEC-WUI-01]`, `[DIRECT][E-WF-WUI-01]` |
| C | sentinel present with non-empty payload | Strict reject (canonical empty-file policy); compatibility may accept ignored payload. | Strict typed error or compat warning with explicit provenance. | `[DIRECT][E-SPEC-WUI-01]`, `[DIRECT][E-WF-WUI-01]` |
| D | sentinel path exists but open fails | Strict: reject as typed IO fault; compatibility: may collapse to missing branch with warning. | Explicit strict/compat divergence required. | `[DIRECT][E-SPEC-WUI-01]`, `[DIRECT][E-WF-WUI-01]` |

No datver/version line exists for this sentinel surface.

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
wepp_ui_file         = strict_wepp_ui_file | compat_wepp_ui_file ;
strict_wepp_ui_file  = empty_file ;
compat_wepp_ui_file  = byte_stream ;

empty_file           = "" ;
byte_stream          = { byte } ;
```

### 2.2 Two-Layer Model Contract

- Source model captures sentinel presence/openability and payload length only; no token/field parsing occurs.
- Simulation model normalizes:
  - requested/effective mode surfaces (`ui_run_requested`, `ui_run`),
  - sentinel provenance (`wepp_ui_file_present`, `payload_bytes`, `payload_nonempty`),
  - parser open branch provenance (`open_result`),
  - cross-file soil-version compatibility state with deterministic multi-soil reduction.
- Parser does not execute `watbal`/`watbal_hourly`; downstream runtime selects branch based on normalized mode flag.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| derived `ui_run_requested` | `external.requested_hourly_seepage` | `wepp_ui.mode.ui_run_requested` | flag | int | 1 | yes | all | `1` when orchestrator requests hourly seepage, else `0` | `run_options.wepp_ui_requested` |
| `ui_run` | derived effective mode from sentinel/open + policy gates | `wepp_ui.mode.ui_run` | flag | int | 1 | yes | all | `1` when sentinel open succeeds and strict/compat policy gates pass; `0` only explicit missing-sentinel branch (or compatibility-mode open-error collapse branch); strict non-ENOENT open-error branch is typed failure and does not emit normalized effective-mode state | `run_options.wepp_ui_enabled` |
| derived `wepp_ui_file_present` | sentinel presence/open branch | `wepp_ui.mode.wepp_ui_file_present` | flag | bool | 1 | yes | all | `true` when file open succeeds | `wepp_ui.sidecar_present` |
| derived `payload_bytes` | file metadata | `wepp_ui.mode.payload_bytes` | bytes | int | 1 | yes | all | byte count of sidecar payload | `wepp_ui.payload_bytes` |
| derived `payload_nonempty` | file metadata | `wepp_ui.mode.payload_nonempty` | flag | bool | 1 | yes | all | `payload_bytes > 0` | `wepp_ui.payload_nonempty` |
| derived `open_result` | sentinel open branch outcome | `wepp_ui.mode.open_result` | enum | string | 1 | yes | all | `missing`, `open_success`, `open_error_collapsed_compat`; strict non-ENOENT open-error path is typed failure (`WUI-E-000`) with no normalized-state emission | `wepp_ui.open_result` |
| `solwpv` | external soil-surface version token | `wepp_ui.crossfile.solwpv` | version | real | 1..n | conditional | all | sourced from soil-file parser output | `soil.version` |
| derived `solwpv_reduced_min` | deterministic multi-soil reduction from `solwpv[1..n]` | `wepp_ui.crossfile.solwpv_reduced_min` | version | real | 0..1 | conditional | all | minimum finite `solwpv` across active soil profiles; unresolved when no valid values available | `soil.version_reduced_min` |
| derived `soil_compatibility_state` | cross-file check outcome | `wepp_ui.crossfile.soil_compatibility_state` | enum | string | 1 | yes | all | `compatible_7778_or_newer` / `legacy_2006` / `unresolved` | `wepp_ui.soil_compatibility_state` |
| derived `mode_divergence` | requested-vs-effective mode comparison | `wepp_ui.mode.mode_divergence` | flag | bool | 1 | yes | all | `true` when `ui_run_requested != ui_run` in normalized branches | `wepp_ui.mode_divergence` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| derived `ui_run_requested` | `external.requested_hourly_seepage` | `wepp_ui.mode.ui_run_requested` | `input::orchestrator::run_options` | init | immutable | mode-intent diagnostics and closure checks | `G-WUI-001`, `G-WUI-007` |
| `ui_run` | `derived.ui_run` | `wepp_ui.mode.ui_run` | `input::sidecar::wepp_ui` | init,daily,event | immutable | `watbal` branch selector | `G-WUI-001`, `G-WUI-004` |
| derived `wepp_ui_file_present` | `derived.file_present` | `wepp_ui.mode.wepp_ui_file_present` | `input::sidecar::wepp_ui` | init | immutable | mode/provenance diagnostics | `G-WUI-001` |
| derived `payload_bytes` | `derived.payload_bytes` | `wepp_ui.mode.payload_bytes` | `input::sidecar::wepp_ui` | init | immutable | strict/compat policy gate | `G-WUI-002` |
| derived `payload_nonempty` | `derived.payload_nonempty` | `wepp_ui.mode.payload_nonempty` | `input::sidecar::wepp_ui` | init | immutable | strict/compat policy gate | `G-WUI-002` |
| derived `open_result` | `derived.open_result` | `wepp_ui.mode.open_result` | `input::sidecar::wepp_ui` | init | immutable | strict/compat open-branch observability | `G-WUI-006` |
| `solwpv` | `external.soil.solwpv` | `wepp_ui.crossfile.solwpv` | `input::crossfile::soil` | init | immutable | soil-compat validator | `G-WUI-003` |
| derived `solwpv_reduced_min` | `derived.solwpv_reduced_min` | `wepp_ui.crossfile.solwpv_reduced_min` | `input::sidecar::wepp_ui` | init | immutable | deterministic multi-soil compatibility reduction | `G-WUI-003`, `G-WUI-005` |
| derived `soil_compatibility_state` | `derived.soil_compatibility_state` | `wepp_ui.crossfile.soil_compatibility_state` | `input::sidecar::wepp_ui` | init | immutable | policy/diagnostic branching | `G-WUI-003`, `G-WUI-005` |
| derived `mode_divergence` | `derived.mode_divergence` | `wepp_ui.mode.mode_divergence` | `input::sidecar::wepp_ui` | init,daily,event | immutable | requested-vs-effective mode observability | `G-WUI-004`, `G-WUI-007` |

## 5. State Ownership and Mutability

- `input::sidecar::wepp_ui` owns sentinel-derived mode/provenance surfaces.
- Parsed sentinel state is immutable after parser finalization.
- Runtime hydrology modules own mutable water-balance accumulators but may not mutate parser-owned mode flags.
- Forbidden mutation path: runtime modules rewriting `ui_run` after parser finalization without explicit orchestrator policy transition.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-WUI-001` | Derive requested/effective mode (`ui_run_requested`, `ui_run`) from run-options plus sentinel branch outcomes. | parse preamble/finalize | `C-WUI-001` |
| `D-WUI-002` | Derive payload-byte/nonempty metadata. | parse preamble/finalize | `C-WUI-002` |
| `D-WUI-003` | Derive deterministic multi-soil `solwpv_reduced_min` and `soil_compatibility_state` from `solwpv[1..n]`. | cross-file validation | `C-WUI-003` |
| `D-WUI-004` | Derive `mode_divergence` (`ui_run_requested != ui_run`) for normalized branches. | parse/cross-file finalize | `C-WUI-004` |

Closure hooks:
- `C-WUI-001`: mode-flag derivation must be deterministic and explicit.
- `C-WUI-002`: strict/compat payload policy must be observable.
- `C-WUI-003`: soil-version compatibility policy branch must be explicit and guard-linked.
- `C-WUI-004`: requested-vs-effective mode divergence must be exported and guard-linked.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `WUI-E-000` | io | non-ENOENT sentinel open failure in strict mode |
| `WUI-E-001` | semantic | non-empty sentinel payload in strict mode |
| `WUI-E-002` | cross-file | strict soil-version incompatibility for hourly mode (`ui_run=1` with incompatible `solwpv`) |
| `WUI-E-003` | runtime-guard | post-parse mode closure mismatch (`requested` vs `effective` branch) |
| `WUI-E-004` | runtime-guard | missing/invalid cross-file soil-version surface when required for policy gate |
| `WUI-W-001` | compat-warning | sentinel absent => daily default branch |
| `WUI-W-002` | compat-warning | non-empty sentinel payload ignored |
| `WUI-W-003` | compat-warning | non-recommended soil-version accepted with hourly mode |
| `WUI-W-004` | compat-warning | open-error collapsed with missing branch in compatibility mode |

No silent parser-side masking is permitted in strict mode.

## 8. Cross-File Consistency Constraints

1. `ui_run` mode selection must align with water-balance branch entry (`watbal` vs `watbal_hourly`) and output labeling semantics. `[DIRECT][E-WF-WUI-01]`
2. Soil-version compatibility (`solwpv`) must be evaluated explicitly when hourly mode is requested, using deterministic reduction `solwpv_reduced_min=min(solwpv[1..n])` across active soil profiles. `[DIRECT][E-SPEC-WUI-01]`, `[INFERENCE][E-WF-WUI-01]`
3. Orchestrator run-option toggle surfaces must map losslessly to sentinel presence behavior. `[DIRECT][E-WP-WUI-01]`
4. Replay/observability surfaces must expose requested/effective mode and divergence if any. `[DIRECT][E-WF-WUI-01]`, `[DIRECT][E-WP-WUI-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `ui_run_requested,ui_run,mode_divergence` | `wepp_ui.mode.{ui_run_requested,ui_run,mode_divergence}` | `openwepp.boundary.mode_selection.wepp_ui.v1` | explicit requested/effective/divergence fields + aliases | primary mode-selection observability surface |
| `wepp_ui_file_present,payload_bytes,payload_nonempty,open_result` | `wepp_ui.mode.*` | `openwepp.boundary.observability.parser_warnings.v1` | explicit sentinel provenance/open-branch fields | strict/compat diagnostics |
| `solwpv,solwpv_reduced_min,soil_compatibility_state` | `wepp_ui.crossfile.*` | `openwepp.boundary.crossfile.compatibility.v1` | canonical + aliases `soil.version*` | compatibility policy observability |

## 10. Compatibility Policy

- Strict mode:
  - requires sentinel open success for hourly activation;
  - requires empty sentinel payload (`0` bytes);
  - distinguishes non-ENOENT open errors as typed failures;
  - enforces strict soil-compatibility policy branch when hourly mode is requested;
  - exports requested/effective mode plus divergence for every normalized branch.
- Compatibility mode:
  - allows sentinel absence with explicit daily-default warning/provenance (`WUI-W-001`);
  - allows non-empty payload while ignoring content (`WUI-W-002`);
  - allows non-recommended soil-version with warning (`WUI-W-003`);
  - may collapse open errors with missing branch while emitting explicit warning (`WUI-W-004`);
  - must export requested/effective mode plus divergence when a collapse branch is used.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-WUI-001` | deterministic requested+sentinel branch -> effective `ui_run` derivation | parse preamble/finalize | `WUI-E-003`/`WUI-W-001` |
| `G-WUI-002` | strict/compat payload emptiness policy | parse metadata/policy gate | strict: `WUI-E-001`; compat: `WUI-W-002` |
| `G-WUI-003` | soil-version compatibility evaluation for hourly mode using `solwpv_reduced_min` | cross-file validator | strict: `WUI-E-002`; compat: `WUI-W-003` |
| `G-WUI-004` | water-balance branch closure (`ui_run` vs selected kernel path) and requested/effective divergence closure | runtime branch validator | `WUI-E-003` |
| `G-WUI-005` | cross-file soil-version availability closure | cross-file validator | `WUI-E-004` |
| `G-WUI-006` | strict open-error handling (non-ENOENT) | preamble open handler | strict: `WUI-E-000`; compat: `WUI-W-004` |
| `G-WUI-007` | requested-vs-effective mode observability export closure | parse/runtime boundary validator | `WUI-E-003` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`ui_run`, `solwpv`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `WEPPUI-GAP-001` | Usersum 7778-soil recommendation vs permissive legacy behavior requires governance ratification of enforcement severity. | `[DIRECT][E-SPEC-WUI-01]` | `HOLD` |
| `WEPPUI-GAP-002` | Legacy merges missing and open-failure branches; strict typed IO-fault policy requires fixture-backed migration evidence. | `[DIRECT][E-SPEC-WUI-01]`, `[DIRECT][E-WF-WUI-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE15. |
