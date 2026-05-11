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
