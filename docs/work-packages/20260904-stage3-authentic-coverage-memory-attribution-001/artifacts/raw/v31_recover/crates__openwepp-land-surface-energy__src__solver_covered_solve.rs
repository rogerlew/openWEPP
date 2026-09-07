#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CoveredStepNorms {
    pub hydraulic_mm: f64,
    pub beta: f64,
    pub temperature_k: f64,
    pub humidity_kg_kg: f64,
    pub ci_pa: f64,
}

impl CoveredStepNorms {
    fn accepted(self) -> bool {
        self.hydraulic_mm <= 1.0e-7
            && self.beta <= 1.0e-10
            && self.temperature_k <= 1.0e-8
            && self.humidity_kg_kg <= 1.0e-12
    }

    fn diagnostics(self) -> StepNorms {
        StepNorms {
            temperature_k: Some(self.temperature_k),
            humidity_kg_kg: Some(self.humidity_kg_kg),
            ci_pa: Some(self.ci_pa),
            hydraulic_mm: Some(self.hydraulic_mm),
            beta: Some(self.beta),
        }
    }

    fn governed_threshold_exceeded(self) -> bool {
        [
            self.hydraulic_mm,
            self.beta,
            self.temperature_k,
            self.humidity_kg_kg,
        ]
        .iter()
        .all(|step| step.is_finite())
            && (self.hydraulic_mm > 1.0e-7
                || self.beta > 1.0e-10
                || self.temperature_k > 1.0e-8
                || self.humidity_kg_kg > 1.0e-12)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CoveredFullTrialNoUpdateRefusal {
    DomainInvalid,
    GovernedStepThresholdExceeded,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CoveredHalvedTrialProbe {
    DomainInvalid,
    EvaluationIncomplete,
    Complete(CoveredStepNorms),
}

fn covered_complete_residuals_pass(residuals: &[f64]) -> bool {
    !residuals.is_empty()
        && residuals
            .iter()
            .all(|residual| residual.is_finite() && residual.abs() <= 1.0)
}

fn covered_halved_no_update_witness(
    current_residuals: &[f64],
    full_trial_refusal: Option<CoveredFullTrialNoUpdateRefusal>,
    is_first_domain_valid_halved_trial: bool,
    prospective_steps: CoveredStepNorms,
) -> bool {
    covered_complete_residuals_pass(current_residuals)
        && full_trial_refusal.is_some()
        && is_first_domain_valid_halved_trial
        && prospective_steps.accepted()
}

fn covered_first_domain_valid_halved_no_update_witness<F>(
    current_residuals: &[f64],
    full_trial_refusal: Option<CoveredFullTrialNoUpdateRefusal>,
    mut probe: F,
) -> Option<(u32, CoveredStepNorms)>
where
    F: FnMut(u32) -> CoveredHalvedTrialProbe,
{
    if !covered_complete_residuals_pass(current_residuals) || full_trial_refusal.is_none() {
        return None;
    }
    for exponent in 1..=MAX_BACKTRACKING_HALVINGS {
        match probe(exponent) {
            CoveredHalvedTrialProbe::DomainInvalid => {}
            CoveredHalvedTrialProbe::EvaluationIncomplete => return None,
            CoveredHalvedTrialProbe::Complete(steps) => {
                return covered_halved_no_update_witness(
                    current_residuals,
                    full_trial_refusal,
                    true,
                    steps,
                )
                .then_some((exponent, steps));
            }
        }
    }
    None
}

fn empty_step_norms() -> StepNorms {
    StepNorms {
        temperature_k: None,
        humidity_kg_kg: None,
        ci_pa: None,
        hydraulic_mm: None,
        beta: None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredColumnCandidate {
    pub solution: Vec<f64>,
    pub evaluation: CoveredColumnEvaluation,
    pub surface_enthalpy_j_m2_tile: f64,
    pub soil_temperature_k: Vec<f64>,
    pub root_water: Vec<SourceWaterFlux>,
    pub ground_water: GroundWaterFlux,
    pub iterations: u32,
    pub backtracking_count: u32,
    pub step_norms: CoveredStepNorms,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CoveredColumnSolveOutcome {
    Accepted(Box<CoveredColumnCandidate>),
    Rejected(NumericalFailure),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredFiniteDifferenceStencil {
    Centered,
    InwardFromUpperBound,
    InwardFromLowerBound,
}

const COVERED_COMPONENT_DEPENDENCY_GRAPH_VERSION: &str =
    "covered-component-temperature-dependency-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoveredComponentReplaySolveClass {
    Potential,
    FixedFinal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoveredComponentReplayStencil {
    Centered,
    InwardLower,
    InwardUpper,
    RejectedBeforeProbe,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CoveredComponentDependencyReplaySweepAudit {
    pub solve_class: CoveredComponentReplaySolveClass,
    pub solve_ordinal: u64,
    pub map_ordinal: u64,
    pub iteration_ordinal: u32,
    pub sweep_ordinal: u32,
    pub occupancy_count: usize,
    pub soil_count: usize,
    pub stencils: Vec<CoveredComponentReplayStencil>,
    pub logical_probe_count: u32,
    pub identity_anchor_probe_count: u32,
    pub component_replay_probe_count: u32,
    pub complete_probe_count: u32,
    pub completed: bool,
    pub failed: bool,
    pub short_circuited: bool,
    pub graph_version: String,
    pub graph_sha256: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CoveredComponentDependencyReplayAudit {
    pub sweeps: Vec<CoveredComponentDependencyReplaySweepAudit>,
    #[cfg(any(test, feature = "test-support"))]
    pub internal_records: Vec<CoveredComponentReplayInternalRecord>,
}

#[cfg(any(test, feature = "test-support"))]
#[path = "solver_component_dependency_replay_test_support.rs"]
mod component_dependency_replay_test_support;
#[cfg(any(test, feature = "test-support"))]
pub(crate) use component_dependency_replay_test_support::{
    CoveredComponentReplayBacktrackingDecision, exact_component_replay_fixed_owner_projection,
    exact_component_replay_potential_owner_projection, record_component_replay_internal_candidate,
};
#[cfg(any(test, feature = "test-support"))]
pub use component_dependency_replay_test_support::{
    CoveredComponentReplayDenseTrace, CoveredComponentReplayExactProjection,
    CoveredComponentReplayInternalRecord,
};

#[cfg(any(test, feature = "test-support"))]
macro_rules! record_backtracking_attempt {
    ($($argument:expr),+ $(,)?) => {
        component_dependency_replay_test_support::record_component_replay_backtracking_attempt(
            $($argument),+
        )
    };
}

#[cfg(not(any(test, feature = "test-support")))]
macro_rules! record_backtracking_attempt {
    ($($argument:expr),+ $(,)?) => {};
}

std::thread_local! {
    static COVERED_COMPONENT_DEPENDENCY_REPLAY_AUDIT: std::cell::RefCell<Option<CoveredComponentDependencyReplayAudit>> = const { std::cell::RefCell::new(None) };
    static COVERED_COMPONENT_DEPENDENCY_REPLAY_SOLVE_ORDINAL: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

pub fn begin_covered_component_dependency_replay_audit() {
    COVERED_COMPONENT_DEPENDENCY_REPLAY_SOLVE_ORDINAL.with(|ordinal| ordinal.set(0));
    COVERED_COMPONENT_DEPENDENCY_REPLAY_AUDIT.with(|audit| {
        *audit.borrow_mut() = Some(CoveredComponentDependencyReplayAudit::default());
    });
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoveredComponentReplayMapIdentity {
    ordinal: u64,
    exact_transaction_identity: String,
}

pub(crate) fn new_covered_component_replay_map_identity(
    exact_transaction_identity: &str,
) -> CoveredComponentReplayMapIdentity {
    CoveredComponentReplayMapIdentity {
        ordinal: next_covered_component_dependency_replay_map_ordinal(),
        exact_transaction_identity: exact_transaction_identity.to_owned(),
    }
}

#[cfg(test)]
fn new_standalone_covered_component_replay_map_identity() -> CoveredComponentReplayMapIdentity {
    let ordinal = next_covered_component_dependency_replay_map_ordinal();
    CoveredComponentReplayMapIdentity {
        ordinal,
        exact_transaction_identity: format!("standalone-covered-solve-{ordinal}"),
    }
}

fn next_covered_component_dependency_replay_solve_ordinal() -> u64 {
    COVERED_COMPONENT_DEPENDENCY_REPLAY_SOLVE_ORDINAL.with(|ordinal| {
        let current = ordinal.get();
        ordinal.set(current.saturating_add(1));
        current
    })
}

#[must_use]
pub fn take_covered_component_dependency_replay_audit() -> CoveredComponentDependencyReplayAudit {
    COVERED_COMPONENT_DEPENDENCY_REPLAY_AUDIT
        .with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

fn retain_covered_component_dependency_replay_sweep(
    sweep: CoveredComponentDependencyReplaySweepAudit,
) {
    debug_assert_eq!(
        sweep.logical_probe_count,
        sweep.identity_anchor_probe_count
            + sweep.component_replay_probe_count
            + sweep.complete_probe_count
    );
    COVERED_COMPONENT_DEPENDENCY_REPLAY_AUDIT.with(|audit| {
        if let Some(run) = audit.borrow_mut().as_mut() {
            run.sweeps.push(sweep);
        }
    });
}

#[derive(Clone)]
struct CoveredComponentReplayBaseNodes {
    liquid_preparations: Vec<CoveredLiquidPreparation>,
    routed_liquid: Vec<crate::covered_liquid::CoveredOccupancyLiquidLedger>,
    leaf: Vec<CoveredOccupancyLeafReplayBaseNodes>,
}

#[derive(Clone, Copy)]
struct CoveredOccupancyLeafReplayBaseNodes {
    current: [LeafTrialState; 2],
    maximum: [LeafTrialState; 2],
}

#[derive(Clone, Copy)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "exact graph-node action bits"
)]
struct CoveredOccupancyReplayPlan {
    leaf_current: [bool; 2],
    leaf_maximum: [bool; 2],
    vapor: [bool; 2],
    emax: [bool; 2],
    wet: bool,
    hydraulic: bool,
    liquid: bool,
    energy: [bool; 4],
}

struct CoveredComponentTemperatureReplayPlan<'a> {
    base: &'a CoveredColumnEvaluation,
    base_nodes: &'a CoveredComponentReplayBaseNodes,
    execution: &'a CoveredComponentReplayExecutionPlan,
}

#[allow(clippy::too_many_arguments)]
fn evaluate_covered_component_temperature_replay(
    validated: &ValidatedCoveredEvaluationInputs<'_>,
    trial: &[f64],
    frozen: &CoveredFrozenBranches,
    base: &CoveredColumnEvaluation,
    occupancy_index: usize,
    component_index: usize,
    execution: &CoveredComponentReplayExecutionPlan,
    base_nodes: &CoveredComponentReplayBaseNodes,
) -> Result<CoveredColumnEvaluation, LandSurfaceEnergyError> {
    if occupancy_index >= validated.column.occupancies.len()
        || component_index >= 4
        || execution.occupancy_replay.len() != validated.column.occupancies.len()
    {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_component_dependency_replay_integrity",
        ));
    }
    let replay = CoveredComponentTemperatureReplayPlan {
        base,
        base_nodes,
        execution,
    };
    evaluate_covered_column_validated_with_replay(
        validated,
        trial,
        Some(frozen),
        None,
        Some(&replay),
        None,
    )
}

struct CoveredComponentTemperatureDependencyGraph {
    version: &'static str,
    occupancy_count: usize,
    soil_count: usize,
    sha256: String,
    inclusive_closure: BTreeMap<String, BTreeSet<String>>,
}

fn evaluate_covered_occupancy_energy_tail(
    context: &CoveredOccupancyTrialContext<'_>,
    occupancy: &CoveredOccupancyInputs,
    geometry: CoveredOccupancyComponentGeometry,
    gas_branches: [V10LeafGasBranch; 2],
    wet_branch: WaterBranch,
    vapor_flux_kg_m2_s: [f64; 3],
    replay: Option<(&CoveredOccupancyEvaluation, CoveredOccupancyReplayPlan)>,
) -> CoveredOccupancyEnergyTail {
    let column = context.column;
    let rho = column.pressure_pa / (DRY_AIR_GAS_CONSTANT_J_KG_K * context.canopy_air_temperature_k);
    let execute_sensible = replay.map_or([true; 4], |(_, plan)| plan.sensible);
    let execute_energy = replay.map_or([true; 4], |(_, plan)| plan.energy);
    let sensible_to_canopy_air_w_m2 = std::array::from_fn(|index| {
        if !execute_sensible[index] {
            return replay.map_or(0.0, |(base, _)| base.sensible_to_canopy_air_w_m2[index]);
        }
        if geometry.areas[index].to_bits() == 0.0_f64.to_bits() {
            return 0.0;
        }
        let conductance = if index < 2 {
            occupancy.gb_leaf_m_s
        } else if index == 2 {
            occupancy.gb_wet_m_s
        } else {
            occupancy.gb_stem_m_s
        };
        rho * AIR_HEAT_CAPACITY_J_KG_K
            * conductance
            * geometry.areas[index]
            * (geometry.temperatures_k[index] - context.canopy_air_temperature_k)
    });
    let absorbed_shortwave_w_m2 = std::array::from_fn(|index| {
        if !execute_energy[index] {
            return replay.map_or(0.0, |(base, _)| base.absorbed_shortwave_w_m2[index]);
        }
        match index {
            0 => occupancy.sun.absorbed_shortwave_w_m2_tile * (1.0 - geometry.wet_fraction),
            1 => occupancy.shade.absorbed_shortwave_w_m2_tile * (1.0 - geometry.wet_fraction),
            2 => {
                geometry.wet_fraction
                    * (occupancy.sun.absorbed_shortwave_w_m2_tile
                        + occupancy.shade.absorbed_shortwave_w_m2_tile
                        + occupancy.stem_absorbed_shortwave_w_m2_tile)
            }
            _ => (1.0 - geometry.wet_fraction) * occupancy.stem_absorbed_shortwave_w_m2_tile,
        }
    });
    let latent: [f64; 4] = std::array::from_fn(|index| {
        if execute_energy[index] && index < 3 {
            column.latent_heat_j_kg * vapor_flux_kg_m2_s[index]
        } else {
            0.0
        }
    });
    let physical_residuals: [f64; 4] = std::array::from_fn(|index| {
        if execute_energy[index] {
            absorbed_shortwave_w_m2[index] + context.component_longwave_w_m2[index]
                - sensible_to_canopy_air_w_m2[index]
                - latent[index]
        } else {
            replay.map_or(0.0, |(base, _)| base.residuals[6 + index])
        }
    });
    let wet_energy_tolerance = crate::physics::energy_tolerance(
        physical_residuals[2].abs()
            + context.component_longwave_w_m2[2].abs()
            + sensible_to_canopy_air_w_m2[2].abs()
            + latent[2].abs(),
    );
    let v10_nonpositive = gas_branches.iter().any(|branch| {
        matches!(
            branch,
            V10LeafGasBranch::ExactZeroPar | V10LeafGasBranch::RespirationDominated
        )
    });
    let v10_inactive_wet = column.authority.admits_nonpositive_assimilation()
        && v10_nonpositive
        && context.caps.is_none()
        && wet_branch == WaterBranch::AuthorizationActiveOrTie
        && context.liquid.preliminary_store / column.interval_s
            <= crate::physics::water_tolerance(
                context.liquid.preliminary_store / column.interval_s,
            )
        && physical_residuals[2].abs() <= wet_energy_tolerance;
    let residuals = std::array::from_fn(|index| {
        if !execute_energy[index] {
            replay.map_or(0.0, |(base, _)| base.residuals[6 + index])
        } else if geometry.areas[index].to_bits() == 0.0_f64.to_bits()
            || (index == 2 && v10_inactive_wet)
        {
            geometry.temperatures_k[index]
                - inactive_component_temperature_anchor_k(index, context.canopy_air_temperature_k)
        } else {
            physical_residuals[index]
        }
    });
    let tolerances = std::array::from_fn(|index| {
        if replay.is_some_and(|(_, plan)| !plan.tolerance[index]) {
            replay.map_or(0.0, |(base, _)| base.tolerances[6 + index])
        } else if geometry.areas[index].to_bits() == 0.0_f64.to_bits() {
            crate::physics::energy_tolerance(1.0)
        } else {
            crate::physics::energy_tolerance(
                absorbed_shortwave_w_m2[index].abs()
                    + context.component_longwave_w_m2[index].abs()
                    + sensible_to_canopy_air_w_m2[index].abs()
                    + latent[index].abs(),
            )
        }
    });
    let net_longwave_w_m2 = std::array::from_fn(|index| {
        if execute_energy[index] {
            context.component_longwave_w_m2[index]
        } else {
            replay.map_or(0.0, |(base, _)| base.net_longwave_w_m2[index])
        }
    });
    CoveredOccupancyEnergyTail {
        residuals,
        tolerances,
        absorbed_shortwave_w_m2,
        sensible_to_canopy_air_w_m2,
        net_longwave_w_m2,
    }
}

impl CoveredComponentTemperatureDependencyGraph {
    fn expand(occupancy_count: usize, soil_count: usize) -> Self {
        let mut records = Vec::new();
        for occupancy in 0..occupancy_count {
            for component in ["sun", "shade", "wet", "stem"] {
                records.push(format!("node:probe[{occupancy},{component}]"));
                records.push(format!(
                    "edge:probe[{occupancy},{component}]->longwave.layer[{occupancy}]"
                ));
                records.push(format!(
                    "edge:probe[{occupancy},{component}]->occ.sensible[{occupancy},{component}]"
                ));
                records.push(format!(
                    "edge:probe[{occupancy},{component}]->occ.energy[{occupancy},{component}]"
                ));
                records.push(format!(
                    "edge:probe[{occupancy},{component}]->occ.tolerance[{occupancy},{component}]"
                ));
                records.push(format!(
                    "edge:probe[{occupancy},{component}]->occ.output[{occupancy}]"
                ));
            }
            for component in ["sun", "shade"] {
                for family in ["leaf.current", "leaf.maximum", "vapor"] {
                    records.push(format!("node:occ.{family}[{occupancy},{component}]"));
                }
            }
            records.push(format!("node:occ.wet[{occupancy}]"));
            records.push(format!("node:occ.hydraulic[{occupancy}]"));
            records.push(format!("node:occ.liquid[{occupancy}]"));
            records.push(format!("node:occ.route_match[{occupancy}]"));
            for component in ["sun", "shade", "wet", "stem"] {
                for family in ["sensible", "energy", "tolerance"] {
                    records.push(format!("node:occ.{family}[{occupancy},{component}]"));
                }
            }
            for node in ["prepare", "wet", "finalize", "incident", "stemflow"] {
                records.push(format!("node:route.{node}[{occupancy}]"));
            }
            records.push(format!("node:longwave.layer[{occupancy}]"));
            records.push(format!("node:occ.output[{occupancy}]"));
        }
        for soil in 0..soil_count {
            records.push(format!("node:lower.soil[{soil}]"));
        }
        for node in [
            "longwave.column",
            "shared.heat",
            "shared.vapor",
            "shared.tolerance",
            "lower.ground_output",
            "residual.raw",
            "residual.tolerance",
            "residual.normalized",
            "result.ground_release",
            "result.ground_stemflow",
            "result.output",
        ] {
            records.push(format!("node:{node}"));
        }
        for occupancy in 0..occupancy_count {
            records.push(format!(
                "edge:route.incident[{occupancy}]->route.prepare[{occupancy}]"
            ));
            for component in ["sun", "shade"] {
                let probe = format!("probe[{occupancy},{component}]");
                for target in [
                    format!("occ.leaf.current[{occupancy},{component}]"),
                    format!("occ.leaf.maximum[{occupancy},{component}]"),
                    format!("occ.vapor[{occupancy},{component}]"),
                    format!("occ.hydraulic[{occupancy}]"),
                    format!("occ.sensible[{occupancy},{component}]"),
                    format!("occ.energy[{occupancy},{component}]"),
                    format!("occ.tolerance[{occupancy},{component}]"),
                    format!("occ.output[{occupancy}]"),
                ] {
                    records.push(format!("edge:{probe}->{target}"));
                }
            }
            for target in [
                format!("route.wet[{occupancy}]"),
                format!("occ.wet[{occupancy}]"),
                format!("occ.sensible[{occupancy},wet]"),
                format!("occ.energy[{occupancy},wet]"),
                format!("occ.tolerance[{occupancy},wet]"),
                format!("occ.liquid[{occupancy}]"),
                format!("occ.output[{occupancy}]"),
            ] {
                records.push(format!("edge:probe[{occupancy},wet]->{target}"));
            }
            for target in [
                format!("occ.sensible[{occupancy},stem]"),
                format!("occ.energy[{occupancy},stem]"),
                format!("occ.tolerance[{occupancy},stem]"),
                format!("occ.output[{occupancy}]"),
            ] {
                records.push(format!("edge:probe[{occupancy},stem]->{target}"));
            }
            for target in [
                format!("route.wet[{occupancy}]"),
                format!("route.finalize[{occupancy}]"),
                format!("longwave.layer[{occupancy}]"),
                format!("occ.wet[{occupancy}]"),
                format!("occ.hydraulic[{occupancy}]"),
                format!("occ.liquid[{occupancy}]"),
                format!("occ.output[{occupancy}]"),
            ] {
                records.push(format!("edge:route.prepare[{occupancy}]->{target}"));
            }
            records.push(format!(
                "edge:route.wet[{occupancy}]->route.finalize[{occupancy}]"
            ));
            records.push(format!(
                "edge:route.finalize[{occupancy}]->occ.route_match[{occupancy}]"
            ));
            if occupancy + 1 < occupancy_count {
                for prefix in ["incident", "stemflow"] {
                    records.push(format!(
                        "edge:route.finalize[{occupancy}]->route.{prefix}[{}]",
                        occupancy + 1
                    ));
                }
                records.push(format!(
                    "edge:route.stemflow[{occupancy}]->route.stemflow[{}]",
                    occupancy + 1
                ));
            } else {
                records.push(format!(
                    "edge:route.finalize[{occupancy}]->result.ground_release"
                ));
                records.push(format!(
                    "edge:route.finalize[{occupancy}]->result.ground_stemflow"
                ));
            }
            records.push(format!("edge:longwave.layer[{occupancy}]->longwave.column"));
            for component in ["sun", "shade"] {
                records.push(format!("edge:occ.leaf.current[{occupancy},{component}]->occ.leaf.maximum[{occupancy},{component}]"));
                records.push(format!("edge:occ.leaf.current[{occupancy},{component}]->occ.vapor[{occupancy},{component}]"));
                records.push(format!(
                    "edge:occ.leaf.current[{occupancy},{component}]->occ.hydraulic[{occupancy}]"
                ));
                records.push(format!("edge:occ.leaf.maximum[{occupancy},{component}]->occ.vapor[{occupancy},{component}]"));
                records.push(format!(
                    "edge:occ.leaf.maximum[{occupancy},{component}]->occ.hydraulic[{occupancy}]"
                ));
                records.push(format!(
                    "edge:occ.vapor[{occupancy},{component}]->occ.hydraulic[{occupancy}]"
                ));
                records.push(format!(
                    "edge:occ.vapor[{occupancy},{component}]->shared.vapor"
                ));
                for target in ["energy", "tolerance"] {
                    records.push(format!("edge:occ.leaf.current[{occupancy},{component}]->occ.{target}[{occupancy},{component}]"));
                    records.push(format!("edge:occ.vapor[{occupancy},{component}]->occ.{target}[{occupancy},{component}]"));
                }
                records.push(format!(
                    "edge:occ.leaf.current[{occupancy},{component}]->occ.output[{occupancy}]"
                ));
                records.push(format!(
                    "edge:occ.leaf.maximum[{occupancy},{component}]->occ.output[{occupancy}]"
                ));
                records.push(format!(
                    "edge:occ.vapor[{occupancy},{component}]->occ.output[{occupancy}]"
                ));
            }
            for target in [
                format!("occ.hydraulic[{occupancy}]"),
                format!("occ.energy[{occupancy},wet]"),
                format!("occ.tolerance[{occupancy},wet]"),
                format!("occ.liquid[{occupancy}]"),
                format!("occ.output[{occupancy}]"),
                "shared.vapor".to_owned(),
            ] {
                records.push(format!("edge:occ.wet[{occupancy}]->{target}"));
            }
            for component in ["sun", "shade", "wet", "stem"] {
                records.push(format!("edge:occ.sensible[{occupancy},{component}]->occ.energy[{occupancy},{component}]"));
                records.push(format!("edge:occ.sensible[{occupancy},{component}]->occ.tolerance[{occupancy},{component}]"));
                records.push(format!(
                    "edge:occ.sensible[{occupancy},{component}]->occ.output[{occupancy}]"
                ));
                records.push(format!("edge:occ.energy[{occupancy},{component}]->occ.tolerance[{occupancy},{component}]"));
                records.push(format!(
                    "edge:occ.energy[{occupancy},{component}]->occ.output[{occupancy}]"
                ));
                records.push(format!(
                    "edge:occ.tolerance[{occupancy},{component}]->occ.output[{occupancy}]"
                ));
                records.push(format!(
                    "edge:occ.sensible[{occupancy},{component}]->shared.heat"
                ));
            }
            for source in ["hydraulic", "liquid", "route_match"] {
                records.push(format!(
                    "edge:occ.{source}[{occupancy}]->occ.output[{occupancy}]"
                ));
            }
            records.push(format!(
                "edge:occ.liquid[{occupancy}]->occ.route_match[{occupancy}]"
            ));
            for target in ["residual.raw", "residual.tolerance", "shared.heat", "shared.vapor", "result.output"] {
                records.push(format!("edge:occ.output[{occupancy}]->{target}"));
            }
            for component in ["sun", "shade", "wet", "stem"] {
                records.push(format!(
                    "edge:longwave.column->occ.energy[{occupancy},{component}]"
                ));
                records.push(format!(
                    "edge:longwave.column->occ.tolerance[{occupancy},{component}]"
                ));
            }
            records.push(format!("edge:longwave.column->occ.output[{occupancy}]"));
        }
        records.extend([
            "edge:longwave.column->lower.ground_output".to_owned(),
            "edge:longwave.column->result.output".to_owned(),
            "edge:shared.heat->residual.raw".to_owned(),
            "edge:shared.vapor->residual.raw".to_owned(),
            "edge:shared.heat->shared.tolerance".to_owned(),
            "edge:shared.vapor->shared.tolerance".to_owned(),
            "edge:shared.tolerance->residual.tolerance".to_owned(),
            "edge:lower.ground_output->residual.raw".to_owned(),
            "edge:lower.ground_output->residual.tolerance".to_owned(),
            "edge:lower.ground_output->result.output".to_owned(),
            "edge:residual.raw->residual.normalized".to_owned(),
            "edge:residual.tolerance->residual.normalized".to_owned(),
            "edge:residual.normalized->result.output".to_owned(),
            "edge:result.ground_release->result.output".to_owned(),
            "edge:result.ground_stemflow->result.output".to_owned(),
        ]);
        records.sort();
        records.dedup();
        let nodes = records
            .iter()
            .filter_map(|record| record.strip_prefix("node:").map(str::to_owned))
            .collect::<BTreeSet<_>>();
        let mut direct_edges = BTreeMap::<String, BTreeSet<String>>::new();
        for record in &records {
            if let Some(edge) = record.strip_prefix("edge:") {
                if let Some((source, target)) = edge.split_once("->") {
                    direct_edges
                        .entry(source.to_owned())
                        .or_default()
                        .insert(target.to_owned());
                }
            }
        }
        let mut inclusive_closure = BTreeMap::new();
        for source in &nodes {
            let mut closure = BTreeSet::from([source.clone()]);
            let mut frontier = vec![source.clone()];
            while let Some(node) = frontier.pop() {
                if let Some(targets) = direct_edges.get(&node) {
                    for target in targets {
                        if closure.insert(target.clone()) {
                            frontier.push(target.clone());
                        }
                    }
                }
            }
            inclusive_closure.insert(source.clone(), closure);
        }
        let execution_plans = (0..occupancy_count)
            .flat_map(|occupancy_index| {
                let inclusive_closure = &inclusive_closure;
                (0..4).map(move |component_index| {
                    let component = ["sun", "shade", "wet", "stem"][component_index];
                    let closure =
                        &inclusive_closure[&format!("probe[{occupancy_index},{component}]")];
                    CoveredComponentReplayExecutionPlan {
                        occupancy_replay: (0..occupancy_count)
                            .map(|target| CoveredOccupancyReplayPlan {
                                leaf_current: ["sun", "shade"].map(|leaf| {
                                    closure.contains(&format!("occ.leaf.current[{target},{leaf}]"))
                                }),
                                    leaf_maximum: ["sun", "shade"].map(|leaf| {
                                    closure.contains(&format!("occ.leaf.maximum[{target},{leaf}]"))
                                }),
                                vapor: ["sun", "shade"].map(|leaf| {
                                    closure.contains(&format!("occ.vapor[{target},{leaf}]"))
                                }),
                                emax: ["sun", "shade"].map(|leaf| {
                                    closure.contains(&format!("occ.leaf.maximum[{target},{leaf}]"))
                                        || closure.contains(&format!("occ.hydraulic[{target}]"))
                                }),
                                wet: closure.contains(&format!("occ.wet[{target}]")),
                                hydraulic: closure.contains(&format!("occ.hydraulic[{target}]")),
                                liquid: closure.contains(&format!("occ.liquid[{target}]")),
                                route_match: closure
                                    .contains(&format!("occ.route_match[{target}]")),
                                sensible: ["sun", "shade", "wet", "stem"].map(|component| {
                                    closure.contains(&format!("occ.sensible[{target},{component}]"))
                                }),
                                energy: ["sun", "shade", "wet", "stem"].map(|component| {
                                    closure.contains(&format!("occ.energy[{target},{component}]"))
                                }),
                                tolerance: ["sun", "shade", "wet", "stem"].map(|component| {
                                    closure
                                        .contains(&format!("occ.tolerance[{target},{component}]"))
                                }),
                            })
                            .collect(),
                        route_prepare: (0..occupancy_count)
                            .map(|target| closure.contains(&format!("route.prepare[{target}]")))
                            .collect(),
                        route_wet: (0..occupancy_count)
                            .map(|target| closure.contains(&format!("route.wet[{target}]")))
                            .collect(),
                        route_finalize: (0..occupancy_count)
                            .map(|target| closure.contains(&format!("route.finalize[{target}]")))
                            .collect(),
                        longwave_layers: (0..occupancy_count)
                            .map(|target| closure.contains(&format!("longwave.layer[{target}]")))
                            .collect(),
                        longwave_column: closure.contains("longwave.column"),
                        lower_soil: (0..soil_count)
                            .map(|soil| closure.contains(&format!("lower.soil[{soil}]")))
                            .collect(),
                        result_ground_release: closure.contains("result.ground_release"),
                        result_ground_stemflow: closure.contains("result.ground_stemflow"),
                    }
                })
            })
            .collect();
        let mut digest = Sha256::new();
        for value in [
            COVERED_COMPONENT_DEPENDENCY_GRAPH_VERSION.as_bytes(),
            &occupancy_count.to_le_bytes(),
            &soil_count.to_le_bytes(),
        ] {
            digest.update(value.len().to_le_bytes());
            digest.update(value);
        }
        for record in records {
            digest.update(record.len().to_le_bytes());
            digest.update(record.as_bytes());
        }
        Self {
            version: COVERED_COMPONENT_DEPENDENCY_GRAPH_VERSION,
            occupancy_count,
            soil_count,
            sha256: format!("{:x}", digest.finalize()),
            inclusive_closure,
        }
    }

    fn validates_topology(&self, occupancy_count: usize, soil_count: usize) -> bool {
        self.version == COVERED_COMPONENT_DEPENDENCY_GRAPH_VERSION
            && self.occupancy_count == occupancy_count
            && self.soil_count == soil_count
            && self.sha256.len() == 64
    }

    fn proves_component_probe(&self, occupancy_index: usize, component_index: usize) -> bool {
        let component = ["sun", "shade", "wet", "stem"].get(component_index);
        component.is_some_and(|component| {
            self.inclusive_closure
                .get(&format!("probe[{occupancy_index},{component}]"))
                .is_some_and(|closure| closure.contains("result.output"))
        })
    }

    fn complete_occupancy_replay_mask(
        &self,
        occupancy_index: usize,
        component_index: usize,
    ) -> Option<Vec<bool>> {
        let component = ["sun", "shade", "wet", "stem"].get(component_index)?;
        let closure = self
            .inclusive_closure
            .get(&format!("probe[{occupancy_index},{component}]"))?;
        Some(
            (0..self.occupancy_count)
                .map(|target| {
                    closure.contains(&format!("occ.hydraulic[{target}]"))
                        || closure.contains(&format!("occ.wet[{target}]"))
                        || closure.contains(&format!("occ.liquid[{target}]"))
                })
                .collect(),
        )
    }
}

fn covered_finite_difference_stencil(
    current: &[f64],
    minus: &[f64],
    plus: &[f64],
    occupancy_count: usize,
    ground_uses_liquid_vapor_phase_domain: bool,
) -> Result<CoveredFiniteDifferenceStencil, LandSurfaceEnergyError> {
    if !covered_trial_is_valid(
        current,
        occupancy_count,
        ground_uses_liquid_vapor_phase_domain,
    ) {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_jacobian_bound",
        ));
    }
    match (
        covered_trial_is_valid(
            minus,
            occupancy_count,
            ground_uses_liquid_vapor_phase_domain,
        ),
        covered_trial_is_valid(plus, occupancy_count, ground_uses_liquid_vapor_phase_domain),
    ) {
        (true, true) => Ok(CoveredFiniteDifferenceStencil::Centered),
        (true, false) => Ok(CoveredFiniteDifferenceStencil::InwardFromUpperBound),
        (false, true) => Ok(CoveredFiniteDifferenceStencil::InwardFromLowerBound),
        (false, false) => Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_jacobian_bound",
        )),
    }
}

fn covered_finite_difference_value(
    stencil: CoveredFiniteDifferenceStencil,
    current: f64,
    minus: Option<f64>,
    plus: Option<f64>,
    perturbation: f64,
) -> Result<f64, LandSurfaceEnergyError> {
    let missing_probe = || LandSurfaceEnergyError::ConstitutiveDomain("covered_jacobian_bound");
    match stencil {
        CoveredFiniteDifferenceStencil::Centered => Ok((plus.ok_or_else(missing_probe)?
            - minus.ok_or_else(missing_probe)?)
            / (2.0 * perturbation)),
        CoveredFiniteDifferenceStencil::InwardFromUpperBound => {
            Ok((current - minus.ok_or_else(missing_probe)?) / perturbation)
        }
        CoveredFiniteDifferenceStencil::InwardFromLowerBound => {
            Ok((plus.ok_or_else(missing_probe)? - current) / perturbation)
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
std::thread_local! {
    static FORCE_COMPLETE_COVERED_JACOBIAN_PROBES: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
std::thread_local! {
    static COVERED_JACOBIAN_FULL_PROBE_AUDIT: std::cell::Cell<Option<u32>> = const { std::cell::Cell::new(None) };
    static COVERED_DENSE_SWEEP_TRACE: std::cell::RefCell<Option<Vec<CoveredDenseSweepTrace>>> = const { std::cell::RefCell::new(None) };
}

#[cfg(any(test, feature = "test-support"))]
pub struct ForcedCompleteCoveredComponentProbeGuard {
    prior: bool,
}

#[cfg(any(test, feature = "test-support"))]
impl Drop for ForcedCompleteCoveredComponentProbeGuard {
    fn drop(&mut self) {
        FORCE_COMPLETE_COVERED_JACOBIAN_PROBES.with(|force| force.set(self.prior));
    }
}

#[cfg(any(test, feature = "test-support"))]
#[must_use]
pub fn force_complete_covered_component_probes_for_scope()
-> ForcedCompleteCoveredComponentProbeGuard {
    let prior = FORCE_COMPLETE_COVERED_JACOBIAN_PROBES.with(|force| {
        let prior = force.get();
        force.set(true);
        prior
    });
    ForcedCompleteCoveredComponentProbeGuard { prior }
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct CoveredDenseSweepTrace {
    iteration: u32,
    current_bits: Vec<u64>,
    jacobian_bits: Vec<Vec<u64>>,
    rhs_bits: Vec<u64>,
    delta_bits: Vec<u64>,
    pivot_bits: u64,
    matrix_norm_bits: u64,
}

#[cfg(test)]
fn begin_covered_dense_sweep_trace() {
    COVERED_DENSE_SWEEP_TRACE.with(|trace| *trace.borrow_mut() = Some(Vec::new()));
}

#[cfg(test)]
fn take_covered_dense_sweep_trace() -> Vec<CoveredDenseSweepTrace> {
    COVERED_DENSE_SWEEP_TRACE.with(|trace| trace.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
fn record_covered_dense_sweep_trace(
    iteration: u32,
    current: &[f64],
    jacobian: &[Vec<f64>],
    rhs: &[f64],
    delta: &[f64],
    pivot: f64,
    matrix_norm: f64,
) {
    COVERED_DENSE_SWEEP_TRACE.with(|trace| {
        let mut trace = trace.borrow_mut();
        if let Some(trace) = trace.as_mut() {
            trace.push(CoveredDenseSweepTrace {
                iteration,
                current_bits: current.iter().map(|value| value.to_bits()).collect(),
                jacobian_bits: jacobian
                    .iter()
                    .map(|row| row.iter().map(|value| value.to_bits()).collect())
                    .collect(),
                rhs_bits: rhs.iter().map(|value| value.to_bits()).collect(),
                delta_bits: delta.iter().map(|value| value.to_bits()).collect(),
                pivot_bits: pivot.to_bits(),
                matrix_norm_bits: matrix_norm.to_bits(),
            });
        }
    });
}

#[cfg(not(test))]
fn record_covered_dense_sweep_trace(
    _: u32,
    _: &[f64],
    _: &[Vec<f64>],
    _: &[f64],
    _: &[f64],
    _: f64,
    _: f64,
) {
}

#[cfg(test)]
fn begin_covered_jacobian_full_probe_audit() {
    FORCE_COMPLETE_COVERED_JACOBIAN_PROBES.with(|force| force.set(false));
    COVERED_JACOBIAN_FULL_PROBE_AUDIT.with(|audit| audit.set(Some(0)));
}

#[cfg(test)]
fn begin_forced_complete_covered_jacobian_probe_audit() {
    FORCE_COMPLETE_COVERED_JACOBIAN_PROBES.with(|force| force.set(true));
    COVERED_JACOBIAN_FULL_PROBE_AUDIT.with(|audit| audit.set(Some(0)));
}

#[cfg(test)]
fn take_covered_jacobian_full_probe_audit() -> u32 {
    FORCE_COMPLETE_COVERED_JACOBIAN_PROBES.with(|force| force.set(false));
    COVERED_JACOBIAN_FULL_PROBE_AUDIT.with(|audit| audit.take().unwrap_or_default())
}

#[cfg(test)]
fn force_complete_covered_jacobian_probes() -> bool {
    FORCE_COMPLETE_COVERED_JACOBIAN_PROBES.with(std::cell::Cell::get)
}

#[cfg(not(test))]
const fn force_complete_covered_jacobian_probes() -> bool {
    false
}

struct ValidatedCoveredIterationMap<'a> {
    validated: &'a ValidatedCoveredEvaluationInputs<'a>,
    evaluation: CoveredColumnEvaluation,
}

impl<'a> ValidatedCoveredIterationMap<'a> {
    fn evaluate(
        validated: &'a ValidatedCoveredEvaluationInputs<'a>,
        trial: &[f64],
    ) -> Result<Self, LandSurfaceEnergyError> {
        Ok(Self {
            validated,
            evaluation: evaluate_covered_column_validated(validated, trial, None, None)?,
        })
    }

    fn into_jacobian_base(self, trial: &[f64]) -> ValidatedCoveredJacobianBase<'a> {
        let frozen = freeze_covered_branches(&self.evaluation);
        ValidatedCoveredJacobianBase {
            validated: self.validated,
            trial: trial.to_vec(),
            evaluation: self.evaluation,
            frozen,
        }
    }
}

struct ValidatedCoveredJacobianBase<'a> {
    validated: &'a ValidatedCoveredEvaluationInputs<'a>,
    trial: Vec<f64>,
    evaluation: CoveredColumnEvaluation,
    frozen: CoveredFrozenBranches,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredSignedProbe {
    Minus,
    Plus,
}

struct ValidatedCoveredComponentReplaySweepBase<'a> {
    jacobian: ValidatedCoveredJacobianBase<'a>,
    graph: CoveredComponentTemperatureDependencyGraph,
    validated_input_address: usize,
    solve_class: CoveredComponentReplaySolveClass,
    solve_ordinal: u64,
    iteration_ordinal: u32,
    sweep_ordinal: u32,
}

impl<'a> ValidatedCoveredComponentReplaySweepBase<'a> {
    fn new(
        jacobian: ValidatedCoveredJacobianBase<'a>,
        solve_class: CoveredComponentReplaySolveClass,
        solve_ordinal: u64,
        iteration_ordinal: u32,
        sweep_ordinal: u32,
    ) -> Result<Self, LandSurfaceEnergyError> {
        let occupancy_count = jacobian.validated.column.occupancies.len();
        let soil_count = jacobian.validated.column.ground.soil_nodes.len();
        let graph = CoveredComponentTemperatureDependencyGraph::expand(occupancy_count, soil_count);
        let expected_solve_class = if jacobian.validated.caps.is_some() {
            CoveredComponentReplaySolveClass::FixedFinal
        } else {
            CoveredComponentReplaySolveClass::Potential
        };
        if solve_class != expected_solve_class
            || !graph.validates_topology(occupancy_count, soil_count)
            || jacobian.trial.len() != jacobian.evaluation.normalized_residuals.len()
            || jacobian.frozen != freeze_covered_branches(&jacobian.evaluation)
        {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "covered_component_dependency_replay_integrity",
            ));
        }
        Ok(Self {
            graph,
            graph_sha256,
            jacobian,
            solve_class,
            solve_ordinal,
            iteration_ordinal,
            sweep_ordinal,
        }
    }

    fn mint_probe(
        &self,
        probe: &[f64],
        column_index: usize,
        sign: CoveredSignedProbe,
        perturbation: f64,
        stencil: CoveredFiniteDifferenceStencil,
    ) -> Result<ValidatedCoveredComponentProbeReplay<'_>, LandSurfaceEnergyError> {
        let occupancy_count = self.jacobian.validated.column.occupancies.len();
        let component_index = column_index % 10;
        let mut expected_minus = self.jacobian.trial.clone();
        let mut expected_plus = self.jacobian.trial.clone();
        if column_index < self.jacobian.trial.len() && perturbation.is_finite() {
            expected_minus[column_index] -= perturbation;
            expected_plus[column_index] += perturbation;
        }
        let expected_stencil = covered_finite_difference_stencil(
            &self.jacobian.trial,
            &expected_minus,
            &expected_plus,
            occupancy_count,
            covered_ground_uses_liquid_vapor_phase_domain(self.jacobian.validated.column),
        )?;
        let minus = covered_trial_is_valid(
            &expected_minus,
            occupancy_count,
            covered_ground_uses_liquid_vapor_phase_domain(self.jacobian.validated.column),
        )
        .then_some(expected_minus);
        let plus = covered_trial_is_valid(
            &expected_plus,
            occupancy_count,
            covered_ground_uses_liquid_vapor_phase_domain(self.jacobian.validated.column),
        )
        .then_some(expected_plus);
        Ok(ValidatedCoveredSignedProbePair {
            base: self,
            column_index,
            perturbation_bits: perturbation.to_bits(),
            stencil,
            minus,
            plus,
        })
    }

    fn component_replay_is_eligible(&self, column_index: usize) -> bool {
        covered_component_replay_is_eligible_for_graph(&self.jacobian, &self.graph, column_index)
    }
}

fn covered_component_replay_is_eligible_for_graph(
    jacobian: &ValidatedCoveredJacobianBase<'_>,
    graph: &CoveredComponentTemperatureDependencyGraph,
    column_index: usize,
) -> bool {
    let occupancy_count = jacobian.validated.column.occupancies.len();
    let component_index = column_index % 10;
    jacobian.validated.column.authority == CoveredColumnAuthority::V11SnowCovered
        && jacobian.validated.stage3_boundary.is_some()
        && column_index < 10 * occupancy_count
        && (6..10).contains(&component_index)
        && graph.validates_topology(
            occupancy_count,
            jacobian.validated.column.ground.soil_nodes.len(),
        )
        && graph.proves_component_probe(column_index / 10, component_index - 6)
}

/// Canonically constructs the only two possible signed probes from the exact
/// immutable sweep trial.  Taking one sign moves its vector out; taking that
/// sign again or requesting a sign excluded by the selected inward stencil is
/// an integrity error.
struct ValidatedCoveredSignedProbePair<'a> {
    base: &'a ValidatedCoveredComponentReplaySweepBase<'a>,
    column_index: usize,
    perturbation_bits: u64,
    stencil: CoveredFiniteDifferenceStencil,
    minus: Option<Vec<f64>>,
    plus: Option<Vec<f64>>,
}

impl<'a> ValidatedCoveredSignedProbePair<'a> {
    fn take(
        &mut self,
        sign: CoveredSignedProbe,
    ) -> Result<Option<ValidatedCoveredSignedProbe<'a>>, LandSurfaceEnergyError> {
        let admitted = matches!(
            (self.stencil, sign),
            (CoveredFiniteDifferenceStencil::Centered, _)
                | (
                    CoveredFiniteDifferenceStencil::InwardFromUpperBound,
                    CoveredSignedProbe::Minus
                )
                | (
                    CoveredFiniteDifferenceStencil::InwardFromLowerBound,
                    CoveredSignedProbe::Plus
                )
        );
        if !admitted {
            return Ok(None);
        }
        let probe = match sign {
            CoveredSignedProbe::Minus => self.minus.take(),
            CoveredSignedProbe::Plus => self.plus.take(),
        }
        .ok_or(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_component_dependency_replay_integrity",
        ))?;
        Ok(Some(ValidatedCoveredSignedProbe {
            base: self.base,
            probe,
            column_index: self.column_index,
            sign,
            perturbation_bits: self.perturbation_bits,
            stencil: self.stencil,
        }))
    }

    #[cfg(test)]
    fn take_required(
        &mut self,
        sign: CoveredSignedProbe,
    ) -> Result<ValidatedCoveredSignedProbe<'a>, LandSurfaceEnergyError> {
        self.take(sign)?
            .ok_or(LandSurfaceEnergyError::ConstitutiveDomain(
                "covered_component_dependency_replay_integrity",
            ))
    }
}

struct ValidatedCoveredSignedProbe<'a> {
    base: &'a ValidatedCoveredComponentReplaySweepBase<'a>,
    probe: Vec<f64>,
    column_index: usize,
    sign: CoveredSignedProbe,
    perturbation_bits: u64,
    stencil: CoveredFiniteDifferenceStencil,
    graph_sha256: &'a str,
    map_ordinal: u64,
    solve_ordinal: u64,
    iteration_ordinal: u32,
    sweep_ordinal: u32,
}

