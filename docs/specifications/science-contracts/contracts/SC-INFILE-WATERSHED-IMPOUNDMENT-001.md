---
contract_id: SC-INFILE-WATERSHED-IMPOUNDMENT-001
title: Watershed Impoundment Input Parser Contract (.imp)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.1
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-WATERSHED-IMPOUNDMENT-001 Watershed Impoundment Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-IMP-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md` (canonical `.imp` grammar, branch model, field semantics, and open gaps).
- `[DIRECT][E-SURVEY-IMP-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and ownership scope).
- `[DIRECT][E-WF-IMP-01]` `/workdir/wepp-forest/src/infile.for`, `/workdir/wepp-forest/src/wshini.for`, `/workdir/wepp-forest/src/impint.for`, `/workdir/wepp-forest/src/verchk.for`, `/workdir/wepp-forest/src/pmximp.inc` (legacy version/count checks and section branch reads cited in spec).
- `[INFERENCE][E-PHYS-IMP-01]` Physical/common-sense invariants: stage-area-length arrays have equal cardinality, stage bounds are monotone/consistent, and branch payload must be complete when an outlet structure is enabled.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for surface `infile-watershed-impoundment-imp` (`.imp`) and parse-to-runtime handoff for impoundment geometry/outlet-structure parameters.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | explicit datver line and `datver >= 94.301` | Accept. | Canonical modern path. | `[DIRECT][E-SPEC-IMP-01]` |
| B | explicit datver line and `datver < 94.301` | Reject. | Emit typed `UnsupportedDatver`. | `[DIRECT][E-SPEC-IMP-01]`, `[DIRECT][E-WF-IMP-01]` |
| C | legacy no-datver form (`first token <= 10`) | Strict reject; compat-only optional branch. | Accept only under explicit compatibility mode with warning. | `[DIRECT][E-SPEC-IMP-01]`, `[DIRECT][E-WF-IMP-01]` |

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
imp_file = preamble impoundment_block{jpond} ;

preamble = datver_line jpond_line | legacy_compat_jpond_line ;
impoundment_block = impdes_lines
                    drop_spillway_section
                    culvert1_section
                    culvert2_section
                    rockfill_section
                    emergency_spillway_section
                    filter_barrier_section
                    perforated_riser_section
                    misc_section
                    stage_area_length_section ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves branch-local payload and section order exactly as parsed.
- Simulation model normalizes each impoundment into typed substructures:
  - `drop_spillway`, `culverts[2]`, `rockfill`, `emergency_spillway`, `filter_barrier`, `perforated_riser`,
  - `misc` stage/timestep/infiltration controls,
  - `stage_area_length_curve` arrays.
