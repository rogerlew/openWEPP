---
contract_id: SC-INFILE-SNOW-001
title: Snow Sidecar Input Parser Contract (snow.txt)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-SNOW-001 Snow Sidecar Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-SNOW-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/snow.spec.md` (canonical sidecar grammar, symbols, and unresolved policy gaps).
- `[DIRECT][E-SURVEY-SNOW-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and ownership notes).
- `[DIRECT][E-WF-SNOW-01]` `/workdir/wepp-forest/src/infile.for`, `/workdir/wepp-forest/src/cclim.inc`, `/workdir/wepp-forest/src/cwint.inc`, `/workdir/wepp-forest/src/stmtim.for`, `/workdir/wepp-forest/src/snowd.for`, `/workdir/wepp-forest/src/inidat.for` (legacy read/default behavior and winter-state use).
- `[DIRECT][E-WP-SNOW-01]` `/workdir/wepppy/wepppy/nodb/core/wepp.py`, `/workdir/wepppy/wepppy/microservices/rq_engine/wepp_run_payload.py` (modern sidecar emission and payload ingress).
- `[INFERENCE][E-PHYS-SNOW-01]` Process/common-sense invariants: `newsnw` and `ssd` represent density-like controls and must be finite positive values; rain-snow partition threshold `rst` must remain finite.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for optional sidecar surface `infile-snow` (`snow.txt`) and parse-to-runtime handoff of snow process control parameters.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | file absent | Accept optional absence branch. | Apply canonical defaults with explicit provenance output. | `[DIRECT][E-WF-SNOW-01]` |
| B | file present with 3 scalar records | Accept. | Canonical parse path. | `[DIRECT][E-SPEC-SNOW-01]`, `[DIRECT][E-WF-SNOW-01]` |
| C | file present malformed/incomplete | Reject. | Emit typed parse/record closure error. | `[INFERENCE][E-SPEC-SNOW-01]` |
| D | prefixed/version-like variant (extra leading line before canonical triplet) | Reject in strict and compatibility modes. | Emit typed unsupported-format error to prevent semantic record-shift masking. | `[DIRECT][E-SPEC-SNOW-01]`, `[INFERENCE][E-WF-SNOW-01]` |

No datver header is defined for this sidecar.

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
snow_file           = strict_snow_file | compat_snow_file ;
strict_snow_file    = rst_line newsnw_line ssd_line ;
compat_snow_file    = rst_line_compat newsnw_line_compat ssd_line_compat ;

rst_line            = real ;
newsnw_line         = real ;
ssd_line            = real ;

rst_line_compat     = real [trailing_tokens] ;
newsnw_line_compat  = real [trailing_tokens] ;
ssd_line_compat     = real [trailing_tokens] ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves three scalar records in parse order.
- Simulation model normalizes into typed winter-sidecar state:
  - `rst` rain/snow threshold,
  - `newsnw` new-snow density parameter,
  - `ssd` settling-threshold density parameter,
  - derived optional-surface/default provenance markers.
