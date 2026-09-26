//! Analytic expected-red controls for the deferred Stage 2 controller body.
use super::m1_trust_region_controller::M1TrustRegionControllerError;
use super::m1_trust_region_controller_interface::{
    M1_TRUST_REGION_CONTROLLER_DIMENSION as N, M1TrustRegionControllerEvaluation as Evaluation,
    M1TrustRegionControllerEvaluator as Evaluator, M1TrustRegionCurrentHydraulics,
};
const DERIVED_TOLERANCE: f64 = 1e-12;
#[derive(Clone, Copy)]
enum FixtureKind {
    Ordinary(f64),
    FrozenRawJacobian,
    ConstrainedStationarity,
    FullNoUpdate,
    PostUpdateAdmission,
    ReplacementWitness,
    ReplacementWitnessThreeExcess,
    ReplacementWitnessFullDomainInvalid,
    ReplacementWitnessInterveningDomainInvalid,
    ReplacementWitnessFullEvaluationError,
    ReplacementWitnessReplacementEvaluationError,
    ReplacementWitnessFullIncompleteGoverned,
    ReplacementWitnessMixedThenComplete,
    ReplacementWitnessThreeExcessMixedReplacement,
    ReplacementWitnessExhaustedOrdinaryUpdate,
    NegativeHumidityExcess,
    PhaseUnchanged,
    PhaseChanged,
    PhaseFinalDisagreement,
    PhasePredictorSelectionError,
    PhaseFinalSelectionError,
    PhaseSelectedSideError,
    PhaseSelectedRetry,
    PhaseSelectedRetryDrift,
    PhaseAcceptedUpdateReset,
    ExactRoot,
    HydraulicPoison(usize),
    WetNoneNone,
    WetMixed,
    LongProgress { target: f64, perturb_at_183: bool },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FixturePhase {
    Natural,
    Selected,
}
#[derive(Clone, Copy)]
enum EvaluationFault {
    RawResidualNaN(usize),
    DynamicNormalizerNaN(usize),
    DynamicNormalizerZero(usize),
    CoordinateNaN(usize),
    WetTemperatureNaN(usize),
    HydraulicNaN(usize),
    BetaNaN(usize),
    CiNaN(usize),
}
#[derive(Clone, Copy)]
enum GovernedFiniteProjection {
    TemperatureCoordinate(usize, f64),
    WetTemperature(usize, f64),
    Humidity(f64),
    Hydraulic(usize, f64),
    Beta(usize, f64),
    Ci(usize, f64),
}
fn raw_normalizer_faults() -> Vec<(EvaluationFault, &'static str)> {
    (0..N)
        .map(|index| {
            (
                EvaluationFault::RawResidualNaN(index),
                "nonfinite weighting",
            )
        })
        .chain((0..N).flat_map(|index| {
            [
                (
                    EvaluationFault::DynamicNormalizerNaN(index),
                    "invalid normalizer",
                ),
                (
                    EvaluationFault::DynamicNormalizerZero(index),
                    "invalid normalizer",
                ),
            ]
        }))
        .collect()
}
fn governed_faults() -> Vec<EvaluationFault> {
    let mut faults = [0, 1, 2, 6, 7, 8, 12, 14, 15, 16, 17, 18, 19, 20]
        .into_iter()
        .map(EvaluationFault::CoordinateNaN)
        .collect::<Vec<_>>();
    faults.push(EvaluationFault::CoordinateNaN(13));
    faults.extend((0..2).map(EvaluationFault::WetTemperatureNaN));
    faults.extend((0..8).map(EvaluationFault::HydraulicNaN));
    faults.extend((0..4).map(EvaluationFault::BetaNaN));
    faults.extend((0..4).map(EvaluationFault::CiNaN));
    faults
}
fn governed_threshold_roles() -> Vec<GovernedFiniteProjection> {
    [0, 1, 2, 6, 7, 8, 12, 14, 15, 16, 17, 18, 19, 20]
        .into_iter()
        .map(|index| GovernedFiniteProjection::TemperatureCoordinate(index, 0.0))
        .chain((0..2).map(|index| GovernedFiniteProjection::WetTemperature(index, 0.0)))
        .chain(core::iter::once(GovernedFiniteProjection::Humidity(0.0)))
        .chain((0..8).map(|index| GovernedFiniteProjection::Hydraulic(index, 0.0)))
        .chain((0..4).map(|index| GovernedFiniteProjection::Beta(index, 0.0)))
        .collect()
}
fn threshold_projection(
    role: GovernedFiniteProjection,
    negative: bool,
    above: bool,
) -> (GovernedFiniteProjection, f64, f64) {
    match role {
        GovernedFiniteProjection::TemperatureCoordinate(index, _)
        | GovernedFiniteProjection::WetTemperature(index, _) => {
            let base = 300.0_f64;
            let offset = if above { 175_922 } else { 175_921 };
            let value = f64::from_bits(if negative {
                base.to_bits() - offset
            } else {
                base.to_bits() + offset
            });
            let projection = match role {
                GovernedFiniteProjection::TemperatureCoordinate(_, _) => {
                    GovernedFiniteProjection::TemperatureCoordinate(index, value)
                }
                GovernedFiniteProjection::WetTemperature(_, _) => {
                    GovernedFiniteProjection::WetTemperature(index, value)
                }
                _ => unreachable!(),
            };
            (projection, base, 1.0e-8)
        }
        GovernedFiniteProjection::Humidity(_) => {
            let base = 0.01_f64;
            let offset = if above { 576_461 } else { 576_460 };
            let value = f64::from_bits(if negative {
                base.to_bits() - offset
            } else {
                base.to_bits() + offset
            });
            (GovernedFiniteProjection::Humidity(value), base, 1.0e-12)
        }
        GovernedFiniteProjection::Hydraulic(index, _) => {
            let threshold = 1.0e-7_f64;
            let magnitude = if above {
                f64::from_bits(threshold.to_bits() + 1)
            } else {
                f64::from_bits(threshold.to_bits() - 1)
            };
            let value = if negative { -magnitude } else { magnitude };
            (
                GovernedFiniteProjection::Hydraulic(index, value),
                0.0,
                threshold,
            )
        }
        GovernedFiniteProjection::Beta(index, _) => {
            let base = 0.5_f64;
            let offset = if negative {
                if above { 1_801_440 } else { 1_801_439 }
            } else if above {
                900_720
            } else {
                900_719
            };
            let value = f64::from_bits(if negative {
                base.to_bits() - offset
            } else {
                base.to_bits() + offset
            });
            (GovernedFiniteProjection::Beta(index, value), base, 1.0e-10)
        }
        GovernedFiniteProjection::Ci(_, _) => unreachable!(),
    }
}
impl FixturePhase {
    const fn code(self) -> u8 {
        match self {
            Self::Natural => 0,
            Self::Selected => 1,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Payload {
    materialized_coordinate_bits: [u64; N],
    marker: u64,
}
#[derive(Clone, Debug, PartialEq)]
enum FixtureOperation {
    Evaluate([f64; N]),
    PreEvaluatorTrialDomain([f64; N]),
    RawJacobian {
        base: [f64; N],
        selected_phase: Option<u8>,
        predictor_direction: [f64; N],
    },
    CompleteCurrentHydraulics([f64; N]),
    SelectPhase {
        base: [f64; N],
        step: [f64; N],
    },
    SelectedSideProbe {
        base: [f64; N],
        phase: u8,
        direction: [f64; N],
    },
    PhaseActiveSetInconsistent {
        selected_phase: u8,
        final_phase: u8,
    },
    Materialize([f64; N]),
}
struct Fixture {
    kind: FixtureKind,
    base: [f64; N],
    scale: [f64; N],
    lower: [f64; N],
    upper: [f64; N],
    operation_log: Vec<FixtureOperation>,
    evaluator_entries: u16,
    owner_sentinel: u64,
    materializer_inputs: Vec<[f64; N]>,
    payload_marker: u64,
    injected_evaluate_error: Option<&'static str>,
    injected_materialize_error: Option<&'static str>,
    trial_domain_entries: u8,
    evaluation_fault: Option<(u16, EvaluationFault)>,
    governed_finite_projection: Option<(u16, GovernedFiniteProjection)>,
    selected_jacobian_rank_deficient: bool,
    hydraulics: HydraulicLifecycleFixture,
}

struct HydraulicLifecycleFixture {
    current_mode: HydraulicCurrentMode,
    completion_mode: HydraulicCompletionMode,
    completion_calls: u8,
    post_update_resets: u8,
    post_update_mode: HydraulicPostUpdateMode,
}

#[derive(Default)]
enum HydraulicCurrentMode {
    #[default]
    Complete,
    PendingInitial,
    PendingCandidates,
}

#[derive(Default)]
enum HydraulicCompletionMode {
    #[default]
    Complete,
    LeavePending,
    ErrorOnCall(u8),
}

#[derive(Default)]
enum HydraulicPostUpdateMode {
    #[default]
    SelectedSideError,
    ResetPending,
}
impl Fixture {
    fn project_governed_finite_operand(&self, evaluation: &mut Evaluation<Payload>, ordinal: u16) {
        if let Some((projection_ordinal, projection)) = self.governed_finite_projection {
            if ordinal == projection_ordinal {
                match projection {
                    GovernedFiniteProjection::TemperatureCoordinate(index, value) => {
                        evaluation.coordinates[index] = value;
                    }
                    GovernedFiniteProjection::WetTemperature(index, value) => {
                        evaluation.wet_temperature_k[index] = Some(value);
                    }
                    GovernedFiniteProjection::Humidity(value) => evaluation.coordinates[13] = value,
                    GovernedFiniteProjection::Hydraulic(index, value) => {
                        if let M1TrustRegionCurrentHydraulics::Complete(values) =
                            &mut evaluation.current_hydraulics
                        {
                            values[index] = value;
                        }
                    }
                    GovernedFiniteProjection::Beta(index, value) => evaluation.beta[index] = value,
                    GovernedFiniteProjection::Ci(index, value) => evaluation.ci_pa[index] = value,
                }
            }
        }
    }
    fn project_evaluation_fault(&self, evaluation: &mut Evaluation<Payload>, ordinal: u16) {
        if let Some((fault_ordinal, fault)) = self.evaluation_fault {
            if ordinal == fault_ordinal {
                match fault {
                    EvaluationFault::RawResidualNaN(index) => {
                        evaluation.raw_residual[index] = f64::NAN;
                    }
                    EvaluationFault::DynamicNormalizerNaN(index) => {
                        evaluation.dynamic_normalizers[index] = f64::NAN;
                    }
                    EvaluationFault::DynamicNormalizerZero(index) => {
                        evaluation.dynamic_normalizers[index] = 0.0;
                    }
                    EvaluationFault::CoordinateNaN(index) => {
                        evaluation.coordinates[index] = f64::NAN;
                    }
                    EvaluationFault::WetTemperatureNaN(index) => {
                        evaluation.wet_temperature_k[index] = Some(f64::NAN);
                    }
                    EvaluationFault::HydraulicNaN(index) => {
                        if let M1TrustRegionCurrentHydraulics::Complete(values) =
                            &mut evaluation.current_hydraulics
                        {
                            values[index] = f64::NAN;
                        }
                    }
                    EvaluationFault::BetaNaN(index) => {
                        evaluation.beta[index] = f64::NAN;
                    }
                    EvaluationFault::CiNaN(index) => {
                        evaluation.ci_pa[index] = f64::NAN;
                    }
                }
            }
        }
    }
    fn new(kind: FixtureKind) -> Self {
        let mut base = [300.0; N];
        let mut scale = [1.0; N];
        let mut lower = [f64::NEG_INFINITY; N];
        let mut upper = [f64::INFINITY; N];
        for i in [0, 1, 2, 6, 7, 8] {
            lower[i] = 263.15;
            upper[i] = 373.15;
        }
        lower[12] = 258.15;
        upper[12] = 373.15;
        lower[13] = 0.0;
        upper[13] = 0.1;
        base[3] = 0.01;
        base[9] = 0.01;
        base[4] = 1000.0;
        base[10] = 1000.0;
        base[5] = 0.0;
        base[11] = 0.0;
        base[13] = 0.01;
        scale[3] = 0.01;
        scale[9] = 0.01;
        scale[4] = 1000.0;
        scale[10] = 1000.0;
        scale[5] = 1e-5;
        scale[11] = 1e-5;
        scale[13] = 0.001;
        lower[5] = 0.0;
        lower[11] = 0.0;
        Self {
            kind,
            base,
            scale,
            lower,
            upper,
            operation_log: Vec::new(),
            evaluator_entries: 0,
            owner_sentinel: 0x4d31_5452_5553_5443,
            materializer_inputs: Vec::new(),
            payload_marker: 0,
            injected_evaluate_error: None,
            injected_materialize_error: None,
            trial_domain_entries: 0,
            evaluation_fault: None,
            governed_finite_projection: None,
            selected_jacobian_rank_deficient: false,
            hydraulics: HydraulicLifecycleFixture {
                current_mode: HydraulicCurrentMode::Complete,
                completion_mode: HydraulicCompletionMode::Complete,
                completion_calls: 0,
                post_update_resets: 0,
                post_update_mode: HydraulicPostUpdateMode::SelectedSideError,
            },
        }
    }
    fn z(&self, x: &[f64; N], i: usize) -> f64 {
        (x[i] - self.base[i]) / self.scale[i]
    }
    fn long_progress_residual(
        &self,
        x: &[f64; N],
        i: usize,
        target: f64,
        perturb_at_183: bool,
    ) -> f64 {
        let z = self.z(x, i);
        if i == 3 {
            z - target
                - if perturb_at_183 && z > 182.0 && z < 184.0 {
                    8.0
                } else {
                    0.0
                }
        } else {
            z
        }
    }
    fn normalizer(&self, x: &[f64; N], i: usize) -> f64 {
        match self.kind {
            FixtureKind::FrozenRawJacobian if i == 0 => 1.0 + self.z(x, 0) / 2.0,
            FixtureKind::FrozenRawJacobian if i == 1 => 0.5,
            FixtureKind::FullNoUpdate
            | FixtureKind::PostUpdateAdmission
            | FixtureKind::ReplacementWitness
            | FixtureKind::ReplacementWitnessThreeExcess
            | FixtureKind::ReplacementWitnessFullDomainInvalid
            | FixtureKind::ReplacementWitnessInterveningDomainInvalid
            | FixtureKind::ReplacementWitnessFullEvaluationError
            | FixtureKind::ReplacementWitnessReplacementEvaluationError
            | FixtureKind::ReplacementWitnessFullIncompleteGoverned
            | FixtureKind::ReplacementWitnessMixedThenComplete
            | FixtureKind::ReplacementWitnessThreeExcessMixedReplacement
            | FixtureKind::ReplacementWitnessExhaustedOrdinaryUpdate
            | FixtureKind::NegativeHumidityExcess => 2f64.powi(-60),
            _ => 1.0,
        }
    }
    fn raw_multiplier(&self) -> f64 {
        if matches!(
            self.kind,
            FixtureKind::FullNoUpdate
                | FixtureKind::PostUpdateAdmission
                | FixtureKind::ReplacementWitness
                | FixtureKind::ReplacementWitnessThreeExcess
                | FixtureKind::ReplacementWitnessFullDomainInvalid
                | FixtureKind::ReplacementWitnessInterveningDomainInvalid
                | FixtureKind::ReplacementWitnessFullEvaluationError
                | FixtureKind::ReplacementWitnessReplacementEvaluationError
                | FixtureKind::ReplacementWitnessFullIncompleteGoverned
                | FixtureKind::ReplacementWitnessMixedThenComplete
                | FixtureKind::ReplacementWitnessThreeExcessMixedReplacement
                | FixtureKind::ReplacementWitnessExhaustedOrdinaryUpdate
                | FixtureKind::NegativeHumidityExcess
        ) {
            2f64.powi(-60)
        } else {
            1.0
        }
    }
    fn residual(&self, x: &[f64; N], i: usize) -> f64 {
        match self.kind {
            FixtureKind::LongProgress {
                target,
                perturb_at_183,
            } => self.long_progress_residual(x, i, target, perturb_at_183),
            _ => self.residual_regular(x, i),
        }
    }
    fn residual_regular(&self, x: &[f64; N], i: usize) -> f64 {
        let z = self.z(x, i);
        match (self.kind, i) {
            (FixtureKind::Ordinary(c), 0) => -2.0 + z + c * z * z,
            (
                FixtureKind::FrozenRawJacobian
                | FixtureKind::PhaseUnchanged
                | FixtureKind::PhaseChanged
                | FixtureKind::PhaseFinalDisagreement
                | FixtureKind::PhasePredictorSelectionError
                | FixtureKind::PhaseFinalSelectionError
                | FixtureKind::PhaseSelectedSideError
                | FixtureKind::PhaseSelectedRetry
                | FixtureKind::PhaseSelectedRetryDrift
                | FixtureKind::PhaseAcceptedUpdateReset
                | FixtureKind::HydraulicPoison(_)
                | FixtureKind::WetNoneNone
                | FixtureKind::WetMixed,
                0,
            ) => -1.2 + z,
            (
                FixtureKind::FrozenRawJacobian
                | FixtureKind::PhaseUnchanged
                | FixtureKind::PhaseChanged
                | FixtureKind::PhaseFinalDisagreement
                | FixtureKind::PhasePredictorSelectionError
                | FixtureKind::PhaseFinalSelectionError
                | FixtureKind::PhaseSelectedSideError
                | FixtureKind::PhaseSelectedRetry
                | FixtureKind::PhaseSelectedRetryDrift
                | FixtureKind::PhaseAcceptedUpdateReset,
                1,
            ) => -1.0 + z,
            (FixtureKind::ConstrainedStationarity, 5) => 2.0 + z,
            (FixtureKind::FullNoUpdate, 0) => z - 2f64.powi(-28),
            (FixtureKind::PostUpdateAdmission, 0) => 2f64.powi(29) * z - 2.0,
            (FixtureKind::PostUpdateAdmission, _) => 2f64.powi(29) * z,
            (FixtureKind::ReplacementWitnessExhaustedOrdinaryUpdate, 0) => {
                // This full-rank linear row is zero along the prescribed
                // trajectory without manufacturing a zero residual/Jacobian
                // inconsistency for other analytic coordinates.
                z / 4.0
            }
            (
                FixtureKind::ReplacementWitness
                | FixtureKind::ReplacementWitnessThreeExcess
                | FixtureKind::ReplacementWitnessFullDomainInvalid
                | FixtureKind::ReplacementWitnessInterveningDomainInvalid
                | FixtureKind::ReplacementWitnessFullEvaluationError
                | FixtureKind::ReplacementWitnessReplacementEvaluationError
                | FixtureKind::ReplacementWitnessFullIncompleteGoverned
                | FixtureKind::ReplacementWitnessMixedThenComplete
                | FixtureKind::ReplacementWitnessThreeExcessMixedReplacement,
                0,
            ) => {
                let magnitude = if matches!(
                    self.kind,
                    FixtureKind::ReplacementWitness
                        | FixtureKind::ReplacementWitnessFullDomainInvalid
                        | FixtureKind::ReplacementWitnessMixedThenComplete
                ) {
                    -27
                } else {
                    -25
                };
                -2f64.powi(magnitude) + z / 4.0
            }
            (
                FixtureKind::ReplacementWitnessThreeExcess
                | FixtureKind::ReplacementWitnessThreeExcessMixedReplacement,
                3,
            ) => -0.5 + z / 4.0 - 2f64.powi(20) * z * z,
            (
                FixtureKind::ReplacementWitness
                | FixtureKind::ReplacementWitnessExhaustedOrdinaryUpdate
                | FixtureKind::ReplacementWitnessFullDomainInvalid
                | FixtureKind::ReplacementWitnessInterveningDomainInvalid
                | FixtureKind::ReplacementWitnessFullEvaluationError
                | FixtureKind::ReplacementWitnessReplacementEvaluationError
                | FixtureKind::ReplacementWitnessFullIncompleteGoverned
                | FixtureKind::ReplacementWitnessMixedThenComplete,
                3,
            ) => -0.5 + z / 4.0 - z * z / 4.0,
            (
                FixtureKind::ReplacementWitness
                | FixtureKind::ReplacementWitnessThreeExcess
                | FixtureKind::ReplacementWitnessThreeExcessMixedReplacement
                | FixtureKind::ReplacementWitnessFullDomainInvalid
                | FixtureKind::ReplacementWitnessInterveningDomainInvalid
                | FixtureKind::ReplacementWitnessFullEvaluationError
                | FixtureKind::ReplacementWitnessReplacementEvaluationError
                | FixtureKind::ReplacementWitnessFullIncompleteGoverned
                | FixtureKind::ReplacementWitnessMixedThenComplete
                | FixtureKind::ReplacementWitnessExhaustedOrdinaryUpdate,
                _,
            ) => z / 4.0,
            (FixtureKind::NegativeHumidityExcess, 13) => z + 2e-9,
            _ => z,
        }
    }
    fn record(&mut self, operation: FixtureOperation) {
        self.operation_log.push(operation);
    }

    fn analytic_hydraulic_operands(&self) -> [f64; 8] {
        std::array::from_fn(|index| {
            if matches!(self.kind, FixtureKind::HydraulicPoison(poison) if poison == index)
                && self.evaluator_entries == 2
            {
                f64::NAN
            } else {
                0.0
            }
        })
    }

    fn exhausted_witness_wet_temperature_k(&self, x: &[f64; N]) -> [f64; 2] {
        let z = self.z(x, 3);
        // The governed operand is a declared coordinate function.  It is
        // nonzero at the full and first-replacement candidates, but equal at
        // z=1/4 and z=3/4 so the accepted new base has a fresh admissible
        // no-update witness.
        let delta = 2f64.powi(-20) * (z - 0.25) * (z - 0.75) * (1.0 + z);
        [300.0 + delta, 300.0]
    }
}
impl Evaluator for Fixture {
    type Error = &'static str;
    type Payload = Payload;
    type Phase = FixturePhase;
    fn initial_coordinates(&self) -> [f64; N] {
        self.base
    }
    fn coordinate_scales(&self) -> [f64; N] {
        self.scale
    }
    fn coordinate_bounds(&self) -> [[f64; N]; 2] {
        [self.lower, self.upper]
    }
    fn evaluate(&mut self, x: [f64; N]) -> Result<Evaluation<Payload>, Self::Error> {
        self.evaluator_entries += 1;
        self.record(FixtureOperation::Evaluate(x));
        if matches!(
            self.kind,
            FixtureKind::ReplacementWitnessFullEvaluationError
        ) && self.evaluator_entries == 2
        {
            return Err("full replacement fixture evaluation error");
        }
        if matches!(
            self.kind,
            FixtureKind::ReplacementWitnessReplacementEvaluationError
        ) && self.evaluator_entries == 3
        {
            return Err("replacement fixture evaluation error");
        }
        if let Some(error) = self.injected_evaluate_error {
            return Err(error);
        }
        for i in 0..N {
            if !x[i].is_finite()
                || x[i] < self.lower[i]
                || x[i] > self.upper[i]
                || !self.normalizer(&x, i).is_finite()
                || self.normalizer(&x, i) <= 0.0
            {
                return Err("analytic fixture domain");
            }
        }
        let mut evaluation = Evaluation {
            coordinates: x,
            raw_residual: std::array::from_fn(|i| self.raw_multiplier() * self.residual(&x, i)),
            dynamic_normalizers: std::array::from_fn(|i| self.normalizer(&x, i)),
            wet_temperature_k: if matches!(
                self.kind,
                FixtureKind::ReplacementWitnessFullIncompleteGoverned
            ) && self.evaluator_entries == 2
            {
                [Some(f64::NAN); 2]
            } else if matches!(
                self.kind,
                FixtureKind::ReplacementWitnessExhaustedOrdinaryUpdate
            ) {
                self.exhausted_witness_wet_temperature_k(&x).map(Some)
            } else if matches!(self.kind, FixtureKind::PhaseAcceptedUpdateReset)
                && !same_coordinate_bits(x, self.base)
            {
                [Some(310.0), Some(300.0)]
            } else if matches!(self.kind, FixtureKind::WetNoneNone) {
                [None, None]
            } else if (matches!(self.kind, FixtureKind::WetMixed) && self.evaluator_entries == 2)
                || (matches!(self.kind, FixtureKind::ReplacementWitnessMixedThenComplete)
                    && x[0] - self.base[0] > 2f64.powi(-27))
                || (matches!(
                    self.kind,
                    FixtureKind::ReplacementWitnessThreeExcessMixedReplacement
                ) && x[0] - self.base[0] > 2f64.powi(-27)
                    && x[0] - self.base[0] < 2f64.powi(-25))
            {
                [Some(300.0), None]
            } else {
                [Some(300.0); 2]
            },
            current_hydraulics: if match self.hydraulics.current_mode {
                HydraulicCurrentMode::Complete => false,
                HydraulicCurrentMode::PendingInitial => self.evaluator_entries == 1,
                HydraulicCurrentMode::PendingCandidates => self.evaluator_entries > 1,
            } {
                M1TrustRegionCurrentHydraulics::Pending
            } else {
                M1TrustRegionCurrentHydraulics::Complete(self.analytic_hydraulic_operands())
            },
            beta: [0.5; 4],
            ci_pa: [20.0; 4],
            payload: Payload {
                materialized_coordinate_bits: [0; N],
                marker: 0,
            },
        };
        self.project_evaluation_fault(&mut evaluation, self.evaluator_entries);
        self.project_governed_finite_operand(&mut evaluation, self.evaluator_entries);
        Ok(evaluation)
    }
    fn pre_evaluator_trial_domain(&mut self, coordinates: [f64; N]) -> Result<bool, Self::Error> {
        self.record(FixtureOperation::PreEvaluatorTrialDomain(coordinates));
        self.trial_domain_entries += 1;
        let coordinate_domain = coordinates
            .iter()
            .enumerate()
            .all(|(i, value)| *value >= self.lower[i] && *value <= self.upper[i]);
        let prescribed_invalid = match self.kind {
            FixtureKind::ReplacementWitnessFullDomainInvalid => self.trial_domain_entries == 1,
            FixtureKind::ReplacementWitnessInterveningDomainInvalid => {
                self.trial_domain_entries == 2
            }
            FixtureKind::PhaseSelectedRetry | FixtureKind::PhaseSelectedRetryDrift => {
                coordinates[0] > self.base[0] + 0.3
            }
            _ => false,
        };
        Ok(coordinate_domain && !prescribed_invalid)
    }
    fn raw_jacobian(
        &mut self,
        e: &Evaluation<Payload>,
        selected_phase: Option<&FixturePhase>,
        predictor_direction: [f64; N],
    ) -> Result<[[f64; N]; N], Self::Error> {
        self.record(FixtureOperation::RawJacobian {
            base: e.coordinates,
            selected_phase: selected_phase.map(|phase| phase.code()),
            predictor_direction,
        });
        if selected_phase.is_some() && self.selected_jacobian_rank_deficient {
            return Ok([[0.0; N]; N]);
        }
        Ok(std::array::from_fn(|i| {
            std::array::from_fn(|j| {
                if i == j {
                    self.raw_multiplier()
                        * match self.kind {
                            FixtureKind::Ordinary(c) if i == 0 => {
                                (1.0 + 2.0 * c * self.z(&e.coordinates, 0)) / self.scale[0]
                            }
                            FixtureKind::PostUpdateAdmission => 2f64.powi(29) / self.scale[i],
                            FixtureKind::ReplacementWitness
                            | FixtureKind::ReplacementWitnessThreeExcess
                            | FixtureKind::ReplacementWitnessFullDomainInvalid
                            | FixtureKind::ReplacementWitnessInterveningDomainInvalid
                            | FixtureKind::ReplacementWitnessFullEvaluationError
                            | FixtureKind::ReplacementWitnessReplacementEvaluationError
                            | FixtureKind::ReplacementWitnessFullIncompleteGoverned
                            | FixtureKind::ReplacementWitnessMixedThenComplete
                            | FixtureKind::ReplacementWitnessThreeExcessMixedReplacement
                            | FixtureKind::ReplacementWitnessExhaustedOrdinaryUpdate
                                if i == 3 =>
                            {
                                (0.25 - self.z(&e.coordinates, 3) / 2.0) / self.scale[i]
                            }
                            FixtureKind::ReplacementWitness
                            | FixtureKind::ReplacementWitnessThreeExcess
                            | FixtureKind::ReplacementWitnessFullDomainInvalid
                            | FixtureKind::ReplacementWitnessInterveningDomainInvalid
                            | FixtureKind::ReplacementWitnessFullEvaluationError
                            | FixtureKind::ReplacementWitnessReplacementEvaluationError
                            | FixtureKind::ReplacementWitnessFullIncompleteGoverned
                            | FixtureKind::ReplacementWitnessMixedThenComplete
                            | FixtureKind::ReplacementWitnessThreeExcessMixedReplacement
                            | FixtureKind::ReplacementWitnessExhaustedOrdinaryUpdate => {
                                0.25 / self.scale[i]
                            }
                            _ if selected_phase == Some(&FixturePhase::Selected) => {
                                2.0 / self.scale[i]
                            }
                            _ => 1.0 / self.scale[i],
                        }
                } else {
                    0.0
                }
            })
        }))
    }
    fn complete_current_hydraulics(
        &mut self,
        evaluation: &mut Evaluation<Payload>,
    ) -> Result<(), Self::Error> {
        if matches!(
            evaluation.current_hydraulics,
            M1TrustRegionCurrentHydraulics::Pending
        ) {
            self.hydraulics.completion_calls += 1;
            self.record(FixtureOperation::CompleteCurrentHydraulics(
                evaluation.coordinates,
            ));
            match self.hydraulics.completion_mode {
                HydraulicCompletionMode::ErrorOnCall(call)
                    if call == self.hydraulics.completion_calls =>
                {
                    return Err("pending hydraulic completion error");
                }
                HydraulicCompletionMode::LeavePending => return Ok(()),
                HydraulicCompletionMode::Complete | HydraulicCompletionMode::ErrorOnCall(_) => {}
            }
            evaluation.current_hydraulics =
                M1TrustRegionCurrentHydraulics::Complete(self.analytic_hydraulic_operands());
        }
        Ok(())
    }
    fn reset_current_hydraulics_after_nonroot_post_update(
        &mut self,
        evaluation: &mut Evaluation<Payload>,
    ) {
        self.hydraulics.post_update_resets += 1;
        if matches!(
            self.hydraulics.post_update_mode,
            HydraulicPostUpdateMode::ResetPending
        ) {
            evaluation.current_hydraulics = M1TrustRegionCurrentHydraulics::Pending;
        }
    }
    fn select_phase(
        &mut self,
        e: &Evaluation<Payload>,
        physical_direction: [f64; N],
    ) -> Result<FixturePhase, Self::Error> {
        self.record(FixtureOperation::SelectPhase {
            base: e.coordinates,
            step: physical_direction,
        });
        let selections = self
            .operation_log
            .iter()
            .filter(|operation| matches!(operation, FixtureOperation::SelectPhase { .. }))
            .count();
        match self.kind {
            FixtureKind::PhasePredictorSelectionError if selections == 2 => {
                Err("PhaseActiveSetUnrepresentable")
            }
            FixtureKind::PhaseFinalSelectionError if selections == 3 => {
                Err("PhaseActiveSetUnrepresentable")
            }
            FixtureKind::PhaseFinalDisagreement if selections == 3 => Ok(FixturePhase::Natural),
            FixtureKind::PhaseSelectedRetryDrift if selections == 4 => Ok(FixturePhase::Natural),
            FixtureKind::PhaseChanged
            | FixtureKind::PhaseFinalDisagreement
            | FixtureKind::PhaseFinalSelectionError
            | FixtureKind::PhaseSelectedSideError
            | FixtureKind::PhaseSelectedRetry
            | FixtureKind::PhaseSelectedRetryDrift
            | FixtureKind::PhaseAcceptedUpdateReset
                if physical_direction[0] > 0.0 =>
            {
                Ok(FixturePhase::Selected)
            }
            _ => Ok(FixturePhase::Natural),
        }
    }
    fn phase_active_set_inconsistent(
        &mut self,
        _current: &Evaluation<Payload>,
        selected_phase: &FixturePhase,
        final_phase: &FixturePhase,
    ) -> Self::Error {
        self.record(FixtureOperation::PhaseActiveSetInconsistent {
            selected_phase: selected_phase.code(),
            final_phase: final_phase.code(),
        });
        "PhaseActiveSetInconsistent"
    }
    fn selected_side_probe(
        &mut self,
        current: &Evaluation<Payload>,
        phase: &FixturePhase,
        direction: [f64; N],
    ) -> Result<(), Self::Error> {
        self.record(FixtureOperation::SelectedSideProbe {
            base: current.coordinates,
            phase: phase.code(),
            direction,
        });
        let probes = self
            .operation_log
            .iter()
            .filter(|operation| matches!(operation, FixtureOperation::SelectedSideProbe { .. }))
            .count();
        if matches!(self.kind, FixtureKind::PhaseSelectedSideError)
            || matches!(self.kind, FixtureKind::PhaseSelectedRetry) && probes == 2
            || matches!(self.kind, FixtureKind::PhaseAcceptedUpdateReset)
                && matches!(
                    self.hydraulics.post_update_mode,
                    HydraulicPostUpdateMode::SelectedSideError
                )
                && !same_coordinate_bits(current.coordinates, self.base)
        {
            Err("selected-side owning error")
        } else {
            Ok(())
        }
    }
    fn materialize(&mut self, e: &Evaluation<Payload>) -> Result<Payload, Self::Error> {
        self.record(FixtureOperation::Materialize(e.coordinates));
        if let Some(error) = self.injected_materialize_error {
            return Err(error);
        }
        self.materializer_inputs.push(e.coordinates);
        self.payload_marker += 1;
        Ok(Payload {
            materialized_coordinate_bits: e.coordinates.map(f64::to_bits),
            marker: self.payload_marker,
        })
    }
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= DERIVED_TOLERANCE);
}

fn operation_kinds(log: &[FixtureOperation]) -> Vec<u8> {
    log.iter()
        .map(|operation| match operation {
            FixtureOperation::Evaluate(_) => 0,
            FixtureOperation::SelectPhase { .. } => 1,
            FixtureOperation::RawJacobian { .. } => 2,
            FixtureOperation::SelectedSideProbe { .. } => 3,
            FixtureOperation::PreEvaluatorTrialDomain(_) => 4,
            FixtureOperation::Materialize(_) => 5,
            FixtureOperation::PhaseActiveSetInconsistent { .. } => 6,
            FixtureOperation::CompleteCurrentHydraulics(_) => 7,
        })
        .collect()
}

fn long_progress_kinds(accepted_before_terminal: usize, terminal_materializes: bool) -> Vec<u8> {
    let mut kinds = vec![0, 1, 2];
    for _ in 0..accepted_before_terminal {
        kinds.extend([1, 3, 4, 0, 1, 2]);
    }
    kinds.extend([1, 3, 4, 0]);
    if terminal_materializes {
        kinds.push(5);
    }
    kinds
}
fn long_progress_rejection_kinds() -> Vec<u8> {
    let mut kinds = vec![0, 1, 2];
    for proposal in 0..30 {
        kinds.extend([1, 3, 4, 0]);
        if proposal == 29 {
            kinds.push(5);
        } else if proposal != 24 {
            kinds.extend([1, 2]);
        }
    }
    kinds
}

fn only_lower_five(mask: [bool; N]) -> bool {
    mask.iter()
        .enumerate()
        .all(|(index, active)| *active == (index == 5))
}

fn stationarity_log(base: [f64; N], proposals: usize) -> Vec<FixtureOperation> {
    let zero = [0.0; N];
    let mut log = vec![
        FixtureOperation::Evaluate(base),
        FixtureOperation::SelectPhase { base, step: zero },
        FixtureOperation::RawJacobian {
            base,
            selected_phase: None,
            predictor_direction: zero,
        },
    ];
    for _ in 0..proposals {
        log.push(FixtureOperation::SelectPhase { base, step: zero });
        log.push(FixtureOperation::SelectedSideProbe {
            base,
            phase: 0,
            direction: zero,
        });
    }
    log
}

fn same_coordinate_bits(left: [f64; N], right: [f64; N]) -> bool {
    left.map(f64::to_bits) == right.map(f64::to_bits)
}

fn stationarity_radius(exponent: usize) -> f64 {
    4f64.powi(-i32::try_from(exponent).unwrap())
}

fn assert_stationarity_proposal(
    proposal: &super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition,
    base: [f64; N],
    index: usize,
) {
    assert_eq!(
        proposal.radius_before.to_bits(),
        stationarity_radius(index).to_bits()
    );
    assert_eq!(
        proposal.next_radius.to_bits(),
        stationarity_radius(index + 1).to_bits()
    );
    assert_eq!(proposal.disposition, super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::NoPredictedReductionAndShrunk);
    assert!(same_coordinate_bits(proposal.base_before, base));
    assert!(same_coordinate_bits(proposal.base_after, base));
    assert_eq!(proposal.stage1_observation.entered_svd_factorizations, 2);
    assert!(
        proposal
            .stage1_observation
            .initial_lower_mask
            .iter()
            .all(|active| !active)
    );
    assert!(
        proposal
            .stage1_observation
            .initial_upper_mask
            .iter()
            .all(|active| !active)
    );
    assert!(
        proposal
            .stage1_observation
            .initial_free_mask
            .iter()
            .all(|active| *active)
    );
    assert_eq!(proposal.stage1_observation.event_trace.len(), 2);
    assert!(only_lower_five(
        proposal.stage1_observation.event_trace[0].lower_mask
    ));
    assert!(
        proposal.stage1_observation.event_trace[0]
            .upper_mask
            .iter()
            .all(|active| !active)
    );
    assert!(!proposal.stage1_observation.event_trace[0].free_mask[5]);
    assert_eq!(
        proposal.stage1_observation.event_trace[1].action,
        super::M1TrustRegionKktAction::Return
    );
    assert!(only_lower_five(
        proposal.stage1_observation.event_trace[1].lower_mask
    ));
    assert!(
        proposal.stage1_observation.event_trace[1]
            .upper_mask
            .iter()
            .all(|active| !active)
    );
    assert_eq!(
        proposal.stage1_observation.event_trace[1].free_mask,
        std::array::from_fn(|index| index != 5)
    );
    assert!(only_lower_five(proposal.stage1_observation.active_lower));
    assert!(
        proposal
            .stage1_observation
            .active_upper
            .iter()
            .all(|active| !active)
    );
}

fn full_no_update_log(
    base: [f64; N],
    direction: [f64; N],
    candidate: [f64; N],
) -> Vec<FixtureOperation> {
    vec![
        FixtureOperation::Evaluate(base),
        FixtureOperation::SelectPhase {
            base,
            step: [0.0; N],
        },
        FixtureOperation::RawJacobian {
            base,
            selected_phase: None,
            predictor_direction: [0.0; N],
        },
        FixtureOperation::SelectPhase {
            base,
            step: direction,
        },
        FixtureOperation::SelectedSideProbe {
            base,
            phase: 0,
            direction,
        },
        FixtureOperation::PreEvaluatorTrialDomain(candidate),
        FixtureOperation::Evaluate(candidate),
        FixtureOperation::Materialize(base),
    ]
}

fn assert_full_no_update_proposal(
    proposal: &super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition,
    base: [f64; N],
    candidate: [f64; N],
) {
    assert_eq!(
        proposal.disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::RootAdmitted
    );
    assert!(!proposal.installed);
    assert!(same_coordinate_bits(proposal.base_before, base));
    assert!(same_coordinate_bits(proposal.base_after, base));
    assert!(same_coordinate_bits(
        proposal.stage1_observation.base_coordinates,
        base
    ));
    assert!(same_coordinate_bits(
        proposal.stage1_observation.candidate_coordinates,
        candidate
    ));
    assert_eq!(
        proposal.stage1_observation.scaled_step.map(f64::to_bits),
        proposal.scaled_step.map(f64::to_bits)
    );
    assert_eq!(proposal.scaled_step[0].to_bits(), 2f64.powi(-28).to_bits());
    assert!(
        proposal.scaled_step[1..]
            .iter()
            .all(|value| value.to_bits() == 0)
    );
    assert_eq!(proposal.lambda.to_bits(), 0.0f64.to_bits());
    assert_eq!(proposal.stage1_observation.entered_svd_factorizations, 1);
    assert_eq!(
        proposal.predicted_reduction.to_bits(),
        2f64.powi(-57).to_bits()
    );
    assert_eq!(
        proposal.actual_reduction.unwrap().to_bits(),
        2f64.powi(-57).to_bits()
    );
    assert_eq!(proposal.rho.unwrap().to_bits(), 1.0f64.to_bits());
    assert_eq!(proposal.moving_merit.unwrap().to_bits(), 0.0f64.to_bits());
    assert_eq!(
        proposal.dynamic_max_normalized_residual.unwrap().to_bits(),
        0.0f64.to_bits()
    );
}

fn replacement_log(
    base: [f64; N],
    first_direction: [f64; N],
    first: [f64; N],
    second_direction: [f64; N],
    second: [f64; N],
) -> Vec<FixtureOperation> {
    vec![
        FixtureOperation::Evaluate(base),
        FixtureOperation::SelectPhase {
            base,
            step: [0.0; N],
        },
        FixtureOperation::RawJacobian {
            base,
            selected_phase: None,
            predictor_direction: [0.0; N],
        },
        FixtureOperation::SelectPhase {
            base,
            step: first_direction,
        },
        FixtureOperation::SelectedSideProbe {
            base,
            phase: 0,
            direction: first_direction,
        },
        FixtureOperation::PreEvaluatorTrialDomain(first),
        FixtureOperation::Evaluate(first),
        FixtureOperation::SelectPhase {
            base,
            step: second_direction,
        },
        FixtureOperation::SelectedSideProbe {
            base,
            phase: 0,
            direction: second_direction,
        },
        FixtureOperation::PreEvaluatorTrialDomain(second),
        FixtureOperation::Evaluate(second),
        FixtureOperation::Materialize(base),
    ]
}

fn governed_excess_steps() -> [[f64; N]; 11] {
    const BITS: [(u64, u64); 11] = [
        (0x3e6f_ffff_ffff_ff00, 0x3fef_ffff_ffff_ff00),
        (0x3e4f_ffff_ffff_ffc0, 0x3fcf_ffff_ffff_ffc0),
        (0x3e2f_ffff_ffff_ffd0, 0x3faf_ffff_ffff_ffd0),
        (0x3e0f_ffff_ffff_ffd0, 0x3f8f_ffff_ffff_ffd0),
        (0x3def_ffff_ffff_ffd0, 0x3f6f_ffff_ffff_ffd0),
        (0x3dcf_ffff_ffff_ffd0, 0x3f4f_ffff_ffff_ffd0),
        (0x3daf_ffff_ffff_ffd0, 0x3f2f_ffff_ffff_ffd0),
        (0x3d8f_ffff_ffff_ffd0, 0x3f0f_ffff_ffff_ffd0),
        (0x3d6f_ffff_ffff_ffd0, 0x3eef_ffff_ffff_ffd0),
        (0x3d4f_ffff_ffff_ffd0, 0x3ecf_ffff_ffff_ffd0),
        (0x3d2f_ffff_ffff_ffd0, 0x3eaf_ffff_ffff_ffd0),
    ];
    BITS.map(|(temperature, moisture)| {
        let mut step = [0.0; N];
        step[0] = f64::from_bits(temperature);
        step[3] = f64::from_bits(moisture);
        step
    })
}

fn exhausted_witness_steps() -> [[f64; N]; 2] {
    [
        std::array::from_fn(|index| if index == 3 { 1.0 } else { 0.0 }),
        std::array::from_fn(|index| if index == 3 { 0.25 } else { 0.0 }),
    ]
}

fn exhausted_witness_reset_step() -> [f64; N] {
    std::array::from_fn(|index| {
        if index == 3 {
            // The shared 48-bisection feasible endpoint is two ulps below
            // the 0.5 radius; this is an independently frozen oracle.
            f64::from_bits(0x3fdf_ffff_ffff_fffe)
        } else {
            0.0
        }
    })
}

fn exhausted_witness_wet_temperature(
    coordinates: [f64; N],
    base: [f64; N],
    scale: [f64; N],
) -> f64 {
    let z = (coordinates[3] - base[3]) / scale[3];
    300.0 + 2f64.powi(-20) * (z - 0.25) * (z - 0.75) * (1.0 + z)
}

fn exhausted_witness_residual(coordinates: [f64; N], base: [f64; N], scale: [f64; N]) -> f64 {
    let z = (coordinates[3] - base[3]) / scale[3];
    -0.5 + z / 4.0 - z * z / 4.0
}

fn exhausted_witness_merit(
    base_coordinates: [f64; N],
    candidate_coordinates: [f64; N],
    scaled_step: [f64; N],
    initial_base: [f64; N],
    scale: [f64; N],
) -> (f64, f64, f64, f64) {
    let base_residual = exhausted_witness_residual(base_coordinates, initial_base, scale);
    let candidate_residual = exhausted_witness_residual(candidate_coordinates, initial_base, scale);
    let base_z = (base_coordinates[3] - initial_base[3]) / scale[3];
    let linear = base_residual + (0.25 - base_z / 2.0) * scaled_step[3];
    let predicted = 0.5 * (base_residual * base_residual - linear * linear);
    let actual = 0.5 * (base_residual * base_residual - candidate_residual * candidate_residual);
    let rho = actual / predicted;
    (predicted, actual, rho, candidate_residual.abs())
}

fn assert_exhausted_witness_full_rank(
    proposal: &super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition,
    base_coordinates: [f64; N],
    initial_base: [f64; N],
    scale: [f64; N],
) {
    let base_z = (base_coordinates[3] - initial_base[3]) / scale[3];
    let normalizer = 2f64.powi(-60);
    let raw_multiplier = 2f64.powi(-60);
    for row in 0..N {
        for column in 0..N {
            let expected = if row == column {
                let derivative = if row == 3 { 0.25 - base_z / 2.0 } else { 0.25 };
                // Mirror the source arithmetic order: weight * raw_jacobian,
                // followed by the coordinate scale.
                let raw_jacobian = raw_multiplier * (derivative / scale[row]);
                let weighted_jacobian = (1.0 / normalizer) * raw_jacobian;
                weighted_jacobian * scale[column]
            } else {
                0.0
            };
            if row == column {
                assert_ne!(expected.to_bits(), 0.0f64.to_bits());
            }
            assert_eq!(
                proposal.weighted_jacobian[row][column].to_bits(),
                expected.to_bits()
            );
        }
    }
}

fn positive_replacement_steps() -> [[f64; N]; 2] {
    [
        std::array::from_fn(|index| match index {
            0 => 2f64.powi(-26),
            3 => 1.0,
            _ => 0.0,
        }),
        std::array::from_fn(|index| match index {
            0 => 2f64.powi(-28),
            3 => 0.25,
            _ => 0.0,
        }),
    ]
}

fn governed_excess_log(
    base: [f64; N],
    scale: [f64; N],
    steps: &[[f64; N]; 11],
) -> Vec<FixtureOperation> {
    let zero = [0.0; N];
    let mut log = vec![
        FixtureOperation::Evaluate(base),
        FixtureOperation::SelectPhase { base, step: zero },
        FixtureOperation::RawJacobian {
            base,
            selected_phase: None,
            predictor_direction: zero,
        },
    ];
    for &step in steps {
        let direction = std::array::from_fn(|index| scale[index] * step[index]);
        let candidate = std::array::from_fn(|index| base[index] + direction[index]);
        log.extend([
            FixtureOperation::SelectPhase {
                base,
                step: direction,
            },
            FixtureOperation::SelectedSideProbe {
                base,
                phase: 0,
                direction,
            },
            FixtureOperation::PreEvaluatorTrialDomain(candidate),
            FixtureOperation::Evaluate(candidate),
        ]);
    }
    log
}

fn witness_attempt_log(
    base: [f64; N],
    scale: [f64; N],
    steps: &[[f64; N]],
    evaluated: &[bool],
    materialize: bool,
) -> Vec<FixtureOperation> {
    assert_eq!(steps.len(), evaluated.len());
    let zero = [0.0; N];
    let mut log = vec![
        FixtureOperation::Evaluate(base),
        FixtureOperation::SelectPhase { base, step: zero },
        FixtureOperation::RawJacobian {
            base,
            selected_phase: None,
            predictor_direction: zero,
        },
    ];
    for (step, entered_evaluator) in steps.iter().zip(evaluated) {
        let direction = std::array::from_fn(|index| scale[index] * step[index]);
        let candidate = std::array::from_fn(|index| base[index] + direction[index]);
        log.extend([
            FixtureOperation::SelectPhase {
                base,
                step: direction,
            },
            FixtureOperation::SelectedSideProbe {
                base,
                phase: 0,
                direction,
            },
            FixtureOperation::PreEvaluatorTrialDomain(candidate),
        ]);
        if *entered_evaluator {
            log.push(FixtureOperation::Evaluate(candidate));
        }
    }
    if materialize {
        log.push(FixtureOperation::Materialize(base));
    }
    log
}

fn assert_replacement_trace(
    trace: &[super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition],
    base: [f64; N],
    first_step: [f64; N],
    second_step: [f64; N],
) {
    let first = &trace[0];
    let second = &trace[1];
    assert_eq!(first.radius_before.to_bits(), 1.0f64.to_bits());
    assert_eq!(first.next_radius.to_bits(), 0.25f64.to_bits());
    assert_eq!(first.scaled_step[0].to_bits(), 2f64.powi(-26).to_bits());
    assert_eq!(first.scaled_step[3].to_bits(), 1.0f64.to_bits());
    assert!(same_coordinate_bits(first.scaled_step, first_step));
    assert!(!first.installed);
    assert!(first.rho.unwrap() < 0.1);
    assert!(same_coordinate_bits(first.base_before, base));
    assert!(same_coordinate_bits(first.base_after, base));
    assert_eq!(second.scaled_step[0].to_bits(), 2f64.powi(-28).to_bits());
    assert_eq!(second.scaled_step[3].to_bits(), 0.25f64.to_bits());
    assert!(same_coordinate_bits(second.scaled_step, second_step));
    assert!(!second.installed);
    assert_eq!(second.radius_before.to_bits(), 0.25f64.to_bits());
    assert_eq!(second.next_radius.to_bits(), 0.25f64.to_bits());
    assert!(same_coordinate_bits(second.base_before, base));
    assert!(same_coordinate_bits(second.base_after, base));
    close(second.rho.unwrap(), 0.7625);
    assert_eq!(first.disposition, super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk);
    assert_eq!(
        second.disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::RootAdmitted
    );
}

fn ordinary_log(base: [f64; N], direction: [f64; N], candidate: [f64; N]) -> Vec<FixtureOperation> {
    vec![
        FixtureOperation::Evaluate(base),
        FixtureOperation::SelectPhase {
            base,
            step: [0.0; N],
        },
        FixtureOperation::RawJacobian {
            base,
            selected_phase: None,
            predictor_direction: [0.0; N],
        },
        FixtureOperation::SelectPhase {
            base,
            step: direction,
        },
        FixtureOperation::SelectedSideProbe {
            base,
            phase: 0,
            direction,
        },
        FixtureOperation::PreEvaluatorTrialDomain(candidate),
        FixtureOperation::Evaluate(candidate),
    ]
}
#[test]
fn m1_stage2_ordinary_four_radius_cases_use_actual_state() {
    for (c, ared, rho, install, radius) in [
        (0.0, 1.5, 1.0, true, 2.0),
        (-0.5, 0.875, 7.0 / 12.0, true, 1.0),
        (-0.875, 31.0 / 128.0, 31.0 / 192.0, true, 0.25),
        (-1.0, 0.0, 0.0, false, 0.25),
    ] as [(f64, f64, f64, bool, f64); 4]
    {
        let mut f = Fixture::new(FixtureKind::Ordinary(c));
        let base = f.base;
        let t = super::m1_trust_region_controller_transition_for_test(&mut f).unwrap();
        let mut candidate = base;
        candidate[0] += 1.0;
        let physical_direction = std::array::from_fn(|i| f.scale[i] * t.scaled_step[i]);
        close(t.scaled_step[0], 1.0);
        close(t.lambda, 1.0);
        close(t.predicted_reduction, 1.5);
        close(t.actual_reduction.unwrap(), ared);
        close(t.rho.unwrap(), rho);
        assert_eq!(t.installed, install);
        assert_eq!(t.base_before.map(f64::to_bits), base.map(f64::to_bits));
        assert_eq!(t.radius_before.to_bits(), 1.0f64.to_bits());
        assert_eq!(
            t.disposition,
            if install {
                super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::AcceptedUpdate
            } else {
                super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk
            }
        );
        assert_eq!(t.next_radius.to_bits(), radius.to_bits());
        assert_eq!(t.entered_materializations, 0);
        assert_eq!(
            t.stage1_observation.base_coordinates.map(f64::to_bits),
            base.map(f64::to_bits)
        );
        assert_eq!(
            t.stage1_observation.candidate_coordinates.map(f64::to_bits),
            candidate.map(f64::to_bits)
        );
        assert_eq!(
            t.stage1_observation.scaled_step.map(f64::to_bits),
            t.scaled_step.map(f64::to_bits)
        );
        assert_eq!(t.stage1_observation.lambda.to_bits(), t.lambda.to_bits());
        assert!(
            t.stage1_observation
                .initial_free_mask
                .iter()
                .all(|free| *free)
        );
        assert!(
            t.stage1_observation
                .active_lower
                .iter()
                .all(|active| !active)
        );
        assert!(
            t.stage1_observation
                .active_upper
                .iter()
                .all(|active| !active)
        );
        assert_eq!(t.stage1_observation.entered_svd_factorizations, 1);
        assert_eq!(
            f.operation_log,
            ordinary_log(base, physical_direction, candidate)
        );
        assert_eq!(f.evaluator_entries, 2);
        assert!(f.materializer_inputs.is_empty());
        assert_eq!(
            t.base_after.map(f64::to_bits),
            if install {
                candidate.map(f64::to_bits)
            } else {
                base.map(f64::to_bits)
            }
        );
    }
}
#[test]
fn m1_stage2_raw_jacobian_and_frozen_merit_are_actual() {
    let mut f = Fixture::new(FixtureKind::FrozenRawJacobian);
    let e = f.evaluate(f.base).unwrap();
    assert_eq!(e.raw_residual[0].to_bits(), (-1.2f64).to_bits());
    assert_eq!(
        f.raw_jacobian(&e, None, [0.0; N]).unwrap()[0][0].to_bits(),
        1.0f64.to_bits()
    );
    f.operation_log.clear();
    f.evaluator_entries = 0;
    let t = super::m1_trust_region_controller_transition_for_test(&mut f).unwrap();
    close(t.scaled_step[0], 0.6);
    close(t.scaled_step[1], 0.8);
    close(t.weighted_jacobian[0][0], 1.0);
    close(t.weighted_jacobian[1][1], 2.0);
    close(t.predicted_reduction, 123.0 / 50.0);
    close(t.actual_reduction.unwrap(), 123.0 / 50.0);
    close(t.rho.unwrap(), 1.0);
    close(t.moving_merit.unwrap(), 788.0 / 4225.0);
    close(t.dynamic_max_normalized_residual.unwrap(), 6.0 / 13.0);
    assert!(t.installed);
    assert_eq!(t.next_radius.to_bits(), 2.0f64.to_bits());
    let mut candidate = f.base;
    candidate[0] += 0.6;
    candidate[1] += 0.8;
    let physical_direction = std::array::from_fn(|i| f.scale[i] * t.scaled_step[i]);
    assert_eq!(f.evaluator_entries, 2);
    assert_eq!(
        f.operation_log,
        vec![
            FixtureOperation::Evaluate(f.base),
            FixtureOperation::SelectPhase {
                base: f.base,
                step: [0.0; N]
            },
            FixtureOperation::RawJacobian {
                base: f.base,
                selected_phase: None,
                predictor_direction: [0.0; N],
            },
            FixtureOperation::SelectPhase {
                base: f.base,
                step: physical_direction
            },
            FixtureOperation::SelectedSideProbe {
                base: f.base,
                phase: 0,
                direction: physical_direction
            },
            FixtureOperation::PreEvaluatorTrialDomain(candidate),
            FixtureOperation::Evaluate(candidate),
        ]
    );
    close(t.stage1_observation.scaled_step[0], 0.6);
    close(t.stage1_observation.scaled_step[1], 0.8);
    assert!(
        t.stage1_observation.scaled_step[2..]
            .iter()
            .all(|value| value.abs() <= DERIVED_TOLERANCE)
    );
    assert_eq!(
        t.stage1_observation.scaled_step.map(f64::to_bits),
        t.scaled_step.map(f64::to_bits)
    );
    assert_eq!(t.base_before.map(f64::to_bits), f.base.map(f64::to_bits));
    assert_eq!(t.base_after.map(f64::to_bits), candidate.map(f64::to_bits));
}

fn phase_fixture_log(
    base: [f64; N],
    predictor: [f64; N],
    selected: Option<[f64; N]>,
    final_direction: [f64; N],
    final_probe: bool,
    inconsistent: bool,
) -> Vec<FixtureOperation> {
    let zero = [0.0; N];
    let mut log = vec![
        FixtureOperation::Evaluate(base),
        FixtureOperation::SelectPhase { base, step: zero },
        FixtureOperation::RawJacobian {
            base,
            selected_phase: None,
            predictor_direction: zero,
        },
        FixtureOperation::SelectPhase {
            base,
            step: predictor,
        },
    ];
    if let Some(selected_direction) = selected {
        log.extend([
            FixtureOperation::RawJacobian {
                base,
                selected_phase: Some(1),
                predictor_direction: predictor,
            },
            FixtureOperation::SelectPhase {
                base,
                step: selected_direction,
            },
        ]);
    }
    if final_probe {
        log.push(FixtureOperation::SelectedSideProbe {
            base,
            phase: u8::from(selected.is_some()),
            direction: final_direction,
        });
    }
    if inconsistent {
        log.push(FixtureOperation::PhaseActiveSetInconsistent {
            selected_phase: 1,
            final_phase: 0,
        });
    }
    log
}

fn phase_predictor() -> [f64; N] {
    std::array::from_fn(|index| match index {
        0 => 0.768_221_279_597_375_5,
        1 => 0.640_184_399_664_479_6,
        _ => 0.0,
    })
}

fn phase_selected_step() -> [f64; N] {
    std::array::from_fn(|index| match index {
        0 => 0.6,
        1 => 0.5,
        _ => 0.0,
    })
}

fn phase_quarter_selected_step() -> [f64; N] {
    std::array::from_fn(|index| match index {
        0 => f64::from_bits(0x3fc8_9544_cafe_c8ad),
        1 => f64::from_bits(0x3fc4_7c63_fe7e_fc90),
        _ => 0.0,
    })
}

fn phase_new_base_natural_step() -> [f64; N] {
    std::array::from_fn(|index| match index {
        0 => f64::from_bits(0x3fe3_3333_3333_3266),
        1 => 0.5,
        _ => 0.0,
    })
}

fn phase_new_base_selected_step() -> [f64; N] {
    std::array::from_fn(|index| match index {
        0 => f64::from_bits(0x3fd3_3333_3333_3266),
        1 => 0.25,
        _ => 0.0,
    })
}

#[test]
fn m1_stage2_unchanged_predictor_phase_uses_one_assembly_and_final_probe() {
    let mut fixture = Fixture::new(FixtureKind::PhaseUnchanged);
    let base = fixture.base;
    let transition = super::m1_trust_region_controller_transition_for_test(&mut fixture).unwrap();
    let predictor = phase_predictor();
    assert_eq!(
        transition.scaled_step.map(f64::to_bits),
        predictor.map(f64::to_bits)
    );
    assert_eq!(transition.stage1_observation.entered_svd_factorizations, 1);
    let mut expected = phase_fixture_log(base, predictor, None, predictor, true, false);
    expected.extend([
        FixtureOperation::PreEvaluatorTrialDomain(
            transition.stage1_observation.candidate_coordinates,
        ),
        FixtureOperation::Evaluate(transition.stage1_observation.candidate_coordinates),
    ]);
    assert_eq!(fixture.operation_log, expected);
    assert_eq!(fixture.evaluator_entries, 2);
}

#[test]
fn m1_stage2_changed_predictor_phase_reassembles_once_and_resolves_stage1() {
    let mut fixture = Fixture::new(FixtureKind::PhaseChanged);
    let base = fixture.base;
    let transition = super::m1_trust_region_controller_transition_for_test(&mut fixture).unwrap();
    let predictor = phase_predictor();
    let selected = phase_selected_step();
    assert_eq!(
        transition.scaled_step.map(f64::to_bits),
        selected.map(f64::to_bits)
    );
    assert_eq!(transition.stage1_observation.entered_svd_factorizations, 1);
    assert_eq!(
        transition.weighted_jacobian[0][0].to_bits(),
        2.0f64.to_bits()
    );
    assert_eq!(
        transition.weighted_jacobian[1][1].to_bits(),
        2.0f64.to_bits()
    );
    let mut expected = phase_fixture_log(base, predictor, Some(selected), selected, true, false);
    expected.extend([
        FixtureOperation::PreEvaluatorTrialDomain(
            transition.stage1_observation.candidate_coordinates,
        ),
        FixtureOperation::Evaluate(transition.stage1_observation.candidate_coordinates),
    ]);
    assert_eq!(fixture.operation_log, expected);
    assert_eq!(fixture.evaluator_entries, 2);
}

#[test]
fn m1_trust_region_refinement_controller_accounting_observes_all_solves_and_errors() {
    let mut changed = Fixture::new(FixtureKind::PhaseChanged);
    super::m1_trust_region_controller_transition_for_test(&mut changed)
        .expect("phase reassembly must retain its actual two solves");
    assert_eq!(changed.accounting_observations.len(), 1);
    let changed_work = changed.accounting_observations[0].clone();
    assert_eq!(changed_work.stage1_calls, 2);
    assert_eq!(changed_work.entered_svd_factorizations, 2);
    assert!(changed_work.total_jacobi_sweeps > 0);
    assert!(changed_work.whole_kkt_operations > 0);
    assert!(changed_work.whole_kkt_guard_events > 0);
    assert!(changed_work.upward_enclosure_operations > 0);
    assert!(changed_work.upward_enclosure_guard_events > 0);

    let mut partial = Fixture::new(FixtureKind::PhaseFinalSelectionError);
    assert!(matches!(
        super::m1_trust_region_controller_transition_for_test(&mut partial),
        Err(M1TrustRegionControllerError::Own(
            "PhaseActiveSetUnrepresentable"
        ))
    ));
    assert_eq!(partial.accounting_observations.len(), 1);
    assert_eq!(partial.accounting_observations[0], changed_work);

    let mut natural = Fixture::new(FixtureKind::PhaseUnchanged);
    super::m1_trust_region_controller_transition_for_test(&mut natural)
        .expect("the natural first face must remain a successful control");
    assert_eq!(natural.accounting_observations.len(), 1);
    let natural_work = natural.accounting_observations[0].clone();

    let mut rank_deficient = Fixture::new(FixtureKind::PhaseChanged);
    rank_deficient.selected_jacobian_rank_deficient = true;
    assert!(matches!(
        super::m1_trust_region_controller_transition_for_test(&mut rank_deficient),
        Err(M1TrustRegionControllerError::Subproblem(refusal))
            if refusal.kind == crate::M1TrustRegionRefusalKind::RankDeficient
    ));
    assert_eq!(rank_deficient.accounting_observations.len(), 1);
    let rank_deficient_work = rank_deficient.accounting_observations[0].clone();
    let mut expected_rank_deficient_work = natural_work;
    expected_rank_deficient_work.stage1_calls += 1;
    expected_rank_deficient_work.entered_svd_factorizations += 1;
    expected_rank_deficient_work.total_jacobi_sweeps += 1;
    assert_eq!(rank_deficient_work, expected_rank_deficient_work);

    let mut initial = Fixture::new(FixtureKind::PhaseUnchanged);
    initial.injected_evaluate_error = Some("initial accounting error");
    assert!(matches!(
        super::m1_trust_region_controller_transition_for_test(&mut initial),
        Err(M1TrustRegionControllerError::Own(
            "initial accounting error"
        ))
    ));
    assert_eq!(initial.accounting_observations.len(), 1);
    assert_eq!(
        initial.accounting_observations[0],
        crate::M1TrustRegionWorkObservation::default()
    );

    let mut run = Fixture::new(FixtureKind::ExactRoot);
    let exact_root_run = super::m1_trust_region_controller_run_for_test(&mut run)
        .expect("existing analytic root scope must remain successful");
    assert_eq!(run.accounting_observations.len(), 1);
    let exact_root_work = &run.accounting_observations[0];
    assert_eq!(exact_root_work, &exact_root_run.work);
    assert!(exact_root_work.entered_svd_factorizations > 0);
    assert!(exact_root_work.total_jacobi_sweeps > 0);
    assert_eq!(
        exact_root_run.refinement_scalar_operations,
        exact_root_work.refinement_scalar_operations
    );
    assert_eq!(
        exact_root_run.refinement_guard_events,
        exact_root_work.refinement_guard_events
    );
    assert_eq!(
        u32::from(exact_root_run.refinement_attempted_entries),
        exact_root_work.refinement_attempted_entries
    );
    assert_eq!(
        u32::from(exact_root_run.refinement_denied_entries),
        exact_root_work.refinement_denied_entries
    );
}

#[test]
fn m1_stage2_phase_selection_errors_retain_owning_error_and_chronology() {
    let predictor = phase_predictor();
    let selected = phase_selected_step();
    for (kind, selected_direction) in [
        (FixtureKind::PhasePredictorSelectionError, None),
        (FixtureKind::PhaseFinalSelectionError, Some(selected)),
    ] {
        let mut fixture = Fixture::new(kind);
        let base = fixture.base;
        assert!(matches!(
            super::m1_trust_region_controller_transition_for_test(&mut fixture),
            Err(M1TrustRegionControllerError::Own(
                "PhaseActiveSetUnrepresentable"
            ))
        ));
        assert_eq!(
            fixture.operation_log,
            phase_fixture_log(
                base,
                predictor,
                selected_direction,
                selected_direction.unwrap_or(predictor),
                false,
                false
            )
        );
        assert_eq!(fixture.evaluator_entries, 1);
        assert_eq!(fixture.owner_sentinel, 0x4d31_5452_5553_5443);
        assert_eq!(fixture.payload_marker, 0);
        assert!(fixture.materializer_inputs.is_empty());
    }
}

#[test]
fn m1_stage2_final_phase_disagreement_is_owning_and_precedes_trial() {
    let mut fixture = Fixture::new(FixtureKind::PhaseFinalDisagreement);
    let base = fixture.base;
    let predictor = phase_predictor();
    let selected = phase_selected_step();
    assert!(matches!(
        super::m1_trust_region_controller_transition_for_test(&mut fixture),
        Err(M1TrustRegionControllerError::Own(
            "PhaseActiveSetInconsistent"
        ))
    ));
    assert_eq!(
        fixture.operation_log,
        phase_fixture_log(base, predictor, Some(selected), selected, false, true)
    );
    assert_eq!(fixture.evaluator_entries, 1);
    assert_eq!(fixture.owner_sentinel, 0x4d31_5452_5553_5443);
    assert_eq!(fixture.payload_marker, 0);
    assert!(fixture.materializer_inputs.is_empty());
}

#[test]
fn m1_stage2_selected_side_owner_error_follows_final_phase_verification() {
    let mut fixture = Fixture::new(FixtureKind::PhaseSelectedSideError);
    let base = fixture.base;
    let predictor = phase_predictor();
    let selected = phase_selected_step();
    assert!(matches!(
        super::m1_trust_region_controller_transition_for_test(&mut fixture),
        Err(M1TrustRegionControllerError::Own(
            "selected-side owning error"
        ))
    ));
    assert_eq!(
        fixture.operation_log,
        phase_fixture_log(base, predictor, Some(selected), selected, true, false)
    );
    assert_eq!(fixture.evaluator_entries, 1);
    assert_eq!(fixture.owner_sentinel, 0x4d31_5452_5553_5443);
    assert_eq!(fixture.payload_marker, 0);
    assert!(fixture.materializer_inputs.is_empty());
}

#[test]
fn m1_stage2_selected_retry_retains_selected_tangent_after_domain_refusal() {
    let mut fixture = Fixture::new(FixtureKind::PhaseSelectedRetry);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let result = super::m1_trust_region_controller_run_for_test(&mut fixture);
    assert!(matches!(
        result,
        Err(M1TrustRegionControllerError::Own(
            "selected-side owning error"
        ))
    ));
    let predictor = phase_predictor();
    let selected = phase_selected_step();
    let quarter = phase_quarter_selected_step();
    let mut expected = phase_fixture_log(base, predictor, Some(selected), selected, true, false);
    expected.extend([
        FixtureOperation::PreEvaluatorTrialDomain(std::array::from_fn(|index| {
            base[index] + selected[index]
        })),
        FixtureOperation::SelectPhase {
            base,
            step: quarter,
        },
        FixtureOperation::SelectedSideProbe {
            base,
            phase: 1,
            direction: quarter,
        },
    ]);
    assert_eq!(fixture.operation_log, expected);
    assert_eq!(fixture.evaluator_entries, 1);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.payload_marker, 0);
    assert!(fixture.materializer_inputs.is_empty());
}

#[test]
fn m1_stage2_selected_retry_phase_drift_is_terminal_without_third_assembly() {
    let mut fixture = Fixture::new(FixtureKind::PhaseSelectedRetryDrift);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let result = super::m1_trust_region_controller_run_for_test(&mut fixture);
    assert!(matches!(
        result,
        Err(M1TrustRegionControllerError::Own(
            "PhaseActiveSetInconsistent"
        ))
    ));
    let predictor = phase_predictor();
    let selected = phase_selected_step();
    let quarter = phase_quarter_selected_step();
    let mut expected = phase_fixture_log(base, predictor, Some(selected), selected, true, false);
    expected.extend([
        FixtureOperation::PreEvaluatorTrialDomain(std::array::from_fn(|index| {
            base[index] + selected[index]
        })),
        FixtureOperation::SelectPhase {
            base,
            step: quarter,
        },
        FixtureOperation::PhaseActiveSetInconsistent {
            selected_phase: 1,
            final_phase: 0,
        },
    ]);
    assert_eq!(fixture.operation_log, expected);
    assert_eq!(fixture.evaluator_entries, 1);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.payload_marker, 0);
    assert!(fixture.materializer_inputs.is_empty());
}

#[test]
fn m1_stage2_exact_root_full_rank_admits_complete_no_update_witness() {
    let mut fixture = Fixture::new(FixtureKind::ExactRoot);
    let base = fixture.base;
    let run = super::m1_trust_region_controller_run_for_test(&mut fixture).unwrap();
    assert_eq!(run.proposals, 1);
    assert_eq!(run.accepted_updates, 0);
    assert_eq!(run.entered_materializations, 1);
    assert_eq!(
        run.proposal_trace[0].scaled_step.map(f64::to_bits),
        [0.0; N].map(f64::to_bits)
    );
    assert_eq!(
        run.proposal_trace[0]
            .stage1_observation
            .base_coordinates
            .map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(
        run.proposal_trace[0]
            .stage1_observation
            .candidate_coordinates
            .map(f64::to_bits),
        base.map(f64::to_bits)
    );
    let proposal = &run.proposal_trace[0];
    assert_eq!(proposal.lambda.to_bits(), 0.0f64.to_bits());
    assert_eq!(proposal.stage1_observation.entered_svd_factorizations, 1);
    assert_eq!(proposal.predicted_reduction.to_bits(), 0.0f64.to_bits());
    assert_eq!(proposal.actual_reduction, None);
    assert_eq!(proposal.rho, None);
    assert_eq!(proposal.moving_merit, None);
    assert_eq!(
        proposal.dynamic_max_normalized_residual.unwrap().to_bits(),
        0.0f64.to_bits()
    );
    assert!(!proposal.installed);
    assert_eq!(proposal.next_radius.to_bits(), 1.0f64.to_bits());
    assert_eq!(
        proposal.disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::RootAdmitted
    );
    for row in 0..N {
        for column in 0..N {
            let expected: f64 = if row == column {
                if matches!(row, 5 | 11) {
                    f64::from_bits(0x3fef_ffff_ffff_ffff)
                } else {
                    1.0
                }
            } else {
                0.0
            };
            assert_eq!(
                proposal.weighted_jacobian[row][column].to_bits(),
                expected.to_bits()
            );
        }
    }
    assert_eq!(fixture.materializer_inputs, vec![base]);
    assert_eq!(fixture.payload_marker, 1);
    assert_eq!(
        fixture.operation_log,
        full_no_update_log(base, [0.0; N], base)
    );
}

#[test]
fn m1_stage2_all_eight_hydraulic_operands_fail_closed_before_admission() {
    for poison in 0..8 {
        let mut fixture = Fixture::new(FixtureKind::HydraulicPoison(poison));
        let base = fixture.base;
        assert!(matches!(
            super::m1_trust_region_controller_transition_for_test(&mut fixture),
            Err(M1TrustRegionControllerError::Numerical(
                "nonfinite governed step operand"
            ))
        ));
        assert_eq!(fixture.evaluator_entries, 2);
        assert_eq!(fixture.owner_sentinel, 0x4d31_5452_5553_5443);
        assert_eq!(fixture.payload_marker, 0);
        assert!(fixture.materializer_inputs.is_empty());
        let step = std::array::from_fn(|index| {
            if index == 0 {
                0.999_999_999_999_997_6
            } else {
                0.0
            }
        });
        let candidate = std::array::from_fn(|index| base[index] + step[index]);
        assert_eq!(fixture.operation_log, ordinary_log(base, step, candidate));
    }
}

#[test]
fn m1_stage2_none_none_wet_operands_preserve_complete_governed_transition() {
    let mut fixture = Fixture::new(FixtureKind::WetNoneNone);
    let transition = super::m1_trust_region_controller_transition_for_test(&mut fixture).unwrap();
    assert_eq!(
        transition.disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::AcceptedUpdate
    );
    assert_eq!(fixture.evaluator_entries, 2);
    assert!(fixture.materializer_inputs.is_empty());
}

#[test]
fn m1_stage2_mixed_wet_presence_rejects_ordinary_nonroot_step_and_shrinks() {
    let mut fixture = Fixture::new(FixtureKind::WetMixed);
    let base = fixture.base;
    let transition = super::m1_trust_region_controller_transition_for_test(&mut fixture).unwrap();
    assert!(!transition.installed);
    assert_eq!(
        transition.base_after.map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(transition.next_radius.to_bits(), 0.25f64.to_bits());
    assert_eq!(transition.entered_materializations, 0);
    assert_eq!(fixture.evaluator_entries, 2);
    assert_eq!(fixture.payload_marker, 0);
    assert_eq!(fixture.owner_sentinel, 0x4d31_5452_5553_5443);
    assert!(fixture.materializer_inputs.is_empty());
    let step = std::array::from_fn(|index| {
        if index == 0 {
            0.999_999_999_999_997_6
        } else {
            0.0
        }
    });
    let candidate = std::array::from_fn(|index| base[index] + step[index]);
    assert_eq!(fixture.operation_log, ordinary_log(base, step, candidate));
}

#[test]
fn m1_stage2_full_mixed_wet_presence_cannot_open_witness_then_complete_quarter_updates() {
    let mut fixture = Fixture::new(FixtureKind::ReplacementWitnessMixedThenComplete);
    let base = fixture.base;
    let run = super::m1_trust_region_controller_run_for_test(&mut fixture).unwrap();
    let steps = positive_replacement_steps();
    let complete =
        std::array::from_fn(|index| base[index] + fixture.scale[index] * steps[1][index]);
    assert_eq!(run.proposals, 2);
    assert_eq!(run.accepted_updates, 1);
    assert_eq!(run.entered_materializations, 1);
    assert_eq!(fixture.materializer_inputs, vec![complete]);
    assert_eq!(fixture.payload_marker, 1);
    assert_eq!(fixture.owner_sentinel, 0x4d31_5452_5553_5443);
    assert_eq!(fixture.evaluator_entries, 3);
    assert_eq!(run.proposal_trace.len(), 2);
    assert_eq!(
        run.proposal_trace[0].scaled_step.map(f64::to_bits),
        steps[0].map(f64::to_bits)
    );
    assert_eq!(
        run.proposal_trace[1].scaled_step.map(f64::to_bits),
        steps[1].map(f64::to_bits)
    );
    assert!(!run.proposal_trace[0].installed);
    assert!(run.proposal_trace[1].installed);
    assert_eq!(
        run.proposal_trace[0].base_before.map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(
        run.proposal_trace[1].base_before.map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(
        run.proposal_trace[1].base_after.map(f64::to_bits),
        complete.map(f64::to_bits)
    );
    assert_eq!(
        run.proposal_trace[0]
            .stage1_observation
            .candidate_coordinates
            .map(f64::to_bits),
        std::array::from_fn(|index| base[index] + fixture.scale[index] * steps[0][index])
            .map(f64::to_bits)
    );
    assert_eq!(
        run.proposal_trace[1]
            .stage1_observation
            .candidate_coordinates
            .map(f64::to_bits),
        complete.map(f64::to_bits)
    );
    let mut expected = witness_attempt_log(base, fixture.scale, &steps[..2], &[true, true], false);
    expected.push(FixtureOperation::Materialize(complete));
    assert_eq!(fixture.operation_log, expected);
}

#[test]
fn m1_stage2_mixed_replacement_is_consumed_and_three_excess_exhausts() {
    let mut fixture = Fixture::new(FixtureKind::ReplacementWitnessThreeExcessMixedReplacement);
    let base = fixture.base;
    let steps = governed_excess_steps();
    let mut calibration = Fixture::new(FixtureKind::ReplacementWitnessThreeExcessMixedReplacement);
    for (index, step) in steps.iter().enumerate() {
        let index = i32::try_from(index).expect("eleven governed endpoints fit in i32");
        let candidate: [f64; N] = std::array::from_fn(|coordinate| {
            calibration.base[coordinate] + calibration.scale[coordinate] * step[coordinate]
        });
        let temperature_displacement = candidate[0] - calibration.base[0];
        assert_eq!(
            temperature_displacement.to_bits(),
            2f64.powi(-24 - 2 * index).to_bits()
        );
        assert_eq!(
            temperature_displacement > 2f64.powi(-27) && temperature_displacement < 2f64.powi(-25),
            index == 1
        );
        let evaluation = calibration.evaluate(candidate).unwrap();
        assert_eq!(
            evaluation.wet_temperature_k,
            if index == 1 {
                [Some(300.0), None]
            } else {
                [Some(300.0); 2]
            }
        );
    }
    let run = super::m1_trust_region_controller_run_for_test(&mut fixture).unwrap();
    assert_eq!(
        run.terminal_refusal,
        Some(super::M1TrustRegionRefusalKind::RadiusExhausted)
    );
    assert_eq!(run.proposals, 11);
    assert_eq!(run.accepted_updates, 0);
    assert_eq!(run.entered_materializations, 0);
    assert_eq!(
        run.final_coordinates.map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(fixture.evaluator_entries, 12);
    assert_eq!(fixture.payload_marker, 0);
    assert_eq!(fixture.owner_sentinel, 0x4d31_5452_5553_5443);
    assert!(fixture.materializer_inputs.is_empty());
    assert_eq!(run.proposal_trace.len(), 11);
    for (index, proposal) in run.proposal_trace.iter().enumerate() {
        assert_eq!(
            proposal.scaled_step.map(f64::to_bits),
            steps[index].map(f64::to_bits)
        );
        assert_eq!(
            proposal.radius_before.to_bits(),
            stationarity_radius(index).to_bits()
        );
        assert_eq!(
            proposal.next_radius.to_bits(),
            stationarity_radius(index + 1).to_bits()
        );
        assert_eq!(
            proposal.base_before.map(f64::to_bits),
            base.map(f64::to_bits)
        );
        assert_eq!(
            proposal.base_after.map(f64::to_bits),
            base.map(f64::to_bits)
        );
    }
    assert_eq!(
        run.proposal_trace[10].next_radius.to_bits(),
        2f64.powi(-22).to_bits()
    );
    assert_eq!(
        fixture.operation_log,
        governed_excess_log(base, fixture.scale, &steps)
    );
}

#[test]
fn m1_stage2_accepted_update_reassembles_natural_phase_at_new_base() {
    let mut fixture = Fixture::new(FixtureKind::PhaseAcceptedUpdateReset);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let result = super::m1_trust_region_controller_run_for_test(&mut fixture);
    assert!(matches!(
        result,
        Err(M1TrustRegionControllerError::Own(
            "selected-side owning error"
        ))
    ));
    let predictor = phase_predictor();
    let selected = phase_selected_step();
    let natural = phase_new_base_natural_step();
    let reselected = phase_new_base_selected_step();
    let mut new_base = base;
    new_base[0] += selected[0];
    new_base[1] += selected[1];
    let mut expected = phase_fixture_log(base, predictor, Some(selected), selected, true, false);
    expected.extend([
        FixtureOperation::PreEvaluatorTrialDomain(new_base),
        FixtureOperation::Evaluate(new_base),
        FixtureOperation::SelectPhase {
            base: new_base,
            step: [0.0; N],
        },
        FixtureOperation::RawJacobian {
            base: new_base,
            selected_phase: None,
            predictor_direction: [0.0; N],
        },
        FixtureOperation::SelectPhase {
            base: new_base,
            step: natural,
        },
        FixtureOperation::RawJacobian {
            base: new_base,
            selected_phase: Some(1),
            predictor_direction: natural,
        },
        FixtureOperation::SelectPhase {
            base: new_base,
            step: reselected,
        },
        FixtureOperation::SelectedSideProbe {
            base: new_base,
            phase: 1,
            direction: reselected,
        },
    ]);
    assert_eq!(fixture.operation_log, expected);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.payload_marker, 0);
    assert!(fixture.materializer_inputs.is_empty());
}
#[test]
fn m1_stage2_constrained_stationarity_is_nonterminal_rejection() {
    let mut f = Fixture::new(FixtureKind::ConstrainedStationarity);
    let sentinel = f.owner_sentinel;
    let t = super::m1_trust_region_controller_transition_for_test(&mut f).unwrap();
    assert_eq!(t.scaled_step.map(f64::to_bits), [0.0; N].map(f64::to_bits));
    close(t.weighted_jacobian[5][5], 1.0);
    close(t.stage1_observation.final_g[5], 2.0);
    close(t.stage1_observation.final_h[5], 2.0);
    close(t.predicted_reduction, 0.0);
    assert_eq!(t.actual_reduction, None);
    assert_eq!(t.rho, None);
    assert_eq!(t.moving_merit, None);
    assert_eq!(t.dynamic_max_normalized_residual, None);
    assert_eq!(t.disposition, super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::NoPredictedReductionAndShrunk);
    assert_eq!(t.radius_before.to_bits(), 1.0f64.to_bits());
    assert_eq!(t.base_before.map(f64::to_bits), f.base.map(f64::to_bits));
    assert_eq!(t.base_after.map(f64::to_bits), f.base.map(f64::to_bits));
    assert_eq!(t.next_radius.to_bits(), 0.25f64.to_bits());
    assert!(!t.installed);
    assert_eq!(t.entered_materializations, 0);
    assert_eq!(t.stage1_observation.entered_svd_factorizations, 2);
    assert_eq!(t.stage1_observation.completed_face_pivots, 1);
    assert_eq!(t.stage1_observation.event_trace.len(), 2);
    assert!(
        t.stage1_observation
            .initial_lower_mask
            .iter()
            .all(|active| !active)
    );
    assert!(
        t.stage1_observation
            .initial_upper_mask
            .iter()
            .all(|active| !active)
    );
    assert!(
        t.stage1_observation
            .initial_free_mask
            .iter()
            .all(|active| *active)
    );
    assert_eq!(
        t.stage1_observation.event_trace[0].action,
        super::M1TrustRegionKktAction::ActivateLower
    );
    assert_eq!(t.stage1_observation.event_trace[0].selected_coordinate, 5);
    assert!(t.stage1_observation.event_trace[0].lower_mask[5]);
    assert!(only_lower_five(
        t.stage1_observation.event_trace[0].lower_mask
    ));
    assert!(
        t.stage1_observation.event_trace[0]
            .upper_mask
            .iter()
            .all(|active| !active)
    );
    assert!(!t.stage1_observation.event_trace[0].free_mask[5]);
    assert_eq!(
        t.stage1_observation.event_trace[1].action,
        super::M1TrustRegionKktAction::Return
    );
    assert!(only_lower_five(
        t.stage1_observation.event_trace[1].lower_mask
    ));
    assert!(
        t.stage1_observation.event_trace[1]
            .upper_mask
            .iter()
            .all(|active| !active)
    );
    assert_eq!(
        t.stage1_observation.event_trace[1].free_mask,
        std::array::from_fn(|index| index != 5)
    );
    assert!(
        t.stage1_observation.event_trace[1]
            .free_mask
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != 5)
            .all(|(_, free)| *free)
    );
    assert!(only_lower_five(t.stage1_observation.active_lower));
    assert!(
        t.stage1_observation
            .active_upper
            .iter()
            .all(|active| !active)
    );
    assert_eq!(f.operation_log, stationarity_log(f.base, 1));
    assert_eq!(f.evaluator_entries, 1);
    assert_eq!(f.owner_sentinel, sentinel);
}
#[test]
fn m1_stage2_stationarity_exhausts_after_eleven_proposals_and_twenty_two_faces() {
    let mut f = Fixture::new(FixtureKind::ConstrainedStationarity);
    let base = f.base;
    let sentinel = f.owner_sentinel;
    let r = super::m1_trust_region_controller_run_for_test(&mut f).unwrap();
    assert_eq!(
        (
            r.proposals,
            r.entered_svd_factorizations,
            r.accepted_updates,
            r.entered_materializations
        ),
        (11, 22, 0, 0)
    );
    assert_eq!(
        r.terminal_refusal,
        Some(super::M1TrustRegionRefusalKind::RadiusExhausted)
    );
    assert_eq!(
        r.final_coordinates.map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert!(r.materialized_coordinates.is_none());
    assert_eq!(r.proposal_trace.len(), 11);
    for (index, proposal) in r.proposal_trace.iter().enumerate() {
        assert_stationarity_proposal(proposal, base, index);
    }
    assert_eq!(
        r.proposal_trace
            .iter()
            .map(|p| u16::from(p.stage1_observation.entered_svd_factorizations))
            .sum::<u16>(),
        22
    );
    assert_eq!(4f64.powi(-11).to_bits(), 2f64.powi(-22).to_bits());
    assert_eq!(
        f.operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::RawJacobian { .. }))
            .count(),
        1
    );
    assert_eq!(
        f.operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::Evaluate(_)))
            .count(),
        1
    );
    assert!(
        !f.operation_log
            .iter()
            .any(|entry| matches!(entry, FixtureOperation::Materialize(_)))
    );
    assert_eq!(f.owner_sentinel, sentinel);
    assert_eq!(f.operation_log, stationarity_log(base, 11));
    assert_eq!(f.evaluator_entries, 1);
}
#[test]
fn m1_stage2_full_no_update_materializes_current_base() {
    let mut f = Fixture::new(FixtureKind::FullNoUpdate);
    let base = f.base;
    let e = f.evaluate(base).unwrap();
    assert_eq!(e.raw_residual[0].to_bits(), (-2f64.powi(-88)).to_bits());
    assert_eq!(e.dynamic_normalizers[0].to_bits(), 2f64.powi(-60).to_bits());
    f.operation_log.clear();
    f.evaluator_entries = 0;
    let sentinel = f.owner_sentinel;
    let r = super::m1_trust_region_controller_run_for_test(&mut f).unwrap();
    assert_eq!(r.initial_radius.to_bits(), 1.0f64.to_bits());
    assert_eq!(
        (r.proposals, r.accepted_updates, r.entered_materializations),
        (1, 0, 1)
    );
    assert!(same_coordinate_bits(
        r.materialized_coordinates.unwrap(),
        base
    ));
    assert_eq!(f.materializer_inputs, vec![base]);
    assert_eq!(f.payload_marker, 1);
    assert_eq!(f.owner_sentinel, sentinel);
    let mut expected_candidate = base;
    expected_candidate[0] += 2f64.powi(-28);
    let direction = std::array::from_fn(|i| f.scale[i] * if i == 0 { 2f64.powi(-28) } else { 0.0 });
    assert_eq!(r.proposal_trace.len(), 1);
    assert_full_no_update_proposal(&r.proposal_trace[0], base, expected_candidate);
    assert_eq!(f.evaluator_entries, 2);
    assert_eq!(
        f.operation_log,
        full_no_update_log(base, direction, expected_candidate)
    );
    assert_eq!(
        f.operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::Evaluate(candidate) if same_coordinate_bits(*candidate, expected_candidate)))
            .count(),
        1
    );
    assert_eq!(
        f.operation_log
            .iter()
            .filter(
                |entry| matches!(entry, FixtureOperation::Materialize(current) if same_coordinate_bits(*current, base))
            )
            .count(),
        1
    );
}

