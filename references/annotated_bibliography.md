# WB-33 Annotated Bibliography

Date: 2026-05-10 UTC  
Status: Phase-0B reference consolidation complete (docs-only)

openWEPP note (2026-05-11):
- This bibliography is tracked in git and maintained in this repository.
- Local file paths in entries point to `references/copyrighted/` for
  restricted/local-cache artifacts per `docs/governance/reference-vendoring-policy.md`.
- First-pass rights decisions are logged in
  `references/rights_classification_first_pass_2026-05-11.md`.

## R-01: Lighthill & Whitham (1955) kinematic waves

**Citation**: Lighthill, M. J., and G. B. Whitham (1955). *On Kinematic Waves. I. Flood Movement in Long Rivers*. Proceedings of the Royal Society A, 229, 281-316.  
**Local path**: `/workdir/openWEPP/references/copyrighted/Lighthill_Whitham_1955_Kinematic_Waves.md`  
**Reference quality**: `verified-primary`  
**Topic**: Foundational kinematic-wave theory, characteristics, and shock behavior for flood-wave propagation.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Wave celerity relation presented in Eq. (3) (`c` from flow-concentration slope) and shock-speed continuity relation in Eq. (7) in local extract.
- `[DIRECT]` Single-family characteristic propagation framing for kinematic systems.
- `[INFERENCE]` Supports route-kernel separation between propagation (`wbk_route_03`) and network composition (`wbk_route_04`).
**Kernel mapping**: `wbk_route_03_wave_propagation`, `wbk_route_04_cascade_composition` (`[INFERENCE]`).  
**Notes / caveats**: OCR text is imperfect but equation anchors `(3)` and `(7)` are present.  
**OAR-6 compliance status**: Not sufficient alone; requires implementation-form references (`R-04`, `R-08`, `R-09`).

## R-02: Henderson (1966) Open Channel Flow

**Citation**: Henderson, F. M. (1966). *Open Channel Flow*. Macmillan.  
**Local path**: `external-print-source`  
**Reference quality**: `external-print-source`  
**Topic**: Canonical open-channel hydraulics text used by later routing syntheses.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Secondary-primary bridge in `R-04` cites Henderson p. 365-367 for kinematic celerity specialization and wave framing.
- `[INFERENCE]` May remain bibliographic-only if WB-33 constants are fully derived from open primary references.
**Kernel mapping**: `wbk_route_01_channel_geometry`, `wbk_route_03_wave_propagation`.  
**Notes / caveats**: No local primary copy acquired; treated as paywalled/print source for this pass.  
**OAR-6 compliance status**: Companion-only; cannot be sole constant authority in current workspace.

## R-03: Chow (1959) Open-Channel Hydraulics

**Citation**: Chow, V. T. (1959). *Open-Channel Hydraulics*. McGraw-Hill.  
**Local path**: `external-print-source`  
**Reference quality**: `external-print-source`  
**Topic**: Canonical Manning/open-channel geometry and flow text.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Referenced bibliographically in `R-04` and used as standard Manning/open-channel anchor.
- `[INFERENCE]` Optional companion if geometry constants must trace to Chow directly rather than `R-04`/`R-16`.
**Kernel mapping**: `wbk_route_01_channel_geometry`, `wbk_route_03_wave_propagation`.  
**Notes / caveats**: No open local copy retained in this pass.  
**OAR-6 compliance status**: Companion-only under current evidence set.

## R-04: USGS Professional Paper 1302 (1986)

**Citation**: U.S. Geological Survey (1986). *Basic Concepts of Kinematic-Wave Models*. Professional Paper 1302.  
**Local path**: `/workdir/openWEPP/references/vendorable/USGS_PP1302_1986_Kinematic_Wave_Models.md`  
**Reference quality**: `verified-primary`  
**Distribution status**: `redistributable-first-pass` (USGS publication policy).  
**Topic**: Applied hydrologic synthesis of kinematic/dynamic approximations, celerity relations, and routing applicability.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Eq. (44) anchor in extract: `c = dQ/dA`.
- `[DIRECT]` Eq. (53) anchor in extract: Manning specialization (`c = 5/3 v`).
- `[DIRECT]` Characteristic-method framing and kinematic-shock context are documented.
**Kernel mapping**: `wbk_route_01_channel_geometry`, `wbk_route_03_wave_propagation`, `wbk_route_04_cascade_composition`.  
**Notes / caveats**: OCR quality varies; equation-number anchors are present in extracted text.  
**OAR-6 compliance status**: Strong primary anchor; still paired with method-specific references for implementation constants.

## R-05: Cunge (1969) Muskingum method paper

**Citation**: Cunge, J. A. (1969). "On the Subject of a Flood Propagation Computation Method (Muskingum Method)." *Journal of Hydraulic Research*, 7(2), 205-230. doi:10.1080/00221686909500264.  
**Local path**: `external-print-source`  
**Reference quality**: `external-print-source`  
**Topic**: Primary Muskingum-Cunge derivation lineage for numerical vs physical diffusion alignment.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Secondary equation lineage appears in `R-08` (HEC-HMS Muskingum-Cunge), including `K=Δx/c` and `X` relation.
- `[INFERENCE]` Route-γ can proceed with `R-08` + `R-04` while this remains bibliographic.
**Kernel mapping**: `wbk_route_03_wave_propagation`.  
**Notes / caveats**: DOI resolved; local full text not acquired in this pass.  
**OAR-6 compliance status**: Companion-only in current workspace.

## R-06: Ponce & Yevjevich (1978) variable-parameter Muskingum-Cunge

