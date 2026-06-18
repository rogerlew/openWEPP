use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::hint::black_box;
use std::time::{Duration, Instant};

use openwepp_hillslope_orchestrator::Wb11HydrologyKernel;
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest, HillslopeProductionStateSymbol,
    KernelWritebackPayload, SymbolId, SymbolRegistry,
};
use openwepp_sim_contract::status::StatusClassification;

const ZERO_THRESHOLD: f64 = 1.0e-12;
const WB15_CANCOV_MAX: f64 = 0.999;
const WB15_BIOMASS_TO_KG_HA: f64 = 10_000.0;
const WB15_INTERCEPT_BIOMASS_MAX_KG_HA: f64 = 8_000.0;
const WB15_INTERCEPT_LINEAR_COEFF: f64 = 0.000_627;
const WB15_INTERCEPT_QUADRATIC_COEFF: f64 = 3.733_49e-8;
const WB15_INTERCEPT_MM_TO_M: f64 = 1_000.0;
const WB14_INTERVAL_INFILTRATION_ROUNDOFF_TOLERANCE_M: f64 = 1.0e-10;

const LOGICAL_ITERATIONS: usize = 10_000;
const ARRAY_ITERATIONS: usize = 5_000_000;
const ARRAY_PERF_ITERATIONS: usize = 80_000_000;
const BOUNDARY_ITERATIONS: usize = 20_000;
const WARMUPS: usize = 1_000;
const REPEATS: usize = 5;

const H2637_OFE_DAYS: f64 = 235_961.0;
const LEGACY_US_PER_OFE_DAY: f64 = 38.65;
const LEGACY_H2637_NO_UI_SECONDS: f64 = 9.12;

#[derive(Debug)]
enum PrototypeError {
    Domain(&'static str),
    OutputMismatch {
        symbol: String,
        expected: f64,
        observed: f64,
    },
    Status(String),
}

impl fmt::Display for PrototypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Domain(message) => write!(formatter, "{message}"),
            Self::OutputMismatch {
                symbol,
                expected,
                observed,
            } => write!(
                formatter,
                "output mismatch for {symbol}: expected {expected:.17e}, observed {observed:.17e}"
            ),
            Self::Status(message_id) => {
                write!(
                    formatter,
                    "production kernel returned non-nominal status {message_id}"
                )
            }
        }
    }
}

impl Error for PrototypeError {}

#[derive(Clone, Copy)]
struct RunoffInputs {
    rainfall_input_m: f64,
    closure_tolerance_m: f64,
    soil_conductivity_m_s: f64,
    soil_layer_depth_m: f64,
    theta_residual: f64,
    theta_field_capacity: f64,
    timem_s: [f64; 2],
    intensity_m_s: [f64; 2],
    cancov: f64,
    lai: f64,
    vdmt: f64,
    runon_input_m: f64,
    depression_storage_delta_m: f64,
}

