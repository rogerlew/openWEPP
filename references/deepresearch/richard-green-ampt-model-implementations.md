# Research conclusion

I interpret the requirement as:

1. Infiltration is represented with the **Richards equation**, a Richards-derived approximation, or a **Green–Ampt-family** formulation.
2. The model can generate both:

   * **infiltration-excess runoff**: rainfall/runon exceeds infiltration capacity while the profile remains unsaturated below the surface;
   * **saturation-excess runoff**: rainfall falls on saturated soil, or groundwater/perched water exfiltrates to the surface.
3. It represents a diagnosable downward flux below the active soil profile: **deep percolation, groundwater recharge, or lower-boundary drainage**.

Under that definition, the strongest candidates are:

* **Best open, high-fidelity reference:** ParFlow
* **Best open, unstructured-mesh reference:** Amanzi-ATS
* **Best runoff-mechanism research model:** CATHY
* **Best practical distributed watershed/GIS model:** GSSHA
* **Best open model with selectable Green–Ampt and Richards formulations:** FeST
* **Best explicit partition of runoff sources:** tRIBS
* **Best commercial integrated model:** HydroGeoSphere or MIKE SHE
* **Best cold-region/mountain options:** GEOtop and WaSiM-Richards

If your wording means the model must provide **both Green–Ampt and Richards as selectable formulations in the same package**, the clean verified shortlist is **GSSHA, MIKE SHE, and FeST**. OpenLISEM nominally does as well, but its current documentation reports important groundwater-coupling and percolation limitations.

## Why this is a stricter requirement than “has Richards” or “has Green–Ampt”

A Richards solver alone does not guarantee both runoff mechanisms. It also needs an atmospheric or coupled surface boundary that can switch among:

* prescribed rainfall flux;
* infiltration-capacity-limited flux;
* ponded pressure head;
* upward exfiltration.

Likewise, Green–Ampt alone normally models infiltration-excess runoff. To obtain physically meaningful saturation-excess runoff, Green–Ampt must be coupled to a finite profile, a rising water table, lateral subsurface flow, or at least a layered storage model that can fill from below.

The distinction is also not always binary. A location can generate infiltration-excess runoff early in a storm, subsequently saturate, and then generate saturation-excess runoff. A 2026 unified-runoff study formalized precisely this spatial and temporal transition and showed why treating the two processes as unrelated alternatives is problematic. 

---

# 1. Strong physically coupled Richards-equation models

