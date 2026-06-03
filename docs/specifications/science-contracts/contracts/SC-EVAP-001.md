---
contract_id: SC-EVAP-001
title: Evapotranspiration Stress Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 21
producer_scope:
  - Potential and actual evapotranspiration partition surfaces
  - Evaporation/transpiration stress and availability-limited ET surfaces
  - Root-zone ET extraction and atmospheric-demand coupling surfaces
consumer_scope:
  - Daily water-balance accounting consumers
  - Plant-growth and residue-state consumers influenced by ET stress signals
  - Comparator/replay surfaces using Tier-A daily closure confidence signals
evidence_level: Static
last_reviewed: 2026-06-03
supersedes: []
superseded_by: []
---

# SC-EVAP-001 Evapotranspiration Stress Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for evapotranspiration partition and
stress-domain behavior, including daily potential/actual ET semantics and
cross-domain coupling with water-balance, climate forcing, winter hydrology,
and plant-growth consumers.

## Scientific Scope

In scope:
- Daily potential ET estimation domain and input prerequisites. `[INFERENCE][Static]`
- Partition of potential ET into potential soil evaporation and potential plant
  transpiration. `[DIRECT][Static] + [INFERENCE][Static]`
- Bare-soil evaporation staging, residue attenuation, and root-zone ET
  distribution/stress semantics. `[DIRECT][Static] + [INFERENCE][Static]`
- Plant-growth water-stress coupling boundaries. `[DIRECT][Static] + [INFERENCE][Static]`

Out of scope:
- Kernel implementation details and Rust API naming. `[INFERENCE][Static]`
- Percolation/groundwater routing internals owned by `SC-PERC-001` and
  `SC-SUBHYD-001`. `[INFERENCE][Static]`
- Vegetation growth equations beyond ET stress-boundary obligations owned by
  `SC-PLANT-001`. `[INFERENCE][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-EVAP-CH5-BAL | `references/50201000/chap5.pdf` §5.1 Eq. [5.1.1] | Daily closure includes ET withdrawal term and signed water-balance context. | `[DIRECT][Static]` |
| REF-EVAP-CH5-POT | `references/50201000/chap5.pdf` §5.2 Eq. [5.2.1]-[5.2.7] | Potential ET formulation (Penman/Priestly-Taylor pathways) and climate-input prerequisites. | `[DIRECT][Static]` |
| REF-EVAP-CH5-PART | `references/50201000/chap5.pdf` §5.2 Eq. [5.2.8]-[5.2.9] | Partition of potential ET into potential soil evaporation and potential plant transpiration. | `[DIRECT][Static]` |
| REF-EVAP-CH5-STAGE | `references/50201000/chap5.pdf` §5.2 Eq. [5.2.10]-[5.2.13] | Bare-soil evaporation stage-one/stage-two behavior and residue attenuation relation. | `[DIRECT][Static]` |
| REF-EVAP-CH5-LAI | `references/50201000/chap5.pdf` §5.2 Eq. [5.2.14] | LAI-based adjustment of potential transpiration up to `L <= 3`. | `[DIRECT][Static]` |
| REF-EVAP-CH5-DIST | `references/50201000/chap5.pdf` §5.3 Eq. [5.3.1]-[5.3.4] | Root-zone depth distribution for soil evaporation and layer-wise transpiration extraction under deficit conditions. | `[DIRECT][Static]` |
| REF-EVAP-CH5-LINK | `references/50201000/chap5.pdf` §5.5 Eq. [5.5.1] | Water-stress factor definition `Ws = (Σ Ui)/Etp` and coupling to plant-growth component. | `[DIRECT][Static]` |
| REF-EVAP-CH8-LINK | `references/50201000/chap8.pdf` §8.2.4 Eq. [8.2.14]-[8.2.15] | Plant growth regulation consumes ET-derived water-stress factor. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-EVAP-CH2-FORCING | `references/50201000/chap2.pdf` §2.1.6-§2.1.8 Eq. [2.1.12]-[2.1.14] | Climate generator provides daily solar radiation, dew point, and wind inputs used by ET potential pathways. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-EVAP-CH5-SNOW | `references/50201000/chap5.pdf` §5.1 and §5.3 text | Soil evaporation can be satisfied from snowpack first, then soil water. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-EVAP-LEGACY-STAGE | `/workdir/wepp-forest_260430_baseline/src/evap.for:458-564` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline stage-memory authority for `s1`, `s2`, `tu`, `tv` branch transitions and deficit-coupled `Es` evolution. | `[DIRECT][Static]` |
| REF-EVAP-LEGACY-ETP | `/workdir/wepp-forest_260430_baseline/src/evap.for:583-586` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline LAI full-cover cap authority for potential transpiration partition (`Ep`/`Etp`). | `[DIRECT][Static]` |
| REF-EVAP-LEGACY-PMET | `/workdir/wepp-forest_260430_baseline/src/evappm.for:181-297` and `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:557-559` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline Penman-Monteith ET-demand branch authority (`evappm`) selected by `iflget != 1`, including `kcb`/`rawp` crop coefficients and final-hour hourly call-order. | `[DIRECT][Static]` |
| REF-EVAP-LEGACY-PMET-DEMAND | `/workdir/wepp-forest_260430_baseline/src/evappm.for:181-388` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline-authoritative PMET demand subset: FAO-56 reference ET (`etorc`), basal crop coefficient adjustment (`kcbadj`/`kcbcon`), evaporation-reduction coefficient (`etkr`), water-stress coefficient (`etks`), potential soil evaporation (`potes`), residue-intercepted soil evaporation (`es`), and plant transpiration (`ep`) before post-ET soil redistribution. | `[DIRECT][Static]` |
| REF-EVAP-LEGACY-PMET-SEAM | `/workdir/wepp-forest_260430_baseline/src/evappm.for:430-523`, `/workdir/wepp-forest_260430_baseline/src/swu.for:122-191`, and `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:978-981` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline PMET seam authority: `evappm` computes non-negative soil/residue evaporation `es`, transpiration demand `ep`, and `et = es + ep`; when `es - resint < 0`, `evappm` returns `-xx` to top-layer storage rather than publishing a material negative `es`; later `swu` consumes positive `ep` for root-zone uptake and publishes final `ep = Σu(k)`. PMET-mode WB17 must not re-run Priestley-Taylor/LAI partition on `pmet.ep_m`. | `[DIRECT][Static]` |
| REF-EVAP-LEGACY-SUNMAP-RADPOT | `/workdir/wepp-forest_260430_baseline/src/sunmap.for:181-234` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline horizontal-surface potential radiation (`radpot`) used by `evappm` to derive `rso` when runtime climate projection does not provide `radpot` directly. | `[DIRECT][Static]` |
| REF-EVAP-INFILE-PMET | `SC-INFILE-PMETPARA-001` | Canonical sidecar discoverability, `iflget`, crop-key lookup, `kcb`, `rawp`, and fallback-observability authority. | `[DIRECT][Static]` |
| REF-EVAP-LEGACY-SOILX | `/workdir/wepp-forest_260430_baseline/src/evap.for:609-668` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline layerwise soil-water extraction authority for soil evaporation from `st(i)` with depth-aware allocation. | `[DIRECT][Static]` |
| REF-EVAP-LEGACY-SWU | `/workdir/wepp-forest_260430_baseline/src/swu.for:122-191` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline root-zone transpiration uptake authority (`UPi`, `Ui`) and water-stress ratio lineage (`watstr = ΣUi/ep`). | `[DIRECT][Static]` |
| REF-EVAP-LEGACY-HOURLY-ORDER | `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:471-560` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Baseline hourly water-balance ordering: hourly infiltration/percolation mutate `st(i)` before ET is invoked only on the final hourly pass. | `[DIRECT][Static]` |
| REF-EVAP-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative rate/depth domains and bounded stress factors. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `Eu` | `m d^-1` | Daily potential evapotranspiration after energy-to-depth conversion from Eq. [5.2.1]/[5.2.2] pathway. | ET potential pathway | ET partition pathway |
| `Esp` | `m d^-1` | Potential soil evaporation (Eq. [5.2.8]). | ET partition pathway | soil-evaporation stage logic |
| `Etp` | `m d^-1` | Potential plant transpiration before/after LAI adjustment (Eq. [5.2.9], [5.2.14]). | ET partition pathway | layer uptake and stress-factor pathway |
| `Esu` | `m d^-1` | Stage-one soil-evaporation upper limit (Eq. [5.2.10]). | soil-evaporation stage logic | stage-transition logic |
| `Esb`, `Es` | `m d^-1` | Bare-soil and residue-adjusted actual soil evaporation (Eq. [5.2.12], [5.2.13]). | soil-evaporation stage logic | root-zone ET withdrawal |
| `Tr`, `d2` | `mm d^-0.5`, `d` | Soil transmissivity and stage-two day counter driving Eq. [5.2.11]-[5.2.12]. | soil parameterization + stage state | stage-two evaporation computation |
| `s1`, `s2`, `tu`, `tv` | `m`, `m`, `m`, `d` | Baseline stage-memory state surfaces controlling stage-one/stage-two soil-evaporation transition dynamics. | ET stage-memory pathway | stage-transition and `Es` branch logic |
| `Cr` | `kg ha^-1` | Plant residue mass for evaporation attenuation (Eq. [5.2.13]). | residue/crop state | soil-evaporation attenuation |
| `L` | `m^2 m^-2` | Leaf area index for ET partition and transpiration adjustment. | crop-growth state | ET partition and LAI adjustment |
| `dx`, `ds` | `m` | Maximum and realized soil-evaporation depth (Eq. [5.3.1]-[5.3.2]). | ET root-zone pathway | soil-layer water updates |
| `UPi`, `Ui` | `m d^-1` | Potential and actual layer `i` plant water-use rates (Eq. [5.3.3]-[5.3.4]). | ET root-zone pathway | stress-factor and layer-water updates |
| `Θ`, `Θi`, `Θr`, `Θc`, `ULi` | `m`, `m`, `m^3 m^-3`, `fraction`, `m` | Root-zone/layer water states and thresholds used for ET extraction constraints. | soil/water state pathway | ET distribution logic |
| `Rd`, `rtd`, `pltol` | `m`, `m`, `fraction` | Active root depth and plant-tolerance domain required by baseline `swu` uptake distribution and deficit scaling. | plant-growth/runtime management pathway | post-WB19 root-uptake pathway |
| `Ws` | `fraction` | Plant-growth water-stress factor (`0..1`) from supply-demand ratio (Eq. [5.5.1], Eq. [8.2.15]). | ET coupling pathway | plant-growth regulation |
| `ET` | `m` | Daily cumulative evapotranspiration withdrawal term in water-balance closure Eq. [5.1.1]. | ET integration pathway | daily water-balance closure consumer |
| `RA`, `radpot`, `Tmax`, `Tmin`, `Tdp`, `u_z` | `Ly`, `Ly`, `degC`, `degC`, `degC`, `m s^-1` | Climate forcing surfaces required by potential ET formulations, including baseline `sunmap` potential radiation when `evappm` derives `rso`. | climate forcing pathway | ET potential pathway |
| `iflget`, `kcb`, `rawp` | mode, coefficient, fraction | Legacy ET-method selector and Penman-Monteith crop coefficients from `pmetpara` sidecar lookup. | PMET sidecar/runtime projection | ET demand seed pathway |
| `etorc`, `rn`, `rso`, `rbo`, `fwv`, `rhd` | `mm d^-1`, `MJ m^-2 d^-1`, `MJ m^-2 d^-1`, `MJ m^-2 d^-1`, `m s^-1`, `%` | Baseline `evappm` reference-ET and meteorological intermediate surfaces. | PMET demand seed pathway | WB11/WB17 demand lineage diagnostics |
| `kcbadj`, `kcbcon`, `etke`, `etkr`, `etks` | coefficient, coefficient, coefficient, coefficient, coefficient | Baseline `evappm` dual-coefficient and stress intermediate surfaces. | PMET demand seed pathway | WB11/WB17 demand lineage diagnostics |
| `TEW`, `REW`, `wfevp`, `TAW`, `RAW`, `wftrp` | `mm`, `mm`, `mm`, `mm`, `mm`, `mm` | Baseline `evappm` soil-evaporation and root-zone availability/stress water-depth intermediates. | PMET demand seed pathway | WB11/WB17 demand lineage diagnostics |
| `S` | `m` | Snow-water state that can satisfy evaporation demand before soil-water extraction. | winter hydrology pathway | ET withdrawal precedence logic |
| `F` | `m` | Same-pass infiltration lineage that drives ET stage-memory reset and soil-water availability under hourly WB14/WB12 cadence. | infiltration/runoff partition pathway | ET stage-memory and extraction pathway |

## Algorithm State Surfaces (WB17 ET Production Kernel)

### Required Inputs

| Surface | Symbols |
|---|---|
| Scheduler phase metadata | `phase_name`, `phase_class`, `consumer_adapter` |
| ET consumer-boundary state family | `nsl`, `solthk`, `thetdr`, `thetfc`, `ssc` |
| WB17 ET state inputs | `wb11_soil_water`, `wb11_et_demand`, `lai`, `cancov`, `wb17_residue_interception` |
| WB11 ET-demand branch lineage | `pmetpara.mode.sidecar_present`, `pmetpara.mode.iflget`, selected `kcb`, selected `rawp`, selected PMET crop row, actual ET seed branch, `etorc`, `kcbadj`, `kcbcon`, `etke`, `etkr`, `etks`, `TEW`, `REW`, `wfevp`, `TAW`, `RAW`, `wftrp` |
| Same-pass infiltration cadence lineage | `wb12_infiltration` or WB14-derived same-pass infiltration lineage before ET execution |
| Baseline-authoritative ET stage-memory/state family | `s1`, `s2`, `tu`, `tv`, `wb18_perc_theta_####`, `dg_####`, `thetdr_####`, `frozen_depth_####` |
| Post-WB19 root-uptake state/flux family | `Etp`, `ET`, `wb18_perc_theta_####`, `wb18_perc_ul_####`, `dg_####`, `thetdr_####`, `frozen_depth_####`, `rtd`, `pltol` |

