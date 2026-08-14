# Equation Module Map

Status: `executing / V7 sealed vegetation candidate active / full candidate fail-closed`

Evidence mode: `Static + Ran`

The V1 fixture remains historical equation evidence. V2 topology authority is
bound by `openwepp_c3_woody_v2_topology_vectors.json`, SHA-256
`c02e5e2a2287d84cfc584a6e3ec9c499cf7168160bc71f2577323f19dcb50bf1`.
The complete capped physiology gate is named
`STAGE_B_E11_E15_EXACT_ORACLE`. It passes on the final V6 identity. This is
The subsequent public-water increment proves that the uncommitted public stage
consumes the accepted potential and capped columns. It does not prove E16--E22
or the all-owner candidate/commit.

V5 imports V4 shared-state ownership and V3 radiation/uncapped constitutive
authority unchanged. It now binds the previously missing capped-pass amount/
rate conversion, independent hydraulic-law operand, equality-active branch,
generalized Jacobian, diagnostics, rollback, and exact cap-active vectors.
Production implementation and independent Rust reconstruction pass the focused
capped gate. The sealed `UncommittedWaterPhase` now consumes these kernels and
fails closed before shared C/N and the all-owner candidate.

| Equation | Production function | Independent vector | Positive/poison/guard evidence | Whole-transaction consumer | Status |
|---|---|---|---|---|---|
| E01 | `radiation::{mixed_optics,solve_mixed_column}` matrix exponential/particular solution | V3 authority radiation vectors | direction/band closure, mixed-optics, clumping, Beer, and leaf-angle guards | public potential and capped water passes | public water-phase consumer active |
| E02 | `radiation::{sunlit_absorption,partition_owned_absorption}` | V3 sunlit/owner vectors | stem-only zero-photosynthesis, absorptivity weighting, direct/diffuse and VIS/NIR identity | public occupancy evaluators; later energy owner | public water-phase consumer active; energy owner pending |
| E03 | `radiation::solve_mixed_column` ordered full-column traversal | V3 two-rank directional fixture | nonzero upward lower boundary, rank/identity swaps, terminal closure | public potential and capped water passes | public water-phase consumer active |
| E04 | `interception::liquid_interception`; `column::execute_tile_columns` | V1 wet/condensation plus controlled V2 topology vectors | conditional-area poison; same-tile throughfall/both-drainage routing; stemflow bypass; local/column/stand closure | sealed public potential and capped water stage | public uncommitted water-phase consumer active |
| E05 | `liquid_interception`; `energy::canopy_residual` | integrated wet-canopy vector | dry/wet leaf/stem identity and active store cap | capped coupled solve | focused pass |
| E06 | `energy::{neutral_resistance,leaf_boundary_conductance}` | aerodynamic/energy vectors | calm/nonneutral and domain guards | `energy_input` | focused pass |
| E07 | `photosynthesis::fvcb` | Rubisco/electron/zero/saturated vectors | limitation and compensation branches | both solved leaf classes | focused pass |
| E08 | `photosynthesis::electron_transport` | zero/electron/saturated vectors | zero light and capacity guards | `fvcb` | focused pass |
| E09 | `photosynthesis::smaller_root`; `fvcb` | co-limitation vectors | stable smaller-root implementation and discriminant error | `solve_ci` | focused pass |
| E10 | `photosynthesis::{arrhenius,peaked_response}` | digest-bound biochemical vectors | stable log-domain response; NaN/zero-capacity guards | leaf-temperature residual | focused pass |
| E11 | `photosynthesis::{medlyn,solve_ci}`, `energy::canopy_surface_friction_velocity`, and potential/capped occupancy evaluators | V3 potential vectors plus V5 cap-active vectors | reference-wind misuse, wind-domain, class-beta/equality, inactive-class, cap ordering, and nested failure guards | public typed D/A/F water stage | focused capped oracle and public water integration PASS |
| E12 | `photosynthesis::{carbon_surface,solve_ci}` | coupled-leaf `ci/cs` vector | boundary-resistance distinction and Brent guards | energy leaf nodes | focused pass |
| E13 | `energy::solve_canopy_energy` | integrated energy vectors | six-node residual, wet-store cap, dry-stem owner | public physical candidate | focused pass |
| E14 | `hydraulics::{vulnerability,solve_hydraulics}` plus common-root potential/capped evaluators | V3 four-node potential/failure vectors plus V5 law/cap/tie vectors | height/gravity/common-root, dry/frozen layer, redistribution, cap equality, generalized derivative, singular/pivot, and typed-failure guards | public typed requests, fixed caps, finalized uses, and owner debit | focused capped oracle and public water integration PASS |
| E15 | `occupancy_solver::potential::solve_uncapped_stage_a` and constitutive evaluator; V5 capped evaluator with V6 portability evidence | V3 accepted potential/failure vectors and V5 vectors SHA-256 `6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d` | distinct-beta, class/aggregate equality, exact `F<=A<=D`, equality/near-tie, alternate warm starts, failed-iterate operands, complete vulnerability residual operands, diagnostics, rollback, and 27 capped poisons | `water_phase::execute_uncommitted_water_phase` | `STAGE_B_E11_E15_EXACT_ORACLE` and public water integration PASS |
| E16 | `occupancy_solver::constitutive`; `carbon_phase::aggregate_stratum_carbon`; `carbon_nitrogen::gpp_kg_c` | accepted production water-phase plus C/N vector | capped gross/net separation, exact class area, interval and tile weighting | final capped physical state | accepted Ag retained and aggregated once per stratum; persistent transition pending |
| E17 | accepted class Rd operands; `aggregate_stratum_carbon`; `update_t10`; `maintenance_respiration`; `carbon_offer` | V3 respiration plus V4 displayed-leaf-N ownership vectors | class-resolved leaf Rd debited once; storage/transfer leaf-N poison; non-leaf tissue/layer respiration and signed reserve priority | final carbon offer | capped Rd/T10 operands retained; persistent public transition pending |
| E18 | `nitrogen_demand`; receipt-bound `finalize_growth`; `vegetation_candidate::construct_ending_strata` | six-tissue allocation vector | independently reconstructed final demand, N-sufficient/limited common-eta allocation, exact internal-use debit, NSC retention, ending-area cache identity | sealed vegetation owner candidate | focused uncommitted candidate PASS |
| E19 | `persistent_phase` potential/final preparation; `nitrogen_protocol`; `vegetation_ledger` | request/finalized bucket vectors plus real V7 full-water two-ULP composition | layer/species/owner identity, immutable potential requests, one arbitration, `F<=A<=D`, final-above-potential branches, exact vegetation N reconstruction | sealed vegetation owner candidate | request/authorization/final-use, growth, and vegetation ledger PASS; BGC debit pending |
| E20 | `prepare_storage_for_onset`; `advance_phenology`; `vegetation_candidate::update_derived_areas` | V7 six-tissue preparation, first/multi-interval onset, terminal remainder, evergreen, migration, and candidate-state identity vectors | exact half of beginning storage; all-six deployment; no same-interval recycling; recomputed LAI/SAI/root caches; canonical ending digest | sealed persistent stratum state | V7 kernel and sealed vegetation candidate PASS; atomic publication pending |
| E21 | `advance_turnover`; `vegetation_candidate::bind_material_proposals` | fine-root/livewood/CWD vectors plus deterministic proposal ordering | ordered bounded turnover, donor/receiver identity, positive transaction-scoped proposal IDs | sealed material proposal escrow | focused candidate PASS; receiver pending |
| E22 | `material_transfer`; `vegetation_ledger::validate_vegetation_ledgers`; BGC `MaterialReceipt` | litter C/N/DM and carbon-as-dry-matter poisons | independent configured-fraction DM reconstruction, proposal aggregate identity, C/N export closure | vegetation proposal; future BGC receiver candidate | vegetation-side candidate PASS; independent BGC receipt pending |

The public consumer executes E01--E15 through an explicitly uncommittable water
stage, constructs typed D/A/F and a receiving-owner water debit, then executes
one crate-private all-strata E19 arbitration from immutable potential requests.
The historical two-ULP HOLD audit is preserved, while
`e19-potential-final-ordering-disposition.md` records why its authority
conclusion was an implementation overconstraint. Final demand is not clamped;
final use remains bounded by potential authorization and unsupported carbon is
retained in NSC. No BGC owner debit, public `CoupledCandidate`, or all-owner
commit is claimed.

V7 lifts `GAP-VEGETATION-027` and implements the standalone E20 preparation and
deployment kernel. This is not yet a public E20 claim: turnover/retranslocation,
E19 nitrogen authorization/allocation, proposals, independent ledgers, and the
atomic multi-owner candidate remain to be composed from immutable beginning
state.

All earlier `focused pass` rows describe the historical single-topology
remediation checkpoint. Each row must be revalidated through the V4 public path;
no helper-only row is terminally implemented. V5 authority availability is not
Rust implementation evidence.
