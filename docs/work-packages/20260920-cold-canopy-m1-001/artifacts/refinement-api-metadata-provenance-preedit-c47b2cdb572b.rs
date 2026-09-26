//! Private frozen-weight arithmetic shared by detached Stage 1 controls.

use std::cmp::Ordering;

#[cfg(test)]
use std::cell::{Cell, RefCell};

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
use crate::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics;

use super::{
    M1TrustRegionBallEndpoint, M1TrustRegionBallObservation, M1TrustRegionCorrectionPosition,
    M1TrustRegionJacobiInput, M1TrustRegionJacobiObservation, M1TrustRegionJacobiPair,
    M1TrustRegionLambdaEvaluation, M1TrustRegionRefinementGuardStage,
    M1TrustRegionRefinementReason, M1TrustRegionRefusalKind,
};
use super::{M1TrustRegionMeritInput, M1TrustRegionMeritObservation};

const TWO_POW_53: f64 = 9_007_199_254_740_992.0;

#[cfg(test)]
thread_local! {
    static REFINEMENT_FROZEN_WEIGHTED_MERIT_ENTRIES: Cell<u32> = const { Cell::new(0) };
}

#[cfg(test)]
pub(crate) fn refinement_frozen_weighted_merit_entries_for_test() -> u32 {
    REFINEMENT_FROZEN_WEIGHTED_MERIT_ENTRIES.with(Cell::get)
}

// This is a deliberately opt-in, test-only flight recorder for one retained
// physical refusal.  It copies operands already produced by Stage 1; every
// write is best-effort so observation can never replace the typed result.
#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone, Debug)]
pub(crate) struct M1FacePivotCapture {
    pub(crate) base: [f64; 21],
    pub(crate) raw_residual: [f64; 21],
    pub(crate) raw_jacobian: [[f64; 21]; 21],
    pub(crate) normalizers: [f64; 21],
    pub(crate) scales: [f64; 21],
    pub(crate) lower: [f64; 21],
    pub(crate) upper: [f64; 21],
    pub(crate) weighted_residual: Vec<f64>,
    pub(crate) weighted_matrix: Vec<Vec<f64>>,
    pub(crate) initial_radius: f64,
    pub(crate) completed_subproblems: Vec<M1FacePivotSubproblemSummary>,
    pub(crate) operations: Vec<M1FacePivotCaptureOperation>,
    pub(crate) accepted_installs: Vec<M1FacePivotAcceptedInstall>,
    pub(crate) post_update_admissions: Vec<M1FacePivotPostUpdateAdmission>,
    pub(crate) lifecycle: Vec<&'static str>,
    pub(crate) refusal: Option<M1FacePivotCaptureRefusal>,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone, Debug)]
pub(crate) struct M1FacePivotSubproblemSummary {
    pub(crate) entered_factorizations: u8,
    pub(crate) completed_pivots: u8,
    pub(crate) completed_sweeps: u8,
    pub(crate) refused: bool,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone, Debug)]
pub(crate) struct M1FacePivotPostUpdateAdmission {
    pub(crate) previous_coordinates: [f64; 21],
    pub(crate) previous_raw: [f64; 21],
    pub(crate) previous_normalizers: [f64; 21],
    pub(crate) previous_wet_temperature_k: [Option<f64>; 2],
    pub(crate) previous_hydraulics: M1TrustRegionCurrentHydraulics,
    pub(crate) previous_beta: [f64; 4],
    pub(crate) previous_ci_pa: [f64; 4],
    pub(crate) current_coordinates: [f64; 21],
    pub(crate) current_raw: [f64; 21],
    pub(crate) current_normalizers: [f64; 21],
    pub(crate) current_wet_temperature_k: [Option<f64>; 2],
    pub(crate) current_hydraulics: M1TrustRegionCurrentHydraulics,
    pub(crate) current_beta: [f64; 4],
    pub(crate) current_ci_pa: [f64; 4],
    pub(crate) current_max: f64,
    pub(crate) governed_temperature: Option<f64>,
    pub(crate) governed_humidity: Option<f64>,
    pub(crate) governed_hydraulic: Option<f64>,
    pub(crate) governed_beta: Option<f64>,
    pub(crate) governed_ci: Option<[f64; 4]>,
    pub(crate) governed_pass: Option<bool>,
    pub(crate) governed_thresholds: [f64; 4],
    pub(crate) admitted: bool,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone, Debug)]
pub(crate) struct M1FacePivotAcceptedInstall {
    pub(crate) base_before: [f64; 21],
    pub(crate) raw_before: [f64; 21],
    pub(crate) normalizers_before: [f64; 21],
    pub(crate) base_after: [f64; 21],
    pub(crate) raw_after: [f64; 21],
    pub(crate) normalizers_after: [f64; 21],
    pub(crate) scaled_step: [f64; 21],
    pub(crate) rho: f64,
    pub(crate) predicted_reduction: f64,
    pub(crate) actual_reduction: f64,
    pub(crate) dynamic_max_before: f64,
    pub(crate) dynamic_max_after: f64,
    pub(crate) radius_before: f64,
    pub(crate) radius_after: f64,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone, Debug)]
pub(crate) struct M1FacePivotCaptureOperation {
    pub(crate) action: &'static str,
    pub(crate) p_before: [f64; 21],
    pub(crate) p_candidate: [f64; 21],
    pub(crate) conceptual_p_after: [f64; 21],
    pub(crate) scratch_p_at_refusal: Option<[f64; 21]>,
    pub(crate) applied: bool,
    pub(crate) lower_before: [bool; 21],
    pub(crate) upper_before: [bool; 21],
    pub(crate) free_before: [bool; 21],
    pub(crate) lower_after: [bool; 21],
    pub(crate) upper_after: [bool; 21],
    pub(crate) free_after: [bool; 21],
    pub(crate) free_ids: Vec<usize>,
    pub(crate) radius: f64,
    pub(crate) lambda: Option<f64>,
    pub(crate) weighted_residual: Option<[f64; 21]>,
    pub(crate) weighted_matrix: Option<Vec<Vec<f64>>>,
    pub(crate) crossing_theta: Option<f64>,
    pub(crate) selected_coordinate: Option<usize>,
    pub(crate) selected_lower: Option<bool>,
    pub(crate) residual: Option<[f64; 21]>,
    pub(crate) g: Option<[f64; 21]>,
    pub(crate) lambda_times_p: Option<[f64; 21]>,
    pub(crate) h: Option<[f64; 21]>,
    pub(crate) optimality: Option<super::M1TrustRegionOptimalityObservation>,
    pub(crate) entered_factorizations: u8,
    pub(crate) completed_pivots: u8,
    pub(crate) completed_sweeps: u8,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone, Debug)]