### Required Outputs

| Surface | Output |
|---|---|
| ET flux outputs | `ET`, `Ws`, `Ep`, `Es`, `Er`, `Etp`, aggregate `UPi`, aggregate `Ui`, layer `UPi_####`, layer `Ui_####`; `Ep`/`Ws` become final after post-WB19 root uptake |
| ET state updates | `wb18_perc_theta_####`, `wb11_soil_water`, `wb17_residue_interception` |
| Scheduler/kernel failure surface | Typed hard-fail status for missing/non-finite/out-of-range ET state domains |

### Mutated State Surfaces

WB17 mutates ET boundary surfaces deterministically:
- bare-soil partition: `Esp = wb11_et_demand * exp(-0.5 * (cancov + 0.1))`
  using the baseline `eaj` uncovered-soil branch.
- residue-adjusted soil demand: `Esp_soil = max(Esp - wb17_residue_interception, 0)`.
- stage-memory reduction: optional `s1`, `s2`, `tu`, `tv` state transitions
  reduce `Esp_soil` before layer extraction when the full stage family is
  present.
- LAI-adjusted transpiration demand: `Etp = wb11_et_demand` for `lai > 3`,
  otherwise `Etp = lai * wb11_et_demand / 3`.
- ET demand cap and residue split follow baseline `evap.for:566-604`, so
  residue evaporation is tracked as `Er` and soil extraction demand excludes
  residue interception.
- soil extraction mutates `wb18_perc_theta_####` by depth-aware extraction
  from the upper `0.10 m` baseline evaporation zone.
- plant extraction mutates `wb18_perc_theta_####` through baseline `swu`
  root-depth weighting and deficit scaling when `Etp > 0` and `rtd > 0`;
  this extraction is scheduled after WB19 drainage/lateral mutation per
  `SC-WATBAL-001#INV-WATBAL-028`.
- aggregate update recomputes `wb11_soil_water = Σ(wb18_perc_theta_i +
  thetdr_i * (dg_i - frozen_depth_i))` after soil evaporation and again after
  post-WB19 root uptake.
- stress update: `Ws = 1` for `Etp <= 1e-12`, otherwise `Ws = Ep / Etp`.

## Algorithm Specification (WB17 ET Production Execution)

1. Require finite ET inputs (`wb11_soil_water`, `wb11_et_demand`, `lai`,
   `cancov`, `wb17_residue_interception`) and enforce non-negative domains.
2. Compute deterministic uncovered-soil evaporation demand from baseline
   `eaj = exp(-0.5 * (cancov + 0.1))`, then compute LAI-adjusted plant
   transpiration demand from `lai` and `wb11_et_demand`.
3. Compute explicit residue evaporation partition (`Er`) and remaining
   soil-evaporation demand (`Es`) before soil-water extraction.
4. Compute explicit soil evaporation, plant transpiration, total ET, and stress
   ratio (`Ws`) through layer-first `st(i)` mutation semantics mapped to
   `wb18_perc_theta_####`.
5. In hourly-lane execution, consume same-pass infiltration lineage from WB14
   and layer state mutated by prior hourly infiltration/percolation; stale
   `wb12_infiltration` compatibility state cannot drive stage-memory or ET
   acceptance when authoritative same-pass lineage exists.
6. Reject missing, non-finite, or out-of-range ET inputs/outputs with typed
   hard-fail status; no silent fallback/clamping/defaulting paths are permitted.

## Branch and Guard Table (WB17 ET Kernel)

