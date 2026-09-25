//! Private frozen-weight arithmetic shared by detached Stage 1 controls.

use std::cmp::Ordering;

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
use std::cell::{Cell, RefCell};

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
use crate::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics;

use super::{
    M1TrustRegionBallEndpoint, M1TrustRegionBallObservation, M1TrustRegionJacobiInput,
    M1TrustRegionJacobiObservation, M1TrustRegionJacobiPair, M1TrustRegionLambdaEvaluation,
    M1TrustRegionRefusalKind,
};
use super::{M1TrustRegionMeritInput, M1TrustRegionMeritObservation};

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
    pub(crate) crossing_theta: Option<f64>,
    pub(crate) selected_coordinate: Option<usize>,
    pub(crate) selected_lower: Option<bool>,
    pub(crate) g: Option<[f64; 21]>,
    pub(crate) lambda_times_p: Option<[f64; 21]>,
    pub(crate) h: Option<[f64; 21]>,
    pub(crate) entered_factorizations: u8,
    pub(crate) completed_pivots: u8,
    pub(crate) completed_sweeps: u8,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[derive(Clone, Debug)]
pub(crate) struct M1FacePivotCaptureRefusal {
    pub(crate) stage: &'static str,
    pub(crate) kind: M1TrustRegionRefusalKind,
    pub(crate) p: [f64; 21],
    pub(crate) lower: [bool; 21],
    pub(crate) upper: [bool; 21],
    pub(crate) free: [bool; 21],
    pub(crate) entered_factorizations: u8,
    pub(crate) completed_pivots: u8,
    pub(crate) completed_sweeps: u8,
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
thread_local! {
    static FACE_PIVOT_CAPTURE: RefCell<Option<M1FacePivotCapture>> = const { RefCell::new(None) };
    static FACE_PIVOT_CAPTURE_ENABLED: Cell<bool> = const { Cell::new(false) };
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
        ("g", operation.g),
        ("lambda_times_p", operation.lambda_times_p),
        ("h", operation.h),
    ] {
        let _ = write!(output, ",\"{name}\":");
        append_optional_bits(output, values);
    }
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
                "{{\"stage\":\"{}\",\"kind\":\"{:?}\",\"entered\":{},\"pivots\":{},\"sweeps\":{},\"p\":",
                refusal.stage,
                refusal.kind,
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
        operations: vec![M1FacePivotCaptureOperation {
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
            crossing_theta: Some(f64::INFINITY),
            selected_coordinate: Some(1),
            selected_lower: Some(true),
            g: Some([1.0; 21]),
            lambda_times_p: Some([2.0; 21]),
            h: Some([3.0; 21]),
            entered_factorizations: 43,
            completed_pivots: 42,
            completed_sweeps: 7,
        }],
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
            p: [1.0; 21],
            lower: [false; 21],
            upper: [true; 21],
            free: [false; 21],
            entered_factorizations: 43,
            completed_pivots: 42,
            completed_sweeps: 7,
        }),
    }
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
#[test]
fn m1_face_pivot_populated_capture_json_parses() {
    let capture = populated_face_pivot_capture_for_test();
    let json = m1_face_pivot_capture_json_for_test(Some(&capture));
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("valid populated JSON");
    assert_eq!(parsed["operations"][0]["coordinate"], 1);
    assert_eq!(parsed["operations"][0]["lower"], true);
    assert_eq!(parsed["post_update_admissions"][0]["pass"], false);
    assert_eq!(parsed["refusal"]["kind"], "FacePivotLimit");
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

fn lambda_step(factor: &JacobiFactor, y: &[f64], lambda: f64) -> Result<Vec<f64>, &'static str> {
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
                let term = factor.a[row][free] * factor.v[free][j];
                if !term.is_finite() {
                    return Err("nonfinite A_free V product");
                }
                av += term;
                if !av.is_finite() {
                    return Err("nonfinite A_free V sum");
                }
            }
            let u = av / sigma;
            if !u.is_finite() || !y_value.is_finite() {
                return Err("nonfinite U");
            }
            let term = u * y_value;
            if !term.is_finite() {
                return Err("nonfinite Uty product");
            }
            uty += term;
            if !uty.is_finite() {
                return Err("nonfinite Uty sum");
            }
        }
        let denom = sigma * sigma + lambda;
        let gain = sigma / denom;
        let coefficient = -(gain * uty);
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
            let term = factor.v[out][j] * coefficients[j];
            if !term.is_finite() {
                return Err("nonfinite V product");
            }
            sum += term;
            if !sum.is_finite() {
                return Err("nonfinite V sum");
            }
        }
        *output = sum;
    }
    Ok(step)
}

