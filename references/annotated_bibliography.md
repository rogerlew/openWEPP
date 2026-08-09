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
**Local path**: `references/copyrighted/dun2009.pdf`
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

## R-22A: Srivastava et al. (2013) WEPP linear-reservoir baseflow paper

**Citation**: Srivastava, A., M. Dobre, J. Q. Wu, W. J. Elliot, E. A. Bruner, S. Dun, E. S. Brooks, and I. S. Miller (2013). *Modifying WEPP to improve streamflow simulation in a Pacific Northwest watershed*. Transactions of the ASABE, 56(2), 603-611. https://doi.org/10.13031/2013.42691
**Local path**: `references/copyrighted/Srivastava2013.pdf`
**Reference quality**: `verified-primary`
**Topic**: Peer-reviewed companion to Srivastava (2013) dissertation's Priest River groundwater/baseflow work, adding a linear-reservoir baseflow routine to WEPP streamflow simulation.
**Key equations / concepts for WEPP baseflow and channel-inflow behavior**:
- `[DIRECT]` Streamflow components are surface runoff, subsurface lateral flow, and groundwater baseflow; WEPP already simulated the first two plus deep percolation, but required a baseflow component for groundwater-contributing watersheds.
- `[DIRECT]` Baseflow is determined using a linear reservoir model driven by WEPP-simulated deep percolation and groundwater storage/outflow proportionality.
- `[DIRECT]` Priest River evaluation reports improved streamflow agreement with baseflow included and identifies simulated baseflow as a substantial share of annual streamflow and precipitation.
- `[INFERENCE]` Use as the peer-reviewed companion authority for the dissertation's linear-reservoir equations and for distinguishing subsurface lateral flow from groundwater baseflow in watershed routing.
**Kernel mapping**: `groundwater linear-reservoir baseflow`, `deep-percolation recharge`, `hillslope-pass groundwater baseflow volume`, `watershed channel inflow partition`.
**Notes / caveats**: This PDF is the 2013 ASABE paper, not the dissertation PDF. The dissertation remains the primary equation/source-code interpretation authority where it gives fuller derivations and calibration method detail.
**OAR-6 compliance status**: Primary companion authority for the linear-reservoir WEPP baseflow implementation lineage.

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

## R-35: Anderson (1976) NOAA NWS-19 point energy/mass balance snow model

**Citation**: Anderson, E. A. (1976). *A Point Energy and Mass Balance Model of a Snow Cover*. NOAA Technical Report NWS-19, U.S. Dept. of Commerce / National Weather Service.
**Local path**: `references/copyrighted/noaa_6392_DS1.pdf` (+ OCR `references/copyrighted/noaa_6392_DS1.md`)
**Reference quality**: `verified-primary`
**Distribution status**: `public-domain` (U.S. Government work, 17 U.S.C. 105; eligible to move to `vendorable/`, left in cache to avoid breaking references).
**Topic**: The foundational energy-balance snow-cover model — surface energy exchange (net radiation + turbulent sensible/latent fluxes + advected heat), within-pack heat transfer, and §III density changes (compaction, destructive/constructive/melt metamorphism). The Anderson-1976 lineage behind SNOBAL densification.
**Key equations / concepts**: §II energy-balance melt (no degree-day factor — melt is the energy-balance residual); §III "Increase in density due to compaction" (p.37) and metamorphism terms (pp.38–40) = the PTM/POC overburden-compaction authority. Scanned report; OCR text layer added 2026-06-25.
**Kernel mapping**: `SC-SNOWFREEZE-001` `GAP-SNOWFREEZE-002`; melt-modernization decision and `physics_bulk` densification (`docs/planning/snow-frost-fidelity-strategy.md` §2/§5).
**Notes / caveats**: Pre-remote-sensing (1976); the energy-balance melt assumed met fluxes that gridded shortwave now supplies. Authority for *compaction* and *energy-balance melt*, not for a degree-day melt factor.
**OAR-6 compliance status**: Primary physics authority for snow energy-balance melt and densification.

## R-36: Anderson (2006) SNOW-17 NWSRFS documentation

**Citation**: Anderson, E. A. (2006). *Snow Accumulation and Ablation Model — SNOW-17*. NWSRFS User Documentation, NOAA/National Weather Service.
**Local path**: `references/vendorable/Anderson2006_SNOW17.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `redistributable` (U.S. Government work, public domain; downloaded from weather.gov).
**Topic**: The operational temperature-index (degree-day) snow model — the lineage of the *melt factor* itself, with seasonally varying `MFMAX`/`MFMIN` (sinusoidal between solstices) and forest-cover reduction.
**Key equations / concepts**: Non-rain melt `M = Mf·(Ta − Tb)` with seasonal `Mf`; rain-on-snow melt; areal depletion; the forest-class melt-factor table (dense conifer lowest → open highest). The authority for *why a melt factor must vary seasonally and with canopy* rather than being a single fitted constant.
**Kernel mapping**: Melt-modernization decision (`docs/planning/snow-frost-fidelity-strategy.md` §2/§5); documented lighter-melt fallback context.
**Notes / caveats**: Distinct from R-35 (NWS-19): SNOW-17 is the degree-day operational model; NWS-19 is the energy-balance research model. Cite the right one for the right claim.
**OAR-6 compliance status**: Primary authority for the degree-day melt-factor lineage.

## R-37: Ohmura (2001) physical basis of the temperature-index melt method

**Citation**: Ohmura, A. (2001). *Physical Basis for the Temperature-Based Melt-Index Method*. Journal of Applied Meteorology, 40(4), 753–761. https://doi.org/10.1175/1520-0450(2001)040<0753:PBFTTB>2.0.CO;2
**Local path**: `references/copyrighted/Ohmura2001_meltindex.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (AMS journal copyright; freely readable, not redistributable).
**Topic**: Why air temperature indexes melt — incoming longwave radiation (the largest ablation-season source) and sensible heat are both temperature-correlated; net shortwave `(1−α)I` is not, and is what the index misses.
**Key equations / concepts**: ~60% of clear-sky atmospheric emission from the lowest 100 m (>90% under overcast), making screen-height `Ta` a good longwave proxy; the justification for an enhanced temperature-index that adds an explicit shortwave term.
**Kernel mapping**: Melt-physical-interpretation authority for the strategy decision (`docs/planning/snow-frost-fidelity-strategy.md` §1/§5).
**Notes / caveats**: Glacier/ice-sheet framing; the physical argument transfers to seasonal snow.
**OAR-6 compliance status**: Primary authority for the melt-factor physical interpretation.

## R-38: Krinner et al. (2018) ESM-SnowMIP

**Citation**: Krinner, G., et al. (2018). *ESM-SnowMIP: assessing snow models and quantifying snow-related climate feedbacks*. Geoscientific Model Development, 11, 5027–5049. https://doi.org/10.5194/gmd-11-5027-2018
**Local path**: `references/vendorable/Krinner2018_ESM-SnowMIP.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `redistributable` (Copernicus, CC-BY 4.0).
**Topic**: Multi-model snow intercomparison; model complexity does not by itself explain performance spread.
**Key equations / concepts**: "Complexity ≠ skill" evidence supporting modernization-not-complexification of the melt path.
**Kernel mapping**: Strategy guardrail/justification (`docs/planning/snow-frost-fidelity-strategy.md` §1).
**Notes / caveats**: Climate-model context; use as a methodological prior, not a parameter source.
**OAR-6 compliance status**: Supporting intercomparison authority.

## R-39: Lute et al. (2022) SnowClim v1.0

**Citation**: Lute, A. C., J. T. Abatzoglou, and T. E. Link (2022). *SnowClim v1.0: a high-resolution snow modeling framework*. Geoscientific Model Development, 15, 5045–5071. https://doi.org/10.5194/gmd-15-5045-2022
**Local path**: `references/vendorable/Lute2022_SnowClim.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `redistributable` (Copernicus, CC-BY 4.0).
**Topic**: Energy-balance snow framework in the Marks/SNOBAL lineage; §2.2.7 documents the shallow-snowpack pack-temperature instability.
**Key equations / concepts**: Shallow-snow cold-content/mass instability and the `SWE < 15 mm × Δt_hours → T_pack = min(T_air, 0)` clamp; cited in the PySnobal CSS WY2017 disposition.
**Kernel mapping**: PySnobal thin-snow disposition (`...snowfrost-fidelity-h.../artifacts/pysnobal-css-wy2017-disposition.md`); `GAP-SNOWFREEZE-002`.
**Notes / caveats**: Reference/diagnostic context under ADR-0017; not a runtime authority.
**OAR-6 compliance status**: Supporting authority for shallow-snow numerical stability.

## R-40: Vionnet et al. (2012) Crocus / SURFEX

**Citation**: Vionnet, V., et al. (2012). *The detailed snowpack scheme Crocus and its implementation in SURFEX v7.2*. Geoscientific Model Development, 5, 773–791. https://doi.org/10.5194/gmd-5-773-2012
**Local path**: `references/vendorable/Vionnet2012_Crocus.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `redistributable` (Copernicus, CC-BY 4.0).
**Topic**: Detailed multilayer snowpack scheme — fresh-snow density, settling/metamorphism, and albedo as a fuller reference implementation.
**Key equations / concepts**: Temperature/wind fresh-snow density; viscosity-based settling; spectral-band optical-grain-size albedo — candidate fuller alternatives to the bulk approach.
**Kernel mapping**: `physics_bulk` fresh-snow-density and albedo open decisions (`docs/planning/snow-frost-fidelity-strategy.md` §5/§9).
**Notes / caveats**: Multilayer scheme; heavier than the bulk target. Reference, not a drop-in.
**OAR-6 compliance status**: Supporting reference-implementation authority.

## R-41: Gupta et al. (2023) shortwave distribution → coupled melt + ET

**Citation**: Gupta, A., et al. (2023). Hydrology and Earth System Sciences, 27, 191–212. https://doi.org/10.5194/hess-27-191-2023
**Local path**: `references/vendorable/Gupta2023_HESS.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `redistributable` (Copernicus, CC-BY 4.0).
**Topic**: Distributing shortwave by slope/aspect shifts snowmelt timing *and* the ET regression slope simultaneously — direct evidence that one radiation field forces both processes.
**Key equations / concepts**: ~20-day melt-timing shift with concurrent ET-slope change (1.55→1.18); the empirical basis for the "calibrate melt coefficients, never the shared radiation forcing" guardrail.
**Kernel mapping**: Radiation/ET-coupling guardrail (`docs/planning/snow-frost-fidelity-strategy.md` §2/§4).
**Notes / caveats**: Confirm exact citation/author list on retrieval (agent-relayed metadata).
**OAR-6 compliance status**: Supporting authority for the radiation/ET-coupling discipline.

## R-42: Ménard et al. (2021) scientific/human error in snow-model intercomparison

**Citation**: Ménard, C. B., et al. (2021). *Scientific and Human Errors in a Snow Model Intercomparison*. Bulletin of the American Meteorological Society, 102(1), E61–E79. https://doi.org/10.1175/BAMS-D-19-0329.1
**Local path**: `references/copyrighted/Menard2021_BAMS.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (AMS journal copyright; freely readable, not redistributable).
**Topic**: Much inter-model spread traces to implementation/human error rather than physics; corrections cut error substantially.
**Key equations / concepts**: Implementation correctness can dominate model-structure choice — supports investing in a correct energy-balance implementation over added complexity.
**Kernel mapping**: Strategy methodology prior (`docs/planning/snow-frost-fidelity-strategy.md` §1).
**Notes / caveats**: Methodological prior, not a parameter source.
**OAR-6 compliance status**: Supporting intercomparison authority.

## R-43: Pellicciotti et al. (2005) enhanced temperature-index melt model

**Citation**: Pellicciotti, F., B. Brock, U. Strasser, P. Burlando, M. Funk, and J. Corripio (2005). *An enhanced temperature-index glacier melt model including the shortwave radiation balance: development and testing for Haut Glacier d'Arolla, Switzerland*. Journal of Glaciology, 51(175), 573–587. https://doi.org/10.3189/172756505781829124
**Local path**: `references/copyrighted/pellicciotti2005.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (IGS / Cambridge Core journal copyright; operator-supplied local cache).
**Topic**: The reference enhanced-temperature-index (ETI) melt model that makes the shortwave term explicit: `M = TF·T + SRF·(1−α)·I` for `T > T_T = 1 °C`.
**Key equations / concepts**: Calibrated `TF = 0.05 mm h^-1 degC^-1`, `SRF = 0.0094 mm h^-1 W^-1 m^2` (near the physical conversion `0.01078`); accounts for 90–95% of energy-balance reference melt; the documented lighter-melt alternative if the modernized CoE energy balance is ever too heavy.
**Kernel mapping**: Melt-modernization decision, documented ETI fallback (`docs/planning/snow-frost-fidelity-strategy.md` §2/§5).
**Notes / caveats**: Glacier development site; degrades under persistent overcast. Use `SRF` as the transferable default, `TF` as a small calibratable knob.
**OAR-6 compliance status**: Primary authority for the ETI melt form and factor values.

## R-44: Carenzo et al. (2009) ETI transferability/robustness

**Citation**: Carenzo, M., F. Pellicciotti, S. Rimkus, and P. Burlando (2009). *Assessing the transferability and robustness of an enhanced temperature-index glacier-melt model*. Journal of Glaciology, 55(190), 258–274. https://doi.org/10.3189/002214309788608804
**Local path**: `references/copyrighted/carenzo2009.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (IGS / Cambridge Core; operator-supplied local cache).
**Topic**: Cross-site/season/glacier transferability of the ETI parameters.
**Key equations / concepts**: `SRF` mean 0.0093, CV ~6% (stable/transferable) vs `TF` mean 0.055, CV ~56% (volatile); ETI more transferable than a constant degree-day factor — the quantitative basis for "physical default, optional calibration."
**Kernel mapping**: Melt-modernization guardrail (`docs/planning/snow-frost-fidelity-strategy.md` §2/§4).
**Notes / caveats**: Glacier context; the transferability asymmetry is the load-bearing finding.
**OAR-6 compliance status**: Primary authority for ETI parameter transferability.

## R-45: Hock (1999) distributed temperature-index melt with potential solar radiation

**Citation**: Hock, R. (1999). *A distributed temperature-index ice- and snowmelt model including potential direct solar radiation*. Journal of Glaciology, 45(149), 101–111. https://doi.org/10.3189/S0022143000003087
**Local path**: `references/copyrighted/hock1999.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (IGS / Cambridge Core; operator-supplied local cache).
**Topic**: Adds DEM-derived potential clear-sky direct radiation to a degree-day model — spatial/diurnal melt structure with no extra met data.
**Key equations / concepts**: `M = (MF + r_snow/ice·I)·T`; the radiation-index precursor to the Pellicciotti ETI form.
**Kernel mapping**: ETI lineage context (`docs/planning/snow-frost-fidelity-strategy.md` §5).
**Notes / caveats**: Potential (modeled) radiation, not measured; superseded by ETI where measured/gridded shortwave is available.
**OAR-6 compliance status**: Supporting authority for radiation-index melt.

## R-46: Brock, Willis & Sharp (2000) albedo parameterization