- Parser does not execute hydraulic conversions; numeric unit-conversion behavior in legacy runtime is treated as downstream process-layer behavior.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `watershed.impoundment.version.datver` | none | real | 0..1 | conditional | see Section 1 | none | `file_version` |
| `jpond` | `header.jpond` | `watershed.impoundment.declared_count` | count | int | 1 | yes | all | none | `impoundment_count_declared` |
| `impdes(1)` | `imp[i].impdes_1` | `watershed.impoundment.items[i].description.line1` | text | string | jpond | yes | all | none | `description_1` |
| `impdes(2)` | `imp[i].impdes_2` | `watershed.impoundment.items[i].description.line2` | text | string | jpond | yes | all | none | `description_2` |
| `impdes(3)` | `imp[i].impdes_3` | `watershed.impoundment.items[i].description.line3` | text | string | jpond | yes | all | none | `description_3` |
| `strdes` | `imp[i].branch_comment[*]` | `watershed.impoundment.items[i].branch_comments[*]` | text | string | variable | conditional | all | present for enabled branch structures (drop, culvert1, culvert2, rockfill, emergency, filter, riser) | `structure_branch_comment` |
| `ids` | `imp[i].drop.ids` | `watershed.impoundment.items[i].drop_spillway.kind` | enum | int | jpond | yes | all | none | `drop_spillway_kind` |
| `diars` | `imp[i].drop.diars` | `...drop_spillway.pipe_diameter_m` | m | real | subset(jpond, `ids=1`) | conditional | all | none | `drop_pipe_diameter_m` |
| `lenrs` | `imp[i].drop.lenrs` | `...drop_spillway.weir_length_m` | m | real | subset(jpond, `ids=2/3`) | conditional | all | none | `drop_weir_length_m` |
| `widrs` | `imp[i].drop.widrs` | `...drop_spillway.weir_width_m` | m | real | subset(jpond, `ids=2/3`) | conditional | all | none | `drop_weir_width_m` |
| `hrs` | `imp[i].drop.hrs` | `...drop_spillway.crest_height_m` | m | real | subset(jpond, `ids=1/2/3`) | conditional | all | none | `drop_crest_height_m` |
| `coefw` (drop) | `imp[i].drop.coefw` | `...drop_spillway.coefficient_weir` | none | real | subset(jpond, `ids=1/2/3`) | conditional | all | none | `drop_coef_weir` |
| `coefo` (drop) | `imp[i].drop.coefo` | `...drop_spillway.coefficient_orifice` | none | real | subset(jpond, `ids=1/2/3`) | conditional | all | none | `drop_coef_orifice` |
| `diabl` (drop) | `imp[i].drop.diabl` | `...drop_spillway.base_diameter_m` | m | real | subset(jpond, `ids=1/2/3`) | conditional | all | none | `drop_base_diameter_m` |
| `hitbl` | `imp[i].drop.hitbl` | `...drop_spillway.inlet_height_m` | m | real | subset(jpond, `ids=3`) | conditional | all | none | `drop_inlet_height_m` |
| `wdbl` | `imp[i].drop.wdbl` | `...drop_spillway.inlet_width_m` | m | real | subset(jpond, `ids=3`) | conditional | all | none | `drop_inlet_width_m` |
| `hrh` (drop) | `imp[i].drop.hrh` | `...drop_spillway.riser_height_m` | m | real | subset(jpond, `ids=1/2/3`) | conditional | all | none | `drop_riser_height_m` |
| `lbl` (drop) | `imp[i].drop.lbl` | `...drop_spillway.barrel_length_m` | m | real | subset(jpond, `ids=1/2/3`) | conditional | all | none | `drop_barrel_length_m` |
| `sbl` (drop) | `imp[i].drop.sbl` | `...drop_spillway.barrel_slope_m_per_m` | m/m | real | subset(jpond, `ids=1/2/3`) | conditional | all | none | `drop_barrel_slope` |
| `hblot` | `imp[i].drop.hblot` | `...drop_spillway.outlet_stage_m` | m | real | subset(jpond, `ids=1/2/3`) | conditional | all | none | `drop_outlet_stage_m` |
| `ke` (drop) | `imp[i].drop.ke` | `...drop_spillway.loss_coeff_entrance` | none | real | subset(jpond, `ids=1/2/3`) | conditional | all | none | `drop_ke` |
| `kb` (drop) | `imp[i].drop.kb` | `...drop_spillway.loss_coeff_bend` | none | real | subset(jpond, `ids=1/2/3`) | conditional | all | none | `drop_kb` |
| `kc` (drop) | `imp[i].drop.kc` | `...drop_spillway.loss_coeff_contraction` | none | real | subset(jpond, `ids=1/2/3`) | conditional | all | none | `drop_kc` |
| `icv(1)` | `imp[i].culvert1.icv` | `...culverts[1].enabled_code` | enum | int | jpond | yes | all | none | `culvert1_enabled_code` |
| `ncv(1)` | `imp[i].culvert1.ncv` | `...culverts[1].count` | count | int | jpond | yes | all | none | `culvert1_count` |
| `arcv(1)` | `imp[i].culvert1.arcv` | `...culverts[1].cross_section_area_m2` | m^2 | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_area_m2` |
| `hitcv(1)` | `imp[i].culvert1.hitcv` | `...culverts[1].inlet_height_m` | m | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_inlet_height_m` |
| `hcv(1)` | `imp[i].culvert1.hcv` | `...culverts[1].diameter_height_m` | m | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_diameter_height_m` |
| `lcv(1)` | `imp[i].culvert1.lcv` | `...culverts[1].length_m` | m | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_length_m` |
| `scv(1)` | `imp[i].culvert1.scv` | `...culverts[1].slope_m_per_m` | m/m | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_slope` |
| `hcvot(1)` | `imp[i].culvert1.hcvot` | `...culverts[1].outlet_stage_m` | m | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_outlet_stage_m` |
| `ke(1)` | `imp[i].culvert1.ke` | `...culverts[1].loss_coeff_entrance` | none | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_ke` |
| `kb(1)` | `imp[i].culvert1.kb` | `...culverts[1].loss_coeff_bend` | none | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_kb` |
| `kc(1)` | `imp[i].culvert1.kc` | `...culverts[1].loss_coeff_contraction` | none | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_kc` |
| `kus(1)` | `imp[i].culvert1.kus` | `...culverts[1].submergence_coeff` | none | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_kus` |
| `mus(1)` | `imp[i].culvert1.mus` | `...culverts[1].submergence_exp` | none | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_mus` |
| `cs(1)` | `imp[i].culvert1.cs` | `...culverts[1].discharge_coeff` | none | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_cs` |
| `ys(1)` | `imp[i].culvert1.ys` | `...culverts[1].tailwater_depth_m` | m | real | subset(jpond, `icv(1)>=1`) | conditional | all | none | `culvert1_ys_m` |
| `icv(2)` | `imp[i].culvert2.icv` | `...culverts[2].enabled_code` | enum | int | jpond | yes | all | none | `culvert2_enabled_code` |
| `ncv(2)` | `imp[i].culvert2.ncv` | `...culverts[2].count` | count | int | jpond | yes | all | none | `culvert2_count` |
| `arcv(2)` | `imp[i].culvert2.arcv` | `...culverts[2].cross_section_area_m2` | m^2 | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_area_m2` |
| `hitcv(2)` | `imp[i].culvert2.hitcv` | `...culverts[2].inlet_height_m` | m | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_inlet_height_m` |
| `hcv(2)` | `imp[i].culvert2.hcv` | `...culverts[2].diameter_height_m` | m | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_diameter_height_m` |
| `lcv(2)` | `imp[i].culvert2.lcv` | `...culverts[2].length_m` | m | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_length_m` |
| `scv(2)` | `imp[i].culvert2.scv` | `...culverts[2].slope_m_per_m` | m/m | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_slope` |
| `hcvot(2)` | `imp[i].culvert2.hcvot` | `...culverts[2].outlet_stage_m` | m | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_outlet_stage_m` |
| `ke(2)` | `imp[i].culvert2.ke` | `...culverts[2].loss_coeff_entrance` | none | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_ke` |
| `kb(2)` | `imp[i].culvert2.kb` | `...culverts[2].loss_coeff_bend` | none | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_kb` |
| `kc(2)` | `imp[i].culvert2.kc` | `...culverts[2].loss_coeff_contraction` | none | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_kc` |
| `kus(2)` | `imp[i].culvert2.kus` | `...culverts[2].submergence_coeff` | none | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_kus` |
| `mus(2)` | `imp[i].culvert2.mus` | `...culverts[2].submergence_exp` | none | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_mus` |
| `cs(2)` | `imp[i].culvert2.cs` | `...culverts[2].discharge_coeff` | none | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_cs` |
| `ys(2)` | `imp[i].culvert2.ys` | `...culverts[2].tailwater_depth_m` | m | real | subset(jpond, `icv(2)>=1`) | conditional | all | none | `culvert2_ys_m` |
| `irf` | `imp[i].rockfill.irf` | `...rockfill.enabled_code` | enum | int | jpond | yes | all | none | `rockfill_enabled_code` |
| `lnrf` | `imp[i].rockfill.lnrf` | `...rockfill.length_m` | m | real | subset(jpond, `irf!=0`) | conditional | all | none | `rockfill_length_m` |
| `hrf` | `imp[i].rockfill.hrf` | `...rockfill.height_m` | m | real | subset(jpond, `irf!=0`) | conditional | all | none | `rockfill_height_m` |
| `hotrf` | `imp[i].rockfill.hotrf` | `...rockfill.outlet_stage_m` | m | real | subset(jpond, `irf!=0`) | conditional | all | none | `rockfill_outlet_stage_m` |
| `wdrf` | `imp[i].rockfill.wdrf` | `...rockfill.width_m` | m | real | subset(jpond, `irf!=0`) | conditional | all | none | `rockfill_width_m` |
| `diarf` | `imp[i].rockfill.diarf` | `...rockfill.diameter_m` | m | real | subset(jpond, `irf!=0`) | conditional | all | none | `rockfill_diameter_m` |
| `ies` | `imp[i].emergency.ies` | `...emergency_spillway.kind` | enum | int | jpond | yes | all | none | `emergency_spillway_kind` |
| `bwes` | `imp[i].emergency.open_channel.bwes` | `...emergency_spillway.open_channel.bottom_width_m` | m | real | subset(jpond, `ies=1`) | conditional | all | none | `emergency_bottom_width_m` |
| `sses` | `imp[i].emergency.open_channel.sses` | `...emergency_spillway.open_channel.side_slope` | m/m | real | subset(jpond, `ies=1`) | conditional | all | none | `emergency_side_slope` |
| `nes` | `imp[i].emergency.open_channel.nes` | `...emergency_spillway.open_channel.manning_n` | none | real | subset(jpond, `ies=1`) | conditional | all | none | `emergency_manning_n` |
| `hes` | `imp[i].emergency.hes` | `...emergency_spillway.crest_stage_m` | m | real | subset(jpond, `ies=1/2`) | conditional | all | none | `emergency_crest_stage_m` |
| `hmxes` | `imp[i].emergency.open_channel.hmxes` | `...emergency_spillway.open_channel.max_stage_m` | m | real | subset(jpond, `ies=1`) | conditional | all | none | `emergency_max_stage_m` |
| `ses1` | `imp[i].emergency.open_channel.ses1` | `...emergency_spillway.open_channel.seg1_slope` | m/m | real | subset(jpond, `ies=1`) | conditional | all | none | `emergency_seg1_slope` |
| `les1` | `imp[i].emergency.open_channel.les1` | `...emergency_spillway.open_channel.seg1_length_m` | m | real | subset(jpond, `ies=1`) | conditional | all | none | `emergency_seg1_length_m` |
| `ses2` | `imp[i].emergency.open_channel.ses2` | `...emergency_spillway.open_channel.seg2_slope` | m/m | real | subset(jpond, `ies=1`) | conditional | all | none | `emergency_seg2_slope` |
| `les2` | `imp[i].emergency.open_channel.les2` | `...emergency_spillway.open_channel.seg2_length_m` | m | real | subset(jpond, `ies=1`) | conditional | all | none | `emergency_seg2_length_m` |
| `ses3` | `imp[i].emergency.open_channel.ses3` | `...emergency_spillway.open_channel.seg3_slope` | m/m | real | subset(jpond, `ies=1`) | conditional | all | none | `emergency_seg3_slope` |
| `npts` | `imp[i].emergency.rating.npts` | `...emergency_spillway.rating_curve.point_count` | count | int | subset(jpond, `ies=2`) | conditional | all | none | `emergency_rating_point_count` |
| `hest` | `imp[i].emergency.rating.hest[*]` | `...emergency_spillway.rating_curve.stage_m[*]` | m | real[] | sum(npts) | conditional | all | none | `emergency_rating_stage_m` |
| `qes` | `imp[i].emergency.rating.qes[*]` | `...emergency_spillway.rating_curve.flow_m3_s[*]` | m^3/s | real[] | sum(npts) | conditional | all | none | `emergency_rating_flow_m3_s` |
| `iff` | `imp[i].filter.iff` | `...filter_barrier.kind` | enum | int | jpond | yes | all | none | `filter_barrier_kind` |
| `vsl` | `imp[i].filter.vsl` | `...filter_barrier.velocity_m_s` | m/s | real | subset(jpond, `iff!=0`) | conditional | all | none | `filter_velocity_m_s` |
| `wdff` | `imp[i].filter.wdff` | `...filter_barrier.width_m` | m | real | subset(jpond, `iff!=0`) | conditional | all | none | `filter_width_m` |
| `hff` | `imp[i].filter.hff` | `...filter_barrier.height_m` | m | real | subset(jpond, `iff!=0`) | conditional | all | none | `filter_height_m` |
| `hotff` | `imp[i].filter.hotff` | `...filter_barrier.outlet_stage_m` | m | real | subset(jpond, `iff!=0`) | conditional | all | none | `filter_outlet_stage_m` |
| `ipr` | `imp[i].riser.ipr` | `...perforated_riser.enabled_code` | enum | int | jpond | yes | all | none | `perforated_riser_enabled_code` |
| `hr` | `imp[i].riser.hr` | `...perforated_riser.height_m` | m | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_height_m` |
| `hb` | `imp[i].riser.hb` | `...perforated_riser.base_height_m` | m | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_base_height_m` |
| `hs` | `imp[i].riser.hs` | `...perforated_riser.slot_height_m` | m | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_slot_height_m` |
| `hd` | `imp[i].riser.hd` | `...perforated_riser.drawdown_height_m` | m | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_drawdown_height_m` |
| `diar` | `imp[i].riser.diar` | `...perforated_riser.riser_diameter_m` | m | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_diameter_m` |
| `as` | `imp[i].riser.as` | `...perforated_riser.slot_area_m2` | m^2 | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_slot_area_m2` |
| `diab` | `imp[i].riser.diab` | `...perforated_riser.barrel_diameter_m` | m | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_barrel_diameter_m` |
| `hrh` (riser) | `imp[i].riser.hrh` | `...perforated_riser.headwall_height_m` | m | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_headwall_height_m` |
| `lbl` (riser) | `imp[i].riser.lbl` | `...perforated_riser.barrel_length_m` | m | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_barrel_length_m` |
| `sbl` (riser) | `imp[i].riser.sbl` | `...perforated_riser.barrel_slope_m_per_m` | m/m | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_barrel_slope` |
| `diabl` (riser) | `imp[i].riser.diabl` | `...perforated_riser.base_diameter_m` | m | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_base_diameter_m` |
| `cb` | `imp[i].riser.cb` | `...perforated_riser.discharge_coeff_barrel` | none | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_cb` |
| `coefw` (riser) | `imp[i].riser.coefw` | `...perforated_riser.coefficient_weir` | none | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_coef_weir` |
| `coefo` (riser) | `imp[i].riser.coefo` | `...perforated_riser.coefficient_orifice` | none | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_coef_orifice` |
| `cs` (riser) | `imp[i].riser.cs` | `...perforated_riser.discharge_coeff_slot` | none | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_cs` |
| `ke` (riser) | `imp[i].riser.ke` | `...perforated_riser.loss_coeff_entrance` | none | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_ke` |
| `kb` (riser) | `imp[i].riser.kb` | `...perforated_riser.loss_coeff_bend` | none | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_kb` |
| `kc` (riser) | `imp[i].riser.kc` | `...perforated_riser.loss_coeff_contraction` | none | real | subset(jpond, `ipr!=0`) | conditional | all | none | `riser_kc` |
| `hot` | `imp[i].misc.hot` | `...misc.overtop_stage_m` | m | real | jpond | yes | all | none | `overtop_stage_m` |
| `hfull` | `imp[i].misc.hfull` | `...misc.full_sediment_stage_m` | m | real | jpond | yes | all | none | `full_sediment_stage_m` |
| `h` | `imp[i].misc.h` | `...misc.initial_stage_m` | m | real | jpond | yes | all | none | `initial_stage_m` |
| `deltat` | `imp[i].misc.deltat` | `...misc.initial_timestep_hr` | hr | real | jpond | yes | all | none | `initial_timestep_hr` |
| `qinf` | `imp[i].misc.qinf` | `...misc.infiltration_rate_m_per_d` | m/d | real | jpond | yes | all | none | `infiltration_rate_m_per_d` |
| `isize` | `imp[i].misc.isize` | `...misc.structure_size_class` | enum | int | jpond | yes | all | none | `structure_size_class` |
| `ndiv` | `imp[i].misc.ndiv` | `...misc.particle_subclass_divisions` | count | int | jpond | yes | all | none | `particle_subclass_divisions` |
| `nalpts` | `imp[i].curve.nalpts` | `...stage_area_length_curve.point_count` | count | int | jpond | yes | all | none | `stage_area_length_point_count` |
| `hmin` | `imp[i].curve.hmin` | `...stage_area_length_curve.hmin_m` | m | real | jpond | yes | all | none | `curve_hmin_m` |
| `a0` | `imp[i].curve.a0` | `...stage_area_length_curve.a0_m2` | m^2 | real | jpond | yes | all | none | `curve_a0_m2` |
| `l0` | `imp[i].curve.l0` | `...stage_area_length_curve.l0_m` | m | real | jpond | yes | all | none | `curve_l0_m` |
| `hal` | `imp[i].curve.hal[*]` | `...stage_area_length_curve.stage_m[*]` | m | real[] | sum(nalpts) | yes | all | none | `curve_stage_m` |
| `area` | `imp[i].curve.area[*]` | `...stage_area_length_curve.area_m2[*]` | m^2 | real[] | sum(nalpts) | yes | all | none | `curve_area_m2` |
| `length` | `imp[i].curve.length[*]` | `...stage_area_length_curve.length_m[*]` | m | real[] | sum(nalpts) | yes | all | none | `curve_length_m` |
| derived `structure_enabled_flags` | branch ids | `...structure_flags` | flags | record(bool) | jpond | yes | all | derived from `ids/icv/irf/ies/iff/ipr` | `structure_flags` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `datver,jpond` | `header` | `watershed.impoundment.version`, `declared_count` | `input::watershed::impoundment` | init | immutable | version/count closure gate vs `.str` | `G-IMP-001`, `G-IMP-002` |
| `impdes(*)` | `items[*].description` | `watershed.impoundment.items[*].description` | `input::watershed::impoundment` | init | immutable | provenance/observability | `G-IMP-003` |
| `strdes` | `items[*].branch_comment[*]` | `watershed.impoundment.items[*].branch_comments[*]` | `input::watershed::impoundment` | init,watershed | immutable | source-fidelity provenance and branch diagnostics | `G-IMP-005` |
| drop-spillway fields (`ids` + payload) | `items[*].drop` | `...drop_spillway` | `input::watershed::impoundment` | init,watershed,event | immutable | outlet hydraulics routing | `G-IMP-004`, `G-IMP-005` |
| culvert fields (`icv/ncv` + payload) | `items[*].culverts[1..2]` | `...culverts[1..2]` | `input::watershed::impoundment` | init,watershed,event | immutable | culvert outflow routing | `G-IMP-006`, `G-IMP-007` |
| rockfill fields (`irf` + payload) | `items[*].rockfill` | `...rockfill` | `input::watershed::impoundment` | init,watershed,event | immutable | rockfill spill routing | `G-IMP-008` |
| emergency spillway fields (`ies` + payload) | `items[*].emergency` | `...emergency_spillway` | `input::watershed::impoundment` | init,watershed,event | immutable | emergency outflow branch | `G-IMP-009`, `G-IMP-010` |
| filter barrier fields (`iff` + payload) | `items[*].filter` | `...filter_barrier` | `input::watershed::impoundment` | init,watershed,event | immutable | sediment/filter routing modifiers | `G-IMP-011` |
| riser fields (`ipr` + payload) | `items[*].riser` | `...perforated_riser` | `input::watershed::impoundment` | init,watershed,event | immutable | perforated-riser outflow branch | `G-IMP-012` |
| misc stage/timestep fields | `items[*].misc` | `...misc` | `input::watershed::impoundment` | init,watershed,daily,event | immutable | storage update loop and infiltration timestep policy | `G-IMP-013` |
| curve fields (`nalpts`,`hmin`,`a0`,`l0`,`hal`,`area`,`length`) | `items[*].curve` | `...stage_area_length_curve` | `input::watershed::impoundment` | init,watershed,event | immutable | storage-area-length interpolation | `G-IMP-014`, `G-IMP-015` |
| derived `structure_enabled_flags` | `derived.structure_flags` | `watershed.impoundment.items[*].structure_flags` | `input::watershed::impoundment` | init,watershed,event | immutable | branch dispatch for outlet hydraulics | `G-IMP-016` |

### 4.1 Symbol-Level Propagation Coverage

This normative table closes symbol-level propagation for externally relevant fields using full parser-contract propagation-map shape (required columns).

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `watershed.impoundment.version.datver` | `input::watershed::impoundment` | `init` | `immutable` | `version/count closure gate vs .str` | `G-IMP-001`, `G-IMP-002` |
| `jpond` | `header.jpond` | `watershed.impoundment.declared_count` | `input::watershed::impoundment` | `init` | `immutable` | `version/count closure gate vs .str` | `G-IMP-001`, `G-IMP-002` |
| `impdes(1)` | `items[*].description.line1` | `watershed.impoundment.items[*].description.line1` | `input::watershed::impoundment` | `init` | `immutable` | `provenance/observability` | `G-IMP-003` |
| `impdes(2)` | `items[*].description.line2` | `watershed.impoundment.items[*].description.line2` | `input::watershed::impoundment` | `init` | `immutable` | `provenance/observability` | `G-IMP-003` |
| `impdes(3)` | `items[*].description.line3` | `watershed.impoundment.items[*].description.line3` | `input::watershed::impoundment` | `init` | `immutable` | `provenance/observability` | `G-IMP-003` |
| `strdes` | `items[*].branch_comment[*]` | `watershed.impoundment.items[*].branch_comments[*]` | `input::watershed::impoundment` | `init,watershed` | `immutable` | `source-fidelity provenance and branch diagnostics` | `G-IMP-005` |
| `ids` | `items[*].drop.kind` | `watershed.impoundment.items[*].drop_spillway.kind` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `diars` | `items[*].drop.pipe_diameter_m` | `watershed.impoundment.items[*].drop_spillway.pipe_diameter_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `lenrs` | `items[*].drop.weir_length_m` | `watershed.impoundment.items[*].drop_spillway.weir_length_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `widrs` | `items[*].drop.weir_width_m` | `watershed.impoundment.items[*].drop_spillway.weir_width_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `hrs` | `items[*].drop.crest_height_m` | `watershed.impoundment.items[*].drop_spillway.crest_height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `coefw` (drop) | `items[*].drop.coefficient_weir` | `watershed.impoundment.items[*].drop_spillway.coefficient_weir` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `coefo` (drop) | `items[*].drop.coefficient_orifice` | `watershed.impoundment.items[*].drop_spillway.coefficient_orifice` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `diabl` (drop) | `items[*].drop.base_diameter_m` | `watershed.impoundment.items[*].drop_spillway.base_diameter_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `hitbl` | `items[*].drop.inlet_height_m` | `watershed.impoundment.items[*].drop_spillway.inlet_height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `wdbl` | `items[*].drop.inlet_width_m` | `watershed.impoundment.items[*].drop_spillway.inlet_width_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `hrh` (drop) | `items[*].drop.riser_height_m` | `watershed.impoundment.items[*].drop_spillway.riser_height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `lbl` (drop) | `items[*].drop.barrel_length_m` | `watershed.impoundment.items[*].drop_spillway.barrel_length_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `sbl` (drop) | `items[*].drop.barrel_slope_m_per_m` | `watershed.impoundment.items[*].drop_spillway.barrel_slope_m_per_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `hblot` | `items[*].drop.outlet_stage_m` | `watershed.impoundment.items[*].drop_spillway.outlet_stage_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `ke` (drop) | `items[*].drop.loss_coeff_entrance` | `watershed.impoundment.items[*].drop_spillway.loss_coeff_entrance` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `kb` (drop) | `items[*].drop.loss_coeff_bend` | `watershed.impoundment.items[*].drop_spillway.loss_coeff_bend` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `kc` (drop) | `items[*].drop.loss_coeff_contraction` | `watershed.impoundment.items[*].drop_spillway.loss_coeff_contraction` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `outlet hydraulics routing` | `G-IMP-004`, `G-IMP-005` |
| `icv(1)` | `items[*].culverts[1].enabled_code` | `watershed.impoundment.items[*].culverts[1].enabled_code` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `ncv(1)` | `items[*].culverts[1].count` | `watershed.impoundment.items[*].culverts[1].count` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `arcv(1)` | `items[*].culverts[1].cross_section_area_m2` | `watershed.impoundment.items[*].culverts[1].cross_section_area_m2` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `hitcv(1)` | `items[*].culverts[1].inlet_height_m` | `watershed.impoundment.items[*].culverts[1].inlet_height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `hcv(1)` | `items[*].culverts[1].diameter_height_m` | `watershed.impoundment.items[*].culverts[1].diameter_height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `lcv(1)` | `items[*].culverts[1].length_m` | `watershed.impoundment.items[*].culverts[1].length_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `scv(1)` | `items[*].culverts[1].slope_m_per_m` | `watershed.impoundment.items[*].culverts[1].slope_m_per_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `hcvot(1)` | `items[*].culverts[1].outlet_stage_m` | `watershed.impoundment.items[*].culverts[1].outlet_stage_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `ke(1)` | `items[*].culverts[1].loss_coeff_entrance` | `watershed.impoundment.items[*].culverts[1].loss_coeff_entrance` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `kb(1)` | `items[*].culverts[1].loss_coeff_bend` | `watershed.impoundment.items[*].culverts[1].loss_coeff_bend` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `kc(1)` | `items[*].culverts[1].loss_coeff_contraction` | `watershed.impoundment.items[*].culverts[1].loss_coeff_contraction` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `kus(1)` | `items[*].culverts[1].submergence_coeff` | `watershed.impoundment.items[*].culverts[1].submergence_coeff` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `mus(1)` | `items[*].culverts[1].submergence_exp` | `watershed.impoundment.items[*].culverts[1].submergence_exp` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `cs(1)` | `items[*].culverts[1].discharge_coeff` | `watershed.impoundment.items[*].culverts[1].discharge_coeff` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `ys(1)` | `items[*].culverts[1].tailwater_depth_m` | `watershed.impoundment.items[*].culverts[1].tailwater_depth_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `icv(2)` | `items[*].culverts[2].enabled_code` | `watershed.impoundment.items[*].culverts[2].enabled_code` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `ncv(2)` | `items[*].culverts[2].count` | `watershed.impoundment.items[*].culverts[2].count` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `arcv(2)` | `items[*].culverts[2].cross_section_area_m2` | `watershed.impoundment.items[*].culverts[2].cross_section_area_m2` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `hitcv(2)` | `items[*].culverts[2].inlet_height_m` | `watershed.impoundment.items[*].culverts[2].inlet_height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `hcv(2)` | `items[*].culverts[2].diameter_height_m` | `watershed.impoundment.items[*].culverts[2].diameter_height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `lcv(2)` | `items[*].culverts[2].length_m` | `watershed.impoundment.items[*].culverts[2].length_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `scv(2)` | `items[*].culverts[2].slope_m_per_m` | `watershed.impoundment.items[*].culverts[2].slope_m_per_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `hcvot(2)` | `items[*].culverts[2].outlet_stage_m` | `watershed.impoundment.items[*].culverts[2].outlet_stage_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `ke(2)` | `items[*].culverts[2].loss_coeff_entrance` | `watershed.impoundment.items[*].culverts[2].loss_coeff_entrance` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `kb(2)` | `items[*].culverts[2].loss_coeff_bend` | `watershed.impoundment.items[*].culverts[2].loss_coeff_bend` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `kc(2)` | `items[*].culverts[2].loss_coeff_contraction` | `watershed.impoundment.items[*].culverts[2].loss_coeff_contraction` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `kus(2)` | `items[*].culverts[2].submergence_coeff` | `watershed.impoundment.items[*].culverts[2].submergence_coeff` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `mus(2)` | `items[*].culverts[2].submergence_exp` | `watershed.impoundment.items[*].culverts[2].submergence_exp` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `cs(2)` | `items[*].culverts[2].discharge_coeff` | `watershed.impoundment.items[*].culverts[2].discharge_coeff` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `ys(2)` | `items[*].culverts[2].tailwater_depth_m` | `watershed.impoundment.items[*].culverts[2].tailwater_depth_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `culvert outflow routing` | `G-IMP-006`, `G-IMP-007` |
| `irf` | `items[*].rockfill.enabled_code` | `watershed.impoundment.items[*].rockfill.enabled_code` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `rockfill spill routing` | `G-IMP-008` |
| `lnrf` | `items[*].rockfill.length_m` | `watershed.impoundment.items[*].rockfill.length_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `rockfill spill routing` | `G-IMP-008` |
| `hrf` | `items[*].rockfill.height_m` | `watershed.impoundment.items[*].rockfill.height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `rockfill spill routing` | `G-IMP-008` |
| `hotrf` | `items[*].rockfill.outlet_stage_m` | `watershed.impoundment.items[*].rockfill.outlet_stage_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `rockfill spill routing` | `G-IMP-008` |
| `wdrf` | `items[*].rockfill.width_m` | `watershed.impoundment.items[*].rockfill.width_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `rockfill spill routing` | `G-IMP-008` |
| `diarf` | `items[*].rockfill.diameter_m` | `watershed.impoundment.items[*].rockfill.diameter_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `rockfill spill routing` | `G-IMP-008` |
| `ies` | `items[*].emergency.kind` | `watershed.impoundment.items[*].emergency_spillway.kind` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `bwes` | `items[*].emergency.open_channel.bottom_width_m` | `watershed.impoundment.items[*].emergency_spillway.open_channel.bottom_width_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `sses` | `items[*].emergency.open_channel.side_slope` | `watershed.impoundment.items[*].emergency_spillway.open_channel.side_slope` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `nes` | `items[*].emergency.open_channel.manning_n` | `watershed.impoundment.items[*].emergency_spillway.open_channel.manning_n` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `hes` | `items[*].emergency.crest_stage_m` | `watershed.impoundment.items[*].emergency_spillway.crest_stage_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `hmxes` | `items[*].emergency.open_channel.max_stage_m` | `watershed.impoundment.items[*].emergency_spillway.open_channel.max_stage_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `ses1` | `items[*].emergency.open_channel.seg1_slope` | `watershed.impoundment.items[*].emergency_spillway.open_channel.seg1_slope` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `les1` | `items[*].emergency.open_channel.seg1_length_m` | `watershed.impoundment.items[*].emergency_spillway.open_channel.seg1_length_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `ses2` | `items[*].emergency.open_channel.seg2_slope` | `watershed.impoundment.items[*].emergency_spillway.open_channel.seg2_slope` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `les2` | `items[*].emergency.open_channel.seg2_length_m` | `watershed.impoundment.items[*].emergency_spillway.open_channel.seg2_length_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `ses3` | `items[*].emergency.open_channel.seg3_slope` | `watershed.impoundment.items[*].emergency_spillway.open_channel.seg3_slope` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `npts` | `items[*].emergency.rating_curve.point_count` | `watershed.impoundment.items[*].emergency_spillway.rating_curve.point_count` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `hest` | `items[*].emergency.rating_curve.stage_m[*]` | `watershed.impoundment.items[*].emergency_spillway.rating_curve.stage_m[*]` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `qes` | `items[*].emergency.rating_curve.flow_m3_s[*]` | `watershed.impoundment.items[*].emergency_spillway.rating_curve.flow_m3_s[*]` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `emergency outflow branch` | `G-IMP-009`, `G-IMP-010` |
| `iff` | `items[*].filter.kind` | `watershed.impoundment.items[*].filter_barrier.kind` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `sediment/filter routing modifiers` | `G-IMP-011` |
| `vsl` | `items[*].filter.velocity_m_s` | `watershed.impoundment.items[*].filter_barrier.velocity_m_s` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `sediment/filter routing modifiers` | `G-IMP-011` |
| `wdff` | `items[*].filter.width_m` | `watershed.impoundment.items[*].filter_barrier.width_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `sediment/filter routing modifiers` | `G-IMP-011` |
| `hff` | `items[*].filter.height_m` | `watershed.impoundment.items[*].filter_barrier.height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `sediment/filter routing modifiers` | `G-IMP-011` |
| `hotff` | `items[*].filter.outlet_stage_m` | `watershed.impoundment.items[*].filter_barrier.outlet_stage_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `sediment/filter routing modifiers` | `G-IMP-011` |
| `ipr` | `items[*].riser.enabled_code` | `watershed.impoundment.items[*].perforated_riser.enabled_code` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `hr` | `items[*].riser.height_m` | `watershed.impoundment.items[*].perforated_riser.height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `hb` | `items[*].riser.base_height_m` | `watershed.impoundment.items[*].perforated_riser.base_height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `hs` | `items[*].riser.slot_height_m` | `watershed.impoundment.items[*].perforated_riser.slot_height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `hd` | `items[*].riser.drawdown_height_m` | `watershed.impoundment.items[*].perforated_riser.drawdown_height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `diar` | `items[*].riser.riser_diameter_m` | `watershed.impoundment.items[*].perforated_riser.riser_diameter_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `as` | `items[*].riser.slot_area_m2` | `watershed.impoundment.items[*].perforated_riser.slot_area_m2` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `diab` | `items[*].riser.barrel_diameter_m` | `watershed.impoundment.items[*].perforated_riser.barrel_diameter_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `hrh` (riser) | `items[*].riser.headwall_height_m` | `watershed.impoundment.items[*].perforated_riser.headwall_height_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `lbl` (riser) | `items[*].riser.barrel_length_m` | `watershed.impoundment.items[*].perforated_riser.barrel_length_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `sbl` (riser) | `items[*].riser.barrel_slope_m_per_m` | `watershed.impoundment.items[*].perforated_riser.barrel_slope_m_per_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `diabl` (riser) | `items[*].riser.base_diameter_m` | `watershed.impoundment.items[*].perforated_riser.base_diameter_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `cb` | `items[*].riser.discharge_coeff_barrel` | `watershed.impoundment.items[*].perforated_riser.discharge_coeff_barrel` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `coefw` (riser) | `items[*].riser.coefficient_weir` | `watershed.impoundment.items[*].perforated_riser.coefficient_weir` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `coefo` (riser) | `items[*].riser.coefficient_orifice` | `watershed.impoundment.items[*].perforated_riser.coefficient_orifice` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `cs` (riser) | `items[*].riser.discharge_coeff_slot` | `watershed.impoundment.items[*].perforated_riser.discharge_coeff_slot` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `ke` (riser) | `items[*].riser.loss_coeff_entrance` | `watershed.impoundment.items[*].perforated_riser.loss_coeff_entrance` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `kb` (riser) | `items[*].riser.loss_coeff_bend` | `watershed.impoundment.items[*].perforated_riser.loss_coeff_bend` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `kc` (riser) | `items[*].riser.loss_coeff_contraction` | `watershed.impoundment.items[*].perforated_riser.loss_coeff_contraction` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `perforated-riser outflow branch` | `G-IMP-012` |
| `hot` | `items[*].misc.overtop_stage_m` | `watershed.impoundment.items[*].misc.overtop_stage_m` | `input::watershed::impoundment` | `init,watershed,daily,event` | `immutable` | `storage update loop and infiltration timestep policy` | `G-IMP-013` |
| `hfull` | `items[*].misc.full_sediment_stage_m` | `watershed.impoundment.items[*].misc.full_sediment_stage_m` | `input::watershed::impoundment` | `init,watershed,daily,event` | `immutable` | `storage update loop and infiltration timestep policy` | `G-IMP-013` |
| `h` | `items[*].misc.initial_stage_m` | `watershed.impoundment.items[*].misc.initial_stage_m` | `input::watershed::impoundment` | `init,watershed,daily,event` | `immutable` | `storage update loop and infiltration timestep policy` | `G-IMP-013` |
| `deltat` | `items[*].misc.initial_timestep_hr` | `watershed.impoundment.items[*].misc.initial_timestep_hr` | `input::watershed::impoundment` | `init,watershed,daily,event` | `immutable` | `storage update loop and infiltration timestep policy` | `G-IMP-013` |
| `qinf` | `items[*].misc.infiltration_rate_m_per_d` | `watershed.impoundment.items[*].misc.infiltration_rate_m_per_d` | `input::watershed::impoundment` | `init,watershed,daily,event` | `immutable` | `storage update loop and infiltration timestep policy` | `G-IMP-013` |
| `isize` | `items[*].misc.structure_size_class` | `watershed.impoundment.items[*].misc.structure_size_class` | `input::watershed::impoundment` | `init,watershed,daily,event` | `immutable` | `storage update loop and infiltration timestep policy` | `G-IMP-013` |
| `ndiv` | `items[*].misc.particle_subclass_divisions` | `watershed.impoundment.items[*].misc.particle_subclass_divisions` | `input::watershed::impoundment` | `init,watershed,daily,event` | `immutable` | `storage update loop and infiltration timestep policy` | `G-IMP-013` |
| `nalpts` | `items[*].curve.nalpts` | `watershed.impoundment.items[*].stage_area_length_curve.point_count` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `storage-area-length interpolation` | `G-IMP-014`, `G-IMP-015` |
| `hmin` | `items[*].curve.hmin` | `watershed.impoundment.items[*].stage_area_length_curve.hmin_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `storage-area-length interpolation` | `G-IMP-014`, `G-IMP-015` |
| `a0` | `items[*].curve.a0` | `watershed.impoundment.items[*].stage_area_length_curve.a0_m2` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `storage-area-length interpolation` | `G-IMP-014`, `G-IMP-015` |
| `l0` | `items[*].curve.l0` | `watershed.impoundment.items[*].stage_area_length_curve.l0_m` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `storage-area-length interpolation` | `G-IMP-014`, `G-IMP-015` |
| `hal` | `items[*].curve.hal[*]` | `watershed.impoundment.items[*].stage_area_length_curve.stage_m[*]` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `storage-area-length interpolation` | `G-IMP-014`, `G-IMP-015` |
| `area` | `items[*].curve.area[*]` | `watershed.impoundment.items[*].stage_area_length_curve.area_m2[*]` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `storage-area-length interpolation` | `G-IMP-014`, `G-IMP-015` |
| `length` | `items[*].curve.length[*]` | `watershed.impoundment.items[*].stage_area_length_curve.length_m[*]` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `storage-area-length interpolation` | `G-IMP-014`, `G-IMP-015` |
| derived `structure_enabled_flags` | `derived.structure_flags` | `watershed.impoundment.items[*].structure_flags` | `input::watershed::impoundment` | `init,watershed,event` | `immutable` | `branch dispatch for outlet hydraulics` | `G-IMP-016` |

