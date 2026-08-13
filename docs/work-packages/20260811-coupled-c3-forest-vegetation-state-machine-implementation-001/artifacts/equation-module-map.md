# Equation Module Map

Status: `executing / V6 capped oracle PASS / public water transaction pending`

Evidence mode: `Static + Ran`

The V1 fixture remains historical equation evidence. V2 topology authority is
bound by `openwepp_c3_woody_v2_topology_vectors.json`, SHA-256
`c02e5e2a2287d84cfc584a6e3ec9c499cf7168160bc71f2577323f19dcb50bf1`.
The complete capped physiology gate is named
`STAGE_B_E11_E15_EXACT_ORACLE`. It passes on the final V6 identity. This is
focused equation/ownership evidence only; it does not prove that the public
transaction consumes the accepted capped column.

V5 imports V4 shared-state ownership and V3 radiation/uncapped constitutive
authority unchanged. It now binds the previously missing capped-pass amount/
rate conversion, independent hydraulic-law operand, equality-active branch,
generalized Jacobian, diagnostics, rollback, and exact cap-active vectors.
Production implementation and independent Rust reconstruction pass the focused
capped gate. The public path retains no integration or completion claim until
the complete water transaction consumes these kernels.

| Equation | Production function | Independent vector | Positive/poison/guard evidence | Whole-transaction consumer | Status |
|---|---|---|---|---|---|
| E01 | `radiation::{mixed_optics,solve_mixed_column}` matrix exponential/particular solution | V3 authority radiation vectors | direction/band closure, mixed-optics, clumping, Beer, and leaf-angle guards | V3 occupancy radiation preparation | focused V3 kernel pass; public consumer pending |
| E02 | `radiation::{sunlit_absorption,partition_owned_absorption}` | V3 sunlit/owner vectors | stem-only zero-photosynthesis, absorptivity weighting, direct/diffuse and VIS/NIR identity | energy leaf/stem owners | focused V3 kernel pass; public consumer pending |
| E03 | `radiation::solve_mixed_column` ordered full-column traversal | V3 two-rank directional fixture | nonzero upward lower boundary, rank/identity swaps, terminal closure | public candidate preparation | focused V3 kernel pass; public consumer pending |
| E04 | `interception::liquid_interception`; `column::execute_tile_columns` internal engine | V1 wet/condensation plus controlled V2 topology vectors | conditional-area poison; same-tile throughfall/both-drainage routing; stemflow bypass; local/column/stand closure | V2 public tile-column candidate path | internal routing pass; exact occupancy solver/public path pending |
| E05 | `liquid_interception`; `energy::canopy_residual` | integrated wet-canopy vector | dry/wet leaf/stem identity and active store cap | capped coupled solve | focused pass |
| E06 | `energy::{neutral_resistance,leaf_boundary_conductance}` | aerodynamic/energy vectors | calm/nonneutral and domain guards | `energy_input` | focused pass |
| E07 | `photosynthesis::fvcb` | Rubisco/electron/zero/saturated vectors | limitation and compensation branches | both solved leaf classes | focused pass |
| E08 | `photosynthesis::electron_transport` | zero/electron/saturated vectors | zero light and capacity guards | `fvcb` | focused pass |
| E09 | `photosynthesis::smaller_root`; `fvcb` | co-limitation vectors | stable smaller-root implementation and discriminant error | `solve_ci` | focused pass |
| E10 | `photosynthesis::{arrhenius,peaked_response}` | digest-bound biochemical vectors | stable log-domain response; NaN/zero-capacity guards | leaf-temperature residual | focused pass |
| E11 | `photosynthesis::{medlyn,solve_ci}`, `energy::canopy_surface_friction_velocity`, and potential/capped occupancy evaluators | V3 potential vectors plus V5 cap-active vectors | reference-wind misuse, wind-domain, class-beta/equality, inactive-class, cap ordering, and nested failure guards | typed potential request pass; public capped transaction integration pending | focused capped oracle PASS; public integration pending |
| E12 | `photosynthesis::{carbon_surface,solve_ci}` | coupled-leaf `ci/cs` vector | boundary-resistance distinction and Brent guards | energy leaf nodes | focused pass |
| E13 | `energy::solve_canopy_energy` | integrated energy vectors | six-node residual, wet-store cap, dry-stem owner | public physical candidate | focused pass |
| E14 | `hydraulics::{vulnerability,solve_hydraulics}` plus common-root potential/capped evaluators | V3 four-node potential/failure vectors plus V5 law/cap/tie vectors | height/gravity/common-root, dry/frozen layer, redistribution, cap equality, generalized derivative, singular/pivot, and typed-failure guards | typed potential requests; public finalized-use integration pending | focused capped oracle PASS; public integration pending |
| E15 | `occupancy_solver::potential::solve_uncapped_stage_a` and constitutive evaluator; V5 capped evaluator with V6 portability evidence | V3 accepted potential/failure vectors and V5 vectors SHA-256 `6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d` | distinct-beta, class/aggregate equality, exact `F<=A<=D`, equality/near-tie, alternate warm starts, failed-iterate operands, diagnostics, rollback, and 27 capped poisons | potential water requests and internal final column; public transaction integration pending | `STAGE_B_E11_E15_EXACT_ORACLE` PASS; public integration pending |
| E16 | `carbon_nitrogen::gpp_kg_c` | C/N vector | gross/net separation and molar conversion | final physical state | focused pass |
| E17 | `update_t10`; `maintenance_respiration`; `carbon_offer` | V3 respiration plus V4 displayed-leaf-N ownership vectors | class-resolved leaf Rd debited once; storage/transfer leaf-N poison; non-leaf tissue/layer respiration and signed reserve priority | final carbon offer | V4 ownership kernel pass; persistent public transition pending |
| E18 | `nitrogen_demand`; `finalize_growth` | six-tissue allocation vector | N-sufficient/limited common-eta allocation and NSC retention | post-N-authorization state | focused pass |
| E19 | pending V2 post-occupancy aggregation; proportional BGC arbitration | request/finalized bucket vectors | layer/species identity and unused-authorization poisons | public N request/auth/use path | V2 public path pending |
| E20 | `advance_phenology`; V4 displayed-leaf-C/SLA area ownership | V4 shared-state and 155 whole-state mutation vectors | storage/transfer leaf-C area poisons; removed offset-field shape guards; onset/offset and GSI timing posture | persistent stratum state | V4 state/ownership pass; accepted public transition pending |
| E21 | `advance_turnover` | fine-root/livewood/CWD vectors | ordered bounded turnover and receiver identity | material proposals | focused pass |
| E22 | `material_transfer`; BGC `MaterialReceipt` | litter C/N/DM vectors | carbon-as-dry-matter and duplicate-receipt poisons | BGC receiver candidate | focused pass |

`c3_vegetation_implementation_contract` currently proves retained pure-kernel
vectors and that the public consumer fails closed before the capped pass. It does not
claim that the public consumer invokes E01--E22.

All earlier `focused pass` rows describe the historical single-topology
remediation checkpoint. Each row must be revalidated through the V4 public path;
no helper-only row is terminally implemented. V5 authority availability is not
Rust implementation evidence.