**Citation**: Brock, B. W., I. C. Willis, and M. J. Sharp (2000). *Measurement and parameterization of albedo variations at Haut Glacier d'Arolla, Switzerland*. Journal of Glaciology, 46(155), 675–688. https://doi.org/10.3189/172756500781832675
**Local path**: `references/copyrighted/brock2000.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (IGS / Cambridge Core; operator-supplied local cache).
**Topic**: Albedo decay as a function of accumulated daily-maximum temperature since snowfall (a metamorphism/grain-size proxy needing only temperature).
**Key equations / concepts**: Deep-snow logarithmic vs shallow-snow exponential decay toward the underlying surface albedo; the leading temperature-only albedo scheme for the modernized melt shortwave term. (Exact coefficients to be read from the PDF, not from secondary summaries.)
**Kernel mapping**: Albedo open decision (`docs/planning/snow-frost-fidelity-strategy.md` §5/§9).
**Notes / caveats**: Glacier site; pair with canopy attenuation for forested fixtures.
**OAR-6 compliance status**: Primary candidate authority for the albedo state.

## R-47: Walter et al. (2005) process-based snowmelt with minimal data

**Citation**: Walter, M. T., E. S. Brooks, D. K. McCool, L. G. King, M. Molnau, and J. Boll (2005). *Process-based snowmelt modeling: does it require more input data than temperature-index modeling?* Journal of Hydrology, 300, 65–75. https://doi.org/10.1016/j.jhydrol.2004.05.002
**Local path**: `references/copyrighted/walter2005.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (Elsevier; operator-supplied local cache).
**Topic**: An uncalibrated surface energy balance driven by `Tmax`/`Tmin` (+ now gridded shortwave) outperforms a best-fit temperature index — in WEPP's own USDA-ARS/PNW author lineage (McCool, King, Boll).
**Key equations / concepts**: Bristow–Campbell shortwave from temperature range; Stefan–Boltzmann longwave; pooled R²≈0.92 (EB) vs 0.76 (index); melt robust to per-component errors. **The strongest support for modernizing the CoE energy balance rather than adopting a degree-day factor.**
**Kernel mapping**: Melt-modernization decision (`docs/planning/snow-frost-fidelity-strategy.md` §1/§2/§5).
**Notes / caveats**: Windy/highly-variable sites are the boundary case; gridded shortwave is an upgrade on the paper's own Bristow–Campbell estimate.
**OAR-6 compliance status**: Primary WEPP-lineage authority for energy-balance melt on minimal data.

## R-48: Marks et al. (1999) SNOBAL spatially distributed energy-balance snowmelt

**Citation**: Marks, D., J. Domingo, D. Susong, T. Link, and D. Garen (1999). *A spatially distributed energy balance snowmelt model for application in mountain basins*. Hydrological Processes, 13, 1935–1959. https://doi.org/10.1002/(SICI)1099-1085(199909)13:12/13<1935::AID-HYP868>3.0.CO;2-C
**Local path**: `references/copyrighted/marks1999.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (Wiley; operator-supplied local cache).
**Topic**: The SNOBAL two-layer energy-balance snowmelt model and the operational realization of Anderson-1976 densification (PTM/POC compaction).
**Key equations / concepts**: Two-layer mass/energy balance; the compaction lineage implemented in PySnobal `_time_compact.c`/`_h2o_compact.c`; the shallow-snow pack-temperature instability discussed in Lute (2022) traces here.
**Kernel mapping**: `physics_bulk` densification candidate and PySnobal disposition (`docs/planning/snow-frost-fidelity-strategy.md` §5; CSS WY2017 disposition).
**Notes / caveats**: Reference implementation under ADR-0017, not a correctness authority.
**OAR-6 compliance status**: Primary authority for the SNOBAL energy-balance/compaction lineage.

## R-49: Magnusson et al. (2015) snow-model process-representation evaluation

**Citation**: Magnusson, J., D. Wever, R. Essery, N. Helbig, A. Winstral, and T. Jonas (2015). *Evaluating snow models with varying process representations for hydrological applications*. Water Resources Research, 51, 2707–2723. https://doi.org/10.1002/2014WR016498
**Local path**: `references/copyrighted/magnusson2015.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (Wiley/AGU; operator-supplied local cache).
**Topic**: Calibrated simple and physical snow models both perform reasonably; the physical model's edge is fewer calibrated parameters → better transferability, not raw accuracy.
**Key equations / concepts**: Supports "physical defaults + optional calibration" over an over-parameterized empirical fit.
**Kernel mapping**: Strategy guardrail (`docs/planning/snow-frost-fidelity-strategy.md` §4).
**Notes / caveats**: Methodological prior.
**OAR-6 compliance status**: Supporting authority for parameter-parsimony/transferability.

## R-50: Lundquist et al. (2013) forest density and snow retention

**Citation**: Lundquist, J. D., S. E. Dickerson-Lange, J. A. Lutz, and N. C. Cristea (2013). *Lower forest density enhances snow retention in regions with warmer winters: A global framework motivated by Western US observations*. Water Resources Research, 49, 6356–6370. https://doi.org/10.1002/wrcr.20504
**Local path**: `references/copyrighted/lundquist2013.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (Wiley/AGU; operator-supplied local cache).
**Topic**: The net canopy effect on snow melt-out flips sign at ~1 °C DJF mean air temperature (shading-dominated below, longwave-dominated above).
**Key equations / concepts**: A single lumped canopy melt reduction is wrong — the canopy term must be climate-dependent; relevant to the forested SNOTEL/frost fixtures.
**Kernel mapping**: Canopy albedo/melt open decision (`docs/planning/snow-frost-fidelity-strategy.md` §5/§9).
**Notes / caveats**: Basis for not hard-coding a constant canopy melt factor.
**OAR-6 compliance status**: Supporting authority for climate-dependent canopy melt.

## R-51: Varhola et al. (2010) forest canopy snow review

**Citation**: Varhola, A., N. C. Coops, M. Weiler, and R. D. Moore (2010). *Forest canopy effects on snow accumulation and ablation: An integrative review of empirical results*. Journal of Hydrology, 392, 219–233. https://doi.org/10.1016/j.jhydrol.2010.08.009
**Local path**: `references/copyrighted/varhola2010.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (Elsevier; operator-supplied local cache).
**Topic**: Empirical synthesis of canopy effects; ablation is more canopy-sensitive than accumulation (canopy change explained 72% of ablation-rate variance vs 57% for accumulation).
**Key equations / concepts**: Quantitative canopy-density vs melt/accumulation relations for parameterizing forest attenuation without site calibration.
**Kernel mapping**: Forest canopy attenuation (`docs/planning/snow-frost-fidelity-strategy.md` §5).
**Notes / caveats**: Review of empirical regressions, not a mechanistic authority.
**OAR-6 compliance status**: Supporting authority for canopy attenuation magnitudes.

## R-52: WEPP Ch. 3 Winter Hydrology (Savabi et al., NSERL-10)

**Citation**: Savabi, M. R., R. A. Young, G. R. Benoit, J. M. Witte, and D. C. Flanagan (1995). *Chapter 3: Winter Hydrology*, in Flanagan, D. C. and M. A. Nearing (eds.), USDA Water Erosion Prediction Project (WEPP) Hillslope Profile and Watershed Model Documentation, NSERL Report No. 10, USDA-ARS National Soil Erosion Research Laboratory.
**Local path**: `references/50201000/chap3.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `public-domain` (U.S. Government work, USDA-ARS NSERL).
**Topic**: WEPP's winter component — the **production CoE energy-balance snowmelt** that openWEPP ports, plus snow accumulation, settling/density, and frost.
**Key equations / concepts**: `hrmelt = 0.0254 (amelt − bmelt + cmelt + dmelt)`; radiation term `amelt = 0.0607 hrrad (1 − cancov)` then `amelt(0.36 Thr + 1.0)`; turbulent term `cmelt = 0.0188 U (1 − 0.8 cancov)(0.396 Thr + 1.404 hrdew)…`; net radiation `Rnet`; snow settling factor and density (settled-snow ≤ 350) tracking. **No degree-day factor** — confirms the production melt is energy-balance. Modernization target: drive `hrrad` with gridded shortwave + an albedo state, keep `(1 − cancov)`.
**Kernel mapping**: Melt-modernization decision and the `amelt`/`cmelt` production lineage (`docs/planning/snow-frost-fidelity-strategy.md` §1/§2/§5; `crates/openwepp-hillslope-orchestrator/src/hydrology/`).
**Notes / caveats**: The `0.0607`/`0.0188` coefficients lump radiation/turbulent transfer; the modernization replaces the `hrrad` *source*, not the energy-balance structure. Other chapters `chap1.pdf`..`chap14.pdf` are co-located in `references/50201000/`.
**OAR-6 compliance status**: Primary authority for the production WEPP energy-balance melt.

## R-53: Jennings et al. (2018) rain–snow temperature threshold across the NH

**Citation**: Jennings, K. S., T. S. Winchell, B. Livneh, and N. P. Molotch (2018). *Spatial variation of the rain–snow temperature threshold across the Northern Hemisphere*. Nature Communications, 9, 1148. https://doi.org/10.1038/s41467-018-03629-7
**Local path**: article at `references/vendorable/Jennings2018_NatComm.pdf`;
Dryad dataset at
`tests/fixtures/precip_phase_observed/jennings2018/`, with the 1.2 GB hourly
CSV tracked by Git LFS.
**Reference quality**: `verified-primary`
**Distribution status**: `redistributable` (article: Nature Communications,
CC-BY 4.0; Dryad dataset DOI `10.5061/dryad.c9h35`: CC0).
**Topic**: Observed precipitation-phase dataset and a temperature + relative-humidity method for partitioning rain vs snow — the "observed data" basis for replacing a single tuned air-temperature threshold (e.g. WEPP `RST`).
**Key equations / concepts**: ~17.8M Northern-Hemisphere land-station phase observations; the 50% rain/snow **air-temperature** threshold averages 1.0 °C and ranges −0.4 to 2.4 °C (95%), varying with **relative humidity** (humid maritime low, dry continental high) — so a spatially uniform air-temp threshold is structurally wrong. A binary-logistic phase method on temperature + RH outperforms it. Note: Mariana's `RST=-2 °C` for Oregon/DAYMET sits *below* this observed air-temp range, indicating it also corrects the DAYMET **daily-mean** resolution, not only humidity.
**Kernel mapping**: SNOWDENSITY-10.3.4 partition-first disposition; the `10.3.5` partition/thaw candidate (`docs/planning/snow-frost-fidelity-strategy.md` §10.2/§10.3). Complements SMRF's dew-point `Susong1999` partition.
**Notes / caveats**: The Dryad dataset is installed under `tests/fixtures/`.
The exact hourly file is dataset version 1, published 2019-01-31,
`1,206,721,342` bytes, SHA-256
`0cc82fbc5211c2c24b19653c4711d63a88fc4ed7bd90fc39cce84913d071f3a1`,
and is tracked through Git LFS. Dryad identifies it as a cleaned/formatted
version of UCAR RDA `ds464.0`; cite the original dataset as well. The corpus was
used for phase-method validation and is not independent validation for claims
that were selected or tuned using it.
**OAR-6 compliance status**: Primary authority for observed-data rain/snow partition.

## R-54: Susong, Marks & Garen (1999) dew-point precipitation-phase method

**Citation**: Susong, D., D. Marks, and D. Garen (1999). *Methods for developing time-series climate surfaces to drive topographically distributed energy- and water-balance models*. Hydrological Processes, 13(12–13), 2003–2021. https://doi.org/10.1002/(SICI)1099-1085(199909)13:12/13<2003::AID-HYP884>3.0.CO;2-K
**Local path**: `references/copyrighted/source_pdfs/susong1999.pdf` (duplicate `susong1999-2.pdf` may be removed)
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (Wiley; operator-supplied local cache).
**Topic**: The SNOBAL/SMRF default precipitation-phase + new-snow-density method — the `Susong1999` NASDE model ("follows the IPW command `mkprecip`").
**Key equations / concepts**: A **dew-point** lookup table mapping precipitation (dew-point) temperature → fractional percent-snow and new-snow density (transition −0.5 to +0.5 °C dew-point: 100% / 75% / 25% / 0% snow; ρ 75–250 kg/m³). Humidity-aware and fractional — the SNOBAL baseline above WEPP's single-air-temp `RST`.
**Kernel mapping**: rain/snow partition candidate for SNOWDENSITY-10.3.5 (`docs/planning/snow-frost-fidelity-strategy.md` §10.2/§10.3); the SMRF method openWEPP would compare against.
**Notes / caveats**: A parameterized table, not a hydrometeor energy balance; the more physical option is Harder & Pomeroy (R-57).
**OAR-6 compliance status**: Primary authority for the SNOBAL/SMRF dew-point partition.

## R-55: Marks, Kimball, Tingey & Link (1998) rain-on-snow, 1996 PNW flood

**Citation**: Marks, D., J. Kimball, D. Tingey, and T. Link (1998). *The sensitivity of snowmelt processes to climate conditions and forest cover during rain on snow: a case study of the 1996 Pacific Northwest flood*. Hydrological Processes, 12(10–11), 1569–1587.
**Local path**: `references/copyrighted/source_pdfs/marks1998.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (Wiley; operator-supplied local cache).
**Topic**: Energy-balance analysis of rain-on-snow melt and forest-cover effects during the 1996 PNW flood — **Oregon Cascades** sites (Hogg Pass / McKenzie), directly analogous to the `hjandrews_conifer_or` maritime regime and the 10.3.4 maritime over-accumulation/thaw blocker.
**Key equations / concepts**: Rain-on-snow advective + turbulent melt partitioning; the dominance of net radiation + sensible/latent fluxes over rain-heat in maritime RoS; forest-cover modulation. Context for the 10.3.4 thaw/rain-on-snow mechanism ranking.
**Kernel mapping**: SNOWDENSITY-10.3.4 maritime diagnosis; partition/thaw work (§10.2 items 3–4).
**Notes / caveats**: DOI not printed in the PDF (1998); SICI DOI `10.1002/(SICI)1099-1085(199808/09)12:10/11<1569::AID-HYP682>3.0.CO;2-L` is recalled — verify on retrieval. PDF is the authority.
**OAR-6 compliance status**: Supporting authority for maritime rain-on-snow melt physics.

## R-56: Kormos et al. (2014) mountain rain-snow transition zone