pub(crate) struct M1FacePivotCaptureRefusal {
    pub(crate) stage: &'static str,
    pub(crate) kind: M1TrustRegionRefusalKind,
    pub(crate) optimality_reason: Option<&'static str>,
    pub(crate) optimality_coordinate: Option<usize>,
    pub(crate) radius: Option<f64>,
    pub(crate) lambda: Option<f64>,
    pub(crate) free_ids: Vec<usize>,
    pub(crate) p: [f64; 21],
    pub(crate) lower: [bool; 21],
    pub(crate) upper: [bool; 21],
    pub(crate) free: [bool; 21],
    pub(crate) entered_factorizations: u8,
    pub(crate) completed_pivots: u8,
    pub(crate) completed_sweeps: u8,
    pub(crate) optimality: Option<super::M1TrustRegionOptimalityObservation>,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
thread_local! {
    static FACE_PIVOT_CAPTURE: RefCell<Option<M1FacePivotCapture>> = const { RefCell::new(None) };
    static FACE_PIVOT_CAPTURE_ENABLED: Cell<bool> = const { Cell::new(false) };
}

// BVLS-03 control-only recorder.  Values enter it only after the shared
// controller has produced them; it is deliberately not an input to either
// factorization or the refinement decision.
#[cfg(test)]
#[derive(Clone, Debug, Default)]
struct RefinementObservationTrace {
    live_state_entry: Option<[u64; 21]>,
    live_state_exit: Option<[u64; 21]>,
    p0: Option<[f64; 21]>,
    delta0: Option<Vec<f64>>,
    p1: Option<[f64; 21]>,
    p1_box_feasible: Option<bool>,
    residual1: Option<[f64; 21]>,
    delta1: Option<Vec<f64>>,
    p2: Option<[f64; 21]>,
    factor_identity: Option<usize>,
    factor_application_identities: Vec<usize>,
    factor_applications: u8,
    eligibility_scalar_operations: Option<u64>,
    eligibility_guard_events: Option<u64>,
    lower_mask_p0: Option<[bool; 21]>,
    upper_mask_p0: Option<[bool; 21]>,
    lower_mask_p1: Option<[bool; 21]>,
    upper_mask_p1: Option<[bool; 21]>,
    lower_mask_p2: Option<[bool; 21]>,
    upper_mask_p2: Option<[bool; 21]>,
    lower_mask_exit: Option<[bool; 21]>,
    upper_mask_exit: Option<[bool; 21]>,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
enum RefinementRawCandidateMutation {
    FinalBox {
        coordinate: usize,
        scaled_value: f64,
        original_lower: f64,
        original_upper: f64,
        base: f64,
        scale: f64,
        radius: f64,
    },
    ScratchRadius {
        coordinate: usize,
        scaled_value: f64,
        original_lower: f64,
        original_upper: f64,
        base: f64,
        scale: f64,
        radius: f64,
    },
    ActiveValue {
        coordinate: usize,
        scaled_value: f64,
        original_lower: f64,
        original_upper: f64,
        base: f64,
        scale: f64,
        radius: f64,
    },
}

#[cfg(test)]
thread_local! {
    static REFINEMENT_OBSERVATION: RefCell<Option<RefinementObservationTrace>> = const { RefCell::new(None) };
    static REFINEMENT_RAW_MUTATION: RefCell<Option<RefinementRawCandidateMutation>> = const { RefCell::new(None) };
}

#[cfg(test)]
fn refinement_mutate(position: u8, point: &mut [f64; 21]) -> Option<f64> {
    REFINEMENT_RAW_MUTATION
        .try_with(|slot| {
            let mutation = *slot.try_borrow().ok()?.as_ref()?;
            match mutation {
                RefinementRawCandidateMutation::ScratchRadius {
                    coordinate,
                    scaled_value,
                    radius,
                    ..
                } if position == 1 => {
                    point[coordinate] = scaled_value;
                    Some(radius)
                }
                RefinementRawCandidateMutation::FinalBox {
                    coordinate,
                    scaled_value,
                    ..
                } if position == 2 => {
                    point[coordinate] = scaled_value;
                    None
                }
                RefinementRawCandidateMutation::ActiveValue {
                    coordinate,
                    scaled_value,
                    ..
                } if position == 2 => {
                    point[coordinate] = scaled_value;
                    None
                }
                _ => None,
            }
        })
        .ok()
        .flatten()
}

#[cfg(test)]
fn refinement_observe(mut update: impl FnMut(&mut RefinementObservationTrace)) {
    let _ = REFINEMENT_OBSERVATION.try_with(|slot| {
        if let Ok(mut slot) = slot.try_borrow_mut() {
            if let Some(trace) = slot.as_mut() {
                update(trace);
            }
        }
    });
}

#[cfg(test)]
fn refinement_observation_begin() {
    let _ = REFINEMENT_OBSERVATION.try_with(|slot| {
        if let Ok(mut slot) = slot.try_borrow_mut() {
            *slot = Some(RefinementObservationTrace::default());
        }
    });
}

#[cfg(test)]
fn refinement_observation_take() -> Option<RefinementObservationTrace> {
    REFINEMENT_OBSERVATION
        .try_with(|slot| slot.try_borrow_mut().ok().and_then(|mut slot| slot.take()))
        .ok()
        .flatten()
}

#[cfg(test)]
struct RefinementObservationScope(Option<RefinementObservationTrace>);

#[cfg(test)]
impl RefinementObservationScope {
    fn begin() -> Self {
        let prior = REFINEMENT_OBSERVATION
            .try_with(|slot| {
                slot.try_borrow_mut()
                    .ok()
                    .and_then(|mut slot| slot.replace(RefinementObservationTrace::default()))
            })
            .ok()
            .flatten();
        Self(prior)
    }
}

#[cfg(test)]
impl Drop for RefinementObservationScope {
    fn drop(&mut self) {
        let _ = REFINEMENT_OBSERVATION.try_with(|slot| {
            if let Ok(mut slot) = slot.try_borrow_mut() {
                *slot = self.0.take();
            }
        });
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) struct M1FacePivotCaptureScope {
    prior_enabled: bool,
    prior_capture: Option<M1FacePivotCapture>,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
impl Drop for M1FacePivotCaptureScope {
    fn drop(&mut self) {
        let _ = FACE_PIVOT_CAPTURE_ENABLED.try_with(|enabled| enabled.set(self.prior_enabled));
        let _ = FACE_PIVOT_CAPTURE.try_with(|cell| {
            if let Ok(mut capture) = cell.try_borrow_mut() {
                *capture = self.prior_capture.take();
            }
        });
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_face_pivot_capture_begin_for_test() -> M1FacePivotCaptureScope {
    let prior_enabled = FACE_PIVOT_CAPTURE_ENABLED
        .try_with(Cell::get)
        .unwrap_or(false);
    let prior_capture = FACE_PIVOT_CAPTURE
        .try_with(|cell| cell.try_borrow().ok().and_then(|capture| capture.clone()))
        .ok()
        .flatten();
    let _ = FACE_PIVOT_CAPTURE_ENABLED.try_with(|enabled| enabled.set(true));
    let _ = FACE_PIVOT_CAPTURE.try_with(|cell| {
        if let Ok(mut capture) = cell.try_borrow_mut() {
            *capture = None;
        }
    });
    M1FacePivotCaptureScope {
        prior_enabled,
        prior_capture,
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_face_pivot_capture_enabled_for_test() -> bool {
    FACE_PIVOT_CAPTURE_ENABLED
        .try_with(Cell::get)
        .unwrap_or(false)
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_face_pivot_capture_take_for_test() -> Option<M1FacePivotCapture> {
    FACE_PIVOT_CAPTURE
        .try_with(|cell| {
            cell.try_borrow_mut()
                .ok()
                .and_then(|mut capture| capture.take())
        })
        .ok()
        .flatten()
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_bits<const N: usize>(output: &mut String, values: &[f64; N]) {
    use std::fmt::Write;

    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        let _ = write!(output, "\"{:016x}\"", value.to_bits());
    }
    output.push(']');
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_bool_bits<const N: usize>(output: &mut String, values: &[bool; N]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        output.push_str(if *value { "true" } else { "false" });
    }
    output.push(']');
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_optional_bits<const N: usize>(output: &mut String, values: Option<[f64; N]>) {
    match values {
        Some(values) => append_bits(output, &values),
        None => output.push_str("null"),
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_optional_bit(output: &mut String, value: Option<f64>) {
    use std::fmt::Write;

    match value {
        Some(value) => {
            let _ = write!(output, "\"{:016x}\"", value.to_bits());
        }
        None => output.push_str("null"),
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_optional_bits_array<const N: usize>(output: &mut String, values: &[Option<f64>; N]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        append_optional_bit(output, *value);
    }
    output.push(']');
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_hydraulics(output: &mut String, hydraulics: M1TrustRegionCurrentHydraulics) {
    match hydraulics {
        M1TrustRegionCurrentHydraulics::Pending => output.push_str("null"),
        M1TrustRegionCurrentHydraulics::Complete(values) => append_bits(output, &values),
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_named_bits<const N: usize>(output: &mut String, name: &str, values: &[f64; N]) {
    use std::fmt::Write;

    let _ = write!(output, "\"{name}\":");
    append_bits(output, values);
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_optimality_json(
    output: &mut String,
    observation: Option<&super::M1TrustRegionOptimalityObservation>,
) {
    use std::fmt::Write;

    let Some(observation) = observation else {
        output.push_str("null");
        return;
    };
    let _ = write!(
        output,
        "{{\"scalar_operations\":{},\"coordinates\":[",
        observation.scalar_operations
    );
    for (index, coordinate) in observation.coordinates.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        let _ = write!(
            output,
            "{{\"coordinate\":{},\"h\":\"{:016x}\",\"cbar\":\"{:016x}\",\"tau\":\"{:016x}\",\"class\":\"{:?}\"}}",
            coordinate.coordinate,
            coordinate.h.to_bits(),
            coordinate.cbar.to_bits(),
            coordinate.tau.to_bits(),
            coordinate.class
        );
    }
    output.push_str("],\"selected_release\":");
    match observation.selected_release {
        Some(release) => {
            let _ = write!(
                output,
                "{{\"coordinate\":{},\"lower\":{},\"ratio\":",
                release.coordinate, release.lower
            );
            append_optional_bit(output, release.ratio);
            let _ = write!(
                output,
                ",\"tied_coordinates\":{}}}",
                release.tied_coordinates
            );
        }
        None => output.push_str("null"),
    }
    output.push('}');
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_operation_json(output: &mut String, operation: &M1FacePivotCaptureOperation) {
    use std::fmt::Write;

    let _ = write!(
        output,
        "{{\"action\":\"{}\",\"applied\":{}",
        operation.action, operation.applied
    );
    for (name, values) in [
        ("p_before", &operation.p_before),
        ("p_candidate", &operation.p_candidate),
        ("conceptual_p_after", &operation.conceptual_p_after),
    ] {
        output.push(',');
        append_named_bits(output, name, values);
    }
    let _ = write!(output, ",\"scratch_p_at_refusal\":");
    append_optional_bits(output, operation.scratch_p_at_refusal);
    for (name, values) in [
        ("lower_before", &operation.lower_before),
        ("upper_before", &operation.upper_before),
        ("free_before", &operation.free_before),
        ("lower_after", &operation.lower_after),
        ("upper_after", &operation.upper_after),
        ("free_after", &operation.free_after),
    ] {
        let _ = write!(output, ",\"{name}\":");
        append_bool_bits(output, values);
    }
    let _ = write!(
        output,
        ",\"free_ids\":{:?},\"radius\":\"{:016x}\",\"lambda\":",
        operation.free_ids,
        operation.radius.to_bits()
    );
    match operation.lambda {
        Some(value) => {
            let _ = write!(output, "\"{:016x}\"", value.to_bits());
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"weighted_residual\":");
    append_optional_bits(output, operation.weighted_residual);
    output.push_str(",\"weighted_matrix\":");
    match &operation.weighted_matrix {
        Some(matrix) => {
            output.push('[');
            for (index, row) in matrix.iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                output.push('[');
                append_slice_bits(output, row);
                output.push(']');
            }
            output.push(']');
        }
        None => output.push_str("null"),
    }
    output.push_str(",\"theta\":");
    append_optional_bit(output, operation.crossing_theta);
    let _ = write!(output, ",\"coordinate\":");
    match operation.selected_coordinate {
        Some(value) => {
            let _ = write!(output, "{value}");
        }
        None => output.push_str("null"),
    }
    let _ = write!(output, ",\"lower\":");
    match operation.selected_lower {
        Some(value) => {
            let _ = write!(output, "{value}");
        }
        None => output.push_str("null"),
    }
    let _ = write!(
        output,
        ",\"entered\":{},\"pivots\":{},\"sweeps\":{}",
        operation.entered_factorizations, operation.completed_pivots, operation.completed_sweeps
    );
    for (name, values) in [
        ("residual", operation.residual),
        ("g", operation.g),
        ("lambda_times_p", operation.lambda_times_p),
        ("h", operation.h),
    ] {
        let _ = write!(output, ",\"{name}\":");
        append_optional_bits(output, values);
    }
    output.push_str(",\"optimality\":");
    append_optimality_json(output, operation.optimality.as_ref());
    output.push('}');
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_capture_input_json(output: &mut String, capture: &M1FacePivotCapture) {
    use std::fmt::Write;

    for (index, (name, values)) in [
        ("base", &capture.base),
        ("raw_residual", &capture.raw_residual),
        ("normalizers", &capture.normalizers),
        ("scales", &capture.scales),
        ("lower", &capture.lower),
        ("upper", &capture.upper),
    ]
    .iter()
    .enumerate()
    {
        if index != 0 {
            output.push(',');
        }
        append_named_bits(output, name, values);
    }
    output.push_str(",\"raw_jacobian\":[");
    for (index, row) in capture.raw_jacobian.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        append_bits(output, row);
    }
    output.push_str("],\"weighted_residual\":[");
    append_slice_bits(output, &capture.weighted_residual);
    output.push_str("],\"weighted_matrix\":[");
    for (index, row) in capture.weighted_matrix.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        output.push('[');
        append_slice_bits(output, row);
        output.push(']');
    }
    let _ = write!(
        output,
        "],\"initial_radius\":\"{:016x}\",\"operations\":[",
        capture.initial_radius.to_bits()
    );
    for (index, operation) in capture.operations.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        append_operation_json(output, operation);
    }
    output.push_str("],\"completed_subproblems\":[");
    for (index, summary) in capture.completed_subproblems.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        let _ = write!(
            output,
            "{{\"entered\":{},\"pivots\":{},\"sweeps\":{},\"refused\":{}}}",
            summary.entered_factorizations,
            summary.completed_pivots,
            summary.completed_sweeps,
            summary.refused
        );
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_slice_bits(output: &mut String, values: &[f64]) {
    use std::fmt::Write;

    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        let _ = write!(output, "\"{:016x}\"", value.to_bits());
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_installs_json(output: &mut String, installs: &[M1FacePivotAcceptedInstall]) {
    use std::fmt::Write;

    output.push_str("],\"accepted_installs\":[");
    for (index, install) in installs.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        output.push('{');
        for (number, (name, values)) in [
            ("base_before", &install.base_before),
            ("raw_before", &install.raw_before),
            ("normalizers_before", &install.normalizers_before),
            ("base_after", &install.base_after),
            ("raw_after", &install.raw_after),
            ("normalizers_after", &install.normalizers_after),
            ("scaled_step", &install.scaled_step),
        ]
        .iter()
        .enumerate()
        {
            if number != 0 {
                output.push(',');
            }
            append_named_bits(output, name, values);
        }
        let _ = write!(
            output,
            ",\"rho\":\"{:016x}\",\"predicted\":\"{:016x}\",\"actual\":\"{:016x}\",\"max_before\":\"{:016x}\",\"max_after\":\"{:016x}\",\"radius_before\":\"{:016x}\",\"radius_after\":\"{:016x}\"}}",
            install.rho.to_bits(),
            install.predicted_reduction.to_bits(),
            install.actual_reduction.to_bits(),
            install.dynamic_max_before.to_bits(),
            install.dynamic_max_after.to_bits(),
            install.radius_before.to_bits(),
            install.radius_after.to_bits()
        );
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_admissions_json(output: &mut String, admissions: &[M1FacePivotPostUpdateAdmission]) {
    use std::fmt::Write;

    output.push_str("],\"post_update_admissions\":[");
    for (index, admission) in admissions.iter().enumerate() {
        if index != 0 {
            output.push(',');
        }
        output.push('{');
        for (number, (name, values)) in [
            ("previous_coordinates", &admission.previous_coordinates),
            ("previous_raw", &admission.previous_raw),
            ("previous_normalizers", &admission.previous_normalizers),
            ("current_coordinates", &admission.current_coordinates),
            ("current_raw", &admission.current_raw),
            ("current_normalizers", &admission.current_normalizers),
        ]
        .iter()
        .enumerate()
        {
            if number != 0 {
                output.push(',');
            }
            append_named_bits(output, name, values);
        }
        output.push(',');
        append_named_bits(output, "thresholds", &admission.governed_thresholds);
        output.push(',');
        append_named_bits(output, "previous_beta", &admission.previous_beta);
        output.push(',');
        append_named_bits(output, "previous_ci_pa", &admission.previous_ci_pa);
        output.push(',');
        append_named_bits(output, "current_beta", &admission.current_beta);
        output.push(',');
        append_named_bits(output, "current_ci_pa", &admission.current_ci_pa);
        output.push_str(",\"previous_hydraulics\":");
        append_hydraulics(output, admission.previous_hydraulics);
        output.push_str(",\"current_hydraulics\":");
        append_hydraulics(output, admission.current_hydraulics);
        output.push_str(",\"previous_wet_temperature_k\":");
        append_optional_bits_array(output, &admission.previous_wet_temperature_k);
        output.push_str(",\"current_wet_temperature_k\":");
        append_optional_bits_array(output, &admission.current_wet_temperature_k);
        let _ = write!(
            output,
            ",\"current_max\":\"{:016x}\",\"temperature\":",
            admission.current_max.to_bits()
        );
        append_optional_bit(output, admission.governed_temperature);
        output.push_str(",\"humidity\":");
        append_optional_bit(output, admission.governed_humidity);
        output.push_str(",\"hydraulic\":");
        append_optional_bit(output, admission.governed_hydraulic);
        output.push_str(",\"beta\":");
        append_optional_bit(output, admission.governed_beta);
        output.push_str(",\"ci\":");
        append_optional_bits(output, admission.governed_ci);
        let _ = write!(
            output,
            ",\"pass\":{},\"admitted\":{}}}",
            admission
                .governed_pass
                .map_or("null".to_owned(), |value| value.to_string()),
            admission.admitted
        );
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn append_refusal_json(output: &mut String, capture: &M1FacePivotCapture) {
    use std::fmt::Write;

    let _ = write!(
        output,
        "],\"lifecycle\":{:?},\"refusal\":",
        capture.lifecycle
    );
    match &capture.refusal {
        Some(refusal) => {
            let _ = write!(
                output,
                "{{\"stage\":\"{}\",\"kind\":\"{:?}\",\"reason\":",
                refusal.stage, refusal.kind,
            );
            match refusal.optimality_reason {
                Some(reason) => output.push_str(
                    &serde_json::to_string(reason).expect("static optimality reason serializes"),
                ),
                None => output.push_str("null"),
            }
            output.push_str(",\"coordinate\":");
            match refusal.optimality_coordinate {
                Some(coordinate) => {
                    let _ = write!(output, "{coordinate}");
                }
                None => output.push_str("null"),
            }
            output.push_str(",\"radius\":");
            append_optional_bit(output, refusal.radius);
            output.push_str(",\"lambda\":");
            append_optional_bit(output, refusal.lambda);
            let _ = write!(
                output,
                ",\"free_ids\":{:?},\"entered\":{},\"pivots\":{},\"sweeps\":{},\"p\":",
                refusal.free_ids,
                refusal.entered_factorizations,
                refusal.completed_pivots,
                refusal.completed_sweeps
            );
            append_bits(output, &refusal.p);
            output.push_str(",\"lower\":");
            append_bool_bits(output, &refusal.lower);
            output.push_str(",\"upper\":");
            append_bool_bits(output, &refusal.upper);
            output.push_str(",\"free\":");
            append_bool_bits(output, &refusal.free);
            output.push_str(",\"optimality\":");
            append_optimality_json(output, refusal.optimality.as_ref());
            output.push('}');
        }
        None => output.push_str("null"),
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_face_pivot_capture_json_for_test(capture: Option<&M1FacePivotCapture>) -> String {
    let Some(capture) = capture else {
        return "null".to_owned();
    };
    let mut output = String::from("{");
    append_capture_input_json(&mut output, capture);
    append_installs_json(&mut output, &capture.accepted_installs);
    append_admissions_json(&mut output, &capture.post_update_admissions);
    append_refusal_json(&mut output, capture);
    output.push('}');
    output
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn populated_face_pivot_operation_for_test() -> M1FacePivotCaptureOperation {
    M1FacePivotCaptureOperation {
        action: "activate-cap",
        p_before: [0.0; 21],
        p_candidate: [1.0; 21],
        conceptual_p_after: [0.0; 21],
        scratch_p_at_refusal: Some([1.0; 21]),
        applied: false,
        lower_before: [false; 21],
        upper_before: [false; 21],
        free_before: [true; 21],
        lower_after: [false; 21],
        upper_after: [false; 21],
        free_after: [true; 21],
        free_ids: vec![1],
        radius: 1.0,
        lambda: Some(-0.0),
        weighted_residual: None,
        weighted_matrix: None,
        crossing_theta: Some(f64::INFINITY),
        selected_coordinate: Some(1),
        selected_lower: Some(true),
        residual: Some([0.5; 21]),
        g: Some([1.0; 21]),
        lambda_times_p: Some([2.0; 21]),
        h: Some([3.0; 21]),
        optimality: None,
        entered_factorizations: 43,
        completed_pivots: 42,
        completed_sweeps: 7,
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn populated_face_pivot_capture_for_test() -> M1FacePivotCapture {
    M1FacePivotCapture {
        base: [f64::NEG_INFINITY; 21],
        raw_residual: [-0.0; 21],
        raw_jacobian: [[0.0; 21]; 21],
        normalizers: [1.0; 21],
        scales: [1.0; 21],
        lower: [0.0; 21],
        upper: [1.0; 21],
        weighted_residual: vec![f64::INFINITY],
        weighted_matrix: vec![vec![f64::NAN]],
        initial_radius: 1.0,
        completed_subproblems: vec![M1FacePivotSubproblemSummary {
            entered_factorizations: 1,
            completed_pivots: 0,
            completed_sweeps: 0,
            refused: false,
        }],
        operations: vec![populated_face_pivot_operation_for_test()],
        accepted_installs: vec![M1FacePivotAcceptedInstall {
            base_before: [0.0; 21],
            raw_before: [1.0; 21],
            normalizers_before: [2.0; 21],
            base_after: [3.0; 21],
            raw_after: [4.0; 21],
            normalizers_after: [5.0; 21],
            scaled_step: [6.0; 21],
            rho: 0.5,
            predicted_reduction: 1.0,
            actual_reduction: 0.5,
            dynamic_max_before: 1.0,
            dynamic_max_after: 0.5,
            radius_before: 1.0,
            radius_after: 2.0,
        }],
        post_update_admissions: vec![M1FacePivotPostUpdateAdmission {
            previous_coordinates: [0.0; 21],
            previous_raw: [1.0; 21],
            previous_normalizers: [2.0; 21],
            previous_wet_temperature_k: [Some(273.15), None],
            previous_hydraulics: M1TrustRegionCurrentHydraulics::Complete([1.0; 8]),
            previous_beta: [1.0; 4],
            previous_ci_pa: [2.0; 4],
            current_coordinates: [3.0; 21],
            current_raw: [4.0; 21],
            current_normalizers: [5.0; 21],
            current_wet_temperature_k: [Some(274.15), Some(275.15)],
            current_hydraulics: M1TrustRegionCurrentHydraulics::Complete([2.0; 8]),
            current_beta: [3.0; 4],
            current_ci_pa: [4.0; 4],
            current_max: 0.5,
            governed_temperature: Some(1.0),
            governed_humidity: Some(2.0),
            governed_hydraulic: Some(3.0),
            governed_beta: Some(4.0),
            governed_ci: Some([5.0; 4]),
            governed_pass: Some(false),
            governed_thresholds: [1.0; 4],
            admitted: false,
        }],
        lifecycle: vec!["test"],
        refusal: Some(M1FacePivotCaptureRefusal {
            stage: "cap",
            kind: M1TrustRegionRefusalKind::FacePivotLimit,
            optimality_reason: None,
            optimality_coordinate: None,
            radius: None,
            lambda: None,
            free_ids: Vec::new(),
            p: [1.0; 21],
            lower: [false; 21],
            upper: [true; 21],
            free: [false; 21],
            entered_factorizations: 43,
            completed_pivots: 42,
            completed_sweeps: 7,
            optimality: None,
        }),
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[test]
fn m1_face_pivot_populated_capture_json_parses() {
    let mut capture = populated_face_pivot_capture_for_test();
    let json = m1_face_pivot_capture_json_for_test(Some(&capture));
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("valid populated JSON");
    assert_eq!(parsed["operations"][0]["coordinate"], 1);
    assert_eq!(parsed["operations"][0]["lower"], true);
    assert_eq!(parsed["post_update_admissions"][0]["pass"], false);
    assert_eq!(parsed["refusal"]["kind"], "FacePivotLimit");
    assert!(parsed["refusal"]["reason"].is_null());
    assert!(parsed["refusal"]["coordinate"].is_null());
    assert!(parsed["refusal"]["radius"].is_null());
    assert!(parsed["refusal"]["lambda"].is_null());

    let refusal = capture.refusal.as_mut().expect("populated refusal");
    refusal.optimality_reason = Some("optimality_nonfinite");
    refusal.optimality_coordinate = Some(3);
    refusal.radius = Some(-0.0);
    refusal.lambda = Some(f64::INFINITY);
    let json = m1_face_pivot_capture_json_for_test(Some(&capture));
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("valid populated JSON");
    assert_eq!(parsed["refusal"]["reason"], "optimality_nonfinite");
    assert_eq!(parsed["refusal"]["coordinate"], 3);
    assert_eq!(parsed["refusal"]["radius"], "8000000000000000");
    assert_eq!(parsed["refusal"]["lambda"], "7ff0000000000000");
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn record_face_pivot_capture(f: impl FnOnce(&mut M1FacePivotCapture)) {
    let enabled = FACE_PIVOT_CAPTURE_ENABLED
        .try_with(Cell::get)
        .unwrap_or(false);
    if !enabled {
        return;
    }
    let _ = FACE_PIVOT_CAPTURE.try_with(|cell| {
        if let Ok(mut capture) = cell.try_borrow_mut() {
            if let Some(capture) = capture.as_mut() {
                f(capture);
            }
        }
    });
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn seed_kkt_transition_capture_for_test(
    weighted_matrix: &[Vec<f64>],
    weighted_residual: &[f64; 21],
    lower: [f64; 21],
    upper: [f64; 21],
) {
    if !m1_face_pivot_capture_enabled_for_test() {
        return;
    }
    let raw_jacobian =
        std::array::from_fn(|row| std::array::from_fn(|column| weighted_matrix[row][column]));
    let _ = FACE_PIVOT_CAPTURE.try_with(|cell| {
        if let Ok(mut capture) = cell.try_borrow_mut() {
            *capture = Some(M1FacePivotCapture {
                base: [0.0; 21],
                raw_residual: *weighted_residual,
                raw_jacobian,
                normalizers: [1.0; 21],
                scales: [1.0; 21],
                lower,
                upper,
                weighted_residual: weighted_residual.to_vec(),
                weighted_matrix: weighted_matrix.to_vec(),
                initial_radius: 1.0,
                completed_subproblems: Vec::new(),
                operations: Vec::new(),
                accepted_installs: Vec::new(),
                post_update_admissions: Vec::new(),
                lifecycle: Vec::new(),
                refusal: None,
            });
        }
    });
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_face_pivot_capture_accepted_install_for_test(install: M1FacePivotAcceptedInstall) {
    record_face_pivot_capture(|capture| {
        capture.accepted_installs.push(install);
    });
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_face_pivot_capture_post_update_admission_for_test(
    admission: M1FacePivotPostUpdateAdmission,
) {
    record_face_pivot_capture(|capture| {
        capture.post_update_admissions.push(admission);
    });
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
pub(crate) fn m1_face_pivot_capture_lifecycle_for_test(event: &'static str) {
    record_face_pivot_capture(|capture| capture.lifecycle.push(event));
}

pub(crate) fn checked_sum_squares(values: &[f64]) -> Result<f64, &'static str> {
    let mut sum = 0.0;
    for &value in values {
        if !value.is_finite() {
            return Err("nonfinite operand");
        }
        let square = value * value;
        if !square.is_finite() {
            return Err("nonfinite square");
        }
        sum += square;
        if !sum.is_finite() {
            return Err("nonfinite sum");
        }
    }
    Ok(sum)
}

pub(crate) fn weighted_system(
    raw: &[f64],
    jacobian: &[&[f64]],
    normalizers: &[f64],
    scales: &[f64],
) -> Result<(Vec<f64>, Vec<Vec<f64>>), &'static str> {
    if raw.len() != jacobian.len() || raw.len() != normalizers.len() {
        return Err("dimension mismatch");
    }
    let mut f = Vec::with_capacity(raw.len());
    let mut a = Vec::with_capacity(raw.len());
    for row in 0..raw.len() {
        if jacobian[row].len() != scales.len()
            || !raw[row].is_finite()
            || !normalizers[row].is_finite()
            || normalizers[row] <= 0.0
        {
            return Err("invalid weighted system input");
        }
        let weight = 1.0 / normalizers[row];
        let weighted_residual = weight * raw[row];
        if !weight.is_finite() || !weighted_residual.is_finite() {
            return Err("nonfinite weighting");
        }
        let mut a_row = Vec::with_capacity(scales.len());
        for col in 0..scales.len() {
            if !jacobian[row][col].is_finite() || !scales[col].is_finite() {
                return Err("nonfinite weighted system operand");
            }
            let weighted_jacobian = weight * jacobian[row][col];
            let coefficient = weighted_jacobian * scales[col];
            if !weighted_jacobian.is_finite() || !coefficient.is_finite() {
                return Err("nonfinite weighted system product");
            }
            a_row.push(coefficient);
        }
        f.push(weighted_residual);
        a.push(a_row);
    }
    Ok((f, a))
}

pub(crate) fn frozen_weighted_merit(
    raw: &[f64],
    jacobian: &[&[f64]],
    normalizers: &[f64],
    scales: &[f64],
    step: &[f64],
    candidate_raw: &[f64],
    moving_normalizers: &[f64],
) -> Result<(f64, f64, f64, f64), &'static str> {
    #[cfg(test)]
    REFINEMENT_FROZEN_WEIGHTED_MERIT_ENTRIES.with(|count| count.set(count.get().saturating_add(1)));
    if raw.len() != jacobian.len()
        || raw.len() != normalizers.len()
        || raw.len() != candidate_raw.len()
        || raw.len() != moving_normalizers.len()
    {
        return Err("dimension mismatch");
    }
    let (f, a) = weighted_system(raw, jacobian, normalizers, scales)?;
    let vector_input = MeritVectorInput {
        raw,
        normalizers,
        scales,
        step,
        candidate_raw,
        moving_normalizers,
        weighted_residual: &f,
        weighted_matrix: &a,
    };
    let vectors = merit_vectors(&vector_input)?;
    merit_reductions(&f, vectors)
}

struct MeritVectorInput<'a> {
    raw: &'a [f64],
    normalizers: &'a [f64],
    scales: &'a [f64],
    step: &'a [f64],
    candidate_raw: &'a [f64],
    moving_normalizers: &'a [f64],
    weighted_residual: &'a [f64],
    weighted_matrix: &'a [Vec<f64>],
}

struct MeritVectors {
    linear: Vec<f64>,
    candidate: Vec<f64>,
    moving: Vec<f64>,
}

fn merit_vectors(input: &MeritVectorInput<'_>) -> Result<MeritVectors, &'static str> {
    let MeritVectorInput {
        raw,
        normalizers,
        scales,
        step,
        candidate_raw,
        moving_normalizers,
        weighted_residual,
        weighted_matrix,
    } = input;
    let mut linear = Vec::with_capacity(raw.len());
    let mut candidate = Vec::with_capacity(raw.len());
    let mut moving = Vec::with_capacity(raw.len());
    for row in 0..raw.len() {
        if !moving_normalizers[row].is_finite() || moving_normalizers[row] <= 0.0 {
            return Err("invalid normalizer");
        }
        if scales.len() != step.len() {
            return Err("dimension mismatch");
        }
        let weight = 1.0 / normalizers[row];
        let moving_weight = 1.0 / moving_normalizers[row];
        if !weight.is_finite() || !moving_weight.is_finite() {
            return Err("nonfinite weight");
        }
        let weighted = weighted_residual[row];
        let frozen_candidate = weight * candidate_raw[row];
        let moving_candidate = moving_weight * candidate_raw[row];
        if !weighted.is_finite() || !frozen_candidate.is_finite() || !moving_candidate.is_finite() {
            return Err("nonfinite weighting");
        }
        let mut ap = 0.0;
        for col in 0..scales.len() {
            if !step[col].is_finite() {
                return Err("nonfinite operand");
            }
            let term = weighted_matrix[row][col] * step[col];
            if !term.is_finite() {
                return Err("nonfinite intermediate");
            }
            ap += term;
            if !ap.is_finite() {
                return Err("nonfinite accumulation");
            }
        }
        let linear_value = weighted + ap;
        if !linear_value.is_finite() {
            return Err("nonfinite residual");
        }
        linear.push(linear_value);
        candidate.push(frozen_candidate);
        moving.push(moving_candidate);
    }
    Ok(MeritVectors {
        linear,
        candidate,
        moving,
    })
}

fn merit_reductions(
    f: &[f64],
    vectors: MeritVectors,
) -> Result<(f64, f64, f64, f64), &'static str> {
    let MeritVectors {
        linear,
        candidate,
        moving,
    } = vectors;
    let f2 = checked_sum_squares(f)?;
    let linear2 = checked_sum_squares(linear.as_slice())?;
    let candidate2 = checked_sum_squares(candidate.as_slice())?;
    let predicted = predicted_reduction_from_squares(f2, linear2);
    let actual = (0.5 * f2) - (0.5 * candidate2);
    let moving_merit = 0.5 * checked_sum_squares(moving.as_slice())?;
    let rho = actual / predicted;
    if !f2.is_finite()
        || !linear2.is_finite()
        || !candidate2.is_finite()
        || !predicted.is_finite()
        || !actual.is_finite()
        || !moving_merit.is_finite()
        || !rho.is_finite()
    {
        return Err("nonfinite merit");
    }
    Ok((predicted, actual, rho, moving_merit))
}

pub(crate) fn predicted_reduction_from_squares(f2: f64, linear2: f64) -> f64 {
    0.5 * (f2 - linear2)
}

pub(crate) fn m1_trust_region_frozen_merit_for_test(
    _input: &super::M1CoupledColumnInput,
    _trial: &super::M1CoupledColumnTrial,
    value: M1TrustRegionMeritInput,
) -> Result<M1TrustRegionMeritObservation, &'static str> {
    let rows: [&[f64]; 2] = [&value.raw_jacobian[0], &value.raw_jacobian[1]];
    let (predicted_reduction, actual_reduction, rho, moving_weight_trial_merit) =
        frozen_weighted_merit(
            &value.raw_residual,
            &rows,
            &value.frozen_normalizers,
            &value.coordinate_scales,
            &value.scaled_step,
            &value.candidate_raw_residual,
            &value.moving_candidate_normalizers,
        )?;
    Ok(M1TrustRegionMeritObservation {
        predicted_reduction,
        actual_reduction,
        rho,
        moving_weight_trial_merit,
    })
}

struct JacobiFactor {
    // Keep the input matrix as well as its rotated columns.  The frozen method
    // defines U as A_free * V / sigma, with this multiplication accumulated in
    // ascending free-coordinate order; the rotated B is not a substitute in
    // binary64 arithmetic.
    a: Vec<Vec<f64>>,
    b: Vec<Vec<f64>>,
    v: Vec<Vec<f64>>,
    sigma: Vec<f64>,
    order: Vec<usize>,
    visits: Vec<M1TrustRegionJacobiPair>,
    rotations: usize,
    sweeps: u8,
}

pub(crate) struct LambdaTrace {
    pub(crate) initial_bracket: [f64; 2],
    pub(crate) bracket: [f64; 2],
    pub(crate) bracket_evaluations: Vec<(f64, f64)>,
    pub(crate) bisection_evaluations: Vec<(f64, f64)>,
    pub(crate) lambda: f64,
    pub(crate) step: Vec<f64>,
    pub(crate) natural_step: Vec<f64>,
    pub(crate) pre_hook_step: Vec<f64>,
    pub(crate) post_hook_step: Vec<f64>,
    pub(crate) norm: f64,
    pub(crate) gap: f64,
    pub(crate) tolerance: f64,
}

impl Default for LambdaTrace {
    fn default() -> Self {
        Self {
            initial_bracket: [0.0, 0.0],
            bracket: [0.0, 0.0],
            bracket_evaluations: Vec::new(),
            bisection_evaluations: Vec::new(),
            lambda: 0.0,
            step: Vec::new(),
            natural_step: Vec::new(),
            pre_hook_step: Vec::new(),
            post_hook_step: Vec::new(),
            norm: f64::NAN,
            gap: f64::NAN,
            tolerance: f64::NAN,
        }
    }
}

#[derive(Default)]
struct RefinementWorkLedger {
    scalar_operations: u64,
    guard_events: u64,
    attempted_entries: u16,
    denied_entries: u16,
    first_residual_operations: u64,
    second_dot2_operations: u64,
    factor_application_operations: u64,
    coordinate_update_operations: u64,
    whole_kkt_operations: u64,
    whole_kkt_guard_events: u64,
    upward_enclosure_operations: u64,
    upward_enclosure_guard_events: u64,
}

impl RefinementWorkLedger {
    fn scalar(&mut self) {
        self.scalar_operations += 1;
    }

    fn guard(&mut self) {
        self.guard_events += 1;
    }

    fn absorb_dot2(&mut self, counters: RefinementDot2Counters) {
        self.scalar_operations += u64::from(counters.arithmetic);
        self.guard_events += u64::from(counters.guards);
    }
}

const REFINEMENT_SCALAR_CAP: u64 = 4_118_712;
const REFINEMENT_GUARD_CAP: u64 = 12_367_101;

fn refinement_required_work(free_coordinates: usize) -> (u64, u64) {
    let m = free_coordinates as u64;
    let scalar = 12_306 + 88 * m * m + 138 * m + 39_816 + 1_827 + 129;
    let guards = 3 * scalar + 213 + 2 * m;
    (scalar, guards)
}

fn lambda_step(
    factor: &JacobiFactor,
    y: &[f64],
    lambda: f64,
    mut refinement_work: Option<&mut RefinementWorkLedger>,
) -> Result<Vec<f64>, &'static str> {
    macro_rules! scalar {
        () => {
            if let Some(work) = refinement_work.as_deref_mut() {
                work.scalar();
            }
        };
    }
    macro_rules! guard {
        () => {
            if let Some(work) = refinement_work.as_deref_mut() {
                work.guard();
            }
        };
    }
    guard!();
    if y.len() != factor.b.len() || !lambda.is_finite() || lambda < 0.0 {
        return Err("invalid lambda input");
    }
    let cols = factor.sigma.len();
    let mut coefficients = vec![0.0; cols];
    for sorted in 0..cols {
        let j = factor.order[sorted];
        let sigma = factor.sigma[j];
        let mut uty = 0.0;
        for (row, &y_value) in y.iter().enumerate() {
            let mut av = 0.0;
            for free in 0..cols {
                scalar!();
                let term = factor.a[row][free] * factor.v[free][j];
                guard!();
                if !term.is_finite() {
                    return Err("nonfinite A_free V product");
                }
                scalar!();
                av += term;
                guard!();
                if !av.is_finite() {
                    return Err("nonfinite A_free V sum");
                }
            }
            scalar!();
            let u = av / sigma;
            guard!();
            if !u.is_finite() || !y_value.is_finite() {
                return Err("nonfinite U");
            }
            scalar!();
            let term = u * y_value;
            guard!();
            if !term.is_finite() {
                return Err("nonfinite Uty product");
            }
            scalar!();
            uty += term;
            guard!();
            if !uty.is_finite() {
                return Err("nonfinite Uty sum");
            }
        }
        scalar!();
        let sigma_square = sigma * sigma;
        scalar!();
        let denom = sigma_square + lambda;
        scalar!();
        let gain = sigma / denom;
        scalar!();
        // The frozen grouped expression contains both the multiplication and
        // the unary negation; charge both without changing its rounding.
        scalar!();
        let coefficient = -(gain * uty);
        guard!();
        if !denom.is_finite() || !gain.is_finite() || !coefficient.is_finite() {
            return Err("nonfinite lambda coefficient");
        }
        coefficients[j] = coefficient;
    }
    let mut step = vec![0.0; cols];
    for (out, output) in step.iter_mut().enumerate() {
        let mut sum = 0.0;
        for sorted in 0..cols {
            let j = factor.order[sorted];
            scalar!();
            let term = factor.v[out][j] * coefficients[j];
            guard!();
            if !term.is_finite() {
                return Err("nonfinite V product");
            }
            scalar!();
            sum += term;
            guard!();
            if !sum.is_finite() {
                return Err("nonfinite V sum");
            }
        }
        *output = sum;
    }
    Ok(step)
}

fn lambda_norm(
    step: &[f64],
    mut refinement_work: Option<&mut RefinementWorkLedger>,
) -> Result<f64, &'static str> {
    let mut sum = 0.0;
    for &x in step {
        if let Some(work) = refinement_work.as_deref_mut() {
            work.scalar();
        }
        let sq = x * x;
        if let Some(work) = refinement_work.as_deref_mut() {
            work.guard();
        }
        if !sq.is_finite() {
            return Err("nonfinite norm");
        }
        if let Some(work) = refinement_work.as_deref_mut() {
            work.scalar();
        }
        sum += sq;
        if let Some(work) = refinement_work.as_deref_mut() {
            work.guard();
        }
        if !sum.is_finite() {
            return Err("nonfinite norm");
        }
    }
    if let Some(work) = refinement_work.as_deref_mut() {
        work.scalar();
        work.guard();
    }
    let norm = sum.sqrt();
    if norm.is_finite() {
        Ok(norm)
    } else {
        Err("nonfinite norm")
    }
}

fn svd_lambda_ball(
    factor: &JacobiFactor,
    y: &[f64],
    radius: f64,
    override_endpoint: Option<&[f64]>,
    trace: &mut LambdaTrace,
) -> Result<(), M1TrustRegionRefusalKind> {
    if !radius.is_finite() || radius <= 0.0 {
        return Err(M1TrustRegionRefusalKind::DegenerateFace);
    }
    let zero =
        lambda_step(factor, y, 0.0, None).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    let n0 = lambda_norm(&zero, None).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    trace.initial_bracket = [0.0, 0.0];
    trace.bracket = [0.0, 0.0];
    trace.bracket_evaluations.push((0.0, n0));
    if n0 <= radius {
        trace.lambda = 0.0;
        trace.step = zero.clone();
        trace.natural_step = zero.clone();
        trace.pre_hook_step = zero.clone();
        trace.post_hook_step = zero;
        trace.norm = n0;
        trace.gap = radius - n0;
        trace.tolerance = (2_f64.powi(-40) * radius).max(64.0 * f64::EPSILON * radius.max(1.0));
        return Ok(());
    }
    let mut lo = 0.0;
    let mut hi = 1.0;
    let mut high =
        lambda_step(factor, y, hi, None).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    let mut nh = lambda_norm(&high, None).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    trace.initial_bracket = [0.0, hi];
    trace.bracket = [0.0, hi];
    trace.bracket_evaluations.push((hi, nh));
    for _ in 0..48 {
        if nh <= radius {
            break;
        }
        lo = hi;
        hi *= 4.0;
        high =
            lambda_step(factor, y, hi, None).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
        nh = lambda_norm(&high, None).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
        trace.bracket_evaluations.push((hi, nh));
    }
    if nh > radius {
        return Err(M1TrustRegionRefusalKind::LambdaBracket);
    }
    for _ in 0..48 {
        let mid = (lo + hi) * 0.5;
        let step = lambda_step(factor, y, mid, None)
            .map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
        let norm = lambda_norm(&step, None).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
        trace.bisection_evaluations.push((mid, norm));
        if norm <= radius {
            hi = mid;
            high = step;
        } else {
            lo = mid;
        }
    }
    trace.bracket = [lo, hi];
    trace.lambda = hi;
    trace.natural_step = high.clone();
    trace.pre_hook_step = high;
    trace.post_hook_step =
        override_endpoint.map_or_else(|| trace.pre_hook_step.clone(), <[f64]>::to_vec);
    trace.step = trace.post_hook_step.clone();
    let norm =
        lambda_norm(&trace.step, None).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    let gap = radius - norm;
    let tolerance = (2_f64.powi(-40) * radius).max(64.0 * f64::EPSILON * radius.max(1.0));
    trace.norm = norm;
    trace.gap = gap;
    trace.tolerance = tolerance;
    if norm > radius || gap > tolerance {
        return Err(M1TrustRegionRefusalKind::BallAccuracy);
    }
    Ok(())
}

pub(crate) fn m1_trust_region_validate_ball_endpoint_for_test(
    endpoint: M1TrustRegionBallEndpoint,
) -> M1TrustRegionBallObservation {
    // This is deliberately only a data adapter around the shared factorization
    // and ball solve.  The test seam may replace the final coordinate after the
    // canonical bisections; it does not select a verdict or alter their count.
    let mut observed_sweeps = 0;
    let factor = jacobi_factor(vec![vec![endpoint.a]], &[0], 0, &mut observed_sweeps);
    let mut trace = LambdaTrace::default();
    let (factorization_entered, kind) = match factor {
        Ok(factor) => {
            let y = [endpoint.f];
            let result = svd_lambda_ball(
                &factor,
                &y,
                endpoint.radius,
                endpoint.endpoint_override.as_ref().map(|x| &x[..]),
                &mut trace,
            );
            (true, result.err())
        }
        Err("rank deficient") => (true, Some(M1TrustRegionRefusalKind::RankDeficient)),
        Err("no convergence") => (true, Some(M1TrustRegionRefusalKind::SvdNoConvergence)),
        Err(_) => (true, Some(M1TrustRegionRefusalKind::SvdNonFinite)),
    };
    let evaluation = |(lambda, norm): &(f64, f64)| M1TrustRegionLambdaEvaluation {
        lambda: *lambda,
        norm: *norm,
        feasible: *norm <= endpoint.radius,
    };
    let scalar = |values: &[f64]| values.first().copied().unwrap_or(f64::NAN);
    M1TrustRegionBallObservation {
        factorization_entered,
        bracket_endpoint_evaluations: trace.bracket_evaluations.iter().map(evaluation).collect(),
        bisection_evaluations: trace.bisection_evaluations.iter().map(evaluation).collect(),
        bracket_initial: trace.initial_bracket,
        bracket_final: trace.bracket,
        final_upper_lambda: trace.lambda,
        natural_upper_endpoint: scalar(&trace.natural_step),
        pre_hook_endpoint: scalar(&trace.pre_hook_step),
        endpoint_hook_observed: endpoint.endpoint_override,
        post_hook_endpoint: [scalar(&trace.post_hook_step)],
        computed_norm: trace.norm,
        computed_gap: trace.gap,
        computed_tolerance: trace.tolerance,
        accepted: kind.is_none(),
        kind,
    }
}

fn jacobi_column_products(
    matrix: &[Vec<f64>],
    left: usize,
    right: usize,
) -> Result<(f64, f64, f64), &'static str> {
    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut gamma = 0.0;
    for row in matrix {
        let left_value = row[left];
        let right_value = row[right];
        let left_square = left_value * left_value;
        let right_square = right_value * right_value;
        let product = left_value * right_value;
        if !left_square.is_finite() || !right_square.is_finite() || !product.is_finite() {
            return Err("nonfinite SVD product");
        }
        alpha += left_square;
        beta += right_square;
        gamma += product;
        if !alpha.is_finite() || !beta.is_finite() || !gamma.is_finite() {
            return Err("nonfinite SVD accumulation");
        }
    }
    Ok((alpha, beta, gamma))
}

fn rotate_jacobi_columns(
    matrix: &mut [Vec<f64>],
    left: usize,
    right: usize,
    cosine: f64,
    sine: f64,
) -> Result<(), &'static str> {
    for row in matrix {
        let left_value = row[left];
        let right_value = row[right];
        let new_left = cosine * left_value - sine * right_value;
        let new_right = sine * left_value + cosine * right_value;
        if !new_left.is_finite() || !new_right.is_finite() {
            return Err("nonfinite rotation product");
        }
        row[left] = new_left;
        row[right] = new_right;
    }
    Ok(())
}

fn sorted_singular_values(
    rotated_b: &[Vec<f64>],
    free_ids: &[usize],
) -> Result<(Vec<f64>, Vec<usize>), &'static str> {
    let cols = free_ids.len();
    let mut sigma = vec![0.0; cols];
    for col in 0..cols {
        for row in rotated_b {
            let square = row[col] * row[col];
            if !square.is_finite() {
                return Err("nonfinite singular value");
            }
            sigma[col] += square;
            if !sigma[col].is_finite() {
                return Err("nonfinite singular value");
            }
        }
        sigma[col] = sigma[col].sqrt();
        if !sigma[col].is_finite() {
            return Err("nonfinite singular value");
        }
    }
    let max = sigma.iter().copied().fold(0.0, f64::max);
    if max == 0.0 || sigma.iter().any(|s| *s <= 2_f64.powi(-40) * max) {
        return Err("rank deficient");
    }
    let mut order: Vec<usize> = (0..cols).collect();
    order.sort_by(|a, b| {
        sigma[*b]
            .total_cmp(&sigma[*a])
            .then_with(|| free_ids[*a].cmp(&free_ids[*b]))
    });
    Ok((sigma, order))
}

fn jacobi_factor(
    mut b: Vec<Vec<f64>>,
    free_ids: &[usize],
    completed_sweeps: u8,
    work_counter: &mut u8,
) -> Result<JacobiFactor, &'static str> {
    let a = b.clone();
    let cols = free_ids.len();
    if cols == 0
        || b.iter()
            .any(|row| row.len() != cols || row.iter().any(|x| !x.is_finite()))
    {
        return Err("invalid SVD input");
    }
    let mut v = vec![vec![0.0; cols]; cols];
    for (index, row) in v.iter_mut().enumerate() {
        row[index] = 1.0;
    }
    let mut visits = Vec::new();
    let mut rotations = 0usize;
    let mut sweeps = completed_sweeps;
    *work_counter = completed_sweeps;
    if completed_sweeps >= 64 {
        return Err("no convergence");
    }
    for sweep in (completed_sweeps + 1)..=64u8 {
        let mut any = false;
        for left in 0..cols {
            for right in left + 1..cols {
                let (alpha, beta, gamma) = jacobi_column_products(&b, left, right)?;
                if !alpha.is_finite() || !beta.is_finite() || !gamma.is_finite() {
                    return Err("nonfinite SVD intermediate");
                }
                let product = alpha * beta;
                let root = product.sqrt();
                let threshold = 2_f64.powi(-48) * root;
                if !product.is_finite() || !root.is_finite() || !threshold.is_finite() {
                    return Err("nonfinite SVD threshold");
                }
                let rotate = gamma.abs() > threshold;
                visits.push(M1TrustRegionJacobiPair {
                    left,
                    right,
                    rotated: rotate,
                    sweep,
                });
                if rotate {
                    let numerator = beta - alpha;
                    let denominator = 2.0 * gamma;
                    let tau = numerator / denominator;
                    let hypot = tau.hypot(1.0);
                    let divisor = tau.abs() + hypot;
                    let tangent = if tau == 0.0 {
                        1.0
                    } else {
                        tau.signum() / divisor
                    };
                    let cosine = 1.0 / tangent.hypot(1.0);
                    let sine = cosine * tangent;
                    if !numerator.is_finite()
                        || !denominator.is_finite()
                        || !tau.is_finite()
                        || !hypot.is_finite()
                        || !divisor.is_finite()
                        || !tangent.is_finite()
                        || !cosine.is_finite()
                        || !sine.is_finite()
                    {
                        return Err("nonfinite rotation");
                    }
                    rotate_jacobi_columns(&mut b, left, right, cosine, sine)?;
                    rotate_jacobi_columns(&mut v, left, right, cosine, sine)?;
                    rotations += 1;
                    any = true;
                }
            }
        }
        sweeps = sweep;
        *work_counter = sweeps;
        if !any {
            break;
        }
    }
    if sweeps == 64
        && visits
            .iter()
            .rev()
            .take(cols * (cols - 1) / 2)
            .any(|p| p.rotated)
    {
        return Err("no convergence");
    }
    let (sigma, order) = sorted_singular_values(&b, free_ids)?;
    Ok(JacobiFactor {
        a,
        b,
        v,
        sigma,
        order,
        visits,
        rotations,
        sweeps,
    })
}

pub(crate) fn m1_trust_region_jacobi_observation_for_test(
    input: M1TrustRegionJacobiInput,
) -> M1TrustRegionJacobiObservation {
    let mut observed_sweeps = 0;
    let factor = jacobi_factor(
        input.matrix.iter().map(|row| row.to_vec()).collect(),
        &input.free_ids,
        0,
        &mut observed_sweeps,
    )
    .expect("Jacobi fixture");
    let column = |m: &Vec<Vec<f64>>, c| [m[0][c], m[1][c], m[2][c]];
    let mut sorted_b = [[0.0; 3]; 3];
    let mut sorted_v = [[0.0; 3]; 3];
    let mut sorted_sigma = [0.0; 3];
    let mut sorted_ids = [0; 3];
    let mut order = [0; 3];
    for k in 0..3 {
        let c = factor.order[k];
        order[k] = c;
        sorted_sigma[k] = factor.sigma[c];
        sorted_ids[k] = input.free_ids[c];
        sorted_b[k] = column(&factor.b, c);
        sorted_v[k] = column(&factor.v, c);
    }
    M1TrustRegionJacobiObservation {
        completed_sweeps: factor.sweeps,
        pair_visits: factor.visits,
        rotations: factor.rotations,
        b_columns: [
            column(&factor.b, 0),
            column(&factor.b, 1),
            column(&factor.b, 2),
        ],
        v_columns: [
            column(&factor.v, 0),
            column(&factor.v, 1),
            column(&factor.v, 2),
        ],
        unsorted_singular_values: [factor.sigma[0], factor.sigma[1], factor.sigma[2]],
        sorted_singular_values: sorted_sigma,
        sorted_free_ids: sorted_ids,
        sorted_from_unsorted: order,
        sorted_b_columns: sorted_b,
        sorted_v_columns: sorted_v,
    }
}

fn subproblem_refusal(
    kind: M1TrustRegionRefusalKind,
    attempted_subproblems: u8,
    completed_jacobi_sweeps: u8,
    completed_face_pivots: u8,
    entered_svd_factorizations: u8,
    raw_residual_max: f64,
) -> super::M1TrustRegionSubproblemRefusal {
    super::M1TrustRegionSubproblemRefusal {
        kind,
        optimality_reason: None,
        optimality_coordinate: None,
        optimality_radius: None,
        optimality_lambda: None,
        attempted_subproblems,
        completed_jacobi_sweeps,
        completed_face_pivots,
        entered_svd_factorizations,
        raw_residual_max,
        work: Box::new(super::M1TrustRegionWorkObservation {
            stage1_calls: 1,
            entered_svd_factorizations: u32::from(entered_svd_factorizations),
            completed_face_pivots: u32::from(completed_face_pivots),
            ..Default::default()
        }),
        refinement: None,
        #[cfg(feature = "m1-trust-region-physical-stage")]
        budget_refusal: None,
    }
}

struct PreparedSubproblem {
    raw_max: f64,
    weighted_residual: Vec<f64>,
    weighted_matrix: Vec<Vec<f64>>,
    scaled_lower: [f64; 21],
    scaled_upper: [f64; 21],
}

struct KktValues {
    residual: [f64; 21],
    gradient: [f64; 21],
    lambda_times_step: [f64; 21],
    h: [f64; 21],
}

struct SubproblemState {
    p: [f64; 21],
    lower_active: [bool; 21],
    upper_active: [bool; 21],
    free: [bool; 21],
    initial_free: [bool; 21],
    events: Vec<super::M1TrustRegionKktEvent>,
    entered: u8,
    completed_pivots: u8,
    completed_sweeps: u8,
    total_jacobi_sweeps: u32,
    refinement_work: RefinementWorkLedger,
}

impl SubproblemState {
    fn new(input: &super::M1TrustRegionSubproblemInput) -> Self {
        let mut free = [false; 21];
        for (col, is_free) in free.iter_mut().enumerate() {
            *is_free = input.lower[col].partial_cmp(&input.upper[col]) != Some(Ordering::Equal);
        }
        Self {
            p: [0.0; 21],
            lower_active: [false; 21],
            upper_active: [false; 21],
            initial_free: free,
            free,
            events: Vec::new(),
            entered: 0,
            completed_pivots: input.test_work.completed_face_pivots,
            completed_sweeps: input.test_work.completed_jacobi_sweeps,
            total_jacobi_sweeps: 0,
            refinement_work: RefinementWorkLedger::default(),
        }
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn capture_operation(state: &SubproblemState, operation: M1FacePivotCaptureOperation) {
    record_face_pivot_capture(|capture| {
        capture.operations.push(M1FacePivotCaptureOperation {
            action: operation.action,
            p_before: operation.p_before,
            p_candidate: operation.p_candidate,
            conceptual_p_after: operation.conceptual_p_after,
            scratch_p_at_refusal: operation.scratch_p_at_refusal,
            applied: operation.applied,
            lower_before: operation.lower_before,
            upper_before: operation.upper_before,
            free_before: operation.free_before,
            lower_after: state.lower_active,
            upper_after: state.upper_active,
            free_after: state.free,
            free_ids: operation.free_ids,
            radius: operation.radius,
            lambda: operation.lambda,
            weighted_residual: operation.weighted_residual,
            weighted_matrix: operation.weighted_matrix,
            crossing_theta: operation.crossing_theta,
            selected_coordinate: operation.selected_coordinate,
            selected_lower: operation.selected_lower,
            residual: operation.residual,
            g: operation.g,
            lambda_times_p: operation.lambda_times_p,
            h: operation.h,
            optimality: operation.optimality,
            entered_factorizations: state.entered,
            completed_pivots: state.completed_pivots,
            completed_sweeps: state.completed_sweeps,
        });
    });
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn capture_refusal(
    refusal_stage: &'static str,
    kind: M1TrustRegionRefusalKind,
    state: &mut SubproblemState,
) {
    record_face_pivot_capture(|capture| {
        capture.refusal = Some(M1FacePivotCaptureRefusal {
            stage: refusal_stage,
            kind,
            optimality_reason: None,
            optimality_coordinate: None,
            radius: None,
            lambda: None,
            free_ids: Vec::new(),
            p: state.p,
            lower: state.lower_active,
            upper: state.upper_active,
            free: state.free,
            entered_factorizations: state.entered,
            completed_pivots: state.completed_pivots,
            completed_sweeps: state.completed_sweeps,
            optimality: None,
        });
        capture
            .completed_subproblems
            .push(M1FacePivotSubproblemSummary {
                entered_factorizations: state.entered,
                completed_pivots: state.completed_pivots,
                completed_sweeps: state.completed_sweeps,
                refused: true,
            });
    });
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn capture_optimality_failure(
    state: &mut SubproblemState,
    solution: &FaceSolution,
    prepared: &PreparedSubproblem,
    values: Option<&KktValues>,
    assessment: Option<&OptimalityAssessment>,
    failure: OptimalityFailure,
) {
    record_face_pivot_capture(|capture| {
        if let Some(values) = values {
            capture.operations.push(M1FacePivotCaptureOperation {
                action: "optimality-refusal",
                p_before: state.p,
                p_candidate: state.p,
                conceptual_p_after: state.p,
                scratch_p_at_refusal: None,
                applied: false,
                lower_before: state.lower_active,
                upper_before: state.upper_active,
                free_before: state.free,
                lower_after: state.lower_active,
                upper_after: state.upper_active,
                free_after: state.free,
                free_ids: solution.free_ids.clone(),
                radius: solution.radius,
                lambda: Some(solution.trace.lambda),
                weighted_residual: Some(
                    prepared
                        .weighted_residual
                        .clone()
                        .try_into()
                        .expect("fixed KKT dimension"),
                ),
                weighted_matrix: Some(prepared.weighted_matrix.clone()),
                crossing_theta: None,
                selected_coordinate: None,
                selected_lower: None,
                residual: Some(values.residual),
                g: Some(values.gradient),
                lambda_times_p: Some(values.lambda_times_step),
                h: Some(values.h),
                optimality: assessment.map(|value| value.observation.clone()),
                entered_factorizations: state.entered,
                completed_pivots: state.completed_pivots,
                completed_sweeps: state.completed_sweeps,
            });
        }
        capture.refusal = Some(M1FacePivotCaptureRefusal {
            stage: "kkt-optimality",
            kind: M1TrustRegionRefusalKind::TrustRegionOptimalityIndeterminate,
            optimality_reason: Some(failure.reason),
            optimality_coordinate: Some(failure.coordinate),
            radius: Some(solution.radius),
            lambda: Some(solution.trace.lambda),
            free_ids: solution.free_ids.clone(),
            p: state.p,
            lower: state.lower_active,
            upper: state.upper_active,
            free: state.free,
            entered_factorizations: state.entered,
            completed_pivots: state.completed_pivots,
            completed_sweeps: state.completed_sweeps,
            optimality: assessment.map(|value| value.observation.clone()),
        });
    });
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn capture_successful_subproblem(state: &SubproblemState) {
    record_face_pivot_capture(|capture| {
        capture
            .completed_subproblems
            .push(M1FacePivotSubproblemSummary {
                entered_factorizations: state.entered,
                completed_pivots: state.completed_pivots,
                completed_sweeps: state.completed_sweeps,
                refused: false,
            });
    });
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn capture_activation(
    state: &SubproblemState,
    solution: &FaceSolution,
    p_before: [f64; 21],
    p_candidate: [f64; 21],
    masks_before: ([bool; 21], [bool; 21], [bool; 21]),
    crossing: (f64, usize, bool),
    applied: bool,
) {
    if !m1_face_pivot_capture_enabled_for_test() {
        return;
    }
    let (crossing_theta, selected_coordinate, selected_lower) = crossing;
    let (lower_before, upper_before, free_before) = masks_before;
    capture_operation(
        state,
        M1FacePivotCaptureOperation {
            action: if applied { "activate" } else { "activate-cap" },
            p_before,
            p_candidate,
            conceptual_p_after: if applied { state.p } else { p_before },
            scratch_p_at_refusal: (!applied).then_some(p_candidate),
            applied,
            lower_before,
            upper_before,
            free_before,
            lower_after: [false; 21],
            upper_after: [false; 21],
            free_after: [false; 21],
            free_ids: solution.free_ids.clone(),
            radius: solution.radius,
            lambda: Some(solution.trace.lambda),
            weighted_residual: None,
            weighted_matrix: None,
            crossing_theta: Some(crossing_theta),
            selected_coordinate: Some(selected_coordinate),
            selected_lower: Some(selected_lower),
            residual: None,
            g: None,
            lambda_times_p: None,
            h: None,
            optimality: None,
            entered_factorizations: 0,
            completed_pivots: 0,
            completed_sweeps: 0,
        },
    );
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
struct ReleaseCapture<'a> {
    solution: &'a FaceSolution,
    prepared: &'a PreparedSubproblem,
    values: &'a KktValues,
    assessment: &'a OptimalityAssessment,
    p_before: [f64; 21],
    masks_before: ([bool; 21], [bool; 21], [bool; 21]),
    selected: (usize, bool),
    applied: bool,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn capture_release(state: &SubproblemState, capture: &ReleaseCapture<'_>) {
    if !m1_face_pivot_capture_enabled_for_test() {
        return;
    }
    let (lower_before, upper_before, free_before) = capture.masks_before;
    let (selected_coordinate, selected_lower) = capture.selected;
    capture_operation(
        state,
        M1FacePivotCaptureOperation {
            action: if capture.applied {
                "release"
            } else {
                "release-cap"
            },
            p_before: capture.p_before,
            p_candidate: state.p,
            conceptual_p_after: capture.p_before,
            scratch_p_at_refusal: (!capture.applied).then_some(state.p),
            applied: capture.applied,
            lower_before,
            upper_before,
            free_before,
            lower_after: [false; 21],
            upper_after: [false; 21],
            free_after: [false; 21],
            free_ids: capture.solution.free_ids.clone(),
            radius: capture.solution.radius,
            lambda: Some(capture.solution.trace.lambda),
            weighted_residual: Some(
                capture
                    .prepared
                    .weighted_residual
                    .clone()
                    .try_into()
                    .expect("fixed KKT dimension"),
            ),
            weighted_matrix: Some(capture.prepared.weighted_matrix.clone()),
            crossing_theta: None,
            selected_coordinate: Some(selected_coordinate),
            selected_lower: Some(selected_lower),
            residual: Some(capture.values.residual),
            g: Some(capture.values.gradient),
            lambda_times_p: Some(capture.values.lambda_times_step),
            h: Some(capture.values.h),
            optimality: Some(capture.assessment.observation.clone()),
            entered_factorizations: 0,
            completed_pivots: 0,
            completed_sweeps: 0,
        },
    );
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn capture_return(
    state: &SubproblemState,
    solution: &FaceSolution,
    prepared: &PreparedSubproblem,
    values: &KktValues,
    assessment: &OptimalityAssessment,
) {
    capture_operation(
        state,
        M1FacePivotCaptureOperation {
            action: "kkt-return",
            p_before: state.p,
            p_candidate: state.p,
            conceptual_p_after: state.p,
            scratch_p_at_refusal: None,
            applied: true,
            lower_before: state.lower_active,
            upper_before: state.upper_active,
            free_before: state.free,
            lower_after: [false; 21],
            upper_after: [false; 21],
            free_after: [false; 21],
            free_ids: solution.free_ids.clone(),
            radius: solution.radius,
            lambda: Some(solution.trace.lambda),
            weighted_residual: Some(
                prepared
                    .weighted_residual
                    .clone()
                    .try_into()
                    .expect("fixed KKT dimension"),
            ),
            weighted_matrix: Some(prepared.weighted_matrix.clone()),
            crossing_theta: None,
            selected_coordinate: None,
            selected_lower: None,
            residual: Some(values.residual),
            g: Some(values.gradient),
            lambda_times_p: Some(values.lambda_times_step),
            h: Some(values.h),
            optimality: Some(assessment.observation.clone()),
            entered_factorizations: 0,
            completed_pivots: 0,
            completed_sweeps: 0,
        },
    );
}

fn state_refusal(
    kind: M1TrustRegionRefusalKind,
    state: &SubproblemState,
    raw_max: f64,
) -> super::M1TrustRegionSubproblemRefusal {
    let mut refusal = subproblem_refusal(
        kind,
        1,
        state.completed_sweeps,
        state.completed_pivots,
        state.entered,
        raw_max,
    );
    refusal.work = Box::new(state_work_observation(state));
    refusal
}

fn state_work_observation(state: &SubproblemState) -> super::M1TrustRegionWorkObservation {
    super::M1TrustRegionWorkObservation {
        stage1_calls: 1,
        entered_svd_factorizations: u32::from(state.entered),
        completed_face_pivots: u32::from(state.completed_pivots),
        total_jacobi_sweeps: state.total_jacobi_sweeps,
        refinement_scalar_operations: state.refinement_work.scalar_operations,
        refinement_guard_events: state.refinement_work.guard_events,
        refinement_attempted_entries: u32::from(state.refinement_work.attempted_entries),
        refinement_denied_entries: u32::from(state.refinement_work.denied_entries),
        first_residual_operations: state.refinement_work.first_residual_operations,
        second_dot2_operations: state.refinement_work.second_dot2_operations,
        factor_application_operations: state.refinement_work.factor_application_operations,
        coordinate_update_operations: state.refinement_work.coordinate_update_operations,
        whole_kkt_operations: state.refinement_work.whole_kkt_operations,
        whole_kkt_guard_events: state.refinement_work.whole_kkt_guard_events,
        upward_enclosure_operations: state.refinement_work.upward_enclosure_operations,
        upward_enclosure_guard_events: state.refinement_work.upward_enclosure_guard_events,
    }
}

fn optimality_state_refusal(
    failure: OptimalityFailure,
    state: &SubproblemState,
    raw_max: f64,
    solution: &FaceSolution,
) -> super::M1TrustRegionSubproblemRefusal {
    let mut refusal = state_refusal(
        M1TrustRegionRefusalKind::TrustRegionOptimalityIndeterminate,
        state,
        raw_max,
    );
    refusal.optimality_reason = Some(failure.reason);
    refusal.optimality_coordinate = Some(failure.coordinate);
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    {
        refusal.optimality_radius = Some(Box::new(solution.radius));
        refusal.optimality_lambda = Some(Box::new(solution.trace.lambda));
    }
    refusal
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn state_budget_refusal(
    state: &SubproblemState,
    raw_max: f64,
    budget_refusal: crate::M1PhysicalBudgetRefusal,
) -> super::M1TrustRegionSubproblemRefusal {
    let mut refusal = state_refusal(M1TrustRegionRefusalKind::WorkCap, state, raw_max);
    refusal.budget_refusal = Some(budget_refusal);
    refusal
}

#[derive(Clone, Copy, Debug)]
struct OptimalityFailure {
    reason: &'static str,
    coordinate: usize,
}

struct KktAccounting<'a> {
    ledger: Option<&'a mut RefinementWorkLedger>,
    refined: bool,
}

impl<'a> KktAccounting<'a> {
    fn untracked() -> Self {
        Self {
            ledger: None,
            refined: false,
        }
    }

    fn tracked(ledger: &'a mut RefinementWorkLedger, refined: bool) -> Self {
        Self {
            ledger: Some(ledger),
            refined,
        }
    }

    fn kkt_operation(&mut self) {
        if let Some(ledger) = &mut self.ledger {
            ledger.whole_kkt_operations += 1;
            if self.refined {
                ledger.scalar();
            }
        }
    }

    fn kkt_admission(&mut self) {
        if let Some(ledger) = &mut self.ledger {
            ledger.whole_kkt_guard_events += 1;
            if self.refined {
                ledger.guard();
            }
        }
    }

    fn enclosure_operation(&mut self) {
        if let Some(ledger) = &mut self.ledger {
            ledger.upward_enclosure_operations += 1;
            if self.refined {
                ledger.scalar();
            }
        }
    }

    fn enclosure_admission(&mut self) {
        if let Some(ledger) = &mut self.ledger {
            ledger.upward_enclosure_guard_events += 1;
            if self.refined {
                ledger.guard();
            }
        }
    }
}

fn optimality_failure(reason: &'static str, coordinate: usize) -> OptimalityFailure {
    OptimalityFailure { reason, coordinate }
}

fn checked_optimality_operand(value: f64, coordinate: usize) -> Result<(), OptimalityFailure> {
    if !value.is_finite() {
        return Err(optimality_failure("optimality_nonfinite", coordinate));
    }
    if value != 0.0 && !value.is_normal() {
        return Err(optimality_failure(
            "optimality_non_normal_operand",
            coordinate,
        ));
    }
    Ok(())
}

fn checked_optimality_product(
    left: f64,
    right: f64,
    coordinate: usize,
    accounting: &mut KktAccounting<'_>,
) -> Result<f64, OptimalityFailure> {
    accounting.kkt_admission();
    checked_optimality_operand(left, coordinate)?;
    accounting.kkt_admission();
    checked_optimality_operand(right, coordinate)?;
    accounting.kkt_operation();
    let product = left * right;
    accounting.kkt_admission();
    if !product.is_finite() {
        return Err(optimality_failure("optimality_nonfinite", coordinate));
    }
    if product == 0.0 && left != 0.0 && right != 0.0 {
        return Err(optimality_failure(
            "optimality_underflow_to_zero",
            coordinate,
        ));
    }
    if product != 0.0 && !product.is_normal() {
        return Err(optimality_failure(
            "optimality_subnormal_intermediate",
            coordinate,
        ));
    }
    Ok(product)
}

fn checked_optimality_add(
    left: f64,
    right: f64,
    coordinate: usize,
    accounting: &mut KktAccounting<'_>,
) -> Result<f64, OptimalityFailure> {
    accounting.kkt_admission();
    checked_optimality_operand(left, coordinate)?;
    accounting.kkt_admission();
    checked_optimality_operand(right, coordinate)?;
    accounting.kkt_operation();
    let sum = left + right;
    accounting.kkt_admission();
    if !sum.is_finite() {
        return Err(optimality_failure("optimality_nonfinite", coordinate));
    }
    if sum == 0.0 && (left != 0.0 || right != 0.0) && left.to_bits() != (-right).to_bits() {
        return Err(optimality_failure("optimality_zero_addition", coordinate));
    }
    if sum != 0.0 && !sum.is_normal() {
        return Err(optimality_failure(
            "optimality_subnormal_intermediate",
            coordinate,
        ));
    }
    Ok(sum)
}

fn box_feasible_kkt_values(
    weighted_matrix: &[Vec<f64>],
    weighted_residual: &[f64],
    step: &[f64; 21],
    lambda: f64,
) -> Result<KktValues, OptimalityFailure> {
    let mut accounting = KktAccounting::untracked();
    box_feasible_kkt_values_with_accounting(
        weighted_matrix,
        weighted_residual,
        step,
        lambda,
        &mut accounting,
    )
}

fn box_feasible_kkt_values_with_accounting(
    weighted_matrix: &[Vec<f64>],
    weighted_residual: &[f64],
    step: &[f64; 21],
    lambda: f64,
    accounting: &mut KktAccounting<'_>,
) -> Result<KktValues, OptimalityFailure> {
    const N: usize = 21;
    accounting.kkt_admission();
    if lambda < 0.0 {
        return Err(optimality_failure("optimality_negative_lambda", 0));
    }
    checked_optimality_operand(lambda, 0)?;
    let mut residual = [0.0; N];
    let mut gradient = [0.0; N];
    let mut lambda_times_step = [0.0; N];
    let mut h = [0.0; N];
    for row in 0..N {
        let mut ap_sum = 0.0;
        for col in 0..N {
            let term =
                checked_optimality_product(weighted_matrix[row][col], step[col], col, accounting)?;
            ap_sum = checked_optimality_add(ap_sum, term, col, accounting)?;
        }
        residual[row] = checked_optimality_add(weighted_residual[row], ap_sum, row, accounting)?;
    }
    for col in 0..N {
        for row in 0..N {
            let term = checked_optimality_product(
                weighted_matrix[row][col],
                residual[row],
                col,
                accounting,
            )?;
            gradient[col] = checked_optimality_add(gradient[col], term, col, accounting)?;
        }
        lambda_times_step[col] = checked_optimality_product(lambda, step[col], col, accounting)?;
        h[col] = checked_optimality_add(gradient[col], lambda_times_step[col], col, accounting)?;
    }
    Ok(KktValues {
        residual,
        gradient,
        lambda_times_step,
        h,
    })
}

struct FacePreparation {
    radius: f64,
    free_ids: Vec<usize>,
    residual: [f64; 21],
    matrix: Vec<Vec<f64>>,
}

fn prepare_face(
    initial_radius: f64,
    step: &[f64; 21],
    lower_active: &[bool; 21],
    upper_active: &[bool; 21],
    free: &[bool; 21],
    weighted_residual: &[f64],
    weighted_matrix: &[Vec<f64>],
) -> Result<FacePreparation, M1TrustRegionRefusalKind> {
    const N: usize = 21;
    let mut active_square_sum = 0.0;
    for col in 0..N {
        if lower_active[col] || upper_active[col] {
            let square = step[col] * step[col];
            if !square.is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
            active_square_sum += square;
            if !active_square_sum.is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
        }
    }
    let r2 = initial_radius * initial_radius - active_square_sum;
    if !r2.is_finite() || r2 < 0.0 || r2 == 0.0 || !free.iter().any(|value| *value) {
        return Err(M1TrustRegionRefusalKind::DegenerateFace);
    }
    let free_ids: Vec<usize> = (0..N).filter(|&index| free[index]).collect();
    let mut residual = [0.0; N];
    for row in 0..N {
        let mut active_product_sum = 0.0;
        for col in 0..N {
            if lower_active[col] || upper_active[col] {
                let term = weighted_matrix[row][col] * step[col];
                if !term.is_finite() {
                    return Err(M1TrustRegionRefusalKind::SvdNonFinite);
                }
                active_product_sum += term;
                if !active_product_sum.is_finite() {
                    return Err(M1TrustRegionRefusalKind::SvdNonFinite);
                }
            }
        }
        residual[row] = weighted_residual[row] + active_product_sum;
        if !residual[row].is_finite() {
            return Err(M1TrustRegionRefusalKind::SvdNonFinite);
        }
    }
    let matrix = (0..N)
        .map(|row| {
            free_ids
                .iter()
                .map(|&col| weighted_matrix[row][col])
                .collect()
        })
        .collect();
    Ok(FacePreparation {
        radius: r2.sqrt(),
        free_ids,
        residual,
        matrix,
    })
}

fn first_crossing(
    step: &[f64; 21],
    previous_step: &[f64; 21],
    free_ids: &[usize],
    lower: &[f64; 21],
    upper: &[f64; 21],
) -> Result<Option<(f64, usize, bool)>, M1TrustRegionRefusalKind> {
    let mut crossing = None;
    for &col in free_ids {
        let bound = if step[col] < lower[col] {
            Some((lower[col], true))
        } else if step[col] > upper[col] {
            Some((upper[col], false))
        } else {
            None
        };
        if let Some((bound, is_lower)) = bound {
            let theta = (bound - previous_step[col]) / (step[col] - previous_step[col]);
            if !theta.is_finite() || !(0.0..=1.0).contains(&theta) {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
            if crossing.is_none_or(|(best, index, _)| {
                theta < best || (theta.partial_cmp(&best) == Some(Ordering::Equal) && col < index)
            }) {
                crossing = Some((theta, col, is_lower));
            }
        }
    }
    Ok(crossing)
}

fn upward_operand(value: f64, coordinate: usize) -> Result<f64, OptimalityFailure> {
    let mut accounting = KktAccounting::untracked();
    upward_operand_with_accounting(value, coordinate, &mut accounting)
}

fn upward_operand_with_accounting(
    value: f64,
    coordinate: usize,
    accounting: &mut KktAccounting<'_>,
) -> Result<f64, OptimalityFailure> {
    accounting.enclosure_admission();
    checked_optimality_operand(value, coordinate)?;
    let magnitude = value.abs();
    if !magnitude.is_finite() || (magnitude != 0.0 && !magnitude.is_normal()) {
        return Err(optimality_failure(
            "optimality_upward_enclosure",
            coordinate,
        ));
    }
    Ok(magnitude)
}

fn upward_mul(
    left: f64,
    right: f64,
    coordinate: usize,
    operations: &mut u16,
) -> Result<f64, OptimalityFailure> {
    let mut accounting = KktAccounting::untracked();
    upward_mul_with_accounting(left, right, coordinate, operations, &mut accounting)
}

fn upward_mul_with_accounting(
    left: f64,
    right: f64,
    coordinate: usize,
    operations: &mut u16,
    accounting: &mut KktAccounting<'_>,
) -> Result<f64, OptimalityFailure> {
    if left < 0.0 || right < 0.0 {
        return Err(optimality_failure(
            "optimality_upward_enclosure",
            coordinate,
        ));
    }
    let left = upward_operand_with_accounting(left, coordinate, accounting)?;
    let right = upward_operand_with_accounting(right, coordinate, accounting)?;
    if left == 0.0 || right == 0.0 {
        return Ok(0.0);
    }
    *operations += 1;
    accounting.enclosure_operation();
    let product = left * right;
    accounting.enclosure_admission();
    if !product.is_finite() {
        return Err(optimality_failure(
            "optimality_upward_enclosure",
            coordinate,
        ));
    }
    if product == 0.0 {
        return Err(optimality_failure(
            "optimality_underflow_to_zero",
            coordinate,
        ));
    }
    if !product.is_normal() {
        return Err(optimality_failure(
            "optimality_subnormal_intermediate",
            coordinate,
        ));
    }
    *operations += 1;
    accounting.enclosure_operation();
    let upward = product.next_up();
    accounting.enclosure_admission();
    if !upward.is_finite() || !upward.is_normal() {
        return Err(optimality_failure(
            "optimality_upward_enclosure",
            coordinate,
        ));
    }
    Ok(upward)
}

fn upward_add(
    left: f64,
    right: f64,
    coordinate: usize,
    operations: &mut u16,
) -> Result<f64, OptimalityFailure> {
    let mut accounting = KktAccounting::untracked();
    upward_add_with_accounting(left, right, coordinate, operations, &mut accounting)
}

fn upward_add_with_accounting(
    left: f64,
    right: f64,
    coordinate: usize,
    operations: &mut u16,
    accounting: &mut KktAccounting<'_>,
) -> Result<f64, OptimalityFailure> {
    if left < 0.0 || right < 0.0 {
        return Err(optimality_failure(
            "optimality_upward_enclosure",
            coordinate,
        ));
    }
    let left = upward_operand_with_accounting(left, coordinate, accounting)?;
    let right = upward_operand_with_accounting(right, coordinate, accounting)?;
    if right == 0.0 {
        return Ok(left);
    }
    *operations += 1;
    accounting.enclosure_operation();
    let sum = left + right;
    accounting.enclosure_admission();
    if !sum.is_finite() || sum == 0.0 {
        return Err(optimality_failure(
            "optimality_upward_enclosure",
            coordinate,
        ));
    }
    if !sum.is_normal() {
        return Err(optimality_failure(
            "optimality_subnormal_intermediate",
            coordinate,
        ));
    }
    *operations += 1;
    accounting.enclosure_operation();
    let upward = sum.next_up();
    accounting.enclosure_admission();
    if !upward.is_finite() || !upward.is_normal() {
        return Err(optimality_failure(
            "optimality_upward_enclosure",
            coordinate,
        ));
    }
    Ok(upward)
}

fn cbar_and_tau(
    weighted_matrix: &[Vec<f64>],
    weighted_residual: &[f64],
    step: &[f64; 21],
    lambda: f64,
    coordinate: usize,
    operations: &mut u16,
    accounting: &mut KktAccounting<'_>,
) -> Result<(f64, f64), OptimalityFailure> {
    let mut cbar = 0.0;
    for row in 0..21 {
        let mut inner = 0.0;
        for column in 0..21 {
            let term = upward_mul_with_accounting(
                upward_operand_with_accounting(
                    weighted_matrix[row][column],
                    coordinate,
                    accounting,
                )?,
                upward_operand_with_accounting(step[column], coordinate, accounting)?,
                coordinate,
                operations,
                accounting,
            )?;
            inner = upward_add_with_accounting(inner, term, coordinate, operations, accounting)?;
        }
        let row_scale = upward_add_with_accounting(
            upward_operand_with_accounting(weighted_residual[row], coordinate, accounting)?,
            inner,
            coordinate,
            operations,
            accounting,
        )?;
        let row_term = upward_mul_with_accounting(
            upward_operand_with_accounting(
                weighted_matrix[row][coordinate],
                coordinate,
                accounting,
            )?,
            row_scale,
            coordinate,
            operations,
            accounting,
        )?;
        cbar = upward_add_with_accounting(cbar, row_term, coordinate, operations, accounting)?;
    }
    let lambda_term = upward_mul_with_accounting(
        upward_operand_with_accounting(lambda, coordinate, accounting)?,
        upward_operand_with_accounting(step[coordinate], coordinate, accounting)?,
        coordinate,
        operations,
        accounting,
    )?;
    cbar = upward_add_with_accounting(cbar, lambda_term, coordinate, operations, accounting)?;
    if cbar == 0.0 {
        return Ok((0.0, 0.0));
    }
    let gamma = (87.0 / (TWO_POW_53 - 87.0)).next_up();
    accounting.enclosure_admission();
    if !gamma.is_finite() || !gamma.is_normal() {
        return Err(optimality_failure(
            "optimality_upward_enclosure",
            coordinate,
        ));
    }
    let tau = upward_mul_with_accounting(gamma, cbar, coordinate, operations, accounting)?;
    Ok((cbar, tau))
}

struct OptimalityAssessment {
    observation: super::M1TrustRegionOptimalityObservation,
    release: Option<(usize, bool)>,
    free_refusal: Option<usize>,
}

struct KktTransitionOutcome {
    values: Option<KktValues>,
    observation: super::M1TrustRegionOptimalityObservation,
}

fn assess_optimality(
    prepared: &PreparedSubproblem,
    p: &[f64; 21],
    lower_active: &[bool; 21],
    upper_active: &[bool; 21],
    initial_free: &[bool; 21],
    values: &KktValues,
    lambda: f64,
    accounting: &mut KktAccounting<'_>,
) -> Result<OptimalityAssessment, OptimalityFailure> {
    let mut operations = 0;
    let mut coordinates = Vec::with_capacity(21);
    let mut candidates: Vec<(usize, bool, Option<f64>)> = Vec::new();
    let mut free_refusal = None;
    for coordinate in 0..21 {
        let (cbar, tau) = cbar_and_tau(
            &prepared.weighted_matrix,
            &prepared.weighted_residual,
            p,
            lambda,
            coordinate,
            &mut operations,
            accounting,
        )?;
        let h = values.h[coordinate];
        accounting.kkt_admission();
        checked_optimality_operand(h, coordinate)?;
        let fixed = !initial_free[coordinate];
        let class = if fixed {
            super::M1TrustRegionOptimalityClass::Fixed
        } else if lower_active[coordinate] {
            if h < -tau {
                let ratio = if tau == 0.0 {
                    None
                } else {
                    let ratio = h.abs() / tau;
                    accounting.kkt_admission();
                    if !ratio.is_finite() || !ratio.is_normal() {
                        return Err(optimality_failure("optimality_release_ratio", coordinate));
                    }
                    Some(ratio)
                };
                candidates.push((coordinate, true, ratio));
                super::M1TrustRegionOptimalityClass::ReleaseLower
            } else {
                super::M1TrustRegionOptimalityClass::RetainLower
            }
        } else if upper_active[coordinate] {
            if h > tau {
                let ratio = if tau == 0.0 {
                    None
                } else {
                    let ratio = h.abs() / tau;
                    accounting.kkt_admission();
                    if !ratio.is_finite() || !ratio.is_normal() {
                        return Err(optimality_failure("optimality_release_ratio", coordinate));
                    }
                    Some(ratio)
                };
                candidates.push((coordinate, false, ratio));
                super::M1TrustRegionOptimalityClass::ReleaseUpper
            } else {
                super::M1TrustRegionOptimalityClass::RetainUpper
            }
        } else if h.abs() <= tau {
            super::M1TrustRegionOptimalityClass::FreePass
        } else {
            free_refusal.get_or_insert(coordinate);
            super::M1TrustRegionOptimalityClass::FreeRefuse
        };
        if cbar == 0.0 && h != 0.0 {
            return Err(optimality_failure(
                "optimality_free_stationarity",
                coordinate,
            ));
        }
        coordinates.push(super::M1TrustRegionOptimalityCoordinateObservation {
            coordinate,
            h,
            cbar,
            tau,
            class,
        });
    }
    candidates.sort_by(|left, right| match (left.2, right.2) {
        (None, None) => left.0.cmp(&right.0),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(left_ratio), Some(right_ratio)) => right_ratio
            .total_cmp(&left_ratio)
            .then_with(|| left.0.cmp(&right.0)),
    });
    let selected_release = candidates.first().map(|(coordinate, lower, ratio)| {
        let tied_coordinates = candidates
            .iter()
            .filter(|(_, _, candidate_ratio)| candidate_ratio == ratio)
            .fold(0_u8, |count, _| count + 1);
        super::M1TrustRegionOptimalityReleaseObservation {
            coordinate: *coordinate,
            lower: *lower,
            ratio: *ratio,
            tied_coordinates,
        }
    });
    Ok(OptimalityAssessment {
        release: selected_release.map(|selected| (selected.coordinate, selected.lower)),
        free_refusal: selected_release.is_none().then_some(free_refusal).flatten(),
        observation: super::M1TrustRegionOptimalityObservation {
            coordinates,
            selected_release,
            scalar_operations: operations,
        },
    })
}

fn prepare_subproblem(
    input: &super::M1TrustRegionSubproblemInput,
) -> Result<PreparedSubproblem, super::M1TrustRegionSubproblemRefusal> {
    const N: usize = 21;
    let mut raw_max = 0.0_f64;
    for row in 0..N {
        raw_max = raw_max.max(input.raw_residual[row].abs());
        if !input.raw_residual[row].is_finite()
            || !input.normalizers[row].is_finite()
            || input.normalizers[row] <= 0.0
            || !input.base[row].is_finite()
            || !input.scales[row].is_finite()
            || input.scales[row] <= 0.0
            || input.lower[row].is_nan()
            || input.upper[row].is_nan()
            || input.lower[row] > input.upper[row]
            || input.base[row] < input.lower[row]
            || input.base[row] > input.upper[row]
        {
            return Err(subproblem_refusal(
                M1TrustRegionRefusalKind::SvdNonFinite,
                1,
                input.test_work.completed_jacobi_sweeps,
                input.test_work.completed_face_pivots,
                0,
                raw_max,
            ));
        }
    }
    if !input.initial_radius.is_finite() || input.initial_radius <= 0.0 {
        return Err(subproblem_refusal(
            M1TrustRegionRefusalKind::DegenerateFace,
            1,
            input.test_work.completed_jacobi_sweeps,
            input.test_work.completed_face_pivots,
            0,
            raw_max,
        ));
    }
    let rows: Vec<&[f64]> = input.raw_jacobian.iter().map(|row| &row[..]).collect();
    let (weighted_residual, weighted_matrix) = weighted_system(
        &input.raw_residual,
        &rows,
        &input.normalizers,
        &input.scales,
    )
    .map_err(|_| {
        subproblem_refusal(
            M1TrustRegionRefusalKind::SvdNonFinite,
            1,
            input.test_work.completed_jacobi_sweeps,
            input.test_work.completed_face_pivots,
            0,
            raw_max,
        )
    })?;
    let mut scaled_lower = [0.0; N];
    let mut scaled_upper = [0.0; N];
    for col in 0..N {
        scaled_lower[col] = (input.lower[col] - input.base[col]) / input.scales[col];
        scaled_upper[col] = (input.upper[col] - input.base[col]) / input.scales[col];
        if scaled_lower[col].is_nan()
            || scaled_upper[col].is_nan()
            || scaled_lower[col] > scaled_upper[col]
            || (input.lower[col].is_finite() && !scaled_lower[col].is_finite())
            || (input.upper[col].is_finite() && !scaled_upper[col].is_finite())
        {
            return Err(subproblem_refusal(
                M1TrustRegionRefusalKind::SvdNonFinite,
                1,
                input.test_work.completed_jacobi_sweeps,
                input.test_work.completed_face_pivots,
                0,
                raw_max,
            ));
        }
    }
    Ok(PreparedSubproblem {
        raw_max,
        weighted_residual,
        weighted_matrix,
        scaled_lower,
        scaled_upper,
    })
}

fn candidate_coordinates(
    input: &super::M1TrustRegionSubproblemInput,
    step: &[f64; 21],
) -> Option<[f64; 21]> {
    let mut candidate = [0.0; 21];
    for col in 0..21 {
        candidate[col] = input.base[col] + input.scales[col] * step[col];
        if !candidate[col].is_finite() {
            return None;
        }
    }
    Some(candidate)
}

fn raw_jacobian_columns(input: &super::M1TrustRegionSubproblemInput) -> [[f64; 21]; 21] {
    let mut columns = [[0.0; 21]; 21];
    for (col, column) in columns.iter_mut().enumerate() {
        for (row, entry) in column.iter_mut().enumerate() {
            *entry = input.raw_jacobian[row][col];
        }
    }
    columns
}

struct FaceSolution {
    free_ids: Vec<usize>,
    trace: LambdaTrace,
    refined: bool,
    // BVLS-03 applies its two corrections through the exact factor already
    // constructed for this face.  Keeping ownership here prevents a second
    // factorization or a numerically distinct reconstruction.
    factor: Option<JacobiFactor>,
    radius: f64,
}

fn solve_face(
    input: &super::M1TrustRegionSubproblemInput,
    prepared: &PreparedSubproblem,
    state: &mut SubproblemState,
) -> Result<FaceSolution, super::M1TrustRegionSubproblemRefusal> {
    let face = prepare_face(
        input.initial_radius,
        &state.p,
        &state.lower_active,
        &state.upper_active,
        &state.free,
        &prepared.weighted_residual,
        &prepared.weighted_matrix,
    )
    .map_err(|kind| state_refusal(kind, state, prepared.raw_max))?;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    if let Err(budget_refusal) =
        crate::m1_physical_budget_enter(crate::M1PhysicalBudgetOperation::Stage1Factorization)
    {
        return Err(state_budget_refusal(
            state,
            prepared.raw_max,
            budget_refusal,
        ));
    }
    state.entered += 1;
    let factor_result = jacobi_factor(
        face.matrix,
        &face.free_ids,
        input.test_work.completed_jacobi_sweeps,
        &mut state.completed_sweeps,
    );
    let sweeps_added = state
        .completed_sweeps
        .checked_sub(input.test_work.completed_jacobi_sweeps)
        .ok_or_else(|| {
            state_refusal(
                M1TrustRegionRefusalKind::SvdNoConvergence,
                state,
                prepared.raw_max,
            )
        })?;
    state.total_jacobi_sweeps += u32::from(sweeps_added);
    let factor = match factor_result {
        Ok(value) => value,
        Err("rank deficient") => {
            return Err(state_refusal(
                M1TrustRegionRefusalKind::RankDeficient,
                state,
                prepared.raw_max,
            ));
        }
        Err("no convergence") => {
            return Err(state_refusal(
                M1TrustRegionRefusalKind::SvdNoConvergence,
                state,
                prepared.raw_max,
            ));
        }
        Err(_) => {
            return Err(state_refusal(
                M1TrustRegionRefusalKind::SvdNonFinite,
                state,
                prepared.raw_max,
            ));
        }
    };
    state.completed_sweeps = state.completed_sweeps.max(factor.sweeps);
    let mut trace = LambdaTrace::default();
    svd_lambda_ball(&factor, &face.residual, face.radius, None, &mut trace)
        .map_err(|kind| state_refusal(kind, state, prepared.raw_max))?;
    Ok(FaceSolution {
        free_ids: face.free_ids,
        trace,
        refined: false,
        factor: Some(factor),
        radius: face.radius,
    })
}

// The BVLS-03 retained controls carry source operands as hexadecimal binary64
// JSON.  Decode only those operands here, then hand the resulting state to the
// ordinary preparation/factorization path.  This is a test-only custody seam:
// expected Dot2 or KKT results never enter it.
#[cfg(test)]
fn refinement_capture_bit(value: &serde_json::Value) -> Result<f64, String> {
    let text = value
        .as_str()
        .ok_or_else(|| "refinement capture bit is not a string".to_owned())?;
    u64::from_str_radix(text, 16)
        .map(f64::from_bits)
        .map_err(|_| "refinement capture bit is not hexadecimal".to_owned())
}

#[cfg(test)]
fn refinement_capture_vector(value: &serde_json::Value) -> Result<[f64; 21], String> {
    let values = value
        .as_array()
        .ok_or_else(|| "refinement capture vector is not an array".to_owned())?;
    if values.len() != 21 {
        return Err("refinement capture vector has wrong dimension".to_owned());
    }
    let mut result = [0.0; 21];
    for (index, entry) in values.iter().enumerate() {
        result[index] = refinement_capture_bit(entry)?;
    }
    Ok(result)
}

#[cfg(test)]
fn refinement_capture_mask(value: &serde_json::Value) -> Result<[bool; 21], String> {
    let values = value
        .as_array()
        .ok_or_else(|| "refinement capture mask is not an array".to_owned())?;
    if values.len() != 21 {
        return Err("refinement capture mask has wrong dimension".to_owned());
    }
    let mut result = [false; 21];
    for (index, entry) in values.iter().enumerate() {
        result[index] = entry
            .as_bool()
            .ok_or_else(|| "refinement capture mask is not boolean".to_owned())?;
    }
    Ok(result)
}

#[cfg(test)]
fn refinement_capture_matrix(value: &serde_json::Value) -> Result<[[f64; 21]; 21], String> {
    let rows = value
        .as_array()
        .ok_or_else(|| "refinement capture matrix is not an array".to_owned())?;
    if rows.len() != 21 {
        return Err("refinement capture matrix has wrong row count".to_owned());
    }
    let mut result = [[0.0; 21]; 21];
    for (row, values) in rows.iter().enumerate() {
        result[row] = refinement_capture_vector(values)?;
    }
    Ok(result)
}

#[cfg(test)]
fn refinement_capture_operation<'a>(
    capture: &'a serde_json::Value,
) -> Result<&'a serde_json::Value, String> {
    capture["operations"]
        .as_array()
        .and_then(|operations| operations.get(1))
        .ok_or_else(|| "refinement capture lacks operation 1".to_owned())
}

#[cfg(test)]
fn refinement_decode_captured_face(
    capture: &serde_json::Value,
) -> Result<
    (
        super::M1TrustRegionSubproblemInput,
        PreparedSubproblem,
        SubproblemState,
    ),
    String,
> {
    let input = super::M1TrustRegionSubproblemInput {
        raw_residual: refinement_capture_vector(&capture["raw_residual"])?,
        raw_jacobian: refinement_capture_matrix(&capture["raw_jacobian"])?,
        normalizers: refinement_capture_vector(&capture["normalizers"])?,
        scales: refinement_capture_vector(&capture["scales"])?,
        lower: refinement_capture_vector(&capture["lower"])?,
        upper: refinement_capture_vector(&capture["upper"])?,
        initial_radius: refinement_capture_bit(&capture["initial_radius"])?,
        base: refinement_capture_vector(&capture["base"])?,
        test_work: super::M1TrustRegionTestWorkState::default(),
    };
    let prepared = prepare_subproblem(&input)
        .map_err(|_| "refinement capture preparation refused".to_owned())?;
    let operation = refinement_capture_operation(capture)?;
    let mut state = SubproblemState::new(&input);
    state.p = refinement_capture_vector(&operation["p_before"])?;
    state.lower_active = refinement_capture_mask(&operation["lower_before"])?;
    state.upper_active = refinement_capture_mask(&operation["upper_before"])?;
    state.free = refinement_capture_mask(&operation["free_before"])?;
    if !(0..21).all(|column| {
        usize::from(state.lower_active[column])
            + usize::from(state.upper_active[column])
            + usize::from(state.free[column])
            == 1
    }) {
        return Err("refinement capture active/free partition is incomplete".to_owned());
    }
    Ok((input, prepared, state))
}

#[cfg(test)]
fn refinement_solve_captured_face(
    capture: &serde_json::Value,
) -> Result<
    (
        super::M1TrustRegionSubproblemInput,
        PreparedSubproblem,
        SubproblemState,
        FaceSolution,
    ),
    String,
> {
    let (input, prepared, mut state) = refinement_decode_captured_face(capture)?;
    let solution = solve_face(&input, &prepared, &mut state)
        .map_err(|_| "refinement capture face solve refused".to_owned())?;
    Ok((input, prepared, state, solution))
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
struct RefinementWorkReservation {
    remaining_scalar_operations: u64,
    remaining_guard_events: u64,
    required_scalar_operations: u64,
    required_guard_events: u64,
}

#[cfg(test)]
#[derive(Clone, Debug)]
struct RefinementRetainedFaceObservation {
    live_state_bits_entry: [u64; 21],
    live_state_bits_exit: [u64; 21],
    p0_bits: Vec<u64>,
    delta0_free_bits: Vec<u64>,
    p1_bits: Vec<u64>,
    p1_box_feasible: bool,
    dot2_residual_bits: Vec<u64>,
    delta1_free_bits: Vec<u64>,
    p2_bits: Vec<u64>,
    h_bits: Vec<u64>,
    cbar_bits: Vec<u64>,
    tau_bits: Vec<u64>,
    classes: Vec<String>,
    box_feasible: bool,
    ball_feasible: bool,
    factorizations_added: u8,
    factor_application_count: u8,
    factor_identity_before: usize,
    factor_identity_after: usize,
    factor_application_identities: Vec<usize>,
    lower_mask_p0: Vec<bool>,
    upper_mask_p0: Vec<bool>,
    lower_mask_p1: Vec<bool>,
    upper_mask_p1: Vec<bool>,
    lower_mask_p2: Vec<bool>,
    upper_mask_p2: Vec<bool>,
    lower_mask_exit: Vec<bool>,
    upper_mask_exit: Vec<bool>,
    corrections: u8,
    bvls02_pass: bool,
    refinement_scalar_operations: u64,
    first_residual_operations: u64,
    second_dot2_operations: u64,
    factor_application_operations: u64,
    coordinate_update_operations: u64,
    whole_kkt_operations: u64,
    whole_kkt_guard_events: u64,
    upward_enclosure_operations: u64,
    upward_enclosure_guard_events: u64,
    #[cfg(feature = "m1-trust-region-physical-stage")]
    budget_before: Option<crate::M1PhysicalBudgetSnapshot>,
    #[cfg(feature = "m1-trust-region-physical-stage")]
    budget_after: Option<crate::M1PhysicalBudgetSnapshot>,
}

#[cfg(test)]
#[derive(Clone, Debug)]
struct RefinementObservationRefusal {
    reason: &'static str,
    refinement: Option<Box<super::M1TrustRegionRefinementFailureMetadata>>,
    work: Box<super::M1TrustRegionWorkObservation>,
    returned_step: Option<[f64; 21]>,
    bvls02_fallback_calls: u8,
    materialization_calls: u8,
    physical_evaluator_calls: u8,
    hook_log: Vec<&'static str>,
    owner_snapshot_before: [u64; 21],
    owner_snapshot_after: [u64; 21],
    live_state_bits_before: [u64; 21],
    live_state_bits_after: [u64; 21],
    required_scalar_operations: u64,
    required_guard_events: u64,
    attempted_entries_before: u16,
    attempted_entries_after: u16,
    denied_entries_before: u16,
    denied_entries_after: u16,
    corrections: u8,
    dot2_operations: u64,
    factor_applications: u8,
    denied_entry_executed_scalar_operations: u64,
    denied_entry_executed_guard_events: u64,
    prior_eligibility_scalar_operations: u64,
    prior_eligibility_guard_events: u64,
    #[cfg(feature = "m1-trust-region-physical-stage")]
    global_before: Option<RefinementScratchGlobalSnapshot>,
    #[cfg(feature = "m1-trust-region-physical-stage")]
    global_after: Option<RefinementScratchGlobalSnapshot>,
    #[cfg(feature = "m1-trust-region-physical-stage")]
    budget_before: Option<crate::M1PhysicalBudgetSnapshot>,
    #[cfg(feature = "m1-trust-region-physical-stage")]
    budget_after: Option<crate::M1PhysicalBudgetSnapshot>,
}

#[cfg(test)]
fn refinement_class_name(class: super::M1TrustRegionOptimalityClass) -> &'static str {
    match class {
        super::M1TrustRegionOptimalityClass::Fixed => "fixed",
        super::M1TrustRegionOptimalityClass::ReleaseLower => "release_lower",
        super::M1TrustRegionOptimalityClass::RetainLower => "retain_lower",
        super::M1TrustRegionOptimalityClass::ReleaseUpper => "release_upper",
        super::M1TrustRegionOptimalityClass::RetainUpper => "retain_upper",
        super::M1TrustRegionOptimalityClass::FreePass => "free_pass",
        super::M1TrustRegionOptimalityClass::FreeRefuse => "free_refuse",
    }
}

#[cfg(test)]
fn refinement_observation_refusal(
    reason: &'static str,
    state: &SubproblemState,
) -> Box<RefinementObservationRefusal> {
    let bits = state.p.map(f64::to_bits);
    Box::new(RefinementObservationRefusal {
        reason,
        refinement: None,
        work: Box::new(state_work_observation(state)),
        returned_step: None,
        bvls02_fallback_calls: 0,
        materialization_calls: 0,
        physical_evaluator_calls: 0,
        hook_log: Vec::new(),
        owner_snapshot_before: bits,
        owner_snapshot_after: bits,
        live_state_bits_before: bits,
        live_state_bits_after: bits,
        required_scalar_operations: 0,
        required_guard_events: 0,
        attempted_entries_before: state.refinement_work.attempted_entries,
        attempted_entries_after: state.refinement_work.attempted_entries,
        denied_entries_before: state.refinement_work.denied_entries,
        denied_entries_after: state.refinement_work.denied_entries,
        corrections: 0,
        dot2_operations: 0,
        factor_applications: 0,
        denied_entry_executed_scalar_operations: 0,
        denied_entry_executed_guard_events: 0,
        prior_eligibility_scalar_operations: 0,
        prior_eligibility_guard_events: 0,
        #[cfg(feature = "m1-trust-region-physical-stage")]
        global_before: None,
        #[cfg(feature = "m1-trust-region-physical-stage")]
        global_after: None,
        #[cfg(feature = "m1-trust-region-physical-stage")]
        budget_before: None,
        #[cfg(feature = "m1-trust-region-physical-stage")]
        budget_after: None,
    })
}

#[cfg(test)]
fn m1_trust_region_refinement_retained_face_raw_for_test(
    capture: &serde_json::Value,
    reservation: Option<RefinementWorkReservation>,
) -> Result<RefinementRetainedFaceObservation, Box<RefinementObservationRefusal>> {
    let (input, prepared, mut state, mut solution) = refinement_solve_captured_face(capture)
        .map_err(|_| {
            refinement_observation_refusal(
                "refinement_capture_decode",
                &SubproblemState::new(&super::M1TrustRegionSubproblemInput {
                    raw_residual: [0.0; 21],
                    raw_jacobian: [[0.0; 21]; 21],
                    normalizers: [1.0; 21],
                    scales: [1.0; 21],
                    lower: [f64::NEG_INFINITY; 21],
                    upper: [f64::INFINITY; 21],
                    initial_radius: 1.0,
                    base: [0.0; 21],
                    test_work: super::M1TrustRegionTestWorkState::default(),
                }),
            )
        })?;
    let factor_identity_before = solution
        .factor
        .as_ref()
        .map_or(0, |factor| factor as *const JacobiFactor as usize);
    let entered_before_refinement = state.entered;
    let (required_scalar_operations, required_guard_events) =
        refinement_required_work(solution.free_ids.len());
    if let Some(reservation) = reservation {
        // The caller supplies remaining capacity only.  Required work remains
        // derived from the actual free-face dimension in the shared core.
        state.refinement_work.scalar_operations =
            REFINEMENT_SCALAR_CAP.saturating_sub(reservation.remaining_scalar_operations);
        state.refinement_work.guard_events =
            REFINEMENT_GUARD_CAP.saturating_sub(reservation.remaining_guard_events);
    }
    let attempted_entries_before = state.refinement_work.attempted_entries;
    let denied_entries_before = state.refinement_work.denied_entries;
    let ledger_before_refinement = state.refinement_work.scalar_operations;
    let guards_before_refinement = state.refinement_work.guard_events;
    let lower = state.lower_active;
    let upper = state.upper_active;
    let _observation_scope = RefinementObservationScope::begin();
    if let Err(refusal) = refine_face_before_crossing(&input, &prepared, &mut state, &mut solution)
    {
        let trace = refinement_observation_take().ok_or_else(|| {
            refinement_observation_refusal("refinement_observation_missing", &state)
        })?;
        let mut observed = refinement_observation_refusal(
            if refusal.kind == M1TrustRegionRefusalKind::WorkCap {
                "refinement_work_cap"
            } else {
                refusal.optimality_reason.unwrap_or("refinement_refusal")
            },
            &state,
        );
        observed.refinement = refusal.refinement;
        observed.work = refusal.work;
        observed.required_scalar_operations = required_scalar_operations;
        observed.required_guard_events = required_guard_events;
        observed.attempted_entries_before = attempted_entries_before;
        observed.attempted_entries_after = state.refinement_work.attempted_entries;
        observed.denied_entries_before = denied_entries_before;
        observed.denied_entries_after = state.refinement_work.denied_entries;
        observed.corrections = trace.factor_applications;
        observed.dot2_operations = state.refinement_work.second_dot2_operations;
        observed.factor_applications = trace.factor_applications;
        observed.prior_eligibility_scalar_operations =
            trace.eligibility_scalar_operations.ok_or_else(|| {
                refinement_observation_refusal("refinement_observation_missing", &state)
            })?;
        observed.prior_eligibility_guard_events =
            trace.eligibility_guard_events.ok_or_else(|| {
                refinement_observation_refusal("refinement_observation_missing", &state)
            })?;
        observed.denied_entry_executed_scalar_operations = state
            .refinement_work
            .scalar_operations
            .saturating_sub(ledger_before_refinement)
            .saturating_sub(observed.prior_eligibility_scalar_operations);
        observed.denied_entry_executed_guard_events = state
            .refinement_work
            .guard_events
            .saturating_sub(guards_before_refinement)
            .saturating_sub(observed.prior_eligibility_guard_events);
        observed.live_state_bits_before = trace.live_state_entry.ok_or_else(|| {
            refinement_observation_refusal("refinement_observation_missing", &state)
        })?;
        observed.live_state_bits_after = trace.live_state_exit.ok_or_else(|| {
            refinement_observation_refusal("refinement_observation_missing", &state)
        })?;
        return Err(observed);
    }
    let trace = refinement_observation_take()
        .ok_or_else(|| refinement_observation_refusal("refinement_observation_missing", &state))?;
    for (slot, &column) in solution.free_ids.iter().enumerate() {
        state.p[column] = solution.trace.step[slot];
    }
    let transition = kkt_transition(&prepared, &mut state, &solution).map_err(|refusal| {
        refinement_observation_refusal(
            refusal.optimality_reason.unwrap_or("refinement_bvls02"),
            &state,
        )
    })?;
    let bvls02_pass = transition.values.is_some();
    let optimality = transition.observation;
    let p2 = trace
        .p2
        .ok_or_else(|| refinement_observation_refusal("refinement_p2_missing", &state))?;
    let p2_norm = lambda_norm(&p2, None)
        .map_err(|_| refinement_observation_refusal("refinement_final_radius", &state))?;
    let factor_identity_after = solution
        .factor
        .as_ref()
        .map_or(0, |factor| factor as *const JacobiFactor as usize);
    Ok(RefinementRetainedFaceObservation {
        live_state_bits_entry: trace.live_state_entry.ok_or_else(|| {
            refinement_observation_refusal("refinement_observation_missing", &state)
        })?,
        live_state_bits_exit: trace.live_state_exit.ok_or_else(|| {
            refinement_observation_refusal("refinement_observation_missing", &state)
        })?,
        p0_bits: trace
            .p0
            .ok_or_else(|| refinement_observation_refusal("refinement_p0_missing", &state))?
            .map(f64::to_bits)
            .to_vec(),
        delta0_free_bits: trace
            .delta0
            .ok_or_else(|| refinement_observation_refusal("refinement_delta0_missing", &state))?
            .into_iter()
            .map(f64::to_bits)
            .collect(),
        p1_bits: trace
            .p1
            .ok_or_else(|| refinement_observation_refusal("refinement_p1_missing", &state))?
            .map(f64::to_bits)
            .to_vec(),
        p1_box_feasible: trace
            .p1_box_feasible
            .ok_or_else(|| refinement_observation_refusal("refinement_p1_missing", &state))?,
        dot2_residual_bits: trace
            .residual1
            .ok_or_else(|| refinement_observation_refusal("refinement_residual1_missing", &state))?
            .map(f64::to_bits)
            .to_vec(),
        delta1_free_bits: trace
            .delta1
            .ok_or_else(|| refinement_observation_refusal("refinement_delta1_missing", &state))?
            .into_iter()
            .map(f64::to_bits)
            .collect(),
        p2_bits: p2.map(f64::to_bits).to_vec(),
        h_bits: optimality
            .coordinates
            .iter()
            .map(|value| value.h.to_bits())
            .collect(),
        cbar_bits: optimality
            .coordinates
            .iter()
            .map(|value| value.cbar.to_bits())
            .collect(),
        tau_bits: optimality
            .coordinates
            .iter()
            .map(|value| value.tau.to_bits())
            .collect(),
        classes: optimality
            .coordinates
            .iter()
            .map(|value| refinement_class_name(value.class).to_owned())
            .collect(),
        box_feasible: refinement_box_admit(&prepared, &p2, None, None),
        ball_feasible: p2_norm <= input.initial_radius,
        factorizations_added: state.entered - entered_before_refinement,
        factor_application_count: trace.factor_applications,
        factor_identity_before,
        factor_identity_after,
        factor_application_identities: trace.factor_application_identities,
        lower_mask_p0: trace.lower_mask_p0.unwrap_or(lower).to_vec(),
        upper_mask_p0: trace.upper_mask_p0.unwrap_or(upper).to_vec(),
        lower_mask_p1: trace.lower_mask_p1.unwrap_or(lower).to_vec(),
        upper_mask_p1: trace.upper_mask_p1.unwrap_or(upper).to_vec(),
        lower_mask_p2: trace.lower_mask_p2.unwrap_or(lower).to_vec(),
        upper_mask_p2: trace.upper_mask_p2.unwrap_or(upper).to_vec(),
        lower_mask_exit: trace.lower_mask_exit.unwrap_or(state.lower_active).to_vec(),
        upper_mask_exit: trace.upper_mask_exit.unwrap_or(state.upper_active).to_vec(),
        corrections: trace.factor_applications,
        bvls02_pass,
        refinement_scalar_operations: state.refinement_work.scalar_operations,
        first_residual_operations: state.refinement_work.first_residual_operations,
        second_dot2_operations: state.refinement_work.second_dot2_operations,
        factor_application_operations: state.refinement_work.factor_application_operations,
        coordinate_update_operations: state.refinement_work.coordinate_update_operations,
        whole_kkt_operations: state.refinement_work.whole_kkt_operations,
        whole_kkt_guard_events: state.refinement_work.whole_kkt_guard_events,
        upward_enclosure_operations: state.refinement_work.upward_enclosure_operations,
        upward_enclosure_guard_events: state.refinement_work.upward_enclosure_guard_events,
        #[cfg(feature = "m1-trust-region-physical-stage")]
        budget_before: None,
        #[cfg(feature = "m1-trust-region-physical-stage")]
        budget_after: None,
    })
}

#[cfg(test)]
fn m1_trust_region_refinement_retained_face_for_test(
    capture: &serde_json::Value,
    reservation: Option<RefinementWorkReservation>,
) -> Result<RefinementRetainedFaceObservation, Box<RefinementObservationRefusal>> {
    #[cfg(feature = "m1-trust-region-physical-stage")]
    {
        let _outer_scope =
            crate::m1_physical_budget_scope_seeded_for_test(crate::M1PhysicalBudgetSnapshot {
                core: 7,
                core_attempted: 7,
                ..Default::default()
            });
        let before = crate::m1_physical_budget_observation_for_test();
        let result = {
            let _inner_scope = crate::m1_physical_budget_scope_for_test();
            m1_trust_region_refinement_retained_face_raw_for_test(capture, reservation)
        };
        let after = crate::m1_physical_budget_observation_for_test();
        return match result {
            Ok(mut observation) => {
                observation.budget_before = before;
                observation.budget_after = after;
                Ok(observation)
            }
            Err(mut refusal) => {
                refusal.budget_before = before;
                refusal.budget_after = after;
                Err(refusal)
            }
        };
    }
    #[cfg(not(feature = "m1-trust-region-physical-stage"))]
    m1_trust_region_refinement_retained_face_raw_for_test(capture, reservation)
}

#[cfg(test)]
struct RefinementMutationScope(Option<RefinementRawCandidateMutation>);

#[cfg(test)]
impl RefinementMutationScope {
    fn install(mutation: Option<RefinementRawCandidateMutation>) -> Self {
        let prior = REFINEMENT_RAW_MUTATION
            .try_with(|slot| {
                slot.try_borrow_mut()
                    .ok()
                    .map(|mut slot| std::mem::replace(&mut *slot, mutation))
            })
            .ok()
            .flatten()
            .flatten();
        Self(prior)
    }
}

#[cfg(test)]
impl Drop for RefinementMutationScope {
    fn drop(&mut self) {
        let _ = REFINEMENT_RAW_MUTATION.try_with(|slot| {
            if let Ok(mut slot) = slot.try_borrow_mut() {
                *slot = self.0;
            }
        });
    }
}

#[cfg(test)]
#[derive(Clone, Debug)]
struct RefinementScratchHooksObservation {
    p1_free_box_violation: bool,
    physical_evaluator_calls: u8,
    hydraulic_calls: u8,
    materialization_calls: u8,
    live_state_bits_before: [u64; 21],
    live_state_bits_after: [u64; 21],
    #[cfg(feature = "m1-trust-region-physical-stage")]
    budget_before: Option<crate::M1PhysicalBudgetSnapshot>,
    #[cfg(feature = "m1-trust-region-physical-stage")]
    budget_after: Option<crate::M1PhysicalBudgetSnapshot>,
    #[cfg(feature = "m1-trust-region-physical-stage")]
    global_before: RefinementScratchGlobalSnapshot,
    #[cfg(feature = "m1-trust-region-physical-stage")]
    global_after: RefinementScratchGlobalSnapshot,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone, Debug, PartialEq)]
struct RefinementScratchGlobalSnapshot {
    activity: crate::M1LoopActivityObservation,
    accepted_binding: Option<(crate::M1CoupledColumnInput, [f64; 21])>,
    phase_join_selections: u32,
    frozen_weighted_merit_entries: u32,
    complete_evaluator_entries: u32,
    hydraulic_entries: u32,
    actual_hydraulic_entries: u32,
    trial_entries: u32,
    materialization_entries: u32,
    final_assemblies: u32,
    work: (u32, u32),
    budget: Option<crate::M1PhysicalBudgetSnapshot>,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn refinement_scratch_global_snapshot() -> RefinementScratchGlobalSnapshot {
    let activity = crate::m1_loop_activity_observation_for_test();
    let accepted_binding = crate::m1_captured_accepted_adapter_binding_for_test();
    RefinementScratchGlobalSnapshot {
        activity: activity.clone(),
        accepted_binding,
        phase_join_selections: crate::m1_refinement_phase_join_selections_for_test(),
        frozen_weighted_merit_entries: refinement_frozen_weighted_merit_entries_for_test(),
        complete_evaluator_entries: activity.complete_evaluator_entries,
        hydraulic_entries: activity.hydraulic_entries,
        actual_hydraulic_entries: activity.actual_hydraulic_entries,
        trial_entries: activity.trial_entries,
        materialization_entries: activity.materialization_entries,
        final_assemblies: activity.final_assemblies,
        work: crate::m1_work_observation_for_test(),
        budget: crate::m1_physical_budget_observation_for_test(),
    }
}

#[cfg(test)]
fn m1_trust_region_refinement_scratch_hooks_for_test(
    capture: &serde_json::Value,
    mutation: Option<RefinementRawCandidateMutation>,
) -> Result<RefinementScratchHooksObservation, Box<RefinementObservationRefusal>> {
    let (_, prepared, initial_state) = refinement_decode_captured_face(capture).map_err(|_| {
        refinement_observation_refusal(
            "refinement_capture_decode",
            &SubproblemState::new(&super::M1TrustRegionSubproblemInput {
                raw_residual: [0.0; 21],
                raw_jacobian: [[0.0; 21]; 21],
                normalizers: [1.0; 21],
                scales: [1.0; 21],
                lower: [f64::NEG_INFINITY; 21],
                upper: [f64::INFINITY; 21],
                initial_radius: 1.0,
                base: [0.0; 21],
                test_work: super::M1TrustRegionTestWorkState::default(),
            }),
        )
    })?;
    #[cfg(feature = "m1-trust-region-physical-stage")]
    let global_before = refinement_scratch_global_snapshot();
    let _scope = RefinementMutationScope::install(mutation);
    let result = m1_trust_region_refinement_retained_face_for_test(capture, None);
    #[cfg(feature = "m1-trust-region-physical-stage")]
    let global_after = refinement_scratch_global_snapshot();
    let observed = match result {
        Ok(observed) => observed,
        Err(mut refusal) => {
            // The mutation scope remains live through this snapshot and restores
            // its predecessor on return through Drop.
            #[cfg(feature = "m1-trust-region-physical-stage")]
            {
                refusal.global_before = Some(global_before);
                refusal.global_after = Some(global_after);
            }
            return Err(refusal);
        }
    };
    let p1 = observed
        .p1_bits
        .iter()
        .copied()
        .map(f64::from_bits)
        .collect::<Vec<_>>();
    let p1_box_violation = p1.iter().enumerate().any(|(column, value)| {
        initial_state.free[column]
            && (*value < prepared.scaled_lower[column] || *value > prepared.scaled_upper[column])
    });
    Ok(RefinementScratchHooksObservation {
        p1_free_box_violation: p1_box_violation,
        physical_evaluator_calls: {
            #[cfg(feature = "m1-trust-region-physical-stage")]
            {
                (global_after.complete_evaluator_entries - global_before.complete_evaluator_entries)
                    as u8
            }
            #[cfg(not(feature = "m1-trust-region-physical-stage"))]
            {
                0
            }
        },
        hydraulic_calls: {
            #[cfg(feature = "m1-trust-region-physical-stage")]
            {
                (global_after.actual_hydraulic_entries - global_before.actual_hydraulic_entries)
                    as u8
            }
            #[cfg(not(feature = "m1-trust-region-physical-stage"))]
            {
                0
            }
        },
        materialization_calls: {
            #[cfg(feature = "m1-trust-region-physical-stage")]
            {
                (global_after.materialization_entries - global_before.materialization_entries) as u8
            }
            #[cfg(not(feature = "m1-trust-region-physical-stage"))]
            {
                0
            }
        },
        live_state_bits_before: observed.live_state_bits_entry,
        live_state_bits_after: observed.live_state_bits_exit,
        #[cfg(feature = "m1-trust-region-physical-stage")]
        budget_before: observed.budget_before,
        #[cfg(feature = "m1-trust-region-physical-stage")]
        budget_after: observed.budget_after,
        #[cfg(feature = "m1-trust-region-physical-stage")]
        global_before,
        #[cfg(feature = "m1-trust-region-physical-stage")]
        global_after,
    })
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
struct RefinementEligibilityProbe {
    lambda: f64,
    radius: f64,
    p0_outside_box: bool,
    p0_norm_value: f64,
}

#[cfg(test)]
impl RefinementEligibilityProbe {
    fn from_captured_face(capture: &serde_json::Value) -> Self {
        let operation = refinement_capture_operation(capture).expect("retained operation");
        let p0 = refinement_capture_vector(&operation["p_candidate"]).expect("retained p0");
        Self {
            lambda: refinement_capture_bit(&operation["lambda"]).expect("retained lambda"),
            radius: refinement_capture_bit(&operation["radius"]).expect("retained radius"),
            p0_outside_box: false,
            p0_norm_value: lambda_norm(&p0, None).expect("retained p0 norm"),
        }
    }
    fn with_lambda(mut self, lambda: f64) -> Self {
        self.lambda = lambda;
        self
    }
    fn with_trust_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }
    fn with_p0_free_coordinate_outside_box(mut self) -> Self {
        self.p0_outside_box = true;
        self
    }
    fn p0_norm(self) -> f64 {
        self.p0_norm_value
    }
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct RefinementBvls02Outcome {
    outcome: String,
    kind: Option<M1TrustRegionRefusalKind>,
    reason: Option<&'static str>,
    coordinate: Option<usize>,
    crossing_action: Option<super::M1TrustRegionKktAction>,
    lower_mask_before: Vec<bool>,
    upper_mask_before: Vec<bool>,
    lower_mask: Vec<bool>,
    upper_mask: Vec<bool>,
    work: u64,
}

#[cfg(test)]
fn refinement_bvls02_face_outcome(
    prepared: &PreparedSubproblem,
    state: &mut SubproblemState,
    solution: &FaceSolution,
) -> RefinementBvls02Outcome {
    let lower_mask_before = state.lower_active.to_vec();
    let upper_mask_before = state.upper_active.to_vec();
    match activate_first_crossing(prepared, state, solution) {
        Ok(true) => RefinementBvls02Outcome {
            outcome: "crossing".to_owned(),
            kind: None,
            reason: None,
            coordinate: state.events.last().map(|event| event.selected_coordinate),
            crossing_action: state.events.last().map(|event| event.action),
            lower_mask_before,
            upper_mask_before,
            lower_mask: state.lower_active.to_vec(),
            upper_mask: state.upper_active.to_vec(),
            work: 0,
        },
        Ok(false) => match kkt_transition(prepared, state, solution) {
            Ok(value) => RefinementBvls02Outcome {
                outcome: if value.values.is_some() {
                    "return".to_owned()
                } else {
                    "transition".to_owned()
                },
                kind: None,
                reason: None,
                coordinate: None,
                crossing_action: None,
                lower_mask_before,
                upper_mask_before,
                lower_mask: state.lower_active.to_vec(),
                upper_mask: state.upper_active.to_vec(),
                work: u64::from(value.observation.scalar_operations),
            },
            Err(refusal) => RefinementBvls02Outcome {
                outcome: "refusal".to_owned(),
                kind: Some(refusal.kind),
                reason: refusal.optimality_reason,
                coordinate: refusal.optimality_coordinate,
                crossing_action: None,
                lower_mask_before,
                upper_mask_before,
                lower_mask: state.lower_active.to_vec(),
                upper_mask: state.upper_active.to_vec(),
                work: 0,
            },
        },
        Err(refusal) => RefinementBvls02Outcome {
            outcome: "refusal".to_owned(),
            kind: Some(refusal.kind),
            reason: refusal.optimality_reason,
            coordinate: refusal.optimality_coordinate,
            crossing_action: None,
            lower_mask_before,
            upper_mask_before,
            lower_mask: state.lower_active.to_vec(),
            upper_mask: state.upper_active.to_vec(),
            work: 0,
        },
    }
}

#[cfg(test)]
#[derive(Clone, Debug)]
struct RefinementEligibilityObservation {
    action: String,
    maximum_corrections: u8,
    bvls02_outcome: String,
    bvls02: RefinementBvls02Outcome,
    lower_mask_before: Vec<bool>,
    upper_mask_before: Vec<bool>,
    lower_mask_after: Vec<bool>,
    upper_mask_after: Vec<bool>,
    bvls02_work: u64,
    attempted_work_before: u64,
    attempted_work_after: u64,
    denied_work: u64,
}

#[cfg(test)]
fn refinement_apply_eligibility_probe(
    input: &mut super::M1TrustRegionSubproblemInput,
    prepared: &PreparedSubproblem,
    solution: &mut FaceSolution,
    probe: RefinementEligibilityProbe,
) {
    input.initial_radius = probe.radius;
    solution.trace.lambda = probe.lambda;
    if probe.p0_outside_box {
        if let Some((&column, value)) = solution
            .free_ids
            .first()
            .zip(solution.trace.step.first_mut())
        {
            *value = prepared.scaled_upper[column] + 1.0;
        }
    }
}

#[cfg(test)]
fn m1_trust_region_bvls02_outcome_for_test(
    capture: &serde_json::Value,
    probe: RefinementEligibilityProbe,
) -> Result<RefinementBvls02Outcome, Box<RefinementObservationRefusal>> {
    let (mut input, prepared, mut state, mut solution) = refinement_solve_captured_face(capture)
        .map_err(|_| {
            refinement_observation_refusal(
                "refinement_capture_decode",
                &SubproblemState::new(&super::M1TrustRegionSubproblemInput {
                    raw_residual: [0.0; 21],
                    raw_jacobian: [[0.0; 21]; 21],
                    normalizers: [1.0; 21],
                    scales: [1.0; 21],
                    lower: [f64::NEG_INFINITY; 21],
                    upper: [f64::INFINITY; 21],
                    initial_radius: 1.0,
                    base: [0.0; 21],
                    test_work: super::M1TrustRegionTestWorkState::default(),
                }),
            )
        })?;
    refinement_apply_eligibility_probe(&mut input, &prepared, &mut solution, probe);
    Ok(refinement_bvls02_face_outcome(
        &prepared, &mut state, &solution,
    ))
}

#[cfg(test)]
fn m1_trust_region_refinement_eligibility_for_test(
    capture: &serde_json::Value,
    probe: RefinementEligibilityProbe,
    _baseline: Option<&RefinementBvls02Outcome>,
) -> Result<RefinementEligibilityObservation, Box<RefinementObservationRefusal>> {
    let (mut input, prepared, mut state, mut solution) = refinement_solve_captured_face(capture)
        .map_err(|_| {
            refinement_observation_refusal(
                "refinement_capture_decode",
                &SubproblemState::new(&super::M1TrustRegionSubproblemInput {
                    raw_residual: [0.0; 21],
                    raw_jacobian: [[0.0; 21]; 21],
                    normalizers: [1.0; 21],
                    scales: [1.0; 21],
                    lower: [f64::NEG_INFINITY; 21],
                    upper: [f64::INFINITY; 21],
                    initial_radius: 1.0,
                    base: [0.0; 21],
                    test_work: super::M1TrustRegionTestWorkState::default(),
                }),
            )
        })?;
    refinement_apply_eligibility_probe(&mut input, &prepared, &mut solution, probe);
    let before = state.refinement_work.scalar_operations;
    let _observation_scope = RefinementObservationScope::begin();
    let result = refine_face_before_crossing(&input, &prepared, &mut state, &mut solution);
    let trace = refinement_observation_take();
    let refined = result.is_ok()
        && trace
            .as_ref()
            .map_or(false, |value| value.factor_applications == 2);
    let bvls02 = refinement_bvls02_face_outcome(&prepared, &mut state, &solution);
    let after = state.refinement_work.scalar_operations;
    Ok(RefinementEligibilityObservation {
        action: if refined {
            "refine".to_owned()
        } else {
            "bvls02".to_owned()
        },
        maximum_corrections: if refined { 2 } else { 0 },
        bvls02_outcome: bvls02.outcome.clone(),
        lower_mask_before: bvls02.lower_mask_before.clone(),
        upper_mask_before: bvls02.upper_mask_before.clone(),
        lower_mask_after: bvls02.lower_mask.clone(),
        upper_mask_after: bvls02.upper_mask.clone(),
        bvls02_work: bvls02.work,
        bvls02,
        attempted_work_before: before,
        attempted_work_after: after,
        denied_work: after - before,
    })
}

struct RefinementScratch {
    p1: [f64; 21],
}

fn refinement_refusal(
    reason: &'static str,
    state: &SubproblemState,
    raw_max: f64,
) -> super::M1TrustRegionSubproblemRefusal {
    let mut refusal = state_refusal(
        M1TrustRegionRefusalKind::TrustRegionRefinementIndeterminate,
        state,
        raw_max,
    );
    refusal.optimality_reason = Some(reason);
    refusal
}

fn refinement_metadata_refusal(
    state: &SubproblemState,
    raw_max: f64,
    solution: &FaceSolution,
    original_trust_radius: f64,
    reason: super::M1TrustRegionRefinementReason,
    correction_position: Option<super::M1TrustRegionCorrectionPosition>,
    guard_stage: super::M1TrustRegionRefinementGuardStage,
    row: Option<u8>,
    term: Option<u8>,
    coordinate: Option<u8>,
    scratch_box_feasible: Option<bool>,
) -> super::M1TrustRegionSubproblemRefusal {
    let reason_text = match reason {
        super::M1TrustRegionRefinementReason::NonfiniteOperand => "refinement_nonfinite_operand",
        super::M1TrustRegionRefinementReason::SplitterOverflow => "refinement_splitter_overflow",
        super::M1TrustRegionRefinementReason::UnderflowToZero => "refinement_underflow_to_zero",
        super::M1TrustRegionRefinementReason::SubnormalIntermediate => {
            "refinement_subnormal_intermediate"
        }
        super::M1TrustRegionRefinementReason::FactorOutput => "refinement_factor_output",
        super::M1TrustRegionRefinementReason::CoordinateUpdate => "refinement_coordinate_update",
        super::M1TrustRegionRefinementReason::ActiveValue => "refinement_active_value",
        super::M1TrustRegionRefinementReason::ScratchRadius => "refinement_scratch_radius",
        super::M1TrustRegionRefinementReason::FinalBox => "refinement_final_box",
        super::M1TrustRegionRefinementReason::FinalRadius => "refinement_final_radius",
    };
    let mut refusal = refinement_refusal(reason_text, state, raw_max);
    refusal.refinement = Some(Box::new(super::M1TrustRegionRefinementFailureMetadata {
        reason,
        correction_position,
        guard_stage,
        row,
        term,
        coordinate,
        lower_active: state.lower_active,
        upper_active: state.upper_active,
        free: state.free,
        lambda: solution.trace.lambda,
        original_trust_radius,
        reduced_face_radius: solution.radius,
        scratch_box_feasible,
    }));
    refusal
}

fn refinement_p0(
    state_p: &[f64; 21],
    solution: &FaceSolution,
    mut refinement_work: Option<&mut RefinementWorkLedger>,
) -> Result<[f64; 21], &'static str> {
    if solution.free_ids.len() != solution.trace.step.len() {
        return Err("refinement_face_dimension");
    }
    let mut p0 = *state_p;
    for (slot, &column) in solution.free_ids.iter().enumerate() {
        if let Some(work) = refinement_work.as_deref_mut() {
            work.guard();
        }
        p0[column] = solution.trace.step[slot];
        if let Some(work) = refinement_work.as_deref_mut() {
            work.guard();
        }
        if p0[column].to_bits() != solution.trace.step[slot].to_bits() {
            return Err("refinement_factor_output");
        }
    }
    Ok(p0)
}

fn refinement_box_admit(
    prepared: &PreparedSubproblem,
    p: &[f64; 21],
    free_ids: Option<&[usize]>,
    mut refinement_work: Option<&mut RefinementWorkLedger>,
) -> bool {
    let mut admit = |column: usize| {
        if let Some(work) = refinement_work.as_deref_mut() {
            work.guard();
        }
        if p[column] < prepared.scaled_lower[column] {
            return false;
        }
        if let Some(work) = refinement_work.as_deref_mut() {
            work.guard();
        }
        p[column] <= prepared.scaled_upper[column]
    };
    if let Some(free_ids) = free_ids {
        for &column in free_ids {
            if !admit(column) {
                return false;
            }
        }
    } else {
        for column in 0..21 {
            if !admit(column) {
                return false;
            }
        }
    }
    true
}

fn refinement_active_admit(
    p: &[f64; 21],
    lower_active: &[bool; 21],
    upper_active: &[bool; 21],
    state_p: &[f64; 21],
) -> Result<RefinementDot2Counters, RefinementDot2Refusal> {
    let mut counters = RefinementDot2Counters::default();
    for column in 0..21 {
        if let Err(reason) = refinement_dot2_guard(p[column], &mut counters) {
            return Err(RefinementDot2Refusal {
                reason,
                arithmetic: counters.arithmetic,
                guards: counters.guards,
                exceptional_guards: counters.exceptional_guards,
                signed_zero_values: counters.signed_zero_values,
            });
        }
        if lower_active[column] || upper_active[column] {
            counters.guards += 1;
            if p[column].to_bits() == state_p[column].to_bits() {
                continue;
            }
            return Err(RefinementDot2Refusal {
                reason: RefinementDot2Reason::NonfiniteOperand,
                arithmetic: counters.arithmetic,
                guards: counters.guards,
                exceptional_guards: counters.exceptional_guards,
                signed_zero_values: counters.signed_zero_values,
            });
        }
    }
    Ok(counters)
}

fn refinement_eligibility_active_admit(
    p: &[f64; 21],
    lower_active: &[bool; 21],
    upper_active: &[bool; 21],
    state_p: &[f64; 21],
    mut refinement_work: Option<&mut RefinementWorkLedger>,
) -> bool {
    for column in 0..21 {
        if let Some(work) = refinement_work.as_deref_mut() {
            work.guard();
        }
        if !p[column].is_finite() {
            return false;
        }
        if lower_active[column] || upper_active[column] {
            if let Some(work) = refinement_work.as_deref_mut() {
                work.guard();
            }
            if p[column].to_bits() != state_p[column].to_bits() {
                return false;
            }
        }
    }
    true
}

fn refinement_first_residual(
    prepared: &PreparedSubproblem,
    p: &[f64; 21],
) -> Result<([f64; 21], RefinementDot2Counters), RefinementDot2Refusal> {
    let mut counters = RefinementDot2Counters::default();
    let result = (|| -> Result<[f64; 21], RefinementDot2Reason> {
        let mut residual = [0.0; 21];
        for row in 0..21 {
            refinement_dot2_guard(prepared.weighted_residual[row], &mut counters)?;
            let mut sum = 0.0;
            for column in 0..21 {
                refinement_dot2_guard(prepared.weighted_matrix[row][column], &mut counters)?;
                refinement_dot2_guard(p[column], &mut counters)?;
                let product = refinement_dot2_mul(
                    prepared.weighted_matrix[row][column],
                    p[column],
                    &mut counters,
                )?;
                sum = refinement_dot2_add(sum, product, &mut counters)?;
            }
            residual[row] =
                refinement_dot2_add(prepared.weighted_residual[row], sum, &mut counters)?;
        }
        Ok(residual)
    })();
    match result {
        Ok(residual) => Ok((residual, counters)),
        Err(reason) => Err(RefinementDot2Refusal {
            reason,
            arithmetic: counters.arithmetic,
            guards: counters.guards,
            exceptional_guards: counters.exceptional_guards,
            signed_zero_values: counters.signed_zero_values,
        }),
    }
}

fn refinement_second_residual(
    prepared: &PreparedSubproblem,
    p: &[f64; 21],
) -> Result<([f64; 21], RefinementDot2Counters), RefinementDot2Refusal> {
    let mut residual = [0.0; 21];
    let mut counters = RefinementDot2Counters::default();
    for row in 0..21 {
        let mut terms = [0.0; 22];
        let mut weights = [0.0; 22];
        terms[0] = prepared.weighted_residual[row];
        weights[0] = 1.0;
        for column in 0..21 {
            terms[column + 1] = prepared.weighted_matrix[row][column];
            weights[column + 1] = p[column];
        }
        match m1_trust_region_refinement_dot2_for_test(terms, weights) {
            Ok(observation) => {
                counters.arithmetic += observation.arithmetic;
                counters.guards += observation.guards;
                counters.exceptional_guards += observation.exceptional_guards;
                counters.signed_zero_values += observation.signed_zero_values;
                residual[row] = observation.value;
            }
            Err(refusal) => {
                counters.arithmetic += refusal.arithmetic;
                counters.guards += refusal.guards;
                counters.exceptional_guards += refusal.exceptional_guards;
                counters.signed_zero_values += refusal.signed_zero_values;
                return Err(RefinementDot2Refusal {
                    reason: refusal.reason,
                    arithmetic: counters.arithmetic,
                    guards: counters.guards,
                    exceptional_guards: counters.exceptional_guards,
                    signed_zero_values: counters.signed_zero_values,
                });
            }
        }
    }
    Ok((residual, counters))
}

fn refine_face_before_crossing_inner(
    input: &super::M1TrustRegionSubproblemInput,
    prepared: &PreparedSubproblem,
    state: &mut SubproblemState,
    solution: &mut FaceSolution,
) -> Result<(), super::M1TrustRegionSubproblemRefusal> {
    let eligibility_scalar_before = state.refinement_work.scalar_operations;
    let eligibility_guards_before = state.refinement_work.guard_events;
    // The frozen primitive admits both signed zero representations of lambda.
    if solution.trace.lambda != 0.0 {
        return Ok(());
    }
    let p_before = state.p;
    let p0 = refinement_p0(&p_before, solution, Some(&mut state.refinement_work))
        .map_err(|_| refinement_refusal("refinement_factor_output", state, prepared.raw_max))?;
    #[cfg(test)]
    refinement_observe(|trace| {
        trace.p0 = Some(p0);
        trace.lower_mask_p0 = Some(state.lower_active);
        trace.upper_mask_p0 = Some(state.upper_active);
    });
    if !refinement_box_admit(prepared, &p0, None, Some(&mut state.refinement_work))
        || !refinement_eligibility_active_admit(
            &p0,
            &state.lower_active,
            &state.upper_active,
            &state.p,
            Some(&mut state.refinement_work),
        )
    {
        return Ok(());
    }
    let p0_norm = lambda_norm(&p0, Some(&mut state.refinement_work))
        .map_err(|_| refinement_refusal("refinement_nonfinite_operand", state, prepared.raw_max))?;
    state.refinement_work.guard();
    if !(p0_norm < input.initial_radius) {
        return Ok(());
    }
    #[cfg(test)]
    refinement_observe(|trace| {
        trace.eligibility_scalar_operations =
            Some(state.refinement_work.scalar_operations - eligibility_scalar_before);
        trace.eligibility_guard_events =
            Some(state.refinement_work.guard_events - eligibility_guards_before);
    });
    let (required_scalar, required_guards) = refinement_required_work(solution.free_ids.len());
    state.refinement_work.attempted_entries += 1;
    // The p0 ordered norm has already executed as the eligibility predicate.
    // Reserve only the not-yet-executed scalar work while retaining that prior
    // 43-operation charge exactly once in the proposal ledger.
    let eligibility_scalar = state.refinement_work.scalar_operations - eligibility_scalar_before;
    let eligibility_guards = state.refinement_work.guard_events - eligibility_guards_before;
    let Some(remaining_scalar) = required_scalar.checked_sub(eligibility_scalar) else {
        state.refinement_work.denied_entries += 1;
        return Err(state_refusal(
            M1TrustRegionRefusalKind::WorkCap,
            state,
            prepared.raw_max,
        ));
    };
    let Some(remaining_guards) = required_guards.checked_sub(eligibility_guards) else {
        state.refinement_work.denied_entries += 1;
        return Err(state_refusal(
            M1TrustRegionRefusalKind::WorkCap,
            state,
            prepared.raw_max,
        ));
    };
    if state.refinement_work.scalar_operations + remaining_scalar > REFINEMENT_SCALAR_CAP
        || state.refinement_work.guard_events + remaining_guards > REFINEMENT_GUARD_CAP
    {
        state.refinement_work.denied_entries += 1;
        return Err(state_refusal(
            M1TrustRegionRefusalKind::WorkCap,
            state,
            prepared.raw_max,
        ));
    }
    let factor = solution
        .factor
        .as_ref()
        .ok_or_else(|| refinement_refusal("refinement_factor_output", state, prepared.raw_max))?;
    #[cfg(test)]
    refinement_observe(|trace| {
        trace.factor_identity = Some(factor as *const JacobiFactor as usize)
    });
    let residual0 = match refinement_first_residual(prepared, &p0) {
        Ok((residual, counters)) => {
            state.refinement_work.absorb_dot2(counters);
            state.refinement_work.first_residual_operations += u64::from(counters.arithmetic);
            residual
        }
        Err(refusal) => {
            state.refinement_work.absorb_dot2(RefinementDot2Counters {
                arithmetic: refusal.arithmetic,
                guards: refusal.guards,
                exceptional_guards: refusal.exceptional_guards,
                signed_zero_values: refusal.signed_zero_values,
            });
            state.refinement_work.first_residual_operations += u64::from(refusal.arithmetic);
            return Err(refinement_refusal(
                refusal.reason(),
                state,
                prepared.raw_max,
            ));
        }
    };
    let factor_before = state.refinement_work.scalar_operations;
    #[cfg(test)]
    refinement_observe(|trace| {
        trace.factor_applications += 1;
        trace
            .factor_application_identities
            .push(factor as *const JacobiFactor as usize);
    });
    let delta0 = lambda_step(factor, &residual0, 0.0, Some(&mut state.refinement_work));
    state.refinement_work.factor_application_operations +=
        state.refinement_work.scalar_operations - factor_before;
    let delta0 = delta0
        .map_err(|_| refinement_refusal("refinement_factor_output", state, prepared.raw_max))?;
    #[cfg(test)]
    refinement_observe(|trace| trace.delta0 = Some(delta0.clone()));
    if delta0.len() != solution.free_ids.len() {
        return Err(refinement_refusal(
            "refinement_factor_output",
            state,
            prepared.raw_max,
        ));
    }
    let mut factor_counters = RefinementDot2Counters::default();
    for &value in &delta0 {
        if refinement_dot2_guard(value, &mut factor_counters).is_err() {
            state.refinement_work.absorb_dot2(factor_counters);
            return Err(refinement_refusal(
                "refinement_factor_output",
                state,
                prepared.raw_max,
            ));
        }
    }
    state.refinement_work.absorb_dot2(factor_counters);
    let mut scratch = RefinementScratch { p1: p0 };
    for (slot, &column) in solution.free_ids.iter().enumerate() {
        let mut counters = RefinementDot2Counters::default();
        let updated = refinement_dot2_add(scratch.p1[column], delta0[slot], &mut counters);
        state.refinement_work.absorb_dot2(counters);
        state.refinement_work.coordinate_update_operations += u64::from(counters.arithmetic);
        scratch.p1[column] = updated.map_err(|_| {
            refinement_refusal("refinement_coordinate_update", state, prepared.raw_max)
        })?;
    }
    #[cfg(test)]
    let refinement_test_radius =
        refinement_mutate(1, &mut scratch.p1).unwrap_or(input.initial_radius);
    #[cfg(not(test))]
    let refinement_test_radius = input.initial_radius;
    let p1_box_feasible = refinement_box_admit(
        prepared,
        &scratch.p1,
        Some(&solution.free_ids),
        Some(&mut state.refinement_work),
    );
    let p1_active = refinement_active_admit(
        &scratch.p1,
        &state.lower_active,
        &state.upper_active,
        &state.p,
    );
    match p1_active {
        Ok(counters) => state.refinement_work.absorb_dot2(counters),
        Err(refusal) => {
            state.refinement_work.absorb_dot2(RefinementDot2Counters {
                arithmetic: refusal.arithmetic,
                guards: refusal.guards,
                exceptional_guards: refusal.exceptional_guards,
                signed_zero_values: refusal.signed_zero_values,
            });
            return Err(refinement_refusal(
                "refinement_active_value",
                state,
                prepared.raw_max,
            ));
        }
    }
    #[cfg(test)]
    refinement_observe(|trace| {
        trace.p1 = Some(scratch.p1);
        trace.p1_box_feasible = Some(p1_box_feasible);
        trace.lower_mask_p1 = Some(state.lower_active);
        trace.upper_mask_p1 = Some(state.upper_active);
    });
    let p1_norm = lambda_norm(&scratch.p1, Some(&mut state.refinement_work))
        .map_err(|_| refinement_refusal("refinement_nonfinite_operand", state, prepared.raw_max))?;
    state.refinement_work.guard();
    if p1_norm > refinement_test_radius {
        return Err(refinement_metadata_refusal(
            state,
            prepared.raw_max,
            solution,
            input.initial_radius,
            M1TrustRegionRefinementReason::ScratchRadius,
            Some(M1TrustRegionCorrectionPosition::First),
            M1TrustRegionRefinementGuardStage::RadiusComparison,
            None,
            None,
            None,
            Some(p1_box_feasible),
        ));
    }
    let residual1 = match refinement_second_residual(prepared, &scratch.p1) {
        Ok((residual, counters)) => {
            state.refinement_work.absorb_dot2(counters);
            state.refinement_work.second_dot2_operations += u64::from(counters.arithmetic);
            residual
        }
        Err(refusal) => {
            state.refinement_work.absorb_dot2(RefinementDot2Counters {
                arithmetic: refusal.arithmetic,
                guards: refusal.guards,
                exceptional_guards: refusal.exceptional_guards,
                signed_zero_values: refusal.signed_zero_values,
            });
            state.refinement_work.second_dot2_operations += u64::from(refusal.arithmetic);
            return Err(refinement_refusal(
                refusal.reason(),
                state,
                prepared.raw_max,
            ));
        }
    };
    #[cfg(test)]
    refinement_observe(|trace| trace.residual1 = Some(residual1));
    let factor_before = state.refinement_work.scalar_operations;
    #[cfg(test)]
    refinement_observe(|trace| {
        trace.factor_applications += 1;
        trace
            .factor_application_identities
            .push(factor as *const JacobiFactor as usize);
    });
    let delta1 = lambda_step(factor, &residual1, 0.0, Some(&mut state.refinement_work));
    state.refinement_work.factor_application_operations +=
        state.refinement_work.scalar_operations - factor_before;
    let delta1 = delta1
        .map_err(|_| refinement_refusal("refinement_factor_output", state, prepared.raw_max))?;
    #[cfg(test)]
    refinement_observe(|trace| trace.delta1 = Some(delta1.clone()));
    if delta1.len() != solution.free_ids.len() {
        return Err(refinement_refusal(
            "refinement_factor_output",
            state,
            prepared.raw_max,
        ));
    }
    let mut factor_counters = RefinementDot2Counters::default();
    for &value in &delta1 {
        if refinement_dot2_guard(value, &mut factor_counters).is_err() {
            state.refinement_work.absorb_dot2(factor_counters);
            return Err(refinement_refusal(
                "refinement_factor_output",
                state,
                prepared.raw_max,
            ));
        }
    }
    state.refinement_work.absorb_dot2(factor_counters);
    let mut p2 = scratch.p1;
    for (slot, &column) in solution.free_ids.iter().enumerate() {
        let mut counters = RefinementDot2Counters::default();
        let updated = refinement_dot2_add(p2[column], delta1[slot], &mut counters);
        state.refinement_work.absorb_dot2(counters);
        state.refinement_work.coordinate_update_operations += u64::from(counters.arithmetic);
        let updated = updated.map_err(|_| {
            refinement_refusal("refinement_coordinate_update", state, prepared.raw_max)
        })?;
        p2[column] = updated;
    }
    #[cfg(test)]
    let _ = refinement_mutate(2, &mut p2);
    if !refinement_box_admit(prepared, &p2, None, Some(&mut state.refinement_work)) {
        let (coordinate, guard_stage) = (0..21)
            .find_map(|column| {
                (p2[column] < prepared.scaled_lower[column])
                    .then_some((
                        column as u8,
                        M1TrustRegionRefinementGuardStage::FinalBoxLower,
                    ))
                    .or_else(|| {
                        (p2[column] > prepared.scaled_upper[column]).then_some((
                            column as u8,
                            M1TrustRegionRefinementGuardStage::FinalBoxUpper,
                        ))
                    })
            })
            .expect("failed final box admission identifies a coordinate");
        return Err(refinement_metadata_refusal(
            state,
            prepared.raw_max,
            solution,
            input.initial_radius,
            M1TrustRegionRefinementReason::FinalBox,
            Some(M1TrustRegionCorrectionPosition::Second),
            guard_stage,
            None,
            None,
            Some(coordinate),
            Some(p1_box_feasible),
        ));
    }
    let p2_active =
        refinement_active_admit(&p2, &state.lower_active, &state.upper_active, &state.p);
    match p2_active {
        Ok(counters) => state.refinement_work.absorb_dot2(counters),
        Err(refusal) => {
            state.refinement_work.absorb_dot2(RefinementDot2Counters {
                arithmetic: refusal.arithmetic,
                guards: refusal.guards,
                exceptional_guards: refusal.exceptional_guards,
                signed_zero_values: refusal.signed_zero_values,
            });
            let coordinate = (0..21)
                .find(|&column| {
                    (state.lower_active[column] || state.upper_active[column])
                        && p2[column].to_bits() != state.p[column].to_bits()
                })
                .map(|column| column as u8);
            return Err(refinement_metadata_refusal(
                state,
                prepared.raw_max,
                solution,
                input.initial_radius,
                M1TrustRegionRefinementReason::ActiveValue,
                Some(M1TrustRegionCorrectionPosition::Second),
                M1TrustRegionRefinementGuardStage::ActiveEquality,
                None,
                None,
                coordinate,
                Some(p1_box_feasible),
            ));
        }
    }
    let p2_norm = lambda_norm(&p2, Some(&mut state.refinement_work))
        .map_err(|_| refinement_refusal("refinement_final_radius", state, prepared.raw_max))?;
    state.refinement_work.guard();
    if p2_norm > input.initial_radius {
        return Err(refinement_refusal(
            "refinement_final_radius",
            state,
            prepared.raw_max,
        ));
    }
    #[cfg(test)]
    refinement_observe(|trace| {
        trace.p2 = Some(p2);
        trace.lower_mask_p2 = Some(state.lower_active);
        trace.upper_mask_p2 = Some(state.upper_active);
        trace.lower_mask_exit = Some(state.lower_active);
        trace.upper_mask_exit = Some(state.upper_active);
    });
    for (slot, &column) in solution.free_ids.iter().enumerate() {
        state.refinement_work.guard();
        solution.trace.step[slot] = p2[column];
        if solution.trace.step[slot].to_bits() != p2[column].to_bits() {
            return Err(refinement_refusal(
                "refinement_coordinate_update",
                state,
                prepared.raw_max,
            ));
        }
    }
    solution.refined = true;
    Ok(())
}

fn refine_face_before_crossing(
    input: &super::M1TrustRegionSubproblemInput,
    prepared: &PreparedSubproblem,
    state: &mut SubproblemState,
    solution: &mut FaceSolution,
) -> Result<(), super::M1TrustRegionSubproblemRefusal> {
    #[cfg(test)]
    refinement_observe(|trace| trace.live_state_entry = Some(state.p.map(f64::to_bits)));
    let result = refine_face_before_crossing_inner(input, prepared, state, solution);
    #[cfg(test)]
    refinement_observe(|trace| trace.live_state_exit = Some(state.p.map(f64::to_bits)));
    result
}

fn activate_first_crossing(
    prepared: &PreparedSubproblem,
    state: &mut SubproblemState,
    solution: &FaceSolution,
) -> Result<bool, super::M1TrustRegionSubproblemRefusal> {
    let previous_p = state.p;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let lower_before = state.lower_active;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let upper_before = state.upper_active;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let free_before = state.free;
    for (slot, &col) in solution.free_ids.iter().enumerate() {
        state.p[col] = solution.trace.step[slot];
    }
    let crossing = first_crossing(
        &state.p,
        &previous_p,
        &solution.free_ids,
        &prepared.scaled_lower,
        &prepared.scaled_upper,
    )
    .map_err(|kind| state_refusal(kind, state, prepared.raw_max))?;
    let Some((theta, col, is_lower)) = crossing else {
        return Ok(false);
    };
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let candidate_p = state.p;
    if state.completed_pivots >= 42 {
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        {
            capture_activation(
                state,
                solution,
                previous_p,
                candidate_p,
                (lower_before, upper_before, free_before),
                (theta, col, is_lower),
                false,
            );
            capture_refusal(
                "activate-first-crossing",
                M1TrustRegionRefusalKind::FacePivotLimit,
                state,
            );
        }
        return Err(state_refusal(
            M1TrustRegionRefusalKind::FacePivotLimit,
            state,
            prepared.raw_max,
        ));
    }
    for &id in &solution.free_ids {
        state.p[id] = previous_p[id] + theta * (state.p[id] - previous_p[id]);
        if !state.p[id].is_finite() {
            return Err(state_refusal(
                M1TrustRegionRefusalKind::SvdNonFinite,
                state,
                prepared.raw_max,
            ));
        }
    }
    state.p[col] = if is_lower {
        prepared.scaled_lower[col]
    } else {
        prepared.scaled_upper[col]
    };
    state.free[col] = false;
    state.lower_active[col] = is_lower;
    state.upper_active[col] = !is_lower;
    state.completed_pivots += 1;
    state.events.push(super::M1TrustRegionKktEvent {
        action: if is_lower {
            super::M1TrustRegionKktAction::ActivateLower
        } else {
            super::M1TrustRegionKktAction::ActivateUpper
        },
        selected_coordinate: col,
        lower_mask: state.lower_active,
        upper_mask: state.upper_active,
        free_mask: state.free,
        g: None,
        lambda_times_p: None,
        h: None,
    });
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    capture_activation(
        state,
        solution,
        previous_p,
        candidate_p,
        (lower_before, upper_before, free_before),
        (theta, col, is_lower),
        true,
    );
    Ok(true)
}

fn kkt_transition(
    prepared: &PreparedSubproblem,
    state: &mut SubproblemState,
    solution: &FaceSolution,
) -> Result<KktTransitionOutcome, super::M1TrustRegionSubproblemRefusal> {
    let lambda = solution.trace.lambda;
    let values = match {
        let mut accounting = KktAccounting::tracked(&mut state.refinement_work, solution.refined);
        box_feasible_kkt_values_with_accounting(
            &prepared.weighted_matrix,
            &prepared.weighted_residual,
            &state.p,
            lambda,
            &mut accounting,
        )
    } {
        Ok(values) => values,
        Err(failure) => {
            #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
            capture_optimality_failure(state, solution, prepared, None, None, failure);
            return Err(optimality_state_refusal(
                failure,
                state,
                prepared.raw_max,
                solution,
            ));
        }
    };
    let p = state.p;
    let lower_active = state.lower_active;
    let upper_active = state.upper_active;
    let initial_free = state.initial_free;
    let assessment = match {
        let mut accounting = KktAccounting::tracked(&mut state.refinement_work, solution.refined);
        assess_optimality(
            prepared,
            &p,
            &lower_active,
            &upper_active,
            &initial_free,
            &values,
            lambda,
            &mut accounting,
        )
    } {
        Ok(assessment) => assessment,
        Err(failure) => {
            #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
            capture_optimality_failure(state, solution, prepared, Some(&values), None, failure);
            return Err(optimality_state_refusal(
                failure,
                state,
                prepared.raw_max,
                solution,
            ));
        }
    };
    finish_kkt_transition(prepared, state, solution, values, assessment)
}

fn finish_kkt_transition(
    prepared: &PreparedSubproblem,
    state: &mut SubproblemState,
    solution: &FaceSolution,
    values: KktValues,
    assessment: OptimalityAssessment,
) -> Result<KktTransitionOutcome, super::M1TrustRegionSubproblemRefusal> {
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let p_before = state.p;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let lower_before = state.lower_active;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let upper_before = state.upper_active;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let free_before = state.free;
    if let Some((col, is_lower)) = assessment.release {
        if state.completed_pivots >= 42 {
            #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
            {
                capture_release(
                    state,
                    &ReleaseCapture {
                        solution,
                        prepared,
                        values: &values,
                        assessment: &assessment,
                        p_before,
                        masks_before: (lower_before, upper_before, free_before),
                        selected: (col, is_lower),
                        applied: false,
                    },
                );
                capture_refusal(
                    "kkt-release",
                    M1TrustRegionRefusalKind::FacePivotLimit,
                    state,
                );
            }
            return Err(state_refusal(
                M1TrustRegionRefusalKind::FacePivotLimit,
                state,
                prepared.raw_max,
            ));
        }
        state.events.push(super::M1TrustRegionKktEvent {
            action: if is_lower {
                super::M1TrustRegionKktAction::ReleaseLower
            } else {
                super::M1TrustRegionKktAction::ReleaseUpper
            },
            selected_coordinate: col,
            lower_mask: state.lower_active,
            upper_mask: state.upper_active,
            free_mask: state.free,
            g: Some(values.gradient),
            lambda_times_p: Some(values.lambda_times_step),
            h: Some(values.h),
        });
        state.lower_active[col] = false;
        state.upper_active[col] = false;
        state.free[col] = true;
        state.completed_pivots += 1;
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        capture_release(
            state,
            &ReleaseCapture {
                solution,
                prepared,
                values: &values,
                assessment: &assessment,
                p_before,
                masks_before: (lower_before, upper_before, free_before),
                selected: (col, is_lower),
                applied: true,
            },
        );
        return Ok(KktTransitionOutcome {
            values: None,
            observation: assessment.observation,
        });
    }
    if let Some(coordinate) = assessment.free_refusal {
        let failure = optimality_failure("optimality_free_stationarity", coordinate);
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        capture_optimality_failure(
            state,
            solution,
            prepared,
            Some(&values),
            Some(&assessment),
            failure,
        );
        return Err(optimality_state_refusal(
            failure,
            state,
            prepared.raw_max,
            solution,
        ));
    }
    Ok(return_kkt_transition(
        state, solution, prepared, values, assessment,
    ))
}

fn return_kkt_transition(
    state: &mut SubproblemState,
    solution: &FaceSolution,
    prepared: &PreparedSubproblem,
    values: KktValues,
    assessment: OptimalityAssessment,
) -> KktTransitionOutcome {
    state.events.push(super::M1TrustRegionKktEvent {
        action: super::M1TrustRegionKktAction::Return,
        selected_coordinate: usize::MAX,
        lower_mask: state.lower_active,
        upper_mask: state.upper_active,
        free_mask: state.free,
        g: Some(values.gradient),
        lambda_times_p: Some(values.lambda_times_step),
        h: Some(values.h),
    });
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    capture_return(state, solution, prepared, &values, &assessment);
    KktTransitionOutcome {
        values: Some(values),
        observation: assessment.observation,
    }
}

pub(crate) fn solve_subproblem(
    input: &super::M1TrustRegionSubproblemInput,
) -> Result<super::M1TrustRegionSubproblemObservation, super::M1TrustRegionSubproblemRefusal> {
    const N: usize = 21;
    let prepared = prepare_subproblem(input)?;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    {
        if FACE_PIVOT_CAPTURE_ENABLED
            .try_with(Cell::get)
            .unwrap_or(false)
        {
            let _ = FACE_PIVOT_CAPTURE.try_with(|cell| {
                if let Ok(mut slot) = cell.try_borrow_mut() {
                    let accepted_installs = slot
                        .as_ref()
                        .map_or_else(Vec::new, |prior| prior.accepted_installs.clone());
                    let post_update_admissions = slot
                        .as_ref()
                        .map_or_else(Vec::new, |prior| prior.post_update_admissions.clone());
                    let lifecycle = slot
                        .as_ref()
                        .map_or_else(Vec::new, |prior| prior.lifecycle.clone());
                    let completed_subproblems = slot
                        .as_ref()
                        .map_or_else(Vec::new, |prior| prior.completed_subproblems.clone());
                    let operations = slot
                        .as_ref()
                        .map_or_else(Vec::new, |prior| prior.operations.clone());
                    *slot = Some(M1FacePivotCapture {
                        base: input.base,
                        raw_residual: input.raw_residual,
                        raw_jacobian: input.raw_jacobian,
                        normalizers: input.normalizers,
                        scales: input.scales,
                        lower: input.lower,
                        upper: input.upper,
                        weighted_residual: prepared.weighted_residual.clone(),
                        weighted_matrix: prepared.weighted_matrix.clone(),
                        initial_radius: input.initial_radius,
                        completed_subproblems,
                        operations,
                        accepted_installs,
                        post_update_admissions,
                        lifecycle,
                        refusal: None,
                    });
                }
            });
        }
    }
    let mut state = SubproblemState::new(input);
    loop {
        let mut solution = solve_face(input, &prepared, &mut state)?;
        refine_face_before_crossing(input, &prepared, &mut state, &mut solution)?;
        if activate_first_crossing(&prepared, &mut state, &solution)? {
            continue;
        }
        let transition = kkt_transition(&prepared, &mut state, &solution)?;
        let Some(values) = transition.values else {
            continue;
        };
        let candidate = candidate_coordinates(input, &state.p).ok_or_else(|| {
            state_refusal(
                M1TrustRegionRefusalKind::SvdNonFinite,
                &state,
                prepared.raw_max,
            )
        })?;
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        capture_successful_subproblem(&state);
        return Ok(super::M1TrustRegionSubproblemObservation {
            optimality_observation: Some(transition.observation),
            initial_face_step: [0.0; N],
            base_coordinates: input.base,
            candidate_coordinates: candidate,
            initial_lower_mask: [false; N],
            initial_upper_mask: [false; N],
            initial_free_mask: state.initial_free,
            scaled_step: state.p,
            final_residual: values.residual,
            final_g: values.gradient,
            final_lambda_times_p: values.lambda_times_step,
            final_h: values.h,
            lambda: solution.trace.lambda,
            work: state_work_observation(&state),
            event_trace: state.events,
            entered_svd_factorizations: state.entered,
            completed_face_pivots: state.completed_pivots,
            raw_jacobian_columns: raw_jacobian_columns(input),
            active_lower: state.lower_active,
            active_upper: state.upper_active,
            refinement_scalar_operations: state.refinement_work.scalar_operations,
            refinement_guard_events: state.refinement_work.guard_events,
            refinement_attempted_entries: state.refinement_work.attempted_entries,
            refinement_denied_entries: state.refinement_work.denied_entries,
        });
    }
}

pub(crate) fn m1_trust_region_solve_subproblem_for_test(
    _input: &super::M1CoupledColumnInput,
    _trial: &super::M1CoupledColumnTrial,
    input: &super::M1TrustRegionSubproblemInput,
) -> Result<super::M1TrustRegionSubproblemObservation, super::M1TrustRegionSubproblemRefusal> {
    solve_subproblem(input)
}

#[cfg(test)]
pub(crate) fn m1_trust_region_kkt_transition_for_test(
    weighted_matrix: Vec<Vec<f64>>,
    weighted_residual: [f64; 21],
    step: [f64; 21],
    lower_active: [bool; 21],
    upper_active: [bool; 21],
    free: [bool; 21],
    lambda: f64,
) -> Result<super::M1TrustRegionKktTransitionObservation, super::M1TrustRegionSubproblemRefusal> {
    assert!(
        (0..21).all(|index| {
            usize::from(lower_active[index])
                + usize::from(upper_active[index])
                + usize::from(free[index])
                == 1
        }),
        "test seam requires a complete active/free partition"
    );
    let mut scaled_lower = [f64::NEG_INFINITY; 21];
    let mut scaled_upper = [f64::INFINITY; 21];
    for index in 0..21 {
        if lower_active[index] {
            scaled_lower[index] = step[index];
        }
        if upper_active[index] {
            scaled_upper[index] = step[index];
        }
    }
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    seed_kkt_transition_capture_for_test(
        &weighted_matrix,
        &weighted_residual,
        scaled_lower,
        scaled_upper,
    );
    let prepared = PreparedSubproblem {
        raw_max: weighted_residual
            .iter()
            .fold(0.0_f64, |maximum, value| maximum.max(value.abs())),
        weighted_residual: weighted_residual.to_vec(),
        weighted_matrix,
        scaled_lower,
        scaled_upper,
    };
    let mut state = SubproblemState {
        p: step,
        lower_active,
        upper_active,
        free,
        initial_free: [true; 21],
        events: Vec::new(),
        entered: 1,
        completed_pivots: 0,
        completed_sweeps: 0,
        total_jacobi_sweeps: 0,
        refinement_work: RefinementWorkLedger::default(),
    };
    let solution = FaceSolution {
        free_ids: (0..21).filter(|&index| free[index]).collect(),
        trace: LambdaTrace {
            lambda,
            ..LambdaTrace::default()
        },
        refined: false,
        factor: None,
        radius: 1.0,
    };
    let transition = kkt_transition(&prepared, &mut state, &solution)?;
    Ok(super::M1TrustRegionKktTransitionObservation {
        action: state.events.last().map(|event| event.action),
        selected_coordinate: state.events.last().map(|event| event.selected_coordinate),
        optimality: Some(transition.observation),
    })
}

#[cfg(test)]
fn m1_trust_region_signed_zero_crossing_then_kkt_for_test(
    weighted_matrix: Vec<Vec<f64>>,
    weighted_residual: [f64; 21],
    crossing_step: [f64; 21],
) -> Result<(f64, Vec<super::M1TrustRegionKktAction>), super::M1TrustRegionSubproblemRefusal> {
    let mut lower = [f64::NEG_INFINITY; 21];
    let upper = [f64::INFINITY; 21];
    lower[0] = 0.0;
    let prepared = PreparedSubproblem {
        raw_max: weighted_residual
            .iter()
            .fold(0.0_f64, |maximum, value| maximum.max(value.abs())),
        weighted_residual: weighted_residual.to_vec(),
        weighted_matrix,
        scaled_lower: lower,
        scaled_upper: upper,
    };
    let mut state = SubproblemState {
        p: [0.0; 21],
        lower_active: [false; 21],
        upper_active: [false; 21],
        free: [true; 21],
        initial_free: [true; 21],
        events: Vec::new(),
        entered: 1,
        completed_pivots: 0,
        completed_sweeps: 0,
        total_jacobi_sweeps: 0,
        refinement_work: RefinementWorkLedger::default(),
    };
    let solution = FaceSolution {
        free_ids: (0..21).collect(),
        trace: LambdaTrace {
            lambda: 0.0,
            step: crossing_step.to_vec(),
            ..LambdaTrace::default()
        },
        refined: false,
        factor: None,
        radius: 1.0,
    };
    let theta = first_crossing(
        &crossing_step,
        &state.p,
        &solution.free_ids,
        &prepared.scaled_lower,
        &prepared.scaled_upper,
    )
    .map_err(|kind| state_refusal(kind, &state, prepared.raw_max))?
    .expect("negative candidate crosses lower bound")
    .0;
    assert!(activate_first_crossing(&prepared, &mut state, &solution)?);
    let _ = kkt_transition(&prepared, &mut state, &solution)?;
    Ok((
        theta,
        state.events.iter().map(|event| event.action).collect(),
    ))
}

#[cfg(test)]
mod finite_precision_controls {
    use super::{
        box_feasible_kkt_values, m1_trust_region_kkt_transition_for_test,
        m1_trust_region_signed_zero_crossing_then_kkt_for_test, upward_mul,
    };
    use crate::{
        M1TrustRegionKktAction, M1TrustRegionRefusalKind, M1TrustRegionSubproblemInput,
        M1TrustRegionTestWorkState,
    };
    use serde_json::Value;

    const RETAINED_CAPTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/face-pivot-observed-capture.json"
    ));
    const RETAINED_ORACLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/finite-precision-correctness-oracle-retained-face.json"
    ));

    fn bit(value: &Value) -> f64 {
        f64::from_bits(
            u64::from_str_radix(value.as_str().expect("binary64 hex"), 16)
                .expect("valid binary64 bits"),
        )
    }

    fn mask(values: &Value) -> Vec<bool> {
        values
            .as_array()
            .expect("mask array")
            .iter()
            .map(|value| value.as_bool().expect("mask bit"))
            .collect()
    }

    fn vector(value: &Value) -> [f64; 21] {
        std::array::from_fn(|index| bit(&value[index]))
    }

    fn bits(values: [f64; 21]) -> [u64; 21] {
        values.map(f64::to_bits)
    }

    fn partition(lower: [bool; 21], upper: [bool; 21]) -> [bool; 21] {
        std::array::from_fn(|index| !lower[index] && !upper[index])
    }

    fn capture_return_input() -> M1TrustRegionSubproblemInput {
        let mut jacobian = [[0.0; 21]; 21];
        for (coordinate, row) in jacobian.iter_mut().enumerate() {
            row[coordinate] = 1.0;
        }
        M1TrustRegionSubproblemInput {
            raw_residual: [0.0; 21],
            raw_jacobian: jacobian,
            normalizers: [1.0; 21],
            scales: [1.0; 21],
            lower: [f64::NEG_INFINITY; 21],
            upper: [f64::INFINITY; 21],
            initial_radius: 1.0,
            base: [0.0; 21],
            test_work: M1TrustRegionTestWorkState::default(),
        }
    }

    fn capture_nonzero_return_input() -> M1TrustRegionSubproblemInput {
        let mut input = capture_return_input();
        input.raw_residual[0] = 0.25;
        input
    }

    #[test]
    fn retained_capture_uses_shared_ordered_kkt_operation_before_policy_change() {
        let capture: Value = serde_json::from_str(RETAINED_CAPTURE).expect("retained capture");
        let oracle: Value =
            serde_json::from_str(RETAINED_ORACLE).expect("independent retained oracle");
        let release = capture["operations"]
            .as_array()
            .expect("operation array")
            .iter()
            .find(|operation| operation["action"] == "release")
            .expect("recorded lower release");
        let matrix = capture["weighted_matrix"]
            .as_array()
            .expect("weighted matrix")
            .iter()
            .map(|row| vector(row).to_vec())
            .collect::<Vec<_>>();
        let values = box_feasible_kkt_values(
            &matrix,
            &vector(&capture["weighted_residual"]),
            &vector(&release["p_before"]),
            bit(&release["lambda"]),
        )
        .expect("shared KKT operation on retained finite operands");
        let recorded_h = vector(&release["h"]);
        assert_eq!(values.h.map(f64::to_bits), recorded_h.map(f64::to_bits));
        let lower = std::array::from_fn(|index| {
            release["lower_before"][index]
                .as_bool()
                .expect("lower mask")
        });
        let upper = std::array::from_fn(|index| {
            release["upper_before"][index]
                .as_bool()
                .expect("upper mask")
        });
        let transition = m1_trust_region_kkt_transition_for_test(
            matrix,
            vector(&capture["weighted_residual"]),
            vector(&release["p_before"]),
            lower,
            upper,
            partition(lower, upper),
            bit(&release["lambda"]),
        )
        .expect("shared retained transition");
        assert_eq!(
            transition.action,
            Some(M1TrustRegionKktAction::ReleaseLower)
        );
        let optimality = transition
            .optimality
            .expect("expected-red: 02 retained all-coordinate observation");
        assert_eq!(optimality.coordinates.len(), 21);
        for (coordinate, actual) in optimality.coordinates.iter().enumerate() {
            assert_eq!(actual.coordinate, coordinate);
        }
        let oracle_coordinates = oracle["coordinates"]
            .as_array()
            .expect("independent oracle coordinates");
        assert_eq!(oracle_coordinates.len(), 21);
        for coordinate in [5usize, 11usize] {
            let expected = &oracle_coordinates[coordinate];
            let actual = &optimality.coordinates[coordinate];
            assert_eq!(
                actual.h.to_bits(),
                bit(&expected["h_hat"]["bits"]).to_bits()
            );
            assert_eq!(
                actual.cbar.to_bits(),
                bit(&expected["cbar"]["bits"]).to_bits()
            );
            assert_eq!(
                actual.tau.to_bits(),
                bit(&expected["tau"]["bits"]).to_bits()
            );
            assert_eq!(
                actual.class,
                if coordinate == 5 {
                    crate::M1TrustRegionOptimalityClass::ReleaseLower
                } else {
                    crate::M1TrustRegionOptimalityClass::FreeRefuse
                }
            );
        }
    }

    #[test]
    fn expected_red_cancellation_band_must_not_release_on_sign_alone() {
        // A real shared KKT calculation gives a negative lower multiplier
        // through cancellation.  The reviewed 02 envelope retains it; this
        // assertion is intentionally red until the shared transition applies
        // that policy, rather than the current sign-only helper.
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = 1.0;
        matrix[0][1] = 1.0;
        let mut residual = [0.0; 21];
        residual[0] = -1.0 - f64::EPSILON;
        let mut step = [0.0; 21];
        step[1] = 1.0;
        let lower = std::array::from_fn(|index| index == 0);
        let upper = [false; 21];
        assert_eq!(
            m1_trust_region_kkt_transition_for_test(
                matrix,
                residual,
                step,
                lower,
                upper,
                partition(lower, upper),
                0.0
            )
            .expect("shared KKT transition")
            .action,
            Some(M1TrustRegionKktAction::Return),
            "expected-red: reviewed 02 uncertainty-band decision replaces sign-only release"
        );
    }

    #[test]
    fn expected_red_upper_cancellation_band_must_not_release_on_sign_alone() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = 1.0;
        matrix[0][1] = 1.0;
        let mut residual = [0.0; 21];
        residual[0] = 1.0 + f64::EPSILON;
        let mut step = [0.0; 21];
        step[1] = -1.0;
        let upper = std::array::from_fn(|index| index == 0);
        assert_eq!(
            m1_trust_region_kkt_transition_for_test(
                matrix.clone(),
                residual,
                step,
                [false; 21],
                upper,
                partition([false; 21], upper),
                0.0
            )
            .expect("shared upper transition")
            .action,
            Some(M1TrustRegionKktAction::Return),
            "expected-red: uncertainty-band upper multiplier is retained"
        );
    }

    #[test]
    fn resolvable_upper_violation_releases_through_shared_transition() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = 1.0;
        let mut residual = [0.0; 21];
        residual[0] = 1.0;
        let upper = std::array::from_fn(|index| index == 0);
        assert_eq!(
            m1_trust_region_kkt_transition_for_test(
                matrix,
                residual,
                [0.0; 21],
                [false; 21],
                upper,
                partition([false; 21], upper),
                0.0,
            )
            .expect("shared resolvable upper transition")
            .action,
            Some(M1TrustRegionKktAction::ReleaseUpper)
        );
    }

    #[test]
    fn expected_red_release_ranking_uses_observed_ratio_not_multiplier_magnitude() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = 1.0;
        matrix[1][1] = 1.0;
        matrix[2][0] = 100.0;
        matrix[2][2] = 1.0;
        for (coordinate, row) in matrix.iter_mut().enumerate().skip(3) {
            row[coordinate] = 1.0;
        }
        let mut residual = [0.0; 21];
        residual[0] = -2.0;
        residual[1] = -1.0;
        residual[2] = -1.0;
        let mut step = [0.0; 21];
        step[2] = 1.0;
        let lower = std::array::from_fn(|coordinate| coordinate < 2);
        let transition = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            step,
            lower,
            [false; 21],
            partition(lower, [false; 21]),
            0.0,
        )
        .expect("shared release-ranking transition");
        assert_eq!(
            transition.action,
            Some(M1TrustRegionKktAction::ReleaseLower)
        );
        assert_eq!(
            transition.selected_coordinate,
            Some(1),
            "expected-red: larger absolute multiplier at coordinate 0 has the smaller observed ratio"
        );
        let observation = transition
            .optimality
            .expect("expected-red: release-ranking optimality observation");
        let coordinate_zero = &observation.coordinates[0];
        let coordinate_one = &observation.coordinates[1];
        assert!(coordinate_zero.h.abs() > coordinate_one.h.abs());
        assert!(coordinate_zero.tau.is_normal());
        assert!(coordinate_one.tau.is_normal());
        assert!(
            coordinate_one.h.abs() / coordinate_one.tau
                > coordinate_zero.h.abs() / coordinate_zero.tau,
            "expected-red: shared observed thresholds order release ratios"
        );
    }

    #[test]
    fn expected_red_free_stationarity_must_refuse_from_shared_transition() {
        let mut free_matrix = vec![vec![0.0; 21]; 21];
        free_matrix[0][0] = 1.0;
        let mut free_residual = [0.0; 21];
        free_residual[0] = 1.0;
        let free = [true; 21];
        let refusal = m1_trust_region_kkt_transition_for_test(
            free_matrix,
            free_residual,
            [0.0; 21],
            [false; 21],
            [false; 21],
            free,
            0.0,
        )
        .expect_err("expected-red: materially nonstationary free coordinate must refuse");
        assert_eq!(
            refusal.kind,
            M1TrustRegionRefusalKind::TrustRegionOptimalityIndeterminate
        );
        assert_eq!(
            refusal.optimality_reason,
            Some("optimality_free_stationarity")
        );
        assert_eq!(refusal.optimality_coordinate, Some(0));
    }

    #[test]
    fn shared_signed_zero_crossing_precedes_genuine_descent_release() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = 1.0;
        let mut residual = [0.0; 21];
        residual[0] = -1.0;
        let mut step = [0.0; 21];
        step[0] = -f64::MIN_POSITIVE;
        let (theta, actions) =
            m1_trust_region_signed_zero_crossing_then_kkt_for_test(matrix, residual, step)
                .expect("shared signed-zero crossing and descent");
        assert_eq!(theta.to_bits(), (-0.0_f64).to_bits());
        assert_eq!(
            actions,
            vec![
                M1TrustRegionKktAction::ActivateLower,
                M1TrustRegionKktAction::ReleaseLower,
            ]
        );
    }

    #[test]
    fn shared_normal_nonfinite_and_overflow_kkt_operands_are_real_operations() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = 1.0;
        let mut residual = [0.0; 21];
        residual[0] = 1.0;
        assert!(box_feasible_kkt_values(&matrix, &residual, &[0.0; 21], 0.0).is_ok());
        matrix[0][0] = f64::NAN;
        assert!(box_feasible_kkt_values(&matrix, &residual, &[0.0; 21], 0.0).is_err());
        matrix[0][0] = f64::MAX;
        residual[0] = f64::MAX;
        assert!(box_feasible_kkt_values(&matrix, &residual, &[0.0; 21], 0.0).is_err());
    }

    #[test]
    fn expected_red_nonfinite_kkt_operand_has_typed_optimality_refusal() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = f64::NAN;
        let mut residual = [0.0; 21];
        residual[0] = 1.0;
        let refusal = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            [0.0; 21],
            [false; 21],
            [false; 21],
            [true; 21],
            0.0,
        )
        .expect_err("expected-red: nonfinite KKT operand must have 02 refusal metadata");
        assert_eq!(
            refusal.kind,
            M1TrustRegionRefusalKind::TrustRegionOptimalityIndeterminate
        );
        assert_eq!(refusal.optimality_reason, Some("optimality_nonfinite"));
        assert_eq!(refusal.optimality_coordinate, Some(0));
    }

    #[test]
    fn expected_red_overflow_kkt_operand_has_typed_optimality_refusal() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = f64::MAX;
        let mut residual = [0.0; 21];
        residual[0] = f64::MAX;
        let refusal = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            [0.0; 21],
            [false; 21],
            [false; 21],
            [true; 21],
            0.0,
        )
        .expect_err("expected-red: overflowing KKT product must have 02 refusal metadata");
        assert_eq!(
            refusal.kind,
            M1TrustRegionRefusalKind::TrustRegionOptimalityIndeterminate
        );
        assert_eq!(refusal.optimality_reason, Some("optimality_nonfinite"));
        assert_eq!(refusal.optimality_coordinate, Some(0));
    }

    #[test]
    fn expected_red_negative_lambda_has_typed_optimality_refusal() {
        let refusal = m1_trust_region_kkt_transition_for_test(
            vec![vec![0.0; 21]; 21],
            [0.0; 21],
            [0.0; 21],
            [false; 21],
            [false; 21],
            [true; 21],
            -1.0,
        )
        .expect_err("expected-red: negative lambda must have 02 refusal metadata");
        assert_eq!(
            refusal.kind,
            M1TrustRegionRefusalKind::TrustRegionOptimalityIndeterminate
        );
        assert_eq!(
            refusal.optimality_reason,
            Some("optimality_negative_lambda")
        );
        assert_eq!(refusal.optimality_coordinate, Some(0));
    }

    #[test]
    fn expected_red_subnormal_kkt_operand_must_refuse() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = f64::from_bits(1);
        let mut residual = [0.0; 21];
        residual[0] = 1.0;
        let refusal = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            [0.0; 21],
            [false; 21],
            [false; 21],
            [true; 21],
            0.0,
        )
        .expect_err("expected-red: nonzero subnormal KKT operand must refuse");
        assert_eq!(
            refusal.kind,
            M1TrustRegionRefusalKind::TrustRegionOptimalityIndeterminate
        );
        assert_eq!(
            refusal.optimality_reason,
            Some("optimality_non_normal_operand")
        );
        assert_eq!(refusal.optimality_coordinate, Some(0));
    }

    #[test]
    fn expected_red_underflow_to_zero_kkt_product_must_refuse() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = f64::MIN_POSITIVE;
        let mut residual = [0.0; 21];
        residual[0] = f64::MIN_POSITIVE;
        let refusal = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            [0.0; 21],
            [false; 21],
            [false; 21],
            [true; 21],
            0.0,
        )
        .expect_err(
            "expected-red: nonzero normal operands whose KKT product underflows must refuse",
        );
        assert_eq!(
            refusal.kind,
            M1TrustRegionRefusalKind::TrustRegionOptimalityIndeterminate
        );
        assert_eq!(
            refusal.optimality_reason,
            Some("optimality_underflow_to_zero")
        );
        assert_eq!(refusal.optimality_coordinate, Some(0));
    }

    #[test]
    fn expected_red_subnormal_kkt_intermediate_must_refuse() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = f64::MIN_POSITIVE;
        let mut residual = [0.0; 21];
        residual[0] = 0.5;
        let refusal = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            [0.0; 21],
            [false; 21],
            [false; 21],
            [true; 21],
            0.0,
        )
        .expect_err("expected-red: nonzero subnormal KKT intermediate must refuse");
        assert_eq!(
            refusal.kind,
            M1TrustRegionRefusalKind::TrustRegionOptimalityIndeterminate
        );
        assert_eq!(
            refusal.optimality_reason,
            Some("optimality_subnormal_intermediate")
        );
        assert_eq!(refusal.optimality_coordinate, Some(0));
    }

    #[test]
    fn expected_red_zero_cbar_with_zero_h_is_an_explicit_02_observation() {
        let transition = m1_trust_region_kkt_transition_for_test(
            vec![vec![0.0; 21]; 21],
            [0.0; 21],
            [0.0; 21],
            [false; 21],
            [false; 21],
            [true; 21],
            0.0,
        )
        .expect("shared zero KKT transition");
        let observation = transition
            .optimality
            .expect("expected-red: zero-Cbar 02 observation");
        let coordinate = observation
            .coordinates
            .iter()
            .find(|entry| entry.coordinate == 0)
            .expect("expected-red: zero-Cbar coordinate");
        assert_eq!(coordinate.h.to_bits(), 0.0f64.to_bits());
        assert_eq!(coordinate.cbar.to_bits(), 0.0f64.to_bits());
        assert_eq!(coordinate.tau.to_bits(), 0.0f64.to_bits());
        assert_eq!(
            coordinate.class,
            crate::M1TrustRegionOptimalityClass::FreePass
        );
    }

    #[test]
    fn expected_red_zero_identity_enclosure_records_no_scalar_arithmetic() {
        let transition = m1_trust_region_kkt_transition_for_test(
            vec![vec![0.0; 21]; 21],
            [0.0; 21],
            [0.0; 21],
            [false; 21],
            [false; 21],
            [true; 21],
            0.0,
        )
        .expect("shared zero transition");
        assert_eq!(
            transition
                .optimality
                .expect("02 zero observation")
                .scalar_operations,
            0,
            "expected-red: skipped zero identities perform neither scalar arithmetic nor upward steps"
        );
    }

    #[test]
    fn expected_red_nonzero_enclosure_records_exact_scalar_arithmetic() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = 1.0;
        let mut residual = [0.0; 21];
        residual[0] = 1.0;
        let lower = std::array::from_fn(|coordinate| coordinate == 0);
        let transition = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            [0.0; 21],
            lower,
            [false; 21],
            partition(lower, [false; 21]),
            0.0,
        )
        .expect("shared nonzero enclosure transition");
        let observation = transition.optimality.expect("02 nonzero observation");
        assert_eq!(observation.scalar_operations, 6);
        assert!(observation.scalar_operations <= 39_816);
    }

    #[test]
    fn expected_red_tau_subnormal_has_typed_optimality_refusal() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = 1.0;
        let mut residual = [0.0; 21];
        residual[0] = f64::MIN_POSITIVE;
        let lower = std::array::from_fn(|coordinate| coordinate == 0);
        let refusal = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            [0.0; 21],
            lower,
            [false; 21],
            partition(lower, [false; 21]),
            0.0,
        )
        .expect_err("expected-red: subnormal tau must refuse with its stable reason");
        assert_eq!(
            refusal.kind,
            M1TrustRegionRefusalKind::TrustRegionOptimalityIndeterminate
        );
        assert_eq!(
            refusal.optimality_reason,
            Some("optimality_subnormal_intermediate")
        );
        assert_eq!(refusal.optimality_coordinate, Some(0));
    }

    #[test]
    fn expected_red_upward_normal_operands_underflow_with_stable_reason() {
        let mut operations = 0;
        let failure = upward_mul(f64::MIN_POSITIVE, f64::MIN_POSITIVE, 0, &mut operations)
            .expect_err("expected-red: nonzero normal enclosure product underflows");
        assert_eq!(failure.reason, "optimality_underflow_to_zero");
        assert_eq!(failure.coordinate, 0);
        assert_eq!(operations, 1);
    }

    #[test]
    fn expected_red_cbar_overflow_has_typed_02_transition_metadata() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = f64::MAX;
        let mut residual = [0.0; 21];
        residual[0] = -f64::MAX;
        let mut step = [0.0; 21];
        step[0] = 1.0;
        let refusal = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            step,
            [false; 21],
            [false; 21],
            [true; 21],
            0.0,
        )
        .expect_err("expected-red: 02 upward-enclosure transition refusal");
        assert_eq!(
            refusal.kind,
            M1TrustRegionRefusalKind::TrustRegionOptimalityIndeterminate
        );
        assert_eq!(
            refusal.optimality_reason,
            Some("optimality_upward_enclosure")
        );
        assert_eq!(refusal.optimality_coordinate, Some(0));
    }