impl RunoffInputs {
    fn h2637_like_warm_rain() -> Self {
        Self {
            rainfall_input_m: 0.042,
            closure_tolerance_m: 1.0e-9,
            soil_conductivity_m_s: 3.25e-7,
            soil_layer_depth_m: 0.20,
            theta_residual: 0.08,
            theta_field_capacity: 0.34,
            timem_s: [0.0, 86_400.0],
            intensity_m_s: [0.042 / 86_400.0, 0.0],
            cancov: 0.71,
            lai: 2.2,
            vdmt: 1.9,
            runon_input_m: 0.0065,
            depression_storage_delta_m: 0.00125,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct RunoffOutputs {
    infiltration_m: f64,
    q_runoff_m: f64,
    soil_conductivity_m_s: f64,
    effective_conductivity_m_s: f64,
    matric_potential_m: f64,
    interception_m: f64,
    runon_input_m: f64,
    closure_delta_m: f64,
    snow_post_winter_rain_m: f64,
}

#[derive(Clone, Copy)]
enum OutputSource {
    Infiltration,
    RunoffReconciled,
    SoilConductivity,
    EffectiveConductivity,
    MatricPotential,
    Interception,
    DailyIrrigation,
    RunoffQ,
    RunoffCarryover,
    ClosureDelta,
    SnowCoupling,
    SnowRoutedMelt,
    SnowPostWinterRain,
    Zero,
}

impl OutputSource {
    fn value(self, outputs: &RunoffOutputs) -> f64 {
        match self {
            Self::Infiltration => outputs.infiltration_m,
            Self::RunoffReconciled | Self::RunoffQ => outputs.q_runoff_m,
            Self::SoilConductivity => outputs.soil_conductivity_m_s,
            Self::EffectiveConductivity => outputs.effective_conductivity_m_s,
            Self::MatricPotential => outputs.matric_potential_m,
            Self::Interception => outputs.interception_m,
            Self::DailyIrrigation | Self::SnowCoupling | Self::SnowRoutedMelt | Self::Zero => 0.0,
            Self::RunoffCarryover => outputs.runon_input_m,
            Self::ClosureDelta => outputs.closure_delta_m,
            Self::SnowPostWinterRain => outputs.snow_post_winter_rain_m,
        }
    }
}

#[derive(Clone, Copy)]
enum SurfaceKind {
    State,
    Flux,
}

#[derive(Clone, Copy)]
struct OutputSlot {
    kind: SurfaceKind,
    id: SymbolId,
    source: OutputSource,
}

struct OutputPlan {
    slots: Vec<OutputSlot>,
}

#[derive(Clone)]
struct DenseSurface {
    state: Vec<Option<BoundaryValue>>,
    flux: Vec<Option<BoundaryValue>>,
}

impl DenseSurface {
    fn from_maps(
        registry: &SymbolRegistry,
        state: &BTreeMap<BoundarySymbol, BoundaryValue>,
        flux: &BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut surface = Self {
            state: vec![None; registry.len()],
            flux: vec![None; registry.len()],
        };
        for (symbol, value) in state {
            surface.state[registry.id_of(symbol)?.as_usize()] = Some(*value);
        }
        for (symbol, value) in flux {
            surface.flux[registry.id_of(symbol)?.as_usize()] = Some(*value);
        }
        Ok(surface)
    }

    fn export_maps(
        &self,
        registry: &SymbolRegistry,
    ) -> (
        BTreeMap<BoundarySymbol, BoundaryValue>,
        BTreeMap<BoundarySymbol, BoundaryValue>,
    ) {
        (
            export_one(registry, &self.state),
            export_one(registry, &self.flux),
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mode = env::args().nth(1).unwrap_or_else(|| "measure".to_owned());
    let inputs = RunoffInputs::h2637_like_warm_rain();
    let mut state = BTreeMap::new();
    let mut flux = BTreeMap::new();
    seed_logical_surfaces(&inputs, &mut state, &mut flux);
    let production = run_production(&state, &flux)?;
    let (registry_state, registry_flux) = registry_surfaces(&state, &flux, &production.writeback);
    let registry = SymbolRegistry::from_surfaces(&registry_state, &registry_flux)?;
    let mut dense = DenseSurface::from_maps(&registry, &state, &flux)?;
    let plan = OutputPlan::from_production_payload(&registry, &production.writeback)?;

    validate_array_identity(&inputs, &production.writeback, &plan, &mut dense)?;

    if mode == "array-only" {
        let iterations = env::args()
            .nth(2)
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(ARRAY_PERF_ITERATIONS);
        let checksum = time_array_hot_loop(&inputs, &plan, &mut dense, iterations);
        println!("array_only_iterations\t{iterations}");
        println!("array_only_checksum\t{checksum:.17e}");
        return Ok(());
    }

    for _ in 0..WARMUPS {
        black_box(run_production(&state, &flux)?);
        let outputs = array_runoff_physics(inputs)?;
        array_write_outputs(&plan, &mut dense, &outputs);
        black_box(outputs.q_runoff_m);
    }

    println!(
        "metric\trepeat\titerations\tseconds\tns_per_iter\tus_per_ofe_day\tratio_vs_legacy_per_ofe_day\tprojected_h2637_seconds"
    );
    for repeat in 1..=REPEATS {
        let logical = time_logical_production(&state, &flux)?;
        print_timing(
            "logical_production_kernel",
            repeat,
            LOGICAL_ITERATIONS,
            logical,
        );

        let physics = time_array_physics(&inputs)?;
        print_timing("array_physics_only", repeat, ARRAY_ITERATIONS, physics);

        let write = time_array_write(&inputs, &plan, &mut dense)?;
        print_timing("array_write_outputs", repeat, ARRAY_ITERATIONS, write);

        let combined = time_array_combined(&inputs, &plan, &mut dense)?;
        print_timing(
            "array_combined_hot_loop",
            repeat,
            ARRAY_ITERATIONS,
            combined,
        );

        let boundary = time_boundary_materialize(&registry, &dense);
        print_timing(
            "boundary_materialize_once",
            repeat,
            BOUNDARY_ITERATIONS,
            boundary,
        );
    }

    let working_set_bytes = dense_working_set_bytes(&dense);
    println!("working_set_bytes\t{working_set_bytes}");
    println!(
        "output_slot_count\tstate={}\tflux={}\ttotal={}",
        production.writeback.state_updates.len(),
        production.writeback.flux_updates.len(),
        plan.slots.len()
    );
    println!(
        "budget_us_per_ofe_day\tlegacy={LEGACY_US_PER_OFE_DAY:.3}\tle10x=386.000\tle5x=193.000"
    );
    println!("legacy_h2637_no_ui_seconds\t{LEGACY_H2637_NO_UI_SECONDS:.3}");
    println!("h2637_ofe_days\t{H2637_OFE_DAYS:.0}");

    Ok(())
}

fn state_symbol(symbol: HillslopeProductionStateSymbol) -> BoundarySymbol {
    BoundarySymbol::from(symbol)
}

fn insert_state(
    state: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: impl Into<BoundarySymbol>,
    value: f64,
) {
    state.insert(symbol.into(), BoundaryValue::scalar(value));
}

fn registry_surfaces(
    state: &BTreeMap<BoundarySymbol, BoundaryValue>,
    flux: &BTreeMap<BoundarySymbol, BoundaryValue>,
    payload: &KernelWritebackPayload,
) -> (
    BTreeMap<BoundarySymbol, BoundaryValue>,
    BTreeMap<BoundarySymbol, BoundaryValue>,
) {
    let mut registry_state = state.clone();
    let mut registry_flux = flux.clone();
    for field in &payload.state_updates {
        registry_state
            .entry(field.symbol.clone())
            .or_insert(field.value);
    }
    for field in &payload.flux_updates {
        registry_flux
            .entry(field.symbol.clone())
            .or_insert(field.value);
    }
    (registry_state, registry_flux)
}

fn seed_logical_surfaces(
    inputs: &RunoffInputs,
    state: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    flux: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
) {
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb12RainfallInput),
        inputs.rainfall_input_m,
    );
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb12RunonInput),
        inputs.runon_input_m,
    );
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb12RunoffClosureTolerance),
        inputs.closure_tolerance_m,
    );
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb14SoilConductivity),
        inputs.soil_conductivity_m_s,
    );
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb14SoilLayerDepth),
        inputs.soil_layer_depth_m,
    );
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb14SoilThetaResidual),
        inputs.theta_residual,
    );
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb14SoilThetaFieldCapacity),
        inputs.theta_field_capacity,
    );
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb14HyetographNinten),
        2.0,
    );
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb14HyetographNbrkpt),
        2.0,
    );
    insert_state(state, "timem_0001", inputs.timem_s[0]);
    insert_state(state, "timem_0002", inputs.timem_s[1]);
    insert_state(state, "intsty_0001", inputs.intensity_m_s[0]);
    insert_state(state, "intsty_0002", inputs.intensity_m_s[1]);
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb15PlantCancov),
        inputs.cancov,
    );
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb15PlantLai),
        inputs.lai,
    );
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb15PlantVdmt),
        inputs.vdmt,
    );
    insert_state(
        state,
        state_symbol(HillslopeProductionStateSymbol::Wb12DepressionStorageDelta),
        inputs.depression_storage_delta_m,
    );
    insert_state(state, "wb20_forward_solver_lane_enabled", 1.0);
    flux.insert(
        BoundarySymbol::from("wb12_runoff_carryover"),
        BoundaryValue::scalar(inputs.runon_input_m),
    );
}

