---
contract_id: SC-INFILE-FROST-001
title: Frost Sidecar Input Parser Contract (frost.txt)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-FROST-001 Frost Sidecar Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-FROST-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md` (canonical sidecar grammar, symbols, bounds/default behavior, and open gaps).
- `[DIRECT][E-SURVEY-FROST-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and ownership notes).
- `[DIRECT][E-WF-FROST-01]` `/workdir/wepp-forest/src/infile.for`, `/workdir/wepp-forest/src/cflgfs.inc`, `/workdir/wepp-forest/src/cwint.inc`, `/workdir/wepp-forest/src/getfreezecond.for` (legacy parse/default/clamp behavior and runtime class mapping).
- `[DIRECT][E-WP-FROST-01]` `/workdir/wepppy/wepppy/nodb/core/wepp.py`, `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py`, `/workdir/wepppy/wepppy/microservices/rq_engine/wepp_run_payload.py` (modern payload ingress and sidecar emission).
- `[INFERENCE][E-PHYS-FROST-01]` Process/common-sense invariants: layer-count controls must be positive bounded integers; conductivity multipliers/factors must be finite and non-negative within policy bounds.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for optional sidecar surface `infile-frost` (`frost.txt`) and parse-to-runtime handoff of freeze/thaw process controls and conductivity adjustment factors.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | file absent | Accept optional absence branch. | Apply canonical defaults with explicit provenance marker. | `[DIRECT][E-WF-FROST-01]` |
| B | line1 + line2 valid | Accept. | Canonical parse path. | `[DIRECT][E-SPEC-FROST-01]` |
| C | line1-only file | Strict reject. Compat branch allowed. | Strict emits record-count error; compat defaults/clamps line2 coefficients with warning. | `[DIRECT][E-WF-FROST-01]`, `[INFERENCE][E-SPEC-FROST-01]` |
| D | malformed line1 | Reject. | Emit typed parse/arity error. | `[DIRECT][E-WF-FROST-01]` |
| E | prefixed/version-like leading-line variant | Policy unresolved (`FROST-GAP-002`), but explicit taxonomy/guard path is required. | Provisional behavior: reject as unsupported format (`FROST-E-006`) until governance disposition closes gap. | `[DIRECT][E-SPEC-FROST-01]`, `[DIRECT][E-WF-FROST-01]` |

No datver header is defined for this sidecar.

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
frost_file         = strict_frost_file | compat_frost_file ;
strict_frost_file  = line1 line2 ;
compat_frost_file  = line1 [line2] ;
line1         = wintRed fineTop fineBot ;
line2         = ksnowf kresf ksoilf kfactor1 kfactor2 kfactor3 ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves line1 controls and mode-gated line2 coefficient tuple as parsed.
- Simulation model normalizes to typed frost options:
  - structural controls (`wintRed`, `fineTop`, `fineBot`),
  - conductivity coefficients (`ksnowf`, `kresf`, `ksoilf`, `kfactor(1..3)`),
  - derived provenance markers (line2 present/defaulted, clamp usage).