#[test]
fn m1_stage2_full_no_update_candidate_raw_and_normalizer_faults_fail_before_materialization() {
    for (fault, message) in raw_normalizer_faults() {
        let mut fixture = Fixture::new(FixtureKind::FullNoUpdate);
        fixture.evaluation_fault = Some((2, fault));
        let base = fixture.base;
        let sentinel = fixture.owner_sentinel;
        assert!(matches!(
            super::m1_trust_region_controller_run_for_test(&mut fixture),
            Err(M1TrustRegionControllerError::Numerical(actual)) if actual == message
        ));
        let mut candidate = base;
        candidate[0] += 2f64.powi(-28);
        let direction = std::array::from_fn(|index| {
            fixture.scale[index] * if index == 0 { 2f64.powi(-28) } else { 0.0 }
        });
        let mut expected = full_no_update_log(base, direction, candidate);
        assert!(matches!(
            expected.pop(),
            Some(FixtureOperation::Materialize(_))
        ));
        assert_eq!(fixture.operation_log, expected);
        assert_eq!(fixture.evaluator_entries, 2);
        assert!(fixture.materializer_inputs.is_empty());
        assert_eq!(fixture.owner_sentinel, sentinel);
        assert_eq!(fixture.payload_marker, 0);
    }
}

#[test]
fn m1_stage2_full_no_update_candidate_governed_operands_fail_closed() {
    for fault in governed_faults() {
        let mut fixture = Fixture::new(FixtureKind::FullNoUpdate);
        fixture.evaluation_fault = Some((2, fault));
        let base = fixture.base;
        let sentinel = fixture.owner_sentinel;
        assert!(matches!(
            super::m1_trust_region_controller_run_for_test(&mut fixture),
            Err(M1TrustRegionControllerError::Numerical(
                "nonfinite governed step operand"
            ))
        ));
        let mut candidate = base;
        candidate[0] += 2f64.powi(-28);
        let direction = std::array::from_fn(|index| {
            fixture.scale[index] * if index == 0 { 2f64.powi(-28) } else { 0.0 }
        });
        let mut expected = full_no_update_log(base, direction, candidate);
        assert!(matches!(
            expected.pop(),
            Some(FixtureOperation::Materialize(_))
        ));
        assert_eq!(fixture.operation_log, expected);
        assert_eq!(fixture.evaluator_entries, 2);
        assert!(fixture.materializer_inputs.is_empty());
        assert_eq!(fixture.owner_sentinel, sentinel);
        assert_eq!(fixture.payload_marker, 0);
    }
}