struct ValidatedCoveredComponentProbeReplay<'a> {
    base: &'a ValidatedCoveredComponentReplaySweepBase<'a>,
    probe: Vec<f64>,
    column_index: usize,
    sign: CoveredSignedProbe,
    perturbation_bits: u64,
    stencil: CoveredFiniteDifferenceStencil,
    graph_sha256: &'a str,
    map_ordinal: u64,
    solve_ordinal: u64,
    iteration_ordinal: u32,
    sweep_ordinal: u32,
}

/// Compiler-only witness that the actual private replay capability is lifetime
/// tied and consumed once. This wrapper is absent from every library build.
///
/// ```compile_fail,E0382
/// use openwepp_land_surface_energy::CoveredReplayCapabilityCompileFail;
/// fn consume_twice(value: CoveredReplayCapabilityCompileFail<'_>) {
///     value.consume();
///     value.consume();
/// }
/// ```
///
/// ```compile_fail
/// use openwepp_land_surface_energy::CoveredReplayCapabilityCompileFail;
/// fn escape<'a>(value: CoveredReplayCapabilityCompileFail<'a>)
///     -> CoveredReplayCapabilityCompileFail<'static> { value }
/// ```
#[cfg(doctest)]
pub struct CoveredReplayCapabilityCompileFail<'a>(ValidatedCoveredComponentProbeReplay<'a>);