fn run_production(
    state: &BTreeMap<BoundarySymbol, BoundaryValue>,
    flux: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<openwepp_kernel_contract::KernelRunResponse, PrototypeError> {
    let request = HillslopeKernelRequest::with_phase_context(
        "runoff_reconciliation",
        HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
        HillslopeConsumerAdapter::Runoff,
        None,
        state,
        flux,
    );
    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);
    if response.status.classification() != StatusClassification::Nominal {
        return Err(PrototypeError::Status(
            response.status.message_id().to_owned(),
        ));
    }
    Ok(response)
}

fn array_runoff_physics(inputs: RunoffInputs) -> Result<RunoffOutputs, PrototypeError> {
    require_range("rainfall_input_m", inputs.rainfall_input_m, Some(0.0), None)?;
    require_range(
        "closure_tolerance_m",
        inputs.closure_tolerance_m,
        Some(0.0),
        None,
    )?;
    require_range(
        "soil_conductivity_m_s",
        inputs.soil_conductivity_m_s,
        Some(0.0),
        None,
    )?;
    require_range(
        "soil_layer_depth_m",
        inputs.soil_layer_depth_m,
        Some(0.0),
        None,
    )?;
    require_range("theta_residual", inputs.theta_residual, Some(0.0), None)?;
    require_range(
        "theta_field_capacity",
        inputs.theta_field_capacity,
        Some(0.0),
        None,
    )?;

    let moisture_deficit = inputs.theta_field_capacity - inputs.theta_residual;
    if moisture_deficit < -ZERO_THRESHOLD {
        return Err(PrototypeError::Domain(
            "theta_field_capacity below theta_residual",
        ));
    }
    let matric_potential_m = inputs.soil_layer_depth_m * moisture_deficit.max(0.0);
    require_range("matric_potential_m", matric_potential_m, Some(0.0), None)?;

    let interval_duration_s = inputs.timem_s[1] - inputs.timem_s[0];
    if interval_duration_s <= ZERO_THRESHOLD {
        return Err(PrototypeError::Domain(
            "hyetograph interval duration must be positive",
        ));
    }
    let hyetograph_rainfall = inputs.intensity_m_s[0] * interval_duration_s;
    if (inputs.rainfall_input_m - hyetograph_rainfall).abs()
        > inputs.closure_tolerance_m + ZERO_THRESHOLD
    {
        return Err(PrototypeError::Domain(
            "rainfall input does not match hyetograph total",
        ));
    }

    let interception_m = compute_canopy_interception_depth(
        inputs.hyetograph_rainfall(),
        inputs.cancov,
        inputs.lai,
        inputs.vdmt,
    )?;
    let (liquid_after_interception_m, rainfall_scale) = resolve_interception_rainfall_scale(
        hyetograph_rainfall,
        hyetograph_rainfall,
        interception_m,
    )?;
    let scaled_rate = inputs.intensity_m_s[0] * rainfall_scale;
    let infiltration_m = compute_interval_infiltration_depth(
        inputs.soil_conductivity_m_s,
        matric_potential_m,
        0.0,
        scaled_rate,
        interval_duration_s,
    )?
    .min(liquid_after_interception_m);
    if infiltration_m > liquid_after_interception_m + ZERO_THRESHOLD {
        return Err(PrototypeError::Domain("infiltration exceeds liquid input"));
    }

    let q_runoff_m = normalize_non_negative(
        liquid_after_interception_m + inputs.runon_input_m
            - infiltration_m
            - inputs.depression_storage_delta_m,
    );
    require_range("Q", q_runoff_m, Some(0.0), None)?;
    let closure_delta_m = liquid_after_interception_m + inputs.runon_input_m
        - infiltration_m
        - inputs.depression_storage_delta_m
        - q_runoff_m;
    if closure_delta_m.abs() > inputs.closure_tolerance_m + ZERO_THRESHOLD {
        return Err(PrototypeError::Domain("runoff closure exceeded tolerance"));
    }

    Ok(RunoffOutputs {
        infiltration_m,
        q_runoff_m,
        soil_conductivity_m_s: inputs.soil_conductivity_m_s,
        effective_conductivity_m_s: inputs.soil_conductivity_m_s,
        matric_potential_m,
        interception_m,
        runon_input_m: inputs.runon_input_m,
        closure_delta_m,
        snow_post_winter_rain_m: hyetograph_rainfall,
    })
}