**Citation**: Ponce, V. M., and V. Yevjevich (1978). "Muskingum-Cunge method with variable parameters." *Journal of the Hydraulics Division, ASCE*, 104(HY12), 1663-1667 (online rendering extract).  
**Local path**: `/workdir/openWEPP/references/copyrighted/Ponce_Yevjevich_1978_Muskingum_Cunge.md`  
**Reference quality**: `verified-secondary`  
**Topic**: Variable-parameter Muskingum-Cunge forms and coefficient sensitivity.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Online rendering includes storage form and variable-parameter discussion with `K` and `X` lineage references.
- `[INFERENCE]` Use with `R-08` when selecting Muskingum-Cunge branch constants.
**Kernel mapping**: `wbk_route_03_wave_propagation`.  
**Notes / caveats**: Source is an online rendering, not a scanned archival ASCE PDF.  
**OAR-6 compliance status**: Companion reference; pair with `R-08`/`R-04`.

## R-07: Miller & Cunge (1975) simplified unsteady-flow equations

**Citation**: Miller, W. A., and J. A. Cunge (1975). "Simplified Equations of Unsteady Flow," in Mahmood and Yevjevich (eds.), *Unsteady Flow in Open Channels*.  
**Local path**: `external-print-source`  
**Reference quality**: `external-print-source`  
**Topic**: Convective-diffusion equation lineage used by Muskingum-Cunge implementations.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` `R-08` cites Miller & Cunge for convective-diffusion formulation.
- `[INFERENCE]` Bibliographic-only status is acceptable for Phase-1 if constants are derived from available primary implementation references.
**Kernel mapping**: `wbk_route_03_wave_propagation`.  
**Notes / caveats**: No open local full text acquired in this pass.  
**OAR-6 compliance status**: Companion-only in current evidence corpus.

## R-08: HEC-HMS Technical Reference (Muskingum-Cunge)

**Citation**: USACE HEC-HMS Technical Reference Manual. "Muskingum-Cunge Model" web documentation page.  
**Local path**: `/workdir/openWEPP/references/copyrighted/HEC_HMS_TechRef_Muskingum_Cunge.html`  
**Reference quality**: `verified-primary`  
**Topic**: Practical implementation equations for Muskingum-Cunge routing and stability controls.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Eq. (4) celerity: `c = dQ/dA`.
- `[DIRECT]` Eq. (7) recursive routing with `C1..C4` coefficients.
- `[DIRECT]` Eq. (12) `K=Δx/c`; Eq. (13) `X=0.5(1 - Q/(B S_o c Δx))`.
**Kernel mapping**: `wbk_route_03_wave_propagation`.  
**Notes / caveats**: Web-export HTML includes equation numbering anchors and MathJax expressions.  
**OAR-6 compliance status**: Sufficient implementation anchor for Muskingum-Cunge branch when paired with theory lineage (`R-04`, `R-01`).

## R-09: HEC-HMS Technical Reference (Kinematic Wave)

**Citation**: USACE HEC-HMS Technical Reference Manual. "Kinematic Wave Channel Routing Model" web documentation page.  
**Local path**: `/workdir/openWEPP/references/copyrighted/HEC_HMS_TechRef_Kinematic_Wave.html`  
**Reference quality**: `verified-primary`  
**Topic**: Practical kinematic-wave assumptions and applicability limits for channel routing.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Eq. (1) simplification `S_f = S_0`.
- `[DIRECT]` Explicit applicability note: steep channels and no backwater recreation.
- `[INFERENCE]` Used as engineering-implementation constraint for route-γ kernel branch selection.
**Kernel mapping**: `wbk_route_03_wave_propagation`.  
**Notes / caveats**: Web-export HTML contains equation anchors and method-limit notes.  
**OAR-6 compliance status**: Sufficient for method constraints; pair with `R-01`/`R-04` for core theory.

## R-10: Goodrich et al. (2004) Walnut Gulch recharge comparison

**Citation**: Goodrich, D. C., et al. (2004). "Comparison of Methods to Estimate Ephemeral Channel Recharge, Walnut Gulch, San Pedro River Basin, Arizona." In *Groundwater Recharge in a Desert Environment* (AGU Water Science and Application 9), pp. 77-99.  
**Local path**: `/workdir/openWEPP/references/copyrighted/Goodrich_2004_Walnut_Gulch_Ephemeral_Channel.md`  
**Reference quality**: `verified-primary`  
**Topic**: Event-scale transmission-loss accounting and recharge estimation in ephemeral channels.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Eq. (1) reach balance: `R = Qi + Ql - Qo + P - E - T + ΔS`.
- `[DIRECT]` Emphasizes effective wetted perimeter + infiltration-rate calibration coupling for channel losses.
- `[INFERENCE]` Supports kernel split between transmission-loss mechanics and summary/accounting (`wbk_route_02`, `wbk_route_10`).
**Kernel mapping**: `wbk_route_02_transmission_loss`, `wbk_route_05_baseflow_contribution`, `wbk_route_10_summary_accumulator_kernel`.  
**Notes / caveats**: OCR extract quality is good; equation anchors present.  
**OAR-6 compliance status**: Strong primary anchor for water-balance/loss accounting but should be paired with Bouwer/Lane for mechanistic constants.

## R-11: Smith et al. KINEROS chapter (1990 extract / 1995 publication context)

**Citation**: Smith, R. E., D. C. Goodrich, D. A. Woolhiser, and C. L. Unkrich. "KINEROS - A Kinematic Runoff and Erosion Model," chapter in *Computer Models of Watershed Hydrology* (1995 publication context; local extract metadata includes 1990/1995 lineage).  
**Local path**: `/workdir/openWEPP/references/copyrighted/Smith_1990_Kineros.md`  
**Reference quality**: `verified-primary`  
**Topic**: Distributed event-routing formulation with interactive infiltration and channel coupling.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Eq. (20.11) `Q = α h^m`; Eq. (20.12) continuity.
- `[DIRECT]` Eq. (20.23) channel continuity with lateral inflow.
- `[DIRECT]` Eq. (20.3) infiltration capacity relation with `K_s` and capillary-drive terms.
- `[INFERENCE]` Supports explicit orchestration + inflow aggregation kernels as first-class call-graph components.
**Kernel mapping**: `wbk_route_02_transmission_loss`, `wbk_route_03_wave_propagation`, `wbk_route_04_cascade_composition`, `wbk_route_08_orchestrator_kernel`, `wbk_route_09_inflow_aggregation_kernel`.  
**Notes / caveats**: Date/edition reconciliation remains documented; local extract contains equation numbering anchors sufficient for Phase-1/2 tracing.  
**OAR-6 compliance status**: Sufficient companion for distributed routing/infiltration coupling when paired with `R-12`/`R-13`/`R-14`.

## R-12: Lane (1983) NEH Chapter 19 transmission losses

**Citation**: Lane, L. J. (1983). "Chapter 19: Transmission Losses," USDA SCS/NRCS National Engineering Handbook Part 630 (local chapter issue metadata: 2007 issuance of chapter text).  
**Local path**: `/workdir/openWEPP/references/copyrighted/Lane_1983_Transmission_losses_H_210_630_19.md`  
**Reference quality**: `verified-primary`  
**Topic**: Engineering equations for volume and peak reductions in ephemeral channels.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Eq. (19-1)/(19-2) outflow-volume threshold/intercept-slope form.
- `[DIRECT]` Eq. (19-3) peak-flow reduction relation.
- `[DIRECT]` Eq. (19-19), (19-13), (19-14), (19-15), (19-16) differential decay + parameter estimation forms.
**Kernel mapping**: `wbk_route_02_transmission_loss`, `wbk_route_03_wave_propagation` (routing-time coupling implications).  
**Notes / caveats**: Chapter is transmission-loss focused and not a full hydrograph-routing solver.  
**OAR-6 compliance status**: Strong primary anchor for channel-loss parameter families.

## R-13: Lane (1982) distributed semiarid watershed model

**Citation**: Lane, L. J. (1982). "Distributed Model for Small Semiarid Watersheds." *Journal of the Hydraulics Division, ASCE*, 108(HY10), 1114-1131.  
**Local path**: `/workdir/openWEPP/references/copyrighted/Lane_1982_Distributed_Model_of_Semiarid_Watersheds.md`  
**Reference quality**: `verified-primary`  
**Topic**: Distributed watershed routing with explicit transmission-loss channel component.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Eq. (12) outflow-volume relation; Eq. (14) loss differential form; Eq. (17)/(18) parameterized solution.
- `[DIRECT]` Eq. (23) routed-peak expression including loss terms.
- `[INFERENCE]` Supports orchestrator-first sequencing of upland/lateral/channel aggregation in route-γ.
**Kernel mapping**: `wbk_route_02_transmission_loss`, `wbk_route_04_cascade_composition`, `wbk_route_08_orchestrator_kernel`, `wbk_route_09_inflow_aggregation_kernel`.  
**Notes / caveats**: Local extracted markdown has explicit equation numbering.  
**OAR-6 compliance status**: Primary companion for distributed semiarid routing behavior.

## R-14: Bouwer (1969) Theory of Seepage from Open Channels

**Citation**: Bouwer, H. (1969). "Theory of Seepage from Open Channels." In *Advances in Hydroscience*, pp. 121-172.  
**Local path**: `/workdir/openWEPP/references/copyrighted/bouwer1969_Theory_of_seepage.md`  
**Reference quality**: `verified-primary`  
**Topic**: Seepage/infiltration hydrodynamics, geometry effects, and conductivity-bound loss formulations.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Seepage formulations include Eq. (1), (3), (7), (9), (10), (18), (20), (25), (26) across geometric/soil conditions.
- `[DIRECT]` Explicit dependence on hydraulic conductivity (`K`) and channel geometry (`W_s`, `H_w`, wetted perimeter proxies).
- `[INFERENCE]` Provides mechanistic authority for saturation-bounded transmission-loss kernel state progression.
**Kernel mapping**: `wbk_route_02_transmission_loss`, `wbk_route_01_channel_geometry`.  
**Notes / caveats**: Hydroscience chapter is mechanistic and broader than WEPP implementation scope; mapping decisions remain kernel-design inferences.  
**OAR-6 compliance status**: Core primary authority for saturation-cap theoretical grounding.

## R-15: Bouwer & Maddock (1997) Walnut Gulch lineage / stream-aquifer interaction

**Citation**: Bouwer, H., and T. Maddock III (1997). "Making sense of the interactions between groundwater and streamflow: lessons for water masters and adjudicators." *Rivers*, 6(1), 19-31. (Lineage citation used in arid losing-stream literature.)  
**Local path**: `external-print-source`  
**Reference quality**: `external-print-source`  
**Topic**: Stream-aquifer interaction framing relevant to losing-stream interpretation and recharge accounting.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Bibliographic lineage only in this workspace; no local equation capture.
- `[INFERENCE]` Useful policy/hydrologic interpretation companion, not currently required for kernel constants.
**Kernel mapping**: `wbk_route_02_transmission_loss` (contextual).  
**Notes / caveats**: No local full-text source acquired during this pass.  
**OAR-6 compliance status**: Not sufficient for constants in current corpus.

## R-16: HEC-RAS Hydraulic Reference Manual (current version)

**Citation**: USACE Hydrologic Engineering Center (2024 export). *HEC-RAS Hydraulic Reference Manual*, Version 6.6.  
**Local path**: `/workdir/openWEPP/references/vendorable/R16_2024_HEC_RAS_Hydraulic_Reference_Manual_v6_6.md`  
**Reference quality**: `verified-primary`  
**Distribution status**: `redistributable-first-pass` (HEC-RAS front matter: public-domain + distribution language).  
**Topic**: Current engineering practice reference for open-channel hydraulics, continuity/momentum forms, and numerical routing schemes.  
**Key equations / concepts for WB-33**:
- `[DIRECT]` Manual sections include continuity and momentum equation chapters, implicit finite-difference forms, and Manning roughness tables (see extract TOC anchors).
- `[DIRECT]` Diffusion-wave approximation chapter is present and cross-links practical model limits.
- `[INFERENCE]` Provides implementation cross-check authority for geometry/wave-branch sanity bounds in route-γ.
**Kernel mapping**: `wbk_route_01_channel_geometry`, `wbk_route_03_wave_propagation`.  
**Notes / caveats**: This replaces older local v6.4.1 as current-version anchor for Phase-0B.  
**OAR-6 compliance status**: Sufficient as an engineering-practice companion; not a substitute for domain-specific transmission-loss theory sources.
## R-17: NRCS NEH Part 630 routing anchors (Chapter 21 + Chapter 17)

**Citation**: USDA NRCS, National Engineering Handbook (NEH) Part 630, Chapter 21 (*Design Hydrographs*) with flood-routing equation anchors from Part 630 Chapter 17 (*Flood Routing*).
**Local path**: `/workdir/wepp-forest/references/NEH_Part630_Ch21_Hydrologic_Routing.md`
**Reference quality**: `verified-primary`
**Topic**: Storage-indication (Modified Puls) level-pool routing equations and routing-table construction authority for impoundment hydrograph routing.
**Key equations / concepts for WB-34**:
- `[DIRECT]` Chapter 17 Eq. (17-1) continuity relation `Delta t (I - O) = Delta S`.
- `[DIRECT]` Chapter 17 Eq. (17-2) expanded continuity form with `I1`, `I2`, `O1`, `O2`, `S1`, `S2`.
- `[DIRECT]` Chapter 17 Eq. (17-3) storage-indication working form with `(S2/Delta t + O2/2)`.
- `[DIRECT]` Chapter 17 working-curve requirement: solve with `O2` versus `(S2/Delta t + O2/2)`.
- `[DIRECT]` Chapter 17 note that WinTR-20 adjusts `Delta t` internally when large-step distortion conditions occur.
- `[DIRECT]` Chapter 21 explicitly cross-references Chapter 17 for routing through structures.
**Kernel mapping**: `wbk_imp_03_level_pool_routing`, `wbk_imp_01_stage_area_storage` (working-curve support).
**Notes / caveats**: Chapter 21 is the design-hydrograph chapter; routing equations are in Chapter 17 and are cross-referenced by Chapter 21.
**OAR-6 compliance status**: Primary authority for Modified Puls/storage-indication equation family used in WB-34.

## R-18: USDA SCS TR-20 routing documentation

**Citation**: USDA Soil Conservation Service, *Technical Release 20: Computer Program for Project Formulation - Hydrology* (1965 lineage), with WinTR-20 continuity user-guide anchors.
**Local path**: `/workdir/wepp-forest/references/SCS_TR20_Routing_Documentation.md`
**Reference quality**: `verified-primary`
**Topic**: TR-20 routing coefficient workflow, structure working-curve semantics, and time-increment controls for storage routing.
**Key equations / concepts for WB-34**:
- `[DIRECT]` TR-20 permits routing coefficient `C` entry and computes modified coefficient `C*` internally for reach routing.
- `[DIRECT]` TR-20 derives coefficient selection from incremental discharge/area velocity estimates and the main time increment.
- `[DIRECT]` TR-20 ties structure routing to elevation-discharge-storage tables.
- `[DIRECT]` TR-20 increment guidance: increment `D` should remain below small-subwatershed `Tc` for hydrograph fidelity; main increment should not be too large relative to `Tp`.
- `[DIRECT]` WinTR-20 continuity diagnostic for storage routing when required storage-discharge combination exceeds structure working-curve limits.
**Kernel mapping**: `wbk_imp_03_level_pool_routing`, `wbk_imp_02_storage_discharge`.
**Notes / caveats**: Legacy TR-20 OCR is imperfect; coefficient and working-curve semantics are directly recoverable and reinforced by WinTR-20 continuity documentation.
**OAR-6 compliance status**: Primary routing-method companion for `R-17` in WB-34 storage-indication implementation.

## R-19: USBR Design of Small Dams (spillways and outlets)

**Citation**: U.S. Bureau of Reclamation, *Design of Small Dams*, 3rd edition lineage (1987), including chapter-level spillway/outlet equations and errata.
**Local path**: `/workdir/wepp-forest/references/USBR_Design_of_Small_Dams_Spillways_Outlets.md`
**Reference quality**: `verified-primary`
**Topic**: Constitutive discharge equations and coefficient behavior for weirs, spillways, and submerged inlet/orifice control.
**Key equations / concepts for WB-34**:
- `[DIRECT]` Ogee/overflow weir equation (Eq. (3)): `Q = C L H_e^(3/2)`.
- `[DIRECT]` Crest-length adjustment relation associated with Eq. (3).
- `[DIRECT]` Submerged inlet/orifice control forms (`Q = C A ...`) and equivalent boxed forms for head-defined openings.
- `[DIRECT]` Coefficient guidance: sharp-crested coefficient about `3.3`; broad-crested theoretical coefficient about `3.087`.
- `[DIRECT]` Tailwater/submergence effects reduce discharge coefficient; worked examples include `C_s/C_f`-style correction.
**Kernel mapping**: `wbk_imp_02_storage_discharge`, `wbk_imp_03_level_pool_routing` (discharge-function coupling).
**Notes / caveats**: OCR superscript/radical rendering is noisy; normalized notation is used with preserved line anchors in the local extract.
**OAR-6 compliance status**: Primary authority for impoundment outlet/spillway constitutive equations in WB-34.

## R-20: Wang et al. (2018) CPMC accuracy with lateral inflow

**Citation**: Wang, L., S. Lapin, J. Q. Wu, W. J. Elliot, and F. R. Fiedler (2018). *Accuracy of the Muskingum-Cunge method for constant-parameter diffusion-wave channel routing with lateral inflow*. arXiv:1802.04429v1.
**Local path**: `/workdir/wepp-forest/references/WangL_2018_Accuracy_of_Muskingcum-Cunge_1802.04429v1.pdf`
**Reference quality**: `verified-primary`
**Topic**: Order-of-accuracy behavior of constant-parameter Muskingum-Cunge (CPMC) with spatially and temporally variable lateral inflow.
**Key equations / concepts for WB-35**:
- `[DIRECT]` CPMC recursion with `C1..C4` (Eq. (2)) is second-order accurate without extra spatial-temporal restrictions.
- `[DIRECT]` Third-order CPMC requires specific `Delta x` / `Delta t` coupling constraints (Eq. (7), Eq. (8), and dimensionless Eq. (9)).
- `[DIRECT]` Lateral-inflow treatment drives error behavior: second-order average inflow form (Eq. (12)) versus third-order form (Eq. (11)); simplified averaging (Eq. (14)) ignores spatial derivatives.
- `[INFERENCE]` Supports WB-35 classification that WEPP `ipeak=4` is a CPMC second-order baseline unless third-order constraints and higher-order lateral-inflow terms are explicitly enforced.
**Kernel mapping**: `wbk_route_03_wave_propagation`.
**Notes / caveats**: Local filename retains source typo (`Muskingcum`); content is the 2018 arXiv preprint used for method-order traceability.
**OAR-6 compliance status**: Primary authority for CPMC order claims and lateral-inflow discretization implications in WB-35.

## R-21: Dun et al. (2009) WEPP forest-application adaptation

**Citation**: Dun, S., J. Q. Wu, W. J. Elliot, P. R. Robichaud, D. C. Flanagan, J. R. Frankenberger, R. E. Brown, and A. C. Xu (2009). *Adapting the Water Erosion Prediction Project (WEPP) model for forest applications*. Journal of Hydrology, 366(1-4), 46-54. https://doi.org/10.1016/j.jhydrol.2008.12.019
**Local path**: `/workdir/wepp-forest/references/dun2009.pdf`
**Reference quality**: `verified-primary`
**Topic**: Forest-hydrology adaptation of WEPP subsurface routines (deep percolation, lateral flow, and hillslope-to-channel transfer) for steep forested watersheds.
**Key equations / concepts for WEPP forest subsurface-flow adaptation**:
- `[DIRECT]` Percolation routine (Eq. (1a)-(1e)) computes layer drainage using available water, lower-layer saturation state, and unsaturated conductivity terms.
- `[DIRECT]` Subsurface lateral-flow routine (Eq. (3a)-(3c)) applies Darcy-law lateral flux with equivalent hydraulic conductivity, profile drainable thickness, slope gradient, and hillslope length.
- `[DIRECT]` v2008.9 adaptation adds a user-defined restrictive layer for deep percolation and applies a harmonic-mean conductivity substitution in deep-percolation computation (Eq. (1d) term replacement described in text).
- `[DIRECT]` v2008.9 adds explicit hillslope subsurface-flow transfer to channel flow, including a 24-hour uniform-flow assumption when only subsurface flow occurs.
- `[DIRECT]` Performance comparison for Hermada watershed: v2004.7 generated negligible/zero watershed runoff, while v2008.9 simulated mean annual discharge of 262 mm versus observed 275 mm; daily Nash-Sutcliffe improved from -0.17 to 0.45.
- `[INFERENCE]` Supports preserving restrictive-layer and anisotropy controls as first-class behavior in WEPP forest watershed hydrology implementations.
**Kernel mapping**: `legacy hillslope subsurface hydrology (percolation + lateral flow)`, `hillslope-to-channel runoff transfer`, `watershed discharge aggregation` (`[INFERENCE]`).
**Notes / caveats**: Paper is calibration/evaluation focused on one 9-ha Idaho watershed and explicitly notes limitations where groundwater-streamflow interaction is important.
**OAR-6 compliance status**: Primary process authority for forest subsurface-flow behavior and WEPP v2008.9 adaptation rationale; use with channel-routing references when deriving route-kernel constants.

## R-22: Srivastava (2013) dissertation on WEPP groundwater-baseflow integration

**Citation**: Srivastava, A. (2013). *Modeling Hydrological Processes in Three Mountainous Watersheds in the U.S. Pacific Northwest*. Ph.D. dissertation, Washington State University, Pullman, WA.
**Local path**: `/workdir/wepp-forest/references/Srivastava_Diss2013_14.pdf`
**Reference quality**: `verified-primary`
**Topic**: WEPP watershed-hydrology extension using a linear-reservoir groundwater component, with calibration and evaluation in Pacific Northwest forest watersheds.
**Key equations / concepts for WEPP baseflow and channel-inflow behavior**:
- `[DIRECT]` Chapter 2 daily linear-reservoir bookkeeping uses Eq. (1)-(3): `Qbi = kb * Si`, `Qsi = ks * Si`, and `Si+1 = Si + (Di - Qbi - Qsi)`, where `Di` is WEPP-simulated deep percolation recharge.
- `[DIRECT]` Chapter 3 derives an analytical baseflow solution from groundwater continuity plus storage-outflow relations (Eq. (1)-(10)); `kb`, `ks`, and initial storage are estimated with Levenberg-Marquardt least-squares fitting to observed streamflow.
- `[DIRECT]` WEPP watershed runoff assembly is explicitly described as combining hillslope surface runoff and subsurface lateral flow as channel inflow, then routing to outlet on a daily basis.
- `[DIRECT]` Priest River results report no simulated surface runoff (2005-2009), with outlet streamflow originating from subsurface lateral flow alone (no-baseflow scenario) or from subsurface lateral flow plus baseflow (with-baseflow scenario).
- `[DIRECT]` WEPP simulations are run at a daily time step; winter snow accumulation/melt computations down-scale daily climate inputs to hourly values inside the winter routine.
- `[INFERENCE]` Supports the implementation interpretation that channel inflow can be non-zero even when surface runoff is negligible, via subsurface lateral flow and groundwater baseflow contributions.
**Kernel mapping**: `legacy hillslope subsurface hydrology (deep percolation + lateral flow)`, `groundwater linear-reservoir baseflow`, `hillslope-to-channel runoff transfer`, `daily water-balance with hourly winter snow routine`.
**Notes / caveats**: Dissertation aggregates three studies (Priest River, Upper Cedar River, East Deer Creek) and includes both conceptual formulation and calibration/evaluation results; equation numbering and symbols are authoritative in the PDF body.
**OAR-6 compliance status**: Primary authority for WEPP groundwater-baseflow integration logic and watershed-scale flow-component attribution in forested mountainous basins.

## R-23: Wang (2012) dissertation on channel routing and WEPP integration

**Citation**: Wang, L. (2012). *Channel Routing Using Discrete Hayami Convolution Method with Applications to the Water Erosion Prediction Project (WEPP) Model*. Ph.D. dissertation, Washington State University, Pullman, WA.
**Local path**: `/workdir/wepp-forest/references/L_Wang_10757974.pdf`
**Reference quality**: `verified-primary`
**Topic**: Channel-routing method development and evaluation for watershed modeling, with direct WEPP implementation guidance (LKW, CPMC, MVPMC3), lateral-inflow treatment, and routing input semantics.
**Key equations / concepts for WEPP channel routing and lateral inflow handling**:
- `[DIRECT]` Muskingum-Cunge recursion is given in Eq. (2.15): `Q(i+1,j+1) = C1*Q(i,j+1) + C2*Q(i,j) + C3*Q(i+1,j) + C4*Delta x*q`, with `C1..C4` defined from `K`, `X`, and `Delta t`.
- `[DIRECT]` WEPP hillslope daily hydrograph parameters used for channel-routing inflow include `td`, `tc`, `qp`, `V`, and `Vsb` (subsurface runoff volume with an assumed 24-hour duration); Eq. (2.16) uses a double-exponential form to construct a continuous inflow hydrograph.
- `[DIRECT]` WEPP routing-option controls: `ipeak=3` (LKW), `ipeak=4` (CPMC), and `ipeak=5` (MVPMC3); `ipeak=1/2` retain modified EPIC/CREAMS behavior; routing outputs for `ipeak>2` require `chan.inp`.
- `[DIRECT]` Appendix routing code documents the CPMC lateral-only fallback (`qref = 0.5*(qtmin + qtmax)`) for cases with lateral inflow but no inflow from the channel top.
- `[DIRECT]` Discrete Hayami finding: using point kernel values can incur mass-balance error when the rising limb is too short; center-averaged kernel values preserve unity kernel integration and mass balance.
- `[DIRECT]` CPMC accuracy finding: general second-order behavior, with third-order accuracy under specific space-time discretization constraints; lateral-inflow discretization materially controls routing accuracy.
- `[INFERENCE]` Provides implementation-level support for route-γ decisions on hydrograph construction, lateral-inflow averaging, and method selection defaults in WEPP-style daily watershed workflows.
**Kernel mapping**: `wbk_route_03_wave_propagation`, `wbk_route_04_cascade_composition`, `wbk_route_08_orchestrator_kernel`, `wbk_route_09_inflow_aggregation_kernel`.
**Notes / caveats**: This dissertation is the primary umbrella source for several derivative artifacts already in the corpus (including chapter-level CPMC accuracy content also represented by `R-20`); use chapter context when citing equation provenance.
**OAR-6 compliance status**: Primary implementation authority for WEPP channel-routing option semantics (`ipeak`, `chan.inp`) and a core theory-to-code bridge for lateral-inflow routing accuracy.

## R-24: Dun et al. (2010) WEPP frost-simulation subroutine improvements

**Citation**: Dun, S., J. Q. Wu, D. K. McCool, J. R. Frankenberger, and D. C. Flanagan (2010). *Improving Frost-Simulation Subroutines of the Water Erosion Prediction Project (WEPP) Model*. Transactions of the ASABE, 53(5), 1399-1411. https://doi.org/10.13031/2013.34896
**Local path**: `references/copyrighted/Dun2008_10.13031@2013.34896.pdf` (local-only copyrighted cache; filename preserves original intake name).
**Reference quality**: `verified-primary` (full text acquired locally and read for frost-source annotation, 2026-06-25).
**Topic**: The v2006.5 -> v2010.1 rewrite of WEPP's frost simulation: snow-residue-soil discretization, revised lower-front heat-flow and hydraulic parameter computation, frozen saturated hydraulic conductivity, water migration toward the freezing front, and daily water-balance integration; validated against Pullman WA and Morris MN plots.
**Key equations / concepts**: the operative authority lineage for the pinned-baseline frost routines (`frostn`/`frzng`/`frznw`/`mlttp`/`mltbtm`/`frwatc`/`watdst`) that openWEPP's FDHP01 D3 fine-sublayer port implements; CRM eqn [3.8.1]-[3.8.4] energy terms; snow thermal conductivity from density; frozen hydraulic conductivity computed from ice-content-adjusted pore space; and published water migration using generalized Clausius-Clapeyron front potential. The paper creates a source-conflict to resolve before `Qwet` promotion: the published study uses active migration/frozen-front potential for Pullman/Morris calibration, while the pinned baseline source disables the Eq. [3.8.4] migration-heat middle term with `frzng.for` `frzftp = 0.0`.
**Kernel mapping**: `SC-SNOWFREEZE-001` (`INV-SNOWFREEZE-006`/`-012`), FDHP01 work package (`docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/`), `docs/backlog/20260612-frost-heave-frozen-fringe-impedance-formulation.md`.
**Notes / caveats**: Treat as primary WEPP-lineage evidence and as a `Qwet` source-conflict flag, not as standalone production authority to enable migration heat. Any `Qwet` implementation still requires contract-first reconciliation against pinned baseline source, observed frost-depth/runoff behavior, frozen hydraulic-conductivity authority, and mass/energy gates.
**OAR-6 compliance status**: Primary peer-reviewed source for the frost lineage; restricted local cache only.

## R-25: Shen (2011) WSU MS thesis — WEPP snow distribution

**Citation**: Shen, D. (2011). *Simulating Snow Distribution Using the Water Erosion Prediction Project (WEPP) Model*. M.S. thesis, Department of Biological Systems Engineering, Washington State University.
**Local path**: `references/copyrighted/D_Shen_020312.pdf` (cached but previously unindexed — entry added 2026-06-12)
**Reference quality**: `verified-primary` (thesis PDF on disk; title page verified)
**Topic**: WEPP snow accumulation/distribution simulation — the WSU lineage adjacent to the snow density/settling behavior implicated in the FDHP01 F4 finding (openWEPP midwinter snow density ~381 kg/m³ vs legacy ~250 at matching SWE → ~4× insulation deficit).
**Kernel mapping**: snow Stage-2 science review (`docs/backlog/20260605-snow-code-deferred-science-review.md`), F4 snow density/depth-split disposition (FDHP01 staged plan).
**Notes / caveats**: MS thesis, not peer-reviewed journal text; use as lineage/context alongside `snowd.for` source authority.
**OAR-6 compliance status**: Supporting source for the snow density/depth-split work; not a primary physics authority on its own.

## R-26: Watanabe & Flury (2008) frozen-soil hydraulic conductivity

**Citation**: Watanabe, K., and M. Flury (2008). *Capillary bundle model of hydraulic conductivity for frozen soil*. Water Resources Research, 44, W12402. https://doi.org/10.1029/2008WR007012
**Local path**: `references/copyrighted/watanabe2008.pdf` (local-only copyrighted cache).
**Reference quality**: `verified-primary` (full text acquired locally and read for frost-source annotation, 2026-06-25).
**Topic**: Pore-scale capillary-bundle model for frozen hydraulic conductivity as a function of temperature, pore ice, and unfrozen water.
**Key equations / concepts**: Gibbs-Thomson freezing-point depression, ice occupying capillary centers, annular water flow around ice, dominance of ice-free capillaries near 0 degC, and divergence from unfrozen-soil hydraulic conductivity at lower temperatures.
**Kernel mapping**: `SC-SNOWFREEZE-001` future `GAP-SNOWFREEZE-002` adjudication; `docs/backlog/20260612-frost-heave-frozen-fringe-impedance-formulation.md`; candidate `K_frozen(theta_liq, T, soil_params)` research path.
**Notes / caveats**: Strong mechanistic source for frozen hydraulic conductivity, but not a WEPP implementation spec and not enough alone to enable `Qwet`. Pair with Dun et al. (2010), Kurylyk & Watanabe (2013), and observation gates.
**OAR-6 compliance status**: Primary restricted-cache source for frozen hydraulic-conductivity physics.

## R-27: Kurylyk & Watanabe (2013) freezing/thawing math review

**Citation**: Kurylyk, B. L., and K. Watanabe (2013). *The mathematical representation of freezing and thawing processes in variably-saturated, non-deformable soils*. Advances in Water Resources, 60, 160-177. https://doi.org/10.1016/j.advwatres.2013.07.016
**Local path**: `references/copyrighted/kurylyk2013.pdf` (local-only copyrighted cache).
**Reference quality**: `verified-primary` (review article; local full text read for source annotation).
**Topic**: Review and synthesis of Clapeyron equations, soil freezing characteristic curves, soil water retention relationships, and hydraulic conductivity models for partially frozen soils.
**Key equations / concepts**: theory chain from soil water retention or soil freezing characteristic curve to unfrozen water, liquid pressure, and frozen hydraulic conductivity; comparison of model formulations and unresolved assumptions.
**Kernel mapping**: `SC-SNOWFREEZE-001` future contract-first frost-depth physics amendments; candidate authority for SFCC/frozen-K decision criteria.
**Notes / caveats**: Best first review before modifying frost physics. It should shape model-selection gates but does not replace WEPP lineage or observation validation.
**OAR-6 compliance status**: Primary review source; restricted local cache.

## R-28: Dall'Amico et al. (2011) energy-conserving frozen-soil numerics

**Citation**: Dall'Amico, M., S. Endrizzi, S. Gruber, and R. Rigon (2011). *A robust and energy-conserving model of freezing variably-saturated soil*. The Cryosphere, 5, 469-484. https://doi.org/10.5194/tc-5-469-2011
**Local path**: `references/vendorable/Amico2011.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `redistributable` (article text declares CC Attribution 3.0 License).
**Topic**: Coupled Richards-equation and energy-equation solution for variably saturated freezing soil with robust nonlinear convergence and latent-heat consistency.
**Key equations / concepts**: conservative mass/energy formulation, generalized Clapeyron pressure relation, hydraulic-conductivity impedance option, Stefan/Neumann-style validation, and comparison to experimental freezing/thawing data.
**Kernel mapping**: `SC-SNOWFREEZE-001` future numerical-stability and latent-heat gate design; `GAP-SNOWFREEZE-002` benchmark planning.
**Notes / caveats**: Use as numerics and conservation reference, not as a drop-in replacement for WEPP's snow-residue-soil winter column.
**OAR-6 compliance status**: Primary open-access source; vendored under CC-BY 3.0.

