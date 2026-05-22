---
contract_id: SC-INFILE-LCWB-001
title: Channel Last-OFE Water-Balance Sentinel Input Parser Contract (lcwb.txt)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-LCWB-001 Channel Last-OFE Water-Balance Sentinel Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-LCWB-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md` (canonical sentinel specification and gap register).
- `[DIRECT][E-SURVEY-LCWB-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance context).
- `[DIRECT][E-WF-LCWB-01]` `/workdir/wepp-forest/src/wshinp.for`, `/workdir/wepp-forest/src/cchrt1.inc` (legacy sentinel open branch and canonical `lcwbflg` symbol).
- `[DIRECT][E-WF-LCWB-02]` `/workdir/wepp-forest/docs/work-packages/20260506-wb13-legacy-for-retirement-deletion/artifacts/legacy_reference_snapshot/src/watbalprint.for` (historical output-selection behavior keyed by `lcwbflg`).
- `[DIRECT][E-WP-LCWB-01]` `/workdir/wepppy/wepp_runner/templates/watershed.template`, `/workdir/wepppy/wepp_runner/wepp_runner.py`, `/workdir/wepppy/wepppy/wepp/interchange/README.md` (modern downstream output routing and file expectations).
- `[INFERENCE][E-PHYS-LCWB-01]` Process/common-sense invariants: sentinel toggles must remain deterministic and observably branch-safe; strict I/O faults must not silently degrade into missing-file defaults.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for optional sentinel sidecar surface `infile-channel-lcwb` (`lcwb.txt`) and parse-to-runtime handoff of `lcwbflg` branch selection semantics.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | sentinel absent | Accept optional absence branch. | `lcwbflg=0` with explicit provenance. | `[DIRECT][E-WF-LCWB-01]` |
| B | sentinel present/open succeeds | Accept sentinel branch. | `lcwbflg=1` with explicit provenance. | `[DIRECT][E-SPEC-LCWB-01]`, `[DIRECT][E-WF-LCWB-01]` |
| C | sentinel open fails (non-ENOENT) | Strict reject; compatibility may collapse to missing branch with warning. | Explicit strict/compat divergence required. | `[DIRECT][E-SPEC-LCWB-01]`, `[DIRECT][E-WF-LCWB-01]` |
| D | sentinel payload non-empty | Strict reject under canonical empty-sentinel policy; compatibility may accept ignored payload. | Explicit payload-policy observability required. | `[DIRECT][E-SPEC-LCWB-01]`, `[DIRECT][E-WF-LCWB-01]` |
| E | non-watershed run-context | Surface unsupported outside watershed context. | strict: typed applicability error; compat: typed not-applicable branch outcome with explicit warning/provenance. | `[DIRECT][E-SPEC-LCWB-01]`, `[INFERENCE][E-WF-LCWB-01]` |

No datver/version line is defined for this sentinel surface.

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
lcwb_file         = strict_lcwb_file | compat_lcwb_file ;
strict_lcwb_file  = empty_file ;
compat_lcwb_file  = byte_stream ;