impl RunoffInputs {
    const fn hyetograph_rainfall(self) -> f64 {
        self.rainfall_input_m
    }
}

fn compute_canopy_interception_depth(
    rainfall_m: f64,
    cancov: f64,
    lai: f64,
    vdmt: f64,
) -> Result<f64, PrototypeError> {
    require_range("cancov", cancov, Some(0.0), Some(WB15_CANCOV_MAX))?;
    require_range("lai", lai, Some(0.0), None)?;
    require_range("vdmt", vdmt, Some(0.0), None)?;
    if cancov <= ZERO_THRESHOLD || lai <= ZERO_THRESHOLD {
        return Ok(0.0);
    }
    let biomass_kg_ha = vdmt * WB15_BIOMASS_TO_KG_HA;
    require_range("biomass_kg_ha", biomass_kg_ha, Some(0.0), None)?;
    let capped_biomass = biomass_kg_ha.min(WB15_INTERCEPT_BIOMASS_MAX_KG_HA);
    let potential_interception = cancov
        * ((WB15_INTERCEPT_LINEAR_COEFF * capped_biomass
            - WB15_INTERCEPT_QUADRATIC_COEFF * capped_biomass.powi(2))
            / WB15_INTERCEPT_MM_TO_M);
    require_range(
        "potential_interception",
        potential_interception,
        Some(0.0),
        None,
    )?;
    let interception = normalize_non_negative(potential_interception.min(rainfall_m));
    require_range("interception", interception, Some(0.0), Some(rainfall_m))?;
    Ok(interception)
}