- Parser does not perform snow accumulation/melt computations.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `rst` | `line1.rst` | `snow.options.rst` | deg C | real | 0..1 | conditional | all | default `0.0` when file absent | `snow_opts.rst` |
| `newsnw` | `line2.newsnw` | `snow.options.newsnw` | kg/m^3 | real | 0..1 | conditional | all | default `100.0` when file absent | `snow_opts.newsnw` |
| `ssd` | `line3.ssd` | `snow.options.ssd` | kg/m^3 | real | 0..1 | conditional | all | default `250.0` when file absent | `snow_opts.ssd` |
| derived `snow_file_present` | presence branch | `snow.options.snow_file_present` | flag | bool | 1 | yes | all | derived from sidecar presence | `snow_file_present` |
| derived `defaults_applied` | missing-file branch | `snow.options.defaults_applied` | flag | bool | 1 | yes | all | derived default provenance marker | `defaults_applied` |
| derived `surplus_record_count` | records beyond first three | `snow.options.surplus_record_count` | count | int | 1 | conditional | all | strict expects `0`; compat may preserve >0 with warning | `surplus_record_count` |
| derived `trailing_token_lines` | trailing tokens on canonical lines 1..3 | `snow.options.trailing_token_lines` | line-index set | list<int> | 0..3 | conditional | all | compatibility-only warning provenance map | `trailing_token_lines` |
| derived `prefix_variant_detected` | version/prefix preamble detector | `snow.options.prefix_variant_detected` | flag | bool | 1 | yes | all | true when unsupported leading-line variant detected | `prefix_variant_detected` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `rst` | `records.rst` | `snow.options.rst` | `input::sidecar::snow` | init,event | immutable | rain/snow partition branch (`hrtemp > rst`) | `G-SNOW-001`, `G-SNOW-004` |
| `newsnw` | `records.newsnw` | `snow.options.newsnw` | `input::sidecar::snow` | init,event | immutable | new-snow density assignment (`densnf`) | `G-SNOW-002`, `G-SNOW-004` |
| `ssd` | `records.ssd` | `snow.options.ssd` | `input::sidecar::snow` | init,event | immutable | settling threshold branch (`densgy > ssd`) | `G-SNOW-003`, `G-SNOW-004` |
| derived `snow_file_present` | `derived.file_present` | `snow.options.snow_file_present` | `input::sidecar::snow` | init | immutable | diagnostics/provenance | `G-SNOW-005` |
| derived `defaults_applied` | `derived.defaults_applied` | `snow.options.defaults_applied` | `input::sidecar::snow` | init | immutable | diagnostics/provenance | `G-SNOW-005` |
| derived `surplus_record_count` | `derived.surplus_count` | `snow.options.surplus_record_count` | `input::sidecar::snow` | init | immutable | strict/compat observability | `G-SNOW-006` |
| derived `trailing_token_lines` | `derived.trailing_token_lines` | `snow.options.trailing_token_lines` | `input::sidecar::snow` | init | immutable | strict/compat observability | `G-SNOW-006` |
| derived `prefix_variant_detected` | `derived.prefix_variant_detected` | `snow.options.prefix_variant_detected` | `input::sidecar::snow` | init | immutable | unsupported-format rejection path | `G-SNOW-010` |

## 5. State Ownership and Mutability

