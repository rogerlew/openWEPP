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

#[cfg(test)]
std::thread_local! {
    static COVERED_JACOBIAN_FULL_PROBE_AUDIT: std::cell::Cell<Option<u32>> = const { std::cell::Cell::new(None) };
    static FORCE_COMPLETE_COVERED_JACOBIAN_PROBES: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
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

#[cfg(test)]
impl<'a> ValidatedCoveredJacobianBase<'a> {
    fn evaluate(
        validated: &'a ValidatedCoveredEvaluationInputs<'a>,
        trial: &[f64],
    ) -> Result<Self, LandSurfaceEnergyError> {
        Ok(ValidatedCoveredIterationMap::evaluate(validated, trial)?
            .into_jacobian_base(trial))
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
/// Under represented snow, ground and soil temperatures are exact identity
/// anchors and cannot affect any other residual.  Reuse is therefore confined
/// to those columns and to this validated solve; every other probe executes the
/// complete evaluator.
fn covered_jacobian_probe_residuals(
    base: &ValidatedCoveredJacobianBase<'_>,
    probe: &[f64],
    column_index: usize,
) -> Result<Vec<f64>, LandSurfaceEnergyError> {
    let validated = base.validated;
    let current = &base.evaluation;
    let frozen = Some(&base.frozen);
    if probe.len() != base.trial.len() || column_index >= probe.len() {
        record_covered_jacobian_full_probe_audit();
        return Ok(
            evaluate_covered_column_validated(validated, probe, frozen, None)?.normalized_residuals,
        );
    }
    if probe.iter().enumerate().any(|(index, value)| {
        index != column_index && value.to_bits() != base.trial[index].to_bits()
    }) {
        record_covered_jacobian_full_probe_audit();
        return Ok(
            evaluate_covered_column_validated(validated, probe, frozen, None)?.normalized_residuals,
        );
    }
    if !force_complete_covered_jacobian_probes() {
        if let Some(anchor_k) = validated.stage3_identity_anchor_k(column_index) {
            let mut residuals = current.normalized_residuals.clone();
            let raw_residual = probe[column_index] - anchor_k;
            residuals[column_index] = raw_residual / STAGE3_COVERED_IDENTITY_TOLERANCE_K;
            return Ok(residuals);
        }
    }
    record_covered_jacobian_full_probe_audit();
    Ok(evaluate_covered_column_validated(validated, probe, frozen, None)?.normalized_residuals)
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
    solve_covered_column_impl(beginning, caps, initial_trial, false)
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
        let detail = &jacobian_base.evaluation;
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
            let stencil = covered_finite_difference_stencil(
                &x,
                &minus,
                &plus,
                beginning.occupancies.len(),
                ground_uses_liquid_vapor_phase_domain,
            )?;
            // Preserve the canonical minus-then-plus evaluation order while
            // never evaluating a constitutively inadmissible boundary probe.
            let minus_residuals = covered_trial_is_valid(
                &minus,
                beginning.occupancies.len(),
                ground_uses_liquid_vapor_phase_domain,
            )
            .then(|| {
                covered_jacobian_probe_residuals(
                    &jacobian_base,
                    &minus,
                    column_index,
                )
            })
            .transpose()?;
            let plus_residuals = covered_trial_is_valid(
                &plus,
                beginning.occupancies.len(),
                ground_uses_liquid_vapor_phase_domain,
            )
            .then(|| {
                covered_jacobian_probe_residuals(
                    &jacobian_base,
                    &plus,
                    column_index,
                )
            })
            .transpose()?;
            for row in 0..x.len() {
                jacobian[row][column_index] = covered_finite_difference_value(
                    stencil,
                    detail.normalized_residuals[row],
                    minus_residuals
                        .as_ref()
                        .map(|value| value[row]),
                    plus_residuals
                        .as_ref()
                        .map(|value| value[row]),
                    perturbations[column_index],
                )?;
            }
        }
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
                let (occupancy_id, active_bounds) =
                    covered_failure_metadata(beginning, detail, &x);
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
        if complete_current_residuals_pass && full_trial_is_valid {
            let prospective_detail = evaluate_covered_column_validated(
                &validated_evaluation_inputs,
                &prospective,
                None,
                None,
            )?;
            let prospective_steps = covered_step_norms(
                &delta,
                beginning.occupancies.len(),
                detail,
                &prospective_detail,
            );
            if prospective_steps.accepted() {
                return accept_covered_candidate(
                    beginning,
                    x,
                    jacobian_base.evaluation,
                    iteration,
                    backtracking_count,
                    prospective_steps,
                );
            }
            if prospective_steps.governed_threshold_exceeded() {
                full_trial_refusal =
                    Some(CoveredFullTrialNoUpdateRefusal::GovernedStepThresholdExceeded);
            }
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
                    return CoveredHalvedTrialProbe::DomainInvalid;
                }
                let Ok(trial_detail) = evaluate_covered_column_validated(
                    &validated_evaluation_inputs,
                    &trial,
                    None,
                    None,
                ) else {
                    return CoveredHalvedTrialProbe::EvaluationIncomplete;
                };
                let applied: Vec<f64> = delta.iter().map(|value| factor * value).collect();
                let steps = covered_step_norms(
                    &applied,
                    beginning.occupancies.len(),
                    detail,
                    &trial_detail,
                );
                CoveredHalvedTrialProbe::Complete(steps)
            },
        ) {
            return accept_covered_candidate(
                beginning,
                x,
                jacobian_base.evaluation,
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
                continue;
            }
            let Ok(trial_detail) = evaluate_covered_column_validated(
                &validated_evaluation_inputs,
                &trial,
                None,
                None,
            ) else {
                continue;
            };
            let applied: Vec<f64> = delta.iter().map(|value| factor * value).collect();
            let steps = covered_step_norms(
                &applied,
                beginning.occupancies.len(),
                detail,
                &trial_detail,
            );
            rejected_step_norms = Some(steps);
            if is_strict_residual_decrease(norm, &trial_detail.normalized_residuals) {
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

/// Execute the owner-uncapped pass and the fixed-cap pass from the same
/// immutable beginning problem. The supplied cap batch must preserve every
/// potential request identity and amount exactly.
#[cfg(test)]
pub(crate) fn execute_covered_potential_final(
    beginning: &CoveredColumnInputs,
    potential_initial_trial: Vec<f64>,
    caps: &CoveredWaterCaps,
    final_initial_trial: Vec<f64>,
) -> Result<CoveredPotentialFinalTransaction, LandSurfaceEnergyError> {
    let potential = match solve_covered_column(beginning, None, potential_initial_trial)? {
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
    // `beginning`, rather than `potential`, is deliberately passed here.
    let final_pass = match solve_covered_column(beginning, Some(caps), final_initial_trial)? {
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
