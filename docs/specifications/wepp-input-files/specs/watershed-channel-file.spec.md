# Watershed Channel Input File Specification

## 1. Header metadata
- `spec_id`: `SPEC-INFILE-WATERSHED-CHANNEL-CHN-001`
- `surface_id`: `infile-watershed-channel-chn`
- `title`: `WEPP Watershed Channel File (.chn)`
- `status`: `draft-hold`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-21T00:00:00Z`
- `evidence_mode`: `Static`

[DIRECT] This specification targets the watershed channel input file described as the “Watershed channel file” and “Table 25. Channel file description” in `usersum2024`.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7227-7277`.

## 2. Surface scope and applicability
- File extension/surface: `.chn` (`infile-watershed-channel-chn`). [DIRECT] Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7227-7232`.
- Applicability: watershed/channel routing runs (`ivers=3` lineage in legacy WEPP flow). [DIRECT] Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:6790-6799`, `/workdir/wepp-forest/src/infile.for:402-427`.
- Not applicable: hillslope-only runs. [INFERENCE] Evidence: `/workdir/wepp-forest/src/infile.for:402-427`.

[DIRECT] The watershed channel file contains routing method choice, channel shape/hydraulic parameters, and control structure parameters for each channel in increasing channel ID order.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7228-7232`.

[DIRECT] Legacy read path opens unit 18 as watershed channel data and then validates/loads it via `infile.for` and `wshinp.for`.
Evidence: `/workdir/wepp-forest/src/infile.for:402-427`, `/workdir/wepp-forest/src/wshinp.for:351-404`.

[INFERENCE] Hillslope-only run assemblies do not consume `.chn` because this surface is only invoked through watershed structure/channel loading paths.
Evidence: `/workdir/wepp-forest/src/infile.for:402-427`.

## 3. Version/datver applicability matrix

| datver | Provenance | Applicability | Disposition |
|---|---|---|---|
| `99.1` | usersum Table 25 line 1 [DIRECT] | Canonical format for this spec | `active-target` |
| `>=94.301` | legacy `verchk` threshold via `chnchk` [DIRECT] | Legacy compatibility window accepted by WEPP-Forest loader | `legacy-window-hold` |
| `<94.301` | legacy `verchk` reject [DIRECT] | Incompatible in legacy loader | `reject` |

[DIRECT] `usersum2024` names line 1 as version control number `99.1` (`ver`).
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7233-7243`.

[DIRECT] Legacy initializes `chnchk = 94.301` and calls `verchk` against the channel file datver.
Evidence: `/workdir/wepp-forest/src/inidat.for:1160-1163`, `/workdir/wepp-forest/src/infile.for:411-421`, `/workdir/wepp-forest/src/verchk.for:19-31`.

[INFERENCE] For openWEPP canonical authoring, emit/expect `99.1` for forward compatibility; treat pre-99.1 forms as migration scope until format deltas are explicitly dispositioned.

## 4. Record grammar and line-by-line format definition

### 4.1 File-level grammar (canonical)

```text
Line 1:  ver
Line 2:  nchan
Line 3:  ipeak
Line 4:  lw
Repeat for i in 1..nchan:
  Line 5:  comment_1_i
  Line 6:  comment_2_i
  Line 7:  comment_3_i
  Line 8:  ishape_i
  Line 9:  icntrl_i
  Line10:  ienslp_i
  Line11:  flgout_i
  Line12:  chnz_i chnnbr_i
  Line13:  chnn_i chnk_i chntcr_i chnedm_i chneds_i
  Line14:  ctlslp_i ctlz_i ctln_i
  Conditional Line15 (only when icntrl_i == 4):
           rccoef_i rcexp_i rcoset_i
```