## R-29: Kurylyk et al. (2014) cold-regions thaw benchmark solutions

**Citation**: Kurylyk, B. L., J. M. McKenzie, K. T. B. MacQuarrie, and C. I. Voss (2014). *Analytical solutions for benchmarking cold regions subsurface water flow and energy transport models: one-dimensional soil thaw with conduction and advection*. Advances in Water Resources, 70, 172-184. https://doi.org/10.1016/j.advwatres.2014.05.005
**Local path**: `references/copyrighted/kurylyk2014.pdf` (local-only copyrighted cache; accepted manuscript).
**Reference quality**: `verified-primary`
**Topic**: Analytical thaw-front benchmark scenarios with conduction, advection, phase change, porosity, surface temperature, Darcy velocity, and initial-temperature variation.
**Key equations / concepts**: Lunardini and Neumann benchmark framing, Stefan number sensitivity, and published scenario results suitable for future code-to-analytical checks.
**Kernel mapping**: Future `GAP-SNOWFREEZE-002` validation package; benchmark tests before field calibration or `Qwet` tuning.
**Notes / caveats**: Benchmark authority only; it validates one-dimensional thaw/transport numerics, not WEPP winter hydrology as a whole.
**OAR-6 compliance status**: Primary restricted-cache source.

## R-30: Azmatch et al. (2012) SFCC-derived hydraulic conductivity