#[cfg(doctest)]
impl CoveredReplayCapabilityCompileFail<'_> {
    pub fn consume(self) {}
}

impl<'a> ValidatedCoveredSignedProbe<'a> {
    fn into_component_replay(
        self,
    ) -> Result<ValidatedCoveredComponentProbeReplay<'a>, LandSurfaceEnergyError> {
        let binding = self
            .component_binding
            .ok_or(LandSurfaceEnergyError::ConstitutiveDomain(
                "covered_component_dependency_replay_integrity",
            ))?;
        Ok(ValidatedCoveredComponentProbeReplay {
            base: self.base,
            probe: self.probe,
            binding,
        })
    }
}

impl ValidatedCoveredComponentProbeReplay<'_> {
    fn consume(self) -> Result<Vec<f64>, LandSurfaceEnergyError> {
        let local = self.column_index % 10;
        let occupancy_index = self.column_index / 10;
        let perturbation = f64::from_bits(self.perturbation_bits);
        let expected = match self.sign {
            CoveredSignedProbe::Minus => {
                self.base.jacobian.trial[self.column_index] - perturbation
            }
            CoveredSignedProbe::Plus => {
                self.base.jacobian.trial[self.column_index] + perturbation
            }
        };
        if !(6..10).contains(&local)
            || self.probe[self.column_index].to_bits() != expected.to_bits()
            || !matches!(
                self.stencil,
                CoveredFiniteDifferenceStencil::Centered
                    | CoveredFiniteDifferenceStencil::InwardFromUpperBound
                    | CoveredFiniteDifferenceStencil::InwardFromLowerBound
            )
        {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "covered_component_dependency_replay_integrity",
            ));
        }
        Ok(evaluate_covered_component_temperature_replay(
            self.base.jacobian.validated,
            &self.probe,
            &self.base.jacobian.frozen,
            &self.base.jacobian.evaluation,
            occupancy_index,
            local - 6,
            &self
                .base
                .graph
                .complete_occupancy_replay_mask(occupancy_index, local - 6)
                .ok_or(LandSurfaceEnergyError::ConstitutiveDomain(
                    "covered_component_dependency_replay_integrity",
                ))?,
        )?
        .normalized_residuals)
    }
}