    #[test]
    fn fixed_finite_bound_uses_shared_prepare_and_full_solve_metadata() {
        let mut jacobian = [[0.0; 21]; 21];
        jacobian[0][1] = 1.0;
        let mut lower = [0.0; 21];
        let mut upper = [0.0; 21];
        lower[1] = f64::NEG_INFINITY;
        upper[1] = f64::INFINITY;
        let result = super::solve_subproblem(&M1TrustRegionSubproblemInput {
            raw_residual: [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            raw_jacobian: jacobian,
            normalizers: [1.0; 21],
            scales: [1.0; 21],
            lower,
            upper,
            initial_radius: 2.0,
            base: [0.0; 21],
            test_work: M1TrustRegionTestWorkState::default(),
        })
        .expect("shared finite fixed-bound solve");
        assert!(!result.initial_free_mask[0]);
        let observation = result
            .optimality_observation
            .expect("expected-red: fixed coordinate has an explicit 02 observation");
        let fixed = observation
            .coordinates
            .iter()
            .find(|coordinate| coordinate.coordinate == 0)
            .expect("expected-red: fixed coordinate observation");
        assert_eq!(fixed.class, crate::M1TrustRegionOptimalityClass::Fixed);
        assert!(result.initial_free_mask[1]);
        assert_eq!(result.scaled_step[0].to_bits(), 0.0f64.to_bits());
        assert!(result.candidate_coordinates[0].is_finite());
        assert!(result.scaled_step[1].abs() <= 2.0);
    }

    #[test]
    fn expected_red_physical_distinct_bounds_that_collapse_when_scaled_are_free() {
        let mut jacobian = [[0.0; 21]; 21];
        jacobian[0][0] = f64::MIN_POSITIVE;
        for (coordinate, row) in jacobian.iter_mut().enumerate().skip(1) {
            row[coordinate] = 1.0;
        }
        let mut lower = [f64::NEG_INFINITY; 21];
        let mut upper = [f64::INFINITY; 21];
        let mut base = [0.0; 21];
        let mut scales = [1.0; 21];
        lower[0] = 1.0;
        upper[0] = 1.0f64.next_up();
        base[0] = 1.0;
        scales[0] = f64::MAX;
        let result = super::solve_subproblem(&M1TrustRegionSubproblemInput {
            raw_residual: [0.0; 21],
            raw_jacobian: jacobian,
            normalizers: [1.0; 21],
            scales,
            lower,
            upper,
            initial_radius: 1.0,
            base,
            test_work: M1TrustRegionTestWorkState::default(),
        })
        .expect("shared distinct physical-bound solve");
        let observation = result
            .optimality_observation
            .expect("02 physical-bound observation");
        assert_eq!(
            observation.coordinates[0].class,
            crate::M1TrustRegionOptimalityClass::FreePass,
            "expected-red: only physically equal bounds are fixed"
        );
    }

    #[cfg(feature = "m1-trust-region-physical-stage")]
    #[test]
    fn expected_red_successful_return_capture_has_complete_02_operands() {
        let _scope = super::m1_face_pivot_capture_begin_for_test();
        let result =
            super::solve_subproblem(&capture_return_input()).expect("shared return capture solve");
        let capture = super::m1_face_pivot_capture_take_for_test()
            .expect("expected-red: successful return capture");
        let operation = capture
            .operations
            .iter()
            .find(|operation| operation.action == "kkt-return")
            .expect("expected-red: return operation");
        assert_eq!(bits(operation.p_before), bits(result.scaled_step));
        assert_eq!(operation.lower_before, result.active_lower);
        assert_eq!(operation.upper_before, result.active_upper);
        assert_eq!(operation.free_before, [true; 21]);
        assert_eq!(operation.radius.to_bits(), 1.0f64.to_bits());
        assert_eq!(operation.lambda, Some(result.lambda));
        assert_eq!(operation.free_ids, (0..21).collect::<Vec<_>>());
        assert_eq!(operation.residual, Some(result.final_residual));
        assert_eq!(operation.g, Some(result.final_g));
        assert_eq!(operation.lambda_times_p, Some(result.final_lambda_times_p));
        assert_eq!(operation.h, Some(result.final_h));
        let optimality = operation
            .optimality
            .as_ref()
            .expect("expected-red: return optimality observation");
        assert_eq!(optimality.coordinates.len(), 21);
        for (coordinate, observation) in optimality.coordinates.iter().enumerate() {
            assert_eq!(observation.coordinate, coordinate);
            assert_eq!(
                observation.class,
                crate::M1TrustRegionOptimalityClass::FreePass
            );
        }
        assert_eq!(optimality.scalar_operations, 0);
        assert_eq!(
            operation.entered_factorizations,
            result.entered_svd_factorizations
        );
        assert_eq!(operation.completed_pivots, result.completed_face_pivots);
    }

    #[cfg(feature = "m1-trust-region-physical-stage")]
    #[test]
    fn expected_red_capture_retains_ordered_return_records_across_real_subproblems() {
        let _scope = super::m1_face_pivot_capture_begin_for_test();
        let first =
            super::solve_subproblem(&capture_return_input()).expect("first shared return solve");
        let second = super::solve_subproblem(&capture_nonzero_return_input())
            .expect("second shared return solve");
        let capture = super::m1_face_pivot_capture_take_for_test()
            .expect("expected-red: combined return capture");
        let returns = capture
            .operations
            .iter()
            .filter(|operation| operation.action == "kkt-return")
            .collect::<Vec<_>>();
        assert_eq!(
            returns.len(),
            2,
            "expected-red: both subproblem returns persist"
        );
        for (operation, result) in [(returns[0], &first), (returns[1], &second)] {
            assert_eq!(bits(operation.p_before), bits(result.scaled_step));
            assert_eq!(operation.lower_before, result.active_lower);
            assert_eq!(operation.upper_before, result.active_upper);
            assert_eq!(operation.free_before, [true; 21]);
            assert_eq!(operation.lambda, Some(result.lambda));
            assert_eq!(operation.residual, Some(result.final_residual));
            assert_eq!(operation.g, Some(result.final_g));
            assert_eq!(operation.lambda_times_p, Some(result.final_lambda_times_p));
            assert_eq!(operation.h, Some(result.final_h));
            let recorded = operation
                .optimality
                .as_ref()
                .expect("expected-red: ordered return assessment");
            let observed = result
                .optimality_observation
                .as_ref()
                .expect("02 returned assessment");
            assert_eq!(recorded.coordinates.len(), 21);
            assert_eq!(recorded.scalar_operations, observed.scalar_operations);
            assert_eq!(
                operation.entered_factorizations,
                result.entered_svd_factorizations
            );
            assert_eq!(operation.completed_pivots, result.completed_face_pivots);
            assert_eq!(operation.completed_sweeps, 1);
        }
        assert_eq!(bits(first.final_residual), [0_u64; 21]);
        assert_ne!(bits(second.scaled_step), [0_u64; 21]);
        assert_eq!(capture.completed_subproblems.len(), 2);
        for (summary, result) in capture.completed_subproblems.iter().zip([&first, &second]) {
            assert!(!summary.refused);
            assert_eq!(summary.completed_sweeps, 1);
            assert_eq!(summary.completed_sweeps, result.entered_svd_factorizations);
            assert_eq!(
                summary.entered_factorizations,
                result.entered_svd_factorizations
            );
            assert_eq!(summary.completed_pivots, result.completed_face_pivots);
        }
    }

    #[cfg(feature = "m1-trust-region-physical-stage")]
    #[test]
    fn expected_red_nonfinite_transition_capture_records_typed_refusal_without_mutation() {
        let _scope = super::m1_face_pivot_capture_begin_for_test();
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = f64::NAN;
        let mut residual = [0.0; 21];
        residual[0] = 1.0;
        let refusal = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            [0.0; 21],
            [false; 21],
            [false; 21],
            [true; 21],
            0.0,
        )
        .expect_err("shared nonfinite transition refusal");
        assert_eq!(refusal.optimality_reason, Some("optimality_nonfinite"));
        assert_eq!(
            refusal.optimality_radius.map(|value| value.to_bits()),
            Some(1.0f64.to_bits())
        );
        assert_eq!(
            refusal.optimality_lambda.map(|value| value.to_bits()),
            Some(0.0f64.to_bits())
        );
        let capture = super::m1_face_pivot_capture_take_for_test()
            .expect("expected-red: nonfinite transition capture");
        let recorded = capture
            .refusal
            .expect("expected-red: typed capture refusal");
        assert_eq!(recorded.kind, refusal.kind);
        assert_eq!(recorded.optimality_reason, refusal.optimality_reason);
        assert_eq!(
            recorded.optimality_coordinate,
            refusal.optimality_coordinate
        );
        assert_eq!(bits(recorded.p), [0_u64; 21]);
        assert_eq!(recorded.lower, [false; 21]);
        assert_eq!(recorded.upper, [false; 21]);
        assert_eq!(recorded.free, [true; 21]);
        assert_eq!(recorded.entered_factorizations, 1);
        assert_eq!(recorded.completed_pivots, 0);
        assert_eq!(recorded.completed_sweeps, 0);
        assert!(recorded.optimality.is_none());
        assert!(capture.operations.is_empty());
    }