[DIRECT] Table 25 defines lines 1-15 and the `Line 15` conditional branch for rating-curve control.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7233-7310`.

### 4.2 Legacy loader sequence alignment

[DIRECT] Legacy read order matches the canonical line block sequence after initial `eatcom`+datver handling, including unconditional read of three comment lines and conditional read of line 15 when `icntrl==4`.
Evidence: `/workdir/wepp-forest/src/wshinp.for:353-439`.

## 5. Field dictionary and alias mapping

### 5.1 Canonical field dictionary

| Symbol (canonical) | Line | Units | Type | Cardinality | Required | openWEPP boundary alias |
|---|---|---|---|---|---|---|
| `ver` | 1 | unitless | real | 1/file | yes | `datver` |
| `nchan` | 2 | count | int | 1/file | yes | `channel_count` |
| `ipeak` | 3 | enum | int | 1/file | yes | `runoff_peak_method` |
| `lw` | 4 | m/m | real | 1/file | yes | `watershed_length_width_ratio` |
| `comment_1` | 5 | text | string | 1/channel | yes | `channel_comment_1` |
| `comment_2` | 6 | text | string | 1/channel | yes | `channel_comment_2` |
| `comment_3` | 7 | text | string | 1/channel | yes | `channel_comment_3` |
| `ishape` | 8 | enum | int | 1/channel | yes | `channel_shape_flag` |
| `icntrl` | 9 | enum | int | 1/channel | yes | `control_section_flag` |
| `ienslp` | 10 | enum | int | 1/channel | yes | `friction_slope_method_flag` |
| `flgout` | 11 | enum | int | 1/channel | yes | `channel_output_flag` |
| `chnz` | 12a | m/m | real | 1/channel | yes | `channel_inverse_side_slope` |
| `chnnbr` | 12b | unitless | real | 1/channel | yes | `channel_manning_n_bare` |
| `chnn` | 13a | unitless | real | 1/channel | yes | `channel_manning_n_total` |
| `chnk` | 13b | s/m | real | 1/channel | yes | `channel_erodibility` |
| `chntcr` | 13c | N/m^2 | real | 1/channel | yes | `channel_critical_shear` |
| `chnedm` | 13d | m | real | 1/channel | yes | `channel_nonerodible_depth_mid` |
| `chneds` | 13e | m | real | 1/channel | yes | `channel_nonerodible_depth_side` |
| `ctlslp` | 14a | m/m | real | 1/channel | yes | `control_section_slope` |
| `ctlz` | 14b | m/m | real | 1/channel | yes | `control_section_inverse_side_slope` |
| `ctln` | 14c | unitless | real | 1/channel | yes | `control_section_manning_n` |
| `rccoef` | 15a | coefficient | real | 0..1/channel | conditional | `rating_curve_coefficient` |
| `rcexp` | 15b | exponent | real | 0..1/channel | conditional | `rating_curve_exponent` |
| `rcoset` | 15c | m | real | 0..1/channel | conditional | `rating_curve_min_depth` |

[DIRECT] Symbols, units, and semantics for line 1-15 fields come from Table 25 text.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7233-7310`.

[DIRECT] Legacy variable names in channel common blocks and read statements align with the symbol set above (`ipeak`, `ishape`, `icntrl`, `ienslp`, `flgout`, `chnz`, `chnnbr`, `chnn`, `chnk`, `chntcr`, `chnedm`, `chneds`, `ctlslp`, `ctlz`, `ctln`, `rccoef`, `rcexp`, `rcoset`).
Evidence: `/workdir/wepp-forest/src/wshinp.for:369-439`, `/workdir/wepp-forest/src/cchtrl.inc:7-29`, `/workdir/wepp-forest/src/cchpar.inc:7-30`, `/workdir/wepp-forest/src/cchvar.inc:7-26`.

### 5.2 Enum dictionary

- `ipeak`:
  - `1` modified EPIC.
  - `2` CREAMS.
  - `3` Kinematic Wave.
  - `4` Muskingum-Cunge (constant).
  - `5` Muskingum-Cunge (modified variable).
- `ishape`:
  - `1` triangular.
  - `2` naturally eroded (usersum declaration).
- `icntrl`:
  - `0` no control.
  - `1` critical flow.
  - `2` normal flow.
  - `3` normal flow with different roughness.
  - `4` rating curve.
- `ienslp`:
  - `1` CREAMS friction slope.
  - `2` friction slope equals bed slope.
- `flgout`:
  - `0` historically described as overridden by general output flags.