| Model              | Subsurface and surface formulation                                                                                                                                                       | Infiltration- and saturation-excess behavior                                                                                                                                                                                            | Deep percolation meaning                                                                                                    | Assessment                                                                                                                                                                                                                                                                                                        |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **ParFlow**        | Three-dimensional variably saturated Richards equation coupled to overland flow                                                                                                          | Infiltration rejection, surface ponding, saturated-area runoff, and groundwater exfiltration emerge from the coupled pressure/flux solution                                                                                             | Flux through any diagnostic plane, groundwater recharge, or domain-bottom flux                                              | Probably the strongest open-source structured-grid reference model. Scales well to HPC, but mechanism partition usually must be diagnosed from states and boundary fluxes rather than read from two named output variables. ([ParFlow][1])                                                                        |
| **Amanzi-ATS**     | Three-dimensional Richards flow on unstructured meshes, coupled to diffusion-wave surface flow through pressure and flux continuity                                                      | Both mechanisms emerge naturally; saturated cells can discharge upward to the surface                                                                                                                                                   | Recharge or flux through selected subsurface interfaces and lower boundaries                                                | Best open-source choice where irregular geology, variable soil depth, permafrost, or unstructured discretization matters. More framework-like than turnkey watershed software. ([Oak Ridge National Laboratory][2])                                                                                               |
| **CATHY**          | Three-dimensional Richards equation coupled to path-based, quasi-two-dimensional surface routing                                                                                         | Dynamic Neumann/Dirichlet boundary switching explicitly handles infiltration, ponding, exfiltration, Hortonian runoff, and Dunnian runoff                                                                                               | Subsurface flux at depth, recharge, and domain-boundary drainage                                                            | Arguably the best model for studying runoff-generation mechanisms themselves. Its surface routing is path-based rather than a general two-dimensional sheet-flow mesh. Current distribution is research-oriented and carries a research-use restriction. ([Frontiers][3])                                         |
| **HydroGeoSphere** | Fully integrated three-dimensional saturated/unsaturated flow with two-dimensional surface flow and one-dimensional channels                                                             | Both rejected rainfall and groundwater-driven surface discharge are emergent components of one coupled solution                                                                                                                         | Recharge, vadose-zone drainage, aquifer fluxes, and lower-boundary losses can be distinguished                              | One of the strongest commercial full-physics systems. Preferable to MIKE SHE when fully three-dimensional vadose-zone flow is more important than formulation flexibility or GUI workflow. ([aquanty][4])                                                                                                         |
| **MIKE SHE**       | One-dimensional Richards columns or Green–Ampt infiltration, coupled to a saturated groundwater model, overland flow, and channels                                                       | Rainfall can be rejected by infiltration capacity; shallow groundwater limits infiltration and can produce saturated-area runoff/exfiltration when saturated-zone feedback is enabled                                                   | Drainage below the root zone and transfer to the saturated zone are explicitly represented as deep percolation and recharge | The cleanest commercial answer when you specifically want selectable **Richards and Green–Ampt** methods in the same integrated platform. For rigorous saturation excess, use Richards plus saturated-zone feedback rather than an isolated Green–Ampt configuration. ([Mike SHE][5])                             |
| **SHETRAN**        | Distributed variably saturated subsurface flow coupled to overland and channel flow                                                                                                      | Designed to represent both infiltration-excess and saturation-excess generation, including groundwater–surface interactions                                                                                                             | Drainage through the soil profile, recharge, and groundwater flow                                                           | Mature catchment model with sediment and solute capabilities. Scientifically strong, but the current public software ecosystem and executable-centric repository are less straightforward than ParFlow or ATS. ([Newcastle University Research][6])                                                               |
| **GEOtop**         | Richards-equation soil water flow with terrain, energy balance, snow, and runoff routing                                                                                                 | Hortonian runoff occurs when rainfall exceeds infiltration capacity; Dunnian runoff occurs when the water table reaches the surface                                                                                                     | Lower-profile drainage or bottom-boundary flux; groundwater interpretation depends on domain depth and boundary conditions  | Excellent for small mountain and cold-region catchments where snow, freeze–thaw, radiation, and terrain are important. Less attractive for large operational watershed ensembles. ([American Meteorological Society Journals][7])                                                                                 |
| **GSSHA**          | One-dimensional Richards columns over a laterally distributed two-dimensional groundwater model and two-dimensional overland flow; alternative Green–Ampt variants are available         | Official documentation explicitly states that Richards can generate infiltration excess and saturation excess simultaneously in different watershed areas; vadose flow partitions rainfall among runoff, infiltration, recharge, and ET | Recharge from each vadose column to the groundwater domain and losses through configured lower boundaries                   | Probably the best practical match for a GIS-oriented watershed model. It is less computationally extreme than ParFlow but still has dynamic shallow-groundwater feedback. Standard Green–Ampt alone should not be trusted for saturation excess where the water table is shallow. ([CONTENTdm][8])                |
| **WaSiM-Richards** | One-dimensional Richards columns for raster cells, directly coupled to multilayer lateral groundwater flow                                                                               | A rising groundwater table can saturate the soil column and generate surface runoff; infiltration excess comes from the vertical soil solution                                                                                          | Layer fluxes, bottom-of-soil-column flux, and transfer to groundwater are available                                         | An underrated operational option, especially for snow-dominated and mountain basins. It is actively maintained and has Linux/OpenMP/MPI distributions, although source and licensing access are less obvious than for ParFlow or FeST. ([Wasim][9])                                                               |
| **FeST/FEST**      | Selectable Green–Ampt, Ross Richards with Brooks–Corey retention, or Ross Richards with van Genuchten retention; two-zone soil balance, lateral saturated flow, and groundwater coupling | Explicit root-zone and transmission-zone saturation excess; water table in the root zone causes rainfall to become runoff                                                                                                               | Explicit `DP` from the transmission zone; in plains over an aquifer this contributes to groundwater recharge                | Particularly interesting for your criterion: one GPL-licensed Fortran model contains both Green–Ampt and Richards options and explicitly names saturation excess and deep percolation. The subsurface representation is reduced-order compared with ParFlow or HGS, but considerably more auditable. ([Fest][10]) |