    #[cfg(feature = "m1-trust-region-physical-stage")]
    #[test]
    fn expected_red_enclosure_transition_capture_records_typed_refusal_without_fake_arrays() {
        let _scope = super::m1_face_pivot_capture_begin_for_test();
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = f64::MAX;
        let mut residual = [0.0; 21];
        residual[0] = -f64::MAX;
        let mut step = [0.0; 21];
        step[0] = 1.0;
        let refusal = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            step,
            [false; 21],
            [false; 21],
            [true; 21],
            0.0,
        )
        .expect_err("shared enclosure transition refusal");
        assert_eq!(
            refusal.optimality_reason,
            Some("optimality_upward_enclosure")
        );
        assert_eq!(
            refusal.optimality_radius.map(|value| value.to_bits()),
            Some(1.0f64.to_bits())
        );
        assert_eq!(
            refusal.optimality_lambda.map(|value| value.to_bits()),
            Some(0.0f64.to_bits())
        );
        let capture = super::m1_face_pivot_capture_take_for_test()
            .expect("expected-red: enclosure transition capture");
        let recorded = capture
            .refusal
            .expect("expected-red: enclosure typed capture refusal");
        assert_eq!(recorded.kind, refusal.kind);
        assert_eq!(recorded.optimality_reason, refusal.optimality_reason);
        assert_eq!(
            recorded.optimality_coordinate,
            refusal.optimality_coordinate
        );
        assert!(recorded.optimality.is_none());
        let operation = capture
            .operations
            .iter()
            .find(|operation| operation.action == "optimality-refusal")
            .expect("expected-red: enclosure refusal operation");
        assert_eq!(bits(operation.p_before), bits(step));
        assert_eq!(operation.lower_before, [false; 21]);
        assert_eq!(operation.upper_before, [false; 21]);
        assert_eq!(operation.free_before, [true; 21]);
        assert_eq!(operation.radius.to_bits(), 1.0f64.to_bits());
        assert_eq!(operation.lambda, Some(0.0));
        assert_eq!(operation.free_ids, (0..21).collect::<Vec<_>>());
        assert_eq!(operation.residual, Some([0.0; 21]));
        assert_eq!(operation.g, Some([0.0; 21]));
        assert_eq!(operation.lambda_times_p, Some([0.0; 21]));
        assert_eq!(operation.h, Some([0.0; 21]));
    }

