use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Instant;

use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, IndexedKernelWritebackPayload, IndexedWritebackField,
    IndexedWritebackSurface, SymbolRegistry, WritebackDecisionOutcome,
    apply_indexed_kernel_writeback, evaluate_indexed_kernel_writeback,
};
use openwepp_sim_contract::status::SimulationPhase;

const STATE_UPDATE_COUNT: usize = 543;
const FLUX_UPDATE_COUNT: usize = 8;
const DEFAULT_REPETITIONS: usize = 20_000;
const H2637_OFE_DAYS: f64 = 235_961.0;

fn main() {
    let repetitions = std::env::args()
        .nth(1)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(DEFAULT_REPETITIONS);
    let fixture = Fixture::new();

    println!("metric\trepetitions\telapsed_s\tus_per_payload\tns_per_field\tprojected_h2637_s");
    report_metric("evaluate_indexed_payload", repetitions, || {
        let decision =
            evaluate_indexed_kernel_writeback(SimulationPhase::HillslopeKernel, &fixture.payload)
                .expect("indexed writeback decision should construct");
        assert_eq!(decision.outcome, WritebackDecisionOutcome::Accept);
        black_box(decision);
    });

    let decision =
        evaluate_indexed_kernel_writeback(SimulationPhase::HillslopeKernel, &fixture.payload)
            .expect("indexed writeback decision should construct");
    let mut apply_state_surface = fixture.state_surface.clone();
    let mut apply_flux_surface = fixture.flux_surface.clone();
    let mut apply_indexed_surface = fixture.indexed_surface.clone();
    report_metric("apply_indexed_payload", repetitions, || {
        let apply_result = apply_indexed_kernel_writeback(
            SimulationPhase::HillslopeKernel,
            &decision,
            &fixture.payload,
            &mut apply_indexed_surface,
            &fixture.registry,
            &mut apply_state_surface,
            &mut apply_flux_surface,
        )
        .expect("indexed writeback should apply");
        black_box(apply_result);
    });

    let mut combined_state_surface = fixture.state_surface.clone();
    let mut combined_flux_surface = fixture.flux_surface.clone();
    let mut combined_indexed_surface = fixture.indexed_surface.clone();
    report_metric("evaluate_plus_apply_indexed_payload", repetitions, || {
        let decision =
            evaluate_indexed_kernel_writeback(SimulationPhase::HillslopeKernel, &fixture.payload)
                .expect("indexed writeback decision should construct");
        let apply_result = apply_indexed_kernel_writeback(
            SimulationPhase::HillslopeKernel,
            &decision,
            &fixture.payload,
            &mut combined_indexed_surface,
            &fixture.registry,
            &mut combined_state_surface,
            &mut combined_flux_surface,
        )
        .expect("indexed writeback should apply");
        black_box((decision, apply_result));
    });
}

fn report_metric<F>(metric: &str, repetitions: usize, mut operation: F)
where
    F: FnMut(),
{
    for _ in 0..100 {
        operation();
    }

    let started = Instant::now();
    for _ in 0..repetitions {
        operation();
    }
    let elapsed = started.elapsed();
    let elapsed_s = elapsed.as_secs_f64();
    let us_per_payload = elapsed_s * 1_000_000.0 / repetitions as f64;
    let ns_per_field = us_per_payload * 1_000.0 / (STATE_UPDATE_COUNT + FLUX_UPDATE_COUNT) as f64;
    let projected_h2637_s = us_per_payload * H2637_OFE_DAYS / 1_000_000.0;

    println!(
        "{metric}\t{repetitions}\t{elapsed_s:.9}\t{us_per_payload:.6}\t{ns_per_field:.6}\t{projected_h2637_s:.6}"
    );
}

struct Fixture {
    registry: SymbolRegistry,
    payload: IndexedKernelWritebackPayload,
    indexed_surface: IndexedWritebackSurface,
    state_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    flux_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
}

impl Fixture {
    fn new() -> Self {
        let state_symbols = (0..STATE_UPDATE_COUNT)
            .map(|index| BoundarySymbol::from(format!("state_{index:04}")))
            .collect::<Vec<_>>();
        let flux_symbols = (0..FLUX_UPDATE_COUNT)
            .map(|index| BoundarySymbol::from(format!("flux_{index:04}")))
            .collect::<Vec<_>>();
        let mut all_symbols = state_symbols.clone();
        all_symbols.extend(flux_symbols.iter().cloned());
        let registry = SymbolRegistry::from_symbols(all_symbols).expect("registry should build");

        let state_surface = state_symbols
            .iter()
            .cloned()
            .map(|symbol| (symbol, BoundaryValue::scalar(0.0)))
            .collect::<BTreeMap<_, _>>();
        let flux_surface = flux_symbols
            .iter()
            .cloned()
            .map(|symbol| (symbol, BoundaryValue::scalar(0.0)))
            .collect::<BTreeMap<_, _>>();
        let indexed_surface = IndexedWritebackSurface::from_btreemap_surfaces(
            &registry,
            &state_surface,
            &flux_surface,
        )
        .expect("indexed writeback surface should build");

        let state_updates = state_symbols
            .iter()
            .enumerate()
            .map(|(index, symbol)| {
                IndexedWritebackField::bounded(
                    registry.id_of(symbol).expect("state id should exist"),
                    BoundaryValue::scalar(index as f64),
                    Some(0.0),
                    None,
                )
            })
            .collect::<Vec<_>>();
        let flux_updates = flux_symbols
            .iter()
            .enumerate()
            .map(|(index, symbol)| {
                IndexedWritebackField::bounded(
                    registry.id_of(symbol).expect("flux id should exist"),
                    BoundaryValue::scalar(index as f64),
                    Some(0.0),
                    None,
                )
            })
            .collect::<Vec<_>>();
        let payload = IndexedKernelWritebackPayload::with_updates(state_updates, flux_updates);

        Self {
            registry,
            payload,
            indexed_surface,
            state_surface,
            flux_surface,
        }
    }
}