## Important distinction within this group

ParFlow, ATS, CATHY, HydroGeoSphere, and similar integrated solvers produce runoff through **boundary-condition physics**. They generally do not have a hard-coded rule saying “this millimeter is Hortonian and that millimeter is Dunne runoff.” That partition has to be reconstructed.

GSSHA, WaSiM, MIKE SHE, and FeST are easier to configure as conventional watershed models, but they reduce the dimensionality of some subsurface processes. That is not necessarily a disadvantage when the objective is watershed-scale production rather than detailed three-dimensional groundwater architecture.

---

# 2. Reduced-order models that still represent the complete requested process chain

These are not all full Richards solvers, but they may be more useful operationally because the runoff components are explicit and computational cost is much lower.

| Model                          | Formulation                                                                                                                                                                                  | How well it satisfies the requirement                                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **tRIBS**                      | Multiple moving moisture fronts—essentially a sophisticated Green–Ampt/front-tracking family—coupled dynamically to the vadose zone, perched saturation, and groundwater table on a TIN      | Exceptionally good. It can output separate hydrograph components for infiltration excess, saturation excess, perched return flow, and groundwater exfiltration, while also calculating unsaturated-zone outflow and recharge. This is the best model I found for directly auditing runoff-source partition without postprocessing a full Richards solution. ([tRIBS][11])                                                             |
| **WEPP**                       | Modified Green–Ampt–Mein–Larson infiltration, layer-by-layer redistribution and storage-routing percolation, with enhanced lateral-flow/percolation routines                                 | It does represent infiltration excess, profile saturation excess, deep percolation from the bottom layer, lateral flow, and baseflow. But its saturation excess is principally a layered storage/overflow mechanism, not a resolved dynamic groundwater surface and variable-source-area solution. It qualifies algorithmically, but not at the same physical level as GSSHA-Richards or tRIBS. ([WEPPcloud][12])                     |
| **RoGeR**                      | Green–Ampt/Mein–Larson-type infiltration with spatially distributed root-zone and subsoil balances, lateral subsurface flow, groundwater, capillary rise, and optional macropore/bypass flow | A strong open research implementation for explicitly studying Hortonian flow, saturated overland flow, deep percolation, and preferential flow. Some routing and groundwater modules are still described as under testing, so I would evaluate module maturity before production use. ([water][13])                                                                                                                                   |
| **PIHM / MM-PIHM / SHUD**      | Low-dimensional, finite-volume or integral approximations derived from Richards/Darcy flow, coupled to overland flow, channels, and groundwater                                              | These can generate rejected infiltration, saturated-area runoff, exfiltration, recharge, and deep groundwater flux efficiently on unstructured meshes. They should be described as Richards-derived integrated models rather than full vertical-profile Richards solvers. Excellent compromise where ParFlow is too expensive. ([Penn State Pihm][14])                                                                                |
| **EPA SWMM**                   | Green–Ampt infiltration plus a conceptual two-zone groundwater/aquifer system                                                                                                                | Can reject rainfall through Green–Ampt, reduce or stop infiltration when the water table reaches the surface, and represent groundwater discharge and deep aquifer loss. It satisfies the flux checklist but only at subcatchment scale; it does not resolve moving saturated source areas within a hillslope. ([EPA Nepis][15])                                                                                                      |
| **HEC-HMS Layered Green–Ampt** | Green–Ampt infiltration into layered continuous soil storage with percolation and groundwater/baseflow stores                                                                                | Saturated layers can constrain infiltration and produce excess runoff; lower-zone drainage may be divided between groundwater/baseflow and deep aquifer recharge. Useful for efficient lumped or semi-distributed modeling, but saturation excess is storage filling rather than an explicit laterally connected water table. ([HEC Army Corps of Engineers][16])                                                                     |
| **OpenLISEM**                  | Green–Ampt or SWATRE profile solution coupled to two-dimensional event-scale overland flow, erosion, and simplified groundwater                                                              | It nominally satisfies much of the requirement and is attractive for erosion studies. However, the current project documentation says SWATRE does not interact with groundwater, identifies groundwater–channel coupling as experimental, and notes recent percolation/mass-balance defects. I would not use it as the reference implementation for this particular comparison without code-level verification. ([Springer Link][17]) |