**Citation**: Kormos, P. R., D. Marks, J. P. McNamara, H. P. Marshall, A. Winstral, and A. N. Flores (2014). *Snow distribution, melt and surface water inputs to the soil in the mountain rain-snow transition zone*. Journal of Hydrology, 519 (Part D), 190–204. https://doi.org/10.1016/j.jhydrol.2014.06.051
**Local path**: `references/copyrighted/source_pdfs/kormos2014.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (Elsevier; operator-supplied local cache).
**Topic**: Observations and energy-balance modeling in the **mountain rain-snow transition zone** — the regime where phase partition and intermittent melt dominate (the openWEPP maritime blocker regime).
**Key equations / concepts**: Transition-zone snowpack intermittency, surface-water-input timing, and the sensitivity to phase partition; supports the 10.3.4 partition-first finding.
**Kernel mapping**: SNOWDENSITY-10.3.4/10.3.5 (§10.2/§10.3).
**Notes / caveats**: Site-specific (Dry Creek, ID) but the transition-zone process framing transfers.
**OAR-6 compliance status**: Supporting authority for rain-snow transition-zone behavior.

## R-57: Harder & Pomeroy (2013) psychrometric precipitation-phase method

**Citation**: Harder, P., and J. Pomeroy (2013). *Estimating precipitation phase using a psychrometric energy balance method*. Hydrological Processes, 27(13), 1901–1914. https://doi.org/10.1002/hyp.9799
**Local path**: `references/copyrighted/source_pdfs/harder2013.pdf`
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (Wiley; operator-supplied local cache).
**Topic**: The **most physically-based** rain/snow partition — a psychrometric energy balance solving the falling hydrometeor's (ice-bulb) temperature from air temperature + humidity, then partitioning on that physical temperature.
**Key equations / concepts**: Iterative hydrometeor energy/mass balance (sensible + latent exchange of the falling particle); needs only air temp + humidity (both available hourly in openWEPP's winter routine); no site calibration; generalizes by construction (a law, not a fit or table). **The recommended primary method for the SNOWDENSITY-10.3.5 robust partition**, with Susong (R-54) as fallback and Jennings (R-53) as the observed-phase validation set.
**Kernel mapping**: SNOWDENSITY-10.3.5 robust rain/snow partition (`docs/planning/snow-frost-fidelity-strategy.md` §10.2/§10.3).
**Notes / caveats**: Per-event/sub-daily method — apply at openWEPP's existing hourly partition resolution (`snow.hourly.stmtim.rst_c` lineage), not a daily mean.
**OAR-6 compliance status**: Primary candidate authority for a physics-based, calibration-free rain/snow partition.

## R-58: Sturm, Taras, Liston et al. (2010) SWE from snow depth + climate classes

**Citation**: Sturm, M., B. Taras, G. E. Liston, C. Derksen, T. Jonas, and J. Lea (2010). *Estimating Snow Water Equivalent Using Snow Depth Data and Climate Classes*. Journal of Hydrometeorology, 11(6), 1380–1394. https://doi.org/10.1175/2010JHM1202.1
**Local path**: `references/copyrighted/sturm2010_swe_climate_classes.pdf` (author open copy, morageology.com/pubs/296.pdf, downloaded 2026-06-28)
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (AMS journal copyright; author open copy cached locally, not redistributable).
**Topic**: **Regime-divergent (climate-class) snow bulk density** — the canonical Paradigm-1 reference. Bulk density = f(**snow depth, day-of-year, snow climate class**) with *class-specific* densification parameters.
**Key equations / concepts**: ρ(h, DOY) relaxes from ρ0 toward ρmax via class-specific **k1** (depth densification) and **k2** (DOY densification); the six snow classes are Sturm 1995 (alpine / maritime / prairie / tundra / taiga / ephemeral); Bayesian fit to 25,688 depth–density–SWE observations. Direct authority for a **regime-divergent densification trajectory** — the openWEPP cluster-1 split-sign density residual (10.3.21: over-densified at humid-forest, under-densified at mountain).
**Kernel mapping**: Paradigm 1 of the snow-density paradigm assessment (`docs/work-packages/.../snow-density-paradigm-assessment`); post-10.3.21 density-structure decision (§10.2/§10.3).
**Notes / caveats**: It is an **empirical depth+DOY+class regression**, not a process model — under ADR-0028 the clean adoption is to make the existing Anderson/SNOBAL densification *coefficients* regime-adaptive by class, never to fit class params to our SNOTEL/cancov fixtures.
**Paradigm contrast (authors' self-assessment; the openWEPP fork)**: §5 of the paper explicitly frames the class model as the *easier*, not the *better*, option. Lineage of the class approach: McKay & Findlay (1971), McKay & Gray (1981), confirmed by Sturm & Holmgren (1998). "The alternative would be to explicitly model compaction processes, as has been done in several physically based snow models (cf. **Anderson 1976; Koren et al. 1999; Liston et al. 2007**; … Rutter et al. 2009 …). The problem is that these physical models require high-quality daily or even hourly weather and snowfall data because they must **track individual snow layer settlement through time**." Quasi-physical time-since-deposition models (Martinec 1966; Elder et al. 1991; Sturm & Holmgren 1998) are "too complex and too computationally intensive … globally or regionally." Conclusion: "our model is general and **potentially less accurate than explicit locally applied physical models**, [but] far easier to apply globally or regionally." **openWEPP implication:** the class model exists to dodge the global-applicability data/compute constraint, which openWEPP (per-hillslope, hourly forcing) does **not** have — so by the Paradigm-1 authors' own ranking, the explicit layer-settlement model (Paradigm 2, ADR-0029) is the accuracy path, and openWEPP is in its applicable regime. Names the Paradigm-2 reference lineage (Anderson 1976 / Koren 1999 / Liston 2007).
**OAR-6 compliance status**: Primary authority for regime-divergent (climate-class) snow density.

## R-59: Sturm, Holmgren & Liston (1995) seasonal snow-cover classification

**Citation**: Sturm, M., J. Holmgren, and G. E. Liston (1995). *A Seasonal Snow Cover Classification System for Local to Global Applications*. Journal of Climate, 8(5), 1261–1283. https://doi.org/10.1175/1520-0442(1995)008<1261:ASSCCS>2.0.CO;2
**Local path**: `references/copyrighted/sturm1995.pdf` (operator-supplied, 2026-06-28).
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (AMS journal copyright; local cache).
**Topic**: The six-class seasonal snow classification (tundra / taiga / alpine / maritime / prairie / ephemeral) underlying Sturm 2010's density-by-class — and the classification driver for Paradigm 1. **This is the binding authority for SNOWDENSITY-10.3.22's `HOLD-AUTHORITY-GAP`** (the numeric decision thresholds were missing); now acquired.
**Key equations / concepts**: A **binary decision tree on three climate variables — wind, precipitation, air temperature** — assigns the class; each class carries a distinct density / stratigraphy / grain-morphology signature; NH distribution mapped on a 0.5° grid. The {wind, precip, air-temp} drivers are quantities openWEPP already has. The numeric class thresholds are the authority openWEPP must use for forcing-derived class assignment (paired with Sturm 2010 Table 4 density params; ephemeral excluded by Sturm 2010 → fresh-snow fallback).
**Kernel mapping**: Paradigm 1 classification driver; SNOWDENSITY-10.3.22 re-run unblock.
**Notes / caveats**: Class assignment is **climatological** (multi-year normals of cooling-degree-months / precip / wind), not single-day. Modern successors: NSIDC-0768 user guide (R-60) and Sturm & Liston 2021 (R-61, renamed Alpine→Montane Forest, Taiga→Boreal Forest). Still worth pulling: *"Differences in compaction behavior of three climate classes of snow"* (Annals of Glaciology — regime-divergent compaction, cluster-1).
**OAR-6 compliance status**: Classification authority for regime-divergent snow modeling.

## R-60: NSIDC-0768 Global Seasonal-Snow Classification v1 — User Guide (Liston/Sturm)

**Citation**: Liston, G. E., and M. Sturm (2021). *Global Seasonal-Snow Classification, Version 1* (NSIDC-0768) User Guide. NASA National Snow and Ice Data Center DAAC, Boulder, CO. https://nsidc.org/data/nsidc-0768/versions/1
**Local path**: `references/vendorable/NSIDC-0768_GlobalSeasonalSnowClassification_v1_UserGuide.pdf` (downloaded 2026-06-28)
**Reference quality**: `verified-primary`
**Distribution status**: `redistributable` (NSIDC/NASA DAAC open data; citation-on-use requested — vendorable).
**Topic**: The operational **gridded** classification algorithm (Liston/Sturm update of Sturm 1995), driven by air-temperature, precipitation, and wind-speed climatologies.
**Key equations / concepts**: Confirms the three classification drivers and the threshold *types* (a cooling-degree-month threshold delineates ephemeral; a high/low water-equivalent snowfall-rate threshold; wind). Renames Alpine→Montane Forest, Taiga→Boreal Forest. Documents the algorithm at overview level — the full numeric tree is in Sturm 1995 (R-59) / Sturm & Liston 2021 (R-61). The gridded product itself is a lat/lon→class lookup (geographic); openWEPP prefers forcing-derived assignment from the run's own climate.
**Kernel mapping**: Paradigm 1 supporting authority; SNOWDENSITY-10.3.22 (cross-check / variable confirmation).
**Notes / caveats**: Public-domain-leaning NSIDC/NASA open data with a citation requirement → `vendorable/` (committed). The user-guide PDF is the doc; the gridded data array is a separate large product, not pulled.
**OAR-6 compliance status**: Supporting authority (classification algorithm + drivers).

## R-61: Sturm & Liston (2021) revisited global seasonal snow classification

**Citation**: Sturm, M., and G. E. Liston (2021). *Revisiting the Global Seasonal Snow Classification: An Updated Dataset for Earth System Applications*. Journal of Hydrometeorology, 22(11), 2917–2938. https://doi.org/10.1175/JHM-D-21-0070.1
**Local path**: `references/copyrighted/hydr-JHM-D-21-0070.1.pdf` (operator-supplied, 2026-06-28)
**Reference quality**: `verified-primary`
**Distribution status**: `restricted` (AMS journal copyright; local cache).
**Topic**: The updated, higher-resolution global classification with **explicit revised thresholds** — the modern companion to Sturm 1995's tree.
**Key equations / concepts**: Updated decision thresholds on the same three drivers (temperature/precip/wind climatologies); **renames Alpine→Montane Forest and Taiga→Boreal Forest** (a remap is required to pair with Sturm 2010 Table 4, which uses the 1995 names). Useful as a cross-check on the Sturm 1995 thresholds and for global generality.
**Kernel mapping**: Paradigm 1 alternative/cross-check threshold authority; SNOWDENSITY-10.3.22.
**Notes / caveats**: For openWEPP, Sturm 1995 (R-59) is the cleaner pairing (matching class names); use 2021 to validate/extend the thresholds, with name-mapping documented.
**OAR-6 compliance status**: Updated classification-threshold authority.

## R-62: Brooks, Boll & McDaniel (2004) hillslope-scale lateral saturated hydraulic conductivity

**Citation**: Brooks, E. S., J. Boll, and P. A. McDaniel (2004). *A hillslope-scale experiment to measure lateral saturated hydraulic conductivity*. Water Resources Research, 40, W04208. https://doi.org/10.1029/2003WR002858
**Local path**: `references/copyrighted/brooks2004.pdf` (local-only copyrighted cache).
**Reference quality**: `verified-primary`
**Topic**: Direct hillslope-scale (18 × 35 m plot) measurement of lateral saturated hydraulic conductivity above a fragipan restrictive layer (eastern Palouse, Troy, ID), via perched water-level and drain-tile outflow analysis (Parlange et al. 1989 methodology + Childs 1971 sloping-bed solution).
**Key equations / concepts for MOFEFID Lane C**:
- `[DIRECT]` Hillslope-scale lateral `Ks` measured **13.7× / 4.1× / 3.2×** larger than small-core `Ks` in the A / B / E horizons respectively — direct field evidence that core-derived vertical `Ks` understates effective lateral conductivity at the modeling scale.
- `[DIRECT]` `Ks(depth)` best described by a **double-exponential** decline (sharp drop in the first 0.1 m, exponential below); the transmissivity-profile *shape* matters as much as its magnitude.
- `[DIRECT]` Macroporosity identified as the dominant control on hillslope-scale lateral `Ks`; the small-core vs hillslope gap framed as a **measurement-scale problem**, not parameter error.
- `[INFERENCE]` Bears directly on the H2637 71% `runvol` lateral-magnitude flag: physically defensible lateral/vertical effective-conductivity ratios of order 3–14× at hillslope scale bound what the provisional forest `ksatadj`/anisotropy inputs may legitimately encode.
- `[INFERENCE]` Author lineage (Brooks/Boll, U. Idaho) is the same lineage behind WEPP's forest lateral-flow adaptations (Dun et al. 2009, R-21; Srivastava 2013, R-22) — this is the parameter-scale companion to those model papers.
**Kernel mapping**: WB19 lateral flow (`latqcc`), `SC-SUBHYD-001` conductivity anchors; MOFEFID Lane C envelope derivation (`[INFERENCE]`).
**Notes / caveats**: Single site, agricultural-region fragipan soil (not forest); relevance to H2637-like forest hillslopes is via the scale-gap mechanism and the anisotropy bound, not a site-transferable `Ks` value. Applicability mapping is Lane C1 work.
**OAR-6 compliance status**: Primary field authority; combines with site-specific observed datasets (`tests/fixtures/forest_lateral_flow_authority/`) rather than replacing them.

## R-63: Papanicolaou et al. (2018) space/time-variant flow resistance on heterogeneous hillslopes

**Citation**: Papanicolaou, A. N., B. K. B. Abban, D. C. Dermisis, C. P. Giannopoulos, D. C. Flanagan, J. R. Frankenberger, and K. M. Wacha (2018). *Flow resistance interactions on hillslopes with heterogeneous attributes: Effects on runoff hydrograph characteristics*. Water Resources Research, 54, 359–380. https://doi.org/10.1002/2017WR021109
**Local path**: `references/copyrighted/Papanicolaou2018.pdf` (+ full-text markdown `Papanicolaou2018.md`; supplemental data `references/copyrighted/Papanicolaou2018-supplemental/` — validation-case input docx + Figure 4–9 xlsx series).
**Reference quality**: `verified-primary`
**Topic**: Enhanced-WEPP framework routing overland flow OFE-by-OFE (removing the equivalent-plane/equilibrium-storage aggregation) with additive, per-timestep friction factors for grain/raindrop, form, wave, and vegetation resistance, and TVD-MacCormack shock-capturing kinematic-wave solution; stream-power Zone 1/Zone 2 taxonomy for when roughness detail matters.
**Key equations / concepts for MOFEFID Lane D**:
- `[DIRECT]` Friction-factor menu eqs. (2)–(6): skin (Shen & Li `Re<1000`; Hirsch `Re>1000`), form (Abrahams/Lawrence), wave (Hu & Abrahams, `Fr`-gated), vegetation (Katul/Thompson), additive `f_eq` (eq. 7).
- `[DIRECT]` TVD-MacCormack predictor/corrector scheme eqs. (8)–(14) with CFL condition (eq. 12) for the 1-D KWE (Appendix A).
- `[DIRECT]` Four validation cases with published `Ef`: bare 0.91, rock fragments 0.75, vegetation patchiness 0.87, concave/shock (Iwagaki) 0.88 — the Lane D acceptance targets; supplemental carries the series.
- `[DIRECT]` Stream-power threshold taxonomy (`Ψ* = q*·S₀*` vs `I*`, Zone 1 nonlinear / Zone 2 linear) — the scoping instrument for when OFE-by-OFE routing changes answers.
- `[INFERENCE]` WEPP-lineage authority (Flanagan, Frankenberger co-authors); the equivalent-plane limitation it removes is the representation openWEPP inherited via its legacy anchor.
**Kernel mapping**: MOFEFID Lane D (`SC-OFEROUTE` candidate contracts, per-OFE hydrograph transfer over the `INV-RUNOFFPART-029` seam).
**Notes / caveats**: Copyrighted (AGU/WRR) — in-repo for internal validation only; supplemental-derived fixtures follow the campaign §8 governance. Original WEPP v2012.8 basis; friction formulations are semitheoretical with stated applicability regimes (`Re`, `Fr`, submergence).
**OAR-6 compliance status**: Primary physics authority for Lane D; primary-source acquisitions for eqs. (2)–(6) originals tracked separately.

## R-64: Tromp-van Meerveld & McDonnell (2006) Panola 147-storm threshold analysis

**Citation**: Tromp-van Meerveld, H. J., and J. J. McDonnell (2006). *Threshold relations in subsurface stormflow: 1. A 147-storm analysis of the Panola hillslope*. Water Resources Research, 42, W02410. https://doi.org/10.1029/2004WR003778
**Local path**: `references/copyrighted/TrompvanMeerveld_McDonnell2006_panola_threshold_1.pdf` (author-archive copy; title verified).
**Reference quality**: `verified-primary`
**Topic**: 147-storm record from the Panola trenched hillslope: subsurface stormflow is threshold-gated (~55 mm event precipitation), with roughly two orders of magnitude more lateral flow above the threshold.
**MOFEFID Lane C role**: the richest per-event lateral-flow distribution for a site whose observed data we hold (`tests/fixtures/forest_lateral_flow_authority/panola_pmrw_2002/`); primary evidence that event lateral fraction is nonlinear/threshold-shaped, so the acceptance envelope must be conditioned on event size and antecedent state, not a fixed ratio.
**Kernel mapping**: Lane C1 envelope derivation; WB19 lateral magnitude judgment (`[INFERENCE]`).
**Notes / caveats**: Deep annotation (key equations/threshold parameters) deferred to Lane C1 close reading — only abstract/first page verified at intake.

## R-65: Tromp-van Meerveld & McDonnell (2006) fill-and-spill hypothesis

**Citation**: Tromp-van Meerveld, H. J., and J. J. McDonnell (2006). *Threshold relations in subsurface stormflow: 2. The fill and spill hypothesis*. Water Resources Research, 42, W02411. https://doi.org/10.1029/2004WR003800
**Local path**: `references/copyrighted/TrompvanMeerveld_McDonnell2006_fill_and_spill_2.pdf` (author-archive copy; title verified).
**Reference quality**: `verified-primary`
**Topic**: Mechanistic companion to R-64: bedrock-depression storage must fill before downslope connectivity establishes ("fill and spill"); connected conditions deliver >75× more subsurface flow.
**MOFEFID Lane C role**: mechanism authority for the threshold shape in the Panola envelope; cautions that a smooth-restrictive-layer model (WEPP-style) will not reproduce the connectivity discontinuity — an expected structural divergence to declare, not a defect signal.
**Kernel mapping**: Lane C1 applicability limits (`[INFERENCE]`).
**Notes / caveats**: Intake-level annotation; deep read in Lane C1.

## R-66: Freer et al. (2002) bedrock topography control on subsurface stormflow

**Citation**: Freer, J., J. J. McDonnell, K. J. Beven, N. E. Peters, D. A. Burns, R. P. Hooper, B. Aulenbach, and C. Kendall (2002). *The role of bedrock topography on subsurface storm flow*. Water Resources Research, 38(12), 1269. https://doi.org/10.1029/2001WR000872
**Local path**: `references/copyrighted/Freer2002_panola_bedrock_topography.pdf` (author-archive copy; title verified).
**Reference quality**: `verified-primary`
**Topic**: Panola trench + digital terrain analysis: subsurface flow delivery is organized by bedrock-surface topography, not ground-surface topography.
**MOFEFID Lane C role**: spatial-organization caveat for envelope construction — trench-section observations sample an impeding-surface flow net, so site-to-model mapping must aggregate to scales where that organization averages out.
**Kernel mapping**: Lane C1 applicability limits (`[INFERENCE]`).
**Notes / caveats**: Intake-level annotation.

## R-67: McGuire & McDonnell (2010) WS10 hillslope-stream connectivity

**Citation**: McGuire, K. J., and J. J. McDonnell (2010). *Hydrological connectivity of hillslopes and streams: Characteristic time scales and nonlinearities*. Water Resources Research, 46, W10543. https://doi.org/10.1029/2010WR009341
**Local path**: `references/copyrighted/McGuire_McDonnell2010_ws10_connectivity.pdf` (USDA Treesearch scan; title verified).
**Reference quality**: `verified-primary`
**Topic**: HJ Andrews WS10 hillslope response analysis: threshold-linear behavior with an average hillslope quick-flow ratio of ~0.58 above ~20 mm antecedent rainfall.
**MOFEFID Lane C role**: **highest-value envelope anchor** — an explicit event lateral-flow ratio with threshold conditioning for the exact hillslope whose observed record we hold (`hjandrews_ws10_hf024/`); candidate central value for the primary Lane C site.
**Kernel mapping**: Lane C1 envelope derivation; H2637 applicability mapping (`[INFERENCE]`).
**Notes / caveats**: Ratio/threshold numbers above are from the research-agent read; Lane C1 must re-derive them from the paper + HF024 data before they enter the envelope.

## R-68: Weiler, McDonnell, Tromp-van Meerveld & Uchida (2005) subsurface stormflow synthesis

**Citation**: Weiler, M., J. J. McDonnell, H. J. Tromp-van Meerveld, and T. Uchida (2005). *Subsurface Stormflow* (Ch. 112). In M. G. Anderson (ed.), Encyclopedia of Hydrological Sciences, Vol. 3, 1719–1732. Wiley. https://doi.org/10.1002/0470848944.hsa119
**Local path**: `references/copyrighted/Weiler2005_subsurface_stormflow_encyclopedia.pdf` (free-access Wiley copy; title verified).
**Reference quality**: `verified-primary`
**Topic**: Cross-site synthesis of trench/hillslope subsurface-stormflow studies: mechanisms, controls, and observed magnitudes.
**MOFEFID Lane C role**: the cross-site assembly source — lets single-site numbers (R-64, R-67, Maimai, Panola) be combined into one envelope with defensible spread.
**Kernel mapping**: Lane C1 envelope derivation (`[INFERENCE]`).
**Notes / caveats**: Intake-level annotation.

## R-69: Blume & van Meerveld (2015) subsurface connectivity methods review

**Citation**: Blume, T., and H. J. van Meerveld (2015). *From hillslope to stream: methods to investigate subsurface connectivity*. WIREs Water, 2(3), 177–198. https://doi.org/10.1002/wat2.1071
**Local path**: `references/copyrighted/Blume_vanMeerveld2015_subsurface_connectivity_methods.pdf` (GFZ repository copy; title verified).
**Reference quality**: `verified-primary`
**Topic**: Methods review for measuring/interpreting subsurface lateral connectivity and flow.
**MOFEFID Lane C role**: measurement-artifact guard for the rubric — how trench/well observations can mislead when compared to model output (the Lane C analog of the comparator like-for-like discipline).
**Kernel mapping**: Lane C2 rubric design (`[INFERENCE]`).
**Notes / caveats**: Intake-level annotation.

## R-70: Srivastava et al. (2017) WEPP streamflow with baseflow, snow-dominated forest watershed

**Citation**: Srivastava, A., J. Q. Wu, W. J. Elliot, E. S. Brooks, and D. C. Flanagan (2017). *Modeling streamflow in a snow-dominated forest watershed using the Water Erosion Prediction Project (WEPP) model*. Transactions of the ASABE, 60(4), 1171–1187. https://doi.org/10.13031/trans.12035
**Local path**: `references/copyrighted/Srivastava2017_ToASABE_wepp_streamflow.pdf`
**Reference quality**: `verified-primary`
**Topic**: Adds nonlinear groundwater baseflow to WEPP v2012.8 and evaluates streamflow on an Upper Cedar River (PNW) subwatershed; WEPP-Cur vs WEPP-Mod NSE 0.55→0.76.
**MOFEFID Lane C role**: the WEPP-lineage calibration precedent for partitioning lateral flow vs baseflow at watershed scale — context for what magnitude the lateral channel is *expected* to carry when baseflow is represented vs absent (bears on interpreting H2637's 71% routed-lateral share).
**Kernel mapping**: WB19/baseflow boundary; Lane C3 interpretation (`[INFERENCE]`).
**Notes / caveats**: Extends the held R-21 (Dun 2009) and R-22 (Srivastava 2013 dissertation) lineage.

## R-71: Pirastru et al. (2017) lateral saturated conductivity of soil horizons in large monoliths

**Citation**: Pirastru, M., R. Marrosu, S. Di Prima, S. Keesstra, F. Giadrossich, and M. Niedda (2017). *Lateral saturated hydraulic conductivity of soil horizons evaluated in large-volume soil monoliths*. Water, 9(11), 862. https://doi.org/10.3390/w9110862
**Local path**: `references/copyrighted/Pirastru2017_lateral_ks_monoliths.pdf` (MDPI, gold OA CC-BY; title verified).
**Reference quality**: `verified-primary`
**Topic**: In-situ ~0.12 m³ monolith drainage experiments measuring horizon lateral Ks on a shallow hillslope: median 2450 mm/h (A) and 552 mm/h (B), halving near the restrictive layer; consistent with drain-data hillslope-scale values — macropore network captured at monolith scale.
**MOFEFID Lane C role**: independent, freely-licensed cross-check on the R-62 (Brooks 2004) scale argument and on lateral-Ks magnitudes over restrictive layers; CC-BY license permits vendoring if needed.
**Kernel mapping**: Lane C1 conductivity envelope (`[INFERENCE]`).
**Notes / caveats**: Mediterranean hillslope; applicability mapping needed like all sites.

## R-72: Hu & Abrahams (2006) partitioning resistance to overland flow

**Citation**: Hu, S., and A. D. Abrahams (2006). *Partitioning resistance to overland flow on rough mobile beds*. Earth Surface Processes and Landforms, 31(10), 1280–1291. https://doi.org/10.1002/esp.1333
**Local path**: `references/copyrighted/Hu_Abrahams2006_partitioning_resistance.pdf` (USDA Jornada bibliography copy; title verified).
**Reference quality**: `verified-primary`
**Topic**: Flume-derived partition of total overland-flow resistance into surface (grain), form, wave, and bed-mobility components; source of the Froude-gated wave-resistance regression.
**MOFEFID Lane D role**: **formulation-tier** — primary source for Papanicolaou eq. (5) (`f_w = 3.32 λ / Fr^0.5`, `Fr > 0.5` regime) and for the additive-partition assumption (eq. 7); carries the unit conventions and regime bounds the implementation must honor.
**Kernel mapping**: Lane D1 contracts (wave-resistance invariants), D3 friction kernels.
**Notes / caveats**: Intake-level annotation; regime-bound extraction is D1 work.

## R-73: Wu, Yevjevich & Woolhiser (1978) surface roughness spatial distribution — CSU Hydrology Paper 96

**Citation**: Wu, Y.-H., V. Yevjevich, and D. A. Woolhiser (1978). *Effects of Surface Roughness and Its Spatial Distribution on Runoff Hydrographs*. Hydrology Paper No. 96, Colorado State University, Fort Collins, CO.
**Local path**: `references/copyrighted/Wu_Yevjevich_Woolhiser1978_CSU_HP96_equivalent_plane.pdf` (USDA-ARS Tucson archive scan, 57 pp.; title verified).
**Reference quality**: `verified-primary`
**Topic**: The equivalent-plane / equilibrium-storage treatment of spatially distributed roughness for runoff hydrographs.
**MOFEFID Lane D role**: **the legacy baseline** — this is the representation WEPP's original overland routing inherited and that the OFE-by-OFE enhancement replaces; the Lane D ADR must characterize legacy behavior against this source (ADR-0024 source-intent anchor for the *old* routing).
**Kernel mapping**: Lane D1 ADR (representation decision); comparator-hygiene reference.
**Notes / caveats**: Public USDA-ARS-hosted scan.

## R-74: Iwagaki (1955) runoff analysis by characteristics — DPRI Bulletin 10

**Citation**: Iwagaki, Y. (1955). *Fundamental Studies on the Runoff Analysis by Characteristics*. Bulletins — Disaster Prevention Research Institute, Kyoto University, No. 10, 1–25.
**Local path**: `references/copyrighted/Iwagaki1955_runoff_characteristics_DPRI10.pdf` (KURENAI open-access repository copy, hdl:2433/123659; 28 pp.).
**Reference quality**: `verified-primary`
**Topic**: Method-of-characteristics kinematic-wave solutions on a cascade of planes, with the three-section laterally-fed flume experiments (2%/1.5%/1% gradients).
**MOFEFID Lane D role**: validation Case 4 primary source — the shock-formation dataset (the near-vertical rising limb at ~23 s) that tests the TVD-MacCormack scheme's shock capture; Papanicolaou's Ef 0.88 target is against this data.
**Kernel mapping**: Lane D2 fixtures (Case 4), D-val acceptance.
**Notes / caveats**: Scanned document; OCR quality limited — numeric extraction should prefer the supplemental `Figure_4.xlsx` series with this as provenance.

## R-75: Abban et al. (2017) rainfall-driven microroughness change

**Citation**: Abban, B. K. B., A. N. Papanicolaou, C. P. Giannopoulos, D. C. Dermisis, K. M. Wacha, C. G. Wilson, and M. Elhakeem (2017). *Quantifying the changes of soil surface microroughness due to rainfall impact on a smooth surface*. Nonlinear Processes in Geophysics, 24(3), 569–579. https://doi.org/10.5194/npg-24-569-2017
**Local path**: `references/copyrighted/Abban2017_microroughness_rainfall_NPG.pdf` (Copernicus, gold OA CC-BY 3.0; DOI verified from PDF).
**Reference quality**: `verified-primary`
**Topic**: Field quantification of sub-5 mm soil-surface microroughness evolution under rainfall.
**MOFEFID Lane D role**: validation Case 1 provenance (the bare-surface plot experiments) and the empirical basis for treating grain roughness as time-variant; CC-BY license permits vendoring.
**Kernel mapping**: Lane D2 fixtures (Case 1).
**Notes / caveats**: Intake-level annotation.

## R-76: Helmers et al. (2012) prairie filter strips, Walnut Creek

**Citation**: Helmers, M. J., X. Zhou, H. Asbjornsen, R. Kolka, M. D. Tomer, and R. M. Cruse (2012). *Sediment removal by prairie filter strips in row-cropped ephemeral watersheds*. Journal of Environmental Quality, 41(5), 1531–1539. https://doi.org/10.2134/jeq2011.0473
**Local path**: `references/copyrighted/Helmers2012_prairie_filter_strips.pdf` (USDA Treesearch copy; title verified).
**Reference quality**: `verified-primary`
**Topic**: Hillslope-scale prairie/vegetated-strip observations in Walnut Creek, IA (Neal Smith NWR).
**MOFEFID Lane D role**: the field hillslope behind Papanicolaou §3.2 (the 65% space/time-invariant underprediction case) — the heterogeneous-OFE, S-profile validation target for the full OFE-by-OFE routing.
**Kernel mapping**: Lane D2 fixtures (§3.2 case), D-val.
**Notes / caveats**: Papanicolaou cites this for the observed hydrograph/hillslope configuration; the exact storm series is in the supplemental Figure_5.xlsx.

## R-77: Lawrence (1997) macroscale surface roughness and frictional resistance

**Citation**: Lawrence, D. S. L. (1997). *Macroscale surface roughness and frictional resistance in overland flow*. Earth Surface Processes and Landforms, 22(4), 365–382. https://doi.org/10.1002/(SICI)1096-9837(199704)22:4<365::AID-ESP693>3.0.CO;2-6
**Local path**: `references/copyrighted/lawrence1997.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: Three-regime (partially/marginally/well-inundated) frictional resistance as a function of inundation ratio and roughness concentration.
**MOFEFID Lane D role**: **formulation-tier** — primary source behind Papanicolaou eq. (4) form resistance; must be read together with the Abrahams (1998) discussion (still operator-tracked) for the contested applicability limits.
**Notes / caveats**: Intake-level annotation; regime-bound extraction is Lane D1 work.

