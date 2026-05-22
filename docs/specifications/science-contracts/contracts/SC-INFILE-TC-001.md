---
contract_id: SC-INFILE-TC-001
title: Channel TC Sentinel Input Parser Contract (tc.txt)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-TC-001 Channel TC Sentinel Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-TC-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tc.spec.md` (canonical sentinel behavior and governance gaps).
- `[DIRECT][E-SURVEY-TC-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and ownership context).
- `[DIRECT][E-WF-TC-01]` `/workdir/wepp-forest/docs/work-packages/20260506-wb13-legacy-for-retirement-deletion/artifacts/legacy_reference_snapshot/src/wshdrv.for` (legacy sentinel latch and `tc_out.txt` activation behavior).
- `[DIRECT][E-WP-TC-01]` `/workdir/wepppy/wepppy/nodb/core/wepp.py`, `/workdir/wepppy/wepppy/nodb/mods/omni/omni.py`, `/workdir/wepppy/wepppy/rq/wepp_rq_stage_post.py` (modern sentinel creation and downstream artifact handling).
- `[INFERENCE][E-PHYS-TC-01]` Process/common-sense invariants: sentinel mode toggles must be deterministic and should not silently suppress requested diagnostics under IO faults.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for sentinel sidecar surface `infile-channel-tc` (`tc.txt`) and parse-to-runtime handoff of diagnostic-output mode activation (`tc_out.txt`) in watershed runs.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | sentinel absent | Accept optional absence branch. | `luntc=0`, no TC output activation. | `[DIRECT][E-WF-TC-01]` |
| B | sentinel present and open succeeds | Accept sentinel presence. | `luntc=1` and TC output activation branch enabled. | `[DIRECT][E-SPEC-TC-01]`, `[DIRECT][E-WF-TC-01]` |
| C | sentinel present but open fails | Strict: typed IO failure. Compatibility: may collapse to missing branch with warning. | Explicit strict/compat divergence required. | `[DIRECT][E-SPEC-TC-01]`, `[DIRECT][E-WF-TC-01]` |
| D | sentinel has arbitrary body content | Accept in compatibility and strict (content-insensitive); body is ignored. | Must not infer parameters from body content. | `[DIRECT][E-SPEC-TC-01]`, `[DIRECT][E-WF-TC-01]` |