[DIRECT] Enum values are directly listed in Table 25.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7244-7300`.

### 5.3 Alias mapping policy

[INFERENCE] openWEPP internal/state names may use Rust-style snake_case, but canonical symbol columns and equation references remain WEPP/legacy names. Alias mappings in this specification are normative for boundary translation and traceability.

## 6. Conditional branches and optional sections

- Branch A: `Line 15` is present only when `icntrl == 4` (rating curve).
- Branch B: `Line 14` must still be present even when values are not used due to `icntrl` mode.
- Branch C: For `icntrl == 0`, legacy overwrites control parameters from channel/slope context.
- Branch D: For `ipeak > 2`, legacy attempts to read sidecar `chan.inp` for routing output/step control.

[DIRECT] Usersum states line 15 conditionality and line 14 mandatory presence even when overridden.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7288-7335`.

[DIRECT] Legacy behavior for branch C and D:
- `icntrl == 0`: `ctln <- chnn`, `ctlz <- chnz`, `ctlslp <- slplst`.
- `ipeak > 2`: reads `chan.inp` with fallbacks when absent/error.
Evidence: `/workdir/wepp-forest/src/wshinp.for:428-487`.

## 7. Cross-file consistency constraints and coupling dependencies

1. Channel-count closure:
- `nchan` in `.chn` must match both structure-derived channel count and management-derived `jstruc`.

2. Width/length source coupling:
- `chnlen` and `chnwid` used by routing are sourced from channel slope/topography context (`slplen`, `fwidth`), not directly from `.chn`.

3. Control-slope override coupling:
- For `icntrl == 0`, legacy assigns control slope from `slplst` (downstream slope from slope processing path).

4. Optional sidecar coupling:
- `chan.inp` is conditionally consumed when `ipeak > 2`.
- `tcr.txt` can post-adjust `chntcr` values after `.chn` parse.

5. Roughness closure guard:
- Legacy clamps `chnn` upward when `chnn < chnnbr`.

[DIRECT] Constraint evidence:
- count closure: `/workdir/wepp-forest/src/wshinp.for:361-367`
- width/length coupling: `/workdir/wepp-forest/src/wshinp.for:397-402`
- control-slope override: `/workdir/wepp-forest/src/wshinp.for:428-431`, `/workdir/wepp-forest/src/cchtrl.inc:28`
- sidecar coupling: `/workdir/wepp-forest/src/wshinp.for:183-194`, `/workdir/wepp-forest/src/wshinp.for:469-487`
- roughness clamp: `/workdir/wepp-forest/src/wshinp.for:415-417`

[DIRECT] WEPPpy generates `pw0.chn` with datver `99.1` and writes `ipeak` (`4` for Muskingum-Cunge default, else `2`) plus one channel block per channel template.
Evidence: `/workdir/wepppy/wepppy/nodb/core/wepp.py:2482-2506`, `/workdir/wepppy/wepppy/wepp/management/data/channels.defs:85-95`.

[INFERENCE] Static inspection of `/workdir/wepppyo3` in this authoring pass did not surface a dedicated `.chn` input parser; inspected crates in scope reference channel loss interchange outputs instead.
Evidence: `/workdir/wepppyo3/wepp_interchange/src/loss.rs:294-298`, `/workdir/wepppyo3/wepp_interchange/src/loss.rs:390-407`.

[INFERENCE] openWEPP parser/data-model contracts should explicitly represent these couplings (especially count closure and sidecar-triggered behavior) rather than treating `.chn` in isolation.

## 8. Defaulting and missing-file behavior (typed error expectations)

### 8.1 Required parse-time failures
- Missing `.chn` file -> typed error (`MissingFile`).
- Premature EOF before required lines per `nchan` -> typed error (`UnexpectedEof`).
- Non-numeric token where numeric required -> typed error (`ParseNumber`).
- `icntrl == 4` without line 15 triple -> typed error (`MissingRatingCurveLine`).
- Cross-file `nchan` mismatch against structure/management surfaces -> typed error (`ChannelCountMismatch`).

[INFERENCE] Error IDs above are openWEPP boundary names; canonical semantics derive from mandatory line structure and legacy closure checks.

### 8.2 Legacy-derived defaulting behaviors to preserve (or explicitly deprecate)
- `flgout` line value may be overridden by global output mode (`watsum`).
- If `chan.inp` is unavailable under `ipeak > 2`, legacy defaults to zeroed/disabled routing-output controls.