| Branch ID | Trigger | Required symbols | Guard class | Failure posture |
|---|---|---|---|---|
| `BR-EVAP-WB17-EXECUTE` | phase class `hydrology_evapotranspiration` | `wb11_soil_water`, `wb11_et_demand`, `lai`, `wb17_residue_interception` | runtime | deterministic ET partition + writeback execution |
| `BR-EVAP-WB11-PMET-SEED` | `pmetpara.mode.iflget != 1` | PMET sidecar selector, crop lookup, `kcb`, `rawp`, climate forcing used by `evappm` | runtime + governance | must execute baseline-authoritative `evappm` demand seeding or remain explicit `HOLD`; no PT fallback or coefficient-tuned proxy |
| `BR-EVAP-WB11-PMET-MIGRATED-SEED` | `pmetpara.mode.iflget != 1` after HPHYS0263 migration | `rad`, `radpot` or `deglat` + calendar date for baseline `sunmap` derivation, `tmax`, `tmin`, `tdpt`, `vwind`, `elevm`, `canhgt`, `lai`, `rtd`, `kcb`, `rawp`, `st(i)`, `thetfc(i)`, `thetdr(i)`, `solthk(i)`, `dg(i)`, `fin`, `resint` | runtime | compute `wb11_et_demand` from pinned `evappm.for:181-388` subset and label actual branch `evappm_pmet` |
| `BR-EVAP-WB17-PMET-COMPONENT-SEAM` | `wb11_et_seed_branch_evappm = 1` | `pmet.es_m`, `pmet.ep_m`, `wb17_residue_interception`, `wb18_perc_theta_####`, `dg_####` | runtime | consume PMET `es`/`ep` components as the WB17 ET-phase boundary; publish `Etp = pmet.ep_m` for later `swu`, derive `Es`/`Er` from non-negative `pmet.es_m`, reject material negative `pmet.es_m` while snapping only within-tolerance negative roundoff to zero, and skip Priestley-Taylor/LAI re-partition |
| `BR-EVAP-WB11-MISSING` | required ET symbol absent | ET required symbols | runtime | typed hard-fail (`HKERNEL-WB11-ET-E-001`) |
| `BR-EVAP-WB11-NONFINITE` | ET symbol is NaN/Inf | ET required symbols | runtime | typed hard-fail (`HKERNEL-WB11-ET-E-002`) |
| `BR-EVAP-WB11-DOMAIN` | ET symbol/derived flux outside domain bounds | ET required + emitted symbols | runtime | typed hard-fail (`HKERNEL-WB11-ET-E-003`) |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-EVAP-001 | Potential ET partition invariant: partition equations (Eq. [5.2.8]-[5.2.9]) must be applied so that emitted potential components satisfy `Eu = Esp + Etp` within declared tolerance in ET-depth units. | hard-fail | REF-EVAP-CH5-PART, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-002 | Potential ET input-domain invariant: selected potential ET pathway (Eq. [5.2.1] or [5.2.2]) requires finite climate inputs and explicit pathway declaration; negative or undefined `Eu` is invalid. | hard-fail | REF-EVAP-CH5-POT, REF-EVAP-CH2-FORCING, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-003 | Bare-soil stage invariant: stage-one/stage-two evaporation transitions and transmissivity dependence must follow Eq. [5.2.10]-[5.2.12], including explicit restart to stage one when precipitation satisfies accumulated stage-two evaporation condition. | hard-fail | REF-EVAP-CH5-STAGE | `[DIRECT][Static]` |
| INV-EVAP-004 | Residue attenuation invariant: actual soil evaporation must follow Eq. [5.2.13], remain non-negative, and not exceed bare-soil evaporation for non-negative residue mass. | hard-fail | REF-EVAP-CH5-STAGE, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-005 | Soil-evaporation depth invariant: Eq. [5.3.1]-[5.3.2] constraints hold with `0 <= ds <= dx`; if snowpack water satisfies `Es`, soil-water extraction branch remains zero. | hard-fail | REF-EVAP-CH5-DIST, REF-EVAP-CH5-SNOW, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-006 | Root-zone uptake distribution invariant: layer-wise potential uptake distribution (Eq. [5.3.3]) must preserve declared root-depth weighting and emit non-negative `UPi` values with finite layer sums. | hard-fail | REF-EVAP-CH5-DIST, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-007 | Water-deficit adjustment invariant: actual layer uptake branch (Eq. [5.3.4]) must enforce threshold behavior at `Θi` vs `Θc ULi`, with `0 <= Ui <= UPi` and explicit scaling in deficit branch. | hard-fail | REF-EVAP-CH5-DIST, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-008 | Stress-factor invariant: stress factor `Ws` must follow Eq. [5.5.1]/[8.2.15], remain within `[0,1]`, and be emitted with declared units/semantics for plant-growth consumers. | hard-fail | REF-EVAP-CH5-LINK, REF-EVAP-CH8-LINK, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-009 | Coupling completeness invariant: ET boundary payload must include required surfaces for water-balance closure (`ET` term context) and plant-growth stress coupling (`Ws`, demand/supply terms) with unit-consistent semantics. | hard-fail | REF-EVAP-CH5-BAL, REF-EVAP-CH5-LINK, REF-EVAP-CH8-LINK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-010 | Governance limitation invariant: ET contract interpretation must remain explicit about daily-step process scope and cited method assumptions (modified Ritchie framework and pathway preconditions); missing scope labeling blocks promotion. | governance-fail | REF-EVAP-CH5-POT, REF-EVAP-CH5-STAGE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-011 | WB17 ET execution invariant: ET phase computes deterministic partitioned ET components (`Er`, `Es`, `Ep`) and derived closure outputs (`ET`, `Ws`) from required WB17 ET symbols and updates `wb11_soil_water` without implicit fallback branches. | hard-fail | REF-EVAP-CH5-PART, REF-EVAP-CH5-LINK, REF-EVAP-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-012 | WB17 ET guard invariant: missing/non-finite/out-of-range WB17 ET domains must surface typed hard failures (`HKERNEL-WB11-ET-E-001..003`) and cannot be silently clamped/defaulted. | hard-fail | REF-EVAP-PHYS-BOUNDS | `[INFERENCE][Static]` |
| INV-EVAP-013 | SIMIMPL21 baseline-authority invariant: ET contract authority must preserve baseline stage-memory transitions (`s1`, `s2`, `tu`, `tv`), depth-aware soil evaporation extraction from `st(i)`, and root-zone uptake semantics (`UPi`, `Ui`, `Ws = ΣUi/Etp`) with explicit branch lineage to legacy `evap` + `swu` routines. | hard-fail | REF-EVAP-LEGACY-STAGE, REF-EVAP-LEGACY-SOILX, REF-EVAP-LEGACY-SWU, REF-EVAP-CH5-DIST, REF-EVAP-CH5-LINK | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-014 | HPHYS0242 hourly ET/infiltration ordering invariant: hourly-lane ET must execute only after the same-day WB14 infiltration and WB18 percolation lineage has mutated layer state, and stage-memory/soil-extraction logic must consume same-pass infiltration lineage rather than stale `wb12_infiltration` compatibility state. Missing or conflicting same-pass infiltration lineage is a typed hard failure, not a zero/default substitution. | hard-fail | REF-EVAP-LEGACY-HOURLY-ORDER, REF-EVAP-LEGACY-STAGE, REF-EVAP-LEGACY-SOILX, REF-EVAP-CH5-DIST, SC-WATBAL-001#INV-WATBAL-034 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-015 | HPHYS0249 WB17 layer-storage invariant: promoted WB17 `Ep`/`Es` evidence must mutate runtime layer storage (`wb18_perc_theta_####` as the openWEPP alias for baseline `st(i)`) for upper-zone soil evaporation (`evap.for:618-668`) before WB19 and for root uptake (`swu.for:122-187`) after WB19 drainage/lateral mutation before recomputing final `wb11_soil_water`. Scalar-only subtraction from `wb11_soil_water`, LAI-only `exp(-0.4*lai)` soil partitioning, root uptake ahead of WB19, or root uptake that bypasses `rtd`/`wb18_perc_ul_####` is non-authoritative. | hard-fail | REF-EVAP-LEGACY-SOILX, REF-EVAP-LEGACY-SWU, REF-EVAP-LEGACY-HOURLY-ORDER, SC-WATBAL-001#INV-WATBAL-037 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-016 | HPHYS0250 final `Ep` lineage invariant: promoted WB17 `Ep` evidence must preserve plant-management/growth runtime activation through scheduler execution so `rtd` can be produced before water-balance phases, post-WB19 `PlantRootUptake` consumes active `rtd`/`pltol`/`Etp` lineage, and WB13 publication consumes the final root-uptake flux `Ep = ΣUi`, not the pre-`swu` ET-phase seed or stale state-surface aliases. Stripping PL runtime sentinel surfaces, suppressing growth phases, leaving `rtd=0` by scheduler construction when management data are present, or allowing same-name state `Ep` to shadow final flux `Ep` is invalid closure evidence. | hard-fail | REF-EVAP-LEGACY-ETP, REF-EVAP-LEGACY-SWU, REF-EVAP-CH5-DIST, REF-EVAP-CH5-LINK, SC-WATBAL-001#INV-WATBAL-038 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-017 | HPHYS0251 `swu.for` uptake-magnitude invariant: promoted WB17 root-uptake evidence must preserve baseline `swu.for:122-191` semantics by deriving `pltol(itype)` from management plant data when present, applying the explicit legacy normalization branch (`pltol <= 0 -> 0.25`, `pltol < 0.1 -> 0.1`, `pltol > 0.4 -> 0.4`), computing layer potential uptake with baseline cumulative weighting (`ub=3.065`, `uob=0.953346`), publishing layer `UPi_####` and `Ui_####`, capping actual `Ui` by post-WB19 `wb18_perc_theta_####`, and setting final `Ep=ΣUi`, `Ws=ΣUi/Etp` for positive demand. A fixed default that masks crop data, omitted layer uptake surfaces, or unlabeled silent domain clamping is invalid closure evidence. | hard-fail | REF-EVAP-LEGACY-SWU, REF-EVAP-CH5-DIST, REF-EVAP-CH5-LINK, SC-WATBAL-001#INV-WATBAL-039 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-018 | HPHYS0260 WB17 trace-localization invariant: residual ownership claims for H1/H7/H39 `Ep` must consume trace-grade post-`PlantRootUptake` evidence for aggregate `UPi`, aggregate `Ui`, layer `UPi_####`, layer `Ui_####`, final `Ep`, `Etp`, `Ws`, and post-uptake `wb18_perc_theta_####`. When `Ep = ΣUi_####`, `0 <= Ui_#### <= UPi_####`, and `Ws = Ep/Etp` for positive `Etp` close internally, continuation must not assign the stable `Ep` residual to trace publication or WB13 shadowing without new baseline-authoritative divergence evidence. | hard-fail | REF-EVAP-LEGACY-SWU, REF-EVAP-CH5-DIST, REF-EVAP-CH5-LINK, INV-EVAP-016, INV-EVAP-017, SC-WATBAL-001#INV-WATBAL-046 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-019 | HPHYS0261 WB17 `Ep` magnitude/initialization invariant: residual ownership claims for H1/H7/H39 `Ep` must expose trace-grade `evap`/`swu` seam inputs before changing equations: `Etp`, pre- and post-growth plant state available at trace boundaries (`lai`, `cancov`, `rtd`), raw `pltol`, effective legacy-normalized `pltol`, WB18 `ul(i)`, `pltol*ul(i)` stress thresholds, storage-to-threshold ratios, layer `UPi_####`/`Ui_####`, final `Ep`, and `Ws`. Evidence must preserve baseline call-order provenance that `evap.for` seeds `ep` from current `lai` before `watbal(_hourly).for` calls `ptgrp`/`ptgra`, while `swu.for` consumes the already-seeded `ep` after growth/root update. Heuristic `Ep` scaling or hidden LAI/root-depth substitution is invalid. | hard-fail | REF-EVAP-LEGACY-ETP, REF-EVAP-LEGACY-SWU, REF-EVAP-LEGACY-HOURLY-ORDER, INV-EVAP-018, SC-WATBAL-001#INV-WATBAL-047 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-020 | HPHYS0262 PMET demand-seeding invariant: H1/H7/H39 `Ep` magnitude closure claims must expose whether a `pmetpara` sidecar selected legacy `evappm` (`iflget != 1`) or `evap` (`iflget = 1`), the selected crop row and coefficients (`kcb`, `rawp`), fallback status, and the actual ET-demand seed branch that produced `wb11_et_demand`. When `pmetpara` selects `evappm`, closure requires a baseline-authoritative `evappm.for` port for `wb11_et_demand`; substituting Priestley-Taylor `evap`, hidden crop-coefficient tuning, or any proxy Penman-Monteith formula is invalid and must remain `HOLD`. | hard-fail | REF-EVAP-LEGACY-PMET, REF-EVAP-INFILE-PMET, INV-EVAP-019, SC-WATBAL-001#INV-WATBAL-048 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-021 | HPHYS0263 EVAPPM migration invariant: when `pmetpara.mode.iflget != 1`, WB11 ET-demand seeding must compute the PMET demand subset from pinned `evappm.for:181-388`: reference ET `etorc`, adjusted basal coefficient `kcbadj`, basal canopy coefficient `kcbcon`, soil evaporation coefficient `etke`, evaporation reduction coefficient `etkr`, water-stress coefficient `etks`, residue-intercepted soil evaporation `es`, and transpiration `ep`. The emitted `wb11_et_demand` must be the migrated PMET plant-transpiration demand consumed by WB17/SWU, and the actual branch trace must be `evappm_pmet`. `radpot` must be either runtime-provided or derived from pinned `sunmap.for:181-234`; Priestley-Taylor demand, coefficient-only tuning, and formulas not traced to the pinned lines are invalid for PMET-mode closure. | hard-fail | REF-EVAP-LEGACY-PMET-DEMAND, REF-EVAP-LEGACY-PMET, REF-EVAP-LEGACY-SUNMAP-RADPOT, SC-WATBAL-001#INV-WATBAL-049 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-022 | HPHYS0264 PMET seam invariant: when `wb11_et_seed_branch_evappm = 1`, WB17 evapotranspiration execution must consume migrated PMET components from `INV-EVAP-021` directly: `pmet.ep_m` is the positive `Etp` demand passed to post-WB19 `swu` root uptake, and `pmet.es_m` is the PMET soil/residue evaporation component whose extraction lineage produces `Es` plus `Er`. PMET mode must not apply the Chapter-5/Priestley-Taylor partition equations `Esp = Eu * exp(-0.5*(cancov+0.1))` or `Etp = lai * Eu / 3` to `pmet.ep_m`; material negative `pmet.es_m` is domain-invalid, and only within-tolerance negative roundoff may canonicalize to zero before WB17/WB13 publication. Final `Ep` remains authoritative only after `swu.for:122-191` lineage consumes `Etp`; full pinned `evappm.for:460-523` post-ET redistribution remains separate migration scope. | hard-fail | REF-EVAP-LEGACY-PMET-SEAM, REF-EVAP-LEGACY-SWU, INV-EVAP-021, SC-WATBAL-001#INV-WATBAL-050 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-023 | HPHYS0265 first-large longer-season `Ep` divergence localization invariant: after HPHYS0264 PMET seam closure, any residual ownership claim for seasonal `Ep` must be backed by multi-day H1/H7/H39 trace evidence at the first candidate/baseline WAT row where `|candidate Ep - baseline Ep| > 0.05 mm`. Evidence must include seed branch, `pmet.ep_m`, WB17 `Etp`, final `Ep`, `ΣUi`, `Ws`, `pltol`, effective `pltol`, `rtd`, `lai`, layer `Ui`, layer storage-to-threshold ratios, and same-day storage/snow/runoff/percolation/lateral terms. If WB17 identities close but candidate `Ep` still differs from baseline, closure may not assign the residual to WB17 publication or the PMET seam; it must classify the residual as upstream demand/growth/storage/snow-runoff coupling until baseline-authoritative evidence identifies a narrower defect. | governance-hold | REF-EVAP-LEGACY-PMET-SEAM, REF-EVAP-LEGACY-SWU, INV-EVAP-018, INV-EVAP-021, INV-EVAP-022, SC-WATBAL-001#INV-WATBAL-051 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-EVAP-024 | HPHYS0267 post-lateral/pre-SWU stress-threshold invariant: seasonal `Ep` residual ownership claims after HPHYS0266 must expose WB17 stress inputs on the same post-lateral state consumed by `swu`: `Etp`, final `Ep`, `ΣUi`, `Ws`, effective `pltol`, `ul_i`, `pltol*ul_i`, post-lateral `theta_i`, storage-to-threshold ratios, and any overlapping WB19 lateral withdrawal layers. If WB17 identities close on the post-lateral state, production WB17 changes require baseline-authoritative evidence that the consumed storage or threshold inputs differ from pinned `swu.for` lineage; heuristic stress rescaling is invalid. | governance-hold | INV-EVAP-023, INV-EVAP-019, REF-EVAP-LEGACY-SWU, SC-SUBHYD-001#INV-SUBHYD-031, SC-WATBAL-001#INV-WATBAL-053 | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-EVAP-001` | runtime | ET partition assembler | Typed hard error on partition residual above tolerance | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-002` | runtime | Potential ET pathway selector/validator | Typed hard error on missing forcing inputs, undefined pathway, or negative/non-finite `Eu` | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-003` | runtime | Soil-evaporation stage state machine | Typed hard error on invalid stage transition or transmissivity branch misuse | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-EVAP-004` | runtime | Residue attenuation calculator | Typed hard error when `Es` violates Eq. [5.2.13] or non-negative/bounded domain | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-005` | runtime | Root-zone soil-evaporation depth updater | Typed hard error on invalid `ds`/`dx` bounds or snow-first precedence violation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-006` | runtime | Layer-wise potential uptake distributor | Typed hard error on negative/non-finite `UPi` or invalid root-depth weighting output | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-007` | runtime | Layer-wise actual uptake branch evaluator | Typed hard error on threshold-branch mismatch or invalid `Ui` bounds | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-008` | runtime | Stress-factor calculator and boundary validator | Typed hard error on out-of-range/undefined `Ws` | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-009` | runtime | Cross-domain ET boundary payload validator | Typed hard error on missing required ET/stress surfaces or units mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-010` | governance | Contract review + promotion checklist | Promotion `HOLD` if method/scope caveats are not explicit in contract/disposition artifacts | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-011` | runtime | WB17 ET production kernel execution path | Typed hard error on non-deterministic/malformed partition/writeback ET outputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-012` | runtime | WB17 ET guard table (`HKERNEL-WB11-ET-E-001..003`) | Typed hard error on missing/non-finite/domain-invalid ET inputs/outputs | Tier-A gate | `[INFERENCE][Static]` |
| `INV-EVAP-013` | runtime + governance | Stage-memory/root-uptake lineage validator for legacy `evap` + `swu` authority closure | Typed hard error / explicit `HOLD` when stage-memory, depth extraction, or `UPi`/`Ui` lineage semantics are missing or contradicted | SIMIMPL ET migration gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-014` | runtime + governance | WB17 scheduler-order validator plus same-pass infiltration-lineage resolver | Typed hard error / explicit `HOLD` when hourly ET runs before WB14/WB18 lineage, consumes stale compatibility infiltration, or silently defaults missing infiltration lineage to zero | HPHYS cadence/order closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-015` | runtime + governance | WB17 layer-storage ET lineage validator across soil evaporation, `swu` uptake, and aggregate writeback | Typed hard error / explicit `HOLD` when WB17 emits `Ep`/`Es` from scalar-only storage, bypasses `wb18_perc_theta_####`, or fails to recompute aggregate storage after layer mutation | HPHYS0249 WB17/storage closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-016` | runtime + governance | Final `Ep` lineage validator spanning PL scheduler activation, post-WB19 `PlantRootUptake`, and WB13 flux-authoritative publication | Typed hard error / explicit `HOLD` when growth/runtime sentinel stripping suppresses `rtd`, when `PlantRootUptake` does not publish final `Ep`, or when WB13 consumes stale pre-`swu` aliases | HPHYS0250 `Ep` lineage closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-017` | runtime + governance | Baseline `swu.for` magnitude validator spanning effective crop `pltol`, layer `UPi`/`Ui`, and final `Ep`/`Ws` publication | Typed hard error / explicit `HOLD` when crop `pltol` is masked by an unconditional default, legacy normalization is not observable, layer uptake traces are absent, or final `Ep`/`Ws` are not derived from post-WB19 `Ui` | HPHYS0251 uptake-magnitude closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-018` | runtime + governance | WB17 trace-localization validator spanning post-`PlantRootUptake` aggregate/layer `UPi`/`Ui`, final `Ep`, `Etp`, `Ws`, and post-uptake layer storage | Typed hard error / explicit `HOLD` when trace evidence omits layer uptake maps, when `Ep`/`Ws` identities do not reconcile, or when residual ownership is assigned without this trace-grade evidence | HPHYS0260 WB17 residual-classification gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-019` | runtime + governance | WB17 `Ep` magnitude/initialization validator spanning `evap` partition inputs, `swu` stress thresholds, and legacy call-order provenance | Typed hard error / explicit `HOLD` when trace evidence omits `pltol`, effective `pltol`, `ul(i)`, or threshold ratios, or when correction substitutes heuristic `Ep` scaling for baseline-authoritative lineage | HPHYS0261 magnitude/initialization gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-020` | runtime + governance | WB11 ET-demand branch validator spanning `pmetpara` sidecar mode, crop lookup, `kcb`/`rawp`, fallback status, actual seed branch, and baseline `evappm` provenance | Typed hard error / explicit `HOLD` when PMET mode is hidden, ignored, or satisfied by PT/proxy demand instead of baseline-authoritative `evappm.for` migration | HPHYS0262 PMET demand-seeding gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-021` | runtime + governance | Migrated PMET demand validator spanning `etorc`, `kcbadj`, `kcbcon`, `etke`, `etkr`, `etks`, `TEW/REW`, `TAW/RAW`, final `es`/`ep`, branch label, and `wb11_et_demand` | Typed hard error / explicit `HOLD` when migrated branch intermediates are missing, non-finite, out of domain, or not traceable to pinned `evappm.for` equations | HPHYS0263 EVAPPM migration gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-022` | runtime + governance | PMET seam validator spanning `wb11_et_seed_branch_evappm`, `pmet.es_m`, `pmet.ep_m`, ET-phase `Es`/`Er`, pre-SWU `Etp`, post-SWU `Ep`, and branch proof | Typed hard error / explicit `HOLD` when PMET mode reuses PT partition, omits `pmet.es_m`, or claims final `Ep` before SWU root uptake | HPHYS0264 PMET seam correction gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-023` | governance | First-large longer-season `Ep` divergence classifier spanning candidate/baseline WAT rows and multi-day trace `pmet.ep_m`/`Etp`/`Ep`/`Ui`/`Ws`/plant/storage context | Explicit `HOLD` when seasonal `Ep` ownership is assigned without first-divergence evidence, or when WB17 identities close but ownership is still assigned to WB17 publication/PMET seam | HPHYS0265 first-divergence localization gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-EVAP-024` | governance | Post-lateral/pre-SWU stress-threshold classifier spanning `Etp`, `Ep`, `ΣUi`, `Ws`, effective `pltol`, `ul_i`, `pltol*ul_i`, post-lateral `theta_i`, and WB19 withdrawal overlap | Explicit `HOLD` when WB17 identities close and no baseline-authoritative stress-threshold defect is proven | HPHYS0267 stress-threshold lineage gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow Chapter-5/Chapter-8 WEPP notation.
WB17 implementation now uses explicit runtime aliases for executed ET
equation vectors.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `Eu` | `wb11_et_demand` | ET demand surface consumed by WB17 partition runtime | `m d^-1` -> `m d^-1` | `[DIRECT][Static]` |
| `L` | `lai` | LAI-driven partition surface | `m^2 m^-2` -> `m^2 m^-2` | `[DIRECT][Static]` |
| `cv` | `cancov` | baseline canopy-cover surface consumed by `eaj = exp(-0.5*(cv+0.1))` | fraction preserved | `[DIRECT][Static]` |
| `Er` | `wb17_residue_interception` (input) + `Er` (flux output) | residue evaporation partition surface | `m d^-1` -> `m` daily flux output | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Esp`, `Etp`, `Es`, `Ep` | `Esp`, `Etp` (derived runtime), `Es`, `Ep` (flux outputs) | ET partition and component output surfaces | `m d^-1` potential -> `m` daily component flux outputs | `[DIRECT][Static] + [INFERENCE][Static]` |
| `s1`, `s2`, `tu`, `tv` | identity names (canonical stage-memory surface family) | stage-transition memory and threshold state | `m` / `d` preserved | `[DIRECT][Static]` |
| `dx`, `ds`, `UPi`, `Ui` | identity names | root-zone distribution and uptake surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `st(i)`, `Θi` | `wb18_perc_theta_####` | baseline layer storage mutated by evaporation, transpiration, percolation, lateral flow, and aggregate recomputation | `m` preserved | `[DIRECT][Static]` |
| `dg(i)` | `dg_####` | layer thickness/depth increment used for depth partitioning and aggregate recomputation | `m` preserved | `[DIRECT][Static]` |
| `thetdr(i)`, `ULi` | `thetdr_####`, `wb18_perc_ul_####` | residual and upper-limit layer surfaces used by ET and aggregate recomputation | `m^3 m^-3` / `m` preserved | `[DIRECT][Static]` |
| `Ws` | identity name | ET-to-plant stress boundary surface | `fraction` preserved | `[DIRECT][Static]` |
| `ET` | identity name | ET-to-water-balance closure boundary surface | `m` preserved | `[DIRECT][Static]` |
| `F` | `wb12_infiltration` or WB14 same-pass infiltration lineage | ET stage-memory reset and soil-water availability driver under hourly cadence | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `RA`, `Tmax`, `Tmin`, `Tdp`, `u_z` | identity names | climate-to-ET forcing surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `S` | identity name | snow-to-ET precedence surface | `m` preserved | `[DIRECT][Static]` |
| HPHYS0261 trace WB17 magnitude/initialization diagnostics | `pl_pltol`, `pl_swu_effective_pltol`, `wb18_ul_layers_m`, `wb17_swu_stress_threshold_layers_m`, `wb17_swu_storage_to_threshold_layers`, `pl_lai`, `pl_rtd`, `etp_m`, `ep_m`, `ui_m`, `wb17_ui_layers_m`, `ws` | Opt-in trace observability for classifying H1/H7/H39 `Ep` magnitude residuals at the `evap`/`swu` seam before equation or compensation changes | `m`, `mm`, and dimensionless plant/stress units preserved as named | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0262 trace PMET demand-seeding diagnostics | `pmet_sidecar_present`, `pmet_iflget`, `pmet_selected_kcb`, `pmet_selected_rawp`, `pmet_selected_line_index`, `pmet_lookup_fallback_first_row_used`, `wb11_et_demand_m`, `wb11_et_seed_branch` | Opt-in trace observability for classifying whether `pmetpara`/`evappm` lineage owns H1/H7/H39 `Ep` demand magnitude residuals | mode flags dimensionless; coefficients dimensionless; demand `m d^-1` | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0263 trace EVAPPM migration diagnostics | `pmet_etorc_mm`, `pmet_rn_mj_m2`, `pmet_fwv_m_s`, `pmet_rhd_pct`, `pmet_kcbadj`, `pmet_kcbcon`, `pmet_etke`, `pmet_etkr`, `pmet_etks`, `pmet_tew_mm`, `pmet_rew_mm`, `pmet_wfevp_mm`, `pmet_taw_mm`, `pmet_raw_mm`, `pmet_wftrp_mm`, `pmet_es_m`, `pmet_ep_m` | Opt-in trace observability proving WB11 PMET demand comes from migrated `evappm.for` intermediate lineage rather than PT or proxy demand | declared metric units preserved; final `es`/`ep` in `m d^-1` | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0264 trace PMET seam diagnostics | `wb11_et_seed_branch`, `wb11_et_demand_m`, `pmet_es_m`, `pmet_ep_m`, `es_m`, `er_m`, `etp_m`, `ui_m`, `ep_m`, `ws` | Opt-in trace observability that PMET mode consumes EVAPPM `es`/`ep` components at the WB17 seam, publishes `Etp = pmet.ep_m` before SWU, and leaves final `Ep` to post-WB19 root uptake | `m d^-1` ET surfaces and dimensionless stress units preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0265 trace first-divergence diagnostics | `sim_day_index`, `julian_day`, `simulation_year`, `wb11_et_seed_branch`, `pmet_ep_m`, `etp_m`, `ep_m`, `ui_m`, `wb17_ui_layers_m`, `ws`, `pl_lai`, `pl_rtd`, `pl_pltol`, `pl_swu_effective_pltol`, `wb17_swu_storage_to_threshold_layers`, `wb13_total_soil_mm`, `wb13_dp_mm`, `q_m`, `qd_m`, WAT `Snow-Water` | Multi-day trace evidence for the first large longer-season `Ep` residual after PMET seam closure; required to separate WB17/SWU identity defects from upstream demand/growth/storage/snow-runoff coupling | `m d^-1`, `mm`, dimensionless stress ratios | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0267 trace stress-threshold diagnostics | `wb18_theta_layers_m`, `wb18_ul_layers_m`, `wb17_swu_stress_threshold_layers_m`, `wb17_swu_storage_to_threshold_layers`, `wb19_lateral_withdrawal_layers_m`, `etp_m`, `ep_m`, `ui_m`, `wb17_ui_layers_m`, `ws` | Post-lateral/pre-SWU trace evidence for classifying whether WB17 stress thresholds or consumed layer storage own first seasonal `Ep` residuals | `m d^-1`, `m`, and dimensionless ratios preserved | `[DIRECT][Static] + [INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| Zero-canopy partition day | `L = 0` causing `Esp = Eu` and `Etp = 0` under Eq. [5.2.8]-[5.2.9]. | Explicit partition-equation outcome. | `[DIRECT][Static]` |
| High-canopy LAI cap day | `L > 3` where potential transpiration is not additionally adjusted by Eq. [5.2.14]. | Explicit LAI-adjustment branch limit. | `[DIRECT][Static]` |
| Stage-two soil evaporation day | Accumulated evaporation exceeds stage-one limit and stage-two equation governs. | Explicit Eq. [5.2.10]-[5.2.12] stage behavior. | `[DIRECT][Static]` |
| Snow-supplied evaporation day | Snow-water content satisfies daily `Es`, yielding no soil-water extraction for evaporation. | Explicit §5.3 snow-first soil-evaporation ordering. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Layer compensation day | Some layers have deficit-adjusted uptake while others compensate within Eq. [5.3.4] bounds. | Explicit root-compensation statement in §5.3. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invalid States

- Negative or non-finite ET rates/depths (`Eu`, `Esp`, `Etp`, `Esb`, `Es`, `UPi`, `Ui`) beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Partition mismatch for Eq. [5.2.8]-[5.2.9] beyond declared tolerance (`Eu` not equal to `Esp + Etp`). `[DIRECT][Static] + [INFERENCE][Static]`
- Bare-soil stage output violating declared stage transitions or residue attenuation relation. `[DIRECT][Static] + [INFERENCE][Static]`
- Soil-evaporation depth bounds violated (`ds < 0` or `ds > dx`) or snow-first precedence violated. `[DIRECT][Static] + [INFERENCE][Static]`
- Layer uptake branch mismatch (`Ui > UPi`, negative uptake, or missing Eq. [5.3.4] threshold handling). `[DIRECT][Static] + [INFERENCE][Static]`
- Stress factor outside `[0,1]` or undefined at boundary emission time. `[DIRECT][Static] + [INFERENCE][Static]`
- ET boundary payload missing required surfaces for closure/stress coupling semantics. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-EVAP-P-001: Emit ET partition/stress surfaces (`Eu`, `Esp`, `Etp`, `Es`, `UPi`, `Ui`, `Ws`) with canonical symbols and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-P-002: Apply explicit Eq. [5.2.*] and Eq. [5.3.*] branch logic for stage transitions, LAI adjustment, and deficit uptake; no implicit fallback branches. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-P-003: Enforce invariant failures via typed errors; no silent clamping/defaulting for invalid ET/stress states. `[INFERENCE][Static]`
- OBL-EVAP-P-004: Preserve coupling-ready stress semantics for plant-growth consumers (`Ws` bounded and unit-consistent). `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-P-005: Preserve baseline-authoritative stage-memory and uptake lineage surfaces (`s1`, `s2`, `tu`, `tv`, `UPi`, `Ui`) so downstream contracts/tests can validate deterministic `evap` + `swu` semantics. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-P-006: Preserve opt-in trace observability for aggregate and layer
  `UPi`/`Ui` uptake surfaces so HPHYS0260 residual classification can
  distinguish WB17 internal identity divergence from baseline-magnitude
  follow-up work. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-P-007: Preserve opt-in trace observability for raw/effective
  `pltol`, WB18 `ul(i)`, storage-to-threshold ratios, `Etp`, final `Ep`,
  `ΣUi`, `lai`, and `rtd` so HPHYS0261 residual classification can separate
  `swu` stress clipping from upstream `evap`/initialization magnitude.
  `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-P-008: Preserve opt-in trace observability for `pmetpara` sidecar
  mode, selected `kcb`/`rawp`, fallback status, and actual `wb11_et_demand`
  seed branch before a package assigns H1/H7/H39 `Ep` residual ownership to
  `evap`/`evappm` lineage or changes ET-demand physics. `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-EVAP-C-001: Water-balance consumers must ingest ET withdrawal with Eq. [5.1.1] sign/units semantics unchanged. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-C-002: Plant-growth consumers must reject malformed/out-of-domain stress payloads and preserve Eq. [8.2.15] supply-demand interpretation. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-C-003: Climate and winter boundary consumers/providers must preserve forcing/snow semantics required for ET branch selection and snow-first evaporation precedence. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-EVAP-C-004: All consumers must fail explicitly on invariant-violating payloads and carry invariant IDs in error context. `[INFERENCE][Static]`