#[cfg(test)]
impl<'a> ValidatedCoveredJacobianBase<'a> {
    fn evaluate(
        validated: &'a ValidatedCoveredEvaluationInputs<'a>,
        trial: &[f64],
    ) -> Result<Self, LandSurfaceEnergyError> {
        Ok(ValidatedCoveredIterationMap::evaluate(validated, trial)?.into_jacobian_base(trial))
    }
}

#[cfg(test)]
fn record_covered_jacobian_full_probe_audit() {
    COVERED_JACOBIAN_FULL_PROBE_AUDIT.with(|audit| {
        if let Some(count) = audit.get() {
            audit.set(Some(count.saturating_add(1)));
        }
    });
}

#[cfg(not(test))]
fn record_covered_jacobian_full_probe_audit() {}

/// Returns the ordered normalized residuals for one admitted Jacobian probe.
///
/// Under represented snow, ground and soil temperatures retain their exact
/// identity-anchor synthesis. Component-temperature probes additionally use
/// the graph-bound, single-sweep dependency replay below. Every other probe
/// executes the complete evaluator.
fn covered_component_temperature_probe_residuals(
    base: &ValidatedCoveredComponentReplaySweepBase<'_>,
    probe: &[f64],
    column_index: usize,
    sign: CoveredSignedProbe,
    perturbation: f64,
    stencil: CoveredFiniteDifferenceStencil,
    audit: &mut CoveredComponentDependencyReplaySweepAudit,
) -> Result<Vec<f64>, LandSurfaceEnergyError> {
    let jacobian = &base.jacobian;
    let validated = jacobian.validated;
    let current = &jacobian.evaluation;
    let frozen = Some(&jacobian.frozen);
    audit.logical_probe_count = audit.logical_probe_count.saturating_add(1);
    if probe.len() != jacobian.trial.len() || column_index >= probe.len() {
        audit.complete_probe_count = audit.complete_probe_count.saturating_add(1);
        record_covered_jacobian_full_probe_audit();
        return Ok(
            evaluate_covered_column_validated(validated, probe, frozen, None)?.normalized_residuals,
        );
    }
    if probe.iter().enumerate().any(|(index, value)| {
        index != column_index && value.to_bits() != jacobian.trial[index].to_bits()
    }) {
        audit.complete_probe_count = audit.complete_probe_count.saturating_add(1);
        record_covered_jacobian_full_probe_audit();
        return Ok(
            evaluate_covered_column_validated(validated, probe, frozen, None)?.normalized_residuals,
        );
    }
    if !force_complete_covered_jacobian_probes() {
        if let Some(anchor_k) = validated.stage3_identity_anchor_k(column_index) {
            audit.identity_anchor_probe_count = audit.identity_anchor_probe_count.saturating_add(1);
            let mut residuals = current.normalized_residuals.clone();
            let raw_residual = probe[column_index] - anchor_k;
            residuals[column_index] = raw_residual / STAGE3_COVERED_IDENTITY_TOLERANCE_K;
            return Ok(residuals);
        }
        let local = column_index % 10;
        if validated.column.authority == CoveredColumnAuthority::V11SnowCovered
            && validated.stage3_boundary.is_some()
            && column_index < 10 * validated.column.occupancies.len()
            && (6..10).contains(&local)
            && base.graph.validates_topology(
                validated.column.occupancies.len(),
                validated.column.ground.soil_nodes.len(),
            )
            && base.graph.sha256 == base.graph_sha256
            && base
                .graph
                .proves_component_probe(column_index / 10, local - 6)
        {
            let capability = base.mint_probe(
                probe,
                column_index,
                sign,
                perturbation,
                stencil,
            )?;
            audit.component_replay_probe_count =
                audit.component_replay_probe_count.saturating_add(1);
            return capability.consume();
        }
    }
    audit.complete_probe_count = audit.complete_probe_count.saturating_add(1);
    record_covered_jacobian_full_probe_audit();
    Ok(
        evaluate_covered_column_validated(validated, &signed_probe.probe, frozen, None)?
            .normalized_residuals,
    )
}

