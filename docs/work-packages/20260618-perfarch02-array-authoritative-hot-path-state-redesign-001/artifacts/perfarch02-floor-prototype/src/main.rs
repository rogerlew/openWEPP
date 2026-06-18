use std::collections::BTreeMap;
use std::error::Error;
use std::hint::black_box;
use std::time::{Duration, Instant};

use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, KernelWritebackPayload, SymbolId, SymbolRegistry,
    WRITEBACK_REJECT_DOMAIN_MESSAGE_ID, WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID, WritebackField,
    apply_kernel_writeback, evaluate_kernel_writeback,
};
use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, SimulationStatus};

const STATE_SYMBOL_COUNT: usize = 3_072;
const FLUX_SYMBOL_COUNT: usize = 1_024;
const STATE_UPDATE_COUNT: usize = 96;
const FLUX_UPDATE_COUNT: usize = 64;
const ITERATIONS: usize = 200_000;
const WARMUPS: usize = 5_000;
const REPEATS: usize = 5;
const H2637_OFE_DAYS: f64 = 235_961.0;
const LEGACY_H2637_NO_UI_SECONDS: f64 = 9.12;

#[derive(Clone, Copy)]
enum SurfaceKind {
    State,
    Flux,
}

#[derive(Clone, Copy)]
struct IndexedField {
    id: SymbolId,
    value: BoundaryValue,
    minimum: Option<f64>,
    maximum: Option<f64>,
}

struct IndexedPayload {
    state_updates: Vec<IndexedField>,
    flux_updates: Vec<IndexedField>,
}

struct IndexedDecision {
    accepted: bool,
    message_id: String,
    lazy_subjects: Vec<String>,
}

struct DenseSurfacePair {
    state: Vec<Option<BoundaryValue>>,
    flux: Vec<Option<BoundaryValue>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let (base_state, base_flux) = build_surfaces();
    let registry = SymbolRegistry::from_surfaces(&base_state, &base_flux)?;
    let payload = build_payload();
    let indexed_payload = resolve_payload(&registry, &payload)?;
    let mut dense = DenseSurfacePair::from_maps(&registry, &base_state, &base_flux)?;

    validate_identity(
        &registry,
        &payload,
        &indexed_payload,
        &base_state,
        &base_flux,
    )?;
    validate_failure_path(&registry)?;

    for _ in 0..WARMUPS {
        let decision = evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &payload)?;
        let mut logical_state = base_state.clone();
        let mut logical_flux = base_flux.clone();
        let applied = apply_kernel_writeback(
            SimulationPhase::HillslopeKernel,
            &decision,
            &payload,
            &mut logical_state,
            &mut logical_flux,
        )?;
        black_box(applied);

        let decision = evaluate_indexed_payload(
            &registry,
            SimulationPhase::HillslopeKernel,
            &indexed_payload,
        )?;
        dense.apply(&indexed_payload);
        black_box(decision);
    }

    println!("metric\trepeat\tseconds\tseconds_per_iter\tprojected_h2637_seconds\tprojected_ratio");
    for repeat in 1..=REPEATS {
        let logical = time_logical_current(&payload, &base_state, &base_flux)?;
        print_measurement("logical_current", repeat, logical);

        let indexed = time_indexed_candidate(&registry, &indexed_payload, &base_state, &base_flux)?;
        print_measurement("array_authoritative", repeat, indexed);
    }

    let export_duration = time_export_once(&registry, &dense)?;
    println!(
        "export_once\t1\t{:.9}\t{:.9}\t{:.9}\t{:.6}",
        export_duration.as_secs_f64(),
        export_duration.as_secs_f64(),
        export_duration.as_secs_f64(),
        export_duration.as_secs_f64() / LEGACY_H2637_NO_UI_SECONDS
    );

    Ok(())
}

fn print_measurement(metric: &str, repeat: usize, duration: Duration) {
    let seconds = duration.as_secs_f64();
    let seconds_per_iter = seconds / ITERATIONS as f64;
    let projected_h2637_seconds = seconds_per_iter * H2637_OFE_DAYS;
    let projected_ratio = projected_h2637_seconds / LEGACY_H2637_NO_UI_SECONDS;
    println!(
        "{metric}\t{repeat}\t{seconds:.9}\t{seconds_per_iter:.12}\t{projected_h2637_seconds:.9}\t{projected_ratio:.6}"
    );
}

fn build_surfaces() -> (
    BTreeMap<BoundarySymbol, BoundaryValue>,
    BTreeMap<BoundarySymbol, BoundaryValue>,
) {
    let mut state = BTreeMap::new();
    let mut flux = BTreeMap::new();
    for index in 1..=STATE_SYMBOL_COUNT {
        state.insert(
            BoundarySymbol::from(format!("state_{index:04}")),
            BoundaryValue::scalar(index as f64 * 0.01),
        );
    }
    for index in 1..=FLUX_SYMBOL_COUNT {
        flux.insert(
            BoundarySymbol::from(format!("flux_{index:04}")),
            BoundaryValue::scalar(index as f64 * 0.001),
        );
    }
    (state, flux)
}