fn resolve_interception_rainfall_scale(
    hyetograph_rainfall: f64,
    interception_rainfall_input: f64,
    interception: f64,
) -> Result<(f64, f64), PrototypeError> {
    let liquid_after_interception_raw = interception_rainfall_input - interception;
    require_range(
        "liquid_after_interception_raw",
        liquid_after_interception_raw,
        Some(0.0),
        Some(interception_rainfall_input),
    )?;
    let liquid_after_interception = normalize_non_negative(liquid_after_interception_raw);
    if hyetograph_rainfall <= ZERO_THRESHOLD {
        return Ok((liquid_after_interception, 0.0));
    }
    let rainfall_scale = liquid_after_interception / hyetograph_rainfall;
    require_range("rainfall_scale", rainfall_scale, Some(0.0), None)?;
    Ok((liquid_after_interception, rainfall_scale))
}

fn compute_interval_infiltration_depth(
    conductivity: f64,
    matric_potential: f64,
    cumulative_infiltration_start: f64,
    rainfall_rate: f64,
    interval_duration: f64,
) -> Result<f64, PrototypeError> {
    if interval_duration <= 0.0 {
        return Err(PrototypeError::Domain("interval duration must be positive"));
    }
    let interval_rainfall_depth = rainfall_rate * interval_duration;
    require_range(
        "interval_rainfall_depth",
        interval_rainfall_depth,
        Some(0.0),
        None,
    )?;
    if rainfall_rate <= conductivity + ZERO_THRESHOLD {
        return Ok(interval_rainfall_depth.max(0.0));
    }
    let interval_infiltration = if matric_potential <= ZERO_THRESHOLD {
        conductivity * interval_duration
    } else {
        let denominator = rainfall_rate - conductivity;
        if denominator <= ZERO_THRESHOLD {
            interval_rainfall_depth
        } else {
            let ponding_threshold = (conductivity * matric_potential) / denominator;
            require_range("ponding_threshold", ponding_threshold, Some(0.0), None)?;
            if cumulative_infiltration_start >= ponding_threshold - ZERO_THRESHOLD {
                solve_ponded_cumulative_infiltration(
                    conductivity,
                    matric_potential,
                    cumulative_infiltration_start,
                    interval_duration,
                )? - cumulative_infiltration_start
            } else {
                let infiltration_to_ponding =
                    (ponding_threshold - cumulative_infiltration_start).max(0.0);
                let time_to_ponding = infiltration_to_ponding / rainfall_rate;
                if time_to_ponding >= interval_duration - ZERO_THRESHOLD {
                    interval_rainfall_depth
                } else {
                    let ponded_duration = interval_duration - time_to_ponding;
                    let cumulative_end = solve_ponded_cumulative_infiltration(
                        conductivity,
                        matric_potential,
                        ponding_threshold,
                        ponded_duration,
                    )?;
                    infiltration_to_ponding + (cumulative_end - ponding_threshold)
                }
            }
        }
    };
    if !interval_infiltration.is_finite()
        || interval_infiltration < -ZERO_THRESHOLD
        || interval_infiltration
            > interval_rainfall_depth + WB14_INTERVAL_INFILTRATION_ROUNDOFF_TOLERANCE_M
    {
        return Err(PrototypeError::Domain(
            "interval infiltration outside bounds",
        ));
    }
    Ok(interval_infiltration.max(0.0).min(interval_rainfall_depth))
}