## R-78: Katul, Poggi & Ridolfi (2011) vegetation flow resistance

**Citation**: Katul, G. G., D. Poggi, and L. Ridolfi (2011). *A flow resistance model for assessing the impact of vegetation on flood routing mechanics*. Water Resources Research, 47, W08533. https://doi.org/10.1029/2010WR010278
**Local path**: `references/copyrighted/Water Resources Research - 2011 - Katul - A flow resistance model for assessing the impact of vegetation on flood routing.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: Vegetation drag resistance from canonical length scales (adjustment length `L_c`, canopy height `h_c`, depth `h`), with momentum absorption coefficient `β`.
**MOFEFID Lane D role**: **formulation-tier** — primary source for Papanicolaou eq. (6) `f_veg` including the `β = min(0.135√(LAI/h_c), 0.33)` estimate.
**Notes / caveats**: Intake-level annotation.

## R-79: Jomaa et al. (2012) rock-fragment coverage flume experiments

**Citation**: Jomaa, S., D. A. Barry, B. C. P. Heng, A. Brovelli, G. C. Sander, and J.-Y. Parlange (2012). *Influence of rock fragment coverage on soil erosion and hydrological response: Laboratory flume experiments and modeling*. Water Resources Research, 48, W05535. https://doi.org/10.1029/2011WR011255
**Local path**: `references/copyrighted/jomaa2012.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: EPFL 6 m flume with controlled rock-fragment cover under 74 mm/h rainfall.
**MOFEFID Lane D role**: validation Case 2 primary source (isolated roughness elements, Ef 0.75 target).
**Notes / caveats**: Case series also in supplemental `Figure_4.xlsx`.

## R-80: Thompson et al. (2011) unsteady overland flow over permeability contrasts

