# Primary-Source Ledger

Status: `PROCESS-AUTHORITY LEADS; NO COMPLETE FAMILY ADMITTED`

| Family | Primary source | Authorized use in this package | Remaining admission work |
|---|---|---|---|
| wet-canopy interception | Gash (1979), DOI `10.1002/qj.49710544304` | Supports finite wet-canopy storage/evaporation as a distinct component. | Exact selected formulation, units, parameters, cadence, guards, and vectors. |
| canopy/soil resistance | Shuttleworth and Wallace (1985), *QJRMS* 111:839-855 | Supports independently constrained canopy and soil flux components. | Exact two-source energy/resistance implementation and ownership chain. |
| multilayer radiation | Verstraete (1988), NASA NTRS `19880062508` | Supports explicit multilayer radiative transfer rather than ignored optical fields. | Selected operator, direct/diffuse bands, closure, invalid-optics policy. |
| root uptake | Javaux et al. (2013), DOI `10.2136/vzj2013.02.0042`; Cai et al. (2018), DOI `10.5194/hess-22-2449-2018` | Supports root-distribution and soil-state control of uptake. | Selected layer request law, stress response, competition, frozen/dry branches, vectors. |
| C3 temperature response | Medlyn et al. (2002), DOI `10.1046/j.1365-3040.2002.00891.x`; Bernacchi et al. (2013), DOI `10.1111/pce.12118` | Supports explicit temperature-dependent C3 capacity semantics. | Complete selected C3 equation family, capacity inputs, scaling, convergence, tests. |
| canopy conductance | Samanta et al. (2008), DOI `10.1029/2007WR006761` | Supports independent canopy-conductance evaluation and scale scrutiny. | Selected conductance law, LAI basis, domains, parameter classes, vectors. |

Open-ended literature search remains appropriate for unresolved constitutive
families. It is not appropriate for choosing every user's site-specific value.
