//! Executable, package-local authority for persisted-restart wire semantics.
//!
//! This crate is evidence tooling. It is not linked by the openWEPP workspace
//! and exposes no production selector or runtime path.

mod canonical;
mod hydrology_core;
mod primitives;

pub use canonical::{CanonicalJsonError, from_canonical_bytes, to_canonical_bytes};
pub use hydrology_core::{DirectHydrologyCoreError, DirectWaterStateRestartV1};
pub use primitives::{
    HexF64, HexU128, Sha256Hex, WireCount, WireDayIndex, WireIntervalIndex, WireLaneIndex,
    WirePrimitiveError,
};