No datver/version line exists for this sentinel surface.

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
tc_file = byte_stream ;
byte_stream = { byte } ;
```

### 2.2 Two-Layer Model Contract

- Source model captures sentinel presence/openability and payload metadata only; no body token parsing.
- Simulation model normalizes:
  - requested/effective mode surfaces (`luntc_requested`, `luntc`),
  - provenance (`tc_file_present`, `payload_bytes`, `payload_nonempty`, `open_result`),
  - warning trigger observability for ignored sentinel payload,
  - run-context applicability surface (`run_context`),
  - expected output activation state (`tc_out_expected`).
- Parser does not parse `tc_out.txt`; downstream output subsystem owns output-file content contract.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| derived `luntc_requested` | `external.requested_tc_output` | `tc.mode.luntc_requested` | flag | int | 1 | yes | all | `1` when orchestrator requests TC output, else `0` | `observability.channel_tc.requested` |
| `luntc` | derived effective mode from sentinel/open + context gates | `tc.mode.luntc` | flag | int | 1 | yes | all | `1` when sentinel open succeeds in watershed context; `0` only explicit missing-sentinel branch (or compatibility open-error collapse branch); strict non-ENOENT open-error branch is typed failure and does not emit normalized effective-mode state | `observability.channel_tc.enabled` |
| derived `tc_file_present` | sentinel presence/open branch | `tc.mode.tc_file_present` | flag | bool | 1 | yes | all | `true` when open succeeds | `tc.sidecar_present` |
| derived `payload_bytes` | file metadata | `tc.mode.payload_bytes` | bytes | int | 1 | yes | all | byte count of sentinel payload | `tc.payload_bytes` |
| derived `payload_nonempty` | file metadata | `tc.mode.payload_nonempty` | flag | bool | 1 | yes | all | `payload_bytes > 0` | `tc.payload_nonempty` |
| derived `payload_ignored_warning_emitted` | compat warning trigger surface | `tc.mode.payload_ignored_warning_emitted` | flag | bool | 1 | yes | all | `true` only when compatibility mode accepts `payload_nonempty=true` and emits `TC-W-003`; otherwise `false` | `tc.payload_ignored_warning_emitted` |
| derived `open_result` | sentinel open branch outcome | `tc.mode.open_result` | enum | string | 1 | yes | all | `missing`, `open_success`, `open_error_collapsed_compat`; strict non-ENOENT open-error path is typed failure (`TC-E-000`) with no normalized-state emission | `tc.open_result` |
| derived `run_context` | orchestrator/runtime context surface | `tc.mode.run_context` | enum | string | 1 | yes | all | `watershed` or `hillslope`; `tc.txt` supported only for `watershed` | `run.context` |
| derived `mode_divergence` | requested-vs-effective mode comparison | `tc.mode.mode_divergence` | flag | bool | 1 | yes | all | `true` when `luntc_requested != luntc` in normalized branches | `tc.mode_divergence` |
| derived `tc_out_expected` | mode-derived output activation marker | `tc.mode.tc_out_expected` | flag | bool | 1 | yes | all | `true` when `luntc=1` | `outputs.tc_out.expected` |
| `tc_out.txt` | downstream output artifact (external) | `tc.outputs.tc_out_path` | path | string | 0..1 | conditional | all | materialized by runtime only when enabled | `outputs.tc_out.path` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| derived `luntc_requested` | `external.requested_tc_output` | `tc.mode.luntc_requested` | `input::orchestrator::run_options` | init | immutable | mode-intent diagnostics and closure checks | `G-TC-001`, `G-TC-007` |
| `luntc` | `derived.luntc` | `tc.mode.luntc` | `input::sidecar::tc` | init,watershed,event | immutable | output activation gate for `tc_out.txt` | `G-TC-001`, `G-TC-003`, `G-TC-004` |
| derived `tc_file_present` | `derived.file_present` | `tc.mode.tc_file_present` | `input::sidecar::tc` | init | immutable | parser provenance/diagnostics | `G-TC-001` |
| derived `payload_bytes` | `derived.payload_bytes` | `tc.mode.payload_bytes` | `input::sidecar::tc` | init | immutable | observability metadata | `G-TC-002` |
| derived `payload_nonempty` | `derived.payload_nonempty` | `tc.mode.payload_nonempty` | `input::sidecar::tc` | init | immutable | explicit content-insensitive warning trigger input | `G-TC-002`, `G-TC-008` |
| derived `payload_ignored_warning_emitted` | `derived.payload_ignored_warning_emitted` | `tc.mode.payload_ignored_warning_emitted` | `input::sidecar::tc` | init | immutable | warning branch observability | `G-TC-002`, `G-TC-008` |
| derived `open_result` | `derived.open_result` | `tc.mode.open_result` | `input::sidecar::tc` | init | immutable | strict/compat open-branch observability | `G-TC-003` |
| derived `run_context` | `external.run_context` | `tc.mode.run_context` | `input::orchestrator::run_context` | init | immutable | watershed-only applicability validator | `G-TC-006` |
| derived `mode_divergence` | `derived.mode_divergence` | `tc.mode.mode_divergence` | `input::sidecar::tc` | init,watershed,event | immutable | requested-vs-effective mode observability | `G-TC-004`, `G-TC-007` |
| derived `tc_out_expected` | `derived.tc_out_expected` | `tc.mode.tc_out_expected` | `input::sidecar::tc` | init,watershed | immutable | output expectations validator | `G-TC-004`, `G-TC-005`, `G-TC-006` |
| `tc_out.txt` | external runtime output surface | `tc.outputs.tc_out_path` | `runtime::watershed::outputs` | watershed,event | mutable | interchange/post-run artifact staging | `G-TC-005` |

## 5. State Ownership and Mutability

- `input::sidecar::tc` owns sentinel-derived mode/provenance state.
- Sentinel-derived parser state is immutable after parse finalization.
- Runtime output modules own mutable output-artifact lifecycle (`tc_out.txt` creation/close/move), but may not mutate parser-owned sentinel-state surfaces.
- Forbidden mutation path: downstream modules toggling `luntc` after parser finalization without explicit orchestrator transition.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-TC-001` | Derive requested/effective mode (`luntc_requested`, `luntc`) from run-options plus sentinel open branch. | parse preamble/finalize | `C-TC-001` |
| `D-TC-002` | Derive payload-byte/nonempty metadata and explicit ignored-payload warning trigger surfaces. | parse preamble | `C-TC-002` |
| `D-TC-003` | Derive output-expectation marker from `luntc`. | parse finalize | `C-TC-003` |
| `D-TC-004` | Derive run-context applicability state and requested-vs-effective divergence marker. | parse/cross-file finalize | `C-TC-004` |

Closure hooks:
- `C-TC-001`: `luntc` derivation must be deterministic and explicit.
- `C-TC-002`: payload metadata and warning-trigger surfaces must be observable (content-insensitive semantics retained).
- `C-TC-003`: sentinel-enabled mode must propagate to explicit output expectation state.
- `C-TC-004`: watershed applicability and requested-vs-effective divergence surfaces must be explicit and guard-linked.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `TC-E-000` | io | non-ENOENT sentinel open error in strict mode |
| `TC-E-001` | cross-file | sentinel policy used outside watershed context where surface is unsupported |
| `TC-E-002` | runtime-guard | `tc_out_expected=true` but output activation closure fails |
| `TC-E-003` | runtime-guard | parser/routing divergence in sentinel mode flag |
| `TC-W-001` | compat-warning | optional sentinel absent branch taken (`luntc=0`) |
| `TC-W-002` | compat-warning | open-error collapsed with missing branch in compatibility mode |
| `TC-W-003` | compat-warning | content-insensitive sentinel body ignored |
| `TC-W-004` | compat-warning | `tc_out.txt` expected but absent at post-run harvest stage |