## 5. State Ownership and Mutability

- `input::watershed::impoundment` owns parsed source structure and normalized impoundment parameter state.
- Parsed impoundment parameter structures are immutable after parse success.
- Hydrologic state variables (current storage, stage, outflow, sediment state) are mutable in runtime impoundment/routing modules only.
- Forbidden mutation path: runtime hydrology modules mutating canonical parsed branch parameters or stage-area-length input arrays.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-IMP-001` | Derive enabled-structure flags from `ids/icv/irf/ies/iff/ipr` branches. | per impoundment finalize | `C-IMP-001` |
| `D-IMP-002` | Derive emergency rating-curve cardinality closure from `npts`. | per impoundment finalize | `C-IMP-002` |
| `D-IMP-003` | Derive stage-area-length curve closure using `nalpts`. | per impoundment finalize | `C-IMP-003` |

Closure hooks:
- `C-IMP-001`: branch arity closure: enabled structures must have complete payload, disabled structures must not leak branch-only fields.
- `C-IMP-002`: if `ies=2`, `hest` and `qes` arrays must both exist and match `npts`.
- `C-IMP-003`: `hal/area/length` arrays must each have `nalpts` items and maintain non-decreasing stage ordering.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `IMP-E-000` | io | missing/unopenable `.imp` file |
| `IMP-E-001` | syntax | token parse failure in required numeric fields |
| `IMP-E-002` | syntax | unexpected EOF in branch or array payload |
| `IMP-E-003` | semantic | unsupported datver policy result |
| `IMP-E-004` | semantic | invalid declared count/domain (`jpond`, `nalpts`, `ndiv`, branch enums) |
| `IMP-E-005` | semantic | branch payload arity mismatch for enabled structure |
| `IMP-E-006` | semantic | invalid physical-domain values (negative geometry/rates where disallowed) |
| `IMP-E-007` | cross-file | impoundment count mismatch with structure-derived `npond` policy |
| `IMP-E-008` | runtime-guard | post-parse curve closure or branch-closure invariant failure |
| `IMP-E-009` | cross-file | impoundment ordering mismatch versus structural indexing from `.str` |
| `IMP-W-001` | compat-warning | no-datver compatibility preamble accepted |
| `IMP-W-002` | compat-warning | surplus `.imp` records ignored when `jpond > npond` in compatibility mode |

No silent fallback/default masking is permitted for malformed required branch payload.

## 8. Cross-File Consistency Constraints

1. `.str` structural impoundment count (`npond`) and `.imp` declared `jpond` must satisfy mode policy (`strict`: exact; `compat`: allow `jpond > npond` with warning and deterministic truncation). `[DIRECT][E-SPEC-IMP-01]`
2. Impoundment ordering must be consistent with structural element indexing used by watershed routines. `[DIRECT][E-SPEC-IMP-01]`, `[INFERENCE][E-WF-IMP-01]`
3. Maximum supported impoundment count policy (`usersum` narrative vs legacy `mximp`) remains explicit and enforced by parser configuration. `[DIRECT][E-SPEC-IMP-01]`
4. Watershed structure restrictions for channel/hillslope feed into impoundments are topology-validator responsibilities but must be linked to this surface as closure dependencies. `[DIRECT][E-SPEC-IMP-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `datver,jpond` | `watershed.impoundment.version`, `declared_count` | parser output manifest | canonical names + aliases (`file_version`,`impoundment_count_declared`) | no unit conversion at parser boundary |
| `strdes` | `watershed.impoundment.items[*].branch_comments[*]` | provenance payload | canonical branch comment text preserved with section-qualified provenance tags | required for source-fidelity/round-trip evidence |
| structure-branch symbols (`ids`,`icv`,`ncv`,`irf`,`ies`,`iff`,`ipr`) | `items[*].structure_flags` | watershed routing setup boundary | canonical symbols preserved plus normalized `structure_flags` | branch flags control required payload decoding |
| branch parameter payloads (drop/culvert/rockfill/emergency/filter/riser) | `items[*].{drop,culverts,rockfill,emergency,filter,riser}` | outlet-structure configuration boundary | typed nested records with canonical keys and alias map from Section 3 | branch-disabled records omitted, not default-filled |
| misc and curve symbols (`hot`,`hfull`,`h`,`deltat`,`qinf`,`isize`,`ndiv`,`nalpts`,`hmin`,`a0`,`l0`,`hal`,`area`,`length`) | `items[*].misc`, `items[*].curve` | storage/area-length interpolation boundary | canonical symbols preserved and aliases exported | array cardinality closure guaranteed before export |

## 10. Compatibility Policy

- Strict mode:
  - requires explicit datver line;
  - accepts datver only when `>= 94.301`;
  - enforces exact branch grammar and array closure;
  - enforces strict `jpond == npond` cross-file count closure.
- Compatibility mode:
  - may accept no-datver legacy preamble when explicitly enabled;
  - emits `IMP-W-001` on no-datver acceptance;
  - may allow `jpond > npond` with deterministic ignore of surplus impoundment records and emits `IMP-W-002`;
  - does not allow branch-arity violations or malformed array payload.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-IMP-001` | datver policy gate | header parse | `IMP-E-003` |
| `G-IMP-002` | `jpond` domain + configured max-count policy | header parse | `IMP-E-004` |
| `G-IMP-003` | required 3-line description block per impoundment | per-item parse | `IMP-E-002` |
| `G-IMP-004` | `ids` enum domain | drop section parse | `IMP-E-004` |
| `G-IMP-005` | drop-section conditional payload arity | drop section parse | `IMP-E-005` |
| `G-IMP-006` | `icv/ncv` domains for both culvert blocks | culvert section parse | `IMP-E-004` |
| `G-IMP-007` | culvert conditional payload arity and domain checks | culvert section parse | `IMP-E-005`/`IMP-E-006` |
| `G-IMP-008` | rockfill branch payload closure | rockfill section parse | `IMP-E-005` |
| `G-IMP-009` | `ies` enum domain | emergency section parse | `IMP-E-004` |
| `G-IMP-010` | emergency branch closure (`ies=1` open-channel, `ies=2` rating curve) | emergency section parse/finalize | `IMP-E-005` |
| `G-IMP-011` | filter barrier conditional payload closure | filter section parse | `IMP-E-005` |
| `G-IMP-012` | perforated riser conditional payload closure | riser section parse | `IMP-E-005` |
| `G-IMP-013` | misc-domain checks (`hot/hfull/h/deltat/qinf/isize/ndiv`) | misc parse | `IMP-E-006` |
| `G-IMP-014` | `nalpts` positive and curve baseline line required | curve parse | `IMP-E-004`/`IMP-E-002` |
| `G-IMP-015` | equal cardinality + monotone stage ordering in `hal/area/length` | curve finalize | `IMP-E-008` |
| `G-IMP-016` | cross-file count closure vs `.str` `npond` | cross-surface validator | `IMP-E-007` |
| `G-IMP-017` | impoundment ordering closure vs `.str` structural indexing | cross-surface validator | `IMP-E-009` |
| `G-IMP-018` | no-datver compatibility path must emit warning | preamble policy gate | `IMP-W-001` |
| `G-IMP-019` | surplus impoundment truncation in compatibility mode must emit warning | cross-surface validator | `IMP-W-002` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical variable names remain authoritative and unchanged, including:
`datver`, `jpond`, `ids`, `icv`, `ncv`, `irf`, `ies`, `iff`, `ipr`, `hot`,
`hfull`, `h`, `deltat`, `qinf`, `isize`, `ndiv`, `nalpts`, `hmin`, `a0`, `l0`,
`hal`, `area`, `length`, and branch-specific parameters listed in Section 3.

openWEPP runtime names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `IMP-GAP-001` | Maximum supported impoundment-count policy conflict (`usersum` 10 vs legacy `mximp=25`) needs explicit architecture decision. | `[DIRECT][E-SPEC-IMP-01]` | `HOLD` |
| `IMP-GAP-002` | Text-extraction ambiguity in usersum branch line numbering requires fixture-backed parse corpus validation. | `[DIRECT][E-SPEC-IMP-01]` | `HOLD` |
| `IMP-GAP-003` | Additional culvert coefficients (`kus/mus/cs/ys`) are legacy-observed but under-specified in usersum table; authority note remains partially inference-backed. | `[DIRECT][E-SPEC-IMP-01]`, `[INFERENCE][E-WF-IMP-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.1` | Added source-fidelity `strdes` coverage, symbol-level propagation coverage mapping, ordering guard, and explicit compatibility warning outcomes. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE07. |