    #[cfg(feature = "m1-trust-region-physical-stage")]
    #[test]
    fn expected_red_free_stationarity_capture_retains_real_face_metadata() {
        let _scope = super::m1_face_pivot_capture_begin_for_test();
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = 1.0;
        let mut residual = [0.0; 21];
        residual[0] = 1.0;
        let refusal = m1_trust_region_kkt_transition_for_test(
            matrix,
            residual,
            [0.0; 21],
            [false; 21],
            [false; 21],
            [true; 21],
            0.0,
        )
        .expect_err("shared free-stationarity refusal");
        assert_eq!(
            refusal.optimality_reason,
            Some("optimality_free_stationarity")
        );
        assert_eq!(
            refusal.optimality_radius.map(|value| value.to_bits()),
            Some(1.0f64.to_bits())
        );
        assert_eq!(
            refusal.optimality_lambda.map(|value| value.to_bits()),
            Some(0.0f64.to_bits())
        );
        let capture =
            super::m1_face_pivot_capture_take_for_test().expect("shared free-stationarity capture");
        let recorded = capture
            .refusal
            .expect("shared free-stationarity refusal record");
        assert_eq!(recorded.optimality_reason, refusal.optimality_reason);
        assert_eq!(
            recorded.optimality_coordinate,
            refusal.optimality_coordinate
        );
        assert_eq!(bits(recorded.p), [0_u64; 21]);
        assert_eq!(recorded.lower, [false; 21]);
        assert_eq!(recorded.upper, [false; 21]);
        assert_eq!(recorded.free, [true; 21]);
        assert_eq!(recorded.entered_factorizations, 1);
        assert_eq!(recorded.completed_pivots, 0);
        assert_eq!(recorded.completed_sweeps, 0);
        let operation = capture
            .operations
            .iter()
            .find(|operation| operation.action == "optimality-refusal")
            .expect("shared free-stationarity operation");
        assert_eq!(operation.radius.to_bits(), 1.0f64.to_bits());
        assert_eq!(operation.lambda, Some(0.0));
        assert_eq!(operation.free_ids, (0..21).collect::<Vec<_>>());
        assert_eq!(
            operation.residual,
            Some([
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0
            ])
        );
        assert_eq!(
            operation.g,
            Some([
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0
            ])
        );
        assert_eq!(operation.lambda_times_p, Some([0.0; 21]));
        assert_eq!(
            operation.h,
            Some([
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0
            ])
        );
    }