No silent parser-side masking is permitted in strict mode for non-ENOENT open failures.

## 8. Cross-File Consistency Constraints

1. `tc.txt` sentinel semantics are watershed-specific; hillslope-only contexts must explicitly reject or ignore with typed behavior, driven by explicit `run_context` model state. `[DIRECT][E-SPEC-TC-01]`, `[INFERENCE][E-WF-TC-01]`
2. `luntc` mode flag must align with `tc_out` output activation lifecycle. `[DIRECT][E-WF-TC-01]`
3. Post-run tooling expectations for optional `tc_out.txt` must remain consistent with parser mode provenance. `[DIRECT][E-WP-TC-01]`
4. Sentinel body content must remain semantically inert across parser/runtime boundaries. `[DIRECT][E-SPEC-TC-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `luntc_requested,luntc,mode_divergence` | `tc.mode.{luntc_requested,luntc,mode_divergence}` | `openwepp.boundary.mode_selection.tc.v1` | explicit requested/effective/divergence fields + aliases | primary sentinel mode export |
| `tc_file_present,payload_bytes,payload_nonempty,payload_ignored_warning_emitted,open_result,run_context,tc_out_expected` | `tc.mode.*` | `openwepp.boundary.observability.parser_warnings.v1` | explicit branch/warning/context fields | strict/compat diagnostics and applicability observability |
| `tc_out.txt` | `tc.outputs.tc_out_path` | `openwepp.boundary.outputs.optional_artifacts.v1` | output path + expected flag | supports post-run staging and interchange |

## 10. Compatibility Policy

- Strict mode:
  - treats sentinel as content-insensitive presence switch;
  - distinguishes non-ENOENT open failures as typed errors (`TC-E-000`);
  - requires explicit watershed-context applicability;
  - exports requested/effective mode plus divergence for all normalized branches.
- Compatibility mode:
  - allows sentinel absence with explicit warning/provenance (`TC-W-001`);
  - may collapse open errors with missing branch while emitting `TC-W-002`;
  - ignores sentinel body payload content with explicit warning/provenance (`TC-W-003`) when `payload_nonempty=true` and exports warning-trigger field;
  - exports requested/effective mode plus divergence when collapse branches are used.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-TC-001` | deterministic requested+sentinel branch -> effective `luntc` derivation | parse preamble/finalize | `TC-E-003`/`TC-W-001` |
| `G-TC-002` | payload metadata capture with content-insensitive semantics | parse metadata | `TC-E-003`/`TC-W-003` |
| `G-TC-003` | strict open-error policy (non-ENOENT) | preamble open handler | strict: `TC-E-000`; compat: `TC-W-002` |
| `G-TC-004` | mode-to-output expectation mapping (`luntc` -> `tc_out_expected`) and requested/effective divergence closure | parse finalize | `TC-E-003` |
| `G-TC-005` | runtime output activation closure (`tc_out_expected` vs output lifecycle) | runtime/post-run validator | `TC-E-002`/`TC-W-004` |
| `G-TC-006` | watershed-context applicability enforcement using explicit `run_context` surface | cross-context validator | `TC-E-001` |
| `G-TC-007` | requested-vs-effective mode observability export closure | parse/runtime boundary validator | `TC-E-003` |
| `G-TC-008` | content-insensitive warning trigger closure (`payload_nonempty` -> `TC-W-003` in compatibility mode) | parse policy gate / boundary validator | `TC-E-003`/`TC-W-003` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`luntc`, `tc_out.txt`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `TC-GAP-001` | Active source-authority provenance for `tc.txt` currently depends on retirement snapshot references and must be formally ratified. | `[DIRECT][E-SPEC-TC-01]`, `[DIRECT][E-WF-TC-01]` | `RATIFIED-W4DR-001 (2026-05-22)` |
| `TC-GAP-002` | Strict vs compatibility policy for open-error collapse requires fixture-backed governance closure. | `[DIRECT][E-SPEC-TC-01]`, `[DIRECT][E-WF-TC-01]` | `RATIFIED-W4DR-002 (2026-05-22)` |
| `TC-GAP-003` | `tc_out.txt` row-level output grammar authority remains unresolved in parser-contract scope and needs downstream output-contract alignment. | `[DIRECT][E-SPEC-TC-01]` | `RATIFIED-W4DR-012 (2026-05-22)` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-22` | `0.1.1` | Ratified HOLD gaps via ARCH13 decisions `W4DR-001/002/012`; parser/output boundary for `tc_out.txt` set to output-contract ownership. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE16. |
