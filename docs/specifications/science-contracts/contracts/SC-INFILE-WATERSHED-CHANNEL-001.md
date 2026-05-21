---
contract_id: SC-INFILE-WATERSHED-CHANNEL-001
title: Watershed Channel Input Parser Contract (.chn)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.1
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-WATERSHED-CHANNEL-001 Watershed Channel Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-CHN-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md` (canonical `.chn` grammar, symbols, policy decisions, and unresolved gaps).
- `[DIRECT][E-SURVEY-CHN-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (`.chn` parser surface provenance).
- `[DIRECT][E-WF-CHN-01]` `/workdir/wepp-forest/src/infile.for`, `/workdir/wepp-forest/src/wshinp.for`, `/workdir/wepp-forest/src/inidat.for`, `/workdir/wepp-forest/src/verchk.for` (legacy version gate, branch behavior, defaults, and sidecar coupling captured by spec).
- `[DIRECT][E-WP-CHN-01]` `/workdir/wepppy/wepppy/nodb/core/wepp.py` and `/workdir/wepppy/wepppy/wepp/management/data/channels.defs` (current generated `.chn` patterns reflected in spec).
- `[INFERENCE][E-PHYS-CHN-01]` Physical/common-sense invariants: positive channel dimensions/roughness domains, declared channel count closure, and complete conditional rating-curve payload when enabled.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for surface `infile-watershed-channel-chn` (`.chn`) and parse-to-runtime handoff for channel-routing/control-section parameter state.

### 1.2 Version/Datver Applicability Matrix

| Case | Input datver | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | `99.1` | Accept. | Canonical modern path. | `[DIRECT][E-SPEC-CHN-01]` |
| B | `94.301 <= datver < 99.1` | Compat candidate only. | Accepted only under explicit compatibility policy flag. | `[DIRECT][E-SPEC-CHN-01]`, `[DIRECT][E-WF-CHN-01]` |
| C | `datver < 94.301` | Reject. | Emit typed `UnsupportedDatver`. | `[DIRECT][E-WF-CHN-01]` |

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
chn_file = datver_line nchan_line ipeak_line lw_line channel_block{nchan} ;

channel_block = comment_1 comment_2 comment_3 ishape_line icntrl_line ienslp_line
                flgout_line geom_line erod_line control_line [rating_curve_line] ;

geom_line = chnz chnnbr ;
erod_line = chnn chnk chntcr chnedm chneds ;
control_line = ctlslp ctlz ctln ;
rating_curve_line = rccoef rcexp rcoset ;
```

`rating_curve_line` is required iff `icntrl == 4`.

### 2.2 Two-Layer Model Contract

- Source model preserves per-channel line sequence (including three comment lines) and branch-specific payload.
- Simulation model normalizes into typed `ChannelDefinition` records with:
  - channel geometry/roughness/erodibility fields,
  - control-section policy fields,
  - optional rating-curve record,
  - explicit compatibility-derived annotations (when strict/compat behaviors diverge).
- Parser does not execute routing computations; it only provides validated typed input state.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `ver` | `header.ver` | `watershed.channel.version.datver` | none | real | 1 | yes | all | none | `channel_datver` |
| `nchan` | `header.nchan` | `watershed.channel.channel_count` | count | int | 1 | yes | all | none | `channel_count` |
| `ipeak` | `header.ipeak` | `watershed.channel.routing.peak_method` | enum | int | 1 | yes | all | none | `runoff_peak_method` |
| `lw` | `header.lw` | `watershed.channel.routing.length_width_ratio` | m/m | real | 1 | yes | all | none | `watershed_length_width_ratio` |
| `comment_1` | `channels[i].comment_1` | `watershed.channel.channels[i].comment_1` | text | string | nchan | yes | all | none | `channel_comment_1` |
| `comment_2` | `channels[i].comment_2` | `watershed.channel.channels[i].comment_2` | text | string | nchan | yes | all | none | `channel_comment_2` |
| `comment_3` | `channels[i].comment_3` | `watershed.channel.channels[i].comment_3` | text | string | nchan | yes | all | none | `channel_comment_3` |
| `ishape` | `channels[i].ishape` | `watershed.channel.channels[i].shape_code` | enum | int | nchan | yes | all | none | `channel_shape_flag` |
| `icntrl` | `channels[i].icntrl` | `watershed.channel.channels[i].control_mode` | enum | int | nchan | yes | all | none | `control_section_flag` |
| `ienslp` | `channels[i].ienslp` | `watershed.channel.channels[i].friction_slope_mode` | enum | int | nchan | yes | all | none | `friction_slope_method_flag` |
| `flgout` | `channels[i].flgout` | `watershed.channel.channels[i].output_flag` | enum | int | nchan | yes | all | none | `channel_output_flag` |
| `chnz` | `channels[i].chnz` | `watershed.channel.channels[i].geom.inverse_side_slope` | m/m | real | nchan | yes | all | none | `channel_inverse_side_slope` |
| `chnnbr` | `channels[i].chnnbr` | `watershed.channel.channels[i].roughness.bare_n` | none | real | nchan | yes | all | none | `channel_manning_n_bare` |
| `chnn` | `channels[i].chnn` | `watershed.channel.channels[i].roughness.total_n` | none | real | nchan | yes | all | none | `channel_manning_n_total` |
| `chnk` | `channels[i].chnk` | `watershed.channel.channels[i].erodibility.k` | s/m | real | nchan | yes | all | none | `channel_erodibility` |
| `chntcr` | `channels[i].chntcr` | `watershed.channel.channels[i].erodibility.tau_c` | N/m^2 | real | nchan | yes | all | none | `channel_critical_shear` |
| `chnedm` | `channels[i].chnedm` | `watershed.channel.channels[i].erodibility.nonerodible_depth_mid` | m | real | nchan | yes | all | none | `channel_nonerodible_depth_mid` |
| `chneds` | `channels[i].chneds` | `watershed.channel.channels[i].erodibility.nonerodible_depth_side` | m | real | nchan | yes | all | none | `channel_nonerodible_depth_side` |
| `ctlslp` | `channels[i].ctlslp` | `watershed.channel.channels[i].control.slope` | m/m | real | nchan | yes | all | none | `control_section_slope` |
| `ctlz` | `channels[i].ctlz` | `watershed.channel.channels[i].control.inverse_side_slope` | m/m | real | nchan | yes | all | none | `control_section_inverse_side_slope` |
| `ctln` | `channels[i].ctln` | `watershed.channel.channels[i].control.manning_n` | none | real | nchan | yes | all | none | `control_section_manning_n` |
| `rccoef` | `channels[i].rating_curve.rccoef` | `watershed.channel.channels[i].rating_curve.coefficient` | coeff | real | subset(nchan, `icntrl=4`) | conditional | all | none | `rating_curve_coefficient` |
| `rcexp` | `channels[i].rating_curve.rcexp` | `watershed.channel.channels[i].rating_curve.exponent` | exponent | real | subset(nchan, `icntrl=4`) | conditional | all | none | `rating_curve_exponent` |
| `rcoset` | `channels[i].rating_curve.rcoset` | `watershed.channel.channels[i].rating_curve.min_depth_m` | m | real | subset(nchan, `icntrl=4`) | conditional | all | none | `rating_curve_min_depth` |
| derived `has_rating_curve` | `icntrl==4` | `watershed.channel.channels[i].rating_curve.enabled` | flag | bool | nchan | yes | all | from `icntrl` | `rating_curve_enabled` |
| derived `control_override_applied` | `icntrl==0` branch | `watershed.channel.channels[i].control.override_mode` | enum | string | nchan | yes | all | derived branch marker | `control_override_mode` |
| derived `sidecar_required` | `ipeak > 2` | `watershed.channel.routing.sidecar_required` | flag | bool | 1 | yes | all | derived from `ipeak` | `sidecar_required` |
| derived `tcr_overlay_present` | `tcr.txt` availability | `watershed.channel.adjustments.tcr_overlay_present` | flag | bool | 1 | yes | all | derived by cross-file sidecar detection | `tcr_overlay_present` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ver` | `header.ver` | `watershed.channel.version` | `input::watershed::channel` | init | immutable | version policy gate | `G-CHN-001` |
| `nchan` | `header.nchan` | `watershed.channel.channel_count` | `input::watershed::channel` | init,watershed | immutable | cross-file count closure vs `.str/.man` | `G-CHN-002`, `G-CHN-010` |
| `ipeak` | `header.ipeak` | `watershed.channel.routing.peak_method` | `input::watershed::channel` | init,watershed,event | immutable | routing-method selection; sidecar `chan.inp` requirement gate | `G-CHN-003`, `G-CHN-011` |
| `lw` | `header.lw` | `watershed.channel.routing.length_width_ratio` | `input::watershed::channel` | init,watershed | immutable | watershed routing scaling | `G-CHN-004` |
| `comment_1` | `channels[*].comment_1` | `watershed.channel.channels[*].comment_1` | `input::watershed::channel` | init | immutable | provenance/diagnostics export | `G-CHN-005` |
| `comment_2` | `channels[*].comment_2` | `watershed.channel.channels[*].comment_2` | `input::watershed::channel` | init | immutable | provenance/diagnostics export | `G-CHN-005` |
| `comment_3` | `channels[*].comment_3` | `watershed.channel.channels[*].comment_3` | `input::watershed::channel` | init | immutable | provenance/diagnostics export | `G-CHN-005` |
| `ishape` | `channels[*].ishape` | `watershed.channel.channels[*].shape_code` | `input::watershed::channel` | init,watershed,event | immutable | channel hydraulics branches | `G-CHN-006` |
| `icntrl` | `channels[*].icntrl` | `watershed.channel.channels[*].control_mode` | `input::watershed::channel` | init,watershed,event | immutable | control-section branch selection | `G-CHN-006`, `G-CHN-007` |
| `ienslp` | `channels[*].ienslp` | `watershed.channel.channels[*].friction_slope_mode` | `input::watershed::channel` | init,watershed,event | immutable | friction-slope method branch | `G-CHN-006` |
| `flgout` | `channels[*].flgout` | `watershed.channel.channels[*].output_flag` | `input::watershed::channel` | init,watershed | immutable | channel output planning | `G-CHN-006` |
| `chnz` | `channels[*].chnz` | `watershed.channel.channels[*].geom.inverse_side_slope` | `input::watershed::channel` | init,watershed,event | immutable | hydraulic geometry calculations | `G-CHN-008` |
| `chnnbr` | `channels[*].chnnbr` | `watershed.channel.channels[*].roughness.bare_n` | `input::watershed::channel` | init,watershed,event | immutable | roughness guard and erosion coupling | `G-CHN-008` |
| `chnn` | `channels[*].chnn` | `watershed.channel.channels[*].roughness.total_n` | `input::watershed::channel` | init,watershed,event | immutable | roughness guard and control override | `G-CHN-008`, `G-CHN-012` |
| `chnk` | `channels[*].chnk` | `watershed.channel.channels[*].erodibility.k` | `input::watershed::channel` | init,watershed,event | immutable | channel erosion routines | `G-CHN-009` |
| `chntcr` | `channels[*].chntcr` | `watershed.channel.channels[*].erodibility.tau_c` | `input::watershed::channel` | init,watershed,event | immutable | channel erosion routines | `G-CHN-009`, `G-CHN-014` |
| `chnedm` | `channels[*].chnedm` | `watershed.channel.channels[*].erodibility.nonerodible_depth_mid` | `input::watershed::channel` | init,watershed,event | immutable | channel erosion routines | `G-CHN-009` |
| `chneds` | `channels[*].chneds` | `watershed.channel.channels[*].erodibility.nonerodible_depth_side` | `input::watershed::channel` | init,watershed,event | immutable | channel erosion routines | `G-CHN-009` |
| `ctlslp` | `channels[*].ctlslp` | `watershed.channel.channels[*].control.slope` | `input::watershed::channel` | init,watershed,event | immutable | control-flow equations | `G-CHN-007`, `G-CHN-012` |
| `ctlz` | `channels[*].ctlz` | `watershed.channel.channels[*].control.inverse_side_slope` | `input::watershed::channel` | init,watershed,event | immutable | control-flow equations | `G-CHN-007`, `G-CHN-012` |
| `ctln` | `channels[*].ctln` | `watershed.channel.channels[*].control.manning_n` | `input::watershed::channel` | init,watershed,event | immutable | control-flow equations | `G-CHN-007`, `G-CHN-012` |
| `rccoef` | `channels[*].rating_curve.rccoef` | `watershed.channel.channels[*].rating_curve.coefficient` | `input::watershed::channel` | init,watershed,event | immutable | rating-curve outlet flows | `G-CHN-013` |
| `rcexp` | `channels[*].rating_curve.rcexp` | `watershed.channel.channels[*].rating_curve.exponent` | `input::watershed::channel` | init,watershed,event | immutable | rating-curve outlet flows | `G-CHN-013` |
| `rcoset` | `channels[*].rating_curve.rcoset` | `watershed.channel.channels[*].rating_curve.min_depth_m` | `input::watershed::channel` | init,watershed,event | immutable | rating-curve outlet flows | `G-CHN-013` |
| derived `has_rating_curve` | `derived.has_rating_curve[*]` | `watershed.channel.channels[*].rating_curve.enabled` | `input::watershed::channel` | init,watershed,event | immutable | rating-curve dispatch branch | `G-CHN-013` |
| derived `control_override_applied` | `derived.control_override_applied[*]` | `watershed.channel.channels[*].control.override_mode` | `input::watershed::channel` | init,watershed,event | immutable | control precedence branch observability | `G-CHN-012` |
| derived `sidecar_required` | `derived.sidecar_required` | `watershed.channel.routing.sidecar_required` | `input::watershed::channel` | init,watershed | immutable | orchestration requirement for `chan.inp` | `G-CHN-011` |
| derived `tcr_overlay_present` | `derived.tcr_overlay_present` | `watershed.channel.adjustments.tcr_overlay_present` | `input::watershed::channel` | init,watershed | immutable | adjustment overlay gating | `G-CHN-014` |

## 5. State Ownership and Mutability

- `input::watershed::channel` owns parsed `.chn` source records and normalized channel-parameter state.
- Parsed per-channel records are immutable after parse success.
- `input::watershed::channel_adjustments` owns optional overlay adjustments (`chan.inp`, `tcr.txt`) as non-mutating overlays layered on top of canonical parsed state.
- Routing solver state (discharge, stage, sediment transport) is mutable in runtime routing modules only.
- Forbidden mutation path: runtime modules mutating canonical parsed channel parameter fields (`chn*`, `ctl*`, `rc*`) in place.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-CHN-001` | Derive `has_rating_curve` from `icntrl == 4`. | per-channel parse | `C-CHN-001` |
| `D-CHN-002` | Apply control-parameter precedence marker for `icntrl == 0` (`ctln <- chnn`, `ctlz <- chnz`, `ctlslp <- slplst`) in compatibility semantics metadata. | per-channel finalize | `C-CHN-002` |
| `D-CHN-003` | Derive sidecar requirement flag when `ipeak > 2` (`chan.inp` coupling). | file finalize | `C-CHN-003` |

Closure hooks:
- `C-CHN-001`: if `icntrl == 4`, rating-curve triple must exist.
- `C-CHN-002`: control-override semantics are explicit and observable in strict/compat policy.
- `C-CHN-003`: `ipeak > 2` sidecar policy enforced without silent fallbacks in strict mode.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `CHN-E-000` | io | missing/unopenable `.chn` file |
| `CHN-E-001` | syntax | numeric parse failure in required fields |
| `CHN-E-002` | syntax | missing record/EOF before declared `nchan` block completes |
| `CHN-E-003` | semantic | unsupported datver |
| `CHN-E-004` | semantic | enum-domain failure (`ipeak`,`ishape`,`icntrl`,`ienslp`,`flgout`) |
| `CHN-E-005` | semantic | invalid physical/range domain for geometry/roughness/erodibility fields |
| `CHN-E-006` | semantic | missing/extra rating-curve record relative to `icntrl` |
| `CHN-E-007` | cross-file | channel count mismatch with `.str` and management channel topology |
| `CHN-E-008` | cross-file | missing/invalid `chan.inp` sidecar under strict policy when `ipeak>2` |
| `CHN-E-009` | runtime-guard | post-parse closure failure for override/compat policy invariants |
| `CHN-W-001` | compat-warning | accepted non-canonical datver in compatibility mode |
| `CHN-W-002` | compat-warning | compatibility fallback used for missing/invalid `chan.inp` under `ipeak>2` |
| `CHN-W-003` | compat-warning | legacy `ishape` normalization applied in compatibility mode |
| `CHN-W-004` | compat-warning | control override (`ctln <- chnn`, `ctlz <- chnz`, `ctlslp <- slplst`) applied for `icntrl=0` |
| `CHN-W-005` | compat-warning | `tcr.txt` adjustment overlay present and applied as non-mutating layer |

No silent parser-side normalization is allowed in strict mode.

## 8. Cross-File Consistency Constraints

1. `nchan` must equal structure-derived channel count and management channel-topology count (`jstruc` channel interpretation). `[DIRECT][E-SPEC-CHN-01]`
2. `ipeak > 2` requires `chan.inp` sidecar policy resolution (`strict`: typed error if missing; `compat`: warning + documented fallback). `[DIRECT][E-SPEC-CHN-01]`
3. Optional `tcr.txt` adjustments to `chntcr` are downstream overrides and must not mutate canonical parsed `.chn` source payload; override should be represented as separate adjustment layer. `[DIRECT][E-SPEC-CHN-01]`, `[INFERENCE][E-WF-CHN-01]`
4. `ctlslp`/`ctlz`/`ctln` branch semantics for `icntrl==0` are cross-surface coupled to slope/channel context and must be explicit in runtime planner metadata. `[DIRECT][E-SPEC-CHN-01]`
5. Compatibility window (`94.301..99.1`) remains gated and explicit; unsupported legacy variants are rejected with typed errors. `[DIRECT][E-SPEC-CHN-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `ver,nchan,ipeak,lw` | `watershed.channel.version`, `watershed.channel.routing` | watershed parser output manifest | canonical keys preserved + aliases (`channel_datver`,`channel_count`,`runoff_peak_method`,`watershed_length_width_ratio`) | no unit conversion |
| per-channel mode fields (`ishape,icntrl,ienslp,flgout`) | `watershed.channel.channels[*].mode` | routing setup boundary | canonical keys + enum alias names | strict/compat policy annotation exported separately |
| per-channel parameters (`chnz,chnnbr,chnn,chnk,chntcr,chnedm,chneds,ctlslp,ctlz,ctln`) | `watershed.channel.channels[*].params` | hydrology/erosion kernel init boundary | canonical parameter names preserved with alias map | control-override metadata indicates effective runtime precedence |
| rating-curve fields (`rccoef,rcexp,rcoset`) | `watershed.channel.channels[*].rating_curve` | control-structure boundary | exported only when `icntrl==4`; includes `rating_curve_enabled` flag | missing conditional payload is hard error |
| derived branch/state flags (`has_rating_curve`,`control_override_applied`,`sidecar_required`,`tcr_overlay_present`) | `watershed.channel.derived` | orchestration and diagnostics boundary | explicit derived fields exported with canonical provenance | required for deterministic sidecar and control behavior |
| optional adjustment overlay (`tcr.txt`) | `watershed.channel.adjustments.tcr_overlay` | channel-erodibility adjustment boundary | overlay records keyed by channel id; canonical `chntcr` in parsed payload remains unchanged | prevents in-place mutation of parsed `.chn` state |
| comments (`comment_1..3`) | `watershed.channel.channels[*].comments` | observability/provenance payload | preserved text fields | non-scientific but externally relevant provenance surface |

## 10. Compatibility Policy

- Strict mode:
  - accept canonical datver `99.1` only unless explicit extension is ratified;
  - enforce exact block grammar and branch completeness;
  - reject missing `chan.inp` when `ipeak > 2`;
  - reject unsupported enum/value domains.
- Compatibility mode:
  - may allow datver range `>=94.301` with explicit `CHN-W-001`;
  - may permit documented legacy sidecar fallback for missing `chan.inp` with explicit `CHN-W-002`;
  - may apply legacy `ishape` normalization behaviors with explicit `CHN-W-003`, never silently;
  - emits `CHN-W-004` when control-override branch is taken (`icntrl==0`);
  - emits `CHN-W-005` when non-mutating `tcr.txt` adjustment overlay is applied.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-CHN-001` | datver allowlist/policy | header parse | `CHN-E-003` |
| `G-CHN-002` | positive `nchan` and count closure preconditions | header parse | `CHN-E-005` |
| `G-CHN-003` | `ipeak` enum and sidecar-coupling eligibility | header parse | `CHN-E-004` |
| `G-CHN-004` | `lw` domain validity | header parse | `CHN-E-005` |
| `G-CHN-005` | mandatory 3-comment-line structure per channel block | block parse | `CHN-E-002` |
| `G-CHN-006` | enum domains (`ishape`,`icntrl`,`ienslp`,`flgout`) | block parse | `CHN-E-004` |
| `G-CHN-007` | control-branch completeness + precedence annotation | block finalize | `CHN-E-006`/`CHN-E-009` |
| `G-CHN-008` | geometry/roughness domains (`chnz`,`chnnbr`,`chnn`) | block parse | `CHN-E-005` |
| `G-CHN-009` | erodibility domains (`chnk`,`chntcr`,`chnedm`,`chneds`) | block parse | `CHN-E-005` |
| `G-CHN-010` | cross-file channel-count closure | cross-surface validator | `CHN-E-007` |
| `G-CHN-011` | `ipeak>2` sidecar policy gate (`chan.inp`) | cross-surface validator | `CHN-E-008` |
| `G-CHN-012` | control-slope override observability for `icntrl==0` | closure hook | `CHN-E-009` |
| `G-CHN-013` | rating-curve conditional arity closure | block parse/finalize | `CHN-E-006` |
| `G-CHN-014` | `tcr.txt` overlay must remain non-mutating and separately owned | adjustment overlay gate | `CHN-E-009` |
| `G-CHN-015` | compatibility acceptance/normalization warnings are emitted on every compat branch | policy gate | `CHN-W-001`/`CHN-W-002`/`CHN-W-003`/`CHN-W-004`/`CHN-W-005` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`ver`, `nchan`, `ipeak`, `lw`, `ishape`, `icntrl`, `ienslp`, `flgout`,
`chnz`, `chnnbr`, `chnn`, `chnk`, `chntcr`, `chnedm`, `chneds`,
`ctlslp`, `ctlz`, `ctln`, `rccoef`, `rcexp`, `rcoset`.

openWEPP runtime names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `CHN-GAP-001` | `flgout` semantics conflict (line-level value vs legacy global override) needs final strict/compat policy ratification. | `[DIRECT][E-SPEC-CHN-01]` | `HOLD` |
| `CHN-GAP-002` | Full compatibility posture for pre-99.1 datver variants remains pending migrated sub-spec/fixtures. | `[DIRECT][E-SPEC-CHN-01]` | `HOLD` |
| `CHN-GAP-003` | `ctlslp` provenance for `icntrl==0` via `slplst` coupling needs explicit runtime boundary documentation in architecture docs. | `[DIRECT][E-SPEC-CHN-01]`, `[INFERENCE][E-WF-CHN-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.1` | Added symbol-level propagation coverage, explicit derived-field exports (`sidecar_required`, `tcr_overlay_present`), and typed compatibility warning outcomes. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE06. |
