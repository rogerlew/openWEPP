# Equation Module Map

Status: `REOPENED / V2 occupancy integration active`

Evidence mode: `Static + Ran`

The V1 fixture remains historical equation evidence. V2 topology authority is
bound by `openwepp_c3_woody_v2_topology_vectors.json`, SHA-256
`c02e5e2a2287d84cfc584a6e3ec9c499cf7168160bc71f2577323f19dcb50bf1`.
The complete capped physiology gate is named
`STAGE_B_E11_E15_EXACT_ORACLE` and must pass before E11--E15 return to focused
pass status.

| Equation | Production function | Independent vector | Positive/poison/guard evidence | Whole-transaction consumer | Status |
|---|---|---|---|---|---|
| E01 | `radiation::two_stream` matrix exponential/particular solution | radiation fixed vector | direction/band closure and Beer poison; leaf-angle/nonfinite guards | `transaction::radiation_by_stratum` | focused pass |
| E02 | `radiation::sunlit_absorption` and analytic sunlit LAI | sunlit area/absorption | sun/shade distinction and zero-direct branch | energy leaf-class construction | focused pass |
| E03 | pending V2 tile-column radiation traversal | mixed-top/bottom oracle family | ordered tile/rank traversal; no mixed-stratum averaging | public candidate preparation | V2 public path pending |
| E04 | `interception::liquid_interception` plus V2 tile-column routing | V1 wet/condensation and V2 topology vectors | subfreezing rejection, occupancy identity, same-tile drainage/stemflow routing, and exact water closure | V2 public tile-column candidate path | authority admitted; implementation active |
| E05 | `liquid_interception`; `energy::canopy_residual` | integrated wet-canopy vector | dry/wet leaf/stem identity and active store cap | capped coupled solve | focused pass |
| E06 | `energy::{neutral_resistance,leaf_boundary_conductance}` | aerodynamic/energy vectors | calm/nonneutral and domain guards | `energy_input` | focused pass |
| E07 | `photosynthesis::fvcb` | Rubisco/electron/zero/saturated vectors | limitation and compensation branches | both solved leaf classes | focused pass |
| E08 | `photosynthesis::electron_transport` | zero/electron/saturated vectors | zero light and capacity guards | `fvcb` | focused pass |
| E09 | `photosynthesis::smaller_root`; `fvcb` | co-limitation vectors | stable smaller-root implementation and discriminant error | `solve_ci` | focused pass |
| E10 | `photosynthesis::{arrhenius,peaked_response}` | digest-bound biochemical vectors | stable log-domain response; NaN/zero-capacity guards | leaf-temperature residual | focused pass |
| E11 | `photosynthesis::{medlyn,solve_ci}`; V2 coupled orchestrator pending | coupled-leaf vector | surface VPD and `beta_hyd` solve; ambient-VPD/one-pass poisons | public potential and capped solves | pure kernel retained; V2 public path pending |
| E12 | `photosynthesis::{carbon_surface,solve_ci}` | coupled-leaf `ci/cs` vector | boundary-resistance distinction and Brent guards | energy leaf nodes | focused pass |
| E13 | `energy::solve_canopy_energy` | integrated energy vectors | six-node residual, wet-store cap, dry-stem owner | public physical candidate | focused pass |
| E14 | `hydraulics::{vulnerability,solve_hydraulics}` | four-node and root-profile vectors | gravity/path/series conductance; redistribution poison | potential and capped solves | focused pass |
| E15 | `hydraulics::hydraulic_residual`; V2 cap orchestrator pending | full/cap-active vectors | exact cap amount/rate conversion and gas/hydraulic equality | water requests/final uses | pure kernel retained; V2 public path pending |
| E16 | `carbon_nitrogen::gpp_kg_c` | C/N vector | gross/net separation and molar conversion | final physical state | focused pass |
| E17 | `update_t10`; `maintenance_respiration`; `carbon_offer` | C/N vector | tissue/layer respiration and signed reserve priority | final carbon offer | focused pass |
| E18 | `nitrogen_demand`; `finalize_growth` | six-tissue allocation vector | N-sufficient/limited common-eta allocation and NSC retention | post-N-authorization state | focused pass |
| E19 | pending V2 post-occupancy aggregation; proportional BGC arbitration | request/finalized bucket vectors | layer/species identity and unused-authorization poisons | public N request/auth/use path | V2 public path pending |
| E20 | `advance_phenology`; leaf-C/SLA assignment | deciduous/evergreen vectors | onset/offset edge trajectory and GSI timing-only posture | persistent stratum state | focused pass |
| E21 | `advance_turnover` | fine-root/livewood/CWD vectors | ordered bounded turnover and receiver identity | material proposals | focused pass |
| E22 | `material_transfer`; BGC `MaterialReceipt` | litter C/N/DM vectors | carbon-as-dry-matter and duplicate-receipt poisons | BGC receiver candidate | focused pass |

`c3_vegetation_implementation_contract` currently proves retained pure-kernel
vectors and that the V2 public consumer fails closed before E04. It does not
claim that the public consumer invokes E01--E22.

All earlier `focused pass` rows describe the historical single-topology
remediation checkpoint. Each row must be revalidated through the V2 public path;
no helper-only row is terminally implemented.