#[test]
fn m1_stage2_full_mixed_wet_hydraulic_nan_precedes_incomplete_scope() {
    let mut fixture = Fixture::new(FixtureKind::ReplacementWitnessMixedThenComplete);
    fixture.evaluation_fault = Some((2, EvaluationFault::HydraulicNaN(0)));
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    assert!(matches!(
        super::m1_trust_region_controller_run_for_test(&mut fixture),
        Err(M1TrustRegionControllerError::Numerical(
            "nonfinite governed step operand"
        ))
    ));
    let steps = positive_replacement_steps();
    assert_eq!(
        fixture.operation_log,
        witness_attempt_log(base, fixture.scale, &steps[..1], &[true], false)
    );
    assert_eq!(fixture.evaluator_entries, 2);
    assert!(fixture.materializer_inputs.is_empty());
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.payload_marker, 0);
}

#[test]
fn m1_stage2_initial_raw_and_normalizer_faults_fail_at_weighted_system_guard() {
    for (fault, _) in raw_normalizer_faults() {
        let mut fixture = Fixture::new(FixtureKind::FullNoUpdate);
        fixture.evaluation_fault = Some((1, fault));
        let base = fixture.base;
        let sentinel = fixture.owner_sentinel;
        assert!(matches!(
            super::m1_trust_region_controller_run_for_test(&mut fixture),
            Err(M1TrustRegionControllerError::Numerical(
                "invalid weighted system input"
            ))
        ));
        assert_eq!(fixture.evaluator_entries, 1);
        assert_eq!(
            fixture.operation_log,
            vec![
                FixtureOperation::Evaluate(base),
                FixtureOperation::SelectPhase {
                    base,
                    step: [0.0; N],
                },
                FixtureOperation::RawJacobian {
                    base,
                    selected_phase: None,
                    predictor_direction: [0.0; N],
                },
            ]
        );
        assert_eq!(fixture.base.map(f64::to_bits), base.map(f64::to_bits));
        assert!(fixture.materializer_inputs.is_empty());
        assert_eq!(fixture.owner_sentinel, sentinel);
        assert_eq!(fixture.payload_marker, 0);
    }
}