    #[test]
    fn zero_scale_is_rejected_by_shared_preparation_before_kkt_transition() {
        let mut jacobian = [[0.0; 21]; 21];
        jacobian[0][0] = 1.0;
        let mut lower = [0.0; 21];
        let mut upper = [0.0; 21];
        lower[0] = f64::NEG_INFINITY;
        upper[0] = f64::INFINITY;
        let mut scales = [1.0; 21];
        scales[0] = 0.0;
        let refusal = super::solve_subproblem(&M1TrustRegionSubproblemInput {
            raw_residual: [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            raw_jacobian: jacobian,
            normalizers: [1.0; 21],
            scales,
            lower,
            upper,
            initial_radius: 1.0,
            base: [0.0; 21],
            test_work: M1TrustRegionTestWorkState::default(),
        })
        .expect_err("shared zero-scale preparation refusal");
        assert_eq!(refusal.kind, M1TrustRegionRefusalKind::SvdNonFinite);
        assert_eq!(refusal.optimality_reason, None);
        assert_eq!(refusal.optimality_coordinate, None);
        assert_eq!(refusal.entered_svd_factorizations, 0);
    }

    fn cancellation_band_transition(scale: f64, active_coordinate: usize, step_coordinate: usize) {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][active_coordinate] = scale;
        matrix[0][step_coordinate] = scale;
        let mut residual = [0.0; 21];
        residual[0] = (-1.0 - f64::EPSILON) * scale;
        let mut step = [0.0; 21];
        step[step_coordinate] = 1.0;
        let lower = std::array::from_fn(|index| index == active_coordinate);
        assert_eq!(
            m1_trust_region_kkt_transition_for_test(
                matrix,
                residual,
                step,
                lower,
                [false; 21],
                partition(lower, [false; 21]),
                0.0
            )
            .expect("shared permuted transition")
            .action,
            Some(M1TrustRegionKktAction::Return),
            "expected-red: cancellation band remains unresolved"
        );
    }

