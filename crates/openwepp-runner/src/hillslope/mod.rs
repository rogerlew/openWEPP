mod direct_seed_projections;
mod intake_lane_setup;
mod output_transaction;
mod snow_stage3_v11_production_seed;
#[cfg(test)]
mod snow_stage3_v11_qualification_audit;
mod snowbench;
mod snowbench_coe_density;
mod snowbench_coe_melt;
mod snowbench_jennings_phase;
mod snowbench_physics_bulk;
#[cfg(any(test, feature = "test-fixture-authority"))]
mod test_fixture_authority;
mod transaction_spool;

pub use snowbench::{
    PYSNOBAL_FORCING_COLUMNS, SnowbenchCanopySeriesSummary, SnowbenchError, SnowbenchExportReport,
    SnowbenchExportRequest, export_openwepp_snow_csv_from_wat, export_pysnobal_inputs,
};
pub use snowbench_coe_density::{
    CoeBoundDensityReport, CoeBoundDensityRequest, CoeBoundDensitySummary,
    run_coe_bound_density_snowbench,
};
pub use snowbench_coe_melt::{
    CoeMeltConstants, CoeMeltModel, CoeMeltReport, CoeMeltRequest, CoeMeltSummary,
    run_coe_melt_snowbench,
};
pub use snowbench_jennings_phase::{
    JenningsPhaseValidationReport, JenningsPhaseValidationRequest, run_jennings_phase_validation,
};
pub use snowbench_physics_bulk::{
    PhysicsBulkConstants, PhysicsBulkReport, PhysicsBulkRequest, PhysicsBulkVariant,
    fresh_snow_density_kg_m3, physics_bulk_constants, physics_bulk_constants_for_variant,
    run_physics_bulk_snowbench,
};
#[cfg(feature = "test-fixture-authority")]
pub use test_fixture_authority::{
    Stage3TestFixtureSeedBinding, Stage3TestFixtureSeedProfile,
    author_stage3_v11_owner_seed_fixture,
};

pub(crate) mod laned_active;
#[cfg(test)]
pub(crate) mod laned_shadow;

include!("00_runner_intake_and_lane_setup.rs");
include!("01_scheduler_and_trace.rs");
include!("02_output_and_climate_helpers.rs");
include!("04_direct_publication.rs");
include!("05_runner_execution_and_outputs.rs");
include!("03_tests.rs");