#[cfg(test)]
fn covered_jacobian_probe_residuals(
    base: &ValidatedCoveredJacobianBase<'_>,
    probe: &[f64],
    column_index: usize,
) -> Result<Vec<f64>, LandSurfaceEnergyError> {
    let perturbation = probe
        .get(column_index)
        .zip(base.trial.get(column_index))
        .map_or(0.0, |(probe, current)| (probe - current).abs());
    let sign = probe
        .get(column_index)
        .zip(base.trial.get(column_index))
        .map_or(CoveredSignedProbe::Plus, |(probe, current)| {
            if probe < current {
                CoveredSignedProbe::Minus
            } else {
                CoveredSignedProbe::Plus
            }
        });
    let owned = ValidatedCoveredJacobianBase {
        validated: base.validated,
        trial: base.trial.clone(),
        evaluation: base.evaluation.clone(),
        frozen: base.frozen.clone(),
    };
    let replay_base = ValidatedCoveredComponentReplaySweepBase::new(
        owned,
        if base.validated.caps.is_some() {
            CoveredComponentReplaySolveClass::FixedFinal
        } else {
            CoveredComponentReplaySolveClass::Potential
        },
        0,
        0,
        0,
        0,
    )?;
    let mut pair = replay_base.mint_probe_pair(column_index, perturbation)?;
    let canonical = pair
        .take(sign)?
        .ok_or(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_component_dependency_replay_integrity",
        ))?;
    if canonical.probe.len() != probe.len()
        || canonical
            .probe
            .iter()
            .zip(probe)
            .any(|(canonical, supplied)| canonical.to_bits() != supplied.to_bits())
    {
        record_covered_jacobian_full_probe_audit();
        return Ok(evaluate_covered_column_validated(
            base.validated,
            probe,
            Some(&base.frozen),
            None,
        )?
        .normalized_residuals);
    }
    let mut audit = new_covered_component_dependency_replay_sweep_audit(&replay_base);
    covered_component_temperature_probe_residuals(canonical, &mut audit)
}

fn new_covered_component_dependency_replay_sweep_audit(
    base: &ValidatedCoveredComponentReplaySweepBase<'_>,
) -> CoveredComponentDependencyReplaySweepAudit {
    CoveredComponentDependencyReplaySweepAudit {
        solve_class: base.solve_class,
        solve_ordinal: base.solve_ordinal,
        map_ordinal: base.solve_ordinal,
        iteration_ordinal: base.iteration_ordinal,
        sweep_ordinal: base.sweep_ordinal,
        occupancy_count: base.graph.occupancy_count,
        soil_count: base.graph.soil_count,
        stencils: Vec::with_capacity(base.jacobian.trial.len()),
        logical_probe_count: 0,
        identity_anchor_probe_count: 0,
        component_replay_probe_count: 0,
        complete_probe_count: 0,
        completed: false,
        failed: false,
        short_circuited: false,
        graph_version: base.graph.version.to_owned(),
        graph_sha256: base.graph.sha256.clone(),
    }
}

struct CoveredComponentReplaySweepGuard {
    audit: CoveredComponentDependencyReplaySweepAudit,
    sealed: bool,
}

impl CoveredComponentReplaySweepGuard {
    fn new(audit: CoveredComponentDependencyReplaySweepAudit) -> Self {
        Self {
            audit,
            sealed: false,
        }
    }

    fn seal_success(mut self) {
        self.audit.completed = true;
        retain_covered_component_dependency_replay_sweep(self.audit.clone());
        self.sealed = true;
    }
}

impl std::ops::Deref for CoveredComponentReplaySweepGuard {
    type Target = CoveredComponentDependencyReplaySweepAudit;

    fn deref(&self) -> &Self::Target {
        &self.audit
    }
}

impl std::ops::DerefMut for CoveredComponentReplaySweepGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.audit
    }
}

impl Drop for CoveredComponentReplaySweepGuard {
    fn drop(&mut self) {
        if !self.sealed {
            self.audit.failed = true;
            retain_covered_component_dependency_replay_sweep(self.audit.clone());
        }
    }
}

pub(crate) fn covered_failure_residuals(
    beginning: &CoveredColumnInputs,
    detail: &CoveredColumnEvaluation,
) -> Vec<NormalizedResidual> {
    const OCCUPANCY_IDENTITIES: [&str; 10] = [
        "sun_gas_minus_q1",
        "shade_gas_minus_q1",
        "sun_gas_minus_vulnerability_demand",
        "shade_gas_minus_vulnerability_demand",
        "q1_sum_minus_q2",
        "q2_minus_root_source_sum",
        "sun_leaf_energy",
        "shade_leaf_energy",
        "wet_surface_energy",
        "dry_stem_energy",
    ];
    let occupancy_count = beginning.occupancies.len();
    detail
        .raw_residuals
        .iter()
        .zip(&detail.tolerances)
        .zip(&detail.normalized_residuals)
        .enumerate()
        .map(|(index, ((raw, tolerance), normalized))| {
            let (identity, unit) = if index < 10 * occupancy_count {
                let occupancy = index / 10;
                let local = index % 10;
                let prefix = if occupancy_count == 1 {
                    String::new()
                } else {
                    format!("{}:", beginning.occupancies[occupancy].occupancy_id)
                };
                (
                    format!("{prefix}{}", OCCUPANCY_IDENTITIES[local]),
                    if local < 6 {
                        ResidualUnit::KilogramsPerSquareMeterSecond
                    } else {
                        ResidualUnit::WattsPerSquareMeter
                    },
                )
            } else {
                let shared = index - 10 * occupancy_count;
                match shared {
                    0 => (
                        "shared_canopy_air_heat".into(),
                        ResidualUnit::WattsPerSquareMeter,
                    ),
                    1 => (
                        "shared_canopy_air_vapor".into(),
                        ResidualUnit::KilogramsPerSquareMeterSecond,
                    ),
                    2 => (
                        "ground_surface_energy".into(),
                        ResidualUnit::WattsPerSquareMeter,
                    ),
                    soil => (
                        format!(
                            "soil_thermal:{}",
                            beginning.ground.soil_nodes[soil - 3].layer_id
                        ),
                        ResidualUnit::WattsPerSquareMeter,
                    ),
                }
            };
            diagnostic_residual(identity, *raw, *tolerance, *normalized, unit)
        })
        .collect()
}

fn freeze_covered_branches(detail: &CoveredColumnEvaluation) -> CoveredFrozenBranches {
    let mut frozen = CoveredFrozenBranches {
        ground: Some(detail.ground_water.branch),
        ..Default::default()
    };
    for occupancy in &detail.occupancies {
        if let Some(identity) = occupancy
            .source_water
            .first()
            .map(|value| value.occupancy_id.clone())
        {
            frozen.wet.insert(identity, occupancy.wet_branch);
        }
        for source in &occupancy.source_water {
            frozen.root.insert(
                (source.occupancy_id.clone(), source.layer_id.clone()),
                source.branch,
            );
        }
    }
    frozen
}

fn covered_step_norms(
    applied: &[f64],
    occupancy_count: usize,
    before: &CoveredColumnEvaluation,
    after: &CoveredColumnEvaluation,
) -> CoveredStepNorms {
    let mut result = CoveredStepNorms::default();
    for index in 0..occupancy_count {
        let offset = 10 * index;
        result.hydraulic_mm = result.hydraulic_mm.max(
            applied[offset..offset + 4]
                .iter()
                .map(|value| value.abs())
                .fold(0.0, f64::max),
        );
        result.beta = result.beta.max(
            applied[offset + 4..offset + 6]
                .iter()
                .map(|value| value.abs())
                .fold(0.0, f64::max),
        );
        result.temperature_k = result.temperature_k.max(
            applied[offset + 6..offset + 10]
                .iter()
                .map(|value| value.abs())
                .fold(0.0, f64::max),
        );
        result.ci_pa = result
            .ci_pa
            .max((after.occupancies[index].ci_pa[0] - before.occupancies[index].ci_pa[0]).abs())
            .max((after.occupancies[index].ci_pa[1] - before.occupancies[index].ci_pa[1]).abs());
    }
    let common = 10 * occupancy_count;
    result.temperature_k = result.temperature_k.max(applied[common].abs()).max(
        applied[common + 2..]
            .iter()
            .map(|value| value.abs())
            .fold(0.0, f64::max),
    );
    result.humidity_kg_kg = applied[common + 1].abs();
    result
}