- Parser does not execute freeze-condition selection or conductance kernels.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `wintRed` | `line1.wintRed` | `frost.options.wintRed` | flag | int | 0..1 | conditional | all | default `1` when file absent | `frost_opts.wintRed` |
| `fineTop` | `line1.fineTop` | `frost.options.fineTop` | count | int | 0..1 | conditional | all | default `10` when file absent | `frost_opts.fineTop` |
| `fineBot` | `line1.fineBot` | `frost.options.fineBot` | count | int | 0..1 | conditional | all | default `10` when file absent | `frost_opts.fineBot` |
| `ksnowf` | `line2.ksnowf` | `frost.options.ksnowf` | multiplier | real | 0..1 | conditional | all | default `1.0` when absent/compat-defaulted | `frost_opts.ksnowf` |
| `kresf` | `line2.kresf` | `frost.options.kresf` | multiplier | real | 0..1 | conditional | all | default `1.0` when absent/compat-defaulted | `frost_opts.kresf` |
| `ksoilf` | `line2.ksoilf` | `frost.options.ksoilf` | multiplier | real | 0..1 | conditional | all | default `1.0` when absent/compat-defaulted | `frost_opts.ksoilf` |
| `kfactor(1)` | `line2.kfactor1` | `frost.options.kfactor1` | factor | real | 0..1 | conditional | all | default `0.00001` when absent/compat-defaulted; class meaning unresolved (slot index only) | `frost_opts.kfactor1` |
| `kfactor(2)` | `line2.kfactor2` | `frost.options.kfactor2` | factor | real | 0..1 | conditional | all | default `0.00001` when absent/compat-defaulted; class meaning unresolved (slot index only) | `frost_opts.kfactor2` |
| `kfactor(3)` | `line2.kfactor3` | `frost.options.kfactor3` | factor | real | 0..1 | conditional | all | default `0.5` when absent/compat-defaulted; class meaning unresolved (slot index only) | `frost_opts.kfactor3` |
| derived `frost_file_present` | presence branch | `frost.options.frost_file_present` | flag | bool | 1 | yes | all | derived from sidecar presence | `frost_file_present` |
| derived `line2_present` | line2 parse branch | `frost.options.line2_present` | flag | bool | 1 | yes | all | derived from line2 availability | `line2_present` |
| derived `legacy_clamp_applied` | post-parse clamp/default branch | `frost.options.legacy_clamp_applied` | flag | bool | 1 | conditional | all | derived true when compat clamp/default path alters parsed values | `legacy_clamp_applied` |
| derived `legacy_clamp_fields` | field-level clamp/default provenance | `frost.options.legacy_clamp_fields` | field-name set | list<string> | 0..9 | conditional | all | list of canonical symbols changed by compat clamp/default branch | `legacy_clamp_fields` |
| derived `prefix_variant_detected` | preamble variant detector | `frost.options.prefix_variant_detected` | flag | bool | 1 | yes | all | true when unsupported prefix/version-like line is detected | `prefix_variant_detected` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `wintRed` | `line1.wintRed` | `frost.options.wintRed` | `input::sidecar::frost` | init,daily,event | immutable | frost redistribution branch | `G-FROST-001`, `G-FROST-006` |
| `fineTop` | `line1.fineTop` | `frost.options.fineTop` | `input::sidecar::frost` | init,daily,event | immutable | freeze/thaw layer discretization | `G-FROST-001`, `G-FROST-006` |
| `fineBot` | `line1.fineBot` | `frost.options.fineBot` | `input::sidecar::frost` | init,daily,event | immutable | freeze/thaw layer discretization | `G-FROST-001`, `G-FROST-006` |
| `ksnowf` | `line2.ksnowf` | `frost.options.ksnowf` | `input::sidecar::frost` | init,daily,event | immutable | conductivity adjustment selection | `G-FROST-002`, `G-FROST-006` |
| `kresf` | `line2.kresf` | `frost.options.kresf` | `input::sidecar::frost` | init,daily,event | immutable | conductivity adjustment selection | `G-FROST-002`, `G-FROST-006` |
| `ksoilf` | `line2.ksoilf` | `frost.options.ksoilf` | `input::sidecar::frost` | init,daily,event | immutable | conductivity adjustment selection | `G-FROST-002`, `G-FROST-006` |
| `kfactor(1)` | `line2.kfactor1` | `frost.options.kfactor1` | `input::sidecar::frost` | init,daily,event | immutable | conductivity lower-bound slot 1 (class label unresolved) | `G-FROST-003`, `G-FROST-006` |
| `kfactor(2)` | `line2.kfactor2` | `frost.options.kfactor2` | `input::sidecar::frost` | init,daily,event | immutable | conductivity lower-bound slot 2 (class label unresolved) | `G-FROST-003`, `G-FROST-006` |
| `kfactor(3)` | `line2.kfactor3` | `frost.options.kfactor3` | `input::sidecar::frost` | init,daily,event | immutable | conductivity lower-bound slot 3 (class label unresolved) | `G-FROST-003`, `G-FROST-006` |
| derived `frost_file_present` | `derived.file_present` | `frost.options.frost_file_present` | `input::sidecar::frost` | init | immutable | provenance/diagnostics | `G-FROST-004` |
| derived `line2_present` | `derived.line2_present` | `frost.options.line2_present` | `input::sidecar::frost` | init | immutable | provenance/diagnostics | `G-FROST-005` |
| derived `legacy_clamp_applied` | `derived.clamp_applied` | `frost.options.legacy_clamp_applied` | `input::sidecar::frost` | init | immutable | strict/compat observability | `G-FROST-006`, `G-FROST-007` |
| derived `legacy_clamp_fields` | `derived.clamp_fields` | `frost.options.legacy_clamp_fields` | `input::sidecar::frost` | init | immutable | field-level compat observability | `G-FROST-007` |
| derived `prefix_variant_detected` | `derived.prefix_variant_detected` | `frost.options.prefix_variant_detected` | `input::sidecar::frost` | init | immutable | unsupported-format rejection path | `G-FROST-008` |