---

# 3. Models that nearly qualify but fail an important part of the test

### LGAR

LGAR is a strong layered Green–Ampt infiltration and redistribution method, but its standard formulation assumes that the groundwater table does not affect infiltration. It therefore needs an external groundwater/saturation module before it can represent rising-water-table saturation excess. ([NOAA Institutional Repository][18])

### MODFLOW 6 UZF

UZF handles infiltration rejection, unsaturated storage, ET, recharge, groundwater discharge, and runoff, but it uses a one-dimensional kinematic-wave approximation that neglects capillarity. It is neither Richards nor Green–Ampt in the strict sense. It is an excellent groundwater-centric comparator, not an exact match. ([USGS Water Resources][19])

### SWAT/SWAT+

SWAT can use Green–Ampt and calculate profile drainage/deep percolation. Standard configurations, however, do not cleanly couple Green–Ampt infiltration to a spatially dynamic shallow groundwater table and variable saturated source area. Modified SWAT variants such as SWAT-VSA or SWAT-wil address portions of that problem, but then the runoff physics are no longer the standard model. ([SWATplus][20])

### HEC-RAS rain-on-grid

HEC-RAS has Green–Ampt and Green–Ampt-with-redistribution infiltration coupled to two-dimensional surface hydraulics, but the normal rain-on-grid configuration lacks a continuous, laterally connected groundwater and deep-recharge system. It is suitable for infiltration-excess flooding, not this full process chain. ([HEC Army Corps of Engineers][21])

### HYDRUS

HYDRUS is an excellent Richards-equation soil-column or local-domain model and supports atmospheric switching, ponding, and free-drainage/lower-boundary flux. Standalone HYDRUS does not supply catchment-scale surface routing or moving variable source areas. HYDRUS–KINEROS coupling is promising, but it is a composite system rather than one mature integrated model. ([ARS][22])

### WRF-Hydro

WRF-Hydro can produce infiltration-excess runoff, subsurface saturation/exfiltration, and deep drainage through its land-surface model and groundwater bucket. Whether it satisfies the Richards/Green–Ampt requirement depends on the selected land-surface model and configuration; it is not a single invariant runoff formulation. ([Research Applications Laboratory][23])

### VIC, TOPMODEL, and DHSVM

These can be very capable saturation-excess or variable-source-area models, but their characteristic runoff formulations generally do not meet a strict requirement for Green–Ampt or Richards infiltration. They remain useful conceptual controls.

---

# 4. Deep percolation is not a single interchangeable flux

For model comparison, I would require three different outputs:

[
D_{\mathrm{root}} =
q_z(z_{\mathrm{root\ bottom}})
]

Drainage below the root zone. This water may remain in the vadose zone for days, months, or years.

[
R_{\mathrm{gw}} =
q_{\mathrm{across\ water\ table}}
]

Actual groundwater recharge. This may differ greatly from root-zone drainage because of transient vadose storage, capillary rise, lateral diversion, and ET below the nominal root depth.

[
D_{\mathrm{base}} =
q_z(z_{\mathrm{model\ bottom}})
]