- OBL-EVAP-C-005: Water-balance consumers must preserve stage-memory and root-uptake lineage ordering assertions required for baseline-authoritative ET closure and must not substitute simplified scalar-only ET extraction semantics once SIMIMPL23 migration gates activate. `[DIRECT][Static] + [INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Potential ET and partition closure (`INV-EVAP-001/002`) | ET potential + partition assembly | Hard error; reject ET publish for the day | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Soil-evaporation stage and attenuation (`INV-EVAP-003/004`) | stage-state and attenuation evaluation | Hard error on branch/order/domain failure | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Root-zone extraction and uptake (`INV-EVAP-005/006/007`) | root-zone ET distribution stage | Hard error on bounds or branch mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Stress and boundary coupling (`INV-EVAP-008/009`) | ET-to-plant and ET-to-water-balance handoff | Hard error on malformed stress or missing boundary payload | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Scope/governance labeling (`INV-EVAP-010`) | review/verification/promotion | Governance `HOLD` until scope/method caveats are explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB17 ET production execution and guards (`INV-EVAP-011/012`) | ET kernel execution and guard validation | Hard error on malformed ET domains or invalid deterministic partition updates | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Constants and Parameters Table

| Constant/parameter | Units | Domain | Contract use | Authority |
|---|---|---|---|---|
| `WB11_ET_STATUS_OK` | status message id | `HKERNEL-WB11-ET-OK-001` | Typed nominal status for successful ET phase execution | REF-EVAP-CH5-BAL |
| `WB11_ET_GUARD_MISSING` | status message id | `HKERNEL-WB11-ET-E-001` | Typed missing-input guard code | REF-EVAP-PHYS-BOUNDS |
| `WB11_ET_GUARD_NONFINITE` | status message id | `HKERNEL-WB11-ET-E-002` | Typed non-finite guard code | REF-EVAP-PHYS-BOUNDS |
| `WB11_ET_GUARD_DOMAIN` | status message id | `HKERNEL-WB11-ET-E-003` | Typed domain guard code | REF-EVAP-PHYS-BOUNDS |
| `WB17_ETP_ZERO_THRESHOLD` | `m d^-1` | `1e-12` | Explicit zero-demand denominator guard for `Ws` | REF-EVAP-CH5-LINK |
| `WB17_CANOPY_BARE_SOIL_OFFSET` | coefficient | `0.1` | Baseline uncovered-soil branch offset in `eaj = exp(-0.5*(cv+0.1))` | REF-EVAP-LEGACY-SOILX |
| `WB17_CANOPY_EAJ_COEFF` | coefficient | `0.5` | Baseline uncovered-soil exponential coefficient in `eaj` | REF-EVAP-LEGACY-SOILX |
| `WB17_SOIL_EVAP_DEPTH_M` | `m` | `0.10` | Baseline upper-zone soil evaporation depth limit for layer extraction | REF-EVAP-LEGACY-SOILX |
| `WB17_TRANSPIRATION_LAI_FULL_COVER` | `m^2 m^-2` | `3.0` | Baseline LAI cap for potential transpiration branch | REF-EVAP-LEGACY-ETP |
| `WB17_SWU_UB` | coefficient | `3.065` | Baseline root-uptake exponential distribution coefficient | REF-EVAP-LEGACY-SWU |
| `WB17_SWU_UOB` | coefficient | `0.953346` | Baseline root-uptake normalization coefficient | REF-EVAP-LEGACY-SWU |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity). `[DIRECT][Static]` Contract-specific tolerances:

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-EVAP-001 | Partition closure residual for Eq. [5.2.8]-[5.2.9] | `<= 1e-9 m d^-1` | Residual computed as `Eu - (Esp + Etp)` in ET-depth units. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-EVAP-002 | Non-negative comparator tolerance for ET rates/depths | lower bound `>= -1e-12` in declared ET units | Comparator-noise allowance only; runtime hard-fails on material negatives. | `[INFERENCE][Static]` |
| TOL-EVAP-003 | Soil-evaporation depth bounds tolerance | `-1e-12 m <= ds <= dx + 1e-12 m` | Preserves explicit `0..dx` semantics with floating-noise allowance. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-EVAP-004 | Stress-factor bounds tolerance | `-1e-12 <= Ws <= 1 + 1e-12` | Runtime still enforces bounded stress semantics. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Test-Vector Obligations

Minimum WB17 ET production-kernel conformance vectors:

1. WB17 nominal partition vector emits deterministic ET components (`Ep`, `Es`,
   `Er`) plus aggregate `ET`/`Ws` and updates `wb11_soil_water` from valid WB17
   inputs.
2. Soil-limited vector preserves residue evaporation (`Er`) while reducing
   transpiration (`Ep`) and `Ws` via explicit supply-demand stress relation.
3. Missing required WB17 ET symbol (`wb17_residue_interception`) hard-fails
   with typed status `HKERNEL-WB11-ET-E-001`.
4. Non-finite/domain-invalid WB17 ET inputs hard-fail with typed status
   family `HKERNEL-WB11-ET-E-002/003` and do not mutate orchestrator
   writeback surfaces.
5. Stage-memory transition vector proves deterministic `s1/s2/tu/tv` branch
   progression and deficit transition behavior under infiltration/non-infiltration
   days.
6. Root-uptake lineage vector proves depth-aware `UPi`/`Ui` extraction and
   stress ratio lineage (`Ws = ΣUi/Etp`) consistent with baseline `swu`
   semantics.
7. HPHYS0249 upper-zone soil-evaporation vector proves `Es` extraction mutates
   `wb18_perc_theta_####` layer storage before `wb11_soil_water` writeback.
