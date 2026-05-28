---
contract_id: SC-INFILE-MANAGEMENT-001
title: Management Input Parser Contract (.man)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.2.0
evidence_mode: Static
last_updated_utc: 2026-05-21T19:00:00Z
---

# SC-INFILE-MANAGEMENT-001 Management Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-MAN-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/plant-file.spec.md` (canonical openWEPP management section/scenario structure and datver notes).
- `[DIRECT][E-SURVEY-MAN-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (`.man` parser coverage and legacy/runtime provenance references).
- `[DIRECT][E-WF-MAN-01]` `/home/workdir/wepp-forest/src/infile.for` and `/home/workdir/wepp-forest/src/tilage.for` (legacy management parse/consumption references cited by survey).
- `[DIRECT][E-WP-MAN-01]` `/home/workdir/wepppy/wepppy/wepp/management/managements.py` (`Management._parse` and downgrade behavior noted in spec text/survey).
- `[INFERENCE][E-PHYS-MAN-01]` Physical/common-sense invariants: valid day/date domains, non-negative counts, index closure across section scenario arrays.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for surface `infile-management-man` (`.man`) and transformation from section/scenario source records into normalized simulation management schedules.

### 1.2 Version/Datver Applicability Matrix

| Datver | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- |
| `95.7` | Accept. | Parse baseline section/scenario structure. | `[DIRECT][E-SPEC-MAN-01]` |
| `98.4` | Accept. | Parse baseline structure and legacy-compatible option subsets. | `[DIRECT][E-SPEC-MAN-01]` |
| `2016.3` | Accept. | Parse extended residue/understory/permanent-contour option fields. | `[DIRECT][E-SPEC-MAN-01]` |
| `2017.1` | Accept. | Parse as `2016.3`-family extension until stricter divergence is identified. | `[DIRECT][E-SPEC-MAN-01]`, `[INFERENCE][E-WP-MAN-01]` |
| unknown | Strict reject. Compat reject unless explicitly allowlisted. | Emit typed `UnsupportedDatver`. | `[INFERENCE][E-SPEC-MAN-01]` |

### 1.3 Executable Parser Profile (INIMPL09)

- Non-zero section parsing is executable for canonical cropland scenario loops across plant, operation, initial, surface, contour, drain, yearly, and management sections.
- Parser output includes typed section registries and expanded management schedule slots.
- Executable option support profile:
  - annual/fallow residue option (`resmgt`) supports `1..6` for `95.7/98.4` and `1..7` for `2016.3/2017.1`;
  - perennial management option (`mgtopt`) currently executes `1..3`; higher `2016.3+` options (`4..7`) are typed `MAN-E-004`.
- Rangeland (`landuse=2`) paths are explicitly unsupported for openWEPP `.man` execution and rejected with typed `MAN-E-004`.
- Date-domain guard `G-MAN-008` is executable for parsed cropland surface/yearly date fields (`1..366`, with explicit `0` sentinel support only for perennial `jdharv`, `jdplt`, `jdstop`).

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
man_file = info_section plant_section operation_section initial_section
           surface_section contour_section drainage_section yearly_section
           management_section ;

info_section = datver_line nofe_or_nchan_line total_years_line ;
plant_section = ncrop plant_scenario{ncrop} ;
operation_section = nop op_scenario{nop} ;
initial_section = nini ini_scenario{nini} ;
surface_section = nseq surface_scenario{nseq} ;
contour_section = ncnt contour_scenario{ncnt} ;
drainage_section = ndrain drain_scenario{ndrain} ;
yearly_section = nscen yearly_scenario{nscen} ;
management_section = final_management_schedule ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves section-order and scenario-order identity, plus raw per-section fields by datver branch.
- Simulation model normalizes into:
  - typed scenario registries (`PlantScenario`, `OperationScenario`, `InitialScenario`, etc.),
  - typed yearly schedule graph,
  - expanded management timeline by OFE, year, crop slot.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `info.datver` | `management.version.datver` | none | string/real token | 1 | yes | all | none | `management.datver` |
| `nofe` / `nchan` | `info.ofe_or_channel_count` | `management.topology.count` | count | int | 1 | yes | all | none | `management.topology_count` |
| total years (`nyears*nrots` line) | `info.total_years` | `management.meta.total_years` | years | int | 1 | yes | all | closure checked vs schedule expansion | `management.total_years` |
| `ncrop` | `plant.count` | `management.plants.count` | count | int | 1 | yes | all | none | `plants.count` |
| `nop` | `operation.count` | `management.ops.count` | count | int | 1 | yes | all | none | `operations.count` |
| `nini` | `initial.count` | `management.initial.count` | count | int | 1 | yes | all | none | `initial.count` |
| `nseq` | `surface.count` | `management.surface.count` | count | int | 1 | yes | all | none | `surface.count` |
| `ncnt` | `contour.count` | `management.contour.count` | count | int | 1 | yes | all | none | `contour.count` |
| `ndrain` | `drain.count` | `management.drain.count` | count | int | 1 | yes | all | none | `drain.count` |
| `nscen` | `yearly.count` | `management.yearly.count` | count | int | 1 | yes | all | none | `yearly.count` |
| `lanuse` | `ini[i].lanuse` | `management.initial[i].landuse` | enum | int | nini | yes | all | none | `landuse` |
| `iplant` | `plant[i].iplant` | `management.plants[i].landuse` | enum | int | ncrop | yes | all | none | `plant_landuse` |
| `iop` | `op[i].iop` | `management.ops[i].landuse` | enum | int | nop | yes | all | none | `op_landuse` |
| `crname` | `plant[i].crname` | `management.plants[i].name` | text | string | ncrop | yes | all | none | `plants.name` |
| `crunit` | `plant[i].cropland.crunit` | `management.plants[i].cropland.crunit` | text | string | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.crunit` |
| `bb` | `plant[i].cropland.bb` | `management.plants[i].cropland.bb` | none | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.bb` |
| `bbb` | `plant[i].cropland.bbb` | `management.plants[i].cropland.bbb` | none | real | subset(ncrop, `iplant=1/2`) | conditional | all | none | `plants.params.bbb` |
| `beinp` | `plant[i].cropland.beinp` | `management.plants[i].cropland.beinp` | none | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.beinp` |
| `btemp` | `plant[i].cropland.btemp` | `management.plants[i].cropland.btemp_c` | degC | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.btemp_c` |
| `cf` | `plant[i].cropland.cf` | `management.plants[i].cropland.cf` | m^2/kg | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.cf` |
| `crit` | `plant[i].cropland.crit` | `management.plants[i].cropland.crit` | degC | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.crit` |
| `critvm` | `plant[i].cropland.critvm` | `management.plants[i].cropland.critvm` | kg/m^2 | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.critvm` |
| `cuthgt` | `plant[i].cropland.cuthgt` | `management.plants[i].cropland.cuthgt_m` | m | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.cuthgt_m` |
| `decfct` | `plant[i].cropland.decfct` | `management.plants[i].cropland.decfct` | fraction | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.decfct` |
| `diam` | `plant[i].cropland.diam` | `management.plants[i].cropland.diam_m` | m | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.diam_m` |
| `dlai` | `plant[i].cropland.dlai` | `management.plants[i].cropland.dlai` | none | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.dlai` |
| `dropfc` | `plant[i].cropland.dropfc` | `management.plants[i].cropland.dropfc` | fraction | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.dropfc` |
| `extnct` | `plant[i].cropland.extnct` | `management.plants[i].cropland.extnct` | none | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.extnct` |
| `fact` | `plant[i].cropland.fact` | `management.plants[i].cropland.fact` | none | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.fact` |
| `flivmx` | `plant[i].cropland.flivmx` | `management.plants[i].cropland.flivmx` | none | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.flivmx` |
| `gddmax` | `plant[i].cropland.gddmax` | `management.plants[i].cropland.gddmax` | degC | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.gddmax` |
| `hi` | `plant[i].cropland.hi` | `management.plants[i].cropland.hi` | none | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.hi` |
| `hmax` | `plant[i].params.hmax` | `management.plants[i].params.hmax_m` | m | real | subset(ncrop, `iplant=1/2`) | conditional | all | none | `plants.params.hmax_m` |
| `mfocod` | `plant[i].cropland.mfocod` | `management.plants[i].cropland.mfocod` | enum | int | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.mfocod` |
| `oratea` | `plant[i].cropland.oratea` | `management.plants[i].cropland.oratea` | none | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.oratea` |
| `orater` | `plant[i].cropland.orater` | `management.plants[i].cropland.orater` | none | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.orater` |
| `otemp` | `plant[i].cropland.otemp` | `management.plants[i].cropland.otemp_c` | degC | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.otemp_c` |
| `pitol` | `plant[i].params.pitol` | `management.plants[i].params.pitol` | none | real | subset(ncrop, `iplant=1/2`) | conditional | all | none | `plants.params.pitol` |
| `pltsp` | `plant[i].cropland.pltsp` | `management.plants[i].cropland.pltsp_m` | m | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.pltsp_m` |
| `rdmax` | `plant[i].cropland.rdmax` | `management.plants[i].cropland.rdmax_m` | m | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.rdmax_m` |
| `rsr` | `plant[i].cropland.rsr` | `management.plants[i].cropland.rsr` | none | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.rsr` |
| `rtmmax` | `plant[i].cropland.rtmmax` | `management.plants[i].cropland.rtmmax` | kg/m^2 | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.rtmmax` |
| `spriod` | `plant[i].cropland.spriod` | `management.plants[i].cropland.spriod_days` | days | int | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.spriod_days` |
| `tmpmax` | `plant[i].cropland.tmpmax` | `management.plants[i].cropland.tmpmax_c` | degC | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.tmpmax_c` |
| `tmpmin` | `plant[i].cropland.tmpmin` | `management.plants[i].cropland.tmpmin_c` | degC | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.tmpmin_c` |
| `xmxlai` | `plant[i].cropland.xmxlai` | `management.plants[i].cropland.xmxlai` | none | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.xmxlai` |
| `yld` | `plant[i].cropland.yld` | `management.plants[i].cropland.yld` | kg/m^2 | real | subset(ncrop, `iplant=1`) | conditional | all | none | `plants.cropland.yld` |
| `rcc` | `plant[i].cropland.rcc` | `management.plants[i].cropland.rcc` | none | real | subset(ncrop, `iplant=1`) | conditional | 2016.3+ | none | `plants.cropland.rcc` |
| `aca` | `plant[i].rangeland.aca` | `management.plants[i].rangeland.aca` | none | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.aca` |
| `aleaf` | `plant[i].rangeland.aleaf` | `management.plants[i].rangeland.aleaf` | none | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.aleaf` |
| `ar` | `plant[i].rangeland.ar` | `management.plants[i].rangeland.ar` | none | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.ar` |
| `bugs` | `plant[i].rangeland.bugs` | `management.plants[i].rangeland.bugs` | none | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.bugs` |
| `cf1` | `plant[i].rangeland.cf1` | `management.plants[i].rangeland.cf1` | fraction | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.cf1` |
| `cf2` | `plant[i].rangeland.cf2` | `management.plants[i].rangeland.cf2` | fraction | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.cf2` |
| `cn` | `plant[i].rangeland.cn` | `management.plants[i].rangeland.cn` | ratio | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.cn` |
| `cold` | `plant[i].rangeland.cold` | `management.plants[i].rangeland.cold` | kg/m^2 | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.cold` |
| `ffp` | `plant[i].rangeland.ffp` | `management.plants[i].rangeland.ffp_days` | days | int | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.ffp_days` |
| `gcoeff` | `plant[i].rangeland.gcoeff` | `management.plants[i].rangeland.gcoeff` | none | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.gcoeff` |
| `gdiam` | `plant[i].rangeland.gdiam` | `management.plants[i].rangeland.gdiam_m` | m | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.gdiam_m` |
| `ghgt` | `plant[i].rangeland.ghgt` | `management.plants[i].rangeland.ghgt_m` | m | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.ghgt_m` |
| `gpop` | `plant[i].rangeland.gpop` | `management.plants[i].rangeland.gpop` | count | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.gpop` |
| `gtemp` | `plant[i].rangeland.gtemp` | `management.plants[i].rangeland.gtemp_c` | degC | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.gtemp_c` |
| `plive` | `plant[i].rangeland.plive` | `management.plants[i].rangeland.plive` | kg/m^2 | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.plive` |
| `pscday` | `plant[i].rangeland.pscday` | `management.plants[i].rangeland.pscday` | day-of-year | int | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.pscday` |
| `rgcmin` | `plant[i].rangeland.rgcmin` | `management.plants[i].rangeland.rgcmin` | kg/m^2 | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.rgcmin` |
| `root10` | `plant[i].rangeland.root10` | `management.plants[i].rangeland.root10` | kg/m^2 | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.root10` |
| `rootf` | `plant[i].rangeland.rootf` | `management.plants[i].rangeland.rootf` | fraction | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.rootf` |
| `scday2` | `plant[i].rangeland.scday2` | `management.plants[i].rangeland.scday2` | day-of-year | int | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.scday2` |
| `scoeff` | `plant[i].rangeland.scoeff` | `management.plants[i].rangeland.scoeff` | none | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.scoeff` |
| `sdiam` | `plant[i].rangeland.sdiam` | `management.plants[i].rangeland.sdiam_m` | m | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.sdiam_m` |
| `shgt` | `plant[i].rangeland.shgt` | `management.plants[i].rangeland.shgt_m` | m | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.shgt_m` |
| `spop` | `plant[i].rangeland.spop` | `management.plants[i].rangeland.spop` | count | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.spop` |
| `tcoeff` | `plant[i].rangeland.tcoeff` | `management.plants[i].rangeland.tcoeff` | none | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.tcoeff` |
| `tdiam` | `plant[i].rangeland.tdiam` | `management.plants[i].rangeland.tdiam_m` | m | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.tdiam_m` |
| `tempmn` | `plant[i].rangeland.tempmn` | `management.plants[i].rangeland.tempmn_c` | degC | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.tempmn_c` |
| `thgt` | `plant[i].rangeland.thgt` | `management.plants[i].rangeland.thgt_m` | m | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.thgt_m` |
| `tpop` | `plant[i].rangeland.tpop` | `management.plants[i].rangeland.tpop` | count | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.tpop` |
| `wood` | `plant[i].rangeland.wood` | `management.plants[i].rangeland.wood` | fraction | real | subset(ncrop, `iplant=2`) | conditional | all | none | `plants.rangeland.wood` |
| `opname` | `op[i].opname` | `management.ops[i].name` | text | string | nop | yes | all | none | `ops.name` |
| `mfo1` | `op[i].cropland.mfo1` | `management.ops[i].cropland.mfo1` | none | real | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.mfo1` |
| `mfo2` | `op[i].cropland.mfo2` | `management.ops[i].cropland.mfo2` | none | real | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.mfo2` |
| `numof` | `op[i].cropland.numof` | `management.ops[i].cropland.numof` | count | int | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.numof` |
| `code1` | `op[i].cropland.code1` | `management.ops[i].cropland.code1` | enum | int | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.code1` |
| `resma1` | `op[i].cropland.resma1` | `management.ops[i].cropland.resma1` | enum | int | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.resma1` |
| `cltpos` | `op[i].cropland.cltpos` | `management.ops[i].cropland.cltpos` | enum | int | subset(nop, `iop=1 && code1=3`) | conditional | all | none | `ops.cropland.cltpos` |
| `rho` | `op[i].cropland.rho` | `management.ops[i].cropland.rho_m` | m | real | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.rho_m` |
| `rint` | `op[i].cropland.rint` | `management.ops[i].cropland.rint_m` | m | real | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.rint_m` |
| `rmfo1` | `op[i].cropland.rmfo1` | `management.ops[i].cropland.rmfo1` | none | real | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.rmfo1` |
| `rmfo2` | `op[i].cropland.rmfo2` | `management.ops[i].cropland.rmfo2` | none | real | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.rmfo2` |
| `rro` | `op[i].cropland.rro` | `management.ops[i].cropland.rro_m` | m | real | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.rro_m` |
| `surdis` | `op[i].cropland.surdis` | `management.ops[i].cropland.surdis` | fraction | real | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.surdis` |
| `tdmean` | `op[i].cropland.tdmean` | `management.ops[i].cropland.tdmean_m` | m | real | subset(nop, `iop=1`) | conditional | all | none | `ops.cropland.tdmean_m` |
| `resurf1` | `op[i].cropland.resurf1` | `management.ops[i].cropland.resurf1` | fraction | real | subset(nop, `iop=1`) | conditional | 2016.3+ | none | `ops.cropland.resurf1` |
| `resurnf1` | `op[i].cropland.resurnf1` | `management.ops[i].cropland.resurnf1` | fraction | real | subset(nop, `iop=1`) | conditional | 2016.3+ | none | `ops.cropland.resurnf1` |
| `iresa1` | `op[i].cropland.iresa1` | `management.ops[i].cropland.iresa1` | index | int | subset(nop, operation-dependent) | conditional | all | none | `ops.cropland.iresa1` |
| `frmov1` | `op[i].cropland.frmov1` | `management.ops[i].cropland.frmov1` | fraction | real | subset(nop, operation-dependent) | conditional | all | none | `ops.cropland.frmov1` |
| `fbma1` | `op[i].cropland.fbma1` | `management.ops[i].cropland.fbma1` | fraction | real | subset(nop, operation-dependent) | conditional | all | none | `ops.cropland.fbma1` |
| `frfmov1` | `op[i].cropland.frfmov1` | `management.ops[i].cropland.frfmov1` | fraction | real | subset(nop, operation-dependent) | conditional | all | none | `ops.cropland.frfmov1` |
| `resad1` | `op[i].cropland.resad1` | `management.ops[i].cropland.resad1` | kg/m^2 | real | subset(nop, operation-dependent) | conditional | all | none | `ops.cropland.resad1` |
| `fbrnol` | `op[i].cropland.fbrnol` | `management.ops[i].cropland.fbrnol` | fraction | real | subset(nop, operation-dependent) | conditional | all | none | `ops.cropland.fbrnol` |
| `frsmov1` | `op[i].cropland.frsmov1` | `management.ops[i].cropland.frsmov1` | fraction | real | subset(nop, operation-dependent) | conditional | all | none | `ops.cropland.frsmov1` |
| `oname` | `ini[i].oname` | `management.initial[i].name` | text | string | nini | yes | all | none | `initial.name` |
| `bdtill` | `ini[i].cropland.bdtill` | `management.initial[i].cropland.bdtill` | g/cm^3 | real | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.bdtill` |
| `cancov` | `ini[i].params.cancov` | `management.initial[i].params.cancov` | fraction | real | subset(nini, `lanuse=1/2`) | conditional | all | none | `initial.params.cancov` |
| `daydis` | `ini[i].cropland.daydis` | `management.initial[i].cropland.daydis` | days | real | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.daydis` |
| `dsharv` | `ini[i].cropland.dsharv` | `management.initial[i].cropland.dsharv` | days | int | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.dsharv` |
| `frdp` | `ini[i].params.frdp` | `management.initial[i].params.frdp_m` | m | real | subset(nini, `lanuse=1/2`) | conditional | all | none | `initial.params.frdp_m` |
| `inrcov` | `ini[i].cropland.inrcov` | `management.initial[i].cropland.inrcov` | fraction | real | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.inrcov` |
| `iresd` | `ini[i].cropland.iresd` | `management.initial[i].cropland.iresd` | index | int | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.iresd` |
| `imngmt` | `ini[i].cropland.imngmt` | `management.initial[i].cropland.imngmt` | enum | int | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.imngmt` |
| `imngmt` | `yearly[*].imngmt` | `management.yearly[*].imngmt` | enum | int | subset(nscen, `iscen=1`) | conditional | all | none | `yearly.imngmt` |
| `rfcum` | `ini[i].cropland.rfcum` | `management.initial[i].cropland.rfcum_mm` | mm | real | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.rfcum_mm` |
| `rhinit` | `ini[i].cropland.rhinit` | `management.initial[i].cropland.rhinit_m` | m | real | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.rhinit_m` |
| `rilcov` | `ini[i].cropland.rilcov` | `management.initial[i].cropland.rilcov` | fraction | real | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.rilcov` |
| `rrinit` | `ini[i].cropland.rrinit` | `management.initial[i].cropland.rrinit_m` | m | real | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.rrinit_m` |
| `rspace` | `ini[i].cropland.rspace` | `management.initial[i].cropland.rspace_m` | m | real | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.rspace_m` |
| `rtyp` | `ini[i].cropland.rtyp` | `management.initial[i].cropland.rtyp` | enum | int | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.rtyp` |
| `snodpy` | `ini[i].params.snodpy` | `management.initial[i].params.snodpy_m` | m | real | subset(nini, `lanuse=1/2`) | conditional | all | none | `initial.params.snodpy_m` |
| `thdp` | `ini[i].params.thdp` | `management.initial[i].params.thdp_m` | m | real | subset(nini, `lanuse=1/2`) | conditional | all | none | `initial.params.thdp_m` |
| `tillay(1)` | `ini[i].params.tillay1` | `management.initial[i].params.tillay1_m` | m | real | subset(nini, `lanuse=1/2`) | conditional | all | none | `initial.params.tillay1_m` |
| `tillay(2)` | `ini[i].params.tillay2` | `management.initial[i].params.tillay2_m` | m | real | subset(nini, `lanuse=1/2`) | conditional | all | none | `initial.params.tillay2_m` |
| `width` | `ini[i].cropland.width` | `management.initial[i].cropland.width_m` | m | real | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.width_m` |
| `sumrtm` | `ini[i].cropland.sumrtm` | `management.initial[i].cropland.sumrtm` | kg/m^2 | real | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.sumrtm` |
| `sumsrm` | `ini[i].cropland.sumsrm` | `management.initial[i].cropland.sumsrm` | kg/m^2 | real | subset(nini, `lanuse=1`) | conditional | all | none | `initial.cropland.sumsrm` |
| `usinrcol` | `ini[i].cropland.usinrcol` | `management.initial[i].cropland.usinrcol` | fraction | real | subset(nini, `lanuse=1`) | conditional | 2016.3+ | none | `initial.cropland.usinrcol` |
| `usrilcol` | `ini[i].cropland.usrilcol` | `management.initial[i].cropland.usrilcol` | fraction | real | subset(nini, `lanuse=1`) | conditional | 2016.3+ | none | `initial.cropland.usrilcol` |
| `pptg` | `ini[i].rangeland.pptg` | `management.initial[i].rangeland.pptg_m` | m | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.pptg_m` |
| `rmagt` | `ini[i].rangeland.rmagt` | `management.initial[i].rangeland.rmagt` | kg/m^2 | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.rmagt` |
| `rmogt` | `ini[i].rangeland.rmogt` | `management.initial[i].rangeland.rmogt` | kg/m^2 | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.rmogt` |
| `rrough` | `ini[i].rangeland.rrough` | `management.initial[i].rangeland.rrough_m` | m | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.rrough_m` |
| `resi` | `ini[i].rangeland.resi` | `management.initial[i].rangeland.resi` | fraction | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.resi` |
| `roki` | `ini[i].rangeland.roki` | `management.initial[i].rangeland.roki` | fraction | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.roki` |
| `basi` | `ini[i].rangeland.basi` | `management.initial[i].rangeland.basi` | fraction | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.basi` |
| `cryi` | `ini[i].rangeland.cryi` | `management.initial[i].rangeland.cryi` | fraction | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.cryi` |
| `resr` | `ini[i].rangeland.resr` | `management.initial[i].rangeland.resr` | fraction | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.resr` |
| `rokr` | `ini[i].rangeland.rokr` | `management.initial[i].rangeland.rokr` | fraction | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.rokr` |
| `basr` | `ini[i].rangeland.basr` | `management.initial[i].rangeland.basr` | fraction | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.basr` |
| `cryr` | `ini[i].rangeland.cryr` | `management.initial[i].rangeland.cryr` | fraction | real | subset(nini, `lanuse=2`) | conditional | all | none | `initial.rangeland.cryr` |
| `sname` | `surf[s].sname` | `management.surface[s].name` | text | string | nseq | yes | all | none | `surface.name` |
| `iseq` | `surf[s].iseq` | `management.surface[s].landuse` | enum | int | nseq | yes | all | none | `surface.landuse` |
| `ntill` | `surf[s].ntill` | `management.surface[s].op_count` | count | int | nseq | yes | all | none | `surface.op_count` |
| `mdate` | `surf[s].ops[k].mdate` | `management.surface[s].ops[k].day` | day-of-year | int | sum(ntill) | conditional | all | none | `surface.ops.day` |
| `op` | `surf[s].ops[k].op` | `management.surface[s].ops[k].op_ref` | index | int | sum(ntill) | conditional | all | validates against `nop` | `surface.ops.op_ref` |
| `tildep` | `surf[s].ops[k].tildep` | `management.surface[s].ops[k].tildep_m` | m | real | sum(ntill) | conditional | all | none | `surface.ops.tildep_m` |
| `typtil` | `surf[s].ops[k].typtil` | `management.surface[s].ops[k].typtil` | enum | int | sum(ntill) | conditional | all | none | `surface.ops.typtil` |
| `cname` | `cont[c].cname` | `management.contour[c].name` | text | string | ncnt | yes | all | none | `contour.name` |
| `icont` | `cont[c].icont` | `management.contour[c].landuse` | enum | int | ncnt | conditional | all | must be `1` by spec | `contour.landuse` |
| `cntslp` | `cont[c].cntslp` | `management.contour[c].cntslp` | m/m | real | ncnt | conditional | all | none | `contour.cntslp` |
| `rdghgt` | `cont[c].rdghgt` | `management.contour[c].rdghgt_m` | m | real | ncnt | conditional | all | none | `contour.rdghgt_m` |
| `rowlen` | `cont[c].rowlen` | `management.contour[c].rowlen_m` | m | real | ncnt | conditional | all | none | `contour.rowlen_m` |
| `rowspc` | `cont[c].rowspc` | `management.contour[c].rowspc_m` | m | real | ncnt | conditional | all | none | `contour.rowspc_m` |
| `contours_perm` | `cont[c].contours_perm` | `management.contour[c].permanent_flag` | flag | int | ncnt | conditional | 2016.3+ | none | `contour.permanent_flag` |
| `dname` | `drain[d].dname` | `management.drain[d].name` | text | string | ndrain | yes | all | none | `drain.name` |
| `dcont` | `drain[d].dcont` | `management.drain[d].landuse` | enum | int | ndrain | conditional | all | must be one of `1,2,4` by spec | `drain.landuse` |
| `ddrain` | `drain[d].ddrain` | `management.drain[d].depth_m` | m | real | subset(ndrain, `dcont=1`) | conditional | all | none | `drain.depth_m` |
| `drainc` | `drain[d].drainc` | `management.drain[d].coef_m_d` | m/day | real | subset(ndrain, `dcont=1`) | conditional | all | none | `drain.coef_m_d` |
| `drdiam` | `drain[d].drdiam` | `management.drain[d].diameter_m` | m | real | subset(ndrain, `dcont=1`) | conditional | all | none | `drain.diameter_m` |
| `sdrain` | `drain[d].sdrain` | `management.drain[d].spacing_m` | m | real | subset(ndrain, `dcont=1`) | conditional | all | none | `drain.spacing_m` |
| `mname` | `yearly[y].mname` | `management.yearly[y].name` | text | string | nscen | yes | all | none | `yearly.name` |
| `iscen` | `yearly[y].iscen` | `management.yearly[y].landuse` | enum | int | nscen | yes | all | none | `yearly.landuse` |
| `conset` | `yearly[y].conset` | `management.yearly[y].contour_ref` | index | int | subset(nscen, `iscen=1`) | conditional | all | `0` allowed when `ncnt=0` | `yearly.contour_ref` |
| `resmgt` | `yearly[y].resmgt` | `management.yearly[y].residue_option` | enum | int | subset(nscen, `iscen=1`) | conditional | all | none | `yearly.residue_option` |
| `jdharv` | `yearly[y].jdharv` | `management.yearly[y].jdharv` | day-of-year | int | subset(nscen, `iscen=1`) | conditional | all | `0` allowed for perennial no-senescence cases | `yearly.jdharv` |
| `jdplt` | `yearly[y].jdplt` | `management.yearly[y].jdplt` | day-of-year | int | subset(nscen, `iscen=1`) | conditional | all | `0` allowed for established perennial cases | `yearly.jdplt` |
| `rw` | `yearly[y].rw` | `management.yearly[y].row_width_m` | m | real | subset(nscen, `iscen=1`) | conditional | all | none | `yearly.row_width_m` |
| `jdherb` | `yearly[y].jdherb` | `management.yearly[y].jdherb` | day-of-year | int | subset(nscen, option-dependent) | conditional | all | none | `yearly.jdherb` |
| `jdburn` | `yearly[y].jdburn` | `management.yearly[y].jdburn` | day-of-year | int | subset(nscen, option-dependent) | conditional | all | none | `yearly.jdburn` |
| `fbmag` | `yearly[y].fbmag` | `management.yearly[y].fbmag` | fraction | real | subset(nscen, option-dependent) | conditional | all | none | `yearly.fbmag` |
| `fbrnog` | `yearly[y].fbrnog` | `management.yearly[y].fbrnog` | fraction | real | subset(nscen, option-dependent) | conditional | all | none | `yearly.fbrnog` |
| `jdslge` | `yearly[y].jdslge` | `management.yearly[y].jdslge` | day-of-year | int | subset(nscen, option-dependent) | conditional | all | none | `yearly.jdslge` |
| `jdcut` | `yearly[y].jdcut` | `management.yearly[y].jdcut` | day-of-year | int | subset(nscen, option-dependent) | conditional | all | none | `yearly.jdcut` |
| `frcut` | `yearly[y].frcut` | `management.yearly[y].frcut` | fraction | real | subset(nscen, option-dependent) | conditional | all | none | `yearly.frcut` |
| `jdmove` | `yearly[y].jdmove` | `management.yearly[y].jdmove` | day-of-year | int | subset(nscen, option-dependent) | conditional | all | none | `yearly.jdmove` |
| `frmove` | `yearly[y].frmove` | `management.yearly[y].frmove` | fraction | real | subset(nscen, option-dependent) | conditional | all | none | `yearly.frmove` |
| `jdstop` | `yearly[y].jdstop` | `management.yearly[y].jdstop` | day-of-year | int | subset(nscen, option-dependent) | conditional | all | none | `yearly.jdstop` |
| `ncut` | `yearly[y].ncut` | `management.yearly[y].ncut` | count | int | subset(nscen, `mgtopt=1`) | conditional | all | none | `yearly.ncut` |
| `cutday` | `yearly[y].cutday[k]` | `management.yearly[y].cutday[k]` | day-of-year | int | sum(ncut) | conditional | all | none | `yearly.cutday` |
| `ncycle` | `yearly[y].ncycle` | `management.yearly[y].ncycle` | count | int | subset(nscen, `mgtopt` grazing modes) | conditional | all | none | `yearly.ncycle` |
| `animal` | `yearly[y].animal[*]` | `management.yearly[y].animal[*]` | count | real | variable | conditional | all | none | `yearly.animal` |
| `area` | `yearly[y].area[*]` | `management.yearly[y].area_m2[*]` | m^2 | real | variable | conditional | all | none | `yearly.area_m2` |
| `bodywt` | `yearly[y].bodywt[*]` | `management.yearly[y].bodywt_kg[*]` | kg | real | variable | conditional | all | none | `yearly.bodywt_kg` |
| `digest` | `yearly[y].digest[*]` | `management.yearly[y].digest[*]` | fraction | real | variable | conditional | all | none | `yearly.digest` |
| `gday` | `yearly[y].gday[*]` | `management.yearly[y].gday[*]` | day-of-year | int | variable | conditional | all | none | `yearly.gday` |
| `gend` | `yearly[y].gend[*]` | `management.yearly[y].gend[*]` | day-of-year | int | variable | conditional | all | none | `yearly.gend` |
| `grazig` | `yearly[y].rangeland.grazig` | `management.yearly[y].rangeland.grazig` | flag | int | subset(nscen, `iscen=2`) | conditional | all | none | `yearly.rangeland.grazig` |
| `access` | `yearly[y].rangeland.access` | `management.yearly[y].rangeland.access` | fraction | real | subset(nscen, `iscen=2`) | conditional | all | none | `yearly.rangeland.access` |
| `digmax` | `yearly[y].rangeland.digmax` | `management.yearly[y].rangeland.digmax` | fraction | real | subset(nscen, `iscen=2`) | conditional | all | none | `yearly.rangeland.digmax` |
| `digmin` | `yearly[y].rangeland.digmin` | `management.yearly[y].rangeland.digmin` | fraction | real | subset(nscen, `iscen=2`) | conditional | all | none | `yearly.rangeland.digmin` |
| `suppmt` | `yearly[y].rangeland.suppmt` | `management.yearly[y].rangeland.suppmt_kg_d` | kg/day | real | subset(nscen, `iscen=2`) | conditional | all | none | `yearly.rangeland.suppmt_kg_d` |
| `jgraz` | `yearly[y].rangeland.jgraz` | `management.yearly[y].rangeland.jgraz` | count | int | subset(nscen, `iscen=2`) | conditional | all | none | `yearly.rangeland.jgraz` |
| `send` | `yearly[y].rangeland.send` | `management.yearly[y].rangeland.send` | day-of-year | int | subset(nscen, `iscen=2`) | conditional | all | none | `yearly.rangeland.send` |
| `ssday` | `yearly[y].rangeland.ssday` | `management.yearly[y].rangeland.ssday` | day-of-year | int | subset(nscen, `iscen=2`) | conditional | all | none | `yearly.rangeland.ssday` |
| `ihdate` | `yearly[y].rangeland.ihdate` | `management.yearly[y].rangeland.ihdate` | day-of-year | int | subset(nscen, `iscen=2`) | conditional | all | none | `yearly.rangeland.ihdate` |
| `active` | `yearly[y].rangeland.active` | `management.yearly[y].rangeland.active` | flag | int | subset(nscen, herb branch) | conditional | all | none | `yearly.rangeland.active` |
| `dleaf` | `yearly[y].rangeland.dleaf` | `management.yearly[y].rangeland.dleaf` | fraction | real | subset(nscen, herb branch) | conditional | all | none | `yearly.rangeland.dleaf` |
| `herb` | `yearly[y].rangeland.herb` | `management.yearly[y].rangeland.herb` | fraction | real | subset(nscen, herb branch) | conditional | all | none | `yearly.rangeland.herb` |
| `regrow` | `yearly[y].rangeland.regrow` | `management.yearly[y].rangeland.regrow` | fraction | real | subset(nscen, herb branch) | conditional | all | none | `yearly.rangeland.regrow` |
| `update` | `yearly[y].rangeland.update` | `management.yearly[y].rangeland.update` | fraction | real | subset(nscen, herb branch) | conditional | all | none | `yearly.rangeland.update` |
| `woody` | `yearly[y].rangeland.woody` | `management.yearly[y].rangeland.woody` | flag | int | subset(nscen, herb branch) | conditional | all | none | `yearly.rangeland.woody` |
| `jfdate` | `yearly[y].rangeland.jfdate` | `management.yearly[y].rangeland.jfdate` | day-of-year | int | subset(nscen, burn branch) | conditional | all | none | `yearly.rangeland.jfdate` |
| `alter` | `yearly[y].rangeland.alter` | `management.yearly[y].rangeland.alter` | fraction | real | subset(nscen, burn branch) | conditional | all | none | `yearly.rangeland.alter` |
| `burned` | `yearly[y].rangeland.burned` | `management.yearly[y].rangeland.burned` | fraction | real | subset(nscen, burn branch) | conditional | all | none | `yearly.rangeland.burned` |
| `change` | `yearly[y].rangeland.change` | `management.yearly[y].rangeland.change` | fraction | real | subset(nscen, burn branch) | conditional | all | none | `yearly.rangeland.change` |
| `hurt` | `yearly[y].rangeland.hurt` | `management.yearly[y].rangeland.hurt` | fraction | real | subset(nscen, burn branch) | conditional | all | none | `yearly.rangeland.hurt` |
| `reduce` | `yearly[y].rangeland.reduce` | `management.yearly[y].rangeland.reduce` | fraction | real | subset(nscen, burn branch) | conditional | all | none | `yearly.rangeland.reduce` |
| `mgtopt` | `yearly[*].mgtopt` | `management.yearly[*].mgmt_option` | enum | int | variable | conditional | all | none | `mgmt_option` |
| `itype` | `yearly[*].itype` | `management.yearly[*].plant_scenario_ref` | index | int | variable | conditional | all | validates against `ncrop` | `plant_ref` |
| `tilseq` | `yearly[*].tilseq` | `management.yearly[*].surface_effect_ref` | index | int | variable | conditional | all | strict: validates against `nseq`; compatibility: `0` allowed as explicit no-surface-effect sentinel even when `nseq>0` | `surface_effect_ref` |
| `drset` | `yearly[*].drset` | `management.yearly[*].drain_ref` | index | int | variable | conditional | all | `0` allowed when `ndrain=0` | `drain_ref` |
| `ofeindx` | `management.ofe_loop[*]` | `management.schedule.ofe_initial_ref[*]` | index | int | nofe | yes | all | validates against `nini` | `ofe_initial_ref` |
| `nrots` | `management.nrots` | `management.schedule.rotation_repeats` | count | int | 1 | yes | all | none | `rotation_repeats` |
| `nyears` | `management.nyears` | `management.schedule.rotation_years` | count | int | 1 | yes | all | none | `rotation_years` |
| `nycrop` | `management.year[y].nycrop` | `management.schedule.year[y].crop_slots` | count | int | nyears*nrots*nofe | yes | all | none | `crop_slots` |
| `manindx` | `management.year[y].crop[c].manindx` | `management.schedule.year[y].crop[c].yearly_ref` | index | int | sum(nycrop) | yes | all | validates against `nscen` | `yearly_ref` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `info.datver` | `management.version` | `input::management` | init | immutable | compatibility policy gate | `G-MAN-001` |
| `nofe/nchan` | `info.topology_count` | `management.topology.count` | `input::management` | init | immutable | topology closure with slope/soil | `G-MAN-002` |
| section counts (`ncrop`, `nop`, `nini`, `nseq`, `ncnt`, `ndrain`, `nscen`) | `section_counts` | `management.registry.counts` | `input::management` | init | immutable | scenario index validation | `G-MAN-003` |
| plant/op/initial/surface/contour/drain scenario blocks | `scenarios.*` | `management.registry.*` | `input::management` | init,daily,event | immutable | yearly schedule expansion and dispatch preparation | `G-MAN-004`, `G-MAN-005` |
| yearly references (`itype`,`tilseq`,`conset`,`drset`) | `yearly[*].refs` | `management.yearly_refs` | `input::management` | init,annual | immutable | annual simulation planner | `G-MAN-006` |
| Julian-day schedule fields (`mdate`,`jdharv`,`jdplt`,`jdherb`,`jdburn`,`jdslge`,`jdcut`,`jdmove`,`jdstop`,`gday`,`gend`,`send`,`ssday`,`ihdate`,`jfdate`,`cutday`) | `yearly[*].date_fields` | `management.timeline.date_events` | `input::management` | init,annual,daily | immutable | daily/event management dispatch and crop-transition timing | `G-MAN-008` |
| final schedule refs (`ofeindx`,`manindx`,`nycrop`,`nyears`,`nrots`) | `schedule.raw` | `management.schedule.expanded` | `input::management` | init,annual | immutable post-parse | daily/event management dispatch | `G-MAN-006`, `G-MAN-007` |

## 5. State Ownership and Mutability

- `input::management` owns parsed source and normalized schedule graph.
- Parsed section/scenario registries are immutable after parse.
- Expanded schedule objects are immutable as input state; runtime crop/residue process state is mutable in separate owned process modules.
- Forbidden mutation path: runtime modules rewriting scenario registries or reference indices.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-MAN-001` | Expand nested management loops (`nrots`,`nyears`,`nofe`,`nycrop`) into linearized schedule view. | parse finalize | `C-MAN-001` |
| `D-MAN-002` | Derive expected total years from schedule and compare to info-section total-year declaration. | parse finalize | `C-MAN-002` |
| `D-MAN-003` | Resolve scenario index references to typed pointers/IDs. | parse finalize | `C-MAN-003` |