- `input::sidecar::snow` owns parsed sidecar values and optional-surface provenance markers.
- Parsed sidecar values are immutable after parse success.
- Runtime winter modules own mutable transient snow-state variables (for example snowpack depth, density evolution, melt fluxes) but may not mutate canonical parsed sidecar parameters.
- Forbidden mutation path: runtime modules rewriting canonical parsed `rst/newsnw/ssd` values in place.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-SNOW-001` | Derive optional-surface presence marker from sidecar existence. | parse preamble | `C-SNOW-001` |
| `D-SNOW-002` | Derive default-provenance marker when missing-file branch sets defaults. | parse preamble | `C-SNOW-002` |
| `D-SNOW-003` | Derive surplus-record count when records exist beyond required three. | parse finalize | `C-SNOW-003` |
| `D-SNOW-004` | Derive per-line trailing-token provenance for canonical lines 1..3. | parse line finalize | `C-SNOW-004` |
| `D-SNOW-005` | Derive prefix-variant detection marker before canonical-line assignment. | parse preamble | `C-SNOW-005` |

Closure hooks:
- `C-SNOW-001`: presence branch must be explicit and deterministic.
- `C-SNOW-002`: defaulting branch must be observable, never silent.
- `C-SNOW-003`: surplus-record policy must be mode-gated (strict error vs compat warning).
- `C-SNOW-004`: trailing-token compatibility acceptance must include per-line provenance markers.
- `C-SNOW-005`: unsupported prefix/version variants must be rejected before compatibility surplus handling.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `SNOW-E-000` | io | file open/read failure when sidecar is present but unreadable |
| `SNOW-E-001` | syntax | token parse failure in required numeric fields |
| `SNOW-E-002` | syntax | missing record closure (`<3` records in present-file path) |
| `SNOW-E-003` | semantic | non-finite numeric values |
| `SNOW-E-004` | semantic | invalid domain (`newsnw<=0` or `ssd<=0`) |
| `SNOW-E-005` | runtime-guard | post-parse closure/branch invariant failure |
| `SNOW-E-006` | syntax | strict-mode surplus record rejection (`>3` records in present-file path) |
| `SNOW-E-007` | syntax | strict-mode trailing-token rejection on canonical lines |
| `SNOW-E-008` | syntax | unsupported prefix/version-like leading-line variant |
| `SNOW-W-001` | compat-warning | missing-file defaults branch applied |
| `SNOW-W-002` | compat-warning | trailing tokens accepted in compatibility mode |
| `SNOW-W-003` | compat-warning | surplus records ignored in compatibility mode |

No silent parser-side fallback is permitted for malformed present-file input in strict mode.

## 8. Cross-File Consistency Constraints

1. `snow.txt` coefficients do not replace management initial snow depth (`snodpy`) authority; both surfaces must be interpreted jointly. (guard `G-SNOW-007`)
2. `rst`/`newsnw`/`ssd` must remain available before winter event-time routines consume hourly climate and snow-state branches. (guard `G-SNOW-008`)
3. Payload ingress aliases (`snow_opts_*`) must map losslessly to canonical symbols. (guard `G-SNOW-009`)

Evidence: `[DIRECT][E-SPEC-SNOW-01]`, `[DIRECT][E-WF-SNOW-01]`, `[DIRECT][E-WP-SNOW-01]`.

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `rst,newsnw,ssd` | `snow.options.*` | winter sidecar boundary payload | canonical symbols + aliases `snow_opts.*` | no unit conversion in parser layer |
| derived presence/default markers | `snow.options.{snow_file_present,defaults_applied}` | diagnostics/provenance boundary | explicit derived fields | required observability for optional-surface behavior |
| derived strict/compat provenance markers | `snow.options.{surplus_record_count,trailing_token_lines,prefix_variant_detected}` | strict/compat diagnostics boundary | explicit derived fields | mode-gated error/warning semantics and anti-masking controls |

## 10. Compatibility Policy

- Strict mode:
  - requires exactly three scalar records when file is present;
  - rejects trailing tokens/surplus records with typed strict errors (`SNOW-E-007`, `SNOW-E-006`);
  - rejects unsupported prefix/version-like variants (`SNOW-E-008`);
  - rejects malformed present-file input;
  - treats missing-file branch as explicit default policy output (not parse error).
- Compatibility mode:
  - allows missing-file defaults with `SNOW-W-001`;
  - allows trailing tokens after primary numeric value with per-line provenance `trailing_token_lines` and `SNOW-W-002`;
  - allows surplus records only when canonical first three lines are valid and no prefix-variant marker is present, emitting `SNOW-W-003`;
  - rejects prefix/version-like variants even in compatibility mode to prevent semantic masking (`SNOW-E-008`).

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-SNOW-001` | line-1 numeric parse/finite domain (`rst`) | parse line 1 | `SNOW-E-001`/`SNOW-E-003` |
| `G-SNOW-002` | line-2 numeric parse/finite domain (`newsnw`) | parse line 2 | `SNOW-E-001`/`SNOW-E-003`/`SNOW-E-004` |
| `G-SNOW-003` | line-3 numeric parse/finite domain (`ssd`) | parse line 3 | `SNOW-E-001`/`SNOW-E-003`/`SNOW-E-004` |
| `G-SNOW-004` | positive density-like domain for `newsnw`,`ssd` | parse finalize | `SNOW-E-004` |
| `G-SNOW-005` | explicit missing-file default provenance | presence/default branch | `SNOW-E-005`/`SNOW-W-001` |
| `G-SNOW-006` | surplus/trailing-token policy branch | parse finalize/policy gate | strict: `SNOW-E-006`/`SNOW-E-007`; compat: `SNOW-W-002`/`SNOW-W-003` |
| `G-SNOW-007` | management initial-state coupling (`snodpy` authority remains external to `snow.txt`) | cross-surface validator | `SNOW-E-005` |
| `G-SNOW-008` | pre-winter availability of parsed snow controls before event-time routines | lifecycle validator | `SNOW-E-005` |
| `G-SNOW-009` | alias-lossless mapping (`snow_opts_*` to canonical symbols) | boundary validator | `SNOW-E-005` |
| `G-SNOW-010` | unsupported prefix/version-like leading-line detection | preamble parse | `SNOW-E-008` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`rst`, `newsnw`, `ssd`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `SNOW-GAP-001` | Usersum lacks a dedicated `snow.txt` format table; authority relies on legacy source behavior and modern sidecar practice. | `[DIRECT][E-SPEC-SNOW-01]`, `[DIRECT][E-WF-SNOW-01]` | `HOLD` |
| `SNOW-GAP-002` | Unit-label conflict between wepppy comments and legacy density semantics needs canonical unit ratification. | `[DIRECT][E-SPEC-SNOW-01]`, `[DIRECT][E-WF-SNOW-01]`, `[DIRECT][E-WP-SNOW-01]` | `HOLD` |
| `SNOW-GAP-003` | Final bounds policy for extreme-but-positive `rst/newsnw/ssd` values requires fixture-backed disposition. | `[DIRECT][E-SPEC-SNOW-01]`, `[DIRECT][E-WP-SNOW-01]` | `HOLD` |
| `SNOW-GAP-004` | Compatibility treatment of surplus records is provisional and intentionally excludes prefix/version-like forms to avoid semantic masking. | `[DIRECT][E-SPEC-SNOW-01]`, `[INFERENCE][E-WF-SNOW-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE11. |