fn lambda_norm(step: &[f64]) -> Result<f64, &'static str> {
    let mut sum = 0.0;
    for &x in step {
        let sq = x * x;
        if !sq.is_finite() {
            return Err("nonfinite norm");
        }
        sum += sq;
        if !sum.is_finite() {
            return Err("nonfinite norm");
        }
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
    let zero = lambda_step(factor, y, 0.0).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    let n0 = lambda_norm(&zero).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
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
        lambda_step(factor, y, hi).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    let mut nh = lambda_norm(&high).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
    trace.initial_bracket = [0.0, hi];
    trace.bracket = [0.0, hi];
    trace.bracket_evaluations.push((hi, nh));
    for _ in 0..48 {
        if nh <= radius {
            break;
        }
        lo = hi;
        hi *= 4.0;
        high = lambda_step(factor, y, hi).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
        nh = lambda_norm(&high).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
        trace.bracket_evaluations.push((hi, nh));
    }
    if nh > radius {
        return Err(M1TrustRegionRefusalKind::LambdaBracket);
    }
    for _ in 0..48 {
        let mid = (lo + hi) * 0.5;
        let step =
            lambda_step(factor, y, mid).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
        let norm = lambda_norm(&step).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
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
    let norm = lambda_norm(&trace.step).map_err(|_| M1TrustRegionRefusalKind::SvdNonFinite)?;
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
        attempted_subproblems,
        completed_jacobi_sweeps,
        completed_face_pivots,
        entered_svd_factorizations,
        raw_residual_max,
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
            crossing_theta: operation.crossing_theta,
            selected_coordinate: operation.selected_coordinate,
            selected_lower: operation.selected_lower,
            g: operation.g,
            lambda_times_p: operation.lambda_times_p,
            h: operation.h,
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
    state: &SubproblemState,
) {
    record_face_pivot_capture(|capture| {
        capture.refusal = Some(M1FacePivotCaptureRefusal {
            stage: refusal_stage,
            kind,
            p: state.p,
            lower: state.lower_active,
            upper: state.upper_active,
            free: state.free,
            entered_factorizations: state.entered,
            completed_pivots: state.completed_pivots,
            completed_sweeps: state.completed_sweeps,
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
            crossing_theta: Some(crossing_theta),
            selected_coordinate: Some(selected_coordinate),
            selected_lower: Some(selected_lower),
            g: None,
            lambda_times_p: None,
            h: None,
            entered_factorizations: 0,
            completed_pivots: 0,
            completed_sweeps: 0,
        },
    );
}

#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
struct ReleaseCapture<'a> {
    solution: &'a FaceSolution,
    values: &'a KktValues,
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
            crossing_theta: None,
            selected_coordinate: Some(selected_coordinate),
            selected_lower: Some(selected_lower),
            g: Some(capture.values.gradient),
            lambda_times_p: Some(capture.values.lambda_times_step),
            h: Some(capture.values.h),
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
    subproblem_refusal(
        kind,
        1,
        state.completed_sweeps,
        state.completed_pivots,
        state.entered,
        raw_max,
    )
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

fn box_feasible_kkt_values(
    weighted_matrix: &[Vec<f64>],
    weighted_residual: &[f64],
    step: &[f64; 21],
    lambda: f64,
) -> Result<KktValues, M1TrustRegionRefusalKind> {
    const N: usize = 21;
    let mut residual = [0.0; N];
    let mut gradient = [0.0; N];
    let mut lambda_times_step = [0.0; N];
    let mut h = [0.0; N];
    for row in 0..N {
        let mut ap_sum = 0.0;
        for col in 0..N {
            let term = weighted_matrix[row][col] * step[col];
            if !term.is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
            ap_sum += term;
            if !ap_sum.is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
        }
        residual[row] = weighted_residual[row] + ap_sum;
        if !residual[row].is_finite() {
            return Err(M1TrustRegionRefusalKind::SvdNonFinite);
        }
    }
    for col in 0..N {
        for row in 0..N {
            let term = weighted_matrix[row][col] * residual[row];
            if !term.is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
            gradient[col] += term;
            if !gradient[col].is_finite() {
                return Err(M1TrustRegionRefusalKind::SvdNonFinite);
            }
        }
        lambda_times_step[col] = lambda * step[col];
        h[col] = gradient[col] + lambda_times_step[col];
        if !lambda_times_step[col].is_finite() || !h[col].is_finite() {
            return Err(M1TrustRegionRefusalKind::SvdNonFinite);
        }
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

fn largest_kkt_release(
    lower_active: &[bool; 21],
    upper_active: &[bool; 21],
    h: &[f64; 21],
) -> Option<(usize, bool)> {
    let mut release = None;
    for col in 0..21 {
        let violation = if lower_active[col] && h[col] < 0.0 {
            Some((-h[col], true))
        } else if upper_active[col] && h[col] > 0.0 {
            Some((h[col], false))
        } else {
            None
        };
        if let Some((size, is_lower)) = violation {
            if release.is_none_or(|(best, index, _)| {
                size > best || (size.partial_cmp(&best) == Some(Ordering::Equal) && col < index)
            }) {
                release = Some((size, col, is_lower));
            }
        }
    }
    release.map(|(_, col, is_lower)| (col, is_lower))
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
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
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
    let factor = match jacobi_factor(
        face.matrix,
        &face.free_ids,
        input.test_work.completed_jacobi_sweeps,
        &mut state.completed_sweeps,
    ) {
        Ok(value) => value,
        Err("rank deficient") => {
            return Err(state_refusal(
                M1TrustRegionRefusalKind::RankDeficient,
                state,
                prepared.raw_max,
            ));
        }
        Err("no convergence") => {
            return Err(subproblem_refusal(
                M1TrustRegionRefusalKind::SvdNoConvergence,
                1,
                64,
                state.completed_pivots,
                state.entered,
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
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        radius: face.radius,
    })
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
) -> Result<Option<KktValues>, super::M1TrustRegionSubproblemRefusal> {
    let lambda = solution.trace.lambda;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let p_before = state.p;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let lower_before = state.lower_active;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let upper_before = state.upper_active;
    #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
    let free_before = state.free;
    let values = box_feasible_kkt_values(
        &prepared.weighted_matrix,
        &prepared.weighted_residual,
        &state.p,
        lambda,
    )
    .map_err(|kind| state_refusal(kind, state, prepared.raw_max))?;
    if let Some((col, is_lower)) =
        largest_kkt_release(&state.lower_active, &state.upper_active, &values.h)
    {
        if state.completed_pivots >= 42 {
            #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
            {
                capture_release(
                    state,
                    &ReleaseCapture {
                        solution,
                        values: &values,
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
                values: &values,
                p_before,
                masks_before: (lower_before, upper_before, free_before),
                selected: (col, is_lower),
                applied: true,
            },
        );
        return Ok(None);
    }
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
    Ok(Some(values))
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
                        operations: Vec::new(),
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
        let solution = solve_face(input, &prepared, &mut state)?;
        if activate_first_crossing(&prepared, &mut state, &solution)? {
            continue;
        }
        let Some(values) = kkt_transition(&prepared, &mut state, &solution)? else {
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
            event_trace: state.events,
            entered_svd_factorizations: state.entered,
            completed_face_pivots: state.completed_pivots,
            raw_jacobian_columns: raw_jacobian_columns(input),
            active_lower: state.lower_active,
            active_upper: state.upper_active,
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
) -> Result<Option<super::M1TrustRegionKktAction>, super::M1TrustRegionSubproblemRefusal> {
    let prepared = PreparedSubproblem {
        raw_max: weighted_residual
            .iter()
            .fold(0.0_f64, |maximum, value| maximum.max(value.abs())),
        weighted_residual: weighted_residual.to_vec(),
        weighted_matrix,
        scaled_lower: [f64::NEG_INFINITY; 21],
        scaled_upper: [f64::INFINITY; 21],
    };
    let mut state = SubproblemState {
        p: step,
        lower_active,
        upper_active,
        free,
        initial_free: free,
        events: Vec::new(),
        entered: 1,
        completed_pivots: 0,
        completed_sweeps: 0,
    };
    let solution = FaceSolution {
        free_ids: Vec::new(),
        trace: LambdaTrace {
            lambda,
            ..LambdaTrace::default()
        },
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        radius: 1.0,
    };
    let result = kkt_transition(&prepared, &mut state, &solution)?;
    Ok(result.and_then(|_| state.events.last().map(|event| event.action)))
}

#[cfg(test)]
mod finite_precision_controls {
    use super::{
        box_feasible_kkt_values, largest_kkt_release, m1_trust_region_kkt_transition_for_test,
    };
    use crate::M1TrustRegionKktAction;
    use serde_json::Value;

    const RETAINED_CAPTURE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/face-pivot-observed-capture.json"
    ));

    fn bit(value: &Value) -> f64 {
        f64::from_bits(
            u64::from_str_radix(value.as_str().expect("binary64 hex"), 16)
                .expect("valid binary64 bits"),
        )
    }

    fn vector(value: &Value) -> [f64; 21] {
        std::array::from_fn(|index| bit(&value[index]))
    }

    #[test]
    fn retained_capture_uses_shared_ordered_kkt_operation_before_policy_change() {
        let capture: Value = serde_json::from_str(RETAINED_CAPTURE).expect("retained capture");
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
        assert_eq!(
            largest_kkt_release(&lower, &upper, &values.h),
            Some((5, true))
        );
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
                [false; 21],
                0.0
            )
            .expect("shared KKT transition"),
            None,
            "expected-red: reviewed 02 uncertainty-band decision replaces sign-only release"
        );
    }

    #[test]
    fn expected_red_upper_and_free_coordinates_require_full_shared_transition() {
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
                [false; 21],
                0.0
            )
            .expect("shared upper transition"),
            None,
            "expected-red: uncertainty-band upper multiplier is retained"
        );

        let mut free_matrix = vec![vec![0.0; 21]; 21];
        free_matrix[0][0] = 1.0;
        let mut free_residual = [0.0; 21];
        free_residual[0] = 1.0;
        let free = std::array::from_fn(|index| index == 0);
        assert!(
            m1_trust_region_kkt_transition_for_test(
                free_matrix,
                free_residual,
                [0.0; 21],
                [false; 21],
                [false; 21],
                free,
                0.0,
            )
            .is_err(),
            "expected-red: materially nonstationary free coordinate must refuse"
        );
    }

    #[test]
    fn shared_signed_zero_descent_and_nonfinite_operand_controls_are_not_canned() {
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = 1.0;
        let mut residual = [0.0; 21];
        residual[0] = -1.0;
        let lower = std::array::from_fn(|index| index == 0);
        let mut step = [0.0; 21];
        step[0] = -0.0;
        assert_eq!(
            m1_trust_region_kkt_transition_for_test(
                matrix.clone(),
                residual,
                step,
                lower,
                [false; 21],
                [false; 21],
                0.0
            )
            .expect("shared signed-zero descent transition"),
            Some(M1TrustRegionKktAction::ReleaseLower)
        );
        matrix[0][0] = f64::NAN;
        assert!(
            m1_trust_region_kkt_transition_for_test(
                matrix,
                residual,
                step,
                lower,
                [false; 21],
                [false; 21],
                0.0
            )
            .is_err()
        );
    }

    #[test]
    fn expected_red_cancellation_band_is_stable_under_scaling_and_permutation() {
        for scale in [1.0, 8.0] {
            let mut matrix = vec![vec![0.0; 21]; 21];
            matrix[0][0] = scale;
            matrix[0][1] = scale;
            let mut residual = [0.0; 21];
            residual[0] = (-1.0 - f64::EPSILON) * scale;
            let mut step = [0.0; 21];
            step[1] = 1.0;
            let lower = std::array::from_fn(|index| index == 0);
            assert_eq!(
                m1_trust_region_kkt_transition_for_test(
                    matrix,
                    residual,
                    step,
                    lower,
                    [false; 21],
                    [false; 21],
                    0.0
                )
                .expect("shared scaled transition"),
                None,
                "expected-red: scaled cancellation band remains unresolved"
            );
        }
        let mut matrix = vec![vec![0.0; 21]; 21];
        matrix[0][0] = 1.0;
        matrix[0][1] = 1.0;
        let mut residual = [0.0; 21];
        residual[0] = -1.0 - f64::EPSILON;
        let mut step = [0.0; 21];
        step[0] = 1.0;
        let lower = std::array::from_fn(|index| index == 1);
        assert_eq!(
            m1_trust_region_kkt_transition_for_test(
                matrix,
                residual,
                step,
                lower,
                [false; 21],
                [false; 21],
                0.0
            )
            .expect("shared permuted transition"),
            None,
            "expected-red: permuted cancellation band remains unresolved"
        );
    }

    #[test]
    fn exact_fixed_bound_is_outside_shared_active_and_free_sets() {
        let matrix = vec![vec![0.0; 21]; 21];
        assert_eq!(
            m1_trust_region_kkt_transition_for_test(
                matrix,
                [0.0; 21],
                [0.0; 21],
                [false; 21],
                [false; 21],
                [false; 21],
                0.0,
            )
            .expect("shared fixed-only transition"),
            Some(M1TrustRegionKktAction::Return)
        );
    }
}
