use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;

use openwepp_sim_contract::status::SimulationStatus;
pub use openwepp_unit_boundary::BoundaryError;
use openwepp_unit_boundary::{
    DensityKilogramsPerCubicMeter, DirectionDegrees, ElapsedTimeSeconds,
    FlowRateCubicMetersPerSecond, FractionUnitInterval, HourOfDay, LinearRateMetersPerSecond,
    ProcessRateMillimetersPerHour, RunoffDepthMillimeters, SolarRadiationLangleysPerDay,
    SolarRadiationMegajoulesPerSquareMeterPerDay, SolarRadiationMegajoulesPerSquareMeterPerHour,
    StorageVolumeCubicMeters, SurfaceAreaSquareMeters, TemperatureCelsius, WaterDepthMeters,
};

/// Message id emitted when writeback payload evaluation accepts all fields.
pub const WRITEBACK_ACCEPT_MESSAGE_ID: &str = "KWRITEBACK-ACCEPT-001";
/// Message id emitted when accepted writeback is applied by orchestrator.
pub const WRITEBACK_APPLY_MESSAGE_ID: &str = "KWRITEBACK-APPLY-001";
/// Message id emitted when writeback is rejected for non-finite values.
pub const WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID: &str = "KWRITEBACK-E-NON-FINITE";
/// Message id emitted when writeback is rejected for domain/range violations.
pub const WRITEBACK_REJECT_DOMAIN_MESSAGE_ID: &str = "KWRITEBACK-E-DOMAIN-VIOLATION";

include!("core_types/00_symbol_registry_and_indexed_surfaces.rs");
include!("core_types/01_typed_symbol_surfaces.rs");
include!("core_types/02_boundary_values_and_kernel_requests.rs");
