mod indexed_shadow_surface;
mod intake_lane_setup;
mod snowbench;
mod snowbench_coe_density;
mod snowbench_coe_melt;
mod snowbench_physics_bulk;
mod symbol_registry_audit;

pub use snowbench::{
    PYSNOBAL_FORCING_COLUMNS, SnowbenchError, SnowbenchExportReport, SnowbenchExportRequest,
    export_openwepp_snow_csv_from_wat, export_pysnobal_inputs,
};
pub use snowbench_coe_density::{
    CoeBoundDensityReport, CoeBoundDensityRequest, CoeBoundDensitySummary,
    run_coe_bound_density_snowbench,
};
pub use snowbench_coe_melt::{
    CoeMeltConstants, CoeMeltModel, CoeMeltReport, CoeMeltRequest, CoeMeltSummary,
    run_coe_melt_snowbench,
};
pub use snowbench_physics_bulk::{
    PhysicsBulkConstants, PhysicsBulkReport, PhysicsBulkRequest, PhysicsBulkVariant,
    fresh_snow_density_kg_m3, physics_bulk_constants, physics_bulk_constants_for_variant,
    run_physics_bulk_snowbench,
};

include!("00_runner_intake_and_lane_setup.rs");
include!("01_scheduler_and_trace.rs");
include!("02_output_and_climate_helpers.rs");
include!("04_direct_publication.rs");
include!("05_runner_execution_and_outputs.rs");
include!("03_tests.rs");