fn build_payload() -> KernelWritebackPayload {
    let mut state_updates = Vec::with_capacity(STATE_UPDATE_COUNT);
    let mut flux_updates = Vec::with_capacity(FLUX_UPDATE_COUNT);

    for index in 0..STATE_UPDATE_COUNT {
        let symbol_index = 1 + ((index * 29) % STATE_SYMBOL_COUNT);
        let value = 0.5 + index as f64 * 0.125;
        state_updates.push(WritebackField::bounded(
            format!("state_{symbol_index:04}"),
            BoundaryValue::scalar(value),
            Some(0.0),
            Some(20.0),
        ));
    }

    for index in 0..FLUX_UPDATE_COUNT {
        let symbol_index = 1 + ((index * 17) % FLUX_SYMBOL_COUNT);
        let value = 0.25 + index as f64 * 0.0625;
        flux_updates.push(WritebackField::bounded(
            format!("flux_{symbol_index:04}"),
            BoundaryValue::scalar(value),
            Some(0.0),
            Some(10.0),
        ));
    }

    KernelWritebackPayload::with_updates(state_updates, flux_updates)
}

fn resolve_payload(
    registry: &SymbolRegistry,
    payload: &KernelWritebackPayload,
) -> Result<IndexedPayload, Box<dyn Error>> {
    let mut state_updates = resolve_fields(registry, &payload.state_updates)?;
    let mut flux_updates = resolve_fields(registry, &payload.flux_updates)?;
    state_updates.sort_by_key(|field| field.id);
    flux_updates.sort_by_key(|field| field.id);
    Ok(IndexedPayload {
        state_updates,
        flux_updates,
    })
}

fn resolve_fields(
    registry: &SymbolRegistry,
    fields: &[WritebackField],
) -> Result<Vec<IndexedField>, Box<dyn Error>> {
    let mut indexed = Vec::with_capacity(fields.len());
    for field in fields {
        indexed.push(IndexedField {
            id: registry.id_of(&field.symbol)?,
            value: field.value,
            minimum: field.minimum,
            maximum: field.maximum,
        });
    }
    Ok(indexed)
}

impl DenseSurfacePair {
    fn from_maps(
        registry: &SymbolRegistry,
        state: &BTreeMap<BoundarySymbol, BoundaryValue>,
        flux: &BTreeMap<BoundarySymbol, BoundaryValue>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut dense = Self {
            state: vec![None; registry.len()],
            flux: vec![None; registry.len()],
        };
        for (symbol, value) in state {
            dense.state[registry.id_of(symbol)?.as_usize()] = Some(*value);
        }
        for (symbol, value) in flux {
            dense.flux[registry.id_of(symbol)?.as_usize()] = Some(*value);
        }
        Ok(dense)
    }