pub(crate) fn solve_covered_column(
    beginning: &CoveredColumnInputs,
    caps: Option<&CoveredWaterCaps>,
    initial_trial: Vec<f64>,
) -> Result<CoveredColumnSolveOutcome, LandSurfaceEnergyError> {
    let replay_map_identity = new_standalone_covered_component_replay_map_identity();
    solve_covered_column_in_map(beginning, caps, initial_trial, false, &replay_map_identity)
}

pub(crate) fn solve_v10_full_supply_final(
    beginning: &CoveredColumnInputs,
    caps: &CoveredWaterCaps,
    initial_trial: Vec<f64>,
) -> Result<CoveredColumnSolveOutcome, LandSurfaceEnergyError> {
    solve_covered_column_impl(beginning, Some(caps), initial_trial, true)
}

fn solve_covered_column_impl(
    beginning: &CoveredColumnInputs,
    caps: Option<&CoveredWaterCaps>,
    initial_trial: Vec<f64>,
    allow_v10_initial_final_acceptance: bool,
) -> Result<CoveredColumnSolveOutcome, LandSurfaceEnergyError> {
    validate_covered_caps(beginning, caps)?;
    let ground_uses_liquid_vapor_phase_domain =
        covered_ground_uses_liquid_vapor_phase_domain(beginning);
    if !covered_trial_is_valid(
        &initial_trial,
        beginning.occupancies.len(),
        ground_uses_liquid_vapor_phase_domain,
    ) {
        return Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_initial_trial",
        ));
    }
    let validated_evaluation_inputs =
        ValidatedCoveredEvaluationInputs::try_new_after_caps_validated(beginning, caps)?;
    let replay_solve_ordinal = next_covered_component_dependency_replay_solve_ordinal();
    let inactive_hydraulic_anchors = initial_trial.clone();
    let mut x = initial_trial;
    let mut last_steps = None;
    let mut backtracking_count = 0;
    let mut pivot = None;
    let mut matrix_norm = None;
    for iteration in 0..=MAX_NEWTON_ITERATIONS {
        let iteration_map =
            ValidatedCoveredIterationMap::evaluate(&validated_evaluation_inputs, &x)?;
        let detail = &iteration_map.evaluation;
        let norm = normalized_infinity_norm(&detail.normalized_residuals);
        let v10_nonpositive_assimilation = beginning.authority.admits_nonpositive_assimilation()
            && v10_nonpositive_assimilation_active(detail);
        let v10_initial_final_acceptance = allow_v10_initial_final_acceptance
            && iteration == 0
            && caps.is_some()
            && v10_nonpositive_assimilation
            && v10_initial_final_residuals_pass(&detail.normalized_residuals);
        if norm <= 1.0
            && (last_steps.is_some_and(CoveredStepNorms::accepted) || v10_initial_final_acceptance)
        {
            return accept_covered_candidate(
                beginning,
                x,
                iteration_map.evaluation,
                iteration,
                backtracking_count,
                last_steps.unwrap_or_default(),
            );
        }
        if iteration == MAX_NEWTON_ITERATIONS {
            let (occupancy_id, active_bounds) = covered_failure_metadata(beginning, detail, &x);
            return Ok(CoveredColumnSolveOutcome::Rejected(NumericalFailure {
                kind: NumericalFailureKind::IterationLimit,
                iterations: iteration,
                ordered_residuals: covered_failure_residuals(beginning, detail),
                normalized_residuals: detail.normalized_residuals.clone(),
                occupancy_id,
                active_bounds,
                failed_solution: x,
                backtracking_count,
                step_norms: last_steps.map_or_else(empty_step_norms, CoveredStepNorms::diagnostics),
                pivot_magnitude: pivot,
                matrix_norm,
            }));
        }
        let jacobian_base = iteration_map.into_jacobian_base(&x);
        let replay_base = ValidatedCoveredComponentReplaySweepBase::new(
            jacobian_base,
            if caps.is_some() {
                CoveredComponentReplaySolveClass::FixedFinal
            } else {
                CoveredComponentReplaySolveClass::Potential
            },
            replay_solve_ordinal,
            iteration,
            iteration,
        )?;
        let mut replay_audit = CoveredComponentReplaySweepGuard::new(
            new_covered_component_dependency_replay_sweep_audit(&replay_base),
        );
        let detail = &replay_base.jacobian.evaluation;
        let units: Vec<f64> = (0..beginning.occupancies.len())
            .flat_map(|_| [1000.0, 1000.0, 1000.0, 1000.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])
            .chain([1.0, 0.001, 1.0])
            .chain(std::iter::repeat_n(1.0, beginning.ground.soil_nodes.len()))
            .collect();
        let perturbations: Vec<f64> = x
            .iter()
            .zip(units.iter())
            .map(|(value, unit)| f64::EPSILON.sqrt() * value.abs().max(*unit))
            .collect();
        let mut jacobian = vec![vec![0.0; x.len()]; x.len()];
        for column_index in 0..x.len() {
            let mut minus = x.clone();
            let mut plus = x.clone();
            minus[column_index] -= perturbations[column_index];
            plus[column_index] += perturbations[column_index];
            let stencil = match covered_finite_difference_stencil(
                &x,
                &minus,
                &plus,
                beginning.occupancies.len(),
                ground_uses_liquid_vapor_phase_domain,
            ) {
                Ok(stencil) => stencil,
                Err(error) => {
                    replay_audit.push_stencil(CoveredComponentReplayStencil::RejectedBeforeProbe);
                    return Err(error);
                }
            };
            replay_audit.stencils.push(match stencil {
                CoveredFiniteDifferenceStencil::Centered => CoveredComponentReplayStencil::Centered,
                CoveredFiniteDifferenceStencil::InwardFromUpperBound => {
                    CoveredComponentReplayStencil::InwardUpper
                }
                CoveredFiniteDifferenceStencil::InwardFromLowerBound => {
                    CoveredComponentReplayStencil::InwardLower
                }
            });
            let minus_residuals = covered_trial_is_valid(
                &minus,
                beginning.occupancies.len(),
                ground_uses_liquid_vapor_phase_domain,
            )
            .then(|| {
                covered_component_temperature_probe_residuals(
                    &replay_base,
                    &minus,
                    column_index,
                    CoveredSignedProbe::Minus,
                    perturbations[column_index],
                    stencil,
                    &mut replay_audit,
                )
            })
                .transpose()?;
            let plus_residuals = covered_trial_is_valid(
                &plus,
                beginning.occupancies.len(),
                ground_uses_liquid_vapor_phase_domain,
            )
            .then(|| {
                covered_component_temperature_probe_residuals(
                    &replay_base,
                    &plus,
                    column_index,
                    CoveredSignedProbe::Plus,
                    perturbations[column_index],
                    stencil,
                    &mut replay_audit,
                )
            })
                .transpose()?;
            for row in 0..x.len() {
                jacobian[row][column_index] = covered_finite_difference_value(
                    stencil,
                    detail.normalized_residuals[row],
                    minus_residuals.as_ref().map(|value| value[row]),
                    plus_residuals.as_ref().map(|value| value[row]),
                    perturbations[column_index],
                )?;
            }
        }
        replay_audit.seal_success();
        let mut rhs: Vec<f64> = detail
            .normalized_residuals
            .iter()
            .map(|value| -value)
            .collect();
        for (occupancy_index, occupancy) in beginning.occupancies.iter().enumerate() {
            let completely_inactive = beginning.authority.admits_nonpositive_assimilation()
                && occupancy.sun.leaf_area_m2_m2_tile.to_bits() == 0.0_f64.to_bits()
                && occupancy.shade.leaf_area_m2_m2_tile.to_bits() == 0.0_f64.to_bits()
                && occupancy.stem_area_m2_m2_tile.to_bits() == 0.0_f64.to_bits()
                && occupancy.lai.to_bits() == 0.0_f64.to_bits()
                && occupancy.sai.to_bits() == 0.0_f64.to_bits();
            if !completely_inactive {
                continue;
            }
            let block_start = 10 * occupancy_index;
            for local_index in 0..6 {
                let row_index = block_start + local_index;
                let column_index = block_start + local_index;
                jacobian[row_index].fill(0.0);
                jacobian[row_index][column_index] = 1.0 / units[column_index];
                let target = if local_index < 4 {
                    inactive_hydraulic_anchors[column_index]
                } else {
                    1.0
                };
                rhs[row_index] = (target - x[column_index]) / units[column_index];
            }
        }
        let canopy_air_temperature_index = 10 * beginning.occupancies.len();
        for (occupancy_index, occupancy) in detail.occupancies.iter().enumerate() {
            for (component_index, area) in occupancy
                .component_areas_m2_m2_tile
                .iter()
                .copied()
                .enumerate()
            {
                if area.to_bits() != 0.0_f64.to_bits() {
                    continue;
                }
                let row_index = 10 * occupancy_index + 6 + component_index;
                jacobian[row_index].fill(0.0);
                jacobian[row_index][row_index] = 1.0 / units[row_index];
                let target = inactive_component_temperature_anchor_k(
                    component_index,
                    x[canopy_air_temperature_index],
                );
                rhs[row_index] = (target - x[row_index]) / units[row_index];
            }
        }
        let v10_scaled_potential = caps.is_none() && v10_nonpositive_assimilation;
        if v10_scaled_potential {
            for row in &mut jacobian {
                for (coefficient, unit) in row.iter_mut().zip(&units) {
                    *coefficient *= unit;
                }
            }
        }
        let (mut delta, current_pivot, current_matrix_norm) = match solve_linear(&jacobian, &rhs) {
            Ok(value) => value,
            Err(evidence) => {
                let (occupancy_id, active_bounds) = covered_failure_metadata(beginning, detail, &x);
                return Ok(CoveredColumnSolveOutcome::Rejected(NumericalFailure {
                    kind: NumericalFailureKind::SingularPivot,
                    iterations: iteration,
                    ordered_residuals: covered_failure_residuals(beginning, detail),
                    normalized_residuals: detail.normalized_residuals.clone(),
                    occupancy_id,
                    active_bounds,
                    failed_solution: x,
                    backtracking_count,
                    step_norms: last_steps
                        .map_or_else(empty_step_norms, CoveredStepNorms::diagnostics),
                    pivot_magnitude: Some(evidence.pivot),
                    matrix_norm: Some(evidence.matrix_norm),
                }));
            }
        };
        if v10_scaled_potential {
            for (change, unit) in delta.iter_mut().zip(&units) {
                *change *= unit;
            }
        }
        pivot = Some(current_pivot);
        matrix_norm = Some(current_matrix_norm);
        let prospective: Vec<f64> = x
            .iter()
            .zip(delta.iter())
            .map(|(value, change)| value + change)
            .collect();
        let full_trial_is_valid = covered_trial_is_valid(
            &prospective,
            beginning.occupancies.len(),
            ground_uses_liquid_vapor_phase_domain,
        );
        let complete_current_residuals_pass =
            covered_complete_residuals_pass(&detail.normalized_residuals);
        let mut full_trial_refusal =
            (!full_trial_is_valid).then_some(CoveredFullTrialNoUpdateRefusal::DomainInvalid);
        #[cfg(any(test, feature = "test-support"))]
        if !full_trial_is_valid {
            component_dependency_replay_test_support::record_component_replay_backtracking_attempt(
                0,
                0,
                1.0,
                &prospective,
                None,
                None,
                None,
                CoveredComponentReplayBacktrackingDecision::DomainInvalid,
            );
        }
        #[cfg(any(test, feature = "test-support"))]
        if full_trial_is_valid && !complete_current_residuals_pass {
            component_dependency_replay_test_support::record_component_replay_backtracking_attempt(
                0,
                0,
                1.0,
                &prospective,
                None,
                None,
                None,
                CoveredComponentReplayBacktrackingDecision::CurrentResidualNotClosed,
            );
        }
        if complete_current_residuals_pass && full_trial_is_valid {
            let prospective_detail = match evaluate_covered_column_validated(
                &validated_evaluation_inputs,
                &prospective,
                None,
                None,
            ) {
                Ok(value) => value,
                Err(error) => {
                    record_backtracking_attempt!(
                        0,
                        0,
                        1.0,
                        &prospective,
                        None,
                        None,
                        None,
                        CoveredComponentReplayBacktrackingDecision::EvaluationIncomplete,
                    );
                    return Err(error);
                }
            };
            let prospective_steps = covered_step_norms(
                &delta,
                beginning.occupancies.len(),
                detail,
                &prospective_detail,
            );
            if prospective_steps.accepted() {
                record_backtracking_attempt!(
                    0,
                    0,
                    1.0,
                    &prospective,
                    Some(&prospective_detail),
                    Some(residual_norm(&prospective_detail.normalized_residuals)),
                    Some(prospective_steps),
                    CoveredComponentReplayBacktrackingDecision::NoUpdateWitnessSelected,
                );
                return accept_covered_candidate(
                    beginning,
                    x,
                    replay_base.jacobian.evaluation,
                    iteration,
                    backtracking_count,
                    prospective_steps,
                );
            }
            if prospective_steps.governed_threshold_exceeded() {
                full_trial_refusal =
                    Some(CoveredFullTrialNoUpdateRefusal::GovernedStepThresholdExceeded);
            }
            record_backtracking_attempt!(
                0,
                0,
                1.0,
                &prospective,
                Some(&prospective_detail),
                Some(residual_norm(&prospective_detail.normalized_residuals)),
                Some(prospective_steps),
                if prospective_steps.governed_threshold_exceeded() {
                    CoveredComponentReplayBacktrackingDecision::GovernedRefusal
                } else {
                    CoveredComponentReplayBacktrackingDecision::CompleteNotAccepted
                },
            );
        }
        if let Some((exponent, steps)) = covered_first_domain_valid_halved_no_update_witness(
            &detail.normalized_residuals,
            full_trial_refusal,
            |exponent| {
                let factor = 0.5_f64.powf(f64::from(exponent));
                let trial: Vec<f64> = x
                    .iter()
                    .zip(delta.iter())
                    .map(|(value, change)| value + factor * change)
                    .collect();
                if !covered_trial_is_valid(
                    &trial,
                    beginning.occupancies.len(),
                    ground_uses_liquid_vapor_phase_domain,
                ) {
                    record_backtracking_attempt!(
                        1,
                        exponent,
                        factor,
                        &trial,
                        None,
                        None,
                        None,
                        CoveredComponentReplayBacktrackingDecision::DomainInvalid,
                    );
                    return CoveredHalvedTrialProbe::DomainInvalid;
                }
                let Ok(trial_detail) = evaluate_covered_column_validated(
                    &validated_evaluation_inputs,
                    &trial,
                    None,
                    None,
                ) else {
                    record_backtracking_attempt!(
                        1,
                        exponent,
                        factor,
                        &trial,
                        None,
                        None,
                        None,
                        CoveredComponentReplayBacktrackingDecision::EvaluationIncomplete,
                    );
                    return CoveredHalvedTrialProbe::EvaluationIncomplete;
                };
                let applied: Vec<f64> = delta.iter().map(|value| factor * value).collect();
                let steps = covered_step_norms(
                    &applied,
                    beginning.occupancies.len(),
                    detail,
                    &trial_detail,
                );
                record_backtracking_attempt!(
                    1,
                    exponent,
                    factor,
                    &trial,
                    Some(&trial_detail),
                    Some(normalized_infinity_norm(&trial_detail.normalized_residuals,)),
                    Some(steps),
                    if steps.accepted() {
                        CoveredComponentReplayBacktrackingDecision::NoUpdateWitnessSelected
                    } else {
                        CoveredComponentReplayBacktrackingDecision::CompleteNotAccepted
                    },
                );
                CoveredHalvedTrialProbe::Complete(steps)
            },
        ) {
            return accept_covered_candidate(
                beginning,
                x,
                replay_base.jacobian.evaluation,
                iteration,
                backtracking_count + exponent,
                steps,
            );
        }
        let mut accepted = None;
        let mut rejected_step_norms = None;
        for exponent in 0..=MAX_BACKTRACKING_HALVINGS {
            let factor = 0.5_f64.powf(f64::from(exponent));
            let trial: Vec<f64> = x
                .iter()
                .zip(delta.iter())
                .map(|(value, change)| value + factor * change)
                .collect();
            if !covered_trial_is_valid(
                &trial,
                beginning.occupancies.len(),
                ground_uses_liquid_vapor_phase_domain,
            ) {
                record_backtracking_attempt!(
                    2,
                    exponent,
                    factor,
                    &trial,
                    None,
                    None,
                    None,
                    CoveredComponentReplayBacktrackingDecision::DomainInvalid,
                );
                continue;
            }
            let Ok(trial_detail) =
                evaluate_covered_column_validated(&validated_evaluation_inputs, &trial, None, None)
            else {
                record_backtracking_attempt!(
                    2,
                    exponent,
                    factor,
                    &trial,
                    None,
                    None,
                    None,
                    CoveredComponentReplayBacktrackingDecision::EvaluationIncomplete,
                );
                continue;
            };
            let applied: Vec<f64> = delta.iter().map(|value| factor * value).collect();
            let steps =
                covered_step_norms(&applied, beginning.occupancies.len(), detail, &trial_detail);
            rejected_step_norms = Some(steps);
            let trial_norm = residual_norm(&trial_detail.normalized_residuals);
            let residual_accepted =
                is_strict_residual_decrease(norm, &trial_detail.normalized_residuals);
            record_backtracking_attempt!(
                2,
                exponent,
                factor,
                &trial,
                Some(&trial_detail),
                Some(trial_norm),
                Some(steps),
                if residual_accepted {
                    CoveredComponentReplayBacktrackingDecision::ResidualAccepted
                } else {
                    CoveredComponentReplayBacktrackingDecision::ResidualRejected
                },
            );
            if residual_accepted {
                accepted = Some((trial, steps, exponent));
                break;
            }
        }
        if let Some((trial, steps, exponent)) = accepted {
            x = trial;
            last_steps = Some(steps);
            backtracking_count += exponent;
        } else {
            let (occupancy_id, active_bounds) = covered_failure_metadata(beginning, detail, &x);
            return Ok(CoveredColumnSolveOutcome::Rejected(NumericalFailure {
                kind: NumericalFailureKind::BacktrackingLimit,
                iterations: iteration,
                ordered_residuals: covered_failure_residuals(beginning, detail),
                normalized_residuals: detail.normalized_residuals.clone(),
                occupancy_id,
                active_bounds,
                failed_solution: x,
                backtracking_count: backtracking_count + MAX_BACKTRACKING_HALVINGS,
                step_norms: rejected_step_norms
                    .or(last_steps)
                    .map_or_else(empty_step_norms, CoveredStepNorms::diagnostics),
                pivot_magnitude: pivot,
                matrix_norm,
            }));
        }
    }
    Err(LandSurfaceEnergyError::NumericalAcceptedResidual)
}