8. HPHYS0249 root-uptake vector proves `Ep` extraction uses `rtd`,
   `wb18_perc_ul_####`, and `wb18_perc_theta_####` rather than scalar aggregate
   subtraction.
9. HPHYS0250 final-lineage vector proves scheduler execution retains PL
   runtime activation surfaces so growth can produce nonzero `rtd` before
   WB17/WB19 phases, and WB13 consumes final post-root-uptake flux `Ep`.
10. HPHYS0260 trace-localization vector proves opt-in trace rows serialize
    aggregate `UPi`/`Ui`, layer `UPi_####`/`Ui_####`, final `Ep`, `Etp`,
    `Ws`, and post-uptake layer storage from the post-`PlantRootUptake`
    writeback surface.

## WB13 Daily Output Coupling Addendum

### WB13 ET-Coupled Output Symbols

| WB13 column | ET coupling surface | Units |
|---|---|---|
| `Ep` | Plant-transpiration component exported for daily reporting | `mm` |
| `Es` | Soil-evaporation component exported for daily reporting | `mm` |
| `Er` | Residue-evaporation component exported for daily reporting | `mm` |

### WB13 Coupling Requirements

1. WB13 daily rows must include finite ET-component symbols `Ep`, `Es`, and
   `Er` with non-negative daily magnitudes.