**Citation**: Azmatch, T. F., D. C. Sego, L. U. Arenson, and K. W. Biggar (2012). *Using soil freezing characteristic curve to estimate the hydraulic conductivity function of partially frozen soils*. Cold Regions Science and Technology, 83-84, 103-109. https://doi.org/10.1016/j.coldregions.2012.07.002
**Local path**: `references/copyrighted/azmatch2012.pdf` (local-only copyrighted cache).
**Reference quality**: `verified-primary`
**Topic**: Estimating partially frozen hydraulic conductivity from the soil freezing characteristic curve rather than only the unfrozen soil water characteristic curve.
**Key equations / concepts**: SFCC measurement from unfrozen-water and temperature data; indirect frozen hydraulic-conductivity estimation; comparison against other methods and direct measurements.
**Kernel mapping**: Candidate future `K_frozen` model under `GAP-SNOWFREEZE-002`; not current runtime authority.
**Notes / caveats**: Useful candidate for partially frozen soil, but field-scale WEPP adoption requires source-specific parameter availability and observation gates.
**OAR-6 compliance status**: Primary restricted-cache source.

## R-31: Ming et al. (2020) saturated frozen hydraulic conductivity from SFCC

**Citation**: Ming, F., L. Chen, D. Li, and X. Wei (2020). *Estimation of hydraulic conductivity of saturated frozen soil from the soil freezing characteristic curve*. Science of the Total Environment, 698, 134132. https://doi.org/10.1016/j.scitotenv.2019.134132
**Local path**: `references/copyrighted/ming2020.pdf` (local-only copyrighted cache).
**Reference quality**: `verified-primary`
**Topic**: Saturated frozen hydraulic-conductivity estimation from SFCC-derived pore-size distribution and capillary-bundle assumptions.
**Key equations / concepts**: SFCC plus Gibbs-Thomson pore-size relation, Hagen-Poiseuille/Darcy formulation, validation across datasets spanning multiple orders of hydraulic conductivity.
**Kernel mapping**: Candidate saturated-frozen `K_frozen(T)` model for future `Qwet` or infiltration-resistance research.
**Notes / caveats**: Saturated scope is narrower than openWEPP's variably saturated hillslope column; do not apply blindly to all layers.
**OAR-6 compliance status**: Primary restricted-cache source.