#[test]
fn m1_stage2_post_update_candidate_faults_precede_installation() {
    for (fault, message) in raw_normalizer_faults().into_iter().chain(
        governed_faults()
            .into_iter()
            .map(|fault| (fault, "nonfinite governed step operand")),
    ) {
        let mut fixture = Fixture::new(FixtureKind::PostUpdateAdmission);
        fixture.evaluation_fault = Some((2, fault));
        let base = fixture.base;
        let sentinel = fixture.owner_sentinel;
        let step: [f64; N] =
            std::array::from_fn(|index| if index == 0 { 2f64.powi(-28) } else { 0.0 });
        let direction = std::array::from_fn(|index| fixture.scale[index] * step[index]);
        let mut candidate = base;
        candidate[0] += 2f64.powi(-28);
        assert!(matches!(
            super::m1_trust_region_controller_run_for_test(&mut fixture),
            Err(M1TrustRegionControllerError::Numerical(actual)) if actual == message
        ));
        assert_eq!(fixture.evaluator_entries, 2);
        assert_eq!(
            fixture.operation_log,
            ordinary_log(base, direction, candidate)
        );
        assert_eq!(fixture.base.map(f64::to_bits), base.map(f64::to_bits));
        assert!(fixture.materializer_inputs.is_empty());
        assert_eq!(fixture.owner_sentinel, sentinel);
        assert_eq!(fixture.payload_marker, 0);
    }
}