## 5. State Ownership and Mutability

- `input::sidecar::frost` owns parsed frost sidecar values and compatibility provenance markers.
- Parsed sidecar values are immutable after parse success.
- Runtime winter/freezing modules own mutable transient frost/snow/temperature state but may not mutate canonical parsed sidecar values.
- Forbidden mutation path: runtime modules rewriting parsed `wintRed/fineTop/fineBot/ksnowf/kresf/ksoilf/kfactor*` values in place.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-FROST-001` | Derive sidecar presence marker from file existence. | parse preamble | `C-FROST-001` |
| `D-FROST-002` | Derive line2-presence marker from optional second-line parse branch. | parse line2 branch | `C-FROST-002` |
| `D-FROST-003` | Derive clamp/default provenance marker when compatibility branch alters parsed/defaulted values. | parse finalize | `C-FROST-003` |
| `D-FROST-004` | Derive field-level clamp/default provenance set for compatibility normalization. | parse finalize | `C-FROST-004` |
| `D-FROST-005` | Derive prefix/version-like variant detector before canonical line assignment. | parse preamble | `C-FROST-005` |

Closure hooks:
- `C-FROST-001`: missing-file branch must be explicit and observable.
- `C-FROST-002`: line2 strict/compat branch must be explicit and mode-gated.
- `C-FROST-003`: compatibility clamp/default behavior must be observable; no silent normalization in strict mode.
- `C-FROST-004`: field-level clamp/default provenance must be exported per canonical symbol.
- `C-FROST-005`: unresolved prefix/version policy must still route through explicit typed taxonomy/guard behavior.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `FROST-E-000` | io | sidecar open/read failure when present |
| `FROST-E-001` | syntax | line1 parse failure or arity mismatch |
| `FROST-E-002` | syntax | line2 parse failure/arity mismatch in strict mode |
| `FROST-E-003` | semantic | non-finite numeric values |
| `FROST-E-004` | semantic | out-of-range values in strict mode |
| `FROST-E-005` | runtime-guard | post-parse closure/branch invariant failure |
| `FROST-E-006` | syntax | prefixed/version-like leading-line variant rejected (provisional while `FROST-GAP-002` remains open) |
| `FROST-W-001` | compat-warning | missing-file defaults branch applied |
| `FROST-W-002` | compat-warning | line2 missing/defaulted in compatibility mode |
| `FROST-W-003` | compat-warning | legacy clamp/default normalization applied in compatibility mode |

No silent parse-failure fallback is permitted in strict mode.

## 8. Cross-File Consistency Constraints

1. Frost options must be available before freeze-condition selection and conductivity-class branching routines.
2. `kfactor(1..3)` are treated as slot indices with unresolved class-label semantics; no deterministic class naming is allowed until `FROST-GAP-001` closure.
3. Orchestration alias fields (`frost_opts_*`) must map losslessly to canonical symbols.

Evidence: `[DIRECT][E-SPEC-FROST-01]`, `[DIRECT][E-WF-FROST-01]`, `[DIRECT][E-WP-FROST-01]`.

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| line1 symbols (`wintRed,fineTop,fineBot`) | `frost.options.line1` | frost-sidecar parser output boundary | canonical symbols + aliases `frost_opts.{wintRed,fineTop,fineBot}` | structure controls |
| line2 symbols (`ksnowf,kresf,ksoilf,kfactor(1..3)`) | `frost.options.line2` | frost-sidecar parser output boundary | canonical symbols + aliases `frost_opts.{ksnowf,kresf,ksoilf,kfactor1,kfactor2,kfactor3}` | conductivity controls |
| derived branch markers | `frost.options.{frost_file_present,line2_present,legacy_clamp_applied}` | diagnostics/provenance boundary | explicit derived fields | required strict/compat observability |

## 10. Compatibility Policy

- Strict mode:
  - requires valid line1 and line2 payload when file is present;
  - rejects missing line2, malformed line2, and out-of-range values;
  - rejects prefixed/version-like variants (`FROST-E-006`) while policy remains unresolved;
  - does not apply legacy clamp/default mutation for invalid present-file values.
- Compatibility mode:
  - allows missing-file defaults with `FROST-W-001`;
  - allows line2-missing branch with defaulted line2 coefficients and `FROST-W-002`;
  - still rejects prefixed/version-like variants (`FROST-E-006`) to avoid silent semantic shifts;
  - allows legacy clamp/default normalization with explicit `FROST-W-003`.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-FROST-001` | line1 arity/domain (`wintRed,fineTop,fineBot`) | parse line1 | `FROST-E-001`/`FROST-E-004` |
