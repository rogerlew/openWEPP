# Equation Module Map

Status: `executing / Milestones 4 and 5 closed / default-off full candidate active`

Evidence mode: `Static + Ran`

The V1 fixture remains historical equation evidence. V2 topology authority is
bound by `openwepp_c3_woody_v2_topology_vectors.json`, SHA-256
`c02e5e2a2287d84cfc584a6e3ec9c499cf7168160bc71f2577323f19dcb50bf1`.
The complete capped physiology gate is named
`STAGE_B_E11_E15_EXACT_ORACLE`. It passes on the final V6 identity. The
subsequent public-water increment proves that the uncommitted public stage
consumes the accepted potential and capped columns. The later Milestone 4/5
increments compose E16--E22 and the all-owner candidate/commit; the historical
water-only limitation is retained here only as sequencing evidence.

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
| E02 | `radiation::{sunlit_absorption,partition_owned_absorption}` | V3 sunlit/owner vectors | stem-only zero-photosynthesis, absorptivity weighting, direct/diffuse and VIS/NIR identity | public occupancy evaluators and independent energy owner | public water phase and focused component-energy reconstruction PASS |
| E03 | `radiation::solve_mixed_column` ordered full-column traversal | V3 two-rank directional fixture | nonzero upward lower boundary, rank/identity swaps, terminal closure | public potential and capped water passes | public water-phase consumer active |
| E04 | `interception::liquid_interception`; `column::execute_tile_columns` | V1 wet/condensation plus controlled V2 topology vectors | conditional-area poison; same-tile throughfall/both-drainage routing; stemflow bypass; local/column/stand closure | sealed public potential and capped water stage | public uncommitted water-phase consumer active |
| E05 | `liquid_interception`; `energy::canopy_residual` | integrated wet-canopy vector | dry/wet leaf/stem identity and active store cap | capped coupled solve | public default-off transaction PASS |
| E06 | `energy::{neutral_resistance,leaf_boundary_conductance}` | aerodynamic/energy vectors | calm/nonneutral and domain guards | `energy_input` | public default-off transaction PASS |
| E07 | `photosynthesis::fvcb` | Rubisco/electron/zero/saturated vectors | limitation and compensation branches | both solved leaf classes | public default-off transaction PASS |
| E08 | `photosynthesis::electron_transport` | zero/electron/saturated vectors | zero light and capacity guards | `fvcb` | public default-off transaction PASS |
| E09 | `photosynthesis::smaller_root`; `fvcb` | co-limitation vectors | stable smaller-root implementation and discriminant error | `solve_ci` | public default-off transaction PASS |
| E10 | `photosynthesis::{arrhenius,peaked_response}` | digest-bound biochemical vectors | stable log-domain response; NaN/zero-capacity guards | leaf-temperature residual | public default-off transaction PASS |
| E11 | `photosynthesis::{medlyn,solve_ci}`, `energy::canopy_surface_friction_velocity`, and potential/capped occupancy evaluators | V3 potential vectors plus V5 cap-active vectors | reference-wind misuse, wind-domain, class-beta/equality, inactive-class, cap ordering, and nested failure guards | public typed D/A/F water stage | focused capped oracle and public water integration PASS |
| E12 | `photosynthesis::{carbon_surface,solve_ci}` | coupled-leaf `ci/cs` vector | boundary-resistance distinction and Brent guards | energy leaf nodes | public default-off transaction PASS |
| E13 | `energy::solve_canopy_energy`; orchestrator `vegetation_energy_owner` | integrated energy vectors plus real capped full-water proposal | six-node residual, wet-store cap, signed wet flux, dry-stem owner, finalized-water and tile-basis poisons | default-off four-owner transaction | independent component/stand reconstruction and atomic connection focused PASS |
| E14 | `hydraulics::{vulnerability,solve_hydraulics}` plus common-root potential/capped evaluators | V3 four-node potential/failure vectors plus V5 law/cap/tie vectors | height/gravity/common-root, dry/frozen layer, redistribution, cap equality, generalized derivative, singular/pivot, and typed-failure guards | public typed requests, fixed caps, finalized uses, and owner debit | focused capped oracle and public water integration PASS |
| E15 | `occupancy_solver::potential::solve_uncapped_stage_a` and constitutive evaluator; V5 capped evaluator with V6 portability evidence | V3 accepted potential/failure vectors and V5 vectors SHA-256 `6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d` | distinct-beta, class/aggregate equality, exact `F<=A<=D`, equality/near-tie, alternate warm starts, failed-iterate operands, complete vulnerability residual operands, diagnostics, rollback, and 27 capped poisons | `water_phase::execute_uncommitted_water_phase` | `STAGE_B_E11_E15_EXACT_ORACLE` and public water integration PASS |
| E16 | `occupancy_solver::constitutive`; `carbon_phase::aggregate_stratum_carbon`; `carbon_nitrogen::gpp_kg_c` | accepted production water-phase plus C/N vector | capped gross/net separation, exact class area, interval and tile weighting | default-off four-owner transaction | accepted Ag aggregated once per stratum and atomically published PASS |
| E17 | accepted class Rd operands; `aggregate_stratum_carbon`; `update_t10`; `maintenance_respiration`; `carbon_offer` | V3 respiration plus V4 displayed-leaf-N ownership vectors | class-resolved leaf Rd debited once; storage/transfer leaf-N poison; non-leaf tissue/layer respiration and signed reserve priority | default-off four-owner transaction | capped Rd/T10 operands and persistent transition atomically published PASS |
| E18 | `nitrogen_demand`; receipt-bound `finalize_growth`; `vegetation_candidate::construct_ending_strata` | six-tissue allocation vector | independently reconstructed final demand, N-sufficient/limited common-eta allocation, exact internal-use debit, NSC retention, ending-area cache identity | sealed vegetation owner candidate | focused uncommitted candidate PASS |
| E19 | `persistent_phase` potential/final preparation; `nitrogen_protocol`; `vegetation_ledger`; BGC `construct_biogeochemistry_candidate` | request/finalized bucket vectors plus real V7 full-water two-ULP composition | layer/species/owner identity, immutable potential requests, one arbitration, `F<=A<=D`, final-above-potential branches, exact vegetation and mineral-N reconstruction | default-off four-owner transaction | request/authorization/final-use, growth, both owner ledgers, and exact cross-owner protocol focused PASS |
| E20 | `prepare_storage_for_onset`; `advance_phenology`; `vegetation_candidate::update_derived_areas` | V7 six-tissue preparation, first/multi-interval onset, terminal remainder, evergreen, migration, and candidate-state identity vectors | exact half of beginning storage; all-six deployment; no same-interval recycling; recomputed LAI/SAI/root caches; canonical ending digest | default-off four-owner transaction | V7 kernel, sealed vegetation candidate, and atomic publication focused PASS |
| E21 | `advance_turnover`; `vegetation_candidate::bind_material_proposals`; BGC receiving-owner construction | fine-root/livewood/CWD vectors plus deterministic proposal and receipt ordering | ordered bounded turnover, donor/receiver identity, positive transaction-scoped proposal IDs, exact independently constructed receipt | default-off four-owner transaction | vegetation/BGC candidates and exact proposal/receipt connection focused PASS |
| E22 | `material_transfer`; `vegetation_ledger::validate_vegetation_ledgers`; BGC `MaterialReceipt` and receiver operands | litter C/N/DM, proposal/receipt, and carbon-as-dry-matter poisons | independent configured-fraction DM reconstruction, proposal aggregate identity, receiving-pool C/N/DM closure | default-off four-owner transaction | both owner candidates and atomic cross-owner receipt validation focused PASS |

The public consumer executes E01--E15 through an explicitly uncommittable water
stage, constructs typed D/A/F and a receiving-owner water debit, then executes
one all-strata E19 arbitration from immutable potential requests.
The historical two-ULP HOLD audit is preserved, while
`e19-potential-final-ordering-disposition.md` records why its authority
conclusion was an implementation overconstraint. Final demand is not clamped;
final use remains bounded by potential authorization and unsupported carbon is
retained in NSC. The default-off diagnostic connects the sealed V7 vegetation,
retained water-owner, BGC, and energy candidates and commits them through one
atomic owner envelope. Milestone 6 and terminal package closure remain pending.

V7 lifts `GAP-VEGETATION-027`. The default-off public diagnostic now composes
E20 with turnover/retranslocation, E19 authorization/allocation, proposals,
independent ledgers, and the atomic multi-owner candidate from immutable
beginning state. V5/V6/V7 authority availability remains distinct from the Rust
implementation evidence named in each row.