#[test]
fn m1_stage2_finite_governed_threshold_operands_gate_full_no_update_admission() {
    assert_eq!(governed_threshold_roles().len(), 29);
    for role in governed_threshold_roles() {
        for negative in [false, true] {
            for above in [false, true] {
                let (projection, operand_base, threshold) =
                    threshold_projection(role, negative, above);
                let mut calibration = Fixture::new(FixtureKind::FullNoUpdate);
                calibration.governed_finite_projection = Some((1, projection));
                let evaluation = calibration.evaluate(calibration.base).unwrap();
                let operand = match projection {
                    GovernedFiniteProjection::TemperatureCoordinate(index, _) => {
                        evaluation.coordinates[index]
                    }
                    GovernedFiniteProjection::WetTemperature(index, _) => {
                        evaluation.wet_temperature_k[index].unwrap()
                    }
                    GovernedFiniteProjection::Humidity(_) => evaluation.coordinates[13],
                    GovernedFiniteProjection::Hydraulic(index, _) => {
                        match evaluation.current_hydraulics {
                            M1TrustRegionCurrentHydraulics::Complete(values) => values[index],
                            M1TrustRegionCurrentHydraulics::Pending => {
                                panic!("analytic fixture must provide hydraulic operands")
                            }
                        }
                    }
                    GovernedFiniteProjection::Beta(index, _) => evaluation.beta[index],
                    GovernedFiniteProjection::Ci(_, _) => unreachable!(),
                };
                let difference = (operand - operand_base).abs();
                assert_eq!(
                    operand.to_bits(),
                    match projection {
                        GovernedFiniteProjection::TemperatureCoordinate(_, value)
                        | GovernedFiniteProjection::WetTemperature(_, value)
                        | GovernedFiniteProjection::Humidity(value)
                        | GovernedFiniteProjection::Hydraulic(_, value)
                        | GovernedFiniteProjection::Beta(_, value) => value.to_bits(),
                        GovernedFiniteProjection::Ci(_, _) => unreachable!(),
                    }
                );
                assert_eq!(difference <= threshold, !above);

                let mut fixture = Fixture::new(FixtureKind::FullNoUpdate);
                fixture.governed_finite_projection = Some((2, projection));
                let base = fixture.base;
                let sentinel = fixture.owner_sentinel;
                let mut candidate = base;
                candidate[0] += 2f64.powi(-28);
                let direction = std::array::from_fn(|index| {
                    fixture.scale[index] * if index == 0 { 2f64.powi(-28) } else { 0.0 }
                });
                let transition =
                    super::m1_trust_region_controller_transition_for_test(&mut fixture).unwrap();
                let mut expected = full_no_update_log(base, direction, candidate);
                if above {
                    assert_eq!(
                    transition.disposition,
                    super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk
                );
                    assert!(matches!(
                        expected.pop(),
                        Some(FixtureOperation::Materialize(_))
                    ));
                    assert!(fixture.materializer_inputs.is_empty());
                    assert_eq!(fixture.payload_marker, 0);
                } else {
                    assert_eq!(
                    transition.disposition,
                    super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::RootAdmitted
                );
                    assert_eq!(fixture.materializer_inputs, vec![base]);
                    assert_eq!(fixture.payload_marker, 1);
                }
                assert_eq!(fixture.operation_log, expected);
                assert_eq!(fixture.evaluator_entries, 2);
                assert_eq!(fixture.base.map(f64::to_bits), base.map(f64::to_bits));
                assert_eq!(fixture.owner_sentinel, sentinel);
            }
        }
    }
}

