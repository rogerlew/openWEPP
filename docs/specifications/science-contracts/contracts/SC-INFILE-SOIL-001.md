---
contract_id: SC-INFILE-SOIL-001
title: Soil Input Parser Contract (.sol)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.6
evidence_mode: Static
last_updated_utc: 2026-05-25T00:00:00Z
---

# SC-INFILE-SOIL-001 Soil Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-SOL-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md` (canonical openWEPP soil input format and datver variants).
- `[DIRECT][E-SURVEY-SOL-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (`.sol` parser coverage and legacy/runtime provenance references).
- `[DIRECT][E-WF-SOL-01]` `/home/workdir/wepp-forest/src/infile.for` and `/home/workdir/wepp-forest/src/input.for` (legacy soil parse branches cited by survey).
- `[DIRECT][E-WF-SOL-02]` `/workdir/wepp-forest_260430_baseline/src/input.for:475-482` (legacy parser branch for `solwpv >= 7777` reads 8 OFE-header fields through `shcrit`, omitting `avke`).
- `[DIRECT][E-WP-SOL-01]` `/workdir/wepppy/wepp/soils/utils/wepp_soil_util.py` (`_parse_sol` parser surface cited by survey).
- `[DIRECT][E-OW-SOIL-SEAM-01]` `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs` (soil parser-to-runtime projection seam for canonical `thetdr`/`thetfc` symbols).
- `[INFERENCE][E-PHYS-SOL-01]` Physical/common-sense invariants: positive layer depths, bounded volumetric fractions, non-negative conductivity and erodibility parameters.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for surface `infile-soil-sol` (`.sol`) and typed handoff from source layer records into simulation-ready soil profile state.

### 1.2 Version/Datver Applicability Matrix

| Datver | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- |
| `97.5` | Accept. | Parse base profile+texture form. | `[DIRECT][E-SPEC-SOL-01]` |
| `2006.2` | Accept. | Parse base form + restrictive layer block. | `[DIRECT][E-SPEC-SOL-01]` |
| `7777` | Accept. | Parse extended layer hydraulic fields. | `[DIRECT][E-SPEC-SOL-01]` |
| `7778` | Accept. | Parse `7777` + anisotropy. | `[DIRECT][E-SPEC-SOL-01]` |
| `9002` | Accept. | Parse disturbed-land pre-layer controls + extended layer records. | `[DIRECT][E-SPEC-SOL-01]` |
| `9003` | Accept. | Parse disturbed-land + burn-code variant. | `[DIRECT][E-SPEC-SOL-01]` |
| `9005` | Accept. | Parse revegetation controls + extended layer records. | `[DIRECT][E-SPEC-SOL-01]` |
| `7778` quoted legacy header form | Strict reject. | Compatibility mode may accept OFE header rows where `slid`/`texid` are single-quoted string fields with embedded whitespace and optional trailing `avke` omission (`avke` normalized to `0.0` when omitted). | `[DIRECT][E-WF-SOL-01]`, `[DIRECT][E-WF-SOL-02]`, `[DIRECT][E-WP-SOL-01]` |
| `7778` per-OFE restrictive footer legacy form | Strict reject. | Compatibility mode may accept one restrictive-layer row per OFE block when all per-OFE restrictive rows are identical; normalized to a single profile restrictive-layer state. | `[DIRECT][E-WP-SOL-01]` |
| unknown numeric | Strict reject. Compat reject unless explicitly allowlisted. | Emit typed `UnsupportedDatver`. | `[INFERENCE][E-SPEC-SOL-01]` |

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
sol_file = datver_line solcom_line ntemp_ksflag_line ofe_block{ntemp} [restrictive_layer_line] ;

ofe_block = line4_header layer_control_line? layer_row{nsl} [compat_ofe_restrictive_line] ;

line4_header = (slid texid nsl salb sat ki kr shcrit avke)
             | (quoted(slid) quoted(texid) nsl salb sat ki kr shcrit [avke]) ;  (* quoted header form is compatibility-only *)
layer_control_line = (ksatadj luse stext ksatfac ksatrec)
                 | (ksatadj luse burn_code stext lkeff)
                 | (ksatadj luse burn_code stext texid_enum uksat lkeff) ;
compat_ofe_restrictive_line = slflag ui_bdrkth kslast ;  (* compatibility-only: per-OFE legacy placement *)
```