    #[test]
    fn expected_red_cancellation_band_is_stable_at_unit_scale() {
        cancellation_band_transition(1.0, 0, 1);
    }

    #[test]
    fn expected_red_cancellation_band_is_stable_at_diagonal_scale() {
        cancellation_band_transition(8.0, 0, 1);
    }

    #[test]
    fn expected_red_cancellation_band_is_stable_under_coordinate_permutation() {
        cancellation_band_transition(1.0, 1, 0);
    }
}

// COLD-CANOPY-M1-TR-SVD-BVLS-03 C1 is a test-scoped, non-FMA ORO primitive.
// It has no controller, factor, physical, or materialization connection.
#[cfg(test)]
const REFINEMENT_DOT2_SPLITTER: f64 = 134_217_729.0;

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RefinementDot2Reason {
    NonfiniteOperand,
    SplitterOverflow,
    UnderflowToZero,
    SubnormalIntermediate,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RefinementDot2Refusal {
    reason: RefinementDot2Reason,
    arithmetic: u16,
    guards: u16,
    exceptional_guards: u16,
    signed_zero_values: u16,
}

#[cfg(test)]
impl RefinementDot2Refusal {
    fn reason(self) -> &'static str {
        match self.reason {
            RefinementDot2Reason::NonfiniteOperand => "refinement_nonfinite_operand",
            RefinementDot2Reason::SplitterOverflow => "refinement_splitter_overflow",
            RefinementDot2Reason::UnderflowToZero => "refinement_underflow_to_zero",
            RefinementDot2Reason::SubnormalIntermediate => "refinement_subnormal_intermediate",
        }
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq)]
struct RefinementDot2Observation {
    value: f64,
    initial_product_bits: u64,
    arithmetic: u16,
    guards: u16,
    exceptional_guards: u16,
    signed_zero_values: u16,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default)]