#[test]
fn m1_stage2_hydraulic_threshold_equality_admits_full_no_update() {
    for index in 0..8 {
        for value in [-1.0e-7, 1.0e-7] {
            let mut fixture = Fixture::new(FixtureKind::FullNoUpdate);
            fixture.governed_finite_projection =
                Some((2, GovernedFiniteProjection::Hydraulic(index, value)));
            let base = fixture.base;
            let sentinel = fixture.owner_sentinel;
            let mut candidate = base;
            candidate[0] += 2f64.powi(-28);
            let direction = std::array::from_fn(|slot| {
                fixture.scale[slot] * if slot == 0 { 2f64.powi(-28) } else { 0.0 }
            });
            let transition =
                super::m1_trust_region_controller_transition_for_test(&mut fixture).unwrap();
            assert_eq!(value.abs().to_bits(), 1.0e-7f64.to_bits());
            assert_eq!(
                transition.disposition,
                super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::RootAdmitted
            );
            assert_eq!(
                fixture.operation_log,
                full_no_update_log(base, direction, candidate)
            );
            assert_eq!(fixture.evaluator_entries, 2);
            assert_eq!(fixture.materializer_inputs, vec![base]);
            assert_eq!(fixture.owner_sentinel, sentinel);
            assert_eq!(fixture.payload_marker, 1);
        }
    }
}

#[test]
fn m1_stage2_finite_ci_operands_do_not_create_a_governed_threshold() {
    for index in 0..4 {
        for value in [-1.0e100, 1.0e100] {
            let mut calibration = Fixture::new(FixtureKind::FullNoUpdate);
            calibration.governed_finite_projection =
                Some((1, GovernedFiniteProjection::Ci(index, value)));
            let evaluation = calibration.evaluate(calibration.base).unwrap();
            assert!(evaluation.ci_pa[index].is_finite());
            assert_eq!(evaluation.ci_pa[index].to_bits(), value.to_bits());
            let mut fixture = Fixture::new(FixtureKind::FullNoUpdate);
            fixture.governed_finite_projection =
                Some((2, GovernedFiniteProjection::Ci(index, value)));
            let base = fixture.base;
            let sentinel = fixture.owner_sentinel;
            let transition =
                super::m1_trust_region_controller_transition_for_test(&mut fixture).unwrap();
            let mut candidate = base;
            candidate[0] += 2f64.powi(-28);
            let direction = std::array::from_fn(|slot| {
                fixture.scale[slot] * if slot == 0 { 2f64.powi(-28) } else { 0.0 }
            });
            assert_eq!(
            transition.disposition,
            super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::RootAdmitted
        );
            assert_eq!(
                fixture.operation_log,
                full_no_update_log(base, direction, candidate)
            );
            assert_eq!(fixture.evaluator_entries, 2);
            assert_eq!(fixture.materializer_inputs, vec![base]);
            assert_eq!(fixture.owner_sentinel, sentinel);
            assert_eq!(fixture.payload_marker, 1);
        }
    }
}

pub(crate) fn m1_trust_region_analytic_work_cap_run_for_test()
-> super::m1_trust_region_controller_interface::M1TrustRegionControllerRun {
    let mut fixture = Fixture::new(FixtureKind::LongProgress {
        target: 1000.0,
        perturb_at_183: false,
    });
    super::m1_trust_region_controller_run_for_test(&mut fixture)
        .expect("analytic long-progress fixture must reach its declared terminal refusal")
}

#[test]
fn m1_stage2_long_progress_work_cap_is_fifty_accepted_proposals() {
    let mut fixture = Fixture::new(FixtureKind::LongProgress {
        target: 1000.0,
        perturb_at_183: false,
    });
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let run = super::m1_trust_region_controller_run_for_test(&mut fixture).unwrap();
    assert_eq!(
        run.terminal_refusal,
        Some(super::M1TrustRegionRefusalKind::WorkCap)
    );
    assert_eq!(run.proposals, 50);
    assert_eq!(run.accepted_updates, 50);
    assert_eq!(fixture.hydraulics.post_update_resets, 49);
    assert_eq!(run.proposal_trace.len(), 50);
    assert!(run.proposal_trace.iter().all(|proposal| proposal.installed));
    assert!(run.proposal_trace.iter().all(|proposal| matches!(
        proposal.disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::AcceptedUpdate
    )));
    assert_eq!(
        operation_kinds(&fixture.operation_log),
        long_progress_kinds(49, false)
    );
    assert_eq!(fixture.evaluator_entries, 51);
    assert_eq!(
        fixture
            .operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::RawJacobian { .. }))
            .count(),
        50
    );
    assert_eq!(
        fixture
            .operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::PreEvaluatorTrialDomain(_)))
            .count(),
        50
    );
    assert!(fixture.materializer_inputs.is_empty());
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.payload_marker, 0);
    assert_eq!(fixture.base.map(f64::to_bits), base.map(f64::to_bits));
}

#[test]
fn m1_stage2_long_progress_root_on_fiftieth_update_precedes_next_jacobian() {
    let mut fixture = Fixture::new(FixtureKind::LongProgress {
        target: 383.0,
        perturb_at_183: false,
    });
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let run = super::m1_trust_region_controller_run_for_test(&mut fixture).unwrap();
    assert_eq!(run.proposals, 50);
    assert_eq!(run.accepted_updates, 50);
    assert_eq!(run.entered_materializations, 1);
    assert!(run.proposal_trace.iter().all(|proposal| proposal.installed));
    assert!(run.proposal_trace.iter().all(|proposal| matches!(
        proposal.disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::AcceptedUpdate
    )));
    assert_eq!(fixture.evaluator_entries, 51);
    assert_eq!(
        fixture
            .operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::RawJacobian { .. }))
            .count(),
        50
    );
    assert_eq!(fixture.materializer_inputs.len(), 1);
    assert_eq!(
        operation_kinds(&fixture.operation_log),
        long_progress_kinds(49, true)
    );
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.payload_marker, 1);
    assert_eq!(fixture.base.map(f64::to_bits), base.map(f64::to_bits));
}

#[test]
fn m1_stage2_long_progress_fiftieth_root_materializer_error_precedes_cap() {
    let mut fixture = Fixture::new(FixtureKind::LongProgress {
        target: 383.0,
        perturb_at_183: false,
    });
    fixture.injected_materialize_error = Some("fiftieth root materializer error");
    let sentinel = fixture.owner_sentinel;
    assert!(matches!(
        super::m1_trust_region_controller_run_for_test(&mut fixture),
        Err(M1TrustRegionControllerError::Own(
            "fiftieth root materializer error"
        ))
    ));
    assert_eq!(fixture.evaluator_entries, 51);
    assert_eq!(
        fixture
            .operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::RawJacobian { .. }))
            .count(),
        50
    );
    assert_eq!(
        fixture
            .operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::Materialize(_)))
            .count(),
        1
    );
    assert!(fixture.materializer_inputs.is_empty());
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.payload_marker, 0);
    assert_eq!(
        operation_kinds(&fixture.operation_log),
        long_progress_kinds(49, true)
    );
}

#[test]
fn m1_stage2_long_progress_rejected_global_twenty_fifth_retries_per_base() {
    let mut fixture = Fixture::new(FixtureKind::LongProgress {
        target: 199.0,
        perturb_at_183: true,
    });
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let run = super::m1_trust_region_controller_run_for_test(&mut fixture).unwrap();
    assert_eq!(run.proposals, 30);
    assert_eq!(run.accepted_updates, 29);
    assert_eq!(run.entered_materializations, 1);
    let rejection = &run.proposal_trace[24];
    assert!(!rejection.installed);
    assert_eq!(rejection.radius_before.to_bits(), 8.0f64.to_bits());
    assert_eq!(rejection.next_radius.to_bits(), 2.0f64.to_bits());
    assert_eq!(
        rejection.base_before.map(f64::to_bits),
        rejection.base_after.map(f64::to_bits)
    );
    for (index, proposal) in run.proposal_trace.iter().enumerate() {
        if index != 24 {
            assert!(proposal.installed);
            assert_eq!(
                proposal.disposition,
                super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::AcceptedUpdate
            );
        }
    }
    assert_eq!(
        run.proposal_trace
            .iter()
            .filter(|proposal| proposal.installed)
            .count(),
        29
    );
    assert_eq!(
        run.proposal_trace.iter().filter(|proposal| matches!(
            proposal.disposition,
            super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk
        )).count(),
        1
    );
    assert_eq!(fixture.evaluator_entries, 31);
    assert_eq!(
        fixture
            .operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::RawJacobian { .. }))
            .count(),
        29
    );
    assert_eq!(fixture.materializer_inputs.len(), 1);
    assert_eq!(
        operation_kinds(&fixture.operation_log),
        long_progress_rejection_kinds()
    );
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.payload_marker, 1);
    assert_eq!(fixture.base.map(f64::to_bits), base.map(f64::to_bits));
}