empty_file        = "" ;
byte_stream       = { byte } ;
```

### 2.2 Two-Layer Model Contract

- Source model captures sentinel openability and payload metadata only; no tokenized record parsing occurs.
- Simulation model normalizes:
  - requested/effective mode surfaces (`lcwb_requested`, `lcwbflg`),
  - branch provenance (`lcwb_file_present`, `open_result`),
  - payload-policy observability (`payload_bytes`, `payload_nonempty`, `payload_nonwhitespace`, `payload_ignored_warning_emitted`),
  - run-context applicability and divergence (`run_context`, `mode_divergence`).
- Parser does not write output files; downstream output subsystems own `chnwb`/`chanwb` artifact lifecycle.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| derived `lcwb_requested` | `external.requested_channel_watbal_mode` | `lcwb.mode.lcwb_requested` | flag | int | 1 | yes | all | `1` when orchestrator requests last-OFE mode, else `0` | `channel.last_ofe_watbal_requested` |
| `lcwbflg` | derived from sentinel/open + policy branch | `lcwb.mode.lcwbflg` | flag | int | 1 | yes | all | `1` when sentinel open succeeds in watershed context; `0` on explicit missing branch (or compat open-error collapse branch); strict non-ENOENT open-error emits typed failure and no normalized effective-mode state | `channel.last_ofe_watbal_enabled` |
| derived `lcwb_file_present` | sentinel presence/open branch | `lcwb.mode.lcwb_file_present` | flag | bool | 1 | yes | all | `true` when sentinel open succeeds | `lcwb.sidecar_present` |
| derived `payload_bytes` | file metadata | `lcwb.mode.payload_bytes` | bytes | int | 1 | yes | all | byte size of sentinel payload | `lcwb.payload_bytes` |
| derived `payload_nonempty` | file metadata | `lcwb.mode.payload_nonempty` | flag | bool | 1 | yes | all | `payload_bytes > 0` | `lcwb.payload_nonempty` |
| derived `payload_nonwhitespace` | strict-payload policy surface | `lcwb.mode.payload_nonwhitespace` | flag | bool | 1 | yes | all | `true` when payload contains any non-whitespace bytes; strict policy rejects this branch | `lcwb.payload_nonwhitespace` |
| derived `payload_ignored_warning_emitted` | compat warning trigger surface | `lcwb.mode.payload_ignored_warning_emitted` | flag | bool | 1 | yes | all | `true` only when compatibility accepts non-empty payload and emits warning | `lcwb.payload_ignored_warning_emitted` |
| derived `open_result` | sentinel open branch outcome | `lcwb.mode.open_result` | enum | string | 1 | yes | all | `missing`, `open_success`, `open_error_collapsed_compat`, `not_applicable_compat`; strict non-ENOENT open-error emits typed failure with no normalized state | `lcwb.open_result` |
| derived `run_context` | orchestrator/runtime context surface | `lcwb.mode.run_context` | enum | string | 1 | yes | all | `watershed` or `hillslope`; surface supported only for `watershed` | `run.context` |
| derived `mode_divergence` | requested-vs-effective comparison | `lcwb.mode.mode_divergence` | flag | bool | 1 | yes | all | `true` when `lcwb_requested != lcwbflg` in normalized branches | `lcwb.mode_divergence` |
| derived `ofe_row_selection_policy_mode` | historical-policy projection surface (provisional) | `lcwb.mode.ofe_row_selection_policy_mode` | enum | string | 1 | yes | all | provisional projection (`last_ofe_only` when `lcwbflg=1`, `all_ofe` when `lcwbflg=0`) carried as policy intent while active-source consumer closure remains unresolved (`LCWB-GAP-002`) | `outputs.chnwb.ofe_row_selection_policy_mode` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| derived `lcwb_requested` | `external.requested_channel_watbal_mode` | `lcwb.mode.lcwb_requested` | `input::orchestrator::run_options` | init | immutable | mode-intent diagnostics | `G-LCWB-001`, `G-LCWB-007` |
| `lcwbflg` | `derived.lcwbflg` | `lcwb.mode.lcwbflg` | `input::sidecar::lcwb` | init,watershed,event | immutable | last-OFE selection branch gate | `G-LCWB-001`, `G-LCWB-006` |
| derived `lcwb_file_present` | `derived.file_present` | `lcwb.mode.lcwb_file_present` | `input::sidecar::lcwb` | init | immutable | sentinel provenance | `G-LCWB-001`, `G-LCWB-003` |
| derived `payload_bytes` | `derived.payload_bytes` | `lcwb.mode.payload_bytes` | `input::sidecar::lcwb` | init | immutable | payload policy gate | `G-LCWB-002` |
| derived `payload_nonempty` | `derived.payload_nonempty` | `lcwb.mode.payload_nonempty` | `input::sidecar::lcwb` | init | immutable | payload policy gate | `G-LCWB-002`, `G-LCWB-008` |
| derived `payload_nonwhitespace` | `derived.payload_nonwhitespace` | `lcwb.mode.payload_nonwhitespace` | `input::sidecar::lcwb` | init | immutable | strict payload policy gate | `G-LCWB-002`, `G-LCWB-008` |
| derived `payload_ignored_warning_emitted` | `derived.payload_ignored_warning_emitted` | `lcwb.mode.payload_ignored_warning_emitted` | `input::sidecar::lcwb` | init | immutable | compatibility warning observability | `G-LCWB-002`, `G-LCWB-008` |
| derived `open_result` | `derived.open_result` | `lcwb.mode.open_result` | `input::sidecar::lcwb` | init | immutable | strict/compat open-branch observability | `G-LCWB-003` |
| derived `run_context` | `external.run_context` | `lcwb.mode.run_context` | `input::orchestrator::run_context` | init | immutable | watershed-only applicability validator | `G-LCWB-004` |
| derived `mode_divergence` | `derived.mode_divergence` | `lcwb.mode.mode_divergence` | `input::sidecar::lcwb` | init,watershed,event | immutable | requested-vs-effective mode observability | `G-LCWB-006`, `G-LCWB-007` |
| derived `ofe_row_selection_policy_mode` | `derived.ofe_row_selection_policy_mode` | `lcwb.mode.ofe_row_selection_policy_mode` | `input::sidecar::lcwb` | init | immutable | historical-policy observability/export; not an active-source runtime closure guarantee | `G-LCWB-005` |

## 5. State Ownership and Mutability

- `input::sidecar::lcwb` owns parsed sentinel-provenance and effective-mode state surfaces.
- Sentinel-derived parser state is immutable after parse finalization.
- `runtime::watershed::output_selection` owns mutable output lifecycle state but must not mutate parser-owned mode/provenance fields.
- Forbidden mutation path: downstream output modules mutating `lcwbflg` post-parse without explicit orchestrator transition.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-LCWB-001` | Derive requested/effective mode (`lcwb_requested`, `lcwbflg`) from run options + sentinel branch. | parse preamble/finalize | `C-LCWB-001` |
| `D-LCWB-002` | Derive payload metadata and compatibility warning-trigger surfaces. | parse preamble/finalize | `C-LCWB-002` |
| `D-LCWB-003` | Derive run-context applicability and open-branch outcome (`open_result`). | parse preamble/finalize | `C-LCWB-003` |
| `D-LCWB-004` | Derive requested-vs-effective divergence and provisional `ofe_row_selection_policy_mode` projection. | parse/runtime finalize | `C-LCWB-004` |