fn accept_covered_candidate(
    beginning: &CoveredColumnInputs,
    solution: Vec<f64>,
    detail: CoveredColumnEvaluation,
    iterations: u32,
    backtracking_count: u32,
    step_norms: CoveredStepNorms,
) -> Result<CoveredColumnSolveOutcome, LandSurfaceEnergyError> {
    for (input, evaluated) in beginning.occupancies.iter().zip(&detail.occupancies) {
        if exact_inactive_hydraulic_occupancy(beginning.authority, input)
            && evaluated
                .source_water
                .iter()
                .any(|source| !exact_inactive_source_water(source))
        {
            return Err(LandSurfaceEnergyError::UnsupportedDomain(
                "inactive_hydraulic_source",
            ));
        }
    }
    if beginning.authority.admits_nonpositive_assimilation()
        && detail
            .occupancies
            .iter()
            .flat_map(|occupancy| &occupancy.source_water)
            .any(|source| source.law_kg_m2_tile_s < 0.0 || source.final_kg_m2_tile_s < 0.0)
    {
        return Err(LandSurfaceEnergyError::UnsupportedDomain(
            "hydraulic_redistribution",
        ));
    }
    let root_water = detail
        .occupancies
        .iter()
        .flat_map(|value| value.source_water.clone())
        .collect();
    Ok(CoveredColumnSolveOutcome::Accepted(Box::new(
        CoveredColumnCandidate {
            solution,
            surface_enthalpy_j_m2_tile: detail.ending_surface_enthalpy_j_m2_tile,
            soil_temperature_k: detail.soil_temperature_k.clone(),
            root_water,
            ground_water: detail.ground_water,
            iterations,
            backtracking_count,
            step_norms,
            evaluation: detail,
        },
    )))
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredPotentialFinalTransaction {
    pub potential: Box<CoveredColumnCandidate>,
    pub final_pass: Box<CoveredColumnCandidate>,
}

#[cfg(test)]
pub(crate) fn execute_covered_potential_final(
    beginning: &CoveredColumnInputs,
    potential_initial_trial: Vec<f64>,
    caps: &CoveredWaterCaps,
    final_initial_trial: Vec<f64>,
) -> Result<CoveredPotentialFinalTransaction, LandSurfaceEnergyError> {
    let replay_map_identity = new_standalone_covered_component_replay_map_identity();
    let potential = match solve_covered_column_in_map(
        beginning,
        None,
        potential_initial_trial,
        false,
        &replay_map_identity,
    )? {
        CoveredColumnSolveOutcome::Accepted(value) => value,
        CoveredColumnSolveOutcome::Rejected(_) => {
            return Err(LandSurfaceEnergyError::NumericalAcceptedResidual);
        }
    };
    for source in &potential.root_water {
        let cap = caps
            .root
            .get(&(source.occupancy_id.clone(), source.layer_id.clone()))
            .ok_or(LandSurfaceEnergyError::water_cardinality(
                "missing_potential_root_request_identity",
            ))?;
        let potential_rate =
            source.request_kg_m2_stand_ground / (beginning.tile_fraction * beginning.interval_s);
        if cap.request_rate_kg_m2_tile_s != potential_rate {
            return Err(LandSurfaceEnergyError::water_identity(
                "changed_potential_root_request",
            ));
        }
    }
    let ground_potential_rate = potential.ground_water.request_kg_m2_stand_ground
        / (beginning.tile_fraction * beginning.interval_s);
    if caps.ground.request_rate_kg_m2_tile_s != ground_potential_rate {
        return Err(LandSurfaceEnergyError::water_identity(
            "changed_potential_ground_request",
        ));
    }
    let final_pass = match solve_covered_column_in_map(
        beginning,
        Some(caps),
        final_initial_trial,
        false,
        &replay_map_identity,
    )? {
        CoveredColumnSolveOutcome::Accepted(value) => value,
        CoveredColumnSolveOutcome::Rejected(_) => {
            return Err(LandSurfaceEnergyError::NumericalAcceptedResidual);
        }
    };
    for source in &final_pass.root_water {
        let authorization = source.authorization_kg_m2_stand_ground.ok_or(
            LandSurfaceEnergyError::water_cardinality("missing_final_root_authorization"),
        )?;
        if source.finalized_use_kg_m2_stand_ground > authorization
            || authorization > source.request_kg_m2_stand_ground
        {
            return Err(LandSurfaceEnergyError::water_bound("root_D/A/F"));
        }
    }
    let ground_authorization = final_pass
        .ground_water
        .authorization_kg_m2_stand_ground
        .ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing_final_ground_authorization",
        ))?;
    if final_pass.ground_water.finalized_use_kg_m2_stand_ground > ground_authorization
        || ground_authorization > final_pass.ground_water.request_kg_m2_stand_ground
    {
        return Err(LandSurfaceEnergyError::water_bound("ground_D/A/F"));
    }
    Ok(CoveredPotentialFinalTransaction {
        potential,
        final_pass,
    })
}

#[cfg(test)]
mod inactive_hydraulic_source_tests {
    use super::*;

    fn exact_inactive_source() -> SourceWaterFlux {
        SourceWaterFlux {
            occupancy_id: "inactive-occupancy".into(),
            layer_id: "soil-1".into(),
            law_kg_m2_tile_s: 0.0,
            final_kg_m2_tile_s: 0.0,
            request_kg_m2_stand_ground: 0.0,
            authorization_kg_m2_stand_ground: Some(0.0),
            finalized_use_kg_m2_stand_ground: 0.0,
            branch: WaterBranch::ConstitutiveLaw,
        }
    }