    fn apply(&mut self, payload: &IndexedPayload) {
        for field in &payload.state_updates {
            self.state[field.id.as_usize()] = Some(field.value);
        }
        for field in &payload.flux_updates {
            self.flux[field.id.as_usize()] = Some(field.value);
        }
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

fn validate_identity(
    registry: &SymbolRegistry,
    payload: &KernelWritebackPayload,
    indexed_payload: &IndexedPayload,
    base_state: &BTreeMap<BoundarySymbol, BoundaryValue>,
    base_flux: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<(), Box<dyn Error>> {
    let decision = evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, payload)?;
    let mut logical_state = base_state.clone();
    let mut logical_flux = base_flux.clone();
    apply_kernel_writeback(
        SimulationPhase::HillslopeKernel,
        &decision,
        payload,
        &mut logical_state,
        &mut logical_flux,
    )?;

    let indexed_decision =
        evaluate_indexed_payload(registry, SimulationPhase::HillslopeKernel, indexed_payload)?;
    if !indexed_decision.accepted {
        return Err("indexed prototype rejected the success-path payload".into());
    }

    let mut dense = DenseSurfacePair::from_maps(registry, base_state, base_flux)?;
    dense.apply(indexed_payload);
    let (indexed_state, indexed_flux) = dense.export_maps(registry);
    if indexed_state != logical_state || indexed_flux != logical_flux {
        return Err("array-authoritative export did not match logical writeback".into());
    }
    Ok(())
}

fn validate_failure_path(registry: &SymbolRegistry) -> Result<(), Box<dyn Error>> {
    let payload = KernelWritebackPayload::with_updates(
        vec![WritebackField::bounded(
            "state_0001",
            BoundaryValue::scalar(f64::NAN),
            Some(0.0),
            Some(1.0),
        )],
        vec![WritebackField::bounded(
            "flux_0001",
            BoundaryValue::scalar(11.0),
            Some(0.0),
            Some(10.0),
        )],
    );
    let indexed_payload = resolve_payload(registry, &payload)?;
    let logical = evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, &payload)?;
    let indexed =
        evaluate_indexed_payload(registry, SimulationPhase::HillslopeKernel, &indexed_payload)?;

    if indexed.accepted {
        return Err("indexed prototype accepted an invalid payload".into());
    }
    if logical.status.message_id() != indexed.message_id.as_str() {
        return Err("indexed prototype emitted a different failure message id".into());
    }
    if indexed.lazy_subjects.is_empty() {
        return Err("indexed prototype did not resolve lazy failure subjects".into());
    }
    Ok(())
}

fn evaluate_indexed_payload(
    registry: &SymbolRegistry,
    phase: SimulationPhase,
    payload: &IndexedPayload,
) -> Result<IndexedDecision, Box<dyn Error>> {
    let mut non_finite = Vec::new();
    let mut domain = Vec::new();
    collect_indexed_violations(
        registry,
        SurfaceKind::State,
        &payload.state_updates,
        &mut non_finite,
        &mut domain,
    );
    collect_indexed_violations(
        registry,
        SurfaceKind::Flux,
        &payload.flux_updates,
        &mut non_finite,
        &mut domain,
    );

    if !non_finite.is_empty() {
        let status =
            SimulationStatus::non_finite_failure(phase, WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID)?;
        return Ok(IndexedDecision {
            accepted: false,
            message_id: status.message_id().to_owned(),
            lazy_subjects: non_finite,
        });
    }

    if !domain.is_empty() {
        let status = SimulationStatus::domain_failure(
            phase,
            BoundaryClass::DomainViolation,
            WRITEBACK_REJECT_DOMAIN_MESSAGE_ID,
        )?;
        return Ok(IndexedDecision {
            accepted: false,
            message_id: status.message_id().to_owned(),
            lazy_subjects: domain,
        });
    }

    let status =
        SimulationStatus::ok(phase, openwepp_kernel_contract::WRITEBACK_ACCEPT_MESSAGE_ID)?;
    Ok(IndexedDecision {
        accepted: true,
        message_id: status.message_id().to_owned(),
        lazy_subjects: Vec::new(),
    })
}

fn collect_indexed_violations(
    registry: &SymbolRegistry,
    surface: SurfaceKind,
    fields: &[IndexedField],
    non_finite: &mut Vec<String>,
    domain: &mut Vec<String>,
) {
    for field in fields {
        let value = field.value.as_f64();
        if !value.is_finite() {
            non_finite.push(lazy_subject(registry, surface, field));
            continue;
        }
        if !field_satisfies_domain(field, value) {
            domain.push(lazy_subject(registry, surface, field));
        }
    }
}

fn lazy_subject(registry: &SymbolRegistry, surface: SurfaceKind, field: &IndexedField) -> String {
    let scope = match surface {
        SurfaceKind::State => "state",
        SurfaceKind::Flux => "flux",
    };
    let symbol = registry
        .symbol(field.id)
        .map(BoundarySymbol::as_str)
        .unwrap_or("<unknown>");
    format!("{scope}:{symbol}[{}]", field.value.unit_label())
}

fn field_satisfies_domain(field: &IndexedField, value: f64) -> bool {
    match (field.minimum, field.maximum) {
        (Some(minimum), Some(maximum)) => {
            minimum <= maximum && (minimum..=maximum).contains(&value)
        }
        (Some(minimum), None) => value >= minimum,
        (None, Some(maximum)) => value <= maximum,
        (None, None) => true,
    }
}

fn time_logical_current(
    payload: &KernelWritebackPayload,
    base_state: &BTreeMap<BoundarySymbol, BoundaryValue>,
    base_flux: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<Duration, Box<dyn Error>> {
    let mut state = base_state.clone();
    let mut flux = base_flux.clone();
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let decision = evaluate_kernel_writeback(SimulationPhase::HillslopeKernel, payload)?;
        let applied = apply_kernel_writeback(
            SimulationPhase::HillslopeKernel,
            &decision,
            payload,
            &mut state,
            &mut flux,
        )?;
        black_box(applied);
    }
    Ok(start.elapsed())
}

fn time_indexed_candidate(
    registry: &SymbolRegistry,
    payload: &IndexedPayload,
    base_state: &BTreeMap<BoundarySymbol, BoundaryValue>,
    base_flux: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<Duration, Box<dyn Error>> {
    let mut dense = DenseSurfacePair::from_maps(registry, base_state, base_flux)?;
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let decision =
            evaluate_indexed_payload(registry, SimulationPhase::HillslopeKernel, payload)?;
        dense.apply(payload);
        black_box(decision);
    }
    Ok(start.elapsed())
}

fn time_export_once(
    registry: &SymbolRegistry,
    dense: &DenseSurfacePair,
) -> Result<Duration, Box<dyn Error>> {
    let start = Instant::now();
    let exported = dense.export_maps(registry);
    black_box(exported);
    Ok(start.elapsed())
}