### 2.2 Two-Layer Model Contract

- Source model preserves datver-specific row shape exactly and records per-OFE raw layer rows.
- Simulation model normalizes to typed `SoilProfile` with:
  - canonical core OFE fields,
  - typed layer vector,
  - optional disturbed/revegetation policy block,
  - optional restrictive-layer block.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `soil.version.datver` | none | real | 1 | yes | all | none | `soil.datver` |
| `solcom` | `header.solcom` | `soil.meta.comment` | text | string | 1 | yes | all | none | `soil.comment` |
| `ntemp` | `header.ntemp` | `soil.ofe_count` | count | int | 1 | yes | all | none | `soil.ofe_count` |
| `ksflag` | `header.ksflag` | `soil.mode.ks_adjust_enabled` | flag | int/bool | 1 | yes | all | bool derived | `soil.ksflag` |
| `slid` | `ofe[i].slid` | `soil.ofe[i].soil_id` | text | string | ntemp | yes | all | none | `soil_id` |
| `texid` | `ofe[i].texid` | `soil.ofe[i].texture_id` | text | string | ntemp | yes | all | none | `texture_id` |
| `nsl` | `ofe[i].nsl` | `soil.ofe[i].layer_count` | count | int | ntemp | yes | all | none | `layer_count` |
| `salb` | `ofe[i].salb` | `soil.ofe[i].albedo` | fraction | real | ntemp | yes | all | none | `albedo` |
| `sat` | `ofe[i].sat` | `soil.ofe[i].initial_sat` | m/m | real | ntemp | yes | all | none | `initial_sat` |
| `ki` | `ofe[i].ki` | `soil.ofe[i].ki_base` | kg*s/m^4 | real | ntemp | yes | all | none | `ki_base` |
| `kr` | `ofe[i].kr` | `soil.ofe[i].kr_base` | s/m | real | ntemp | yes | all | none | `kr_base` |
| `shcrit` | `ofe[i].shcrit` | `soil.ofe[i].tau_c_base` | N/m^2 | real | ntemp | yes | all | none | `tau_c_base` |
| `avke` | `ofe[i].avke` | `soil.ofe[i].avke_mm_h` | mm/h | real | ntemp | yes | all | compatibility-only quoted `7778` legacy form may omit trailing field, normalized to `0.0` | `avke_mm_h` |
| `solthk` | `layer[j].solthk` | `soil.ofe[i].layers[j].depth_mm` | mm | real | sum(nsl) | yes | all | none | `depth_mm` |
| `sand` | `layer[j].sand` | `soil.ofe[i].layers[j].sand_pct` | % | real | sum(nsl) | conditional | all | none | `sand_pct` |
| `clay` | `layer[j].clay` | `soil.ofe[i].layers[j].clay_pct` | % | real | sum(nsl) | conditional | all | none | `clay_pct` |
| `orgmat` | `layer[j].orgmat` | `soil.ofe[i].layers[j].orgmat_pct` | % vol | real | sum(nsl) | conditional | all | none | `orgmat_pct` |
| `cec` | `layer[j].cec` | `soil.ofe[i].layers[j].cec_meq_100g` | meq/100g | real | sum(nsl) | conditional | all | none | `cec_meq_100g` |
| `rfg` | `layer[j].rfg` | `soil.ofe[i].layers[j].rock_frag_pct` | % vol | real | sum(nsl) | conditional | all | none | `rock_frag_pct` |
| `bd` | `layer[j].bd` | `soil.ofe[i].layers[j].bulk_density_g_cm3` | g/cm^3 | real | sum(nsl) | conditional | 7777+ | none | `bulk_density_g_cm3` |
| `ksat` | `layer[j].ksat` | `soil.ofe[i].layers[j].ksat_mm_h` | mm/h | real | sum(nsl) | conditional | 7777+ | none | `ksat_mm_h` |
| `anisotropy` | `layer[j].anisotropy` | `soil.ofe[i].layers[j].anisotropy_ratio` | ratio | real | sum(nsl) | conditional | 7778+ | default `1.0` when not present | `anisotropy_ratio` |
| `fc` (measured) | `layer[j].fc` | `soil.ofe[i].layers[j].fc_measured` | m^3/m^3 | real | sum(nsl) | conditional | 7777+ | none | `fc_measured` |
| `wp` (measured) | `layer[j].wp` | `soil.ofe[i].layers[j].wp_measured` | m^3/m^3 | real | sum(nsl) | conditional | 7777+ | none | `wp_measured` |
| `theta_r` | `layer[j].theta_r` | `soil.ofe[i].layers[j].theta_r_rosetta` | m^3/m^3 | real | sum(nsl) | conditional | 9002+ | none | `theta_r_rosetta` |
| `theta_s` | `layer[j].theta_s` | `soil.ofe[i].layers[j].theta_s_rosetta` | m^3/m^3 | real | sum(nsl) | conditional | 9002+ | none | `theta_s_rosetta` |
| `alpha` | `layer[j].alpha` | `soil.ofe[i].layers[j].alpha_vg` | 1/cm | real | sum(nsl) | conditional | 9002+ | none | `alpha_vg` |
| `npar` | `layer[j].npar` | `soil.ofe[i].layers[j].npar_vg` | none | real | sum(nsl) | conditional | 9002+ | none | `npar_vg` |
| `ks` | `layer[j].ks` | `soil.ofe[i].layers[j].ks_rosetta_cm_d` | cm/day | real | sum(nsl) | conditional | 9002+ | none | `ks_rosetta_cm_d` |
| `wp` (Rosetta) | `layer[j].wp_rosetta` | `soil.ofe[i].layers[j].wp_rosetta` | m^3/m^3 | real | sum(nsl) | conditional | 9002+ | appended field in 9002+ layer rows | `wp_rosetta` |
| `fc` (Rosetta) | `layer[j].fc_rosetta` | `soil.ofe[i].layers[j].fc_rosetta` | m^3/m^3 | real | sum(nsl) | conditional | 9002+ | appended field in 9002+ layer rows | `fc_rosetta` |
| `ksatadj` | `ofe[i].disturbed_hdr.ksatadj` | `soil.ofe[i].disturbed_policy.ksatadj` | flag | int | 0..ntemp | conditional | 9002+ | none | `ksatadj` |
| `luse` | `ofe[i].disturbed_hdr.luse` | `soil.ofe[i].disturbed_policy.luse` | enum/text | string | 0..ntemp | conditional | 9002+ | none | `luse` |
| `stext` | `ofe[i].disturbed_hdr.stext` | `soil.ofe[i].disturbed_policy.stext` | enum/text | string | 0..ntemp | conditional | 9002+ | none | `stext` |
| `ksatfac` | `ofe[i].disturbed_hdr.ksatfac` | `soil.ofe[i].disturbed_policy.ksat_floor_mm_h` | mm/h | real | 0..ntemp | conditional | 9002 | none | `ksat_floor_mm_h` |
| `ksatrec` | `ofe[i].disturbed_hdr.ksatrec` | `soil.ofe[i].disturbed_policy.ksat_recovery_per_day` | 1/day | real | 0..ntemp | conditional | 9002 | none | `ksat_recovery_per_day` |
| `burn_code` | `ofe[i].disturbed_hdr.burn_code` | `soil.ofe[i].disturbed_policy.burn_code` | code | int | 0..ntemp | conditional | 9003+ | none | `burn_code` |
| `lkeff` | `ofe[i].disturbed_hdr.lkeff` | `soil.ofe[i].disturbed_policy.ksat_lower_mm_h` | mm/h | real | 0..ntemp | conditional | 9003+ | none | `ksat_lower_mm_h` |
| `texid_enum` | `ofe[i].reveg_hdr.texid_enum` | `soil.ofe[i].reveg_policy.texture_enum` | enum | int | 0..ntemp | conditional | 9005 | none | `texture_enum` |
| `uksat` | `ofe[i].reveg_hdr.uksat` | `soil.ofe[i].reveg_policy.ksat_upper_mm_h` | mm/h | real | 0..ntemp | conditional | 9005 | none | `ksat_upper_mm_h` |
| `slflag,ui_bdrkth,kslast` | `footer.restrictive` | `soil.restrictive_layer` | mixed | record | 0..1 | conditional | 2006.2,7777,7778,9002,9003,9005 | compatibility-only `7778` legacy MOFE form may place one row per OFE; all per-OFE rows must match and normalize to one profile row | `restrictive_layer` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `soil.version.datver` | `input::soil` | init | immutable | parser compatibility gate | `G-SOL-001` |
| `ntemp` | `header.ntemp` | `soil.ofe_count` | `input::soil` | init | immutable | OFE topology consistency checks | `G-SOL-002` |
| `ksflag` | `header.ksflag` | `soil.mode.ks_adjust_enabled` | `input::soil` | init | immutable | conductivity/frost coupling policy | `G-SOL-003` |
| `slid,texid,nsl` | `ofe[i].header` | `soil.ofe[i].id_and_counts` | `input::soil` | init | immutable | profile loader | `G-SOL-004` |
| `salb,sat,ki,kr,shcrit,avke` | `ofe[i].base_fields` | `soil.ofe[i].base_params` | `input::soil` | init | immutable | watbal, evap, erodibility initialization | `G-SOL-005` |
| `solthk,sand,clay,orgmat,cec,rfg` | `ofe[i].layers[j].texture_rows` | `soil.ofe[i].layers[j].texture_state` | `input::soil` | init,daily | immutable | watbal, evap, erosion, root-zone accounting | `G-SOL-006`, `G-SOL-007` |
| `bd,ksat,anisotropy,fc,wp (measured)` | `ofe[i].layers[j].hydraulic_rows` | `soil.ofe[i].layers[j].hydraulic_state` | `input::soil` | init,daily,event | immutable | infiltration, percolation, freeze-thaw conductivity logic | `G-SOL-006`, `G-SOL-007` |
| `theta_r,theta_s,alpha,npar,ks,wp(rosetta),fc(rosetta)` | `ofe[i].layers[j].rosetta_rows` | `soil.ofe[i].layers[j].rosetta_state` | `input::soil` | init,daily,event | immutable | pedotransfer recalculation paths and conductivity adjustments | `G-SOL-006`, `G-SOL-008` |
| `ksatadj,luse,stext,ksatfac,ksatrec,burn_code,lkeff,texid_enum,uksat` | `ofe[i].policy_hdr` | `soil.ofe[i].policy_state` | `input::soil` | init,daily,event | immutable | disturbed-land conductivity adjustment and revegetation controls | `G-SOL-008` |
| restrictive layer row | `footer.restrictive` | `soil.restrictive_layer` | `input::soil` | init | immutable | percolation lower-bound condition | `G-SOL-009` |