[DIRECT] Legacy default/override evidence: `/workdir/wepp-forest/src/wshinp.for:394-397`, `/workdir/wepp-forest/src/wshinp.for:472-485`.

[INFERENCE] openWEPP must disposition whether to preserve these overrides exactly or treat them as explicit warnings/errors in strict mode.

### 8.3 `ipeak > 2` sidecar (`chan.inp`) normative handling
- Strict mode (`compat_mode=false`):
  - Missing or unreadable `chan.inp` when any channel uses `ipeak > 2` -> `ChannelSidecarMissing { sidecar: chan.inp }`.
  - Parse failure in `chan.inp` -> `ChannelSidecarParseError { sidecar: chan.inp }`.
- Compatibility mode (`compat_mode=true`):
  - Missing/unreadable `chan.inp` -> permit legacy fallback (`ichplt=0`, `it=null`, `iwind=0`) and emit `ChannelSidecarCompatibilityFallbackWarning`.
  - Parse failure -> apply same fallback and emit warning; never silently swallow the event.
- [DIRECT] Legacy fallback behavior and zeroed routing controls are observed in `wshinp.for`.
Evidence: `/workdir/wepp-forest/src/wshinp.for:469-485`.

### 8.4 Resolved high-severity policy decisions
- `CHN-POL-001` (`ishape` domain reconciliation):
  - strict mode accepts authoring-domain values `{1,2}` only (`1` triangular, `2` naturally eroded); any other token is `InputDomainError { field: ishape, allowed: [1,2] }`.
  - compatibility mode accepts legacy-expanded values and normalizes to legacy runtime class behavior observed in `wshinp.for` (`ishape >= 2` coerced to naturally eroded runtime class code path under default channel-output mode) with `ChannelShapeCompatibilityNormalizationWarning`.
  - [DIRECT] Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7275-7277`, `/workdir/wepp-forest/src/wshinp.for:386-390`, `/workdir/wepp-forest/src/chnpar.for:83-85`.
- `CHN-POL-002` (control-section roughness precedence reconciliation):
  - canonical runtime rule is legacy precedence: when `icntrl == 0`, override control roughness with `ctln <- chnn` after legacy roughness guard (`chnn >= chnnbr`) is applied.
  - line 14c remains syntactically required but semantically ignored in this branch; compat mode emits `ControlSectionOverrideAppliedWarning` for observability.
  - [DIRECT] Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7316-7340`, `/workdir/wepp-forest/src/wshinp.for:415-417`, `/workdir/wepp-forest/src/wshinp.for:428-431`.

## 9. Example snippets

### 9.1 Minimal valid example (`nchan=1`, no rating curve)

```text
99.1
1
2
1.500000
channel 1 comment a
channel 1 comment b
channel 1 comment c
1
0
1
0
19.99 0.03
0.04 1.0E-6 19.0 900.0 0.0001
0.02 4.0 0.04
```

### 9.2 Representative rating-curve example (`icntrl=4`)

```text
99.1
1
2
1.00000
channel 1 comment a
channel 1 comment b
channel 1 comment c
1
4
1
0
3.83 0.025
0.04 0.001 1.0 0.5 1.0
0.02 4.0 0.02
1.25 1.50 0.10
```

### 9.3 Invalid example A (`icntrl=4` but missing line 15)

```text
99.1
1
2
1.00000
c1
c2
c3
1
4
1
0
3.83 0.025
0.04 0.001 1.0 0.5 1.0
0.02 4.0 0.02
```

Expected: `MissingRatingCurveLine`.

### 9.4 Invalid example B (`nchan` disagrees with structure/management)

```text
99.1
2
4
1.500000
...
```

Expected: `ChannelCountMismatch` when compared with `.str`/management channel count.

## 10. Gap/conflict register and HOLD conditions

