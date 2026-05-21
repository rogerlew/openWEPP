---
contract_id: SC-INFILE-SLOPE-001
title: Slope Input Parser Contract (.slp)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.2
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-SLOPE-001 Slope Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-SLP-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md` (canonical openWEPP slope format, datver branches, typed expectations, and gap register).
- `[DIRECT][E-SURVEY-SLP-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (`.slp` parser coverage and legacy/runtime provenance references).
- `[DIRECT][E-WF-SLP-01]` `/home/workdir/wepp-forest/src/infile.for`, `/home/workdir/wepp-forest/src/input.for`, `/home/workdir/wepp-forest/src/profil.for` (legacy slope parse/derived-profile references captured in spec evidence anchors).
- `[DIRECT][E-WP-SLP-01]` `/workdir/wepppy/wepppy/topo/watershed_abstraction/slope_file.py` (wepppy helper parser behavior captured in spec evidence anchors).
- `[DIRECT][E-WP3-SLP-01]` `/workdir/wepppyo3/wepp_interchange/src/mofe.rs` (single-OFE utility parser constraints captured in spec evidence anchors).
- `[DIRECT][E-PERIDOT-SLP-01]` `/workdir/peridot/src/watershed_abstraction/flowpath.rs:217-246` (first-party `datver=2023.3` slope writer behavior).
- `[INFERENCE][E-PHYS-SLP-01]` Physical/common-sense invariants: OFE lengths positive, slope-point distance monotonicity, at least two points per OFE.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for surface `infile-slope-slp` (`.slp`) and normalized topology/profile handoff state used by hillslope simulation setup.

Out of scope: channel `2025.8` bundle parsing and `.slps` flowpath-bundle parsing.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | `datver=97.5` present | Accept. | Canonical modern path. | `[DIRECT][E-SPEC-SLP-01]` |
| B | `datver=2023.3` present | Accept for hillslope `.slp` branch with `azm fwidth elevation` metadata row. | Strict and compatibility modes accept; elevation exported as optional metadata field. | `[DIRECT][E-SPEC-SLP-01]`, `[DIRECT][E-PERIDOT-SLP-01]` |
| C | legacy no datver line | Strict reject, compat optional branch only. | Compatibility branch must be explicit and typed. | `[DIRECT][E-SPEC-SLP-01]` |
| D | explicit older datver >= compatibility threshold | Compat-only candidate. | Requires explicit policy flag. | `[DIRECT][E-SPEC-SLP-01]` |
| E | unsupported datver (< threshold or unknown) | Reject. | Emit typed `UnsupportedDatver`. | `[DIRECT][E-SPEC-SLP-01]` |

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
slope_file = [datver_line] nelem_line ofe_block{nelem} ;

datver_line = real ;
ofe_block = azm_fwidth_line nslpts_slplen_line slope_pairs ;

azm_fwidth_line = azm fwidth [elevation] ;
nslpts_slplen_line = nslpts slplen ;
slope_pairs = (xinput slpinp){nslpts} ;  (* whitespace and/or comma-delimited numeric tokens *)
```

### 2.2 Two-Layer Model Contract

- Source model is file-faithful and preserves raw OFE blocks and pair order.
- Source model preserves datver-conditioned OFE metadata arity:
  - `97.5`: `azm fwidth`
  - `2023.3`: `azm fwidth elevation`
- Simulation model normalizes to typed `SlopeProfile`:
  - per-OFE geometry,
  - optional per-OFE elevation metadata (`Option<f64>`),
  - per-point typed arrays,
  - explicit distance-mode annotation (`absolute` vs `normalized`).

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `slope.version.datver` | none | real | 0..1 | conditional | see Section 1 | none | `slope.datver` |
| `nelem` / `nwsofe` | `header.nelem` | `slope.ofe_count` | count | int | 1 | yes | all | none | `slope.ofe_count` |
| `azm` | `ofe[i].azm` | `slope.ofe[i].aspect_deg` | degrees | real | nelem | yes | all | none | `aspect_deg` |
| `fwidth` | `ofe[i].fwidth` | `slope.ofe[i].width_m` | m | real | nelem | yes | all | none | `width_m` |
| `elevation` (`z0`) | `ofe[i].elevation` | `slope.ofe[i].elevation_m` | m | real | 0..nelem | conditional | required for `datver=2023.3`; absent otherwise | none | `elevation_m` |
| `nslpts` | `ofe[i].nslpts` | `slope.ofe[i].point_count` | count | int | nelem | yes | all | none | `point_count` |
| `slplen` | `ofe[i].slplen` | `slope.ofe[i].length_m` | m | real | nelem | yes | all | none | `length_m` |
| `xinput` | `ofe[i].pairs[j].xinput` | `slope.ofe[i].points[j].x` | m or nondim | real | sum(nslpts) | yes | all | none | `x` |
| `slpinp` | `ofe[i].pairs[j].slpinp` | `slope.ofe[i].points[j].slope_m_per_m` | m/m | real | sum(nslpts) | yes | all | none | `slope_m_per_m` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `slope.version` | `input::slope` | init | immutable | compatibility gate | `G-SLP-001` |
| `nelem` | `header.nelem` | `slope.ofe_count` | `input::slope` | init | immutable | OFE topology closure with management/soil | `G-SLP-002` |
| `azm,fwidth,elevation?` | `ofe[i].geom` | `slope.ofe[i].geom` | `input::slope` | init,daily | immutable | hydrology geometry setup | `G-SLP-003`, `G-SLP-009` |
| `nslpts,slplen` | `ofe[i].shape_hdr` | `slope.ofe[i].shape_hdr` | `input::slope` | init,daily | immutable | profile interpolation setup | `G-SLP-004` |
| `xinput,slpinp` | `ofe[i].points` | `slope.ofe[i].points` | `input::slope` | init,event | immutable | profile normalization and runoff partition coupling | `G-SLP-005`, `G-SLP-006`, `G-SLP-007` |

## 5. State Ownership and Mutability

- `input::slope` owns parse tree and normalized slope profile state.
- Parsed slope profile is immutable after parse success.
- Derived runtime interpolants are mutable only inside designated profile/solver modules and must not mutate parsed canonical point arrays.
- Forbidden mutation path: rewriting parsed point arrays during event runtime.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-SLP-001` | Determine per-OFE distance mode (`absolute` vs `normalized`) from endpoint pattern. | per OFE parse finalize | `C-SLP-001` |
| `D-SLP-002` | Derive endpoint closure expectation (`last x = slplen` or `1.0` by mode). | per OFE parse finalize | `C-SLP-002` |
| `D-SLP-003` | Derive cross-OFE border slope closure check between adjoining OFEs. | full file finalize | `C-SLP-003` |

Closure hooks:
- `C-SLP-001`: no mixed distance-mode tokens within one OFE.
- `C-SLP-002`: minimum 2 points and endpoint presence, with endpoint equality tolerance `abs_tol=1e-6` (`x_end≈slplen` or `x_end≈1.0` by mode).
- `C-SLP-003`: adjoining OFE boundary slope continuity with `abs_tol=1e-6` on border slopes.
- Numeric closure policy: unless otherwise specified, slope parser closure checks use absolute tolerance `1e-6` and `rel_tol=0` to keep mode behavior deterministic.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `SLP-E-000` | io | missing/unopenable `.slp` file (`InputFileMissing` / `InputFileOpenError`) |
| `SLP-E-001` | syntax | token parse failure (`TokenParseError`) |
| `SLP-E-002` | syntax | insufficient records/pairs (`RecordCountError`) |
| `SLP-E-003` | semantic | unsupported datver / missing required datver in strict mode |
| `SLP-E-004` | semantic | invalid counts/domains (`nelem`, `nslpts`, `slplen`, `fwidth`) |
| `SLP-E-010` | semantic | invalid `2023.3` OFE metadata row arity or elevation domain |
| `SLP-E-005` | semantic | mixed distance-mode within OFE (`DistanceModeMixError`) |
| `SLP-E-006` | semantic | endpoint constraint failure (`EndpointConstraintError`) |
| `SLP-E-007` | cross-file | OFE count mismatch with management/soil topology |
| `SLP-E-008` | cross-file | cross-OFE boundary slope mismatch |
| `SLP-E-009` | runtime-guard | post-parse closure hook failure (`InvariantViolation`) |

No silent parser-side correction of malformed slope input is permitted.

## 8. Cross-File Consistency Constraints

1. OFE count from `.slp` must match management topology (`nofe`) and paired OFE-scoped soil surfaces. `[DIRECT][E-SPEC-SLP-01]`
2. Each OFE must contain at least two slope points including top and endpoint by selected distance mode. `[DIRECT][E-SPEC-SLP-01]`
3. Adjoining OFE border slopes must be continuous. `[DIRECT][E-SPEC-SLP-01]`
4. Distance mode cannot mix dimensional and nondimensional encodings within one OFE. `[DIRECT][E-SPEC-SLP-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `datver,nelem` | `slope.version`, `slope.ofe_count` | hillslope/watershed parser output payload | canonical metadata fields preserved (`datver`,`nelem`) | compat/no-datver branch still emits explicit resolved version policy field |
| `azm,fwidth,elevation?,nslpts,slplen` | `slope.ofe[*].geom`, `slope.ofe[*].shape_hdr` | runtime topology/geometry boundary | aliases from Section 3 (`aspect_deg`,`width_m`,`elevation_m`,`point_count`,`length_m`) with unit continuity | `elevation_m` populated only for `datver=2023.3`; no parser-side rescaling |
| `xinput,slpinp` | `slope.ofe[*].points[*]` | profile/interchange/event-runoff boundary | point records preserve canonical symbols and normalized alias fields (`x`,`slope_m_per_m`) | boundary export includes resolved distance-mode annotation |
| derived `distance_mode` and closure flags | `slope.ofe[*].derived` | diagnostics/observability payload | `distance_mode`, `endpoint_closed`, `border_continuity_ok` | exported as derived diagnostics only; canonical input untouched |

## 10. Compatibility Policy

- Strict mode:
  - require canonical datver path;
  - reject legacy no-datver fallback;
  - accept exact `97.5` and exact `2023.3`;
  - reject explicit datver values other than `97.5` or `2023.3`;
  - for `2023.3`, require OFE metadata row arity `azm fwidth elevation`;
  - reject all malformed shape and cross-OFE constraints.
- Compatibility mode:
  - may enable explicit legacy no-datver read path;
  - may accept explicit legacy datver when `datver >= 91.5` (legacy `slpchk` threshold) and compatibility flag is enabled;
  - rejects explicit datver `< 91.5`;
  - still rejects malformed cardinality/shape violations.

wepppy/wepppyo3 utility-only variants are not canonical parser authority unless explicitly ratified.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-SLP-001` | datver policy gate | header parse | `SLP-E-003` |
| `G-SLP-002` | OFE count domain | header parse | `SLP-E-004` |
| `G-SLP-003` | geometry domain (`fwidth > 0`) | OFE header parse | `SLP-E-004` |
| `G-SLP-004` | `nslpts>=2`, `slplen>0` | OFE shape-header parse | `SLP-E-004` |
| `G-SLP-005` | distance mode consistency | pair parse + hook | `SLP-E-005` |
| `G-SLP-006` | endpoint closure and point monotonicity expectations with `abs_tol=1e-6` | closure hooks | `SLP-E-006`/`SLP-E-009` |
| `G-SLP-007` | cross-OFE boundary continuity with `abs_tol=1e-6` | file finalize | `SLP-E-008` |
| `G-SLP-008` | cross-file topology closure | cross-surface validator | `SLP-E-007` |
| `G-SLP-009` | `2023.3` metadata row arity/domain (`azm fwidth elevation`, finite elevation) | datver-conditioned OFE header parse | `SLP-E-010` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative: `datver`, `nelem`/`nwsofe`, `azm`, `fwidth`, `nslpts`, `slplen`, `xinput`, `slpinp`.

openWEPP runtime names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `SLP-GAP-001` | Canonical maximum OFE count policy remains unresolved (`usersum` vs current legacy build gate). | `[DIRECT][E-SPEC-SLP-01]` | `HOLD` |
| `SLP-GAP-002` | Canonical maximum slope-point policy remains unresolved (`usersum` narrative vs legacy compile-time bound). | `[DIRECT][E-SPEC-SLP-01]` | `HOLD` |
| `SLP-GAP-003` | Non-usersum utility extensions (`wepppy` `2023*` variant) remain undispositioned for canonical parser acceptance. | `[DIRECT][E-SPEC-SLP-01]`, `[DIRECT][E-WP-SLP-01]` | `RESOLVED` by INIMPL08 ratification of exact hillslope `datver=2023.3` support |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.2` | Ratified strict-mode support for exact hillslope `datver=2023.3`; added per-OFE `elevation` metadata mapping, comma-delimited pair tolerance, and `G-SLP-009`/`SLP-E-010` coverage. |
| `2026-05-21` | `0.1.1` | Added boundary export mapping, explicit compat datver threshold behavior, missing-file typed error class, and tolerance-aware closure policy. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE04. |
