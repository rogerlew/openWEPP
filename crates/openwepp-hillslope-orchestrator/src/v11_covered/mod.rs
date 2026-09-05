//! Snow-covered V11 carrier iteration, receipts, and imported-stack execution.

include!("receipt_sets.rs");
include!("execution.rs");
include!("carrier_engine.rs");
include!("carrier_phase.rs");
include!("regime.rs");
include!("fixed_point.rs");
include!("open_snow_receipt_reseal_helpers.rs");
include!("terminal_composition.rs");
include!("open_snow_physical_support.rs");
include!("canonical_covered_solver.rs");
include!("open_snow.rs");

pub(crate) mod physical_outcome_ledger;

const OUTCOME_LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;

#[inline(never)]
fn covered_boxed_execution_v1<T, E>(execute: impl FnOnce() -> Result<T, E>) -> Result<Box<T>, E> {
    execute().map(Box::new)
}

fn reciprocal_longwave_receipt_digest(
    destination: &(OfeId, TileId),
    support: TimeSupport,
    net_longwave_w_m2: f64,
) -> Digest32 {
    let mut bytes = Vec::with_capacity(192);
    bytes.extend_from_slice(b"OPENWEPP_RECIPROCAL_LONGWAVE_RECEIPT_V1\0");
    bytes.extend_from_slice(&support.start_ns().get().to_le_bytes());
    bytes.extend_from_slice(&support.end_ns().get().to_le_bytes());
    bytes.extend_from_slice(destination.0.as_str().as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(destination.1.as_str().as_bytes());
    bytes.extend_from_slice(&net_longwave_w_m2.to_bits().to_le_bytes());
    digest_bytes(&bytes)
}