Closure hooks:
- `C-MAN-001`: loop cardinality closure.
- `C-MAN-002`: total-year closure.
- `C-MAN-003`: index-graph closure (no dangling scenario refs).

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `MAN-E-001` | syntax | token parse failure for required numeric fields |
| `MAN-E-002` | syntax | missing section/record before expected count closure |
| `MAN-E-003` | semantic | unsupported datver |
| `MAN-E-004` | semantic | invalid enum/options domain (`lanuse`, `iplant`, `iop`, `mgtopt`, etc.) and explicit unsupported `landuse=2` execution paths |
| `MAN-E-005` | semantic | negative or invalid section counts |
| `MAN-E-006` | semantic | section ordering violation |
| `MAN-E-007` | cross-file | topology mismatch with slope/soil/watershed surfaces |
| `MAN-E-008` | cross-file | schedule year mismatch vs climate/run configuration |
| `MAN-E-009` | runtime-guard | derived schedule closure failure / dangling index graph |
| `MAN-E-010` | semantic | invalid Julian-day/date domain (`DateDomainError`) |

No silent fallback masking for invalid required management structure.

## 8. Cross-File Consistency Constraints

1. `nofe`/`nchan` must match topology expectations from paired slope/watershed and soil surfaces. `[DIRECT][E-SPEC-MAN-01]`, `[INFERENCE][E-SURVEY-MAN-01]`
2. `nyears*nrots` schedule must be compatible with climate forcing record horizon. `[INFERENCE][E-SURVEY-MAN-01]`, `[INFERENCE][E-PHYS-MAN-01]`
3. OFE-level management references (`ofeindx`) must map to valid initial-condition scenarios and consistent OFE partitioning. `[DIRECT][E-SPEC-MAN-01]`
4. Scenario indices (`itype`, `tilseq`, `drset`, `manindx`) must resolve against declared scenario counts; compatibility mode additionally permits `tilseq=0` as an explicit no-surface-effect sentinel. `[DIRECT][E-SPEC-MAN-01]`, `[INFERENCE][E-SURVEY-MAN-01]`
5. Datver-specific fields (2016.3+ residue/understory/permanent-contour extensions) must only appear in compatible datver families. `[DIRECT][E-SPEC-MAN-01]`, `[DIRECT][E-WP-MAN-01]`
6. Julian-day schedule fields must remain in valid day domains (`1..366`, with `0` only where explicitly allowed by the canonical spec). `[DIRECT][E-SPEC-MAN-01]`, `[INFERENCE][E-PHYS-MAN-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `datver,nofe/nchan,nyears*nrots` and section counts (`ncrop,nop,nini,nseq,ncnt,ndrain,nscen`) | `management.version`, `management.topology`, `management.meta`, `management.registry.counts` | hillslope/watershed parser output payload | canonical names preserved in metadata/count block | no parser-side coercion |
| section-local scenario payload symbols from Section 3 | `management.registry.*` | interchange/config snapshot export | nested section-keyed records with canonical symbol keys and alias metadata | preserves full branch payload fidelity across process boundaries |
| yearly schedule references (`itype,tilseq,conset,drset,mgtopt`) | `management.yearly_refs` | annual planner boundary | integer index fields with canonical names | index domains validated before export |
| date/event symbols (`mdate,jdharv,jdplt,jdherb,jdburn,jdslge,jdcut,jdmove,jdstop,gday,gend,send,ssday,ihdate,jfdate,cutday`) | `management.timeline.date_events` | daily/event dispatch boundary | canonical symbol names + normalized day-of-year integer type | strict mode enforces domain checks before export |
| schedule loop symbols (`ofeindx,nrots,nyears,nycrop,manindx`) | `management.schedule.expanded` | runtime dispatch payload | canonical names with explicit aliases (`ofe_initial_ref`,`rotation_repeats`,`rotation_years`,`crop_slots`,`yearly_ref`) | expanded schedule is immutable post-parse |

## 10. Compatibility Policy

- Strict mode:
  - enforce section order and count closure;
  - reject unknown datver or datver-incompatible extension fields;
  - reject Julian-day/date fields outside contract domains;
  - reject rangeland (`landuse=2`) execution paths with typed unsupported behavior;
  - reject unsupported option enums and dangling scenario references.
- Compatibility mode:
  - may accept select legacy forms where mapping is lossless and explicit;
  - accepts first-token parsing for single-token control records while preserving the same invariant/error checks;
  - may accept legacy date sentinels only where canonical spec already permits them (`0` for specific perennial fields such as `jdharv` and `jdplt`);
  - may accept `tilseq=0` when `nseq>0` as a legacy no-surface-effect sentinel
    (value is preserved as `0` and not remapped);
  - must still fail on broken index closure or invalid section counts.

Downgrade/export behaviors (e.g., 2016.3 to 98.4) are external transformation workflows and not parser-side silent coercions.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-MAN-001` | datver allowlist | info parse | `MAN-E-003` |
| `G-MAN-002` | topology count positive and coherent | info parse + cross-file gate | `MAN-E-005`/`MAN-E-007` |
| `G-MAN-003` | section counts valid | section header parse | `MAN-E-005` |
| `G-MAN-004` | section-order and per-section row arity closure | section parse | `MAN-E-002`/`MAN-E-006` |
| `G-MAN-005` | scenario reference domains (compatibility exception: `tilseq=0` sentinel allowed) | yearly parse | `MAN-E-009` |
| `G-MAN-006` | schedule loop closure | schedule derivation | `MAN-E-009` |
| `G-MAN-007` | total-year closure vs declared years | schedule derivation | `MAN-E-008` |
| `G-MAN-008` | Julian-day/date-domain validity with explicit allowed sentinels | yearly/surface parse + closure hook | `MAN-E-010` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical WEPP/legacy symbols are retained for parser-contract traceability (`datver`, `nofe`, `nchan`, `ncrop`, `nop`, `nini`, `nscen`, `lanuse`, `iplant`, `iop`, `mgtopt`, `itype`, `tilseq`, `drset`, `ofeindx`, `nrots`, `nyears`, `nycrop`, `manindx`, and section-local symbols from the canonical spec).