fn solve_ponded_cumulative_infiltration(
    conductivity: f64,
    matric_potential: f64,
    cumulative_start: f64,
    duration: f64,
) -> Result<f64, PrototypeError> {
    if duration <= ZERO_THRESHOLD {
        return Ok(cumulative_start);
    }
    if matric_potential <= ZERO_THRESHOLD {
        return Ok(cumulative_start + conductivity * duration);
    }
    let rhs = conductivity * duration;
    let start_plus_matric = cumulative_start + matric_potential;
    if start_plus_matric <= 0.0 {
        return Err(PrototypeError::Domain("invalid ponded infiltration start"));
    }
    let target = cumulative_start - matric_potential * start_plus_matric.ln() + rhs;
    let mut lower = cumulative_start;
    let mut upper = cumulative_start + rhs + matric_potential + 1.0;
    for _ in 0..128 {
        let upper_plus_matric = upper + matric_potential;
        if upper_plus_matric <= 0.0 {
            upper *= 2.0;
            continue;
        }
        let residual = upper - matric_potential * upper_plus_matric.ln() - target;
        if residual >= 0.0 {
            break;
        }
        upper = cumulative_start + (upper - cumulative_start) * 2.0 + 1.0;
    }
    for _ in 0..128 {
        let midpoint = 0.5 * (lower + upper);
        let midpoint_plus_matric = midpoint + matric_potential;
        if midpoint_plus_matric <= 0.0 {
            lower = midpoint;
            continue;
        }
        let residual = midpoint - matric_potential * midpoint_plus_matric.ln() - target;
        if residual >= 0.0 {
            upper = midpoint;
        } else {
            lower = midpoint;
        }
        let tolerance = (upper.abs().max(1.0)) * 1.0e-12;
        if (upper - lower) <= tolerance {
            break;
        }
    }
    let solution = 0.5 * (lower + upper);
    if !solution.is_finite() || solution < cumulative_start - ZERO_THRESHOLD {
        return Err(PrototypeError::Domain("ponded infiltration solve failed"));
    }
    Ok(solution)
}

fn require_range(
    name: &'static str,
    value: f64,
    minimum: Option<f64>,
    maximum: Option<f64>,
) -> Result<(), PrototypeError> {
    if !value.is_finite() {
        return Err(PrototypeError::Domain(name));
    }
    if let Some(minimum_value) = minimum
        && value < minimum_value - ZERO_THRESHOLD
    {
        return Err(PrototypeError::Domain(name));
    }
    if let Some(maximum_value) = maximum
        && value > maximum_value + ZERO_THRESHOLD
    {
        return Err(PrototypeError::Domain(name));
    }
    Ok(())
}

fn normalize_non_negative(value: f64) -> f64 {
    if (-ZERO_THRESHOLD..0.0).contains(&value) {
        0.0
    } else {
        value
    }
}

