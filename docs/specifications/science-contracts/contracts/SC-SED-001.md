---
contract_id: SC-SED-001
title: Hillslope Erosion Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 55
producer_scope:
  - Hillslope sediment continuity, detachment/deposition, and transport-capacity surfaces
  - Event erosion boundary payloads consumed by routing/channel domains
  - Sediment size-class and enrichment surfaces at OFE and hillslope exits
consumer_scope:
  - Watershed/channel routing consumers requiring hillslope erosion payload semantics
  - Comparator and replay consumers using erosion closure and sign-consistency surfaces
  - Adjacent soil/runoff/hydraulics domains providing required coupling inputs
evidence_level: Static
last_reviewed: 2026-07-13
supersedes: []
superseded_by: []
---

# SC-SED-001 Hillslope Erosion Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for hillslope sediment continuity,
rill/interrill detachment-deposition behavior, transport-capacity constraints,
and hillslope erosion boundary payloads consumed by downstream routing domains.

## Scientific Scope

In scope:
- Hillslope sediment continuity and sign conventions for `Di`, `Df`, and `G`. `[DIRECT][Static]`
- Rill detachment and deposition branch semantics governed by hydraulic shear and transport capacity. `[DIRECT][Static]`
- Normalized erosion-equation parameter semantics used for profile/OFE routing. `[DIRECT][Static] + [INFERENCE][Static]`
- Sediment enrichment and particle-size-fraction update constraints at deposition transitions and hillslope/OFE exits. `[DIRECT][Static] + [INFERENCE][Static]`
- Required coupling surfaces from hydrology/hydraulics/soil domains into hillslope erosion and from hillslope erosion to channel routing domains. `[DIRECT][Static] + [INFERENCE][Static]`