Closure hooks:
- `C-LCWB-001`: `lcwbflg` must be deterministic and branch-consistent.
- `C-LCWB-002`: strict/compat payload policy must be observable and reproducible.
- `C-LCWB-003`: open-result and run-context applicability outcomes must be explicit.
- `C-LCWB-004`: provisional output-selection policy projection must remain explicitly marked as unresolved-authority (`LCWB-GAP-002`) and exported for observability only.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `LCWB-E-000` | io | non-ENOENT sentinel open failure in strict mode |
| `LCWB-E-001` | semantic | non-whitespace sentinel payload in strict mode |
| `LCWB-E-002` | applicability | surface invoked outside supported watershed context (strict mode) |
| `LCWB-E-003` | runtime-guard | requested/effective mode closure mismatch |
| `LCWB-W-001` | compat-warning | optional sentinel absence default branch taken (`lcwbflg=0`) |
| `LCWB-W-002` | compat-warning | non-empty sentinel payload ignored in compatibility mode |
| `LCWB-W-003` | compat-warning | open-error branch collapsed with missing branch in compatibility mode |
| `LCWB-W-004` | compat-warning | non-watershed context treated as typed not-applicable branch in compatibility mode |

No silent parser-side masking is permitted in strict mode for non-ENOENT open failures.

## 8. Cross-File Consistency Constraints