| Gap ID | Severity | Description | Evidence tag | Provenance tags | HOLD condition |
|---|---|---|---|---|---|
| `CHN-GAP-001` | medium | `ishape` conflict resolved by `CHN-POL-001`; usersum authoring domain and legacy runtime normalization are now both explicit. | [DIRECT] | `usersum2024`, `legacy-code` | Closed as high-severity blocker; retained as provenance note for strict/compat policy traceability. |
| `CHN-GAP-002` | medium | Control-section roughness conflict resolved by `CHN-POL-002`; canonical precedence is `ctln <- chnn` for `icntrl==0` with explicit observability. | [DIRECT] | `usersum2024`, `legacy-code` | Closed as high-severity blocker; retained as provenance note for strict/compat policy traceability. |
| `CHN-GAP-003` | medium | `flgout` semantic conflict: usersum documents line 11 flag, legacy immediately overrides with global `watsum`. | [DIRECT] | `usersum2024`, `legacy-code` | Decide strict/compat mode behavior for line 11 significance. |
| `CHN-GAP-004` | medium | Legacy compatibility window (`>=94.301`) is accepted by version check, but this spec only normatively defines 99.1 line contract. | [DIRECT] | `usersum2024`, `legacy-code` | Add explicit migrated sub-spec or reject pre-99.1 with documented policy. |
| `CHN-GAP-005` | medium | `slplst` is scalar in common block and used for `ctlslp` override when `icntrl=0`; per-channel semantics are not fully explicit in docs. | [DIRECT] | `legacy-code`, `literature` | Resolve and codify intended per-channel vs shared behavior for openWEPP contract. |

Evidence:
- `CHN-GAP-001`: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7275-7277`, `/workdir/wepp-forest/src/chnpar.for:83-85`, `/workdir/wepp-forest/src/wshinp.for:386-390`
- `CHN-GAP-002`: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7316-7340`, `/workdir/wepp-forest/src/wshinp.for:428-431`
- `CHN-GAP-003`: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7295-7301`, `/workdir/wepp-forest/src/wshinp.for:394-397`
- `CHN-GAP-004`: `/workdir/wepp-forest/src/inidat.for:1162`, `/workdir/wepp-forest/src/verchk.for:25-31`
- `CHN-GAP-005`: `/workdir/wepp-forest/src/cchtrl.inc:7-14`, `/workdir/wepp-forest/src/input.for:414-416`, `/workdir/wepp-forest/src/wshinp.for:431`

## 11. Parser-contract handoff map (`SC-INFILE-WATERSHED-CHANNEL-001`)

| Spec requirement | Contract obligation target |
|---|---|
| Line grammar and conditional line-15 branch | Parser must enforce structural completeness and `icntrl==4` conditional record consumption. |
| Canonical symbol continuity + alias map | Contract must expose canonical WEPP names and explicit boundary aliases. |
| Count closure (`nchan` vs structure/management) | Contract must require multi-surface closure check before run assembly. |
| Cross-file overrides (`ctlslp`, `flgout`, `chan.inp`, `tcr.txt`) | Contract must define compatibility-mode behavior and strict-mode diagnostics. |
| Gap register HOLD items | Contract remains `HOLD` until remaining medium-gap policy items (`CHN-GAP-003..005`) are dispositioned. |

[INFERENCE] `SC-INFILE-WATERSHED-CHANNEL-001` can now proceed with `CHN-POL-001/002` fixed; carry forward only medium-gap items (`CHN-GAP-003..005`) as explicit HOLD notes.

Handoff linkage:
- `parser_contract_id`: `SC-INFILE-WATERSHED-CHANNEL-001`
- `canonical_contract_path`: `docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md`
- `handoff_status`: `contract-authored-draft (medium HOLD gaps carried forward to review/disposition)`

## Provenance index
- Primary format authority:
  - `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf`
  - `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt`
- Legacy static behavior:
  - `/workdir/wepp-forest/src/infile.for`
  - `/workdir/wepp-forest/src/wshinp.for`
  - `/workdir/wepp-forest/src/input.for`
  - `/workdir/wepp-forest/src/chnpar.for`
  - `/workdir/wepp-forest/src/inidat.for`
  - `/workdir/wepp-forest/src/verchk.for`
  - `/workdir/wepp-forest/src/cchtrl.inc`
- Modern implementation references:
  - `/workdir/wepppy/wepppy/nodb/core/wepp.py`
  - `/workdir/wepppy/wepppy/wepp/management/channels.py`
  - `/workdir/wepppy/wepppy/wepp/management/data/channels.defs`
  - `/workdir/wepppyo3/` (static scan performed; no dedicated `.chn` parser surfaced in inspected crates)