2. ET-component omission or malformed ET-component values are invalid WB13
   output states and must hard-fail via WB13 typed guards.
3. ET-component boundaries emitted to WB13 remain unit-consistent with
   Chapter-5 daily water-balance closure expectations.
4. When WB17 publishes both a pre-root-uptake `Ep` seed and a post-root-uptake
   `Ep` flux, WB13 must consume the post-root-uptake flux-authoritative value.

## WB15 Canopy-Interception Coupling Addendum

### WB15 Coupling Surfaces

| Surface | Symbols |
|---|---|
| Interception closure surface | `I` |
| ET closure surfaces | `ET`, `Ws` |
| Plant-state interception drivers (external producer) | `cancov`, `lai`, `vdmt` |

### WB15 Coupling Requirements

1. Canopy interception remains an explicit Chapter-5 closure term (`I`) and is
   not implicitly folded into `ET` by boundary consumers.
2. ET kernel outputs (`ET`, `Ws`) must remain finite and domain-valid when
   interception coupling is active.
3. Missing/non-finite/out-of-domain interception symbol `I` at storage-closure
   consumers is a typed hard-fail state; no silent ET-side substitution is
   allowed.
4. Plant-runtime canopy interception drivers are owned by plant-domain
   producers; ET consumers must not redefine or clamp producer payloads.