1. `lcwb.txt` surface is watershed-only and must not activate in hillslope-only context. `[DIRECT][E-SPEC-LCWB-01]`
2. `ofe_row_selection_policy_mode` is a provisional historical-policy projection surface and must remain explicitly non-authoritative until active-source consumer closure is resolved (`LCWB-GAP-002`). `[DIRECT][E-WF-LCWB-02]`, `[INFERENCE][E-WP-LCWB-01]`
3. `lcwbflg` semantics are distinct from channel-routing output gating (`ichout`/`nchnum`) and must remain namespace-separated. `[DIRECT][E-SPEC-LCWB-01]`, `[DIRECT][E-WF-LCWB-01]`
4. Modern output-option routing (`chnwb`) and interchange consumers must receive explicit mode/provenance metadata to avoid silent branch drift. `[DIRECT][E-WP-LCWB-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `lcwb_requested,lcwbflg,mode_divergence` | `lcwb.mode.{lcwb_requested,lcwbflg,mode_divergence}` | `openwepp.boundary.mode_selection.lcwb.v1` | explicit requested/effective/divergence fields + aliases | primary mode-selection observability |
| `lcwb_file_present,payload_bytes,payload_nonempty,payload_nonwhitespace,payload_ignored_warning_emitted,open_result,run_context` | `lcwb.mode.*` | `openwepp.boundary.observability.parser_warnings.v1` | explicit sentinel branch/payload/context fields | strict/compat diagnostics |
| `ofe_row_selection_policy_mode` | `lcwb.mode.ofe_row_selection_policy_mode` | `openwepp.boundary.outputs.chnwb_selection_policy.v1` | explicit provisional policy-projection enum | historical-policy observability only (not active-source runtime guarantee) |

## 10. Compatibility Policy

- Strict mode:
  - treats `lcwb.txt` as empty/whitespace-only sentinel surface;
  - rejects non-ENOENT open failures as typed IO errors;
  - rejects non-whitespace payloads as semantic errors;
  - enforces watershed-context applicability and explicit requested/effective observability.
- Compatibility mode:
  - allows missing sentinel branch with explicit warning/provenance (`LCWB-W-001`);
  - may collapse open errors with missing branch (`LCWB-W-003`);
  - allows non-empty payload while treating contents as semantically inert with explicit warning (`LCWB-W-002`);
  - treats non-watershed context as typed not-applicable branch with explicit warning (`LCWB-W-004`);
  - preserves explicit requested/effective/divergence exports.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-LCWB-001` | deterministic requested+sentinel branch -> effective `lcwbflg` derivation | parse preamble/finalize | `LCWB-E-003`/`LCWB-W-001` |
| `G-LCWB-002` | strict/compat payload policy (`payload_nonwhitespace` in strict; inert payload in compat) | parse metadata/policy gate | strict: `LCWB-E-001`; compat: `LCWB-W-002` |
| `G-LCWB-003` | strict/compat open-branch handling | preamble open handler | strict: `LCWB-E-000`; compat: `LCWB-W-003` |
| `G-LCWB-004` | watershed-context applicability | cross-context validator | strict: `LCWB-E-002`; compat: `LCWB-W-004` |
| `G-LCWB-005` | provisional `lcwbflg` -> `ofe_row_selection_policy_mode` projection closure with unresolved-authority marker | parse/boundary validator | `LCWB-E-003` |
| `G-LCWB-006` | effective-mode and requested/effective divergence closure | runtime/post-run validator | `LCWB-E-003` |
| `G-LCWB-007` | requested-vs-effective observability export closure | parse/runtime boundary validator | `LCWB-E-003` |
| `G-LCWB-008` | compat payload-warning trigger closure (`payload_nonempty`/`payload_nonwhitespace` -> `LCWB-W-002`) | parse policy gate / boundary validator | `LCWB-E-003`/`LCWB-W-002` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`lcwbflg`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `LCWB-GAP-001` | `usersum2024` does not provide a dedicated `lcwb.txt` format definition. | `[DIRECT][E-SPEC-LCWB-01]` | `RATIFIED-W4DR-001 (2026-05-22)` |
| `LCWB-GAP-002` | Active-source consumer closure for `lcwbflg` remains unresolved relative to historical snapshot evidence. | `[DIRECT][E-SPEC-LCWB-01]`, `[DIRECT][E-WF-LCWB-02]` | `RATIFIED-W4DR-011 (2026-05-22)` |
| `LCWB-GAP-003` | Separation between `lcwb` mode semantics and channel-output option gating requires fixture-backed governance closure. | `[DIRECT][E-SPEC-LCWB-01]`, `[DIRECT][E-WP-LCWB-01]` | `RATIFIED-W4DR-003 (2026-05-22)` |
| `LCWB-GAP-004` | Program-level decision on explicit `lcwb` input-surface ownership vs derived compatibility flag remains unresolved. | `[DIRECT][E-SPEC-LCWB-01]` | `RATIFIED-W4DR-003 (2026-05-22)` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-22` | `0.1.1` | Ratified HOLD gaps via ARCH13 decisions `W4DR-001/003/011`; current-source consumer authority adopted for `lcwbflg`. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE18. |