Water leaving the modeled geologic domain. Depending on the lower boundary, this could mean regional aquifer leakage, an artificial free-drainage loss, or simply water leaving an insufficiently deep mesh.

A model reporting “deep percolation” may be reporting any one of these. For example:

* WEPP usually means loss from the bottom of the represented soil profile.
* FeST explicitly labels transmission-zone loss as `DP` and may transfer it to groundwater recharge.
* MIKE SHE distinguishes root-zone drainage and recharge to the saturated zone.
* ParFlow and ATS let you calculate flux across whatever geometric interface you define.

Those fluxes should not be compared without mapping them onto a common control volume.

---

# 5. Runoff attribution should use at least three components

A robust output schema should not force everything into only “infiltration excess” and “saturation excess.” I would report:

[
Q_{\mathrm{IE}}
]

Rainfall or runon rejected while the soil immediately below the surface remains unsaturated.

[
Q_{\mathrm{rain,sat}}
]

Rainfall or runon delivered to an already saturated surface or a profile that becomes saturated during the time step.

[
Q_{\mathrm{return}}
]

Upward groundwater or perched-water exfiltration independent of concurrent rainfall.

Then separately track:

* reinfiltration of runon;
* surface/depression storage;
* channel losses;
* tile/drain discharge;
* lateral subsurface stormflow.

This matters because a full Richards model can produce groundwater exfiltration before rainfall begins. Calling all of that “saturation-excess rainfall runoff” hides a physically distinct source. tRIBS is unusually good here because it already distinguishes several of these hydrograph components.

---

# 6. Selection by objective

| Objective                                                    | First choices            | Reason                                                                                                                |
| ------------------------------------------------------------ | ------------------------ | --------------------------------------------------------------------------------------------------------------------- |
| **High-fidelity open benchmark**                             | ParFlow, ATS             | Most defensible coupled surface–subsurface physics; appropriate as numerical references                               |
| **Explicit IE/SE/return-flow attribution**                   | tRIBS, CATHY             | Runoff mechanisms are exposed more directly than in most integrated solvers                                           |
| **Practical raster/GIS catchment model**                     | GSSHA                    | Richards, Green–Ampt options, two-dimensional overland flow, groundwater, recharge, and operational watershed tooling |
| **Compare Green–Ampt and Richards inside one open codebase** | FeST                     | Both formulations are directly selectable while retaining common forcing, grid, and water-balance machinery           |
| **Commercial integrated workflow**                           | MIKE SHE, HydroGeoSphere | MIKE SHE for modularity and both infiltration formulations; HGS for fully three-dimensional integrated physics        |
| **Cold regions and mountains**                               | GEOtop, WaSiM, ATS       | Better snow, energy, terrain, and/or freeze–thaw representations                                                      |
| **Erosion and land-management response**                     | WEPP, SHETRAN            | Stronger erosion or sediment context, although WEPP has reduced-order saturation physics                              |
| **Efficient unstructured watershed simulation**              | PIHM/SHUD, tRIBS         | Much lower cost than full three-dimensional Richards while retaining groundwater feedback                             |

---

## References