**Citation**: Thompson, S., G. Katul, A. Konings, and L. Ridolfi (2011). *Unsteady overland flow on flat surfaces induced by spatial permeability contrasts*. Advances in Water Resources, 34(8), 1049–1058. https://doi.org/10.1016/j.advwatres.2011.05.012
**Local path**: `references/copyrighted/thompson2011.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: Unsteady overland-flow routing with the Katul resistance formulation over spatially heterogeneous surfaces.
**MOFEFID Lane D role**: companion application of eq. (6); the heterogeneous-surface routing pattern Papanicolaou generalizes OFE-by-OFE.
**Notes / caveats**: Intake-level annotation.

## R-81: García-Navarro, Alcrudo & Savirón (1992) TVD-MacCormack for 1-D open-channel flow

**Citation**: García-Navarro, P., F. Alcrudo, and J. M. Savirón (1992). *1-D open-channel flow simulation using TVD-McCormack scheme*. Journal of Hydraulic Engineering, 118(10), 1359–1372. https://doi.org/10.1061/(ASCE)0733-9429(1992)118:10(1359)
**Local path**: `references/copyrighted/10.1061@ASCE0733-94291992118@101359.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: TVD-corrected MacCormack predictor–corrector applied to 1-D Saint-Venant flow.
**MOFEFID Lane D role**: numerics lineage for Papanicolaou eqs. (8)–(14); source for the TVD-term construction and stability treatment.
**Notes / caveats**: Intake-level annotation.

## R-82: Mingham, Causon & Ingram (2001) TVD MacCormack for transcritical flow

**Citation**: Mingham, C. G., D. M. Causon, and D. M. Ingram (2001). *A TVD MacCormack scheme for transcritical flow*. Proceedings of the Institution of Civil Engineers — Water and Maritime Engineering, 148(3), 167–175. https://doi.org/10.1680/wame.2001.148.3.167
**Local path**: `references/copyrighted/mingham2001.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: The TVD-term-appended-to-corrector MacCormack variant (explicit, second-order).
**MOFEFID Lane D role**: the specific scheme variant the enhanced-WEPP solver follows, including the flux-limiter/`Cf(Cr)` construction in Papanicolaou eqs. (11a–f).
**Notes / caveats**: Intake-level annotation.

## R-83: Whipkey (1965) subsurface stormflow from forested slopes

**Citation**: Whipkey, R. Z. (1965). *Subsurface stormflow from forested slopes*. Bulletin of the International Association of Scientific Hydrology, 10(2), 74–85. https://doi.org/10.1080/02626666509493392
**Local path**: `references/copyrighted/whipkey1965.pdf` (T&F copy; identity verified).
**Reference quality**: `verified-primary`
**Topic**: The foundational forested-slope trench demonstration of lateral subsurface stormflow.
**MOFEFID Lane C role**: historical foundation of the trench-measurement lineage all four fixture sites descend from.
**Notes / caveats**: Intake-level annotation.

## R-84: Hewlett & Hibbert (1963) sloping soil mass drainage

**Citation**: Hewlett, J. D., and A. R. Hibbert (1963). *Moisture and energy conditions within a sloping soil mass during drainage*. Journal of Geophysical Research, 68(4), 1081–1087. https://doi.org/10.1029/JZ068i004p01081
**Local path**: `references/copyrighted/hewlett1963.pdf` (JGR header verified).
**Reference quality**: `verified-primary`
**Topic**: The Coweeta artificial-hillslope drainage experiment; origin of translatory-flow/variable-source-area thinking.
**MOFEFID Lane C role**: Coweeta-context foundation; frames what the Coweeta water-yield records can and cannot say about hillslope lateral flow (context-only per fixture README).
**Notes / caveats**: The 1967 variable-source-area chapter remains operator-tracked.

## R-85: Dunne & Black (1970) partial area contributions

**Citation**: Dunne, T., and R. D. Black (1970). *Partial area contributions to storm runoff in a small New England watershed*. Water Resources Research, 6(5), 1296–1311. https://doi.org/10.1029/WR006i005p01296
**Local path**: `references/copyrighted/dunne1970.pdf` (+ transcription `dunne1970.md`).
**Reference quality**: `verified-primary`
**Topic**: Field demonstration that limited saturated areas produce most storm runoff.
**MOFEFID Lane C role**: bounds the contributing-area dimension of the lateral-flow envelope.
**Notes / caveats**: Intake-level annotation.

## R-86: Harr (1977) water flux in soil and subsoil on a steep forested slope

**Citation**: Harr, R. D. (1977). *Water flux in soil and subsoil on a steep forested slope*. Journal of Hydrology, 33(1–2), 37–58. https://doi.org/10.1016/0022-1694(77)90097-X
**Local path**: `references/copyrighted/harr1977.pdf` (+ transcription `harr1977.md`).
**Reference quality**: `verified-primary`
**Topic**: HJ Andrews steep-slope measured soil/subsoil water fluxes.
**MOFEFID Lane C role**: site-matched physical bound on lateral flux rates for the primary Lane C site (WS10's neighborhood); pairs with R-67.
**Notes / caveats**: Intake-level annotation; flux magnitudes to be re-derived in C1.

## R-87: Mosley (1979) streamflow generation Maimai

**Citation**: Mosley, M. P. (1979). *Streamflow generation in a forested watershed, New Zealand*. Water Resources Research, 15(4), 795–806. https://doi.org/10.1029/WR015i004p00795
**Local path**: `references/copyrighted/mosley1979.pdf` (+ transcription `mosley1979.md`).
**Reference quality**: `verified-primary`
**Topic**: Foundational Maimai subsurface-flow study.
**MOFEFID Lane C role**: Maimai lateral-flow foundation; lower anchor of the M8 evidence lineage completed by R-88/R-89.
**Notes / caveats**: Intake-level annotation.

## R-88: McGlynn, McDonnell & Brammer (2002) Maimai perceptual model review

**Citation**: McGlynn, B. L., J. J. McDonnell, and D. D. Brammer (2002). *A review of the evolving perceptual model of hillslope flowpaths at the Maimai catchments, New Zealand*. Journal of Hydrology, 257(1–4), 1–26. https://doi.org/10.1016/S0022-1694(01)00559-5
**Local path**: `references/copyrighted/mcglynn2002.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: ~25-year synthesis of Maimai trench/tracer flowpath evidence.
**MOFEFID Lane C role**: the quantitative perceptual model against which the Maimai bracket of the envelope is constructed.
**Notes / caveats**: Intake-level annotation.

## R-89: Woods & Rowe (1996) Maimai M8 trench spatial variability

**Citation**: Woods, R., and L. Rowe (1996). *The changing spatial variability of subsurface flow across a hillside*. Journal of Hydrology (New Zealand), 35(1), 51–86.
**Local path**: `references/copyrighted/JoHNZ_1996_v35_1_Woods.pdf` (scan, no text layer) + transcription `JoHNZ_1996_v35_1_Woods.md` (title/abstract verified from transcription).
**Reference quality**: `verified-primary`
**Topic**: The Maimai M8 30-trough trench dataset: per-unit-area subsurface flow distribution and its convergence toward spatial uniformity in large wet events.
**MOFEFID Lane C role**: **the observed M8 trench-dataset paper itself** — pairs directly with the held `maimai_m8/` fixture; a primary envelope source.
**Notes / caveats**: No DOI (NZHS journal). Numeric extraction should use the transcription with the scan as provenance.

## R-90: Bachmair & Weiler (2011) new dimensions of hillslope hydrology

**Citation**: Bachmair, S., and M. Weiler (2011). *New Dimensions of Hillslope Hydrology* (Ch. 23). In Levia, Carlyle-Moses & Tanaka (eds.), Forest Hydrology and Biogeochemistry, Ecological Studies 216, Springer. https://doi.org/10.1007/978-94-007-1363-5_23
**Local path**: `references/copyrighted/bachmair2011.pdf` (chapter title verified).
**Reference quality**: `verified-primary`
**Topic**: Review of forest-hillslope subsurface-flow partitioning, thresholds, and connectivity across trench studies.
**MOFEFID Lane C role**: cross-study framing for how lateral fraction varies with structure and antecedent state; complements R-68.
**Notes / caveats**: Intake-level annotation.

## R-91: Brooks, Boll & McDaniel (2007) SMR distributed response, eastern Palouse

**Citation**: Brooks, E. S., J. Boll, and P. A. McDaniel (2007; online 2006). *Distributed and integrated response of a geographic information system-based hydrologic model in the eastern Palouse region, Idaho*. Hydrological Processes, 21(1), 110–122. https://doi.org/10.1002/hyp.6230
**Local path**: `references/copyrighted/brooks2006.pdf` (journal header verified).
**Reference quality**: `verified-primary`
**Topic**: SMR model applied across the Palouse with measured horizon lateral Ks/anisotropy over fragipan soils.
**MOFEFID Lane C role**: the field-measured Palouse lateral-Ks/anisotropy companion to R-62 — empirical center of the conductivity envelope for that soil class.
**Notes / caveats**: Intake-level annotation.

## R-92: Wigmosta, Vail & Lettenmaier (1994) DHSVM

**Citation**: Wigmosta, M. S., L. W. Vail, and D. P. Lettenmaier (1994). *A distributed hydrology-vegetation model for complex terrain*. Water Resources Research, 30(6), 1665–1679. https://doi.org/10.1029/94WR00436
**Local path**: `references/copyrighted/wigmosta1994.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: DHSVM formulation; the distributed-model precedent of effective lateral Ks calibrated ~100× vertical.
**MOFEFID Lane C role**: anisotropy-multiplier precedent for the upper edge of the conductivity envelope.
**Notes / caveats**: Intake-level annotation.

## R-93: Beven & Germann (1982) macropores and water flow in soils

**Citation**: Beven, K., and P. Germann (1982). *Macropores and water flow in soils*. Water Resources Research, 18(5), 1311–1325. https://doi.org/10.1029/WR018i005p01311
**Local path**: `references/copyrighted/beven1982.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: Foundational statement of macropore-dominated departures from Darcian matrix conductivity in structured field soils.
**MOFEFID Lane C role**: physical basis for a lateral-Ks envelope wider than lab core values (with R-94).
**Notes / caveats**: Intake-level annotation.

## R-94: Beven & Germann (2013) macropores revisited

**Citation**: Beven, K., and P. Germann (2013). *Macropores and water flow in soils revisited*. Water Resources Research, 49(6), 3071–3092. https://doi.org/10.1002/wrcr.20156
**Local path**: `references/copyrighted/beven2013.pdf` (accepted-manuscript copy; title verified).
**Reference quality**: `verified-primary`
**Topic**: 30-year synthesis updating the macropore/effective-conductivity argument.
**MOFEFID Lane C role**: modern uncertainty framing for upscaled lateral Ks.
**Notes / caveats**: Accepted-manuscript version (no journal typesetting).

## R-95: McDaniel et al. (2001) perched water tables on Argixeroll/Fragixeralf hillslopes

**Citation**: McDaniel, P. A., R. W. Gabehart, A. L. Falen, J. E. Hammel, and R. J. Reuter (2001). *Perched Water Tables on Argixeroll and Fragixeralf Hillslopes*. Soil Science Society of America Journal, 65(3), 805–810. https://doi.org/10.2136/sssaj2001.653805x
**Local path**: `references/copyrighted/mcdaniel2001.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: Direct hillslope observations of perched water tables over fragipans in Palouse-class soils.
**MOFEFID Lane C role**: constrains when/where lateral saturated flow is physically active in the R-62/R-91 soil class.
**Notes / caveats**: Intake-level annotation.

## R-96: McDaniel et al. (2008) fragipans, perched water tables, catchment processes

**Citation**: McDaniel, P. A., M. P. Regan, E. Brooks, J. Boll, S. Barndt, A. Falen, S. K. Young, and J. E. Hammel (2008). *Linking fragipans, perched water tables, and catchment-scale hydrological processes*. Catena, 73(2), 166–173. https://doi.org/10.1016/j.catena.2007.05.011
**Local path**: `references/copyrighted/mcdaniel2008.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: Hydropedology linking restrictive layers to perched water tables and lateral throughflow at catchment scale.
**MOFEFID Lane C role**: process authority that restrictive-layer lateral throughflow is real and dominant — keeps the magnitude bound physically honest.
**Notes / caveats**: Intake-level annotation.

## R-97: Hasan, Troch & Boll (2006) hillslope hydrology via local gravity, Moxa