struct RefinementDot2Counters {
    arithmetic: u16,
    guards: u16,
    exceptional_guards: u16,
    signed_zero_values: u16,
}

#[cfg(test)]
fn refinement_dot2_guard(
    value: f64,
    counters: &mut RefinementDot2Counters,
) -> Result<f64, RefinementDot2Reason> {
    counters.guards += 1;
    if value == 0.0 {
        counters.signed_zero_values += 1;
    }
    if !value.is_finite() {
        counters.exceptional_guards += 1;
        return Err(RefinementDot2Reason::NonfiniteOperand);
    }
    if value != 0.0 && value.abs() < f64::MIN_POSITIVE {
        counters.exceptional_guards += 1;
        return Err(RefinementDot2Reason::SubnormalIntermediate);
    }
    Ok(value)
}

#[cfg(test)]
fn refinement_dot2_mul(
    left: f64,
    right: f64,
    counters: &mut RefinementDot2Counters,
) -> Result<f64, RefinementDot2Reason> {
    let value = left * right;
    counters.arithmetic += 1;
    refinement_dot2_guard(value, counters)?;
    if left != 0.0 && right != 0.0 && value == 0.0 {
        counters.exceptional_guards += 1;
        return Err(RefinementDot2Reason::UnderflowToZero);
    }
    Ok(value)
}

#[cfg(test)]
fn refinement_dot2_add(
    left: f64,
    right: f64,
    counters: &mut RefinementDot2Counters,
) -> Result<f64, RefinementDot2Reason> {
    let value = left + right;
    counters.arithmetic += 1;
    refinement_dot2_guard(value, counters)
}

#[cfg(test)]
fn refinement_dot2_sub(
    left: f64,
    right: f64,
    counters: &mut RefinementDot2Counters,
) -> Result<f64, RefinementDot2Reason> {
    let value = left - right;
    counters.arithmetic += 1;
    refinement_dot2_guard(value, counters)
}

#[cfg(test)]
fn refinement_dot2_split(
    value: f64,
    counters: &mut RefinementDot2Counters,
) -> Result<(f64, f64), RefinementDot2Reason> {
    let c = match refinement_dot2_mul(REFINEMENT_DOT2_SPLITTER, value, counters) {
        Err(RefinementDot2Reason::NonfiniteOperand) => {
            // `value` was admitted at primitive entry and the splitter is finite,
            // so this exact failed multiplication is the splitter overflow.
            return Err(RefinementDot2Reason::SplitterOverflow);
        }
        Err(error) => return Err(error),
        Ok(value) => value,
    };
    let abig = refinement_dot2_sub(c, value, counters)?;
    let high = refinement_dot2_sub(c, abig, counters)?;
    let low = refinement_dot2_sub(value, high, counters)?;
    Ok((high, low))
}

#[cfg(test)]
fn refinement_dot2_two_product(
    left: f64,
    right: f64,
    counters: &mut RefinementDot2Counters,
) -> Result<(f64, f64), RefinementDot2Reason> {
    // Preserve the frozen primary-product-before-Split order.
    let product = refinement_dot2_mul(left, right, counters)?;
    let (left_high, left_low) = refinement_dot2_split(left, counters)?;
    let (right_high, right_low) = refinement_dot2_split(right, counters)?;
    let left_high_right_high = refinement_dot2_mul(left_high, right_high, counters)?;
    let remainder = refinement_dot2_sub(product, left_high_right_high, counters)?;
    let left_low_right_high = refinement_dot2_mul(left_low, right_high, counters)?;
    let remainder = refinement_dot2_sub(remainder, left_low_right_high, counters)?;
    let left_high_right_low = refinement_dot2_mul(left_high, right_low, counters)?;
    let remainder = refinement_dot2_sub(remainder, left_high_right_low, counters)?;
    let left_low_right_low = refinement_dot2_mul(left_low, right_low, counters)?;
    let error = refinement_dot2_sub(left_low_right_low, remainder, counters)?;
    Ok((product, error))
}

#[cfg(test)]
fn refinement_dot2_two_sum(
    left: f64,
    right: f64,
    counters: &mut RefinementDot2Counters,
) -> Result<(f64, f64), RefinementDot2Reason> {
    let x = refinement_dot2_add(left, right, counters)?;
    let z = refinement_dot2_sub(x, left, counters)?;
    let x_minus_z = refinement_dot2_sub(x, z, counters)?;
    let left_error = refinement_dot2_sub(left, x_minus_z, counters)?;
    let right_error = refinement_dot2_sub(right, z, counters)?;
    let error = refinement_dot2_add(left_error, right_error, counters)?;
    Ok((x, error))
}

#[cfg(test)]
fn m1_trust_region_refinement_dot2_for_test(
    terms: [f64; 22],
    weights: [f64; 22],
) -> Result<RefinementDot2Observation, RefinementDot2Refusal> {
    let mut counters = RefinementDot2Counters::default();
    let result = (|| -> Result<RefinementDot2Observation, RefinementDot2Reason> {
        for value in terms {
            refinement_dot2_guard(value, &mut counters)?;
        }
        for value in weights {
            refinement_dot2_guard(value, &mut counters)?;
        }
        let (mut p, mut s) = refinement_dot2_two_product(terms[0], weights[0], &mut counters)?;
        let initial_product_bits = p.to_bits();
        for index in 1..terms.len() {
            let (h, r) = refinement_dot2_two_product(terms[index], weights[index], &mut counters)?;
            let (next_p, q) = refinement_dot2_two_sum(p, h, &mut counters)?;
            p = next_p;
            let q_plus_r = refinement_dot2_add(q, r, &mut counters)?;
            s = refinement_dot2_add(s, q_plus_r, &mut counters)?;
        }
        let value = refinement_dot2_add(p, s, &mut counters)?;
        Ok(RefinementDot2Observation {
            value,
            initial_product_bits,
            arithmetic: counters.arithmetic,
            guards: counters.guards,
            exceptional_guards: counters.exceptional_guards,
            signed_zero_values: counters.signed_zero_values,
        })
    })();
    result.map_err(|reason| RefinementDot2Refusal {
        reason,
        arithmetic: counters.arithmetic,
        guards: counters.guards,
        exceptional_guards: counters.exceptional_guards,
        signed_zero_values: counters.signed_zero_values,
    })
}

// These controls load independent retained operands/results; they do not
// manufacture a candidate result or invoke a physical controller path.
#[cfg(test)]
mod bvls03_c1_expected_red_controls {
    use super::m1_trust_region_refinement_dot2_for_test;
    use serde_json::Value;

    const RETAINED_CAPTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/finite-precision-observed-capture.json"
    ));
    const RETAINED_RESULTS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/dot2-results.json"
    ));
    const RETAINED_HISTORICAL_CAPTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/face-pivot-observed-capture.json"
    ));

    fn bit(value: &Value) -> f64 {
        f64::from_bits(
            u64::from_str_radix(value.as_str().expect("binary64 hex"), 16)
                .expect("valid binary64 bits"),
        )
    }

    fn retained_row(case: usize, row: usize) -> ([f64; 22], [f64; 22], u64, u64) {
        let capture: Value = serde_json::from_str(if case == 0 {
            RETAINED_CAPTURE
        } else {
            RETAINED_HISTORICAL_CAPTURE
        })
        .expect("retained capture");
        let results: Value = serde_json::from_str(RETAINED_RESULTS).expect("independent results");
        let p = results["cases"][case]["first_correction"]["p_bits"]
            .as_array()
            .expect("retained first correction");
        let matrix = capture["weighted_matrix"]
            .as_array()
            .expect("weighted matrix");
        let residual = capture["weighted_residual"]
            .as_array()
            .expect("weighted residual");
        let terms = std::array::from_fn(|index| {
            if index == 0 {
                bit(&residual[row])
            } else {
                bit(&matrix[row][index - 1])
            }
        });
        let weights =
            std::array::from_fn(|index| if index == 0 { 1.0 } else { bit(&p[index - 1]) });
        let expected = u64::from_str_radix(
            results["cases"][case]["dot2_rows"][row]["bits"]
                .as_str()
                .expect("retained Dot2 bits"),
            16,
        )
        .expect("valid retained Dot2 bits");
        let guards = results["cases"][case]["dot2_rows"][row]["guards"]
            .as_u64()
            .expect("retained guard count");
        (terms, weights, expected, guards)
    }

    #[test]
    fn m1_trust_region_refinement_dot2_all_retained_rows_preserve_independent_bits_and_guards() {
        for case in 0..2 {
            for row in 0..21 {
                let (terms, weights, expected_bits, expected_guards) = retained_row(case, row);
                let observed = m1_trust_region_refinement_dot2_for_test(terms, weights)
                    .expect("admitted retained row");
                assert_eq!(
                    observed.value.to_bits(),
                    expected_bits,
                    "case {case}, row {row}"
                );
                assert_eq!(observed.arithmetic, 543, "case {case}, row {row}");
                assert_eq!(
                    u64::from(observed.guards),
                    expected_guards,
                    "case {case}, row {row}"
                );
                assert_eq!(observed.exceptional_guards, 0, "case {case}, row {row}");
            }
        }
    }

    #[test]
    fn m1_trust_region_refinement_dot2_preserves_negative_zero() {
        let observed = m1_trust_region_refinement_dot2_for_test(
            [
                -0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
        )
        .expect("signed-zero primitive remains admitted");
        assert_eq!(observed.initial_product_bits, (-0.0_f64).to_bits());
        assert_eq!(observed.value.to_bits(), 0.0_f64.to_bits());
        assert!(observed.signed_zero_values > 0);
    }

    #[test]
    fn m1_trust_region_refinement_dot2_refuses_guarded_primitive_domains() {
        for (terms, weights, reason, expected) in [
            (
                [f64::NAN; 22],
                [1.0; 22],
                "refinement_nonfinite_operand",
                (0, 1, 1, 0),
            ),
            (
                [f64::INFINITY; 22],
                [1.0; 22],
                "refinement_nonfinite_operand",
                (0, 1, 1, 0),
            ),
            (
                [f64::from_bits(1); 22],
                [1.0; 22],
                "refinement_subnormal_intermediate",
                (0, 1, 1, 0),
            ),
            (
                [f64::MIN_POSITIVE; 22],
                [0.5; 22],
                "refinement_subnormal_intermediate",
                (1, 45, 1, 0),
            ),
            (
                [f64::MAX; 22],
                [1.0; 22],
                "refinement_splitter_overflow",
                (2, 46, 1, 0),
            ),
            (
                [f64::MAX; 22],
                [f64::MAX; 22],
                "refinement_nonfinite_operand",
                (1, 45, 1, 0),
            ),
            (
                [f64::MIN_POSITIVE; 22],
                [f64::MIN_POSITIVE; 22],
                "refinement_underflow_to_zero",
                (1, 45, 1, 1),
            ),
        ] {
            let refusal = m1_trust_region_refinement_dot2_for_test(terms, weights)
                .expect_err("guarded primitive domain must refuse");
            assert_eq!(refusal.reason(), reason);
            assert_eq!(
                (
                    refusal.arithmetic,
                    refusal.guards,
                    refusal.exceptional_guards,
                    refusal.signed_zero_values,
                ),
                expected,
            );
        }
    }
}

// C2--C4 are expected-red declarations for the next reviewed seam. They bind
// the retained source operands and independent results directly, rather than a
// second numerical implementation or manufactured refusal values.
#[cfg(test)]
mod bvls03_c2_c4_expected_red_controls {
    use super::{
        RefinementEligibilityProbe, RefinementWorkReservation,
        m1_trust_region_bvls02_outcome_for_test, m1_trust_region_refinement_eligibility_for_test,
        m1_trust_region_refinement_retained_face_for_test,
    };
    #[cfg(feature = "m1-trust-region-physical-stage")]
    use super::{
        RefinementRawCandidateMutation, m1_trust_region_refinement_scratch_hooks_for_test,
    };
    use serde_json::Value;

    const ORDINARY_CAPTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/finite-precision-observed-capture.json"
    ));
    const HISTORICAL_CAPTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/face-pivot-observed-capture.json"
    ));
    const RESULTS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/dot2-results.json"
    ));

    fn retained(case: usize) -> (Value, Value) {
        let capture = serde_json::from_str(if case == 0 {
            ORDINARY_CAPTURE
        } else {
            HISTORICAL_CAPTURE
        })
        .expect("retained capture");
        let results: Value = serde_json::from_str(RESULTS).expect("independent Dot2 results");
        (capture, results["cases"][case].clone())
    }

    fn bits(values: &Value) -> Vec<u64> {
        values
            .as_array()
            .expect("bit array")
            .iter()
            .map(|value| {
                u64::from_str_radix(value.as_str().expect("binary64 bits"), 16).expect("valid bits")
            })
            .collect()
    }

    fn bit(value: &Value) -> f64 {
        f64::from_bits(
            u64::from_str_radix(value.as_str().expect("binary64 bits"), 16).expect("valid bits"),
        )
    }

    fn mask(values: &Value) -> Vec<bool> {
        values
            .as_array()
            .expect("mask array")
            .iter()
            .map(|value| value.as_bool().expect("mask bit"))
            .collect()
    }

    fn coordinate_bits(expected: &Value, field: &str) -> Vec<u64> {
        expected["second_correction"]["assessment"]["coordinates"]
            .as_array()
            .expect("assessment coordinates")
            .iter()
            .map(|coordinate| {
                u64::from_str_radix(coordinate[field].as_str().expect("coordinate bits"), 16)
                    .expect("valid coordinate bits")
            })
            .collect()
    }

    #[test]
    fn m1_trust_region_refinement_retained_face_reuses_factor_and_reproduces_all_bits() {
        for case in 0..2 {
            let (capture, expected) = retained(case);
            let observed = m1_trust_region_refinement_retained_face_for_test(&capture, None)
                .expect("retained face completes through unchanged BVLS02 decision");
            assert_eq!(
                observed.p0_bits,
                bits(&capture["operations"][1]["p_candidate"])
            );
            assert_eq!(
                observed.delta0_free_bits,
                bits(&expected["first_correction"]["delta_free_bits"])
            );
            assert_eq!(
                observed.p1_bits,
                bits(&expected["first_correction"]["p_bits"])
            );
            assert_eq!(
                observed.dot2_residual_bits,
                expected["dot2_rows"]
                    .as_array()
                    .expect("rows")
                    .iter()
                    .map(
                        |row| u64::from_str_radix(row["bits"].as_str().expect("row bits"), 16)
                            .expect("valid row bits")
                    )
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                observed.delta1_free_bits,
                bits(&expected["second_correction"]["delta_free_bits"])
            );
            assert_eq!(
                observed.p2_bits,
                bits(&expected["second_correction"]["p_bits"])
            );
            assert_eq!(observed.h_bits, coordinate_bits(&expected, "hhat_bits"));
            assert_eq!(observed.cbar_bits, coordinate_bits(&expected, "cbar_bits"));
            assert_eq!(observed.tau_bits, coordinate_bits(&expected, "tau_bits"));
            assert_eq!(
                observed.classes,
                expected["second_correction"]["assessment"]["coordinates"]
                    .as_array()
                    .expect("coordinates")
                    .iter()
                    .map(|coordinate| coordinate["classification"]
                        .as_str()
                        .expect("class")
                        .to_owned())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                observed.box_feasible,
                expected["second_correction"]["assessment"]["box_feasible"]
                    .as_bool()
                    .expect("box feasible")
            );
            assert_eq!(
                observed.ball_feasible,
                expected["second_correction"]["assessment"]["ball_feasible"]
                    .as_bool()
                    .expect("ball feasible")
            );
            assert_eq!(observed.factorizations_added, 0);
            assert_eq!(observed.factor_application_count, 2);
            assert_eq!(
                observed.factor_identity_before,
                observed.factor_identity_after
            );
            assert_eq!(
                observed.factor_application_identities,
                vec![observed.factor_identity_before; 2]
            );
            assert_eq!(
                observed.lower_mask_p0,
                mask(&capture["operations"][1]["lower_before"])
            );
            assert_eq!(
                observed.upper_mask_p0,
                mask(&capture["operations"][1]["upper_before"])
            );
            assert_eq!(observed.lower_mask_p1, observed.lower_mask_p0);
            assert_eq!(observed.upper_mask_p1, observed.upper_mask_p0);
            assert_eq!(observed.lower_mask_p2, observed.lower_mask_p0);
            assert_eq!(observed.upper_mask_p2, observed.upper_mask_p0);
            assert_eq!(observed.lower_mask_exit, observed.lower_mask_p0);
            assert_eq!(observed.upper_mask_exit, observed.upper_mask_p0);
            assert_eq!(observed.corrections, 2);
            assert_eq!(observed.first_residual_operations, 903);
            assert_eq!(observed.second_dot2_operations, 11_403);
            assert_eq!(observed.factor_application_operations, 37_920);
            assert_eq!(observed.coordinate_update_operations, 40);
            assert_eq!(
                observed.first_residual_operations
                    + observed.second_dot2_operations
                    + observed.factor_application_operations
                    + observed.coordinate_update_operations,
                50_266
            );
            let (expected_upward_operations, expected_upward_guards, expected_scalar) = if case == 0
            {
                (6_654, 65_916, 58_876)
            } else {
                (6_522, 65_781, 58_744)
            };
            assert_eq!(observed.whole_kkt_operations, 1_827);
            assert_eq!(observed.whole_kkt_guard_events, 5_503);
            assert_eq!(
                observed.upward_enclosure_operations,
                expected_upward_operations
            );
            assert!(observed.upward_enclosure_operations <= 39_816);
            assert_eq!(
                observed.upward_enclosure_guard_events,
                expected_upward_guards
            );
            assert_eq!(observed.refinement_scalar_operations, expected_scalar);
            assert_eq!(
                observed.bvls02_pass,
                expected["second_correction"]["assessment"]["complete_bvls02_and_feasibility_pass"]
                    .as_bool()
                    .expect("BVLS02 decision")
            );
        }
    }

    #[cfg(feature = "m1-trust-region-physical-stage")]
    #[test]
    fn m1_trust_region_refinement_scratch_never_reaches_owner_or_physical_hooks() {
        let (capture, _) = retained(0);
        let observed = m1_trust_region_refinement_scratch_hooks_for_test(&capture, None)
            .expect("retained new-face scratch observation");
        assert!(observed.p1_free_box_violation);
        assert_eq!(observed.physical_evaluator_calls, 0);
        assert_eq!(observed.hydraulic_calls, 0);
        assert_eq!(observed.materialization_calls, 0);
        assert_eq!(observed.global_before, observed.global_after);
        assert_eq!(
            observed.global_after.phase_join_selections
                - observed.global_before.phase_join_selections,
            0
        );
        assert_eq!(
            observed.global_after.frozen_weighted_merit_entries
                - observed.global_before.frozen_weighted_merit_entries,
            0
        );
        assert_eq!(
            observed.live_state_bits_before,
            observed.live_state_bits_after
        );
        assert!(observed.budget_before.is_some());
        assert_eq!(observed.budget_before, observed.budget_after);
        assert_ne!(
            observed.budget_before,
            Some(crate::M1PhysicalBudgetSnapshot::default())
        );
        let operation = &capture["operations"][1];
        let active_coordinate = 5;
        let free_coordinate = 13;
        let p0 = bit(&operation["p_candidate"][free_coordinate]);
        let lower = bit(&capture["lower"][free_coordinate]);
        let upper = bit(&capture["upper"][free_coordinate]);
        let base = bit(&capture["base"][free_coordinate]);
        let scale = bit(&capture["scales"][free_coordinate]);
        let scaled_upper = (upper - base) / scale;
        let radius = bit(&operation["radius"]);
        for (mutation, reason) in [
            (
                RefinementRawCandidateMutation::FinalBox {
                    coordinate: free_coordinate,
                    scaled_value: scaled_upper + 1.0,
                    original_lower: lower,
                    original_upper: upper,
                    base,
                    scale,
                    radius,
                },
                "refinement_final_box",
            ),
            (
                RefinementRawCandidateMutation::ScratchRadius {
                    coordinate: free_coordinate,
                    scaled_value: p0,
                    original_lower: lower,
                    original_upper: upper,
                    base,
                    scale,
                    radius: 0.0,
                },
                "refinement_scratch_radius",
            ),
            (
                RefinementRawCandidateMutation::ActiveValue {
                    coordinate: active_coordinate,
                    scaled_value: bit(&operation["p_candidate"][active_coordinate]) + 1.0,
                    original_lower: bit(&capture["lower"][active_coordinate]),
                    original_upper: bit(&capture["upper"][active_coordinate]),
                    base: bit(&capture["base"][active_coordinate]),
                    scale: bit(&capture["scales"][active_coordinate]),
                    radius,
                },
                "refinement_active_value",
            ),
        ] {
            let refusal =
                m1_trust_region_refinement_scratch_hooks_for_test(&capture, Some(mutation))
                    .expect_err("actual mutated retained input must refuse");
            assert_eq!(refusal.reason, reason);
            let metadata = refusal
                .refinement
                .as_ref()
                .expect("actual refinement guard metadata");
            match reason {
                "refinement_final_box" => {
                    assert_eq!(metadata.reason, M1TrustRegionRefinementReason::FinalBox);
                    assert_eq!(
                        metadata.correction_position,
                        Some(M1TrustRegionCorrectionPosition::Second)
                    );
                    assert_eq!(
                        metadata.guard_stage,
                        M1TrustRegionRefinementGuardStage::FinalBoxUpper
                    );
                    assert_eq!(metadata.coordinate, Some(13));
                }
                "refinement_scratch_radius" => {
                    assert_eq!(
                        metadata.reason,
                        M1TrustRegionRefinementReason::ScratchRadius
                    );
                    assert_eq!(
                        metadata.correction_position,
                        Some(M1TrustRegionCorrectionPosition::First)
                    );
                    assert_eq!(
                        metadata.guard_stage,
                        M1TrustRegionRefinementGuardStage::RadiusComparison
                    );
                    assert_eq!(metadata.coordinate, None);
                }
                "refinement_active_value" => {
                    assert_eq!(metadata.reason, M1TrustRegionRefinementReason::ActiveValue);
                    assert_eq!(
                        metadata.correction_position,
                        Some(M1TrustRegionCorrectionPosition::Second)
                    );
                    assert_eq!(
                        metadata.guard_stage,
                        M1TrustRegionRefinementGuardStage::ActiveEquality
                    );
                    assert_eq!(metadata.coordinate, Some(5));
                }
                _ => unreachable!("control only names typed refinement refusals"),
            }
            assert_eq!(
                metadata.lower_active.to_vec(),
                mask(&operation["lower_before"])
            );
            assert_eq!(
                metadata.upper_active.to_vec(),
                mask(&operation["upper_before"])
            );
            assert_eq!(metadata.free.to_vec(), mask(&operation["free_before"]));
            assert_eq!(
                metadata.lambda.to_bits(),
                bit(&operation["lambda"]).to_bits()
            );
            assert_eq!(
                metadata.original_trust_radius.to_bits(),
                bit(&operation["radius"]).to_bits()
            );
            assert_eq!(metadata.scratch_box_feasible, Some(false));
            assert_eq!(
                refusal.live_state_bits_before,
                refusal.live_state_bits_after
            );
            let global_before = refusal
                .global_before
                .expect("actual pre-refusal global snapshot");
            let global_after = refusal
                .global_after
                .expect("actual post-refusal global snapshot");
            assert_eq!(global_before, global_after);
            let budget_before = refusal
                .budget_before
                .expect("actual pre-refusal budget snapshot");
            let budget_after = refusal
                .budget_after
                .expect("actual restored budget snapshot");
            assert_eq!(budget_before, budget_after);
            assert_ne!(budget_before, crate::M1PhysicalBudgetSnapshot::default());
        }
    }

    #[test]
    fn m1_trust_region_refinement_eligibility_uses_raw_face_predicates_and_counts_denied_work() {
        let (capture, _) = retained(0);
        let base = RefinementEligibilityProbe::from_captured_face(&capture);
        for (probe, expected_action) in [
            (base.with_lambda(0.0), "refine"),
            (base.with_lambda(-0.0), "refine"),
            (base.with_lambda(1.0), "bvls02"),
            (base.with_trust_radius(base.p0_norm()), "bvls02"),
            (base.with_p0_free_coordinate_outside_box(), "bvls02"),
        ] {
            let baseline = (expected_action == "bvls02")
                .then(|| m1_trust_region_bvls02_outcome_for_test(&capture, probe))
                .transpose()
                .expect("same-probe unchanged BVLS02 baseline");
            let observed =
                m1_trust_region_refinement_eligibility_for_test(&capture, probe, baseline.as_ref())
                    .expect("actual raw eligibility probe");
            assert_eq!(observed.action, expected_action);
            assert_eq!(
                observed.maximum_corrections,
                if expected_action == "refine" { 2 } else { 0 }
            );
            if expected_action == "bvls02" {
                assert_eq!(observed.bvls02, baseline.expect("BVLS02 baseline"));
            }
            if probe.p0_outside_box {
                assert_eq!(observed.bvls02.outcome, "crossing");
                let selected = observed
                    .bvls02
                    .coordinate
                    .expect("actual crossing selected a free coordinate");
                assert!(observed.bvls02.crossing_action.is_some());
                assert!(!observed.bvls02.lower_mask_before[selected]);
                assert!(!observed.bvls02.upper_mask_before[selected]);
                assert!(
                    observed.bvls02.lower_mask[selected] || observed.bvls02.upper_mask[selected]
                );
            }
            assert_eq!(
                observed.attempted_work_after,
                observed.attempted_work_before + observed.denied_work
            );
        }
        let refusal = m1_trust_region_refinement_retained_face_for_test(
            &capture,
            Some(RefinementWorkReservation {
                remaining_scalar_operations: 92_037,
                remaining_guard_events: 276_366,
                required_scalar_operations: 92_038,
                required_guard_events: 276_367,
            }),
        )
        .expect_err("one-less-than-required refinement reservation must refuse before entry");
        assert_eq!(refusal.reason, "refinement_work_cap");
        assert_eq!(refusal.required_scalar_operations, 92_038);
        assert_eq!(refusal.required_guard_events, 276_367);
        assert_eq!(
            refusal.attempted_entries_after,
            refusal.attempted_entries_before + 1
        );
        assert_eq!(
            refusal.denied_entries_after,
            refusal.denied_entries_before + 1
        );
        assert_eq!(refusal.corrections, 0);
        assert_eq!(refusal.dot2_operations, 0);
        assert_eq!(refusal.factor_applications, 0);
        assert_eq!(refusal.denied_entry_executed_scalar_operations, 0);
        assert_eq!(refusal.denied_entry_executed_guard_events, 0);
        assert_eq!(refusal.prior_eligibility_scalar_operations, 43);
        assert!(refusal.prior_eligibility_guard_events > 0);
        #[cfg(feature = "m1-trust-region-physical-stage")]
        {
            assert!(refusal.budget_before.is_some());
            assert_eq!(refusal.budget_before, refusal.budget_after);
            assert_ne!(
                refusal.budget_before,
                Some(crate::M1PhysicalBudgetSnapshot::default())
            );
        }
    }
}