### WB15 Contract-Test Vectors

1. Canopy-coupled storage closure vector verifies `I` is consumed as a distinct
   closure term alongside ET and does not overwrite ET semantics.
2. Missing interception symbol `I` at closure consumer boundaries hard-fails
   with typed missing-input status.
3. Non-finite interception symbol `I` hard-fails with typed non-finite status.

## SIMIMPL21 WB11 ET Stage-Memory and Root-Uptake Authority Addendum

1. Canonical ET authority includes baseline stage-memory state surfaces
   `s1`, `s2`, `tu`, and `tv` with deterministic branch transitions for
   stage-one/stage-two evaporation, including infiltration-coupled reset and
   deficit branch behavior from baseline `evap.for`.
2. Soil-evaporation extraction authority is layer-aware: evaporation demand is
   satisfied from layer storage `st(i)` using explicit depth-partitioned
   withdrawal semantics and cannot be represented solely as scalar
   `wb11_soil_water` decrement in canonical closure claims.
3. Root-zone transpiration extraction authority follows baseline `swu.for`:
   layer potential uptake (`UPi`) distribution, deficit adjustment to actual
   uptake (`Ui`), and stress ratio lineage to `Ws`.
4. Contract-derived tests for SIMIMPL22 must include stage-memory and
   root-uptake vectors keyed to these baseline lineage assertions before any
   production-kernel ET migration edits are promotable.

## HPHYS0242 Hourly ET/Infiltration Ordering Addendum

1. Hourly-lane ET is a final-hour consumer in the baseline water-balance loop:
   WB14 infiltration and WB18 percolation mutate layer state first, and ET
   observes that same-pass state.
2. `wb12_infiltration` is only authoritative for ET when it represents the
   same-pass WB14-derived infiltration lineage for the current day/lane.
   Compatibility or pre-WB14 stale state cannot be used to satisfy ET
   stage-memory or soil-extraction claims.
3. Missing, non-finite, negative, or conflict-labeled same-pass infiltration
   lineage must hard-fail in hourly ET lanes; defaulting to zero is prohibited.
4. Contract-derived vectors must include an ET stale-infiltration conflict and
   a scheduler-order proof that ET follows same-day infiltration/percolation.

## HPHYS0249 WB17 Layer-Storage ET Addendum

1. WB17 production authority maps baseline `st(i)` to
   `wb18_perc_theta_####`. The aggregate `wb11_soil_water` is a recomputed
   consequence of layer mutation, not the primary ET extraction state.
2. Soil evaporation follows baseline `evap.for` lineage: `eaj =
   exp(-0.5*(cancov+0.1))`, residue interception is removed before stage
   evaluation, residue evaporation is split back out, and the remaining soil
   evaporation demand is withdrawn from the upper `0.10 m` of layer storage.
3. Plant transpiration follows baseline `swu.for` lineage after WB19
   drainage/lateral mutation when `Etp > 0` and `rtd > 0`: uptake is
   distributed by root-depth weighting, reduced by
   `pltol * wb18_perc_ul_i` deficit scaling when applicable, capped by
   available `wb18_perc_theta_i`, and accumulated into `Ws = ΣUi / Etp`.
4. After WB17 soil evaporation and after post-WB19 root uptake,
   `wb11_soil_water` must be recomputed from
   `wb18_perc_theta_####`, `thetdr_####`, `dg_####`, and optional frozen-depth
   surfaces using the same lineage as `SC-WATBAL-001#INV-WATBAL-037`.
5. Contract-derived vectors must include a soil-evaporation layer-mutation
   case and a root-uptake layer-mutation case before production code edits are
   promotable.

## HPHYS0250 Final Ep Lineage Addendum

1. Management-derived PL runtime activation surfaces must remain present through
   canonical scheduler execution on active-crop days. Removing
   `pl_schedule_slot_count` or an equivalent activation sentinel before
   scheduling suppresses growth phases and is invalid for promoted `Ep` lineage
   claims unless runner-side scheduling first proves that the current
   simulation-year/day has no active crop under the same PL slot-window rules.
   Baseline perennial slots with `jdplt=0` remain active under `ptgrp` semantics
   (`sdate > jdplt` and `jdstop=0` means all simulation days are eligible);
   they are not inactive placeholders.
   Because decomposition runs before same-day ET/root uptake, initial runtime
   publication must provide neutral prior stress (`Ws=1.0`) when no same-day
   stress has yet been computed.
2. Growth-produced `rtd` is the active root-depth boundary for `swu` root uptake.
   If management data are present, `rtd=0` is valid only as a produced plant
   state, not as a scheduler construction artifact caused by disabled growth or
   calendar-year values supplied where PL dispatch requires simulation-year
   indices.
3. Post-WB19 `PlantRootUptake` owns final `Ep`: `Ep = ΣUi`, with `UPi`, `Ui`,
   and `Ws` lineage preserved from `swu.for`. The ET-phase seed `Ep=0` is not a
   daily publication authority after root uptake runs.
4. WB13 daily output must consume flux-authoritative final `Ep` when same-name
   state and flux aliases exist. State-surface `Ep` aliases cannot shadow final
   root-uptake flux in semantic comparator surfaces.
5. HPHYS0250 contract-derived tests must include a runner-side PL sentinel
   preservation guard and a WB13 final-flux publication vector before
   production code edits are promotable.

## HPHYS0251 SWU Uptake Magnitude Addendum

1. Management parsing must preserve crop-specific `pltol(itype)` from the
   baseline plant residue line (`infile.for` `oratea`, `orater`, `otemp`,
   `pltol`, ...) and publish it to the runtime surface consumed by
   `PlantRootUptake`; the baseline default `0.25` remains valid only when no
   crop-specific value is present.
2. `PlantRootUptake` must apply the baseline `swu.for` normalization branch to
   the consumed raw `pltol`: `<=0` becomes `0.25`, values above `0.4` become
   `0.4`, and values below `0.1` become `0.1`. Non-finite values remain typed
   domain errors, not defaults.
3. Layer potential uptake `UPi_####` must be published from the baseline
   cumulative root-depth weighting before water-stress and storage caps.
   Aggregate `UPi` is the sum of layer potential uptake surfaces.
4. Layer actual uptake `Ui_####` must be published after water-stress scaling
   and available-storage caps against post-WB19 `wb18_perc_theta_####`.
   Aggregate `Ui`, final `Ep`, and final `Ws` derive from the same actual layer
   uptake vector.
5. Closure evidence must compare H1/H13/H39 diagnostics and the full 39
   hillslope suite against HPHYS0250 metrics. Promotability remains `HOLD` when
   `Ep`, `Ws`, or aggregate storage residuals are not materially improved or
   when improvement depends on heuristic tuning instead of baseline `swu.for`
   lineage.

## HPHYS0260 WB17 Trace Localization Addendum

1. H1/H7/H39 `Ep` residual classification must consume the
   post-`PlantRootUptake` trace row, not only final WB13 WAT columns.
2. Required trace fields are aggregate `UPi`, aggregate `Ui`, layer
   `UPi_####`, layer `Ui_####`, final `Ep`, `Etp`, `Ws`, and
   post-uptake `wb18_perc_theta_####`.
3. The trace classifier must verify `Ep = ΣUi_####`, aggregate `Ui = ΣUi_####`,
   aggregate `UPi = ΣUPi_####`, `0 <= Ui_#### <= UPi_####`, and
   `Ws = Ep/Etp` when `Etp > 0`.