impl OutputPlan {
    fn from_production_payload(
        registry: &SymbolRegistry,
        payload: &KernelWritebackPayload,
    ) -> Result<Self, Box<dyn Error>> {
        let mut slots =
            Vec::with_capacity(payload.state_updates.len() + payload.flux_updates.len());
        for field in &payload.state_updates {
            slots.push(OutputSlot {
                kind: SurfaceKind::State,
                id: registry.id_of(&field.symbol)?,
                source: classify_source(&field.symbol, SurfaceKind::State, field.value.as_f64()),
            });
        }
        for field in &payload.flux_updates {
            slots.push(OutputSlot {
                kind: SurfaceKind::Flux,
                id: registry.id_of(&field.symbol)?,
                source: classify_source(&field.symbol, SurfaceKind::Flux, field.value.as_f64()),
            });
        }
        Ok(Self { slots })
    }
}

fn classify_source(
    symbol: &BoundarySymbol,
    kind: SurfaceKind,
    production_value: f64,
) -> OutputSource {
    let name = symbol.as_str();
    match (kind, name) {
        (SurfaceKind::State, "wb12_infiltration") => OutputSource::Infiltration,
        (SurfaceKind::State, "wb12_runoff_reconciled") => OutputSource::RunoffReconciled,
        (SurfaceKind::State, "wb14_soil_conductivity_m_s") => OutputSource::SoilConductivity,
        (SurfaceKind::State, "wb14_effective_conductivity_m_s") => {
            OutputSource::EffectiveConductivity
        }
        (SurfaceKind::State, "wb14_matric_potential_m") => OutputSource::MatricPotential,
        (SurfaceKind::Flux, "I") => OutputSource::Interception,
        (SurfaceKind::Flux, "Irr") => OutputSource::DailyIrrigation,
        (SurfaceKind::Flux, "Q") => OutputSource::RunoffQ,
        (SurfaceKind::Flux, "wb12_runoff_carryover") => OutputSource::RunoffCarryover,
        (SurfaceKind::Flux, "wb12_runoff_closure_delta") => OutputSource::ClosureDelta,
        (SurfaceKind::Flux, "S") => OutputSource::SnowCoupling,
        (SurfaceKind::Flux, "snow.routed_melt_m") => OutputSource::SnowRoutedMelt,
        (SurfaceKind::Flux, "snow.post_winter_rain_m") => OutputSource::SnowPostWinterRain,
        _ => {
            if production_value.abs() > ZERO_THRESHOLD {
                eprintln!(
                    "warning: treating non-zero unclassified output {name}={production_value:.17e} as zero"
                );
            }
            OutputSource::Zero
        }
    }
}

fn array_write_outputs(plan: &OutputPlan, surface: &mut DenseSurface, outputs: &RunoffOutputs) {
    for slot in &plan.slots {
        let value = BoundaryValue::scalar(slot.source.value(outputs));
        match slot.kind {
            SurfaceKind::State => surface.state[slot.id.as_usize()] = Some(value),
            SurfaceKind::Flux => surface.flux[slot.id.as_usize()] = Some(value),
        }
    }
}

fn validate_array_identity(
    inputs: &RunoffInputs,
    production: &KernelWritebackPayload,
    plan: &OutputPlan,
    dense: &mut DenseSurface,
) -> Result<(), PrototypeError> {
    let outputs = array_runoff_physics(*inputs)?;
    array_write_outputs(plan, dense, &outputs);
    for field in production
        .state_updates
        .iter()
        .chain(production.flux_updates.iter())
    {
        let source = classify_source(
            &field.symbol,
            output_kind(production, &field.symbol),
            field.value.as_f64(),
        );
        let observed = source.value(&outputs);
        let expected = field.value.as_f64();
        if expected.to_bits() != observed.to_bits() {
            return Err(PrototypeError::OutputMismatch {
                symbol: field.symbol.as_str().to_owned(),
                expected,
                observed,
            });
        }
    }
    Ok(())
}

fn output_kind(payload: &KernelWritebackPayload, symbol: &BoundarySymbol) -> SurfaceKind {
    if payload
        .state_updates
        .iter()
        .any(|field| field.symbol == *symbol)
    {
        SurfaceKind::State
    } else {
        SurfaceKind::Flux
    }
}