## R-32: Amankwah et al. (2021) salt-exclusion soil freezing curve

**Citation**: Amankwah, S. K., et al. (2021). *A Model for the Soil Freezing Characteristic Curve That Represents the Dominant Role of Salt Exclusion*. Water Resources Research, 57, e2021WR030070. https://doi.org/10.1029/2021WR030070
**Local path**: `references/copyrighted/Amankwah2021.pdf` (local-only copyrighted cache).
**Reference quality**: `verified-primary`
**Topic**: Soil freezing characteristic curves where solutes/salt exclusion can dominate freezing-point depression relative to capillary-only generalized Clapeyron formulations.
**Key equations / concepts**: salt-exclusion and combined salt-GCE models, hysteresis and antecedent moisture controls, and salinity as a possible fitting/diagnostic variable.
**Kernel mapping**: Future optional SFCC extension for saline/roadside/reclaimed/irrigated/arid soils; out of current default frost-depth scope unless site evidence requires it.
**Notes / caveats**: Important to keep as a known model limitation, but not a first-order authority for ordinary non-saline WEPP upland erosion cases.
**OAR-6 compliance status**: Primary restricted-cache source.

## R-33: Cheng et al. (2023) frozen hydraulic-conductivity impedance factor

**Citation**: Cheng, S.-H., B. A. Engel, R. Liu, H.-X. Wu, and Y.-B. Wang (2023). *Impedance Factor of Hydraulic Conductivity for Frozen Soil Based on Ice Segregation Theory and Its Application*. Water Resources Research, 59, e2022WR033876. https://doi.org/10.1029/2022WR033876
**Local path**: `references/copyrighted/Cheng2023.pdf` (local-only copyrighted cache).
**Reference quality**: `verified-primary`
**Topic**: Physical interpretation of hydraulic-conductivity impedance factors using ice segregation theory and distinction between closed unsaturated and open saturated freezing systems.
**Key equations / concepts**: impedance factor tied to pore ice segregation, small impedance effect in closed unsaturated systems, stronger need under open saturated/ice-lens-forming conditions, and coupling with van Genuchten hydraulic conductivity.
**Kernel mapping**: Candidate constraint for future `Qwet` or frozen-fringe implementation; supports rejecting a blanket empirical impedance multiplier without regime tests.
**Notes / caveats**: Use to decide when impedance is necessary, not to assert it is universally required.
**OAR-6 compliance status**: Primary restricted-cache source.

## R-34: Devoie et al. (2022) measured SFCC repository

**Citation**: Devoie, E. G., S. Gruber, and J. M. McKenzie (2022). *A repository of measured soil freezing characteristic curves: 1921 to 2021*. Earth System Science Data, 14, 3365-3377. https://doi.org/10.5194/essd-14-3365-2022
**Local path**: `references/vendorable/Devoie2022.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `redistributable` (article text declares Creative Commons Attribution 4.0 License).
**Topic**: Open repository of measured soil freezing characteristic curves from historic and modern studies, with metadata and an R package interface.
**Key equations / concepts**: SFCC data provenance, parameter-prior support, data gaps for coarse soils and in-situ mountainous measurements, and Zenodo dataset DOI `10.5281/zenodo.5592825`.
**Kernel mapping**: Future `GAP-SNOWFREEZE-002` parameter priors, uncertainty analysis, and texture-class sanity checks for SFCC model selection.
**Notes / caveats**: Dataset authority, not an equation authority. Use with original dataset citations and site-specific applicability checks.
**OAR-6 compliance status**: Primary open-access data-source paper; vendored under CC-BY 4.0.