[1]: https://parflow.org/ "https://parflow.org/"
[2]: https://impact.ornl.gov/en/publications/coupling-surface-flow-and-subsurface-flow-in-complex-soil-structu/?utm_source=chatgpt.com "Coupling surface flow and subsurface flow in complex soil structures using mimetic finite differences - Oak Ridge National Laboratory"
[3]: https://www.frontiersin.org/journals/water/articles/10.3389/frwa.2025.1553578/full?utm_source=chatgpt.com "Exploration of coupled surface–subsurface hydrological ..."
[4]: https://www.aquanty.com/hydrogeosphere?utm_source=chatgpt.com "HydroGeoSphere — aquanty"
[5]: https://mikeshe-mike-platform-prod.eu.mike-cloud.com/2026/WM/UZ/Richards/ "Richards Equation - MIKE SHE"
[6]: https://research.ncl.ac.uk/shetran/ "https://research.ncl.ac.uk/shetran/"
[7]: https://journals.ametsoc.org/view/journals/hydr/7/3/jhm497_1.xml "https://journals.ametsoc.org/view/journals/hydr/7/3/jhm497_1.xml"
[8]: https://usace.contentdm.oclc.org/digital/api/collection/p266001coll1/id/3870/download "ERDC TR-13-15 \"Development of a Coupled Framework for Simulating Interactive Effects of Frozen Soil Hydrological Dynamics in Permafrost Regions\""
[9]: https://www.wasim.ch/downloads/doku/wasim/wasim_2007_en.pdf "https://www.wasim.ch/downloads/doku/wasim/wasim_2007_en.pdf"
[10]: https://www.fest.polimi.it/doc/sourcefile/infiltration.f90.html "Infiltration.f90 – FEST"
[11]: https://tribshms.readthedocs.io/ "https://tribshms.readthedocs.io/"
[12]: https://wepp.cloud/weppcloud/usersum/doc/usersum.weppcloud.wepp_model "https://wepp.cloud/weppcloud/usersum/doc/usersum.weppcloud.wepp_model"
[13]: https://water.usask.ca/documents/events/wieler_dicussion_paper.pdf "Model-based quantification of runoff generation processes at high spatial and temporal resolution"
[14]: https://www.pihm.psu.edu/Downloads/Articles/QU%26DUFFY_07.pdf "https://www.pihm.psu.edu/Downloads/Articles/QU%26DUFFY_07.pdf"
[15]: https://nepis.epa.gov/Exe/ZyPURL.cgi?Dockey=P100N3J6.TXT "https://nepis.epa.gov/Exe/ZyPURL.cgi?Dockey=P100N3J6.TXT"
[16]: https://www.hec.usace.army.mil/confluence/hmsdocs/hmstrm/canopy-surface-infiltration-and-runoff-volume/infiltration/layered-green-and-ampt-model "https://www.hec.usace.army.mil/confluence/hmsdocs/hmstrm/canopy-surface-infiltration-and-runoff-volume/infiltration/layered-green-and-ampt-model"
[17]: https://link.springer.com/article/10.1007/s12665-020-08914-7 "https://link.springer.com/article/10.1007/s12665-020-08914-7"
[18]: https://repository.library.noaa.gov/view/noaa/53741/noaa_53741_DS1.pdf "https://repository.library.noaa.gov/view/noaa/53741/noaa_53741_DS1.pdf"
[19]: https://water.usgs.gov/nrp/gwsoftware/ModelMuse/Help/uzf6_unsaturated_zone_flow_pac.html "https://water.usgs.gov/nrp/gwsoftware/ModelMuse/Help/uzf6_unsaturated_zone_flow_pac.html"
[20]: https://swatplus.gitbook.io/io-docs/theoretical-documentation/section-2-hydrology/chapter-2-1-surface-runoff/2-1.2-runoff-volume-green-and-ampt-infiltration-method "https://swatplus.gitbook.io/io-docs/theoretical-documentation/section-2-hydrology/chapter-2-1-surface-runoff/2-1.2-runoff-volume-green-and-ampt-infiltration-method"
[21]: https://www.hec.usace.army.mil/confluence/rasdocs/ras1dtechref/6.5/overview-of-optional-capabilities/modeling-precipitation-and-infiltration/green-ampt "https://www.hec.usace.army.mil/confluence/rasdocs/ras1dtechref/6.5/overview-of-optional-capabilities/modeling-precipitation-and-infiltration/green-ampt"
[22]: https://www.ars.usda.gov/pacific-west-area/riverside-ca/agricultural-water-efficiency-and-salinity-research-unit/docs/model/hydrus-1d-model/ "https://www.ars.usda.gov/pacific-west-area/riverside-ca/agricultural-water-efficiency-and-salinity-research-unit/docs/model/hydrus-1d-model/"
[23]: https://ral.ucar.edu/sites/default/files/public/WRF-HydroV5TechnicalDescription_0.pdf "https://ral.ucar.edu/sites/default/files/public/WRF-HydroV5TechnicalDescription_0.pdf"

