# Supported Domain And Exclusions

Status: `selected`

Evidence mode: `Static`

`OPENWEPP_C3_WOODY_V1` admits vascular C3 evergreen and seasonal-deciduous
tree/shrub strata, including vertically overlapping mixed stands represented
as non-overlapping horizontal topology columns. Every stratum retains its ID,
rank, cover, geometry, parameter-set digest, root profile, state, and ledger.

Required operation includes direct/diffuse VIS/NIR radiation, explicit PAR,
sunlit/shaded gas exchange, liquid interception, leaf energy balance,
interval-equilibrium hydraulic potentials, layer water requests/receipts, persistent C/N
pools, respiration, allocation, storage, transfer, retranslocation, turnover,
mortality, mineral-N requests/receipts, and litter/CWD transfers.

| Input/process | Disposition |
|---|---|
| C4, crop, or nonvascular lifeform | typed `UnsupportedLifeform`; no state mutation |
| canopy snow/ice | typed `UnsupportedProcess::CanopySnow`; ground snow remains `SC-SNOWFREEZE-001` owned |
| recruitment, succession, fire, catastrophic disturbance | typed unsupported process |
| absent stratum parameter or initial-state member | typed schema/state error; no default or synthesis |
| zero strata and zero vegetation stores | valid empty-stand degenerate |
| zero leaf C/LAI | valid dormant/bare stratum; zero leaf gas exchange, persistent nonleaf state retained |
| mixed forest | explicit strata/topology only; profile averaging is invalid configuration |

GSI may supply a bounded onset/offset activity signal. It never directly owns
LAI: `LAI_s = leaf_carbon_s * SLA_s` on the same ground-area basis after
accepted C/N transfers. The version does not make calibration, site
suitability, validation, or geographic transferability claims.