fn time_logical_production(
    state: &BTreeMap<BoundarySymbol, BoundaryValue>,
    flux: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<Duration, PrototypeError> {
    let start = Instant::now();
    let mut checksum = 0.0;
    for _ in 0..LOGICAL_ITERATIONS {
        let response = run_production(state, flux)?;
        checksum += response.writeback.flux_updates[2].value.as_f64();
    }
    black_box(checksum);
    Ok(start.elapsed())
}

fn time_array_physics(inputs: &RunoffInputs) -> Result<Duration, PrototypeError> {
    let start = Instant::now();
    let mut checksum = 0.0;
    for _ in 0..ARRAY_ITERATIONS {
        let outputs = array_runoff_physics(*inputs)?;
        checksum += outputs.q_runoff_m;
    }
    black_box(checksum);
    Ok(start.elapsed())
}

fn time_array_write(
    inputs: &RunoffInputs,
    plan: &OutputPlan,
    dense: &mut DenseSurface,
) -> Result<Duration, PrototypeError> {
    let outputs = array_runoff_physics(*inputs)?;
    let start = Instant::now();
    let mut checksum = 0.0;
    for _ in 0..ARRAY_ITERATIONS {
        array_write_outputs(plan, dense, &outputs);
        checksum += dense.flux[plan.slots[0].id.as_usize()].map_or(0.0, BoundaryValue::as_f64);
    }
    black_box(checksum);
    Ok(start.elapsed())
}

fn time_array_combined(
    inputs: &RunoffInputs,
    plan: &OutputPlan,
    dense: &mut DenseSurface,
) -> Result<Duration, PrototypeError> {
    let start = Instant::now();
    let mut checksum = 0.0;
    for _ in 0..ARRAY_ITERATIONS {
        let outputs = array_runoff_physics(*inputs)?;
        array_write_outputs(plan, dense, &outputs);
        checksum += outputs.q_runoff_m;
    }
    black_box(checksum);
    Ok(start.elapsed())
}

fn time_array_hot_loop(
    inputs: &RunoffInputs,
    plan: &OutputPlan,
    dense: &mut DenseSurface,
    iterations: usize,
) -> f64 {
    let mut checksum = 0.0;
    for _ in 0..iterations {
        let outputs = array_runoff_physics(*inputs).expect("array benchmark input is valid");
        array_write_outputs(plan, dense, &outputs);
        checksum += outputs.q_runoff_m;
    }
    black_box(checksum)
}

fn time_boundary_materialize(registry: &SymbolRegistry, dense: &DenseSurface) -> Duration {
    let start = Instant::now();
    let mut checksum = 0usize;
    for _ in 0..BOUNDARY_ITERATIONS {
        let (state, flux) = dense.export_maps(registry);
        checksum += state.len() + flux.len();
    }
    black_box(checksum);
    start.elapsed()
}

fn export_one(
    registry: &SymbolRegistry,
    slots: &[Option<BoundaryValue>],
) -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut output = BTreeMap::new();
    for (id, symbol) in registry.iter() {
        if let Some(value) = slots[id.as_usize()] {
            output.insert(symbol.clone(), value);
        }
    }
    output
}

fn dense_working_set_bytes(dense: &DenseSurface) -> usize {
    dense.state.len() * std::mem::size_of::<Option<BoundaryValue>>()
        + dense.flux.len() * std::mem::size_of::<Option<BoundaryValue>>()
}

fn print_timing(metric: &str, repeat: usize, iterations: usize, duration: Duration) {
    let seconds = duration.as_secs_f64();
    let ns_per_iter = seconds * 1.0e9 / iterations as f64;
    let us_per_ofe_day = ns_per_iter / 1.0e3;
    let ratio = us_per_ofe_day / LEGACY_US_PER_OFE_DAY;
    let projected_h2637_seconds = us_per_ofe_day * H2637_OFE_DAYS / 1.0e6;
    println!(
        "{metric}\t{repeat}\t{iterations}\t{seconds:.9}\t{ns_per_iter:.3}\t{us_per_ofe_day:.6}\t{ratio:.6}\t{projected_h2637_seconds:.6}"
    );
}