| `G-FROST-002` | line2 arity/domain (`ksnowf,kresf,ksoilf`) | parse line2 | `FROST-E-002`/`FROST-E-004` |
| `G-FROST-003` | line2 arity/domain (`kfactor1..3`) | parse line2 | `FROST-E-002`/`FROST-E-004` |
| `G-FROST-004` | missing-file default provenance marker | preamble branch | `FROST-E-005`/`FROST-W-001` |
| `G-FROST-005` | line2 strict/compat branch policy | line2 branch | strict missing/arity failure: `FROST-E-002`; compat defaulted branch: `FROST-W-002` |
| `G-FROST-006` | strict range/finite checks for all numeric controls | parse finalize | `FROST-E-003`/`FROST-E-004` |
| `G-FROST-007` | compatibility clamp/default observability (file + field-level) | policy gate | `FROST-W-003` |
| `G-FROST-008` | prefix/version-like variant detection and rejection | preamble parse | `FROST-E-006` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`wintRed`, `fineTop`, `fineBot`, `ksnowf`, `kresf`, `ksoilf`, `kfactor(1)`, `kfactor(2)`, `kfactor(3)`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `FROST-GAP-001` | Legacy comment-level `kfactor(1..3)` class mapping descriptions conflict across sources; treat indices as unlabeled slots and defer class semantics to governance disposition. | `[DIRECT][E-SPEC-FROST-01]`, `[DIRECT][E-WF-FROST-01]` | `HOLD` |
| `FROST-GAP-002` | `frost.txt` has no datver line; prefixed-variant policy remains unresolved. Contract currently uses provisional reject (`FROST-E-006`) until policy is ratified. | `[DIRECT][E-SPEC-FROST-01]`, `[DIRECT][E-WF-FROST-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE12. |