#[test]
fn m1_stage2_exact_root_and_replacement_candidate_faults_fail_closed() {
    for (fault, _) in raw_normalizer_faults() {
        let mut fixture = Fixture::new(FixtureKind::ExactRoot);
        fixture.evaluation_fault = Some((2, fault));
        let base = fixture.base;
        let sentinel = fixture.owner_sentinel;
        assert!(matches!(
            super::m1_trust_region_controller_run_for_test(&mut fixture),
            Err(M1TrustRegionControllerError::Numerical(
                "invalid physical residual operand"
            ))
        ));
        assert_eq!(fixture.evaluator_entries, 2);
        let mut expected = full_no_update_log(base, [0.0; N], base);
        assert!(matches!(
            expected.pop(),
            Some(FixtureOperation::Materialize(_))
        ));
        assert_eq!(fixture.operation_log, expected);
        assert!(fixture.materializer_inputs.is_empty());
        assert_eq!(fixture.owner_sentinel, sentinel);
        assert_eq!(fixture.payload_marker, 0);
    }
    for fault in governed_faults() {
        let mut fixture = Fixture::new(FixtureKind::ExactRoot);
        fixture.evaluation_fault = Some((2, fault));
        let base = fixture.base;
        let sentinel = fixture.owner_sentinel;
        assert!(matches!(
            super::m1_trust_region_controller_run_for_test(&mut fixture),
            Err(M1TrustRegionControllerError::Numerical(
                "nonfinite governed step operand"
            ))
        ));
        assert_eq!(fixture.evaluator_entries, 2);
        let mut expected = full_no_update_log(base, [0.0; N], base);
        assert!(matches!(
            expected.pop(),
            Some(FixtureOperation::Materialize(_))
        ));
        assert_eq!(fixture.operation_log, expected);
        assert!(fixture.materializer_inputs.is_empty());
        assert_eq!(fixture.owner_sentinel, sentinel);
        assert_eq!(fixture.payload_marker, 0);
    }
    let steps = positive_replacement_steps();
    for (fault, message) in raw_normalizer_faults().into_iter().chain(
        governed_faults()
            .into_iter()
            .map(|fault| (fault, "nonfinite governed step operand")),
    ) {
        let mut fixture = Fixture::new(FixtureKind::ReplacementWitness);
        fixture.evaluation_fault = Some((3, fault));
        let base = fixture.base;
        let sentinel = fixture.owner_sentinel;
        assert!(matches!(
            super::m1_trust_region_controller_run_for_test(&mut fixture),
            Err(M1TrustRegionControllerError::Numerical(actual)) if actual == message
        ));
        assert_eq!(fixture.evaluator_entries, 3);
        assert_eq!(
            fixture.operation_log,
            witness_attempt_log(base, fixture.scale, &steps[..2], &[true, true], false)
        );
        assert!(fixture.materializer_inputs.is_empty());
        assert_eq!(fixture.owner_sentinel, sentinel);
        assert_eq!(fixture.payload_marker, 0);
    }
}
#[test]
fn m1_stage2_seeded_work_guard_records_actual_pre_entry_refusal() {
    let mut f = Fixture::new(FixtureKind::Ordinary(0.0));
    let sentinel = f.owner_sentinel;
    let entries = f.evaluator_entries;
    let log = f.operation_log.clone();
    assert_eq!(
        super::m1_trust_region_controller_interface::M1_TRUST_REGION_CORE_ENTRY_CAP,
        6_500
    );
    let g = super::m1_trust_region_seeded_entry_guard_for_test(
        &mut f,
        super::m1_trust_region_controller_interface::M1_TRUST_REGION_CORE_ENTRY_CAP,
    )
    .unwrap();
    assert_eq!(
        (g.incoming_core_entries, g.attempted_delta, g.entered_delta),
        (
            super::m1_trust_region_controller_interface::M1_TRUST_REGION_CORE_ENTRY_CAP,
            1,
            0
        )
    );
    assert_eq!(
        g.refused_operation,
        super::M1TrustRegionControllerOperation::ProposalCore
    );
    assert_eq!(
        g.numeric_refusal,
        Some(super::M1TrustRegionRefusalKind::WorkCap)
    );
    assert_eq!(g.entered_owner_error, None);
    assert_eq!(f.evaluator_entries, entries);
    assert_eq!(f.operation_log, log);
    assert_eq!(f.owner_sentinel, sentinel);

    let mut entered = Fixture::new(FixtureKind::Ordinary(0.0));
    let entered_sentinel = entered.owner_sentinel;
    entered.injected_evaluate_error = Some("entered owning error");
    let entered_guard = super::m1_trust_region_seeded_entry_guard_for_test(
        &mut entered,
        super::m1_trust_region_controller_interface::M1_TRUST_REGION_CORE_ENTRY_CAP - 1,
    )
    .unwrap();
    assert_eq!(
        (
            entered_guard.incoming_core_entries,
            entered_guard.attempted_delta,
            entered_guard.entered_delta
        ),
        (6_499, 1, 1)
    );
    assert_eq!(
        entered_guard.refused_operation,
        super::M1TrustRegionControllerOperation::ProposalCore
    );
    assert_eq!(entered_guard.numeric_refusal, None);
    assert_eq!(
        entered_guard.entered_owner_error,
        Some("entered owning error")
    );
    assert_eq!(entered.evaluator_entries, 1);
    assert_eq!(
        entered.operation_log,
        vec![FixtureOperation::Evaluate(entered.base)]
    );
    assert_eq!(entered.owner_sentinel, entered_sentinel);
}

#[test]
fn m1_stage2_post_update_root_materializes_installed_base_before_reassembly() {
    let mut fixture = Fixture::new(FixtureKind::PostUpdateAdmission);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let run = super::m1_trust_region_controller_run_for_test(&mut fixture).unwrap();
    let mut installed = base;
    installed[0] += 2f64.powi(-28);
    let expected_step = std::array::from_fn(|i| if i == 0 { 2f64.powi(-28) } else { 0.0 });
    assert_eq!(run.proposals, 1);
    assert_eq!(run.accepted_updates, 1);
    assert_eq!(run.entered_materializations, 1);
    assert_eq!(
        run.final_coordinates.map(f64::to_bits),
        installed.map(f64::to_bits)
    );
    assert_eq!(
        run.materialized_coordinates.unwrap().map(f64::to_bits),
        installed.map(f64::to_bits)
    );
    assert_eq!(fixture.materializer_inputs, vec![installed]);
    assert_eq!(fixture.payload_marker, 1);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(
        run.proposal_trace[0].scaled_step.map(f64::to_bits),
        expected_step.map(f64::to_bits)
    );
    assert_eq!(run.proposal_trace[0].lambda.to_bits(), 0.0f64.to_bits());
    assert_eq!(
        run.proposal_trace[0].predicted_reduction.to_bits(),
        2.0f64.to_bits()
    );
    assert_eq!(
        run.proposal_trace[0].radius_before.to_bits(),
        1.0f64.to_bits()
    );
    assert_eq!(
        run.proposal_trace[0].next_radius.to_bits(),
        1.0f64.to_bits()
    );
    assert!(run.proposal_trace[0].installed);
    assert_eq!(
        run.proposal_trace[0].base_before.map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(
        run.proposal_trace[0].base_after.map(f64::to_bits),
        installed.map(f64::to_bits)
    );
    assert_eq!(fixture.evaluator_entries, 2);
    assert_eq!(run.proposal_trace[0].disposition, super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::AcceptedUpdate);
    assert_eq!(
        run.proposal_trace[0].actual_reduction.unwrap().to_bits(),
        2.0f64.to_bits()
    );
    assert_eq!(
        run.proposal_trace[0].rho.unwrap().to_bits(),
        1.0f64.to_bits()
    );
    let direction = std::array::from_fn(|i| fixture.scale[i] * expected_step[i]);
    assert_eq!(
        fixture.operation_log,
        vec![
            FixtureOperation::Evaluate(base),
            FixtureOperation::SelectPhase {
                base,
                step: [0.0; N]
            },
            FixtureOperation::RawJacobian {
                base,
                selected_phase: None,
                predictor_direction: [0.0; N]
            },
            FixtureOperation::SelectPhase {
                base,
                step: direction
            },
            FixtureOperation::SelectedSideProbe {
                base,
                phase: 0,
                direction
            },
            FixtureOperation::PreEvaluatorTrialDomain(installed),
            FixtureOperation::Evaluate(installed),
            FixtureOperation::Materialize(installed),
        ]
    );
}

#[test]
fn m1_stage2_first_valid_radius_replacement_materializes_original_base() {
    let mut fixture = Fixture::new(FixtureKind::ReplacementWitness);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let first_step = std::array::from_fn(|i| match i {
        0 => 2f64.powi(-26),
        3 => 1.0,
        _ => 0.0,
    });
    let second_step = std::array::from_fn(|i| match i {
        0 => 2f64.powi(-28),
        3 => 0.25,
        _ => 0.0,
    });
    let run = super::m1_trust_region_controller_run_for_test(&mut fixture).unwrap();
    assert_eq!(run.proposals, 2);
    assert_eq!(run.accepted_updates, 0);
    assert_eq!(run.entered_materializations, 1);
    assert!(same_coordinate_bits(run.final_coordinates, base));
    assert!(same_coordinate_bits(
        run.materialized_coordinates.unwrap(),
        base
    ));
    assert_eq!(fixture.materializer_inputs, vec![base]);
    assert_eq!(fixture.payload_marker, 1);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.evaluator_entries, 3);
    assert_replacement_trace(&run.proposal_trace, base, first_step, second_step);
    let mut first = base;
    first[0] += 2f64.powi(-26);
    first[3] += 0.01;
    let mut second = base;
    second[0] += 2f64.powi(-28);
    second[3] += 0.0025;
    let first_direction = std::array::from_fn(|i| fixture.scale[i] * first_step[i]);
    let second_direction = std::array::from_fn(|i| fixture.scale[i] * second_step[i]);
    assert_eq!(
        fixture.operation_log,
        replacement_log(base, first_direction, first, second_direction, second)
    );
}

#[test]
fn m1_stage2_first_two_governed_temperature_excesses_block_third_small_proposal() {
    let mut fixture = Fixture::new(FixtureKind::ReplacementWitnessThreeExcess);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let steps = governed_excess_steps();
    let result = super::m1_trust_region_controller_run_for_test(&mut fixture);
    let Ok(run) = result else {
        panic!("expected-red: the governed-excess control requires continued radius proposals");
    };
    assert_eq!(
        run.terminal_refusal,
        Some(super::M1TrustRegionRefusalKind::RadiusExhausted)
    );
    assert_eq!(run.entered_materializations, 0);
    assert!(fixture.materializer_inputs.is_empty());
    assert_eq!(fixture.payload_marker, 0);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert!(same_coordinate_bits(run.final_coordinates, base));
    assert_eq!(run.proposal_trace.len(), 11);
    for (index, proposal) in run.proposal_trace.iter().enumerate() {
        let candidate = std::array::from_fn(|coordinate| {
            base[coordinate] + fixture.scale[coordinate] * steps[index][coordinate]
        });
        let base_temperature = -2f64.powi(-25);
        let base_moisture = -0.5;
        let linear_temperature = base_temperature + steps[index][0] / 4.0;
        let linear_moisture = base_moisture + steps[index][3] / 4.0;
        // Candidate residuals use the independently fixed candidate
        // coordinates, including the evaluator's subtract-then-divide order;
        // a scaled-step component is not necessarily that binary64 z value.
        let candidate_temperature_z = (candidate[0] - base[0]) / fixture.scale[0];
        let candidate_moisture_z = (candidate[3] - base[3]) / fixture.scale[3];
        let candidate_temperature = -2f64.powi(-25) + candidate_temperature_z / 4.0;
        let candidate_moisture = -0.5 + candidate_moisture_z / 4.0
            - 2f64.powi(20) * candidate_moisture_z * candidate_moisture_z;
        let base_residual = std::array::from_fn(|coordinate| match coordinate {
            0 => base_temperature,
            3 => base_moisture,
            _ => 0.0,
        });
        let linear_residual = std::array::from_fn(|coordinate| match coordinate {
            0 => linear_temperature,
            3 => linear_moisture,
            _ => 0.0,
        });
        let candidate_residual = std::array::from_fn(|coordinate| match coordinate {
            0 => candidate_temperature,
            3 => candidate_moisture,
            _ => 0.0,
        });
        let sum_squares = |residual: [f64; N]| {
            let mut sum = 0.0;
            for value in residual {
                sum += value * value;
            }
            sum
        };
        let f2 = sum_squares(base_residual);
        let linear2 = sum_squares(linear_residual);
        let candidate2 = sum_squares(candidate_residual);
        let expected_predicted = 0.5 * (f2 - linear2);
        let expected_actual = (0.5 * f2) - (0.5 * candidate2);
        let expected_rho = expected_actual / expected_predicted;
        let expected_dynamic_max = candidate_temperature.abs().max(candidate_moisture.abs());
        assert!(same_coordinate_bits(proposal.scaled_step, steps[index]));
        assert!(same_coordinate_bits(
            proposal.stage1_observation.candidate_coordinates,
            candidate
        ));
        assert!(same_coordinate_bits(proposal.base_before, base));
        assert!(same_coordinate_bits(proposal.base_after, base));
        assert!(!proposal.installed);
        assert_eq!(proposal.disposition, super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk);
        assert_eq!(
            proposal.radius_before.to_bits(),
            stationarity_radius(index).to_bits()
        );
        assert_eq!(
            proposal.next_radius.to_bits(),
            stationarity_radius(index + 1).to_bits()
        );
        close(proposal.predicted_reduction, expected_predicted);
        close(proposal.actual_reduction.unwrap(), expected_actual);
        close(proposal.rho.unwrap(), expected_rho);
        close(
            proposal.dynamic_max_normalized_residual.unwrap(),
            expected_dynamic_max,
        );
        assert!(expected_dynamic_max > 0.5);
    }
    assert!(steps[0][0] > 2f64.powi(-27));
    assert!(steps[1][0] > 2f64.powi(-27));
    assert!(steps[2][0] <= 2f64.powi(-27));
    assert_eq!(fixture.evaluator_entries, 12);
    assert_eq!(
        fixture.operation_log,
        governed_excess_log(base, fixture.scale, &steps)
    );
}

#[test]
fn m1_stage2_full_domain_invalid_opens_one_domain_valid_replacement_witness() {
    let mut fixture = Fixture::new(FixtureKind::ReplacementWitnessFullDomainInvalid);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let steps = positive_replacement_steps();
    let run = super::m1_trust_region_controller_run_for_test(&mut fixture).unwrap();
    assert_eq!(run.proposals, 2);
    assert_eq!(run.accepted_updates, 0);
    assert_eq!(run.entered_materializations, 1);
    assert_eq!(run.terminal_refusal, None);
    assert!(same_coordinate_bits(run.final_coordinates, base));
    assert_eq!(fixture.materializer_inputs, vec![base]);
    assert_eq!(fixture.payload_marker, 1);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.evaluator_entries, 2);
    assert_eq!(
        run.proposal_trace[0].disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk
    );
    assert_eq!(
        run.proposal_trace[1].disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::RootAdmitted
    );
    assert_eq!(
        fixture.operation_log,
        witness_attempt_log(base, fixture.scale, &steps[..2], &[false, true], true)
    );
}

#[test]
fn m1_stage2_intervening_domain_invalid_does_not_consume_replacement_witness() {
    let mut fixture = Fixture::new(FixtureKind::ReplacementWitnessInterveningDomainInvalid);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let steps = governed_excess_steps();
    let run = super::m1_trust_region_controller_run_for_test(&mut fixture).unwrap();
    assert_eq!(run.proposals, 3);
    assert_eq!(run.accepted_updates, 0);
    assert_eq!(run.entered_materializations, 1);
    assert_eq!(run.terminal_refusal, None);
    assert!(same_coordinate_bits(run.final_coordinates, base));
    assert_eq!(fixture.materializer_inputs, vec![base]);
    assert_eq!(fixture.payload_marker, 1);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.evaluator_entries, 3);
    assert_eq!(
        run.proposal_trace.iter().map(|proposal| proposal.disposition).collect::<Vec<_>>(),
        vec![
            super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk,
            super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk,
            super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::RootAdmitted,
        ]
    );
    assert_eq!(
        fixture.operation_log,
        witness_attempt_log(base, fixture.scale, &steps[..3], &[true, false, true], true)
    );
}

#[test]
fn m1_stage2_full_incomplete_evaluation_is_terminal_and_opens_no_witness() {
    let mut fixture = Fixture::new(FixtureKind::ReplacementWitnessFullEvaluationError);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let steps = governed_excess_steps();
    let result = super::m1_trust_region_controller_run_for_test(&mut fixture);
    assert!(matches!(
        result,
        Err(M1TrustRegionControllerError::Own(
            "full replacement fixture evaluation error"
        ))
    ));
    assert_eq!(fixture.evaluator_entries, 2);
    assert!(fixture.materializer_inputs.is_empty());
    assert_eq!(fixture.payload_marker, 0);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(
        fixture.operation_log,
        witness_attempt_log(base, fixture.scale, &steps[..1], &[true], false)
    );
}

#[test]
fn m1_stage2_incomplete_full_governed_evaluation_is_terminal_and_opens_no_witness() {
    let mut fixture = Fixture::new(FixtureKind::ReplacementWitnessFullIncompleteGoverned);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let steps = governed_excess_steps();
    let result = super::m1_trust_region_controller_run_for_test(&mut fixture);
    assert!(matches!(
        result,
        Err(M1TrustRegionControllerError::Numerical(
            "nonfinite governed step operand"
        ))
    ));
    assert_eq!(fixture.evaluator_entries, 2);
    assert!(fixture.materializer_inputs.is_empty());
    assert_eq!(fixture.payload_marker, 0);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(
        fixture.operation_log,
        witness_attempt_log(base, fixture.scale, &steps[..1], &[true], false)
    );
}

#[test]
fn m1_stage2_first_domain_valid_replacement_terminal_error_cannot_open_third_witness() {
    let mut fixture = Fixture::new(FixtureKind::ReplacementWitnessReplacementEvaluationError);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let steps = governed_excess_steps();
    let result = super::m1_trust_region_controller_run_for_test(&mut fixture);
    assert!(matches!(
        result,
        Err(M1TrustRegionControllerError::Own(
            "replacement fixture evaluation error"
        ))
    ));
    assert_eq!(fixture.evaluator_entries, 3);
    assert!(fixture.materializer_inputs.is_empty());
    assert_eq!(fixture.payload_marker, 0);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(
        fixture.operation_log,
        witness_attempt_log(base, fixture.scale, &steps[..2], &[true, true], false)
    );
}