Out of scope:
- Kernel implementation details and Rust API naming. `[INFERENCE][Static]`
- Channel detachment/deposition and watershed outlet sediment routing internals owned by `SC-ROUTE-001`. `[DIRECT][Static] + [INFERENCE][Static]`
- Parameter-calibration campaigns and empirical re-fitting of Chapter-11 coefficients. `[INFERENCE][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-SED-CH11-INTRO | `references/50201000/chap11.pdf` §11.1 | Declares hillslope erosion governing-equation scope (sediment continuity, detachment, deposition, shear, transport capacity). | `[DIRECT][Static]` |
| REF-SED-CH11-CONT | `references/50201000/chap11.pdf` §11.2.1 Eq. [11.2.1] | Sediment continuity equation and sign conventions for `Di` and `Df`. | `[DIRECT][Static]` |
| REF-SED-CH11-DET | `references/50201000/chap11.pdf` §11.2.1 Eq. [11.2.2]-[11.2.3] | Rill detachment-capacity branch, threshold behavior (`τf` vs `τc`), and dependence on `Kr`. | `[DIRECT][Static]` |
| REF-SED-CH11-DEP | `references/50201000/chap11.pdf` §11.2.1 Eq. [11.2.4] | Deposition branch when `G > Tc`, including `β`, `Vf`, and `q` scaling. | `[DIRECT][Static]` |
| REF-SED-CH11-HYDRO | `references/50201000/chap11.pdf` §11.2.2 Eq. [11.2.5]-[11.2.6] | Required hydrologic input variables (`Pr`, `tr`, `Ie`, `te`) and event-to-steady transposition definitions. | `[DIRECT][Static]` |
| REF-SED-CH11-SHEAR | `references/50201000/chap11.pdf` §11.2.3 Eq. [11.2.7] | Shear-stress formulation and active-soil shear partition (`fs/ft`). | `[DIRECT][Static]` |
| REF-SED-CH11-TC | `references/50201000/chap11.pdf` §11.2.4 Eq. [11.2.8]-[11.2.9] | Transport-capacity power-law relation and sandy-soil adjustment floor (`tcadjf >= 0.30`). | `[DIRECT][Static]` |
| REF-SED-CH11-NORM | `references/50201000/chap11.pdf` §11.3 Eq. [11.3.7]-[11.3.15] | Normalized detachment/deposition parameters (`η`, `τcn`, `θ`, `φ`) and solution/output conversion constraints. | `[DIRECT][Static]` |
| REF-SED-CH11-DOWNVAR | `references/50201000/chap11.pdf` §11.4 Eq. [11.4.1]-[11.4.6] | Downslope-variability/runon normalization semantics for OFE-strip routing. | `[DIRECT][Static]` |
| REF-SED-CH11-ENRICH | `references/50201000/chap11.pdf` §11.5 Eq. [11.5.1]-[11.5.6] | Size-class mass conservation and enrichment-ratio semantics. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-CH10-HYDRAULICS | `references/50201000/chap10.pdf` §10.1-§10.2, Eq. [10.1.2], [10.2.1] | Hydraulic roughness/shear partition context for erosion-consumed shear surfaces (`fs`, `ft`). | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-CH7-EROD | `references/50201000/chap7.pdf` §7.10-§7.11, Eq. [7.10.1]-[7.10.15], [7.11.1]-[7.11.18] | Adjusted interrill/rill erodibility and critical-shear parameters (`Kiadj`, `Kradj`, `τcadj`) consumed by Chapter-11 normalizations. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-CH4-RUNOFF | `references/50201000/chap4.pdf` §4.4.2-§4.4.4, Eq. [4.4.17]-[4.4.30] | Hydrology component authority for peak-runoff and effective-duration surfaces used by erosion equations. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-CH13-COUPLING | `references/50201000/chap13.pdf` §13.1 pass-file list | Downstream coupling semantics for hillslope erosion outputs (detachment, deposition, class concentrations, class fractions). | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-HBP-FORMAT | `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` (`EVENT Payload`) | Canonical binary pass serialization field names and units for routing-boundary sediment payloads (`total_detachment_kg`, `total_deposition_kg`, `sediment_concentration_kg_m3[npart]`, `particle_diameter_m[npart]`, `particle_flow_fraction[npart]`). | `[DIRECT][Static]` |
| REF-SED-HBP-READER | `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md` (`Read Contract`, `Required Invariants`) | Watershed reader fail-closed semantics for missing/invalid hillslope payload fields and no-text-fallback posture. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-LEGACY-PARAM | `/workdir/wepp-forest_260430_baseline/src/param.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy normalized-parameter authority (`eata`, `tauc`, `theta`, `phi`) used for Wave-1 runtime parameter derivation provenance. | `[DIRECT][Static]` |
| REF-SED-LEGACY-EROD | `/workdir/wepp-forest_260430_baseline/src/erod.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy detachment-capacity and branch-condition authority used for Wave-1 detachment/deposition runtime branch ordering. | `[DIRECT][Static]` |
| REF-SED-LEGACY-RUNGE | `/workdir/wepp-forest_260430_baseline/src/runge.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy continuity evolution form (`dG/dx` update term as `dcap*((tcap-load)/tcap) + theta`) used for Wave-1 branch/continuity guard alignment. | `[DIRECT][Static]` |
| REF-SED-LEGACY-CONTIN-ROUTE | `/workdir/wepp-forest_260430_baseline/src/contin.for` + `/workdir/wepp-forest_260430_baseline/src/route.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy call-chain authority for hillslope sediment routing (`call route` from CONTIN) and per-segment upper-boundary detach/deposit routing control flow. | `[DIRECT][Static]` |
| REF-SED-LEGACY-PROFIL | `/workdir/wepp-forest_260430_baseline/src/profil.for:37-54` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy Wave-1 geometry normalization authority: `slen` is the terminal slope-profile station and every `xstar` is divided by that terminal station, making the route toe exactly `1.0`. | `[DIRECT][Static]` |
| REF-SED-LEGACY-MOFE-QIN | `/workdir/wepp-forest_260430_baseline/src/xinflo.for:130-151` + `/workdir/wepp-forest_260430_baseline/src/route.for:139-154` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy multi-OFE erosion handoff authority: downstream `qin` follows prior-OFE erosion `qout`, and sediment particle fractions are copied from the prior OFE when flow enters. | `[DIRECT][Static]` |
| REF-SED-LEGACY-XCRIT | `/workdir/wepp-forest_260430_baseline/src/xcrit.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy `mshear` case classification authority (`1..5`) used by hillslope segment routing branch dispatch. | `[DIRECT][Static]` |
| REF-SED-LEGACY-DEPC | `/workdir/wepp-forest_260430_baseline/src/depc.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy deposition-equation partial-solution authority used at route segment upper boundaries and post-detachment deposition follow-up. | `[DIRECT][Static]` |
| REF-SED-LEGACY-DEPEND | `/workdir/wepp-forest_260430_baseline/src/depend.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy authority for solving where deposition ends inside a segment (`xdend`) under increasing/decreasing flow cases. | `[DIRECT][Static]` |
| REF-SED-LEGACY-DEPOS | `/workdir/wepp-forest_260430_baseline/src/depos.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy segment deposition profile update authority (`detach`, `tc`, `load`) in route deposition branches. | `[DIRECT][Static]` |
| REF-SED-LEGACY-ENRICH | `/workdir/wepp-forest_260430_baseline/src/enrich.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy particle-class enrichment authority for deposition transitions and OFE-end finalization (`iendfg` terminal call). | `[DIRECT][Static]` |
| REF-SED-LEGACY-RTPART | `/workdir/wepp-forest_260430_baseline/src/rtpart.for` + `/workdir/wepp-forest_260430_baseline/src/grow.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Provenance correction anchor: `rtpart.for` is plant root-mass partitioning (growth domain) and is not an erosion-routing companion routine. | `[DIRECT][Static]` |
| REF-SED-LEGACY-SLOSS-SEDCON | `/workdir/wepp-forest_260430_baseline/src/sloss.for:305-317` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy per-class exit-concentration authority: `sedcon(i) = avsole/(runoff·efflen)·frcflw(i)` (kg/m³, width-independent, zero when `peakro <= 0`), consumed by the watershed as `sedcon(i)·runvol` per-class mass (`wshred.for:180-186`). | `[DIRECT][Static]` |
| REF-SED-LEGACY-SLOSS-QSOUT | `/workdir/wepp-forest_260430_baseline/src/sloss.for:326-333` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy inter-OFE sediment handoff basis: `qsout = dslod2 / effdrn` — the exported load per unit width divided by the flow duration, i.e. a sediment DISCHARGE per unit width (kg·m⁻¹·s⁻¹); zero when contours hold. The RECEIVER nondimensionalizes it as `strldn = qsout · rspace / tcend / width` on its own scales (`param.for:239-245`, width-guarded). | `[DIRECT][Static]` |
| REF-SED-LEGACY-REID | `/workdir/wepp-forest_260430_baseline/src/reid.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy effective-intensity basis: `effdrr = durre` is the duration of RAINFALL-EXCESS periods and `effint` the mean rainfall intensity over them — an interval with no rainfall-excess period contributes no interrill driver (`Di = Ki·I·q` has no supply), the source intent behind the no-excess-hour theta suppression. | `[DIRECT][Static]` |
| REF-SED-LEGACY-ENRICH | `/workdir/wepp-forest_260430_baseline/src/enrich.for` + `route.for:235/250/448/473` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy particle-class enrichment authority: the do-10 composition blend `frcflw_i ← (frcflw_i·lddend + frac_i·rillod + fidel_i·intlod)/ldtop` at every call point; the do-30 per-class analytic depositional re-proportion over `[xtop, xbot]` (per-class `φ_i = β·fall_i/pkro`, per-class transport `ktrato·tcf1_i·{a,b,c}inftc`, `undflo` guard, `term4a` 1e-8 floor, `gend ≥ 0`) with the do-40 normalization to the routed total and the label-50 `sedmax` reproportion; the `iendfg` OFE-end SSA enrichment ratio (`enrato = Σ frcflw_i·SSA_i / ssasol + 0.005`, SSA constants sand 0.05 / silt 4 / clay 20 / organic 1000/1.73); `tcf1_k = ws_k/Σws` from the LAST `yalin` evaluation (`yalin.for:150-160`, the kt2 shear); `route.for:142-160` flow-composition initialization and the `:443` `loadup ≥ lddend` floor. | `[DIRECT][Static]` |
| REF-SED-LEGACY-SEDSEG-WIDTH | `/workdir/wepp-forest_260430_baseline/src/sedseg.for:389-391,512-514` + `/workdir/wepp-forest_260430_baseline/src/input.for:377-392` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy total-mass width scaling: event `tdet`/`tdep` are per-unit-width profile integrals multiplied by the slope-file profile width `fwidth` (total kg) before pass-file serialization; concentration surfaces are never width-scaled. | `[DIRECT][Static]` |
| REF-SED-LEGACY-FRCFLW-INIT | `/workdir/wepp-forest_260430_baseline/src/route.for:142-160` + `/workdir/wepp-forest_260430_baseline/src/param.for:446,452-458` + `/workdir/wepp-forest_260430_baseline/src/enrich.for:205-213` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy exiting-fraction lineage: `frcflw` initializes to the `prtcmp` detached composition `frac` (no upslope inflow), `enrich` re-proportions only in deposition regions plus the terminal end-of-OFE blend; on the non-cropland path (`fidel = frac`, `intdr = 1`) the zero-deposition/zero-inflow blend collapses exactly to `frcflw = frac`. | `[DIRECT][Static]` |
| REF-SED-ADR0036 | [`ADR-0036`](../../../decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md) (Accepted 2026-07-04) | Decision authority for the hydrograph-resolved solve form (per-hour quasi-steady Wave-1 on hydraulically-active hours), the paired `V_h`/`S_h` interchange surfaces, and the conservation/comparator policy. | `[DIRECT][Static]` |
| REF-SED-DC01-SHAPE | `SC-RUNOFFPART-001#INV-RUNOFFPART-031` + `direct_runtime/03_executor.rs` `dc01_surface_transfer_weights` | Default/off hourly-shape authority: while Lane D routing does not own the surface-water path, the unit-normalized DC01 surface-transfer distribution (WB14 excess + hourly saturation carry; ratified uniform fallback for shapeless runoff days) supplies the hourly-runoff shape consumed by the erosion solve, the serialized `V_h` surface, and downstream runon admission. It cannot carry active-routed-water erosion acceptance. | `[DIRECT][Static]` |
| REF-SED-LANED-ROUTED-HYDROGRAPH | `SC-OFEROUTE-001#INV-OFEROUTE-008` / `SC-OFEROUTE-001#INV-OFEROUTE-012` | Active-routed-water hourly-shape authority: when Lane D routing owns the surface-water path, the Wave-1 hourly erosion substrate consumes the routed outlet hydrograph shape for the lane/OFE, expressed as finite non-negative unit-normalized hourly weights on the same daily runoff-volume basis. Missing, malformed, or non-closing routed shapes fail closed; DC01 source weights may remain default/off diagnostics only. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-SED-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative magnitudes for rates/loads where signed behavior is not explicitly defined; finite denominators and bounded fractions. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `x` | `m` | Downslope distance coordinate in sediment continuity equation. | hillslope geometry surface | erosion solver core |
| `G` | `kg s^-1 m^-1` | Sediment load per unit rill width. | erosion continuity solver | hillslope/channel sediment handoff |
| `Di` | `kg s^-1 m^-2` | Interrill sediment delivery to rills (non-negative). | interrill delivery pathway | sediment continuity solver |
| `Df` | `kg s^-1 m^-2` | Net rill erosion rate (`>0` detachment, `<0` deposition). | rill erosion pathway | sediment continuity solver |
| `Dc` | `kg s^-1 m^-2` | Rill detachment capacity term in Eq. [11.2.2]. | rill detachment branch | sediment continuity solver |
| `Tc` | `kg s^-1 m^-1` | Sediment transport capacity per unit rill width. | transport-capacity pathway | detachment/deposition branch selector |
| `Kr`, `τc` | `s m^-1`, `Pa` | Rill erodibility and critical-shear threshold in Eq. [11.2.3]. | soil/erodibility coupling pathway | rill detachment capacity computation |
| `τf` | `Pa` | Hydraulic shear stress acting on soil particles in rills. | hydraulics coupling pathway | branch threshold and transport-capacity computation |
| `β`, `Vf`, `q` | `fraction`, `m s^-1`, `m^2 s^-1` | Deposition branch parameters in Eq. [11.2.4]. | rainfall-type + hydraulics + sediment pathway | deposition rate computation |
| `Pr`, `tr`, `Vt` | `m s^-1`, `s`, `m` | Peak runoff, effective runoff duration, and event runoff depth (`tr = Vt/Pr`). | hydrology/runoff partition coupling | steady-state hydrologic input set |
| `Ie`, `te` | `m s^-1`, `s` | Effective rainfall intensity and effective duration for interrill delivery. | rainfall-disaggregation/hydrology coupling | interrill delivery computation |
| `Kiadj`, `Kradj`, `τcadj` | `kg s m^-4`, `s m^-1`, `Pa` | Adjusted soil erodibility/shear parameters from Chapter-7 consumed by Chapter-11 normalizations. | soil contract boundary payload | erosion normalization parameters |
| `σir`, `SDRRR`, `Fnozzle`, `Rs`, `w` | `m s^-1`, `fraction`, `fraction`, `m`, `m` | Interrill delivery and geometric/nozzle factors in Eq. [11.3.10]. | interrill/hydraulics/irrigation coupling | interrill parameter `θ` computation |
| `η`, `τcn`, `θ`, `φ` | `fraction`, `fraction`, `fraction`, `fraction` | Normalized erosion parameters controlling detachment/deposition equation forms. | normalization pathway | normalized erosion ODE solver |
| `total_detachment_kg`, `total_deposition_kg` | `kg` | Hillslope event detachment/deposition totals exported in watershed pass-file semantics. | hillslope erosion aggregator | channel/watershed routing consumers |
| `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i` | `kg m^-3`, `m`, `fraction` | Particle-class concentration, representative class diameter, and class fraction at hillslope/OFE exits. | size-class routing/enrichment pathway | channel/watershed routing consumers |
| `particle_class_count` | `count` | Particle-class cardinality for serialized routing-boundary payload arrays. | hillslope erosion aggregator | watershed routing payload validator |
| `ER` | `fraction` | Specific-surface-area enrichment ratio (`SSAsed/SSAsoil`). INTERNAL publication surface only (E.4): carried on the direct-runtime erosion publication operands; NOT yet serialized to the pass/HBP interchange — external exposure is a future additive schema extension. | enrichment pathway | internal diagnostics (interchange exposure pending) |
| `hourly_runoff_fraction[h]` | `fraction` | Unit-normalized hourly water-hydrograph shape used by Wave-1 and serialized as `V_h = runvol · w_h`; DC01 is the default/off authority, and the Lane D routed hydrograph is required when routing owns the surface-water path. | runoff-partition default path or Lane D routed-water producer | erosion hourly solve and HBP EVENT surface |
| `hourly_sediment_mass_kg[h]` | `kg` | Hour-integrated exported sediment mass on the same hourly basis as `hourly_runoff_fraction[h]`; `Σ_h S_h` equals the day's exported sediment mass. | erosion hourly solve | HBP EVENT surface and inter-OFE erosion handoff |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-SED-001 | Sediment continuity invariant: `dG/dx = Df + Di` is enforced with declared units/sign semantics, including `Di >= 0` and signed `Df` branch interpretation. | hard-fail | REF-SED-CH11-CONT, REF-SED-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-002 | Rill-detachment branch invariant: when `τf > τc` and `G < Tc`, detachment follows Eq. [11.2.2]-[11.2.3] with explicit threshold handling (`Df = 0` when `τf <= τc`). | hard-fail | REF-SED-CH11-DET | `[DIRECT][Static]` |
| INV-SED-003 | Deposition branch invariant: when `G > Tc`, deposition follows Eq. [11.2.4], preserving denominator domain (`q > 0`) and signed deposition behavior (`Df < 0`). | hard-fail | REF-SED-CH11-DEP, REF-SED-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-004 | Hydrologic-input invariant: peak runoff, effective runoff duration, and effective rainfall intensity surfaces are present and consistent with Eq. [11.2.5]-[11.2.6] transposition semantics. | hard-fail | REF-SED-CH11-HYDRO, REF-SED-CH4-RUNOFF | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-005 | Shear-partition invariant: soil-active shear partition ratio (`fs/ft`) is explicit and bounded, and `τf` surfaces used by erosion routines are finite and non-negative. | hard-fail | REF-SED-CH11-SHEAR, REF-SED-CH10-HYDRAULICS, REF-SED-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-006 | Transport-capacity invariant: transport capacity follows Eq. [11.2.8], and sandy adjustment in Eq. [11.2.9] preserves `tcadjf >= 0.30` when applicable. | hard-fail | REF-SED-CH11-TC | `[DIRECT][Static]` |
| INV-SED-007 | Normalization invariant: normalized parameters (`η`, `τcn`, `θ`, `φ`) are derived by Eq. [11.3.7]-[11.3.11] with finite denominators and consistent adjusted soil parameters from Chapter 7. | hard-fail | REF-SED-CH11-NORM, REF-SED-CH7-EROD, REF-SED-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-008 | Downslope-variability invariant: OFE-strip routing with runon uses Eq. [11.4.1]-[11.4.6] branch semantics and preserves explicit sediment/water boundary conditions between strips. | hard-fail | REF-SED-CH11-DOWNVAR | `[DIRECT][Static]` |
| INV-SED-009 | Enrichment mass-conservation invariant: class-wise outgoing sediment from deposition transitions cannot exceed incoming-plus-local-contribution mass after correction steps. | hard-fail | REF-SED-CH11-ENRICH, REF-SED-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-010 | Coupling payload invariant: hillslope erosion event outputs (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i`) are emitted with units/sign conventions required by watershed/channel consumers. | hard-fail | REF-SED-CH13-COUPLING, REF-SED-HBP-FORMAT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-011 | Governance-scope invariant: Chapter-11 simplifications (steady-state transposition, fitted transport-capacity adjustments, enrichment procedure caveats) must remain explicit; unlabeled scope over-claims block promotion. | governance-fail | REF-SED-CH11-INTRO, REF-SED-CH11-TC, REF-SED-CH11-ENRICH | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-012 | MOFE01 M-G downstream erosion `qin`/sediment-handoff invariant: for downstream OFEs on active Wave-2 paths, accepted `erod14_qin` must be sourced from the prior OFE erosion `qout` and paired with the sediment/class-fraction handoff defined by legacy `xinflo`/`route` lineage. Public water-balance rows, aggregate runoff, `UpStrmQ`, `SubRIn`, or hourly water carry arrays are necessary water operands but are not sufficient sediment-coupling proof. OFE-1 and single-OFE zero-`qin` cases require explicit zero-upstream provenance. Water-transfer-only `erod14_qin` seeding must be labeled compatibility/follow-on scope and must not publish `erod14_qin_sediment_coupled = true`. DISPOSITION (E.3 / Increment 2c): the Wave-1 hourly handoff (`INV-SED-016` (b)) supplies exactly this lineage — prior-lane erosion `qout`/`qsout` plus the class-fraction handoff — so multi-OFE runs on the Wave-1 chain SATISFY this invariant and truthfully publish `erod14_qin_sediment_coupled = true` with `erod14_qin_source_policy = wave1-hourly-sediment-coupled-handoff`; the hold continues to bind any future non-Wave-1 path. | governance-hold | REF-SED-CH11-DOWNVAR, REF-SED-LEGACY-MOFE-QIN, INV-SED-008, INV-SED-009, INV-SED-010, SC-RUNOFFPART-001#INV-RUNOFFPART-030, SC-WATBAL-001#INV-WATBAL-099, SC-SYSTEM-001#INV-SYSTEM-032 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-013 | Hydrograph-resolved solve-basis invariant (ADR-0036 D1): when the hydrograph-resolved form is the publication authority, the Wave-1 continuity solve must run per **hydraulically-active hour** (`w_h > 0 ∨ qin_h > 0`) on the hourly water-shape authority selected by surface-water ownership. Default/off and pre-active runs use `REF-SED-DC01-SHAPE`; when Lane D routing owns the surface-water path, the erosion substrate MUST use `REF-SED-LANED-ROUTED-HYDROGRAPH` and MUST NOT silently fall back to DC01 source weights. For positive runoff, the selected shape is finite, non-negative, and unit-normalized before use; for no-runoff days it is all-zero. Missing, malformed, or non-closing active-routed-water shapes are typed hard failures. The hour operands remain: hour depth `q_runoff_m · w_h`, hour mean depth-rate `q_runoff_m · w_h / 3600 s` in the peak-rate operand slot, `effdrn_h = 3600 s`, `effint`/`effdrr` from the hour's excess/rainfall intervals, `beta_h` per-hour (`0.5` rainfall hour / `1.0` otherwise), rill width grown sequentially in hour order with end-of-day persistence, and daily erodibility/consolidation/frost/cover operands shared across the day's hours. The legacy day-level `passby` event-size gate precedes hour activation. Excess-only activation is invalid (it skips the full-reinfiltration `qout_h = 0 / qin_h > 0` deposition hour). | hard-fail | REF-SED-ADR0036, REF-SED-DC01-SHAPE, REF-SED-LANED-ROUTED-HYDROGRAPH, REF-SED-LEGACY-CONTIN-ROUTE, INV-SED-001, INV-SED-011 | `[DIRECT][Static]` |
| INV-SED-014 | Hydrograph-resolved closure invariant (ADR-0036 D4): each active hour's solve must satisfy the existing in-solve conservation gates (INV-SED-001/INV-SED-010 machinery per hour); the published daily aggregates must be the hour sums (`tdet/tdep/exported = Σ_h`) to f64-rounding; the serialized hourly surfaces must satisfy the integral closures `Σ_h V_h = runvol` and `Σ_h S_h =` the day's exported sediment mass. WB16 `peakro` remains a separate analytical estimator: `max(V_h/3600) ≠ peakro` is not an error and no rescaling of the hourly profile toward `peakro` is permitted. | hard-fail | REF-SED-ADR0036, INV-SED-001, INV-SED-010, SC-WATBAL-001#INV-WATBAL-099 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SED-015 | Peak-form comparator-arm invariant (ADR-0036 D1/D5): after the hydrograph-resolved flip, the retained daily peak-based solve is a comparator/diagnostic arm only — it must never be the publication authority, its deltas against the hourly form are Investigation-tier (ADR-0017), and it is deleted at the end of the transition window. Publishing peak-form sediment as production output after the flip is an invariant violation. | governance-fail | REF-SED-ADR0036, INV-SED-011 | `[DIRECT][Static]` |
| INV-SED-016 | Multi-OFE Wave-1 chaining invariant (E.3 / Increment 2c): (a) each lane's operand seed derives from ITS OWN OFE's sliced soil/slope/management — the 5-class particle distribution per-OFE by construction (`prtcmp` per-element lineage; a hillslope-global particle-size override is rejected fail-closed, the legacy single-global `partsize.dat` `usr_partsize` being a flagged legacy-MOFE gap not to be inherited); (b) the inter-OFE handoff carries EROSION-lineage surfaces only — the prior lane's per-hour outflow discharge `qout_h`, per-hour exported sediment discharge `qsout_h = S_h / (fwidth · 3600)` (`REF-SED-LEGACY-SLOSS-QSOUT`), exiting class fractions, static end slopes, and solve-final shear/transport coefficient sets — never water-transfer substitutes; (c) all receiver-side derivations follow the legacy order: `strldn = qsout · rspace / (tcend · width)` on the RECEIVER's scale after its own hydraulics/transport (`param.for:243`), the boundary shears via the no-growth `sheart` basis at the inflow discharge on the PRIOR lane's slopes (`param.for:184-196`), and the Eq. [11.4.x] shear/transport coefficient-continuity rewrite (`param.for:249-390`, `INV-SED-008` family) behind the legacy `iplane > 1 ∧ qout > 0 ∧ qin > 0` guard with every documented singular guard preserved; (d) a locally-dry lane with positive upstream inflow still solves (deposition of routed load), and an hour without a rainfall-excess period is theta-suppressed (no interrill supply, `REF-SED-LEGACY-REID` basis); (e) the hillslope HBP EVENT is EXIT-scoped: the exit lane's hourly pair and per-class surfaces with CHAIN-AGGREGATED `tdet`/`tdep` (Σ across lanes, same day), so the `INV-SED-014` sediment closure holds in its telescoped chain form `Σ S_h(exit) = Σ_lanes(tdet − tdep)` under the legacy equal-field-width chain assumption; (f) a quantum refused by the FLUX-consistency diagnostic (the trapezoid-vs-RK4 discretization check, NOT the mass-balance law) contributes zero sediment with a surfaced `flux_refused_quanta` count — the `TOL-SED-005` telescoping mass gate remains hard-fail on every solved quantum; (g) the day toe concentration is a defined 0 when the local volume basis (`runoff · efflen`) or `peakro` is non-positive (inflow-only exit days; the exported mass remains fully published via `S_h`/`tdet`/`tdep`). EROD14/Wave-2 is retired as publication authority (comparator arm only until its deletion; publishing Wave-2 sediment as production output is an invariant violation, the `INV-SED-015` pattern). | hard-fail | REF-SED-CH11-DOWNVAR, REF-SED-LEGACY-MOFE-QIN, REF-SED-ADR0036, INV-SED-008, INV-SED-012, INV-SED-013, INV-SED-014 | `[DIRECT][Static]` |
| INV-SED-017 | Particle-class enrichment invariant (E.4 / Increment 3, `REF-SED-LEGACY-ENRICH`): (a) the flow composition `frcflw` is per-quantum solver state — initialized per `route.for:142-160` (upstream exit composition when inflow exists, local detached `frac` otherwise, zeroed without outflow), blended at every enrichment call point (do-10), re-proportioned through every deposition region (do-30 + the label-50 `sedmax` loop, BOUNDED at 64 iterations as a documented deviation — exceeding the bound is a typed error, never a spin), and summarized at the OFE end (`iendfg`); (b) the TOTAL routed load remains the mass authority — the class solve normalizes to it (do-40) and can never change `tdet`/`tdep`/exported mass or the `INV-SED-014` closures. The pinned baseline then raises every normalized class below the dimensionless `1e-15` `gend` floor before label 50; when those absolute floors inflate their sum above `ldbot`, openWEPP MUST renormalize the nonnegative floored vector back to `ldbot` before applying any `sedmax` cap. This bounded correction retains the baseline floor as a composition stabilizer but supersedes its trace-load defect: label 50 MUST redistribute only a nonnegative remaining mass and MUST NOT manufacture negative class mass from a negative shortfall; (c) every per-class mass and every published fraction is finite and nonnegative; the composition is unit-sum while flow exists (`TOL-SED-006`; the publication split keeps its own `TOL-SED-005` closure) and zero without outflow; (d) the directional law: a depositing region depletes fast-falling classes first (`φ_i` grows with fall velocity) — the exit composition on a depositing profile is finer than the detached composition and the SSA enrichment ratio exceeds the detached-composition ratio; a zero-deposition profile exits with EXACTLY the detached composition; (e) per-class operands are per-OFE (`tcf1` at the kt2 shear from that OFE's classes; mineralogy from that OFE's soil; the entry-gate §4a authority); (f) the interrill-delivery branch selects by the SCHEDULE-scoped parsed lanuse (WS1 tie-in, 2026-07-05): Cropland yearlies run the legacy `drinti` branch (`param.for:412-450` — the branch legacy production actually exercised, since legacy ran every landuse as `lanuse = 1`), Forest yearlies run the `lanuse != 1` branch (`intdr = 1`); a mixed-lanuse schedule fails closed. Non-cropland `fidel = frac` remains exact on the forest branch; cropland `fidel` from `drinti` is now live on masquerade managements. FLAGGED SCIENCE ITEM (identity-doc adjudication, not decided here): whether roughness-driven interrill delivery is universal physics — making legacy's non-cropland `intdr = 1` a symptom-partition of the unfinished non-cropland paths — is an open question for external authority; the port stays source-true per branch; (g) `enrato` is an INTERNAL published diagnostic (the direct-runtime publication operands) with no routing feedback; it is NOT serialized to the pass/HBP interchange — adding it there is a future additive schema extension, not an E.4 obligation. The published per-class exit surfaces (`sedcon` split, HBP `frcflw`, the E.3 intake `exit_fractions`) carry the ENRICHED composition. | hard-fail | REF-SED-LEGACY-ENRICH, REF-SED-CH11-DOWNVAR, INV-SED-008, INV-SED-013, INV-SED-014, INV-SED-016 | `[DIRECT][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-SED-001` | runtime | Sediment continuity assembler | Typed hard error on unit/sign/closure inconsistency | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-002` | runtime | Rill detachment branch evaluator | Typed hard error on threshold/branch violation | Tier-A gate | `[DIRECT][Static]` |
| `INV-SED-003` | runtime | Deposition branch evaluator | Typed hard error on invalid `q` domain or sign inversion | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-004` | runtime | Hydrologic input validator | Typed hard error on missing/inconsistent (`Pr`, `tr`, `Ie`, `te`) payload surfaces | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-005` | runtime | Shear partition/domain validator | Typed hard error on invalid partition ratio or non-finite `τf` surfaces | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-006` | runtime | Transport-capacity calculator | Typed hard error on invalid transport-capacity domain or sandy-adjustment floor violation | Tier-A gate | `[DIRECT][Static]` |
| `INV-SED-007` | runtime | Normalization parameter calculator | Typed hard error on invalid normalized-parameter derivation or denominator domain | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-008` | runtime | OFE-strip routing branch logic | Typed hard error on runon/branch-condition mismatch | Tier-B investigation gate | `[DIRECT][Static]` |
| `INV-SED-009` | runtime | Particle-class enrichment updater | Typed hard error on class-wise mass-conservation breach | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-010` | runtime | Hillslope-to-routing boundary payload validator | Typed hard error on missing/malformed sediment payload fields | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-011` | governance | Review/disposition/verification checklist | Promotion `HOLD` on unlabeled scope-limit or over-claim conditions | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SED-012` | runtime + governance | EROD14 downstream `qin`/prior-OFE sediment handoff validator plus manifest acceptance gate | Explicit `HOLD` when downstream `erod14_qin` is accepted from water-transfer operands alone, prior-OFE erosion `qout` or particle-fraction handoff lineage is absent, or manifests claim sediment-coupled `qin` closure while the source policy remains water-transfer-only | MOFE01 M-G erosion `qin`/sediment coupling gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow Chapter-11 WEPP notation. EROD11
ratifies Wave-0 erosion-lane boundary alias ownership for the required
cross-contract coupling surfaces while preserving canonical identity aliases
for not-yet-implemented erosion internals.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `G`, `Di`, `Df`, `Dc`, `Tc` | identity names | continuity and branch surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Kr`, `τc`, `τf`, `β`, `Vf`, `q` | identity names | detachment/deposition threshold and rate surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Pr`, `tr`, `Vt`, `Ie`, `te` | identity names | hydrologic erosion-input surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Q` (WB12 runoff coupling) | `HillslopeProductionFluxSymbol::Wb12RunoffQ -> Q` | runoff-depth coupling surface consumed by erosion forcing | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `peakro`, `watdur` | `HillslopeProductionStateSymbol::Wb16Peakro -> peakro`; `HillslopeProductionStateSymbol::Wb16Watdur -> watdur` | peak-runoff and runoff-duration forcing surfaces for erosion branches | `m^3 s^-1`, `s` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` | `HillslopeProductionStateSymbol::{Wb16MethodBranch,Wb16Tstar,Wb16Qpstar,Wb16Vstar}` | WB16 branch-traceability surfaces required by erosion observability guards | branch metadata + scalar continuity diagnostics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Kiadj`, `Kradj`, `τcadj` | identity names | soil-to-erosion coupling surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `σir`, `SDRRR`, `Fnozzle`, `Rs`, `w` | identity names | interrill and geometry adjustment surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `η`, `τcn`, `θ`, `φ` | identity names | normalized erosion parameter surfaces | dimensionless semantics preserved | `[DIRECT][Static]` |
| `total_detachment_kg`, `total_deposition_kg` | identity names | hillslope sediment totals to routing pass-file boundary | `kg` preserved | `[DIRECT][Static]` |
| `particle_class_count` | identity name | particle-class cardinality for serialized boundary vectors | count semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `sediment_concentration_kg_m3_i` | `sediment_concentration_kg_m3_{class:04}` | sediment class concentration boundary surfaces for pass serialization and routing intake | `kg m^-3` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `particle_diameter_m_i` | `particle_diameter_m_{class:04}` | sediment class particle-diameter boundary surfaces for pass serialization and routing intake | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `particle_flow_fraction_i` | `particle_flow_fraction_{class:04}` | sediment class flow-fraction boundary surfaces for pass serialization and routing intake | fraction semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ER` | identity name | enrichment-ratio boundary surface | fraction semantics preserved | `[DIRECT][Static]` |

## EROD11 Alias Ownership Register

| Boundary ID | Canonical symbols | Runtime alias surface | Producer ownership | Consumer ownership | Evidence |
|---|---|---|---|---|---|
| `EROD-BND-001` | `Q`, `peakro`, `watdur`, `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` | `HillslopeProductionFluxSymbol::Wb12RunoffQ`; `HillslopeProductionStateSymbol::{Wb16Peakro,Wb16Watdur,Wb16MethodBranch,Wb16Tstar,Wb16Qpstar,Wb16Vstar}` | `SC-RUNOFFPART-001` + `SC-WATBAL-001` via WB12/WB16 kernels | `SC-SED-001` (`INV-SED-004`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-002` | `fr`, `fi/fe`, `w`, `fs`, `ft`, `τf/τfe` | canonical identity boundary symbols (runtime projection owner deferred under erosion-physics `HOLD`) | `SC-HYDRAULICS-001` | `SC-SED-001` (`INV-SED-005`, `INV-SED-006`, `INV-SED-007`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-003` | `total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i` | `total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_{class:04}`, `particle_diameter_m_{class:04}`, `particle_flow_fraction_{class:04}` (hillslope export), plus contributor-prefixed routing aliases `hs{ID}_*` | `SC-SED-001` | `SC-ROUTE-001` (`INV-ROUTE-011`) | `[DIRECT][Static] + [INFERENCE][Static]` |

## EROD12 Cross-Domain Ownership and Guard Closure Addendum

| Cross-domain lane | Producer ownership | Consumer guard ownership | Closure posture | Evidence |
|---|---|---|---|---|
| Hydrology forcing intake (`Q`, `peakro`, `watdur`, `wb16_*`) | `SC-RUNOFFPART-001` (`INV-RUNOFFPART-009`, `INV-RUNOFFPART-011`) + `SC-WATBAL-001` (`INV-WATBAL-007`, `INV-WATBAL-016`) | `SC-SED-001` (`INV-SED-004`) | Canonical producer/consumer guard ownership is explicit for required Wave-0 forcing symbols. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Hydraulics shear/friction intake (`fr`, `fi/fe`, `w`, `fs`, `ft`, `τf/τfe`) | `SC-HYDRAULICS-001` (`INV-HYDRAULICS-009`..`011`) | `SC-SED-001` (`INV-SED-005`..`007`) | Guard ownership and failure posture are explicit with no remaining Wave-0 ownership ambiguity. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Sediment payload export to routing (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i`) | `SC-SED-001` (`INV-SED-010`) | `SC-ROUTE-001` (`INV-ROUTE-011`) | Cross-domain payload validation ownership is explicit for downstream routing intake. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| Shear-below-threshold state | `τf <= τc` with `Df = 0` and no rill detachment contribution. | Explicit threshold behavior in Eq. [11.2.3] narrative. | `[DIRECT][Static]` |
| Transport-at-capacity state | `G = Tc` giving zero net rill source/sink contribution from Eq. [11.2.2]/[11.2.4] branch terms. | Continuity-consistent branch boundary between detachment and deposition regimes. | `[DIRECT][Static] + [INFERENCE][Static]` |
| No-runoff erosion-inactive event | `Pr = 0` and/or `tr = 0` resulting in no active rill transport update for the event. | Hydrologic forcing can yield erosion-inactive events while preserving daily model continuity. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Rainfall-type switch in deposition coefficient | `β = 0.5` for raindrop-impact rill flow and `β = 1.0` for snowmelt/furrow-irrigation cases. | Explicit Chapter-11 deposition coefficient mode rule. | `[DIRECT][Static]` |
| Zero-sprinkler-impact day | `Fnozzle = 1.0` on non-sprinkler rainfall days by default. | Explicit default behavior for nozzle-impact factor in §11.3.3 discussion. | `[DIRECT][Static]` |

## Invalid States

- Missing required hydrologic erosion-driver surfaces (`Pr`, `tr`, `Ie`, `te`) when erosion computations are invoked. `[DIRECT][Static] + [INFERENCE][Static]`
- Non-finite or negative transport/shear/flow-domain terms where not physically permitted (`Tc`, `q`, `τf`, `Vf`, `Kr`). `[DIRECT][Static] + [INFERENCE][Static]`
- Rill-detachment branch executed when threshold conditions are not met (`τf <= τc`) without explicit zero-detachment handling. `[DIRECT][Static] + [INFERENCE][Static]`
- Deposition branch executed with zero/negative `q` denominator semantics in Eq. [11.2.4]. `[DIRECT][Static] + [INFERENCE][Static]`
- Particle-class enrichment update that violates class-wise mass conservation across deposition transition updates. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing or unit-inconsistent hillslope-to-routing sediment payload fields (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i`). `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-SED-P-001: Publish hillslope erosion continuity and branch surfaces using canonical Chapter-11 symbol semantics and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SED-P-002: Enforce explicit detachment/deposition branch predicates and threshold behavior before emitting event outputs. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SED-P-003: Propagate invariant violations as typed errors; do not silently clamp or default materially invalid erosion states. `[INFERENCE][Static]`
- OBL-SED-P-004: Emit routing-boundary sediment payload completeness (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i`) with unit/sign integrity. `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-SED-C-001: Hydraulics and runoff consumers supplying erosion drivers must preserve unit/sign semantics for shear/runoff/intensity fields consumed by Chapter-11 equations. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SED-C-002: Routing/channel consumers must reject malformed hillslope sediment payloads and propagate typed failures with invariant context. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SED-C-003: Soil/management consumers must preserve adjusted erodibility/critical-shear payload semantics (`Kiadj`, `Kradj`, `τcadj`) consumed by erosion normalization. `[DIRECT][Static] + [INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Continuity and branch-threshold semantics (`INV-SED-001/002/003`) | erosion branch assembly | Hard error on branch/sign/closure failure | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Hydrologic/shear/transport input semantics (`INV-SED-004/005/006`) | pre-solve boundary validation | Hard error on malformed domains or missing coupling inputs | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Normalization + downslope routing semantics (`INV-SED-007/008`) | normalized solver + OFE strip transitions | Hard error on invalid normalized states; investigation routing for complex OFE strip mismatches | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Enrichment and routing payload semantics (`INV-SED-009/010`) | class-fraction updater + export boundary | Hard error on class-mass inconsistency or payload incompleteness | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Scope and authority governance (`INV-SED-011`) | review/verification/promotion | Governance `HOLD` until limitations and caveats are explicitly carried forward | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity).

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-SED-001 | Continuity residual tolerance for `INV-SED-001` event accounting surfaces | `<= 1e-9` in `kg s^-1 m^-1`-equivalent residual space | Residual reporting is mandatory; tolerance is for comparator interpretation, not silent runtime clamping. | `[INFERENCE][Static]` |
| TOL-SED-002 | Non-negative-domain tolerance for transport/load magnitudes (`G`, `Tc`, `Di`) | lower bound `>= -1e-12` in declared units | Material negatives beyond tolerance are hard-fail invariant violations. | `[INFERENCE][Static]` |
| TOL-SED-003 | Shear-threshold comparator tolerance for `τf` vs `τc` branch boundary | `abs(τf - τc) <= 1e-9 Pa` treated as threshold boundary condition | Prevents numeric jitter from toggling branch semantics near threshold. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-SED-004 | Deposition denominator floor tolerance for `q` | `q >= 1e-12 m^2 s^-1` for deposition-branch evaluation | Values below floor are invalid for Eq. [11.2.4] denominator semantics. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-SED-005 | Class-fraction closure tolerance at the PUBLICATION split (the normalize-then-split surface: the per-class `sedcon` division by the validated fraction sum) | `abs(sum(sed_frac_i) - 1.0) <= 1e-9` | The publication split normalizes immediately before dividing, so its closure is division-rounding-tight. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-SED-006 | Flow-composition CORRUPTION envelope for the IN-ROUTE enrichment state (`INV-SED-017` (c), the final `frcflw` after the route's blend/re-proportion sequence) | `sum(frcflw_i) ∈ [0.5, 1.5]` while flow exists (or exactly 0 without outflow) | Legacy NEVER re-normalizes after a do-10 blend: when `rillod` floors at 0 (`enrich.for:134` — a transport-capacity-limited stretch gains less load than the interrill term) the blend sum legitimately exceeds 1 by PERCENT scale, and the legacy ER consumes that raw sum (`enrich.for` has no gate at all). Only a do-30 re-proportion normalizes exactly. The envelope is therefore a corruption sanity bound, not a closure law; the PUBLISHED per-class split re-normalizes at the publication boundary, preserving `TOL-SED-005`. (Supersedes the rev-47 `1e-6` bound, which encoded a stricter law than legacy has and false-failed real transport-limited profiles — caught by the G0 fixture at full-suite scope.) | `[DIRECT][Static] + [Ran]` |

## WB16 Hydrologic Peak/Duration Intake Addendum

### WB16 Required Hydrology Inputs

| Surface | Symbols |
|---|---|
| Peak-flow forcing inputs | `peakro`, `watdur`, `Q` |
| WB16 diagnostics payload | `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` |

### WB16 Coupling Rules

1. Sediment-kernel readiness requires finite/non-negative `peakro` and
   `watdur` for erosion forcing derived from Chapter-11 hydrologic inputs.
2. WB16 continuity constraint remains explicit:
   - `watdur = Q/peakro` (within tolerance).
3. Missing or malformed WB16 trace payload symbols are typed boundary failures
   for erosion observability/replay diagnostics.
4. Sediment consumers must not synthesize fallback peak-flow inputs when WB16
   peak/duration symbols are absent or invalid.

### WB16 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB16-PEAK-E-001` |
| Non-finite required symbol | `HKERNEL-WB16-PEAK-E-002` |
| Domain/closure violation | `HKERNEL-WB16-PEAK-E-003` |

## EROD13 Wave-1 Core Runtime Addendum

### Runtime Integration Point

1. Wave-1 core erosion runtime executes in the hillslope
   `closure_diagnostics` scheduler phase after WB16 peak-runoff state writeback
   completes. The dispatched kernel phase class remains
   `hydrology_peak_runoff`; EROD13 authority is enforced by explicit guard-code
   family and symbol set below.
2. Runtime activation is explicit:
   - `erod13_core_enabled = 1` enables Wave-1 erosion execution.
   - `erod13_core_enabled = 0` disables the erosion path for fixtures that do
     not yet seed Wave-1 sediment inputs.
3. When enabled, missing/non-finite/domain-invalid erosion inputs are typed
   hard failures; no fallback synthesis, silent defaults, or silent clamping
   is permitted. `[DIRECT][Static] + [INFERENCE][Static]`

### Wave-1 Runtime Symbols

| Surface family | Symbols | Runtime role |
|---|---|---|
| Activation + hydrology forcing | `erod13_core_enabled`, `Q`, `peakro`, `watdur`, `Ie`, `te` | Activation gate and Chapter-11 hydrologic forcing semantics (`INV-SED-004`). |
| Hydraulics/shear forcing | `fs`, `ft`, `taufe`, `q` | Shear-partition and deposition-denominator semantics (`INV-SED-003`, `INV-SED-005`). |
| Sediment branch forcing | `G`, `Di`, `beta`, `vf`, `dGdx` | Continuity + branch-state forcing for detachment/deposition evaluation (`INV-SED-001`..`003`). |
| Normalization forcing | `cntlen`, `kr`, `kradjf`, `tcadjf`, `shrsol`, `tcend`, `shcrit`, `detinr`, `effdrr`, `effdrn`, `veleff`, `pkro` | Legacy-authority normalized parameter derivation (`INV-SED-007`). |
| Transport-capacity forcing | `erod13_tc_k`, `erod13_tc_m` | Eq. [11.2.8]-style power-law transport-capacity coefficients (`INV-SED-006`). |
| Core outputs | `Dc`, `Tc`, `Df`, `eta`, `taucn`, `theta`, `phi` | Wave-1 core runtime outputs required for invariant checks and downstream Wave-2 entry. |

### Wave-1 Algorithm Specification

1. Validate activation + required forcing symbols:
   - when `erod13_core_enabled = 1`, all required symbols in the table above
     are mandatory and finite.
2. Validate hydrology-forcing continuity (`INV-SED-004`):
   - `Q > 0`, `peakro > 0`, `watdur > 0`, `Ie >= 0`, `te > 0`,
   - `abs(watdur - (Q/peakro)) <= TOL-SED-001`.
3. Compute shear partition (`INV-SED-005`):
   - require `ft > 0`, `0 <= fs <= ft`,
   - compute `tau_f = taufe * (fs/ft)`.
4. Compute normalized parameters (`INV-SED-007`) using pinned legacy `param.for`
   lineage:
   - `eta = cntlen * kr * kradjf * shrsol / tcend`,
   - `taucn = tcadjf * shcrit / shrsol`,
   - `theta = (cntlen * detinr / tcend) * (effdrr/effdrn)`,
   - `phi = beta * veleff / pkro`.
5. Compute transport capacity (`INV-SED-006`):
   - require `tcadjf >= 0.30`,
   - `Tc = tcadjf * erod13_tc_k * tau_f^(erod13_tc_m)`.
6. Compute detachment/deposition branch (`INV-SED-002`, `INV-SED-003`):
   - detachment branch (`tau_f > taucn` and `G < Tc`):
     `Dc = eta * (tau_f - taucn)`,
     `Df = Dc * ((Tc - G)/Tc)`.
   - deposition branch (`G > Tc`):
     require `q > 0`,
     `Df = -(beta * vf / q) * (G - Tc)`.
   - threshold/equilibrium branch:
     `Df = 0`.
7. Enforce continuity (`INV-SED-001`):
   - `abs(dGdx - (Df + Di)) <= TOL-SED-001`.

### EROD13 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol (Wave-1 enabled path) | `HKERNEL-EROD13-CORE-E-001` |
| Non-finite required symbol | `HKERNEL-EROD13-CORE-E-002` |
| Domain/closure violation | `HKERNEL-EROD13-CORE-E-003` |

### Wave-1 Contract-Derived Test Vectors

Minimum vectors required by EROD13 contract-derived tests:

1. Nominal detachment vector (`tau_f > taucn`, `G < Tc`) emits finite
   non-negative `Dc`, positive `Df`, and continuity residual within tolerance.
2. Threshold vector (`tau_f <= taucn`) emits `Df = 0`.
3. Deposition vector (`G > Tc`) emits negative `Df` with valid `q > 0`.
4. Domain guard vector: `tcadjf < 0.30` fails with
   `HKERNEL-EROD13-CORE-E-003`.
5. Missing-symbol vector fails with `HKERNEL-EROD13-CORE-E-001`.
6. Non-finite-symbol vector fails with `HKERNEL-EROD13-CORE-E-002`.
7. Continuity residual violation (`dGdx != Df + Di`) fails with
   `HKERNEL-EROD13-CORE-E-003`.

## EROD14 Wave-2 Multi-OFE and Enrichment Runtime Addendum

> **DELETED — E.3 stage 2e (2026-07-04).** The EROD14/Wave-2 runtime arm
> (kernel, seed projection, and plumbing) is removed from the codebase;
> the Wave-1 chain (`INV-SED-016`) is the sole multi-OFE erosion engine.
> This addendum is retained as the historical specification of the
> deleted arm. Manifest lineage: `erod14_wave2_enabled` (permanently
> `false`) and the `erod14_qin_*` policy surfaces remain;
> `erod14_wave2_kernel_status_seen` is REPLACED by
> `multi_ofe_wave1_chained` (the dead forever-false kernel-status field
> gave way to the informative chain flag). The `Erod14*` typed guard
> codes remain defined (error-code lineage) with no remaining raisers.

### Runtime Integration Point

1. Wave-2 executes in the hillslope `closure_diagnostics` scheduler phase
   after WB16 peak/runoff closure and after EROD13 Wave-1 core calculations
   when both lanes are enabled.
2. Runtime activation is explicit:
   - `erod14_wave2_enabled = 1` enables Wave-2 multi-OFE/enrichment logic.
   - `erod14_wave2_enabled = 0` disables the Wave-2 lane.
3. Production hillslope runner activation policy is explicit:
   - if `erod14_wave2_enabled` is explicitly supplied on the runtime surface,
     it must be finite and binary (`0|1`);
   - if it is absent, runner-owned intake projection must set it from aligned
     OFE topology authority: `1` when validated `nelem > 1`, else `0`.
4. Runner-owned Wave-2 ingress seeding policy is explicit for enabled paths:
   - required symbols must be present before `closure_diagnostics` execution,
   - symbol synthesis must be deterministic from parsed/runtime surfaces plus
     canonical seed families,
   - missing/non-finite/domain-invalid derivation inputs are typed hard-fail
     states (no silent fallback).
5. When enabled, missing/non-finite/domain-invalid Wave-2 symbols are typed
   hard failures; fallback synthesis, silent defaults, and silent domain
   masking are prohibited.

### Wave-2 Runtime Symbols

| Surface family | Symbols | Runtime role |
|---|---|---|
| Activation and case semantics | `erod14_wave2_enabled`, `erod14_case`, `erod14_Qj_minus_1`, `erod14_Vj`, `erod14_Qj`, `erod14_Fh`, `erod14_Fp` | Multi-OFE branch-classification and runon/runoff condition guards (`INV-SED-008`). |
| Geometry and load-transition surfaces | `erod14_xtop`, `erod14_xbot`, `erod14_xdetst`, `erod14_ldtop`, `erod14_ldbot`, `erod14_lddend`, `erod14_qout`, `erod14_qin`, `erod14_qostar`, `erod14_slplen` | Deposition-strip transition boundaries and normalized load scaling (`INV-SED-008`, `INV-SED-009`). |
| Coefficients and class families | `erod14_class_count`, `erod14_ktrato`, `erod14_ainftc`, `erod14_binftc`, `erod14_cinftc`, `erod14_beta`, `theta`, `erod14_fall_*`, `erod14_frcflw_*`, `erod14_frac_*`, `erod14_fidel_*`, `erod14_tcf1_*` | Class-wise enrichment/deposition update surfaces with pinned legacy-form branch semantics (`INV-SED-009`). |
| Class outputs and enrichment exports | `erod14_gend_*`, `erod14_sedmax_*`, `sed_frac_*`, `erod14_sumg`, `ER`, `erod14_ssa_soil`, `erod14_ssa_class_*` | Class-mass closure and enrichment ratio export surfaces. |

### Wave-2 Algorithm Specification

1. Validate multi-OFE case classification (`erod14_case`) using explicit
   case semantics:
   - Case 1: `Qj-1 = 0`, `Vj = 0`, `Qj = 0`
   - Case 2: `Qj-1 > 0`, `Vj > 0`, `Qj > 0`
   - Case 3: `Qj-1 > 0`, `Vj = 0`, `Fh - Fp > 0`, `Qj > 0`
   - Case 4: `Qj-1 > 0`, `Vj = 0`, `Fh - Fp <= 0`, `Qj = 0`
2. Compute class-wise deposition-transition predictions (`gend_i`) from
   legacy `enrich.for` lineage using `pkro`, per-class `phi`, and
   (`xtop`,`xbot`,`qostar`) ratio terms.
3. Scale class-wise loads to `ldbot`, enforce non-negative floors, and enforce
   class-wise maximum mass constraints:
   - `sedmax_i = gu_i + ftheta_i*(xbot-xtop)`,
   - iterative reproportioning for classes below `sedmax_i` until closure.
   - baseline `enrich.for` semantics are authoritative for the
     reproportion loop: when at least one class is clipped to `sedmax_i` and no
     class remains below `sedmax_i` (`ratbot = 0`), do not fail solely on
     `ratbot = 0`; re-enter the clipping pass and accept all-class `sedmax_i`
     saturation once no further clipping is required.
4. Enforce `INV-SED-009` class-mass conservation:
   - `gend_i <= sedmax_i` for all classes at convergence,
   - `sum(gend_i)` must remain finite and non-negative,
   - emitted `sed_frac_i` values are normalized from final class loads.
5. Compute enrichment ratio export:
   - `ER = (sum_i(sed_frac_i * ssa_class_i) / ssa_soil) + 0.005`.

### MOFE01 M-G Downstream `qin` Handoff Boundary

1. Downstream `erod14_qin` is a coupled erosion/sediment boundary, not a public
   WAT alias. Legacy `xinflo` carries prior-OFE erosion `qout` into current-OFE
   `qin`, and legacy `route` carries prior-OFE particle fractions when flow
   enters.
2. For OFE `i > 1`, accepted Wave-2 sediment coupling must prove prior-OFE
   erosion `qout` and incoming class-fraction lineage in addition to any
   water-transfer closure operands.
3. Water-balance transfer closure (`UpStrmQ`, `SubRIn`, `TransferInput`,
   `TransferOutput`, hourly carry arrays, aggregate `Q`, `QOFE`, or
   `wb12_runoff_carryover`) is required water evidence but cannot by itself
   close `INV-SED-012`.
4. Runtime water-transfer-only seeding may remain as compatibility continuity
   only when manifests expose `erod14_qin_source_policy =
   "water-transfer-only-mofe01-mg-sediment-coupling-follow-on"` and
   `erod14_qin_sediment_coupled = false`.
5. The follow-on sediment coupling package must add vectors where downstream
   `qin` is checked against a prior-OFE erosion `qout` source that is not built
   from the same public WAT row used for validation.

### EROD14 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol (Wave-2 enabled path) | `HKERNEL-EROD14-WAVE2-E-001` |
| Non-finite required symbol | `HKERNEL-EROD14-WAVE2-E-002` |
| Domain/closure violation | `HKERNEL-EROD14-WAVE2-E-003` |

### Wave-2 Contract-Derived Test Vectors

Minimum vectors required by EROD14 contract-derived tests:

1. Nominal multi-OFE vector emits finite `erod14_gend_*`, normalized
   `sed_frac_*`, and finite `ER`.
2. Case-four branch vector (`Fh - Fp <= 0`) retains explicit case-four closure
   with zero downstream runoff indicator (`Qj = 0`).
3. Missing-symbol vector fails with `HKERNEL-EROD14-WAVE2-E-001`.
4. Non-finite-symbol vector fails with `HKERNEL-EROD14-WAVE2-E-002`.
5. Case-classification mismatch vector fails with
   `HKERNEL-EROD14-WAVE2-E-003`.
6. Class-conservation violation vector (no feasible reproportion closure)
   re-enters clipping when `ratbot = 0` under all-class `sedmax_i` saturation
   and completes without typed domain failure.
7. Class-fraction normalization violation vector fails with
   `HKERNEL-EROD14-WAVE2-E-003`.

## EROD15 Wave-3 HBP Routing-Boundary Export Addendum

> **DELETED — E.3 stage 2e (2026-07-04).** The EROD15 projection served
> only the deleted Wave-2 arm; the Wave-1 publication projection
> (`INV-SED-010`/`INV-SED-016`) owns the HBP export surface. Retained as
> historical specification.

### Wave-3 Export Symbols and Alias Family

Wave-3 routing-boundary payload export is anchored to pass-serialization
authority in `REF-SED-HBP-FORMAT` and carries the following symbol family:

- hillslope export symbols:
  - `total_detachment_kg`
  - `total_deposition_kg`
  - `particle_class_count`
  - `sediment_concentration_kg_m3_{class:04}`
  - `particle_flow_fraction_{class:04}`
- watershed contributor aliases (routing intake family):
  - `hs{ID}_total_detachment_kg`
  - `hs{ID}_total_deposition_kg`
  - `hs{ID}_particle_class_count`
  - `hs{ID}_sediment_concentration_kg_m3_{class:04}`
  - `hs{ID}_particle_flow_fraction_{class:04}`

### Wave-3 Export Projection Rules

When `erod14_wave2_enabled = 1`, Wave-3 export projection is mandatory and
uses the following runtime mapping:

1. `total_detachment_kg = max(erod14_sumg, 0)`.
2. `total_deposition_kg = max(erod14_lddend, 0)`.
3. `particle_flow_fraction_{class:04} = sed_frac_{class:04}`.
4. `particle_class_count = erod14_class_count`.
5. `sediment_concentration_kg_m3_{class:04}` must be finite and
   non-negative:
   - when `erod14_qout > 0`, compute as `erod14_gend_{class:04} / erod14_qout`;
   - when `erod14_qout <= 0`, emit `0`.

No fallback synthesis from non-Wave-2 symbols is allowed.

### Wave-3 Guard Continuity

Wave-3 payload export preserves existing EROD14 guard-family continuity:

- missing required symbol: `HKERNEL-EROD14-WAVE2-E-001`
- non-finite required symbol: `HKERNEL-EROD14-WAVE2-E-002`
- domain/closure violation: `HKERNEL-EROD14-WAVE2-E-003`

Missing/non-finite/domain-invalid payload surfaces are hard-fail states under
`INV-SED-010`; silent defaults, silent clamping, and consumer-side repair are
prohibited.

### Wave-3 Contract-Derived Test Vectors

Minimum vectors required by EROD15 contract-derived tests:

1. Nominal Wave-3 vector emits finite/non-negative
   `total_detachment_kg`/`total_deposition_kg`, finite
   `sediment_concentration_kg_m3_{class:04}`, and normalized
   `particle_flow_fraction_{class:04}`.
2. Zero-outflow vector (`erod14_qout <= 0`) emits zero concentration and zero
   particle-flow-fraction payloads for all classes.
3. Missing payload symbol vector fails with `HKERNEL-EROD14-WAVE2-E-001`.
4. Non-finite payload symbol vector fails with `HKERNEL-EROD14-WAVE2-E-002`.
5. Domain-invalid payload symbol vector fails with
   `HKERNEL-EROD14-WAVE2-E-003`.

## EROD16 Hillslope ROUTE Branch Authority Addendum

### Authoritative Routine Chain and Ownership

1. Baseline hillslope sediment routing authority is the `CONTIN -> ROUTE`
   call chain (`REF-SED-LEGACY-CONTIN-ROUTE`), not watershed WS10 channel
   routing branches.
2. `route.for` branch companions are:
   - `xcrit.for` (`mshear` classification),
   - `depc.for` and `depend.for` (deposition start/end solution terms),
   - `depos.for` (segment deposition profile updates),
   - `erod.for` (detachment-branch integration),
   - `enrich.for` (class-fraction updates and OFE-end finalization).
3. `rtpart.for` is explicitly out of routing scope (`REF-SED-LEGACY-RTPART`);
   it belongs to plant-growth/root partitioning lineage and must not be cited
   as a sediment-routing companion.

### ROUTE Branch Invariants (Canonical)

1. Segment-loop invariant: route branch execution iterates segmentwise over
   `k = 2..nslpts(iplane)` with explicit case-4 flow-end guards before
   branch dispatch.
2. Upper-boundary deposition invariant: when upper-segment `du < 0`,
   routing must execute `depc -> depend -> depos` semantics before any
   detachment-after-deposition branch calls.
3. `mshear` dispatch invariant: route branch selection is explicit for cases
   `1..5` from `xcrit`, with no branch collapsing or silent fallback between
   case families.
4. Post-detachment deposition invariant: when `ndep != 0`, deposition follow-up
   from `xdbeg` to segment end must execute before advancing to the next
   segment.
5. OFE-end enrichment invariant: terminal `enrich(..., iendfg=1)` semantics are
   required at OFE end even when no additional deposition region is opened in
   the final segment.
6. Profile-station normalization invariant: normalized route geometry follows
   `profil.for` by setting the normalization length to the terminal input
   station and deriving every `xstar = xinput / xinput_terminal`. Therefore a
   parser-accepted compatibility profile whose terminal station is near, but
   not numerically equal to, its declared physical hillslope length still has
   an exact normalized toe of `1.0`. The declared physical length remains the
   dimensional hillslope-length surface; it is not the `xstar` denominator,
   and no downstream clamp or relaxed toe guard may substitute for this
   derivation. `[DIRECT][Static]`

### Alias Continuity Requirements

Canonical symbol continuity for route migration in this contract must preserve:

- segment geometry/state: `xu`, `xl`, `nslpts`, `xdbeg`, `xdend`, `xdetst`,
  `ldlast`, `lddend`
- branch controls: `du`, `dl`, `mshear`, `ndep`, `xc1`, `xc2`
- deposition/transport controls: `ktrato`, `qostar`, `ainftc`, `binftc`,
  `cinftc`, `phi`, `theta`

If runtime symbol names differ, explicit alias mappings are required in
canonical `SC-*` contracts before production migration packages.

The profile-station normalization invariant is enforced in the Wave-1 slope
segment derivation with typed domain failures for missing, non-finite,
non-positive, or non-increasing station geometry. Its contract-derived vector
must include a parser-compatible terminal station below the declared physical
length and must prove a normalized toe of `1.0` within floating-point
tolerance.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-SED-001 | Per-invariant comparator vectors for sediment-branch transitions and class-wise enrichment closures remain uncurated, and this residual automation limitation is explicitly risk-accepted for current governance progression. | Automated per-invariant acceptance remains limited; manual comparator interpretation is required where those vectors are absent. | closed | `[DIRECT][Static]` |
| GAP-SED-002 | Wave-0 erosion-lane alias-ownership ambiguity for required cross-contract boundary symbols is explicitly dispositioned by canonical EROD11 alias ownership registers. | Alias-ownership ambiguity closure is complete for required boundary symbols; production erosion physics remains separately `HOLD`-gated by non-promotable companion/process gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SED-003 | EROD12 ratifies cross-domain ownership/guard closure for required erosion-lane companion boundaries (`SC-HYDRAULICS-001`, `SC-ROUTE-001`) using canonical `SC-*` addenda and row-scoped guard ownership mapping. | Required Wave-0 cross-domain ownership semantics are canonicalized; erosion production implementation remains separately gated by `EROD13+` and non-Wave-0 companion gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SED-004 | Chapter-11 enrichment caveats for mixed-soil, multi-OFE composition effects remain and are explicitly retained as a documented limitation with governance risk acceptance. | Mixed-soil enrichment interpretation may still require manual investigation; this is accepted as an explicit model-governance caveat. | closed | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SED-005 | Baseline `route.for` segment-level branch family (`mshear 1..5`, upper-end deposition/detachment trees, post-detachment deposition closure) was migrated into openWEPP runtime kernels by EROD19 and revalidated by EROD21 parity rerun/hold-lift disposition. | Hillslope sediment-routing process parity closure is now recorded; ongoing comparator monitoring continues under EROD21 evidence artifacts. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SED-006 | WSHEDIMPL38 closed the residual companion watershed channel sediment seam for `chnero/chnrt/detach` integration by retiring unresolved-detachment diagnostics publication (`ws20_detachment_unmigrated_segment_count`, `ws21_detach_unmigrated_segment_count`) and replacing residual invalid-segment fallback continuation in WS20/WS21 process lanes with typed fail-closed guard behavior (`ws20_case12_next_flux_{class:04}`, `ws21_case3_next_flux_{class:04}`, `ws21_case4_next_flux_{class:04}`). | Companion watershed sediment integration now relies on explicit typed guard failure for domain violations and no longer carries unresolved-detachment surrogate counters in production publication surfaces. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-SED-007 | Pre-enrichment per-class publication basis (E.1/Increment 1c-fidelity): the single-OFE Wave-1 per-class exit concentration publishes the `REF-SED-LEGACY-SLOSS-SEDCON` composition split using the `prtcmp` detached fractions as `frcflw` (the `REF-SED-LEGACY-FRCFLW-INIT` initialization). On the enabled scope (single-OFE, zero inflow, non-cropland) this is legacy-exact whenever the profile does not deposit; on depositing days the class distribution is the un-enriched composition (legacy `enrich` would re-proportion toward fines). The class SUM — the mass surface watershed consumers reconstruct — equals the scalar toe concentration to f64 rounding on all days (the composition is gated at the `TOL-SED-005` closure tolerance and the split is normalized by the validated sum). | Class-resolved consumers (enrichment-ratio diagnostics, particle handoff) must not treat the depositing-day distribution as enriched; total-mass consumers are unaffected. E.3 extension: on inflow days the exiting composition is the mass-weighted blend of the upstream flow composition and the local detached composition (the `enrich.for:205-213` terminal blend with non-cropland `fidel = frac`) under proportional depletion — exact on non-depositing receiving OFEs, un-enriched approximation otherwise. **CLOSED (E.4 / Increment 3, `INV-SED-017`): the full `enrich.for` port supersedes both the detached-composition split and the E.3 blend — the published composition is the enriched flow composition on all profiles; the D4 blend survives only as the None-enrichment fallback on non-production quanta.** | closed | `[DIRECT][Static]` |
| GAP-SED-008 | Uniform per-event class-fraction timing (ADR-0036 D2, E.2): the per-class resolution of the serialized hourly sediment mass `S_h` applies the event-level class fractions (`frcflw`, GAP-SED-007 basis) uniformly across the event's hours. NARROWED (E.4): each hour quantum now exits with its OWN enriched composition and the day-level published fractions are the export-mass-weighted blend across quanta — the within-event class dynamics exist in the solver. What remains open is only the INTERCHANGE surface: the serialized hourly `S_h` stays total-mass (no per-class-hourly channel), so hour-resolved class-composition consumers still see the day-level blend; the per-class-hourly channel remains a future additive interchange extension. | Hour-resolved class-composition consumers must not treat the uniform split as enriched timing; hour-resolved total mass (`S_h`) and event-level class mass are unaffected. Superseded with E.4. | open | `[DIRECT][Static]` |
| GAP-SED-009 | Absolute erosion magnitude (E.5 adjudication, 2026-07-05): the single-OFE p61 instrument over-detaches ~6× against legacy delivery at MATCHED per-event, per-width cut-points, and the p102 2-OFE instrument corroborates the same ~4–6× class at OUTLET-AGGREGATE detachment scope (order-of-magnitude evidence only — pass rows are outlet-scoped, so a per-day chain-export delivery comparison does not yet exist), ATTRIBUTED to the erosion ground-cover pathway — the erosion daily covers consume only the mass-derived residue-partition cover, while forest no-decomp ICs declare high `inrcov`/`rilcov` with near-zero seeded residue mass (legacy holds the declared covers; the declared authority already reaches the WB16 friction path and the frost depth seeding, but not the erosion cover operands). NOT water-driven (p102 50-year runoff volume within ~1% of legacy) and NOT structural (all E.1–E.4 conservation closures hold). | RE-JUDGED (2026-07-05, WP `20260705-erosion-ground-cover-authority-defect-closure-001`): the cover pathway is CLOSED (`SC-RESIDUE-001#INV-RESIDUE-020` — pools seeded from the declared IC covers per `init1.for`, covers re-derived daily per `covcal.for`; legacy resolved as recomputed-from-pools). Post-fix: p61 dominant event 3.97 vs legacy 4.2 kg/m (within ~6%, from ~6× over, consistent with the 0.73× water cut); p102 outlet detachment 17.4 vs legacy ~19.4 kg/m/yr (from ~84). Residual gap scope: within-band deviations ride the water judgment; the small-event divergence RE-ATTRIBUTED — the day gate is legacy-exact (0.010 m / 2.78e-6 m/s AND-semantics, `contin.for:970-973`) and the divergence is the WB16 `peakro` operand on trace events (water-side Investigation flag, bounded). Full record: the closure artifact in the 20260705 WP. | closed | `[DIRECT][Static] + [Ran]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-07-13` | `55` | `Codex` | INTVAL trace-load enrichment correction: `INV-SED-017` now requires the pinned do-40 `1e-15` class floor to be renormalized back to the routed `ldbot` mass authority before label-50 caps, forbids negative shortfall redistribution and negative class mass, and records this as a bounded correction of the pinned baseline's degenerate absolute-floor behavior. |
| `2026-07-13` | `54` | `Codex` | INTVAL terminal-station normalization amendment: added pinned `profil.for` authority and made the EROD16 Wave-1 `xstar` denominator the actual terminal profile station, preserving declared physical length while requiring a normalized toe of `1.0` for parser-accepted compatibility endpoints. |
| `2026-07-06` | `53` | `Codex` | D13 routed-hydrograph erosion-shape amendment: `REF-SED-DC01-SHAPE` is narrowed to default/off authority; new `REF-SED-LANED-ROUTED-HYDROGRAPH` binds active-routed-water Wave-1 erosion to finite non-negative unit-normalized routed hydrograph weights and typed fail-closed behavior for missing/malformed shapes; `INV-SED-013` now selects the hourly water shape by surface-water ownership without changing the `INV-SED-014` integral closures or authorizing production/default activation. |
| `2026-07-05` | `52` | `Claude Code` | Forest-lanuse sediment tie-in: `is_cropland` resolves from the schedule-scoped parsed lanuse (Cropland ⇒ the legacy `drinti` interrill branch now LIVE on masquerade managements; Forest ⇒ `intdr = 1`; mixed schedules fail closed) — `INV-SED-017` (f) rewritten; the roughness-delivery universality question flagged as a science item. Evidence: p61 3.90 (was 3.965) and p102 14.7 kg/m/yr (was 17.4) — both in the water-cut judgment band; first native-forest sediment proof (HJ Andrews `ow-lanuse-1` fixture: detachment + intake closure). |
| `2026-07-05` | `51` | `Claude Code` | `TOL-SED-006` corrected to the legacy-faithful CORRUPTION envelope (`[0.5, 1.5]`): legacy never re-normalizes after do-10 blends and its ER consumes the raw sum — the rev-47 `1e-6` bound encoded a stricter law than legacy has and false-failed transport-limited profiles (caught by the G0 fixture once the ground-cover fix activated real enrichment paths). The published split re-normalizes at the publication boundary (`TOL-SED-005` preserved); the enriched-override publication gate uses the same envelope. |
| `2026-07-05` | `50` | `Claude Code` | GAP-SED-009 CLOSED (the ground-cover authority defect-closure WP): erosion covers now derive from ground pools seeded per `init1.for` from the declared IC covers (`SC-RESIDUE-001` rev 12 `INV-RESIDUE-020`); post-fix p61 3.97 vs 4.2 kg/m, p102 17.4 vs ~19.4 kg/m/yr; the small-event divergence re-attributed to the WB16 `peakro` operand (the passby gate is legacy-exact). |
| `2026-07-05` | `49` | `Claude Code` | E.5 Codex round-1 evidence-precision fixes: `GAP-SED-009` now distinguishes p61 (matched per-event, per-width delivery cut-points) from p102 (outlet-aggregate corroboration, order-of-magnitude only — no per-day chain-export series exists yet); evidence label normalized to `[Ran]`. |
| `2026-07-05` | `48` | `Claude Code` | E.5 magnitude adjudication: opened `GAP-SED-009` — absolute magnitude OPEN-BUT-ATTRIBUTED (the erosion ground-cover pathway ignores the management-declared IC covers on forest no-decomp scenarios → ~4–6× over-detachment on both comparator instruments; water near-parity at p102; all structural closures hold). Verdict + follow-on scope in the Increment-4 adjudication artifact. |
| `2026-07-05` | `47` | `Claude Code` | E.4 Codex round-1 alignment: split the class-fraction tolerance into its two surfaces — `TOL-SED-005` scoped to the publication normalize-then-split closure (1e-9 unchanged) and new `TOL-SED-006` for the in-route enrichment unit-sum (1e-6, the floored-`rillod` blend seam rationale; legacy has no gate at all there), with `INV-SED-017` (c) naming both; the `ER` catalog row and `INV-SED-017` (g) now state the ratio is an INTERNAL publication surface not yet serialized to the pass/HBP interchange. |
| `2026-07-05` | `46` | `Claude Code` | E.4/Increment-3 enrichment amendment: added `REF-SED-LEGACY-ENRICH` (the `enrich.for` + `route.for` call-point authority) and `INV-SED-017` (per-quantum `frcflw` state; total-load mass authority preserved; unit-sum; the directional fining law + zero-deposition identity; per-OFE class operands; non-cropland `fidel = frac` exactness; `enrato` published diagnostic). `GAP-SED-007` CLOSED (enriched composition supersedes the detached split and the E.3 blend); `GAP-SED-008` narrowed to the interchange surface only (per-hour compositions exist in the solver; the serialized `S_h` stays total-mass). |
| `2026-07-04` | `45` | `Claude Code` | E.3 stage 2e: the EROD14/Wave-2 arm DELETED from the runtime (kernel `compute_direct_erod14`, EROD15 projection, MOFE03 Wave-2 seed projection, qin clamp + water-transfer-only policy plumbing, the dead downstream qout/fraction handoff channel); the EROD14/EROD15 addenda are marked deleted-historical. Manifest: `erod14_wave2_kernel_status_seen` replaced by `multi_ofe_wave1_chained`; `erod14_wave2_enabled` (false) + `erod14_qin_*` retained for lineage. `INV-SED-015`/`INV-SED-016` comparator-arm deletion clauses executed. |
| `2026-07-04` | `44` | `Claude Code` | E.3/Increment-2c multi-OFE chaining amendment: added `INV-SED-016` (per-lane per-OFE seeds with the fail-closed global-particle-override rule; the EROSION-lineage hourly handoff; receiver-side `strldn`/`sheart`/Eq.[11.4.x]-continuity derivations; inflow-active locally-dry solves + no-excess-hour theta suppression; the EXIT-scoped chain EVENT with `Σ S_h(exit) = Σ_lanes(tdet − tdep)`; the `flux_refused_quanta` diagnostic-skip policy; the inflow-only-day concentration guard; EROD14 retirement to a comparator arm), recorded the `INV-SED-012` DISPOSITION (satisfied by the Wave-1 handoff lineage; `erod14_qin_sediment_coupled = true` truthful on the chain), and extended `GAP-SED-007` with the labeled inflow-day blend. |
| `2026-07-04` | `43` | `Claude Code` | E.2/ADR-0036 hydrograph-substrate amendment: added `REF-SED-ADR0036` + `REF-SED-DC01-SHAPE` (the DC01 transfer-weight hourly-shape authority), `INV-SED-013` (per-hydraulically-active-hour solve basis + operand table + day-level `passby` precedence), `INV-SED-014` (per-hour and integral closures `Σ V_h = runvol` / `Σ S_h = exported mass`; no `peakro` rescale), `INV-SED-015` (peak-form arm comparator-only after the flip), and `GAP-SED-008` (uniform per-event class-fraction timing across hours pending E.4). |
| `2026-07-04` | `42` | `Claude Code` | E.1/Increment-1c-fidelity amendment: added `REF-SED-LEGACY-SLOSS-SEDCON` / `REF-SED-LEGACY-SEDSEG-WIDTH` / `REF-SED-LEGACY-FRCFLW-INIT` provenance anchors (per-class exit-concentration formula, `fwidth` total-mass scaling, exiting-fraction lineage) and opened `GAP-SED-007` labeling the pre-enrichment per-class publication basis (detached-composition split; exact on the non-depositing enabled scope, un-enriched distribution on depositing days, class sum always mass-exact) pending the E.4/2d `enrich.for`-lineage port. |
| `2026-06-14` | `41` | `Codex` | MOFE01 M-G amendment: added `INV-SED-012`, legacy `xinflo`/`route` provenance, and downstream `qin`/particle-fraction handoff boundary authority separating water-transfer-only seeding from accepted sediment-coupled closure. |
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-13 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-11 authority anchors, erosion invariants, guard map, symbol alias map, obligations, tolerances, and gap register for SCI-13 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: normalized evidence-mode casing, corrected `Di` non-negative continuity language, added `ER` alias coverage, added per-row evidence tags in degenerate states, and narrowed companion-gap wording. |
| `2026-05-23` | `3` | `Codex` | WB16 amendment: added hydrologic WB16 peak/duration intake authority (`peakro`, `watdur`) with continuity, traceability, and typed guard requirements for erosion coupling readiness. |
| `2026-05-23` | `4` | `Codex` | EROD11 amendment: ratified Wave-0 alias ownership across runoff/peak-duration and sediment handoff boundaries, added explicit cross-contract boundary ownership register, and downgraded `GAP-SED-002` from non-promotable to promotable-with-risk pending `EROD13+` internal alias expansion. |
| `2026-05-23` | `5` | `Codex` | EROD11 closure amendment: dispositioned alias-ownership ambiguity row `GAP-SED-002` to `closed` for required boundary symbols and made explicit that erosion-physics implementation remains separately governed by non-promotable holds. |
| `2026-05-23` | `6` | `Codex` | EROD11 risk-acceptance amendment: dispositioned `GAP-SED-001` and `GAP-SED-004` from promotable-with-risk to `closed` via explicit governance risk acceptance while preserving non-promotable erosion-physics HOLD posture. |
| `2026-05-23` | `7` | `Codex` | EROD12 amendment: added cross-domain ownership/guard closure addendum for required erosion-lane boundaries and dispositioned `GAP-SED-003` to `closed` while preserving non-Wave-0 implementation holds. |
| `2026-05-25` | `8` | `Codex` | EROD13 Wave-1 amendment: added pinned-baseline legacy authority anchors (`param.for`, `erod.for`, `runge.for`), runtime integration semantics, algorithm/guard specification, and contract-derived vector obligations for `INV-SED-001`..`007` core execution. |
| `2026-05-25` | `9` | `Codex` | EROD14 Wave-2 amendment: added multi-OFE case-classification/runtime authority and class-wise enrichment mass-conservation closure semantics (`INV-SED-008..009`) with typed guard-family continuity (`HKERNEL-EROD14-WAVE2-E-001..003`). |
| `2026-05-25` | `10` | `Codex` | EROD15 Wave-3 amendment: replaced generic sediment handoff naming with HBP pass-serialization field authority, added contributor-prefixed routing alias family, and added Wave-3 export mapping/guard continuity requirements for `INV-SED-010`. |
| `2026-05-25` | `11` | `Codex` | MOFE03 amendment: added production runner activation/seeding authority for `erod14_wave2_enabled` and enabled-path deterministic Wave-2 ingress synthesis from aligned topology/runtime surfaces with typed hard-fail derivation posture. |
| `2026-05-26` | `12` | `Codex` | EROD16 amendment: added canonical hillslope `CONTIN -> ROUTE` branch-authority mapping (`mshear 1..5`, `depc/depend/depos/erod/enrich` routine chain), codified route-branch invariants and alias continuity requirements, corrected `rtpart.for` provenance classification, and opened `GAP-SED-005` until runtime migration closure. |
| `2026-05-26` | `13` | `Codex` | EROD21 closure amendment: dispositioned `GAP-SED-005` to `closed` after EROD19 runtime migration + EROD21 parity rerun/hold-lift evidence, and updated impact language to reflect landed process parity. |
| `2026-05-27` | `14` | `Codex` | WSHEDIMPL01 amendment: normalized unresolved cross-domain watershed sediment closure dependency (`GAP-SED-006`) so `SC-SED-001` boundary payload authority remains explicit while companion channel sediment migration (`chnero/chnrt/detach`) remains non-promotable until WSHED06 closure evidence lands. |
| `2026-05-27` | `15` | `Codex` | WSHEDIMPL06 amendment: ratified WS11 channel sediment publication-family closure (`ws10_channel_{id}_qsed`, `ws10_channel_{id}_tc`) while preserving non-promotable `GAP-SED-006` posture for unresolved full `chnero/chnrt/detach` process-parity migration and validation closure. |
| `2026-05-27` | `16` | `Codex` | WSHEDIMPL15 amendment: ratified WS15 channel-sediment control projection and baseline conversion scaffold publication (`ws10_channel_{id}_{crsh,depmid,depsid}` with fail-closed guards) while preserving non-promotable `GAP-SED-006` posture pending full companion `chnero/chnrt/detach` process migration closure. |
| `2026-05-27` | `17` | `Codex` | WSHEDIMPL16 amendment: ratified contributor `particle_diameter_m` payload ingress projection (`hs{ID}_particle_diameter_m_{class:04}`) with fail-closed WS10 guard continuity, and narrowed `GAP-SED-006` to remaining full companion `chnero/chnrt/detach` process migration closure scope. |
| `2026-05-27` | `18` | `Codex` | WSHEDIMPL17 amendment: ratified WS17 segment/hydraulic scaffold projection/guard closure (`ws10_channel_{id}_nslpts` + segment `x/slope/depa/depb/wida/widb` families) and narrowed `GAP-SED-006` to remaining full companion `chnero/chnrt/detach` process-family migration scope. |
| `2026-05-27` | `19` | `Codex` | WSHEDIMPL18 amendment: migrated baseline `shield`/`trncap` transport-capacity authority into WS10 channel sediment publication (`tc`) using class-aware contributor payload aggregation and removed surrogate `tc=qsed` identity coupling, while preserving non-promotable `GAP-SED-006` posture for unresolved channel detachment/deposition segment-loop families (`case12/case34/detach/dcap/enddet`) and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `20` | `Codex` | WSHEDIMPL19 amendment: ratified fail-closed WS10 channel sediment branch payload export (`particle_class_count`, `particle_flow_fraction_{class:04}`, `particle_diameter_m_{class:04}`) and upstream channel-dependency payload ingress continuity for class-aware aggregation, while preserving non-promotable `GAP-SED-006` posture for unresolved channel detachment/deposition segment-loop families (`case12/case34/detach/dcap/enddet`) and full `chnero/chnrt` inflow-partition parity closure. |
| `2026-05-27` | `21` | `Codex` | WSHEDIMPL20 amendment: added opt-in WS20 segment-loop `case12` routing scaffolding with unresolved-detachment diagnostics publication and retained non-promotable `GAP-SED-006` posture for remaining baseline-authoritative channel detachment/deposition families (`case34/detach/dcap/enddet`) and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `22` | `Codex` | WSHEDIMPL21 amendment: added WS10 opt-in WS21 case34/enddet diagnostics scaffolding (`ws21_case3_segment_count`, `ws21_case4_segment_count`, `ws21_enddet_segment_count`, `ws21_detach_unmigrated_segment_count`) with explicit unresolved detach-capacity diagnostics publication, while retaining non-promotable `GAP-SED-006` posture for remaining baseline-authoritative `detach/dcap` migration and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `23` | `Codex` | WSHEDIMPL22 amendment: replaced WS21 opt-in unresolved fallback with baseline-lineage `dcap` + `case34/enddet` execution and required fail-closed `crfrac` projection gating (`ws10_channel_{id}_crfrac_{class:04}`), while retaining non-promotable `GAP-SED-006` posture for residual WS21 `case4 -> detach` iterative closure (`nt < cnpart`) and remaining full `chnero/chnrt` parity closure. |
| `2026-05-27` | `24` | `Codex` | WSHEDIMPL23 amendment: migrated baseline-authoritative `detach.for` iterative closure behavior for WS21 `case4` rows (`nt < cnpart`) and removed residual WS21 unresolved-detachment fallback requirement for that branch, while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `25` | `Codex` | WSHEDIMPL24 amendment: migrated baseline-authoritative `case12.for` deposition-to-detachment transition continuation (`xdemax < x(i)` into `detach.for`) and added explicit transition diagnostics publication (`ws24_case2_detach_segment_count`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `26` | `Codex` | WSHEDIMPL25 amendment: closed residual WS20 opt-in unresolved-detachment fallback behavior by auto-activating WS21 migration lanes under WS20 opt-in and enforcing fail-closed `crfrac` requirements for WS20-only opt-in lanes, while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `27` | `Codex` | WSHEDIMPL26 amendment: migrated baseline-authoritative `dcap(flagm=2)` max-detachment limiter semantics for WS23 iterative detach closure lanes and retained non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `28` | `Codex` | WSHEDIMPL27 amendment: migrated baseline-authoritative `enddet.for` bracket progression semantics (`xdbig/xdsmal`) for WS21 case4 enddet closure lanes and retained non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `29` | `Codex` | WSHEDIMPL28 amendment: migrated baseline-authoritative `chnrt.for` segment boundary-width semantics (`widb(i-1)` upper boundary, `wida(i)` lower boundary) in WS20 segment-loop routing lanes and retained non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `30` | `Codex` | WSHEDIMPL29 amendment: migrated rectangular-channel width-mutation semantics by projecting `dcap` eroded-width outcomes (`werb`) into WS20 `widb(i-1)` updates and state-symbol writeback (`ws10_channel_{id}_widb_{point:04}`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `31` | `Codex` | WSHEDIMPL30 amendment: migrated erodible-lane shape-transition continuity by activating `ishape=3` routing pathways and applying `depa/depb`-driven rectangular fallback mapping for WS20/WS21 hydraulic and detach-capacity calls, while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `32` | `Codex` | WSHEDIMPL31 amendment: migrated baseline-authoritative lower-boundary width-mutation continuity (`flagc=2`, `wera>wfl`) by projecting detach eroded-width outcomes (`wera`) into WS20 rectangular-lane `wida(i)` updates and state-symbol writeback (`ws10_channel_{id}_wida_{point:04}`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `33` | `Codex` | WSHEDIMPL32 amendment: reconciled parser/runtime naturally eroded shape-class lineage by aligning watershed channel parser projection and WS10 runtime consumption on explicit `ishape=3` mapping semantics (strict domain `1..=3`, compatibility `ishape>3 -> 3`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `34` | `Codex` | WSHEDIMPL33 amendment: reconciled parser/runtime channel `ienslp` lineage by aligning watershed channel parser projection and WS10 runtime seed validation on explicit `ienslp` domain semantics (`1..=2`, fail-closed out-of-domain), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `35` | `Codex` | WSHEDIMPL34 amendment: reconciled parser/runtime watershed-channel Manning relation lineage by aligning parser projection authority and WS10 runtime seed validation on explicit `chnn >= chnnbr` fail-closed semantics, while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `36` | `Codex` | WSHEDIMPL35 amendment: reconciled parser/runtime channel control lineage by projecting `icntrl`/`flgout` into WS10 runtime seed surfaces with explicit fail-closed domain semantics (`icntrl in [0,4]`, `flgout in [0,1]`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `37` | `Codex` | WSHEDIMPL36 amendment: reconciled parser/runtime rating-curve control lineage by projecting `ws10_channel_{id}_{rccoef,rcexp,rcoset}` for `icntrl==4` lanes into WS10 runtime seed surfaces with explicit fail-closed payload-shape/domain semantics (`rccoef>0`, `rcexp>0`, `rcoset>=0`), while retaining non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `38` | `Codex` | WSHEDIMPL37 amendment: added trace linkage for companion WS11 hydrology route-chain parity closure (`wshcqi/wshirs/wshrun`) and `GAP-ROUTE-008` disposition while preserving non-promotable `GAP-SED-006` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `39` | `Codex` | WSHEDIMPL38 amendment: closed `GAP-SED-006` by retiring unresolved-detachment diagnostics symbols and replacing residual WS20/WS21 invalid-segment fallback continuation with typed fail-closed domain guards in companion channel sediment process lanes. |
| `2026-05-28` | `40` | `Codex` | HILLSTAB04 amendment: aligned EROD14 Wave-2 reproportion closure semantics to baseline `enrich.for` for all-class `sedmax` saturation (`ratbot=0` clipping pass re-entry) and updated contract-derived vector obligations to prohibit non-authoritative hard-fail behavior on that branch. |