    #[test]
    fn inactive_source_requires_exact_zero_flow_demand_and_inactive_branch() {
        let source = exact_inactive_source();
        assert!(exact_inactive_source_water(&source));

        let mut negative_law = source.clone();
        negative_law.law_kg_m2_tile_s = -f64::from_bits(1);
        assert!(!exact_inactive_source_water(&negative_law));

        for poison in [
            |source: &mut SourceWaterFlux| source.final_kg_m2_tile_s = f64::from_bits(1),
            |source: &mut SourceWaterFlux| {
                source.request_kg_m2_stand_ground = f64::from_bits(1);
            },
            |source: &mut SourceWaterFlux| {
                source.authorization_kg_m2_stand_ground = Some(f64::from_bits(1));
            },
            |source: &mut SourceWaterFlux| {
                source.finalized_use_kg_m2_stand_ground = f64::from_bits(1);
            },
        ] {
            let mut poisoned = source.clone();
            poison(&mut poisoned);
            assert!(!exact_inactive_source_water(&poisoned));
        }

        let mut active_branch = source;
        active_branch.branch = WaterBranch::AuthorizationActiveOrTie;
        assert!(!exact_inactive_source_water(&active_branch));
    }
}

#[cfg(test)]
mod covered_halved_no_update_witness_tests {
    use super::*;

    fn passing_steps() -> CoveredStepNorms {
        CoveredStepNorms {
            hydraulic_mm: 1.0e-7,
            beta: 1.0e-10,
            temperature_k: 1.0e-8,
            humidity_kg_kg: 1.0e-12,
            ci_pa: f64::MAX,
        }
    }

    #[test]
    fn each_full_witness_refusal_and_first_domain_valid_halving_admit_no_update() {
        for refusal in [
            CoveredFullTrialNoUpdateRefusal::DomainInvalid,
            CoveredFullTrialNoUpdateRefusal::GovernedStepThresholdExceeded,
        ] {
            assert!(covered_halved_no_update_witness(
                &[1.0, -1.0, 0.0],
                Some(refusal),
                true,
                passing_steps(),
            ));
        }
    }

    #[test]
    fn enclosing_preflight_skips_domain_invalid_trials_and_returns_first_complete_witness() {
        let mut examined = Vec::new();
        let witness = covered_first_domain_valid_halved_no_update_witness(
            &[0.0, 1.0],
            Some(CoveredFullTrialNoUpdateRefusal::DomainInvalid),
            |exponent| {
                examined.push(exponent);
                if exponent < 3 {
                    CoveredHalvedTrialProbe::DomainInvalid
                } else {
                    CoveredHalvedTrialProbe::Complete(passing_steps())
                }
            },
        );
        assert_eq!(witness, Some((3, passing_steps())));
        assert_eq!(examined, [1, 2, 3]);
    }

    #[test]
    fn enclosing_preflight_does_not_skip_incomplete_or_failed_first_domain_valid_trial() {
        let mut failed_steps = passing_steps();
        failed_steps.hydraulic_mm = 1.0e-7 + f64::EPSILON;
        for first_domain_valid in [
            CoveredHalvedTrialProbe::EvaluationIncomplete,
            CoveredHalvedTrialProbe::Complete(failed_steps),
        ] {
            let mut examined = Vec::new();
            let witness = covered_first_domain_valid_halved_no_update_witness(
                &[0.0, 1.0],
                Some(CoveredFullTrialNoUpdateRefusal::GovernedStepThresholdExceeded),
                |exponent| {
                    examined.push(exponent);
                    if exponent == 1 {
                        first_domain_valid
                    } else {
                        CoveredHalvedTrialProbe::Complete(passing_steps())
                    }
                },
            );
            assert_eq!(witness, None);
            assert_eq!(examined, [1]);
        }

        let mut examined_without_trigger = false;
        assert_eq!(
            covered_first_domain_valid_halved_no_update_witness(&[0.0, 1.0], None, |_| {
                examined_without_trigger = true;
                CoveredHalvedTrialProbe::Complete(passing_steps())
            },),
            None
        );
        assert!(!examined_without_trigger);
    }

    #[test]
    fn complete_residual_passing_full_and_later_domain_valid_poisons_refuse_the_witness() {
        for residual_poison in [1.0 + f64::EPSILON, f64::INFINITY, f64::NAN] {
            assert!(!covered_halved_no_update_witness(
                &[0.0, residual_poison, 0.5],
                Some(CoveredFullTrialNoUpdateRefusal::DomainInvalid),
                true,
                passing_steps(),
            ));
        }
        assert!(!covered_halved_no_update_witness(
            &[0.0, 1.0],
            None,
            true,
            passing_steps(),
        ));
        assert!(!covered_halved_no_update_witness(
            &[0.0, 1.0],
            Some(CoveredFullTrialNoUpdateRefusal::DomainInvalid),
            false,
            passing_steps(),
        ));
    }

    #[test]
    fn each_governed_prospective_step_coordinate_poison_refuses_the_witness() {
        let mut hydraulic = passing_steps();
        hydraulic.hydraulic_mm = 1.0e-7 + f64::EPSILON;
        let mut beta = passing_steps();
        beta.beta = 1.0e-10 + f64::EPSILON;
        let mut temperature = passing_steps();
        temperature.temperature_k = 1.0e-8 + f64::EPSILON;
        let mut humidity = passing_steps();
        humidity.humidity_kg_kg = 1.0e-12 + f64::EPSILON;
        let mut nonfinite = passing_steps();
        nonfinite.beta = f64::NAN;
        for poison in [hydraulic, beta, temperature, humidity, nonfinite] {
            assert!(!covered_halved_no_update_witness(
                &[0.0, 1.0],
                Some(CoveredFullTrialNoUpdateRefusal::GovernedStepThresholdExceeded),
                true,
                poison,
            ));
        }
    }

    #[test]
    fn full_trial_threshold_excess_classification_requires_every_governed_step_finite() {
        for poison in [
            |steps: &mut CoveredStepNorms, value| steps.hydraulic_mm = value,
            |steps: &mut CoveredStepNorms, value| steps.beta = value,
            |steps: &mut CoveredStepNorms, value| steps.temperature_k = value,
            |steps: &mut CoveredStepNorms, value| steps.humidity_kg_kg = value,
        ] {
            for nonfinite in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut steps = passing_steps();
                poison(&mut steps, nonfinite);
                assert!(!steps.governed_threshold_exceeded());
            }
        }

        for poison in [
            |steps: &mut CoveredStepNorms| steps.hydraulic_mm = 1.0e-7 + f64::EPSILON,
            |steps: &mut CoveredStepNorms| steps.beta = 1.0e-10 + f64::EPSILON,
            |steps: &mut CoveredStepNorms| steps.temperature_k = 1.0e-8 + f64::EPSILON,
            |steps: &mut CoveredStepNorms| steps.humidity_kg_kg = 1.0e-12 + f64::EPSILON,
        ] {
            let mut steps = passing_steps();
            poison(&mut steps);
            assert!(steps.governed_threshold_exceeded());
        }
    }
}

#[cfg(test)]
mod covered_finite_difference_stencil_tests {
    use super::*;

    const OCCUPANCY_COUNT: usize = 1;

    fn admitted_trial() -> Vec<f64> {
        vec![
            -5_000.0,
            -5_000.0,
            -5_000.0,
            -5_000.0,
            0.5,
            0.5,
            LIQUID_VAPOR_PHASE_MINIMUM_K,
            LIQUID_VAPOR_PHASE_MINIMUM_K,
            LIQUID_VAPOR_PHASE_MINIMUM_K,
            273.15,
            273.15,
            0.01,
            273.15,
            273.15,
        ]
    }

    fn canonical_probes(current: &[f64], column: usize, unit_scale: f64) -> (Vec<f64>, Vec<f64>) {
        let h = f64::EPSILON.sqrt() * current[column].abs().max(unit_scale);
        let mut minus = current.to_vec();
        let mut plus = current.to_vec();
        minus[column] -= h;
        plus[column] += h;
        (minus, plus)
    }

    #[test]
    fn beta_zero_and_one_select_only_the_unique_inward_probe() {
        let mut lower = admitted_trial();
        lower[4] = 0.0;
        let (minus, plus) = canonical_probes(&lower, 4, 1.0);
        assert_eq!(
            covered_finite_difference_stencil(&lower, &minus, &plus, OCCUPANCY_COUNT, false)
                .expect("beta=0 is an admitted closed bound"),
            CoveredFiniteDifferenceStencil::InwardFromLowerBound
        );

        let mut upper = admitted_trial();
        upper[5] = 1.0;
        let (minus, plus) = canonical_probes(&upper, 5, 1.0);
        assert_eq!(
            covered_finite_difference_stencil(&upper, &minus, &plus, OCCUPANCY_COUNT, false)
                .expect("beta=1 is an admitted closed bound"),
            CoveredFiniteDifferenceStencil::InwardFromUpperBound
        );
    }

    #[test]
    fn exact_phase_minimum_uses_inward_probe_for_active_or_zero_area_coordinates() {
        let current = admitted_trial();
        for column in [6, 7, 8] {
            let (minus, plus) = canonical_probes(&current, column, 1.0);
            let stencil =
                covered_finite_difference_stencil(&current, &minus, &plus, OCCUPANCY_COUNT, false)
                    .expect("exact liquid-vapor phase minimum is admitted");
            assert_eq!(
                stencil,
                CoveredFiniteDifferenceStencil::InwardFromLowerBound,
                "the trial domain is identical for active and zero-area coordinate {column}"
            );
        }

        let ground_temperature_column = 10 * OCCUPANCY_COUNT + 2;
        let (minus, plus) = canonical_probes(&current, ground_temperature_column, 1.0);
        assert_eq!(
            covered_finite_difference_stencil(&current, &minus, &plus, OCCUPANCY_COUNT, true,)
                .expect("exact liquid-bearing ground phase minimum is admitted"),
            CoveredFiniteDifferenceStencil::InwardFromLowerBound
        );
    }

    #[test]
    fn interior_coordinate_retains_exact_centered_stencil() {
        let current = admitted_trial();
        let (minus, plus) = canonical_probes(&current, 4, 1.0);
        assert_eq!(
            covered_finite_difference_stencil(&current, &minus, &plus, OCCUPANCY_COUNT, false)
                .expect("interior probes"),
            CoveredFiniteDifferenceStencil::Centered
        );
    }

    #[test]
    fn centered_and_inward_formulas_are_exact_and_missing_probes_reject() {
        assert_eq!(
            covered_finite_difference_value(
                CoveredFiniteDifferenceStencil::Centered,
                10.0,
                Some(7.0),
                Some(15.0),
                2.0,
            )
            .expect("centered formula")
            .to_bits(),
            2.0_f64.to_bits()
        );
        assert_eq!(
            covered_finite_difference_value(
                CoveredFiniteDifferenceStencil::InwardFromUpperBound,
                10.0,
                Some(7.0),
                None,
                2.0,
            )
            .expect("upper-bound inward formula")
            .to_bits(),
            1.5_f64.to_bits()
        );
        assert_eq!(
            covered_finite_difference_value(
                CoveredFiniteDifferenceStencil::InwardFromLowerBound,
                10.0,
                None,
                Some(15.0),
                2.0,
            )
            .expect("lower-bound inward formula")
            .to_bits(),
            2.5_f64.to_bits()
        );
        assert!(matches!(
            covered_finite_difference_value(
                CoveredFiniteDifferenceStencil::Centered,
                10.0,
                None,
                Some(15.0),
                2.0,
            ),
            Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "covered_jacobian_bound"
            ))
        ));
    }

    #[test]
    fn outside_current_and_two_inadmissible_probes_fail_closed() {
        let mut outside = admitted_trial();
        outside[4] = -f64::EPSILON;
        let (minus, plus) = canonical_probes(&outside, 4, 1.0);
        assert!(matches!(
            covered_finite_difference_stencil(&outside, &minus, &plus, OCCUPANCY_COUNT, false,),
            Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "covered_jacobian_bound"
            ))
        ));

        let current = admitted_trial();
        let mut inadmissible_minus = current.clone();
        inadmissible_minus[4] = -f64::EPSILON;
        let mut inadmissible_plus = current.clone();
        inadmissible_plus[11] = 0.1 + f64::EPSILON;
        assert!(matches!(
            covered_finite_difference_stencil(
                &current,
                &inadmissible_minus,
                &inadmissible_plus,
                OCCUPANCY_COUNT,
                false,
            ),
            Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "covered_jacobian_bound"
            ))
        ));
    }

    #[test]
    fn exact_phase_bound_outward_newton_direction_has_no_admitted_backtracking_factor() {
        let current = admitted_trial();
        let ground_temperature_column = 10 * OCCUPANCY_COUNT + 2;
        let outward_change_k = -4.198_172_269_516_074;
        for exponent in 0..=MAX_BACKTRACKING_HALVINGS {
            let factor = 0.5_f64.powf(f64::from(exponent));
            let mut trial = current.clone();
            trial[ground_temperature_column] += factor * outward_change_k;
            assert!(
                !covered_trial_is_valid(&trial, OCCUPANCY_COUNT, true),
                "outward phase-bound step unexpectedly admitted at 2^-{exponent}"
            );
        }
    }
}