**Citation**: Hasan, S., P. A. Troch, J. Boll, and C. Kroner (2006). *Modeling the hydrological effect on local gravity at Moxa, Germany*. Journal of Hydrometeorology, 7(3), 346–354. https://doi.org/10.1175/JHM488.1
**Local path**: `references/copyrighted/hydr-jhm488_1.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: Hillslope-storage modeling of subsurface flow constrained by superconducting-gravimeter observations (Boll lineage).
**MOFEFID Lane C role**: peripheral — an independent-observable check on hillslope storage dynamics; low priority for the envelope.
**Notes / caveats**: Operator-supplied; tangential to the core Lane C metric.

## R-98: O'Keeffe et al. (2023) SMR biochar streamflow modeling

**Citation**: O'Keeffe, A., E. Brooks, C. Dunkel, and D. S. Shrestha (2023). *Soil moisture routing modeling of targeted biochar amendment in undulating topographies: an analysis of biochar's effects on streamflow*. AIMS Environmental Science, 10(4), 529–546. https://doi.org/10.3934/environsci.2023030
**Local path**: `references/copyrighted/10.3934_environsci.2023030.pdf` (title verified).
**Reference quality**: `verified-primary`
**Topic**: Modern SMR-lineage application (Brooks co-author) with lateral-flow routing over restrictive layers in Palouse-type topography.
**MOFEFID Lane C role**: current-generation SMR lineage context; documents present-day parameterization practice of the model family WEPP-forest's lateral additions descend from.
**Notes / caveats**: Open access (AIMS).

## R-99: Shen & Li (1973) rainfall sheet-flow resistance — secondary-cited

**Citation**: Shen, H. W., and R.-M. Li (1973). *Rainfall effect on sheet flow over smooth surface*. Journal of the Hydraulics Division ASCE, 99(HY5), 771–792. https://doi.org/10.1061/JYCEAJ.0003646
**Local path**: `not-acquired` (library freeze 2026-07-01).
**Reference quality**: `secondary-via-R-63`
**Topic**: Empirical rain/grain skin-resistance regression for laminar sheet flow.
**MOFEFID Lane D role**: origin of eqs. (2)–(3) in R-63; **cite through R-63**, whose stated form + the supplemental validation fixtures serve as the constant/unit authority.
**OAR-6 compliance status**: Companion-only; cannot be sole constant authority.

## R-100: Abrahams (1998) discussion of Lawrence, + Lawrence reply — secondary-cited

**Citation**: Abrahams, A. D. (1998). *Discussion: 'Macroscale surface roughness and frictional resistance in overland flow'*. Earth Surface Processes and Landforms, 23(9), 857–859 (reply: Lawrence, 861–862).
**Local path**: `not-acquired` (library freeze 2026-07-01).
**Reference quality**: `secondary-via-R-63`
**Topic**: Critique of the regime limits underlying the R-77 inundation-ratio resistance model.
**MOFEFID Lane D role**: eq. (4) applicability limits are instead handled by a documented-uncertainty note in D1 contracts, grounded in R-77 (primary, in hand) and R-63's simplified form.
**OAR-6 compliance status**: Companion-only.

## R-101: Woolhiser (1975) laminar friction coefficient tables — secondary-cited

**Citation**: Woolhiser, D. A. (1975). *Simulation of unsteady overland flow*. In Mahmood & Yevjevich (eds.), Unsteady Flow in Open Channels, Vol. II, 485–508. Water Resources Publications.
**Local path**: `not-acquired` (library freeze 2026-07-01).
**Reference quality**: `secondary-via-KINEROS`
**Topic**: Tabulated k₀ laminar friction coefficients by surface type.
**MOFEFID Lane D role**: coefficients taken from the KINEROS documentation reproduction (Smith 1990, in repo) with R-63 as the usage context.
**OAR-6 compliance status**: Companion-only.

## R-102: Davis (1984) TVD finite difference schemes and artificial viscosity

**Citation**: Davis, S. F. (1984). *TVD Finite Difference Schemes and Artificial Viscosity*. ICASE Report No. 84-20 / NASA CR-172373. Hampton, VA: Institute for Computer Applications in Science and Engineering, NASA Langley Research Center.
**Local path**: `references/copyrighted/19840021490.pdf` (identity verified: title page read 2026-07-06). Companion Gemini-converted markdown: `references/copyrighted/19840021490.md` — conversion, not primary; eq. (3.20) verified faithful against the rendered PDF page (p. 9); any other equation cited as binding authority must be spot-checked against the PDF first.
**Reference quality**: `verified-primary`
**Distribution status**: NTRS public download (document ID 19840021490); first-pass rights classification RECORDED (2026-07-06 addendum, `rights_classification_first_pass_2026-05-11.md`): `copyrighted/` conservatively (ICASE/USRA contractor report, not automatic 17 U.S.C. 105), vendorable candidate pending an explicit NTRS rights statement.
**Topic**: Derivation of the symmetric (non-upwind-weighted), parameter-free TVD artificial-dissipation term addable to MacCormack/Lax-Wendroff codes — the family origin of the `Gr = 0.5·Cf·(1−φ)` construction in R-63 eqs. (11a–f) via R-82.
**MOFEFID Lane D role**: adjudicates the R-63 printed limiter branch for `GAP-OFEROUTE-005` / D10B Leg A: eq. (3.20) (p. 9, read from the rendered page) defines `φ(r) = min(2r, 1) if r > 0; 0 if r ≤ 0` — the exact branch-swap of R-63's printed (11c); eq. (3.18) gives the two-sided per-face dissipation coefficients `K±` relevant to the one-sided-vs-two-sided ratio adjudication.
**Notes / caveats**: Intake-level annotation. Acquired 2026-07-06 (operator) for `20260706-mofefid-d10b-gap005-source-authority-reconciliation-001`; named in R-63 §2.3's own citation chain, so clean-room-compatible.

## R-103: Tseng (2010) kinematic wave computation, efficient implicit method

**Citation**: Tseng, M.-H. (2010). *Kinematic wave computation using an efficient implicit method*. Journal of Hydroinformatics, 12(3), 329–338. IWA Publishing.
**Local path**: `references/copyrighted/Tseng2010_Hydroinformatics.pdf` (identity verified: first page read 2026-07-06).
**Reference quality**: `verified-primary`
**Distribution status**: IWA Publishing copyright; local restricted cache; first-pass rights classification RECORDED (2026-07-06 addendum, `rights_classification_first_pass_2026-05-11.md`): `copyrighted/`.
**Topic**: Finite-difference implicit MacCormack scheme for 1-D kinematic-wave overland/open-channel flow, benchmarked against an explicit MacCormack variant, analytical solutions, and experimental measurement.
**MOFEFID Lane D role**: R-63 §2.3's named source for the applied TVD-MacCormack KWE computation ("MacCormack, 1969, 1985; Tseng, 2010"); D10B Leg-A authority for the `alpha` update-timing (explicit vs implicit) adjudication, and published precedent for the Leg-B acceptance shape (validating KWE schemes against analytic solutions plus experiment rather than another implementation's trace).
**Notes / caveats**: Intake-level annotation. Acquired 2026-07-06 (operator) for `20260706-mofefid-d10b-gap005-source-authority-reconciliation-001`; named in R-63 §2.3's own citation chain, so clean-room-compatible.

## R-104: Knisel (ed.) 1980 CREAMS report — Conservation Research Report 26

**Citation**: Knisel, W. G. (Ed.) (1980). *CREAMS: A Field-Scale Model for Chemicals, Runoff, and Erosion from Agricultural Management Systems*. USDA Conservation Research Report No. 26. 640 pp.
**Local path**: `references/vendorable/creams/312.pdf` (identity verified: title page read 2026-07-10; 690 PDF pages).
**Reference quality**: `verified-primary`
**Distribution status**: USDA publication, US-government work (17 U.S.C. 105) → `vendorable/`. First-pass rights classification RECORDED (2026-07-10 addendum, `rights_classification_first_pass_2026-05-11.md`).
**Topic**: The parent field-scale model of the WEPP watershed channel erosion component; container volume for R-105.
**WSHED-W11A role**: parent-model provenance for the `chnrt` lineage (Ch. 13 §13.5.1 "adapted and modified from the CREAMS model channel erosion routines").
**Notes / caveats**: Acquired 2026-07-10 (operator) for `20260710-wshedw11a-channel-hourly-sediment-authority-001`. Scanned document; not text-searchable.

## R-105: Foster, Lane, Nowlin, Laflen & Young 1980 — CREAMS Chapter 3 (erosion model development)

**Citation**: Foster, G. R., L. J. Lane, J. D. Nowlin, J. M. Laflen and R. A. Young (1980). "Chapter 3. A model to estimate sediment yield from field-sized areas: development of model." In Knisel (Ed.), *CREAMS*, USDA Conservation Research Report No. 26.
**Local path**: `references/vendorable/creams/312-ch3.pdf` (scan; not text-searchable). Companion converted markdown: `references/vendorable/creams/312-ch3.md` — conversion, not primary. Widening-law equations [I-133]–[I-140] and quasi-steady statement at [I-56] verified faithful against the rendered scan (report pp. 54–55, PDF pp. 19–20, read 2026-07-10); any other equation cited as binding authority must be spot-checked against the rendered PDF first.
**Reference quality**: `verified-primary`
**Distribution status**: USDA publication, US-government work → `vendorable/`. RECORDED in 2026-07-10 rights addendum.
**Topic**: Primary source of the WEPP channel erosion physics: quasi-steady sediment continuity with the compute-cost rationale for deleting time terms [I-56]; concentrated-flow detachment [I-128]–[I-132]; the post-nonerodible-layer channel-widening time-evolution law ω = 1 − exp(−t*) with carried state (W_i, t_i) and flow-dependent final width W_f(Q) [I-133]–[I-140]; Yalin transport with multi-class modification [I-93]ff; shear partition [I-141]–[I-143]; L_eff/10 segment discretization.
**WSHED-W11A role**: resolves the widening-clock question for per-interval channel sediment sequencing (authority-matrix Row 3); primary provenance for Rows 2 and 6. Also the held secondary source for Foster & Meyer 1972 (R-111), Yalin 1963 (R-112), and McCool et al. 1966 (R-113).
**Notes / caveats**: Acquired 2026-07-10 (operator) for `20260710-wshedw11a-channel-hourly-sediment-authority-001`.

## R-106: Woolhiser, Smith & Goodrich 1990 — KINEROS documentation (ARS-77)

**Citation**: Woolhiser, D. A., R. E. Smith and D. C. Goodrich (1990). *KINEROS, A Kinematic Runoff and Erosion Model: Documentation and User Manual*. USDA-ARS, ARS-77. 130 pp.
**Local path**: `references/vendorable/kineros/703.pdf` (identity verified: title page read 2026-07-10). Companion converted markdown: `references/vendorable/kineros/703.md` — conversion, not primary; equations cited as binding authority must be spot-checked against the PDF.
**Reference quality**: `verified-primary`
**Distribution status**: USDA-ARS publication, US-government work → `vendorable/`. RECORDED in 2026-07-10 rights addendum.
**Topic**: Formal documentation behind the R-11 chapter extract: kinematic water routing plus unsteady sediment mass balance solved on the same time/space grid as the water solution, kinetic-transfer erosion/deposition source term, per-class routing.
**WSHED-W11A role**: external-canonical authority for the sediment-quantum-equals-water-grid rule (authority-matrix Row 1) and the unsteady fallback form (Row 2). Restates the Bennett 1974 sediment mass-balance equation with citation (`703.md:974`), providing held secondary coverage for R-109.
**Notes / caveats**: Acquired 2026-07-10 (operator) for `20260710-wshedw11a-channel-hourly-sediment-authority-001`.

## R-107: USACE HEC-RAS 1D Sediment Transport manual (2026 web capture)

**Citation**: USACE Hydrologic Engineering Center. *HEC-RAS 1D Sediment Transport* (User's Manual page tree, incl. quasi-unsteady flow chapters). Web capture 2026-07-10 from hec.usace.army.mil confluence documentation.
**Local path**: `references/vendorable/HEC_RAS_1D_Sediment_Transport_UserManual_20260710.pdf` (identity verified: title page read 2026-07-10; quasi-unsteady passages verified present — "series of steady flow profiles", per-computational-increment bed update and its small-bed-change justification).
**Reference quality**: `verified-primary`
**Distribution status**: USACE public documentation, consistent with existing HEC-RAS entries (R-16) → `vendorable/`. RECORDED in 2026-07-10 rights addendum.
**Topic**: The canonical quasi-steady-sequence sediment model class: flow hydrograph approximated by a series of steady profiles; the computational increment is the hydraulic and sediment-transport time step; bed geometry updates each increment and carries to the next.
**WSHED-W11A role**: external-canonical authority for the per-interval quasi-steady solve form and the geometry-carry rule (authority-matrix Rows 1–3). The online 1D Sediment Transport Technical Reference Manual remains the formal citable source; this capture holds the load-bearing passages locally.
**Notes / caveats**: Acquired 2026-07-10 (operator) for `20260710-wshedw11a-channel-hourly-sediment-authority-001`.

## R-108: Gilley, Woolhiser & McWhorter 1985 — interrill erosion model equations (Part I)

**Citation**: Gilley, J. E., D. A. Woolhiser and D. B. McWhorter (1985). "Interrill soil erosion — Part I: Development of model equations." *Transactions of the ASAE* 28(1):147–153.
**Local path**: `references/copyrighted/Gilley,Woolhiser,McWhorter_1985.pdf` (identity verified: title/abstract read 2026-07-10; local-only cache, not committed). Companion converted markdown: `references/copyrighted/Gilley,Woolhiser,McWhorter_1985.md` — conversion, not primary.
**Reference quality**: `verified-primary`
**Distribution status**: ASAE journal copyright → `copyrighted/` (gitignored local cache; metadata tracked here). RECORDED in 2026-07-10 rights addendum.
**Topic**: Rainfall-driven interrill detachment and transport-capacity model equations (Darcy-Weisbach rainfall-resistance depth, impact-pressure detachment, shear×velocity transport factor).
**WSHED-W11A role**: KINEROS-lineage source-term development supporting R-106's upland erosion terms; secondary context only — not channel-sediment physics.
**Notes / caveats**: Acquired 2026-07-10 (operator).

## R-109: Bennett (1974) sediment-yield modeling concepts — secondary-cited

**Citation**: Bennett, J. P. (1974). "Concepts of mathematical modeling of sediment yield." *Water Resources Research* 10(3):485–492.
**Local path**: `not-acquired`.
**Reference quality**: `secondary-via-KINEROS`
**Topic**: The foundational unsteady sediment continuity formulation — the "time terms" whose deletion CREAMS Ch. 3 [I-56] and WEPP Ch. 13 §13.5.5 both state as the quasi-steady assumption.
**WSHED-W11A role**: parent-equation authority for the recorded unsteady fallback form (authority-matrix Row 2); equation restated with citation in held R-106 (`703.md:974`).
**OAR-6 compliance status**: Companion-only.

## R-110: Jeong et al. (2011) sub-daily SWAT sediment algorithms — citation-only

**Citation**: Jeong, J., N. Kannan, J. G. Arnold, R. Glick, L. Gosselink, R. Srinivasan and R. D. Harmel (2011). "Development of sub-daily erosion and sediment transport algorithms for SWAT." *Transactions of the ASABE* 54(5):1685–1691.
**Local path**: `not-acquired` (no open copy located 2026-07-10; ASABE paywall).
**Reference quality**: `citation-only`
**Topic**: Procedural precedent: a daily agricultural watershed model retrofitted with physically based sub-daily channel erosion/sediment routing computed at the flow-routing time step.
**WSHED-W11A role**: non-gating precedent for the sediment-quantum-equals-water-grid rule (authority-matrix Row 1).
**OAR-6 compliance status**: Companion-only.

## R-111: Foster & Meyer (1972) closed-form erosion equation — secondary-cited

**Citation**: Foster, G. R. and L. D. Meyer (1972). "A closed-form soil erosion equation for upland areas." In H. W. Shen (Ed.), *Sedimentation: Symposium to Honor Professor H. A. Einstein*, Ft. Collins, CO. Chapter 12.
**Local path**: `not-acquired`.
**Reference quality**: `secondary-via-CREAMS`
**Topic**: The steady-state sediment-continuity and detachment/transport-coupling model that both the hillslope (Ch. 11) and channel (Ch. 13) WEPP erosion components build on.
**WSHED-W11A role**: parent of the four-case detachment/deposition machinery; working form abstracted in held R-105 ("abstracted from Foster and Meyer (10)") and WEPP Ch. 13 §13.5.1.
**OAR-6 compliance status**: Companion-only.

## R-112: Yalin (1963) bedload transport equation — secondary-cited

**Citation**: Yalin, Y. S. (1963). "An expression for bedload transportation." *Journal of the Hydraulics Division, ASCE* 89(HY3):221–250.
**Local path**: `not-acquired`.
**Reference quality**: `secondary-via-CREAMS`
**Topic**: The channel/overland transport-capacity equation of the CREAMS/WEPP lineage.
**WSHED-W11A role**: transport-capacity authority (authority-matrix Row 6); the full working form including the multi-class excess-capacity modification — which is what the baseline `trncap.for` implements — is carried in held R-105 ([I-93]ff).
**OAR-6 compliance status**: Companion-only.

## R-113: McCool, Gwinn, Ree & Garton (1966) spatially-varied flow in vegetated channels — secondary-cited

**Citation**: McCool, D. K., W. R. Gwinn, W. O. Ree and J. E. Garton (1966). "Spatially varied steady flow in a vegetated channel." *Transactions of the ASAE* 9(3):440–444.
**Local path**: `not-acquired`.
**Reference quality**: `secondary-via-CREAMS`
**Topic**: Source of the β = 1.56 energy coefficient in the spatially-varied flow equations (WEPP Ch. 13 Eq. [13.5.4]).
**WSHED-W11A role**: constant provenance; restated in held R-105 ("β = energy coefficient [1.56 used from McCool and others (23)]").
**OAR-6 compliance status**: Companion-only.

## R-114: NRC Regulatory Guide 1.203 - transient and accident analysis methods

**Citation**: U.S. Nuclear Regulatory Commission (2005). *Regulatory Guide 1.203: Transient and Accident Analysis Methods*. ADAMS Accession No. ML053500170.
**Local path**: `not-acquired`; official source: https://www.nrc.gov/docs/ML0535/ML053500170.pdf
**Reference quality**: `verified-primary-government-guidance`
**Topic**: Evaluation Model Development and Assessment Process (EMDAP), important-phenomena ranking, assessment bases, scaling, applicability, uncertainty, quality assurance, and documentation.
**openWEPP V&V role**: Nuclear-engineering precedent for declared requirements, hierarchical and graded assessment, applicability, uncertainty, configuration control, and independent review. openWEPP applies these disciplines to closable software-verification obligations; this source is not authority for treating developer-characterized environmental evidence as a site-specific licensing or fitness decision.
**Rights / distribution**: U.S. Government publication; remote link only in this intake.

## R-115: NASA-STD-7009B - standard for models and simulations

**Citation**: National Aeronautics and Space Administration (2024). *NASA-STD-7009B: Standard for Models and Simulations*.
**Local path**: `not-acquired`; official source: https://standards.nasa.gov/standard/NASA/NASA-STD-7009
**Reference quality**: `verified-primary-government-standard`
**Topic**: Model life-cycle requirements for intended and permissible use, code and solution verification, conceptual and empirical validation, data pedigree, uncertainty, defects, assessments, and reporting.
**openWEPP V&V role**: Primary source for claim-bounded credibility profiles and separate machine evidence, use assessment, and decision reporting.
**Rights / distribution**: Publicly accessible U.S. Government standard; remote link only in this intake.

## R-116: EPA guidance on environmental-model development, evaluation, and application

**Citation**: U.S. Environmental Protection Agency, Council for Regulatory Environmental Modeling (2009). *Guidance on the Development, Evaluation, and Application of Environmental Models*. EPA/100/K-09/003.
**Local path**: `not-acquired`; official source: https://www.epa.gov/sites/production/files/2015-04/documents/cred_guidance_0309.pdf
**Reference quality**: `verified-primary-government-guidance`
**Topic**: Fit-for-use environmental-model evaluation, conceptual models, quality planning, corroboration, calibration independence, sensitivity, uncertainty, peer review, transparency, and post-audit.
**openWEPP V&V role**: Primary environmental-model authority for avoiding whole-model validity claims, treating observation quality and application context as part of the assessment, and maintaining evaluation as a continuing development, application, and post-audit responsibility.
**Rights / distribution**: U.S. Government publication; remote link only in this intake.

## R-117: ASME V&V 20 - computational fluid dynamics and heat transfer

**Citation**: American Society of Mechanical Engineers (2009, reaffirmed 2021). *ASME V&V 20: Standard for Verification and Validation in Computational Fluid Dynamics and Heat Transfer*.
**Local path**: `not-acquired`; official description: https://www.asme.org/codes-standards/find-codes-standards/standard-for-verification-and-validation-in-computational-fluid-dynamics-and-heat-transfer
**Reference quality**: `verified-consensus-standard-metadata`
**Topic**: Code and solution verification, validation comparison error, experimental and simulation uncertainty, specified validation variables, and validation points.
**openWEPP V&V role**: Consensus-standard basis for quantity-specific validation comparisons and explicit limits on inference away from validation points.
**Rights / distribution**: `restricted`; the full standard is copyrighted and was not acquired or vendored.

## R-118: Sandia verification, validation, and predictive-capability framework

**Citation**: Oberkampf, W. L., T. G. Trucano, and C. Hirsch (2003). *Verification, Validation, and Predictive Capability in Computational Engineering and Physics*. SAND2002-3769. https://doi.org/10.2172/809603
**Local path**: `not-acquired`; public record: https://www.osti.gov/biblio/809603
**Reference quality**: `verified-primary-government-technical-report`
**Topic**: PIRT prioritization, code and solution verification, manufactured solutions, numerical-error estimation, hierarchical validation experiments, statistical metrics, and predictive capability.
**openWEPP V&V role**: Computational-science basis for separate verification and validation ladders and process-importance-driven evidence planning.
**Rights / distribution**: Public U.S. Department of Energy technical report; remote link only in this intake.

## R-119: Sandia Predictive Capability Maturity Model

**Citation**: Oberkampf, W. L., T. G. Trucano, and M. Pilch (2007). *Predictive Capability Maturity Model for Computational Modeling and Simulation*. SAND2007-5948. https://doi.org/10.2172/976951
**Local path**: `not-acquired`; public record: https://www.osti.gov/biblio/976951
**Reference quality**: `verified-primary-government-technical-report`
**Topic**: Separate maturity dimensions for representation, physics fidelity, code verification, solution verification, model validation, uncertainty, and sensitivity.
**openWEPP V&V role**: Basis for a visible evidence profile while preserving the report's warning that maturity assessment does not decide application acceptance.
**Rights / distribution**: Public U.S. Department of Energy technical report; remote link only in this intake.

## R-120: VERA-CS verification and validation plan

**Citation**: Downar, T., S. Palmtag, K. Clarno, and K. Kim (2017). *VERA-CS Verification and Validation Plan*. CASL-U-2017-1287-000, Oak Ridge National Laboratory.
**Local path**: `not-acquired`; official record: https://www.ornl.gov/publication/vera-cs-verification-validation-plan-0
**Reference quality**: `verified-primary-government-technical-report`
**Topic**: Hierarchical V&V of a multiphysics reactor code suite, single-physics readiness before coupled-system reliance, standardized outputs, and automated report generation.
**openWEPP V&V role**: Direct software-program precedent for subsystem-first V&V and generated human tables and figures.
**Rights / distribution**: Public U.S. Department of Energy laboratory report; remote link only in this intake.

## R-121: Nearing (2000) erosion-model evaluation under observation variability

**Citation**: Nearing, M. A. (2000). "Evaluating soil erosion models using measured plot data: accounting for variability in the data." *Earth Surface Processes and Landforms*, 25(9), 1035-1043. https://doi.org/10.1002/1096-9837(200008)25:9%3C1035::AID-ESP121%3E3.0.CO;2-B
**Local path**: `not-acquired`.
**Reference quality**: `verified-primary-peer-reviewed`
**Topic**: Interpretation of prediction error relative to the variability of replicated natural-rainfall plot measurements.
**openWEPP V&V role**: WEPP-domain basis for including observation and natural variability in validation metrics rather than treating measurements as exact truth.
**Rights / distribution**: `restricted`; publisher article was not acquired or vendored.

## R-122: Wang et al. (2023) multi-regime WEPP hillslope evaluation

**Citation**: Wang, S., R. P. McGehee, T. Guo, D. C. Flanagan, and B. A. Engel (2023). "Calibration, validation, and evaluation of the Water Erosion Prediction Project (WEPP) model for hillslopes with natural runoff plot data." *International Soil and Water Conservation Research*, 11(4), 669-687. https://doi.org/10.1016/j.iswcr.2022.10.004
**Local path**: `not-acquired`; open article landing page at the DOI.
**Reference quality**: `verified-primary-peer-reviewed-open-access`
**Topic**: Evaluation across 1,159 plot-years, multiple climates, soils, topographies, crops, event and aggregate scales, calibrated and uncalibrated modes, and event extremes.
**openWEPP V&V role**: Current WEPP-specific evidence for cross-regime stratification, transparent calibration effects, multi-scale metrics, and explicit tail-performance limitations.
**Rights / distribution**: Open-access article; exact Creative Commons redistribution terms were not verified, so no local copy was added.

## R-123: W3C PROV-O provenance ontology

**Citation**: World Wide Web Consortium (2013). *PROV-O: The PROV Ontology*. W3C Recommendation. https://www.w3.org/TR/prov-o/
**Local path**: `not-acquired`; canonical web specification at the cited URL.
**Reference quality**: `verified-primary-open-standard`
**Topic**: Machine-readable provenance through entities, activities, agents, generation, use, derivation, attribution, and bundles.
**openWEPP V&V role**: Optional future export vocabulary for linking evidence statements, data, executions, software agents, and supersession; not a prerequisite for the human dossier or its first tracked manifest.
**Rights / distribution**: W3C Recommendation under W3C document terms; remote link only in this intake.

## R-124: RO-Crate research-object packaging specification

**Citation**: RO-Crate Community (n.d.; accessed 2026-07-13). *RO-Crate Metadata Specification*, current long-term release 1.3. https://www.researchobject.org/ro-crate/specification.html
**Local path**: `not-acquired`; canonical web specification at the cited URL.
**Reference quality**: `verified-primary-open-specification`
**Topic**: JSON-LD packaging of research data, software, workflows, provenance, contextual entities, and human-readable previews.
**openWEPP V&V role**: Candidate future export format for portable dossier evidence bundles after recurring campaigns demonstrate a need; no internal evidence graph is presumed.
**Rights / distribution**: Apache-2.0 specification and documentation; remote link only in this intake.

## R-125: Oreskes, Shrader-Frechette, and Belitz (1994) on open-system model confirmation

**Citation**: Oreskes, N., K. Shrader-Frechette, and K. Belitz (1994). "Verification, validation, and confirmation of numerical models in the earth sciences." *Science*, 263(5147), 641-646. https://doi.org/10.1126/science.263.5147.641
**Local path**: `not-acquired`; bibliographic record and abstract: https://pubmed.ncbi.nlm.nih.gov/17747657/
**Reference quality**: `verified-primary-peer-reviewed`
**Topic**: The limits of strict verification and validation claims for models of open, incompletely known natural systems; nonuniqueness; and the partial evidentiary role of model confirmation.
**openWEPP V&V role**: Epistemic basis for separating hard verification of specified mathematical and software propositions from nonterminal empirical corroboration and from decision-owner application fitness. Agreement can add bounded evidence without proving the model true; contradiction can still reject or narrow a bounded claim.
**Rights / distribution**: `restricted`; publisher article was not acquired or vendored.

## Native-Vegetation ET Reference Intake (2026-08-03)

The following references support the native-vegetation ET backlog item. No
full-text artifact was added during concept intake. Rights default to
`restricted` until the vendoring review confirms otherwise.

## R-126: Penman (1948) combination evaporation

**Citation**: Penman, H. L. (1948). "Natural evaporation from open water, bare soil and grass." *Proceedings of the Royal Society A*, 193, 120-145. https://doi.org/10.1098/rspa.1948.0037
**Local path**: `not-acquired`.
**Reference quality**: `verified-primary-peer-reviewed`.
**Topic**: Radiation-aerodynamic combination equation for potential evaporation.
**openWEPP role**: Atmospheric-demand lineage; not authority for forest-component partition or fire response.
**Rights / distribution**: `restricted`; remote DOI metadata only.

## R-127: Priestley and Taylor (1972) equilibrium evaporation

**Citation**: Priestley, C. H. B., and R. J. Taylor (1972). "On the assessment of surface heat flux and evaporation using large-scale parameters." *Monthly Weather Review*, 100, 81-92. https://doi.org/10.1175/1520-0493(1972)100%3C0081:OTAOSH%3E2.3.CO;2
**Local path**: `not-acquired`.
**Reference quality**: `verified-primary-peer-reviewed`.
**Topic**: Equilibrium evaporation multiplied by an empirical coefficient for extensive saturated surfaces under limited advection.
**openWEPP role**: Optional low-input atmospheric-demand lineage; explicitly not a native-vegetation or fire-severity model.
**Rights / distribution**: `restricted`; remote DOI metadata only.

## R-128: Gash (1979) forest rainfall interception

**Citation**: Gash, J. H. C. (1979). "An analytical model of rainfall interception by forests." *Quarterly Journal of the Royal Meteorological Society*, 105, 43-55. https://doi.org/10.1002/qj.49710544304
**Local path**: `copyrighted/gash1979.pdf` and the operator-supplied
transcription `copyrighted/gash1979.md`. SHA-256: PDF
`920091bea907032133bf3f56d1171ba3b59a8957acaeaecc3043a73924388f22`;
Markdown
`c90d6c3dc8f78e82de0519815bbdffc1efa53f1b3f4514c6dd09da3ee55aff2b`.
**Reference quality**: `verified-primary-peer-reviewed-full-text`.
**Topic**: Forest-canopy wetting, saturation, drainage, and drying with independent wet-canopy evaporation.
**openWEPP role**: Primary event-scale authority for separating canopy
wetting, saturated evaporation, small storms, stemflow, and post-storm drying.
Its discrete-storm and complete-drying assumptions prevent direct substitution
for an arbitrary finite-timestep prognostic store without an explicit bridge.
**Rights / distribution**: `copyrighted-cache`; no affirmative redistribution
license appears in either supplied artifact. Both remain gitignored.

## R-129: Shuttleworth and Wallace (1985) two-source ET

**Citation**: Shuttleworth, W. J., and J. S. Wallace (1985). "Evaporation from sparse crops—an energy combination theory." *Quarterly Journal of the Royal Meteorological Society*, 111, 839-855. https://doi.org/10.1002/qj.49711146910
**Local path**:
`copyrighted/ShuttleworthWallace1985_NERC_Report.pdf`, Appendix IV, pp. 105
onward in Wallace et al., *Measurement and Prediction of Actual Evaporation
from Sparse Dryland Crops*, Institute of Hydrology Report OD 149/1 (1985).
SHA-256:
`b761d661f007a52a5f6c7dcbf0c7d3e9a82698b6e79c6290844c6cefec278626`.
**Reference quality**: `verified-primary-peer-reviewed`.
**Topic**: Coupled soil and canopy resistance network spanning bare ground through closed canopy.
**openWEPP role**: Preferred conceptual foundation for separately constrained live-canopy transpiration and soil evaporation.
**Rights / distribution**: `restricted`; the full article is locally cached
inside a NERC report, whose scanned artifact does not state redistribution
permission. The cache is gitignored and is not vendored.

## R-130: Fisher, Tu, and Baldocchi (2008) PT-JPL partition

**Citation**: Fisher, J. B., K. P. Tu, and D. D. Baldocchi (2008). "Global estimates of the land-atmosphere water flux based on monthly AVHRR and ISLSCP-II data, validated at 16 FLUXNET sites." *Remote Sensing of Environment*, 112, 901-919. https://doi.org/10.1016/j.rse.2007.06.025
**Local path**: `not-acquired`.
**Reference quality**: `verified-primary-peer-reviewed`.
**Topic**: Independent canopy-transpiration, soil-evaporation, and interception constraints applied to Priestley-Taylor potential demand.
**openWEPP role**: Minimal three-component diagnostic-prototype precedent; monthly remote-sensing formulation is not directly portable.
**Rights / distribution**: `restricted`; remote DOI metadata only.

## R-131: Martens et al. (2017) GLEAM v3

**Citation**: Martens, B., et al. (2017). "GLEAM v3: satellite-based land evaporation and root-zone soil moisture." *Geoscientific Model Development*, 10, 1903-1925. https://doi.org/10.5194/gmd-10-1903-2017
**Local path**: `vendorable/Martens2017_GLEAMv3.pdf`. SHA-256:
`51eb4aa1a69bfea44fe06d41d5891f18ec44d0635e6370a34fd4770119ff5eb2`.
**Reference quality**: `verified-primary-peer-reviewed-open-access`.
**Topic**: Separate transpiration, bare-soil evaporation, interception, open-water evaporation, and sublimation with surface/root-zone moisture constraints.
**openWEPP role**: Modular precedent for Gash interception and distinct surface-versus-root moisture stress.
**Rights / distribution**: `vendorable`; article text states Creative Commons
Attribution 3.0.

## R-132: NASA MOD16 terrestrial ET algorithm

**Citation**: Mu, Q., M. Zhao, and S. W. Running. *MODIS Global Terrestrial Evapotranspiration Algorithm Theoretical Basis Document* and MOD16 user guide. https://modis-land.gsfc.nasa.gov/ET.html
**Local path**: `copyrighted/MOD16_User_Guide_V6.pdf`; official Version 2.2
user guide, dated 2019-06-10. SHA-256:
`a43b47bc33256cad2c7f61566bcf32cd8365383a0eee4b6f02f93042fdcdb687`.
**Reference quality**: `verified-primary-government-algorithm-documentation`.
**Topic**: Wet-canopy evaporation, dry-canopy transpiration, and soil evaporation with biome-dependent resistance and stress parameters.
**openWEPP role**: Forest plant-functional-type parameter and three-component architecture reference; not a wholesale implementation target.
**Rights / distribution**: `restricted-pending-review`; the NASA-hosted PDF
does not include an explicit redistribution statement. It is conservatively
cached under the gitignored copyrighted tree.

## R-133: Zhang, Dawes, and Walker (2001) vegetation-change ET

**Citation**: Zhang, L., W. R. Dawes, and G. R. Walker (2001). "Response of mean annual evapotranspiration to vegetation changes at catchment scale." *Water Resources Research*, 37, 701-708. https://doi.org/10.1029/2000WR900325
**Local path**: `not-acquired`.
**Reference quality**: `verified-primary-peer-reviewed`.
**Topic**: Observationally evaluated mean-annual ET response to precipitation, potential ET, vegetation, and plant-available water.
**openWEPP role**: Annual native-vegetation total-ET validation constraint, not a daily component-flux kernel.
**Rights / distribution**: `restricted`; remote DOI metadata only.

## R-134: Roche, Goulden, and Bales (2020) Sierra Nevada wildfire ET

**Citation**: Roche, J. W., M. L. Goulden, and R. C. Bales (2020). "Wildfire controls on evapotranspiration in California's Sierra Nevada." *Journal of Hydrology*, 590, 125364. https://doi.org/10.1016/j.jhydrol.2020.125364
**Local path**: `not-acquired`; U.S. Forest Service record at https://research.fs.usda.gov/treesearch/62600
**Reference quality**: `verified-primary-peer-reviewed`.
**Topic**: First-year and recovery-scale ET response to burn severity and pre-fire vegetation density.
**openWEPP role**: Regional total-ET authority for severity response and the Stevens Canyon diagnostic target matrix.
**Rights / distribution**: `restricted` pending article-rights review; metadata and government record only.

## R-135: White et al. (2020) post-fire forest ET partition

**Citation**: White, D. A., et al. (2020). "The effect of wildfire on the structure and water balance of a high conservation value Hualo forest in central Chile." *Forest Ecology and Management*, 472, 118219. https://doi.org/10.1016/j.foreco.2020.118219
**Local path**: `not-acquired`.
**Reference quality**: `verified-primary-peer-reviewed`.
**Topic**: Pre-/post-fire transpiration, canopy interception, soil evaporation, total ET, and recovery.
**openWEPP role**: Component-partition authority showing that post-fire soil evaporation can rise while total ET declines.
**Rights / distribution**: `restricted`; remote DOI metadata only.

## Vegetation Constitutive Slice Reference Intake (2026-08-08)

These references were reviewed independently for the
`20260808-vegetation-radiation-interception-conductance-slice-001` work
package. RHESSys source expression was not inspected. Open copies are
vendored only where the artifact states an affirmative redistribution license;
all other acquired full text remains in the gitignored copyrighted cache.

## R-136: Best et al. (2011) JULES energy and water fluxes

**Citation**: Best, M. J., et al. (2011). "The Joint UK Land Environment
Simulator (JULES), model description - Part 1: Energy and water fluxes."
*Geoscientific Model Development*, 4, 677-699.
https://doi.org/10.5194/gmd-4-677-2011
**Local path**: `vendorable/Best2011_JULES_Part1.pdf`. SHA-256:
`84a909165937108a48d566ecce6a46d4b4c1fa3a3640c7a4b3d65a41c67355a7`.
**Reference quality**: `verified-primary-peer-reviewed-open-access`.
**Topic**: Surface energy balance, photosynthesis-linked conductance,
store-limited wet-canopy evaporation, finite-timestep throughfall and canopy
storage, and layer-resolved root-zone extraction.
**openWEPP role**: Equation-level candidate authority for canopy liquid
storage and throughfall (Eqs. 46-47) and normalized layer participation
(Eqs. 50-52); incompatibility evidence for a conductance slice that excludes
photosynthesis (Sect. 2.2).
**Rights / distribution**: `vendorable`; article text states Creative Commons
Attribution 3.0.

## R-137: Forrester et al. (2014) forest light absorption

**Citation**: Forrester, D. I., R. Guisasola, X. Tang, A. T. Albrecht,
T. L. Dong, and G. le Maire (2014). "Using a stand-level model to predict
light absorption in stands with vertically and horizontally heterogeneous
canopies." *Forest Ecosystems*, 1, 17.
https://doi.org/10.1186/s40663-014-0017-0
**Local path**: `vendorable/Forrester2014_LightAbsorption.pdf`. SHA-256:
`e37b393b6f05f9b202c3c4ac2a8c19a60cb1f84945bc51118aa13098f2d9dbb0`.
**Reference quality**: `verified-primary-peer-reviewed-open-access`.
**Topic**: Lambert-Beer absorbed-radiation fraction, top-down canopy-layer
allocation, and limitations caused by horizontal and vertical heterogeneity.
**openWEPP role**: Equation-level candidate for component-specific canopy
radiation receipt (Eq. 1) and evidence that extinction coefficients are
species-, architecture-, and period-specific inputs rather than universal
defaults (Sect. 2.2 and Eq. 2).
**Rights / distribution**: `vendorable`; article text states Creative Commons
Attribution 4.0.

## R-138: Bonan et al. (2014) stomatal conductance and hydraulics

**Citation**: Bonan, G. B., M. Williams, R. A. Fisher, and K. W. Oleson
(2014). "Modeling stomatal conductance in the earth system: linking leaf
water-use efficiency and water transport along the soil-plant-atmosphere
continuum." *Geoscientific Model Development*, 7, 2193-2222.
https://doi.org/10.5194/gmd-7-2193-2014
**Local path**: `vendorable/Bonan2014_StomatalConductance.pdf`. SHA-256:
`f30cf69192383fd10e231f858c81e5ad9a5649e653bef419c638b428a4b32fe0`.
**Reference quality**: `verified-primary-peer-reviewed-open-access`.
**Topic**: Ball-Berry and soil-plant-atmosphere conductance models, canopy
radiation, root hydraulics, water-use efficiency, and hydraulic safety.
**openWEPP role**: Strong incompatibility and scope evidence: the evaluated
mechanistic forest-conductance formulations require photosynthesis and/or
plant hydraulics that the current slice excludes.
**Rights / distribution**: `vendorable`; article text states Creative Commons
Attribution 3.0.

## R-139: Lasch-Born et al. (2020) forest model 4C v2.2

**Citation**: Lasch-Born, P., et al. (2020). "Description and evaluation of
the process-based forest model 4C v2.2 at four European forest sites."
*Geoscientific Model Development*, 13, 5311-5343.
https://doi.org/10.5194/gmd-13-5311-2020
**Local path**: `vendorable/LaschBorn2020_4C_v2_2.pdf`. SHA-256:
`2a82f7123cbf262c2845be2ac87c41b998a308f8f28f4b55cb376fe6c61d4e19`.
**Reference quality**: `verified-primary-peer-reviewed-open-access`.
**Topic**: Cohort forest radiation, interception, photosynthesis-linked
stomatal conductance, potential transpiration, and layer root shares.
**openWEPP role**: Cross-model corroboration for the process topology and
evidence that a forest-mechanistic conductance chain normally crosses the
current package's photosynthesis exclusion.
**Rights / distribution**: `vendorable`; article text states Creative Commons
Attribution 4.0.

## R-140: Pereira et al. (2016) wet-canopy evaporation

**Citation**: Pereira, F. L., F. Valente, J. S. David, N. Jackson,
F. Minunno, and J. H. Gash (2016). "Rainfall interception modelling: is the
wet bulb approach adequate to estimate mean evaporation rate from
wet/saturated canopies in all forest types?" *Journal of Hydrology*, 534,
606-615. https://doi.org/10.1016/j.jhydrol.2016.01.035
**Local path**: `copyrighted/Pereira2016_WetCanopy.pdf`, accepted manuscript.
SHA-256:
`634d235c0a82e0723dcc5144ecaabed18cb7b426542ee2b783aa64f53a2abca0`.
**Reference quality**: `verified-primary-peer-reviewed-author-manuscript`.
**Topic**: Wet-canopy evaporation in Gash interception modeling; comparison
of Penman-Monteith and wet-bulb approaches across ventilation regimes.
**openWEPP role**: Regime guard: Penman-Monteith is appropriate for canopies
that are not fully ventilated, while sparse, fully ventilated canopies require
separate treatment; canopy cover alone does not identify the regime.
**Rights / distribution**: `copyrighted-cache`; accepted manuscript states
CC BY-NC-ND 4.0. It is kept gitignored under the repository's conservative
noncommercial/no-derivatives policy.

## R-141: Jarvis (1976) environmental stomatal response

**Citation**: Jarvis, P. G. (1976). "The interpretation of the variations in
leaf water potential and stomatal conductance found in canopies in the
field." *Philosophical Transactions of the Royal Society B*, 273(927),
593-610. https://doi.org/10.1098/rstb.1976.0035
**Local path**: `copyrighted/jarvis1976.pdf` and the operator-supplied
transcription `copyrighted/jarvis1976.md`. SHA-256: PDF
`c8f683110be5b0ce033106466f237f21ff28b2fb02f4f3c9640f1838930ccb10`;
Markdown
`e4ae72367fbae040b0791c661340b1349089f948c637147e2063e93c3ecd9ae5`.
**Reference quality**: `verified-primary-peer-reviewed-full-text`.
**Topic**: Multiplicative empirical stomatal response to irradiance,
temperature, humidity deficit, leaf water status, and carbon dioxide.
**openWEPP role**: Primary leaf-scale authority for the descriptive product of
bounded light, temperature, vapour-pressure-deficit, leaf-water-potential, and
carbon-dioxide responses (Eqs. 4-9). The article explicitly treats the product
assumption as provisional, fits parameters from field data, and reports
species/season differences; it does not supply canopy aggregation or
transferable defaults.
**Rights / distribution**: `copyrighted-cache`; the PDF states Royal Society
copyright and no affirmative redistribution license. Both files remain
gitignored.

## R-142: Stewart (1988) pine-forest surface conductance

**Citation**: Stewart, J. B. (1988). "Modelling surface conductance of pine
forest." *Agricultural and Forest Meteorology*, 43(1), 19-35.
https://doi.org/10.1016/0168-1923(88)90003-2
**Local path**: `copyrighted/stewart1988.pdf` and the operator-supplied
transcription `copyrighted/stewart1988.md`. SHA-256: PDF
`df1719eb3c7b6f78c3d2d55509b077565bc7fef0e7744d67d80c6b907d06c598`;
Markdown
`ec0ef1e89187472a5428daa8d62c7dea77b76250fa3f2dc925ec76f5a1dc5652`.
**Reference quality**: `verified-primary-peer-reviewed-full-text`.
**Topic**: Jarvis-style pine-forest surface conductance driven by solar
radiation, humidity deficit, temperature, and soil moisture deficit, with
separate calibration and validation subsets.
**openWEPP role**: Primary stand-scale authority for a dry-canopy empirical
candidate, `g_s = L K_1 g(S_t) g(delta q) g(T) g(delta theta)` (Eqs. 12 and
17-24). Alternate-day validation reproduced total transpiration within 1%, but
parameters fitted to 1976 underestimated 1974 and 1975 totals by 14% and 11%
and biased low/high conductance. Admission must therefore remain limited to a
stated pine-forest domain with calibrated parameters; the paper does not
support universal defaults.
**Rights / distribution**: `copyrighted-cache`; the Elsevier journal artifact
states no affirmative redistribution license. Both files remain gitignored.

## R-143: Kelliher et al. (1995) maximum vegetation conductance

**Citation**: Kelliher, F. M., R. Leuning, M. R. Raupach, and E.-D. Schulze
(1995). "Maximum conductances for evaporation from global vegetation types."
*Agricultural and Forest Meteorology*, 73(1-2), 1-16.
https://doi.org/10.1016/0168-1923(94)02178-M
**Local path**: `copyrighted/kelliher1995.pdf` and the operator-supplied
transcription `copyrighted/kelliher1995.md`. SHA-256: PDF
`84dbc68328d6ea8686753057e95242deb7eff1f266cc6a72943f0e318c57b95b`;
Markdown
`ffe32d37153ccbd87663f6633d21635145588b4da8068b8972c64f7612020a99`.
**Reference quality**: `verified-primary-peer-reviewed-full-text`.
**Topic**: Observed maximum stomatal and bulk surface conductances across
vegetation types and their relationship to leaf area index.
**openWEPP role**: Primary scale and parameter check for empirical conductance.
It distinguishes leaf stomatal, bulk canopy, and bulk surface conductance;
integrates leaf conductance over LAI (Eqs. 3-7); and finds maximum values near
6 and 18 mm s-1 for natural-vegetation leaf and surface scales versus 12 and
32 mm s-1 for crops. These literature means are observational context, not
transferable production defaults.
**Rights / distribution**: `copyrighted-cache`; the Elsevier journal artifact
states no affirmative redistribution license. Both files remain gitignored.

## R-144: Misson, Panek, and Goldstein (2004) ponderosa-pine conductance

**Citation**: Misson, L., J. A. Panek, and A. H. Goldstein (2004). "A
comparison of three approaches to modeling leaf gas exchange in annually
drought-stressed ponderosa pine forests." *Tree Physiology*, 24(5), 529-541.
https://doi.org/10.1093/treephys/24.5.529
**Local path**: `not-acquired`; full text reviewed through the University of
California eScholarship record https://escholarship.org/uc/item/96747965.
**Reference quality**: `verified-primary-peer-reviewed-repository-full-text`.
**Topic**: Independent calibration/validation comparison of Jarvis,
Ball-Berry, and soil-plant-atmosphere conductance models for drought-stressed
ponderosa pine.
**openWEPP role**: Counterevidence against casually adopting the simple
Jarvis option: the paper reports systematic vapor-pressure-deficit error and
better performance from a photosynthesis-coupled model for its domain.
**Rights / distribution**: `restricted`; article states Heron Publishing
copyright and was not vendored.

## R-145: Cain (1998) plant-canopy evaporation review

**Citation**: Cain, J. D. (1998). *Modelling Evaporation from Plant Canopies*.
Institute of Hydrology Report No. 132. ISBN 0-948540-85-0.
https://nora.nerc.ac.uk/id/eprint/7373/
**Local path**:
`copyrighted/Cain1998_ModellingEvaporationPlantCanopies.pdf`. SHA-256:
`066e8c836786963748bd39601ad3dbea5abfc6a5c8cea53a209aee0f0474538d`.
**Reference quality**: `verified-secondary-government-technical-report`.
**Topic**: Review of Penman-Monteith variables, Jarvis-Stewart conductance,
aerodynamic resistance, radiation attenuation, multilayer and sparse-canopy
models, and forest parameter transferability.
**openWEPP role**: Discovery and incompatibility map only. It warns that
conductance parameters are model-specific, canopy scaling is nontrivial, and
aerodynamic resistance depends on roughness, source height, stability, and
canopy structure; it is not a substitute for primary equation authority.
**Rights / distribution**: `copyrighted-cache`; the report states Institute
of Hydrology copyright and no affirmative redistribution permission.

## R-146: RHESSysEastCoast licensed implementation source

**Citation**: Lin, L. (2021-). *RHESSysEastCoast* source repository.
https://github.com/laurencelin/RHESSysEastCoast
**Local path**: `/workdir/RHESSysEastCoast` at commit
`375c75b1cd2202217651dff43aa113d80b9c1118`.
**Reference quality**: `verified-primary-implementation-source`.
**Topic**: East Coast RHESSys canopy strata, radiation, interception,
Jarvis-style conductance response curves, Penman-Monteith water flux, Farquhar
photosynthesis, phenology, roots, allocation, and coupled state orchestration.
**openWEPP role**: Licensed implementation provenance and source-differential
comparator for the coupled vegetation successor. Exact source behavior is not
automatic scientific authority; cited equations, parameter domains, defaults,
sentinels, numerical floors, and experimental branches require contract
adjudication.
**Rights / distribution**: `MIT-vendorable`; repository `LICENSE` copyright
2021 Laurence Lin, SHA-256
`4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be`.

## R-147: GIS2RHESSys vegetation profiles and definition generation

**Citation**: Lin, L. (2021-). *GIS2RHESSys* source repository.
https://github.com/laurencelin/GIS2RHESSys
**Local path**: `/workdir/GIS2RHESSys` at commit
`6b20883dea7c9fd92f71ec69eaca015ebf6dfe18`.
**Reference quality**: `verified-primary-format-and-parameter-source`.
**Topic**: `vegCollection.csv` vegetation profiles and R generation of RHESSys
`stratum_*.def` files. Intake found 71 fields and 32 profiles spanning generic
and East Coast deciduous, evergreen, shrub, grass, and no-vegetation classes.
**openWEPP role**: Format-compatibility corpus and parameter-provenance input.
Mixed forest must compose explicit profile-bearing strata; profile presence is
not empirical calibration, universal transferability, or scientific validation.
Every consumed field and candidate profile requires a typed unit/domain,
citation/default audit, and deterministic compatibility disposition.
**Rights / distribution**: `MIT-vendorable`; repository `LICENSE` copyright
2021 Laurence Lin, same SHA-256 as R-146.