fn assert_exhausted_witness_initial_trace(
    run: &super::m1_trust_region_controller_interface::M1TrustRegionControllerRun,
    base: [f64; N],
    steps: &[[f64; N]; 2],
    new_base: [f64; N],
) {
    assert_eq!(run.proposal_trace.len(), 3);
    assert_eq!(run.proposals, 3);
    assert_eq!(run.initial_radius.to_bits(), 1.0f64.to_bits());
    assert_eq!(run.entered_svd_factorizations, 3);
    assert_eq!(run.accepted_updates, 1);
    assert_eq!(run.terminal_refusal, None);
    for (index, radius) in [1.0_f64, 0.25].into_iter().enumerate() {
        assert_eq!(
            run.proposal_trace[index].radius_before.to_bits(),
            radius.to_bits()
        );
        assert!(same_coordinate_bits(
            run.proposal_trace[index].scaled_step,
            steps[index]
        ));
        assert!(same_coordinate_bits(
            run.proposal_trace[index].base_before,
            base
        ));
    }
    assert_eq!(
        run.proposal_trace[0].disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk
    );
    assert_eq!(
        run.proposal_trace[1].disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::AcceptedUpdate
    );
    assert!(run.proposal_trace[1].installed);
    assert_eq!(
        run.proposal_trace[1].next_radius.to_bits(),
        0.5f64.to_bits()
    );
    assert!(!same_coordinate_bits(
        run.proposal_trace[1].base_after,
        base
    ));
    assert!(same_coordinate_bits(
        run.proposal_trace[1].base_after,
        new_base
    ));
}

fn assert_exhausted_witness_first_merits(
    run: &super::m1_trust_region_controller_interface::M1TrustRegionControllerRun,
    base: [f64; N],
    scale: [f64; N],
    steps: &[[f64; N]; 2],
    new_base: [f64; N],
) {
    let full_candidate = std::array::from_fn(|index| base[index] + scale[index] * steps[0][index]);
    let (full_predicted, full_actual, full_rho, full_dynamic_max) =
        exhausted_witness_merit(base, full_candidate, steps[0], base, scale);
    let (first_predicted, first_actual, first_rho, first_dynamic_max) =
        exhausted_witness_merit(base, new_base, steps[1], base, scale);
    let full = &run.proposal_trace[0];
    assert_exhausted_witness_full_rank(full, base, base, scale);
    assert_exhausted_witness_full_rank(&run.proposal_trace[1], base, base, scale);
    assert_eq!(full.predicted_reduction.to_bits(), full_predicted.to_bits());
    assert_eq!(
        full.actual_reduction.unwrap().to_bits(),
        full_actual.to_bits()
    );
    assert_eq!(full.rho.unwrap().to_bits(), full_rho.to_bits());
    assert_eq!(
        full.dynamic_max_normalized_residual.unwrap().to_bits(),
        full_dynamic_max.to_bits()
    );
    let first = &run.proposal_trace[1];
    assert_eq!(
        first.predicted_reduction.to_bits(),
        first_predicted.to_bits()
    );
    assert_eq!(
        first.actual_reduction.unwrap().to_bits(),
        first_actual.to_bits()
    );
    assert_eq!(first.rho.unwrap().to_bits(), first_rho.to_bits());
    assert_eq!(
        first.dynamic_max_normalized_residual.unwrap().to_bits(),
        first_dynamic_max.to_bits()
    );
    assert!(first_rho > 0.1);
    assert!(first_dynamic_max < 0.5);
    let base_wet_temperature = exhausted_witness_wet_temperature(base, base, scale);
    assert!(
        (exhausted_witness_wet_temperature(full_candidate, base, scale) - base_wet_temperature)
            .abs()
            > 1.0e-8
    );
    assert!(
        (exhausted_witness_wet_temperature(new_base, base, scale) - base_wet_temperature).abs()
            > 1.0e-8
    );
}

fn assert_exhausted_witness_reset(
    run: &super::m1_trust_region_controller_interface::M1TrustRegionControllerRun,
    base: [f64; N],
    scale: [f64; N],
    new_base: [f64; N],
) -> ([f64; N], [f64; N]) {
    let reset = &run.proposal_trace[2];
    let reset_step = exhausted_witness_reset_step();
    let reset_candidate =
        std::array::from_fn(|index| new_base[index] + scale[index] * reset_step[index]);
    assert!(same_coordinate_bits(reset.base_before, new_base));
    assert!(same_coordinate_bits(reset.base_after, new_base));
    assert!(same_coordinate_bits(reset.scaled_step, reset_step));
    assert!(same_coordinate_bits(
        reset.stage1_observation.scaled_step,
        reset_step
    ));
    assert!(same_coordinate_bits(
        reset.stage1_observation.candidate_coordinates,
        reset_candidate
    ));
    assert_eq!(reset.radius_before.to_bits(), 0.5f64.to_bits());
    assert_eq!(reset.next_radius.to_bits(), 0.5f64.to_bits());
    assert!(!reset.installed);
    assert_eq!(
        reset.disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::RootAdmitted
    );
    let (predicted, actual, rho, dynamic_max) =
        exhausted_witness_merit(new_base, reset_candidate, reset_step, base, scale);
    assert_exhausted_witness_full_rank(reset, new_base, base, scale);
    assert_eq!(reset.predicted_reduction.to_bits(), predicted.to_bits());
    assert_eq!(reset.actual_reduction.unwrap().to_bits(), actual.to_bits());
    assert_eq!(reset.rho.unwrap().to_bits(), rho.to_bits());
    assert_eq!(
        reset.dynamic_max_normalized_residual.unwrap().to_bits(),
        dynamic_max.to_bits()
    );
    assert!(
        (exhausted_witness_wet_temperature(reset_candidate, base, scale)
            - exhausted_witness_wet_temperature(new_base, base, scale))
        .abs()
            <= 1.0e-8
    );
    (reset_step, reset_candidate)
}

#[test]
fn m1_stage2_exhausted_witness_still_allows_ordinary_update_and_resets_at_new_base() {
    let mut fixture = Fixture::new(FixtureKind::ReplacementWitnessExhaustedOrdinaryUpdate);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let steps = exhausted_witness_steps();
    let run = super::m1_trust_region_controller_run_for_test(&mut fixture).unwrap();
    let new_base =
        std::array::from_fn(|index| base[index] + fixture.scale[index] * steps[1][index]);
    assert_exhausted_witness_initial_trace(&run, base, &steps, new_base);
    assert_exhausted_witness_first_merits(&run, base, fixture.scale, &steps, new_base);
    let (reset_step, reset_candidate) =
        assert_exhausted_witness_reset(&run, base, fixture.scale, new_base);
    assert_eq!(run.entered_materializations, 1);
    assert_eq!(
        run.materialized_coordinates.unwrap().map(f64::to_bits),
        new_base.map(f64::to_bits)
    );
    assert_eq!(fixture.materializer_inputs, vec![new_base]);
    assert_eq!(fixture.payload_marker, 1);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.evaluator_entries, 4);
    let first_direction = std::array::from_fn(|index| fixture.scale[index] * steps[0][index]);
    let first_candidate = std::array::from_fn(|index| base[index] + first_direction[index]);
    let second_direction = std::array::from_fn(|index| fixture.scale[index] * steps[1][index]);
    let second_candidate = new_base;
    let reset_direction = std::array::from_fn(|index| fixture.scale[index] * reset_step[index]);
    assert_eq!(
        fixture.operation_log,
        vec![
            FixtureOperation::Evaluate(base),
            FixtureOperation::SelectPhase {
                base,
                step: [0.0; N]
            },
            FixtureOperation::RawJacobian {
                base,
                selected_phase: None,
                predictor_direction: [0.0; N],
            },
            FixtureOperation::SelectPhase {
                base,
                step: first_direction
            },
            FixtureOperation::SelectedSideProbe {
                base,
                phase: 0,
                direction: first_direction
            },
            FixtureOperation::PreEvaluatorTrialDomain(first_candidate),
            FixtureOperation::Evaluate(first_candidate),
            FixtureOperation::SelectPhase {
                base,
                step: second_direction
            },
            FixtureOperation::SelectedSideProbe {
                base,
                phase: 0,
                direction: second_direction
            },
            FixtureOperation::PreEvaluatorTrialDomain(second_candidate),
            FixtureOperation::Evaluate(second_candidate),
            FixtureOperation::SelectPhase {
                base: new_base,
                step: [0.0; N]
            },
            FixtureOperation::RawJacobian {
                base: new_base,
                selected_phase: None,
                predictor_direction: [0.0; N],
            },
            FixtureOperation::SelectPhase {
                base: new_base,
                step: reset_direction
            },
            FixtureOperation::SelectedSideProbe {
                base: new_base,
                phase: 0,
                direction: reset_direction,
            },
            FixtureOperation::PreEvaluatorTrialDomain(reset_candidate),
            FixtureOperation::Evaluate(reset_candidate),
            FixtureOperation::Materialize(new_base),
        ]
    );
}

#[test]
fn m1_stage2_negative_humidity_excess_never_admits_or_materializes() {
    let mut fixture = Fixture::new(FixtureKind::NegativeHumidityExcess);
    let base = fixture.base;
    let sentinel = fixture.owner_sentinel;
    let expected_step = std::array::from_fn(|i| if i == 13 { -2e-9 } else { 0.0 });
    let candidate = std::array::from_fn(|i| base[i] + fixture.scale[i] * expected_step[i]);
    let direction = std::array::from_fn(|i| fixture.scale[i] * expected_step[i]);
    let transition = super::m1_trust_region_controller_transition_for_test(&mut fixture).unwrap();
    assert_eq!(
        transition.base_after.map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(
        transition.base_before.map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(
        transition.scaled_step.map(f64::to_bits),
        expected_step.map(f64::to_bits)
    );
    assert_eq!(transition.radius_before.to_bits(), 1.0f64.to_bits());
    assert_eq!(transition.next_radius.to_bits(), 0.25f64.to_bits());
    assert!(!transition.installed);
    assert_eq!(transition.entered_materializations, 0);
    assert_eq!(transition.disposition, super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk);
    assert!(fixture.materializer_inputs.is_empty());
    assert_eq!(fixture.payload_marker, 0);
    assert_eq!(fixture.owner_sentinel, sentinel);
    assert_eq!(fixture.evaluator_entries, 2);
    assert_eq!(
        fixture.operation_log,
        vec![
            FixtureOperation::Evaluate(base),
            FixtureOperation::SelectPhase {
                base,
                step: [0.0; N]
            },
            FixtureOperation::RawJacobian {
                base,
                selected_phase: None,
                predictor_direction: [0.0; N]
            },
            FixtureOperation::SelectPhase {
                base,
                step: direction
            },
            FixtureOperation::SelectedSideProbe {
                base,
                phase: 0,
                direction
            },
            FixtureOperation::PreEvaluatorTrialDomain(candidate),
            FixtureOperation::Evaluate(candidate),
        ]
    );
}

#[test]
fn m1_stage2_selected_side_error_precedes_pending_current_hydraulics() {
    let mut fixture = Fixture::new(FixtureKind::PhaseSelectedSideError);
    fixture.hydraulics.current_mode = HydraulicCurrentMode::PendingInitial;
    let result = super::m1_trust_region_controller_transition_for_test(&mut fixture);
    assert!(matches!(
        result,
        Err(M1TrustRegionControllerError::Own(
            "selected-side owning error"
        ))
    ));
    assert!(
        fixture
            .operation_log
            .iter()
            .any(|operation| matches!(operation, FixtureOperation::SelectedSideProbe { .. }))
    );
    assert!(!fixture.operation_log.iter().any(|operation| matches!(
        operation,
        FixtureOperation::CompleteCurrentHydraulics(_)
            | FixtureOperation::PreEvaluatorTrialDomain(_)
    )));
}

#[test]
fn m1_stage2_current_hydraulics_complete_after_selected_probe_before_trial() {
    let mut fixture = Fixture::new(FixtureKind::Ordinary(1.0));
    fixture.hydraulics.current_mode = HydraulicCurrentMode::PendingInitial;
    let transition = super::m1_trust_region_controller_transition_for_test(&mut fixture)
        .expect("completed pending base must admit the ordinary update");
    assert_eq!(
        transition.disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::AcceptedUpdate
    );
    assert!(transition.installed);
    assert_eq!(
        fixture
            .operation_log
            .iter()
            .filter(|operation| matches!(operation, FixtureOperation::Evaluate(_)))
            .count(),
        2
    );
    assert_eq!(
        fixture
            .operation_log
            .iter()
            .filter(|operation| matches!(operation, FixtureOperation::PreEvaluatorTrialDomain(_)))
            .count(),
        1
    );
    let selected_probe = fixture
        .operation_log
        .iter()
        .position(|operation| matches!(operation, FixtureOperation::SelectedSideProbe { .. }))
        .expect("controller must perform its selected-side probe");
    let hydraulics = fixture
        .operation_log
        .iter()
        .position(|operation| matches!(operation, FixtureOperation::CompleteCurrentHydraulics(_)))
        .expect("pending current hydraulics must complete");
    assert!(selected_probe < hydraulics);
    if let Some(trial_domain) = fixture
        .operation_log
        .iter()
        .position(|operation| matches!(operation, FixtureOperation::PreEvaluatorTrialDomain(_)))
    {
        assert!(hydraulics < trial_domain);
    }
    assert_eq!(fixture.hydraulics.completion_calls, 1);
}

#[test]
fn m1_stage2_pending_base_hydraulic_error_masks_no_prediction_and_trial() {
    let mut fixture = Fixture::new(FixtureKind::ConstrainedStationarity);
    let base = fixture.base;
    fixture.hydraulics.current_mode = HydraulicCurrentMode::PendingInitial;
    fixture.hydraulics.completion_mode = HydraulicCompletionMode::ErrorOnCall(1);
    let result = super::m1_trust_region_controller_transition_for_test(&mut fixture);
    assert!(matches!(
        result,
        Err(M1TrustRegionControllerError::Own(
            "pending hydraulic completion error"
        ))
    ));
    assert_eq!(fixture.evaluator_entries, 1);
    assert_eq!(fixture.hydraulics.completion_calls, 1);
    assert_eq!(fixture.payload_marker, 0);
    assert!(fixture.materializer_inputs.is_empty());
    assert!(!fixture.operation_log.iter().any(|operation| matches!(
        operation,
        FixtureOperation::PreEvaluatorTrialDomain(_) | FixtureOperation::Materialize(_)
    )));
    assert_eq!(
        fixture.operation_log.last(),
        Some(&FixtureOperation::CompleteCurrentHydraulics(base))
    );
}

#[test]
fn m1_stage2_pending_base_left_pending_fails_closed_before_trial() {
    let mut fixture = Fixture::new(FixtureKind::Ordinary(1.0));
    fixture.hydraulics.current_mode = HydraulicCurrentMode::PendingInitial;
    fixture.hydraulics.completion_mode = HydraulicCompletionMode::LeavePending;
    let result = super::m1_trust_region_controller_transition_for_test(&mut fixture);
    assert!(matches!(
        result,
        Err(M1TrustRegionControllerError::IncompleteScope)
    ));
    assert_eq!(fixture.hydraulics.completion_calls, 1);
    assert!(!fixture.operation_log.iter().any(|operation| matches!(
        operation,
        FixtureOperation::PreEvaluatorTrialDomain(_) | FixtureOperation::Materialize(_)
    )));
}

#[test]
fn m1_stage2_pending_candidate_hydraulics_fail_closed() {
    let mut fixture = Fixture::new(FixtureKind::Ordinary(1.0));
    fixture.hydraulics.current_mode = HydraulicCurrentMode::PendingCandidates;
    let result = super::m1_trust_region_controller_transition_for_test(&mut fixture);
    assert!(matches!(
        result,
        Err(M1TrustRegionControllerError::IncompleteScope)
    ));
    assert_eq!(fixture.hydraulics.completion_calls, 0);
    assert_eq!(fixture.evaluator_entries, 2);
    assert!(
        fixture
            .operation_log
            .iter()
            .any(|operation| matches!(operation, FixtureOperation::PreEvaluatorTrialDomain(_)))
    );
    assert!(fixture.materializer_inputs.is_empty());
}

#[test]
fn m1_stage2_pending_hydraulics_complete_once_per_base_and_reset_after_nonroot_update() {
    let mut fixture = Fixture::new(FixtureKind::PhaseAcceptedUpdateReset);
    let base = fixture.base;
    fixture.hydraulics.current_mode = HydraulicCurrentMode::PendingInitial;
    fixture.hydraulics.post_update_mode = HydraulicPostUpdateMode::ResetPending;
    fixture.hydraulics.completion_mode = HydraulicCompletionMode::ErrorOnCall(2);
    let result = super::m1_trust_region_controller_run_for_test(&mut fixture);
    assert!(matches!(
        result,
        Err(M1TrustRegionControllerError::Own(
            "pending hydraulic completion error"
        ))
    ));
    assert_eq!(fixture.hydraulics.completion_calls, 2);
    let completions: Vec<_> = fixture
        .operation_log
        .iter()
        .filter_map(|operation| match operation {
            FixtureOperation::CompleteCurrentHydraulics(coordinates) => Some(*coordinates),
            _ => None,
        })
        .collect();
    assert_eq!(completions.len(), 2);
    assert_eq!(completions[0].map(f64::to_bits), base.map(f64::to_bits));
    assert_ne!(
        completions[0].map(f64::to_bits),
        completions[1].map(f64::to_bits)
    );
    for coordinates in completions {
        let probe = fixture
            .operation_log
            .iter()
            .position(|operation| matches!(operation, FixtureOperation::SelectedSideProbe { base, .. } if same_coordinate_bits(*base, coordinates)))
            .expect("each pending base must finalize its selected-side probe first");
        let completion = fixture
            .operation_log
            .iter()
            .position(|operation| matches!(operation, FixtureOperation::CompleteCurrentHydraulics(base) if same_coordinate_bits(*base, coordinates)))
            .expect("each pending base must complete exactly once");
        assert!(probe < completion);
    }
    assert_eq!(
        fixture
            .operation_log
            .iter()
            .filter(|operation| matches!(operation, FixtureOperation::PreEvaluatorTrialDomain(_)))
            .count(),
        1
    );
    assert!(fixture.materializer_inputs.is_empty());
}