openWEPP names are explicit aliases only (Section 3 table).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `MAN-GAP-001` | Full non-zero section parsing and executable fixture coverage for canonical cropland `.man` structures is implemented; remaining branch gap is only `mgtopt 4..7` execution coverage. | `[DIRECT][E-SPEC-MAN-01]`, `[DIRECT][E-WP-MAN-01]` | `amended` |
| `MAN-GAP-002` | Formal openWEPP policy for accepting/translating 2016.3+ extended operation codes in strict mode vs external downgrade workflows is not fully ratified. | `[DIRECT][E-SPEC-MAN-01]`, `[DIRECT][E-WP-MAN-01]` | `HOLD` |
| `MAN-GAP-003` | Cross-file governance for management-schedule year closure against run-control surfaces is not yet codified in a dedicated run-surface parser contract. | `[INFERENCE][E-SURVEY-MAN-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-28` | `0.2.1` | HILLSTAB02 amendment: compatibility-mode authority now permits `tilseq=0` as explicit no-surface-effect sentinel even when `nseq>0`, while strict mode keeps full positive index-domain enforcement. |
| `2026-05-21` | `0.2.0` | Ratified executable INIMPL09 parser profile: canonical section-order non-zero parsing, typed registry/schedule output, explicit rangeland unsupported policy, and executable date-domain guard linkage. |
| `2026-05-21` | `0.1.2` | Replaced grouped payload rows in Section 3 with explicit per-symbol field rows for all externally relevant management inputs across plant/operation/initial/surface/contour/drain/yearly branches. |
| `2026-05-21` | `0.1.1` | Expanded management field-coverage matrix, added boundary export mapping, added explicit date-domain guards/taxonomy, and evidence-tagged cross-file constraints. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE03. |
