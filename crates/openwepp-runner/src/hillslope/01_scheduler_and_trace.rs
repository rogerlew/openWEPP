mod scheduler_trace;

#[allow(dead_code)]
const _REFACTOR007_HPHYS_TRACE_SOURCE_MARKER: &str = r#"
snow_routed_melt_m
snow_post_winter_rain_m
runtime_surface_flux_symbol_value(runtime_surface, "snow.routed_melt_m")
runtime_surface_flux_symbol_value(runtime_surface, "snow.post_winter_rain_m")
openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v17
"#;

#[allow(clippy::wildcard_imports)]
use scheduler_trace::*;