## 5. State Ownership and Mutability

- `input::soil` owns source parse tree and normalized `SoilProfile` state. `[INFERENCE][E-SURVEY-SOL-01]`
- Parsed source records are immutable after parse success.
- Kernel modules may maintain mutable dynamic soil process state separately, but immutable parsed inputs cannot be mutated ad hoc.
- Forbidden mutation path: runtime hydrology/erosion modules overwriting parsed layer identity/ordering or datver-governed fields in place.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-SOL-001` | Derive cumulative layer bottoms and verify strictly increasing `solthk` per OFE. | parse finalize | `C-SOL-001` |
| `D-SOL-002` | Normalize optional disturbed/reveg control blocks into one policy enum surface by datver. | per OFE parse finalize | `C-SOL-002` |
| `D-SOL-003` | Derive restrictive-layer effective presence from `slflag` and validate dependent fields. | footer parse finalize | `C-SOL-003` |
| `D-SOL-004` | Runtime export precedence for canonical hydrology theta symbols is datver-compatible and fail-closed: `thetdr := theta_r_rosetta` when present, else `wp_measured`; `thetfc := fc_rosetta` when present, else `fc_measured`; if neither source exists for a required layer, projection fails with typed runtime error. | parser-to-runtime seam projection | `C-SOL-004` |
| `D-SOL-005` | Runtime export of disturbed-land conductivity-adjustment regime metadata is deterministic and fail-closed: `solwpv := datver_raw`; per-OFE policy aliases (`ofe{i}_ksatadj`, `ofe{i}_ksatfac`, `ofe{i}_ksatrec`, `ofe{i}_lkeff`) are projected when datver policy fields exist; primary OFE aliases (`ksatadj`, `ksatfac`, `ksatrec`, `lkeff`) mirror OFE1; when policy is absent, `ksatadj := 0` and regime-only fields remain omitted. | parser-to-runtime seam projection | `C-SOL-005` |

Closure hooks:
- `C-SOL-001`: layer-depth closure and count closure (`nsl` rows present).
- `C-SOL-002`: datver-policy-row arity closure.
- `C-SOL-003`: restrictive-layer field closure and domain checks.
- `C-SOL-004`: runtime theta export source closure (`Rosetta` preferred with measured fallback; no silent defaults when both missing).
- `C-SOL-005`: runtime `ksatadj` regime export closure (`solwpv` + policy aliases) with no silent fabrication of regime-only fields.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `SOL-E-001` | syntax | numeric parse failure (`TokenParseError`) |
| `SOL-E-002` | syntax | insufficient rows for declared counts (`UnexpectedEof`/`RecordCountError`) |
| `SOL-E-003` | semantic | unsupported datver (`UnsupportedDatver`) |
| `SOL-E-004` | semantic | invalid OFE/layer counts (`FieldRangeError`) |
| `SOL-E-005` | semantic | invalid layer-domain values (negative depths, out-of-range fractions, negative conductivity) |
| `SOL-E-006` | semantic | datver row-shape mismatch (`VariantArityError`) |
| `SOL-E-007` | cross-file | OFE count mismatch vs slope/management topology (`CrossFileConsistencyError`) |
| `SOL-E-008` | cross-file | incompatible soil-version policy with required sidecar/frost mode (`CrossFileConsistencyError`) |
| `SOL-E-009` | runtime-guard | post-parse closure hook failure (`InvariantViolation`) |

No silent fallback masking for invalid required inputs. `[DIRECT][E-SPEC-SOL-01]`, `[INFERENCE][E-SURVEY-SOL-01]`

## 8. Cross-File Consistency Constraints

1. `ntemp` must align with OFE partition for hillslope mode and with channel-count semantics when soil entries are channel-scoped. `[DIRECT][E-SPEC-SOL-01]`, `[INFERENCE][E-SURVEY-SOL-01]`
2. Soil layer records must satisfy declared `nsl` per OFE exactly. `[DIRECT][E-SPEC-SOL-01]`
3. Disturbed/revegetation policy fields (`9002+`) must be interpreted consistently with management/use-mode inputs and downstream disturbed-land logic. `[DIRECT][E-SPEC-SOL-01]`, `[INFERENCE][E-SURVEY-SOL-01]`
4. Restrictive-layer fields must be consistent with percolation subsystem expectations when `slflag=1`. `[DIRECT][E-SPEC-SOL-01]`, `[INFERENCE][E-PHYS-SOL-01]`
5. Topology authority is mode-scoped: hillslope runs validate `ntemp == nofe`; watershed/channel-scoped runs validate `ntemp == nchan` from watershed structure surfaces; mixed-scope inputs are rejected as `CrossFileConsistencyError`. `[DIRECT][E-SPEC-SOL-01]`, `[INFERENCE][E-SURVEY-SOL-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `datver,solcom,ntemp,ksflag` | `soil.version`, `soil.meta`, `soil.mode` | hillslope/watershed parse result payload | same canonical names in metadata block | no parser-side coercion |
| `slid,texid,nsl,salb,sat,ki,kr,shcrit,avke` | `soil.ofe[*].base_params` | runtime initialization payload | aliases from Section 3; units preserved | shared across watbal/evap/erosion setup |
| `solthk,sand,clay,orgmat,cec,rfg,bd,ksat,anisotropy,fc,wp,theta_r,theta_s,alpha,npar,ks,wp(rosetta),fc(rosetta)` | `soil.ofe[*].layers[*]` | interchange/hydrology-state export | canonical symbol continuity with explicit alias names for duplicate `wp`/`fc` fields | measured vs Rosetta variants remain distinct in boundary schema |
| `theta_r,fc_rosetta,wp(measured),fc(measured)` | `soil.ofe[*].layers[*]` | hillslope runtime seed projection (`thetdr`,`thetfc`) | `thetdr := theta_r_rosetta` else `wp_measured`; `thetfc := fc_rosetta` else `fc_measured` | fail-closed typed runtime error when required source pair is unavailable or non-finite |
| `ksatadj,luse,stext,ksatfac,ksatrec,burn_code,lkeff,texid_enum,uksat` | `soil.ofe[*].policy_state` | disturbed/revegetation policy boundary | exported as datver-scoped policy blocks | missing datver-inapplicable fields are omitted, not default-filled |
| `datver_raw,ksatadj,ksatfac,ksatrec,lkeff` | `soil.version.datver` + `soil.ofe[*].policy_state` | hillslope runtime seed projection (`solwpv`,`ksatadj`,`ksatfac`,`ksatrec`,`lkeff`) | `solwpv := datver_raw`; primary aliases mirror OFE1 policy; per-OFE aliases prefixed `ofe{i}_*` | `ksatadj` defaults to `0` only when policy is absent; regime-only fields are not synthetic defaults |
| `slflag,ui_bdrkth,kslast` | `soil.restrictive_layer` | lower-boundary/percolation payload | exported as optional restrictive-layer object | present only when `slflag=1` |

## 10. Compatibility Policy

- Strict mode:
  - accept only allowlisted datver variants;
  - reject row-shape mismatches and policy-block arity violations;
  - reject unknown/ambiguous disturbed-land policy shapes.
- Compatibility mode:
  - does not silently rewrite between datver variants;
  - may accept legacy quoted OFE-header identifiers (`'slid' 'texid'`) for
    datver forms observed in legacy MOFE stacks when tokenization is lossless;
  - for quoted legacy `7778` forms, may accept either:
    - 9-token headers (`slid texid nsl salb sat ki kr shcrit avke`), or
    - 8-token headers (`slid texid nsl salb sat ki kr shcrit`) with explicit
      compatibility normalization `avke := 0.0`;
  - for legacy `7778` MOFE stacks, may accept restrictive-layer rows placed
    per OFE (immediately after each OFE layer block) when all per-OFE rows are
    identical; normalize to a single profile restrictive-layer row;
  - may allow explicitly configured legacy aliases only when row-shape mapping is lossless and documented.

Unsupported forms must fail with typed errors from Section 7.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-SOL-001` | datver allowlist | header parse | `SOL-E-003` |
| `G-SOL-002` | `ntemp` positive and bounded | header parse | `SOL-E-004` |
| `G-SOL-003` | `ksflag` enum/domain | header parse | `SOL-E-005` |
| `G-SOL-004` | per-OFE header arity and `nsl` domain | OFE header parse | `SOL-E-004`/`SOL-E-006` |
| `G-SOL-005` | base parameter domains (`sat`, `ki`, `kr`, `shcrit`, `avke`) | OFE header parse | `SOL-E-005` |
| `G-SOL-006` | layer record arity by datver | layer parse | `SOL-E-006` |
| `G-SOL-007` | monotone positive `solthk`, bounded fractions | layer closure | `SOL-E-005`/`SOL-E-009` |
| `G-SOL-008` | disturbed/reveg policy row validity | policy parse | `SOL-E-006`/`SOL-E-005` |
| `G-SOL-009` | restrictive-layer closure | footer parse | `SOL-E-009` |
| `G-SOL-010` | compatibility-only quoted header parse must unquote to exactly two identifier fields (`slid`,`texid`) and preserve numeric arity/order for remaining fields with either 9-token form (includes `avke`) or 8-token legacy form (omits `avke`, normalized to `0.0`) | OFE header parse | `SOL-E-006` |
| `G-SOL-011` | compatibility-only per-OFE restrictive-layer rows must either be absent or pairwise identical before profile-level normalization | OFE/footer compatibility parse | `SOL-E-006` |
| `G-SOL-012` | runtime theta export closure requires at least one valid source per required layer for each canonical symbol (`thetdr`: `theta_r_rosetta` or `wp_measured`; `thetfc`: `fc_rosetta` or `fc_measured`) with no silent defaulting | parser-to-runtime seam projection | typed runtime seam failure (`HS-RUNTIME-E-*`) |
| `G-SOL-013` | runtime `ksatadj` regime metadata export closure requires finite `solwpv` and binary `ksatadj` aliases; active-regime fields (`ksatfac`, `ksatrec`, `lkeff`) must only be exported from datver-applicable policy records | parser-to-runtime seam projection | typed runtime seam failure (`HS-RUNTIME-E-*`) |

## 12. Legacy Symbol Continuity and Alias Map

Canonical soil symbols are preserved (`datver`, `solcom`, `ntemp`, `ksflag`, `slid`, `texid`, `nsl`, `salb`, `sat`, `ki`, `kr`, `shcrit`, `avke`, `solthk`, `ksat`, `anisotropy`, `fc`, `wp`, `ksatadj`, `luse`, `stext`, `burn_code`, `texid_enum`, `uksat`, `lkeff`, `slflag`, `ui_bdrkth`, `kslast`).

openWEPP runtime names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `SOL-GAP-001` | Runtime export precedence for duplicated measured vs Rosetta `wp`/`fc` fields is ratified in `D-SOL-004`; non-runtime downstream interaction policy outside parser-to-runtime seam remains partially unspecified. | `[DIRECT][E-SPEC-SOL-01]`, `[DIRECT][E-OW-SOIL-SEAM-01]`, `[INFERENCE][E-SURVEY-SOL-01]` | `HOLD` |
| `SOL-GAP-002` | `ksflag` frost/freeze suppression behavior is documented as build-specific in current WEPP-Forest usage and is not yet formalized as a portable parser-contract compatibility rule. | `[DIRECT][E-SPEC-SOL-01]`, `[INFERENCE][E-WF-SOL-01]` | `HOLD` |
| `SOL-GAP-003` | Operational policy for `9005` revegetation controls (`uksat`, `lkeff`, `texid_enum`) is partially wepppy-derived and needs finalized openWEPP authority language. | `[DIRECT][E-SPEC-SOL-01]`, `[DIRECT][E-WP-SOL-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-25` | `0.1.6` | MOFE13 amendment: added deterministic parser-to-runtime export authority for `solwpv`/`ksatadj` regime metadata (`ksatfac`,`ksatrec`,`lkeff`) including primary + per-OFE aliases and fail-closed closure hook `C-SOL-005`. |
| `2026-05-25` | `0.1.5` | MOFE09 amendment: ratified parser-to-runtime theta export precedence (`theta_r_rosetta -> wp_measured` fallback for `thetdr`; `fc_rosetta -> fc_measured` fallback for `thetfc`) with explicit fail-closed guard linkage (`D-SOL-004`, `G-SOL-012`). |
| `2026-05-25` | `0.1.4` | MOFE07 compatibility amendment: accept legacy `7778` per-OFE restrictive-layer row placement in compatibility mode when rows are identical; normalize to one profile restrictive layer (`G-SOL-011`). |
| `2026-05-25` | `0.1.3` | MOFE07 compatibility amendment: accept quoted legacy `7778` 8-token OFE headers missing trailing `avke`, with explicit normalization `avke := 0.0`; added baseline evidence anchor `E-WF-SOL-02`. |
| `2026-05-25` | `0.1.2` | MOFE07 addendum: compatibility-only authority for quoted OFE header identifiers in legacy soil forms (`'slid' 'texid' ...`) with explicit guard linkage `G-SOL-010`. |
| `2026-05-21` | `0.1.1` | Expanded per-field coverage for 7777+/9002+/9005 layer and policy fields, added boundary export mapping, and clarified `ntemp` mode-scoped topology authority. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE02. |