4. If these identities close, the stable H1/H7/H39 day-1 `Ep` residual is not
   evidence of trace publication or WB13 shadowing failure; continuation must
   target baseline-authoritative magnitude/initialization lineage unless new
   divergence evidence appears.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-EVAP-001 | Per-invariant comparator vectors for all ET invariant families are not yet curated in this package. | Limits immediate automation depth for invariant-specific acceptance checks. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-EVAP-002 | WB17 now fixes executed runtime aliases for `Eu`, `L`, and residue-partition ET symbols, but cross-domain alias harmonization for full Chapter-5 ET variable family remains incomplete. | Partial alias closure still leaves downstream harmonization risk. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-EVAP-003 | Companion contracts (`SC-PERC-001`, `SC-SUBHYD-001`, `SC-RESIDUE-001`) are not fully authored, so coupled ownership boundaries remain provisional. | Promotion-readiness depends on downstream contract completion/consistency. | non-promotable | `[DIRECT][Static]` |
| GAP-EVAP-004 | Chapter-5 validation emphasizes total ET and water-balance behavior; component-level partition validation (`Esp` vs `Etp` vs stage transitions) is not fully separated in available cited evidence. | Partition-subcomponent confidence is lower than aggregate ET confidence until dedicated evidence is added. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-EVAP-005 | Canonical authority now explicitly defines legacy stage-memory/state-transition, layer `st(i)` extraction, and `swu` uptake lineage (`s1`, `s2`, `tu`, `tv`, `UPi`, `Ui`), but full production WB17 runtime parity remains pending until HPHYS0249 implementation/test evidence closes layer-storage extraction and remaining snow/runtime coupling residuals are dispositioned. | Contract authority is closed for WB17 layer-first migration; implementation promotability remains blocked until HPHYS0249 evidence lands and residual families are not in known violation. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-EVAP-006 | HPHYS0251 adds explicit authority for management-derived `pltol(itype)`, legacy normalization, and layer `UPi_####`/`Ui_####` publication, but implementation promotability remains blocked until package evidence shows material `Ep`/`Ws` and aggregate-storage residual reduction without heuristic tuning. | Root-uptake deficit scaling authority is complete, but runtime parity confidence remains tied to HPHYS0251 implementation and full-suite metrics. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-EVAP-007 | HPHYS0260 adds trace-grade WB17 residual classification authority but does not itself change root-uptake physics. | Closure remains `HOLD` when identities close but comparator residuals persist, because follow-on work must then target baseline-authoritative magnitude or initialization lineage. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-EVAP-008 | HPHYS0262 made PMET demand-seeding lineage observable; HPHYS0263 must close the WB11 demand portion by migrating pinned `evappm.for:181-388` when `pmetpara.mode.iflget != 1`. | PMET sidecar discovery and crop lookup evidence can classify the residual, but cannot close WB17 parity with PT or proxy demand under PMET mode. | non-promotable until HPHYS0263 gate evidence passes | `[DIRECT][Static]` |
| GAP-EVAP-009 | HPHYS0263 intentionally scopes to the WB11 demand seed subset of `evappm.for`; post-ET soil redistribution in `evappm.for:460-523` remains separately governed if later diagnostics assign residual ownership there. | Full routine migration includes state mutation after `es`/`ep`; this package only promotes the demand seam needed to remove PT seeding under PMET mode. | promotable-with-risk for WB11 demand, non-promotable for full `evappm` state-redistribution closure | `[DIRECT][Static]` |
| GAP-EVAP-010 | HPHYS0264 corrects the WB11/WB17 PMET component seam but does not complete `evappm.for:460-523` post-ET soil redistribution parity beyond existing openWEPP layer-extraction behavior. | PMET component wiring can close double-partition risk, but full EVAPPM storage redistribution remains separately non-promotable until baseline-authoritative migration evidence lands. | non-promotable for full `evappm` state-redistribution closure | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-EVAP-011 | HPHYS0265 first-divergence localization is an ownership gate, not itself a proof that WB17/SWU math is wrong. | If trace identities close at the first large `Ep` divergence, the package must keep disposition in `HOLD` and hand off narrowed upstream/storage/snow-runoff ownership instead of patching WB17 heuristically. | non-promotable for production correction without baseline-authoritative defect proof | `[INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-06-03` | `15` | `Codex` | HPHYS0262 amendment: added `INV-EVAP-020` requiring PMET sidecar, selected `kcb`/`rawp`, fallback status, and actual ET-demand seed-branch lineage before closing H1/H7/H39 `Ep` magnitude residuals; baseline `evappm.for` migration is required for PMET closure. |
| `2026-06-03` | `16` | `Codex` | HPHYS0263 amendment: added `INV-EVAP-021` and `REF-EVAP-LEGACY-PMET-DEMAND` requiring the pinned `evappm.for:181-388` PMET demand subset to seed WB11 demand when `pmetpara.mode.iflget != 1`, with intermediate trace diagnostics and no PT/proxy substitution. |
| `2026-06-03` | `17` | `Codex` | HPHYS0263 amendment: added `REF-EVAP-LEGACY-SUNMAP-RADPOT` and required `radpot` to be runtime-provided or derived from pinned `sunmap.for:181-234` before EVAPPM `rso` computation. |
| `2026-06-03` | `18` | `Codex` | HPHYS0264 amendment: added `REF-EVAP-LEGACY-PMET-SEAM` and `INV-EVAP-022` requiring PMET-mode WB17 to consume migrated `pmet.es_m`/`pmet.ep_m` components directly, preserve SWU final `Ep`, and prohibit Priestley-Taylor re-partition of PMET `ep`. |
| `2026-06-03` | `19` | `Codex` | HPHYS0264 review disposition: corrected PMET `pmet.es_m` domain to reject material negatives while snapping only within-tolerance negative roundoff, and reconciled remaining full `evappm` redistribution scope to `evappm.for:460-523`. |
| `2026-06-03` | `20` | `Codex` | HPHYS0265 amendment: added `INV-EVAP-023` requiring first-large longer-season `Ep` divergence localization evidence before assigning seasonal `Ep` residual ownership or patching WB17/SWU behavior. |
| `2026-06-03` | `21` | `Codex` | HPHYS0267 amendment: added `INV-EVAP-024` requiring post-lateral/pre-SWU stress-threshold lineage evidence before WB17 residual ownership claims. |
| `2026-06-03` | `14` | `Codex` | HPHYS0261 amendment: added `INV-EVAP-019` requiring trace-grade WB17 `Ep` magnitude/initialization evidence across `evap` partition state, `swu` effective `pltol`, WB18 `ul(i)`, and legacy call-order provenance. |
| `2026-06-03` | `13` | `Codex` | HPHYS0260 amendment: added `INV-EVAP-018` requiring trace-grade post-`PlantRootUptake` aggregate/layer `UPi`/`Ui`, final `Ep`, `Etp`, and `Ws` identity evidence before assigning H1/H7/H39 `Ep` residual ownership. |
| `2026-06-02` | `12` | `Codex` | HPHYS0251 amendment: added `INV-EVAP-017` for baseline `swu.for` uptake magnitude, crop-specific `pltol(itype)` projection/normalization, and layer `UPi_####`/`Ui_####` publication. |
| `2026-06-02` | `11` | `Codex` | HPHYS0250 amendment: added `INV-EVAP-016` requiring PL runtime activation preservation, post-WB19 final `Ep = ΣUi` lineage, and WB13 flux-authoritative final `Ep` publication. |
| `2026-06-02` | `10` | `Codex` | HPHYS0249 amendment: added `INV-EVAP-015` requiring WB17 `Ep`/`Es` production to mutate `wb18_perc_theta_####` layer storage using baseline `evap.for` and `swu.for` lineage before aggregate `wb11_soil_water` writeback. |
| `2026-06-02` | `10a` | `Codex` | HPHYS0249 review follow-up: corrected `WB17_TRANSPIRATION_LAI_FULL_COVER` citation to `evap.for` authority and recorded the per-crop `pltol` runtime projection gap. |
| `2026-06-01` | `9` | `Codex` | HPHYS0242 amendment: added `INV-EVAP-014`, baseline hourly final-hour ET ordering authority, same-pass WB14 infiltration lineage requirements, and stale/default infiltration rejection posture for hourly ET lanes. |
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-07 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-5/8 authority anchors, invariants, guard map, alias map, obligations, tolerances, and gap register for SCI-07 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: added missing symbol/alias coverage (`Θc`, `ET`), normalized evidence-mode casing, strengthened snow provenance anchor wording, and evidence-tagged degenerate/tolerance claims. |
| `2026-05-23` | `3` | `Codex` | WB10 amendment: added explicit ET phase-entry routing authority, unsupported-class typed hard-fail posture, and WB10 ET test-vector obligations. |
| `2026-05-23` | `4` | `Codex` | WB11 amendment: promoted ET section from routing-only scaffolding to production-kernel authority with deterministic `ET`/`Ws` updates, typed ET guard codes (`HKERNEL-WB11-ET-E-001..003`), and WB11 contract-derived vectors. |
| `2026-05-23` | `5` | `Codex` | WB13 amendment: added ET component coupling authority for canonical daily output columns (`Ep`, `Es`, `Er`) with explicit WB13 malformed-output hard-fail posture. |
| `2026-05-23` | `6` | `Codex` | WB15 amendment: added explicit canopy-interception coupling requirements so `I` remains a distinct closure term relative to ET outputs (`ET`, `Ws`) under typed consumer guard posture. |
| `2026-05-23` | `7` | `Codex` | WB17 amendment: replaced WB11 ET surrogate algorithm authority with equation-driven WB17 partition semantics (`Esp`, `Etp`, `Er`, `Es`, `Ep`) using explicit runtime alias mapping and WB17 contract-derived vector obligations. |
| `2026-05-25` | `8` | `Codex` | SIMIMPL21 amendment: added baseline-authoritative stage-memory and root-uptake lineage authority (`s1`, `s2`, `tu`, `tv`, layer extraction, `UPi/Ui/Ws`) with explicit legacy provenance anchors and downstream SIMIMPL22/SIMIMPL23 gating obligations. |
