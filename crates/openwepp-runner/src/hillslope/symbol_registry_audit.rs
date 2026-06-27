use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::HillslopeWritebackSurface;
use openwepp_input_contract::parsers::climate::{ClimateDailyRecord, ClimateFile};
use openwepp_kernel_contract::{
    BoundarySymbol, SymbolRegistry, SymbolRegistryAuditReport, begin_symbol_registry_audit,
    finish_symbol_registry_audit,
};
use serde::Serialize;

use crate::errors::HillslopeCliError;

use super::HillslopeClimateExecutionState;

const SYMBOL_REGISTRY_AUDIT_PATH_ENV: &str = "OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH";
const SYMBOL_REGISTRY_AUDIT_SCHEMA: &str = "openwepp.symbol_registry_audit.v1";
const DEFAULT_LAYER_COUNT: usize = 1;
const DEFAULT_FINE_LAYER_COUNT: usize = 16;
const MAX_FROST_FINE_CONTROL_COUNT: usize = 10;
const DEFAULT_PARTICLE_CLASS_COUNT: usize = 5;
const MOFE_HOURLY_CARRY_ARRAY_COUNT: usize = 24;

#[derive(Debug)]
pub(super) struct SymbolRegistryAuditRun {
    report_path: PathBuf,
}

impl SymbolRegistryAuditRun {
    pub(super) fn finish(self) -> Result<(), HillslopeCliError> {
        let report = finish_symbol_registry_audit().ok_or_else(|| {
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "symbol_registry_audit",
                detail: "symbol registry audit was not active at finish".to_string(),
            }
        })?;
        write_report(&self.report_path, &report)?;
        if !report.is_complete() {
            return Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "symbol_registry_audit",
                detail: format!(
                    "frozen symbol registry missed {} post-freeze symbols; report written to {}",
                    report.unknown_symbol_count(),
                    self.report_path.display()
                ),
            });
        }
        Ok(())
    }
}

pub(super) fn begin_if_requested(
    state: &HillslopeClimateExecutionState,
    climate: &ClimateFile,
) -> Result<Option<SymbolRegistryAuditRun>, HillslopeCliError> {
    let Some(path) = env::var_os(SYMBOL_REGISTRY_AUDIT_PATH_ENV).map(PathBuf::from) else {
        return Ok(None);
    };
    if path.as_os_str().is_empty() {
        return Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "symbol_registry_audit",
            detail: format!("{SYMBOL_REGISTRY_AUDIT_PATH_ENV} must not be empty"),
        });
    }

    let registry = build_registry_for_run(state, climate, "symbol_registry_audit")?;
    begin_symbol_registry_audit(registry).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: "symbol_registry_audit",
            detail: error.to_string(),
        }
    })?;
    Ok(Some(SymbolRegistryAuditRun { report_path: path }))
}

pub(super) fn build_registry_for_run(
    state: &HillslopeClimateExecutionState,
    climate: &ClimateFile,
    surface_name: &'static str,
) -> Result<SymbolRegistry, HillslopeCliError> {
    SymbolRegistry::from_symbols(collect_run_symbols(state, climate)).map_err(|error| {
        HillslopeCliError::RuntimeSurfaceFailure {
            surface: surface_name,
            detail: error.to_string(),
        }
    })
}

fn write_report(
    report_path: &Path,
    report: &SymbolRegistryAuditReport,
) -> Result<(), HillslopeCliError> {
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent).map_err(|source| HillslopeCliError::OutputWrite {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    let payload = SymbolRegistryAuditReportJson {
        schema: SYMBOL_REGISTRY_AUDIT_SCHEMA,
        registry_symbol_count: report.registry_symbol_count(),
        constructed_symbol_count: report.constructed_symbol_count(),
        unknown_symbol_count: report.unknown_symbol_count(),
        unknown_symbols: report
            .unknown_symbols()
            .iter()
            .map(|symbol| symbol.as_str().to_owned())
            .collect(),
    };
    let json = serde_json::to_string_pretty(&payload)
        .map_err(|source| HillslopeCliError::ManifestSerialize { source })?;
    fs::write(report_path, json).map_err(|source| HillslopeCliError::OutputWrite {
        path: report_path.to_path_buf(),
        source,
    })
}

#[derive(Debug, Serialize)]
struct SymbolRegistryAuditReportJson {
    schema: &'static str,
    registry_symbol_count: usize,
    constructed_symbol_count: usize,
    unknown_symbol_count: usize,
    unknown_symbols: Vec<String>,
}

fn collect_run_symbols(
    state: &HillslopeClimateExecutionState,
    climate: &ClimateFile,
) -> BTreeSet<String> {
    let mut symbols = BTreeSet::new();
    collect_surface_symbols(&mut symbols, &state.runtime_surface);
    if let Some(persistent_lane_state) = state.persistent_lane_state.as_ref() {
        for lane in persistent_lane_state.lane_states() {
            collect_surface_symbols(&mut symbols, &lane.writeback_surface);
        }
    }
    collect_static_symbols(&mut symbols);
    collect_climate_symbols(&mut symbols, climate);
    collect_layer_symbols(&mut symbols, state);
    collect_frost_symbols(&mut symbols, state);
    collect_mofe_symbols(&mut symbols);
    collect_pl_symbols(&mut symbols, state);
    collect_ofe_symbols(&mut symbols, state.contributor_ofe_count);
    collect_erod_symbols(&mut symbols, state);
    collect_irrigation_symbols(&mut symbols, state);
    symbols
}

fn collect_surface_symbols(symbols: &mut BTreeSet<String>, surface: &HillslopeWritebackSurface) {
    symbols.extend(
        surface
            .state_surface
            .keys()
            .map(|symbol| symbol.as_str().to_owned()),
    );
    symbols.extend(
        surface
            .flux_surface
            .keys()
            .map(|symbol| symbol.as_str().to_owned()),
    );
}

fn collect_static_symbols(symbols: &mut BTreeSet<String>) {
    symbols.extend(STATIC_SYMBOLS.iter().map(|symbol| (*symbol).to_owned()));
    for month in 1..=12 {
        for root in ["obmaxt", "obmint", "radave", "obrain"] {
            symbols.insert(format!("{root}_{month:04}"));
        }
    }
}

fn collect_climate_symbols(symbols: &mut BTreeSet<String>, climate: &ClimateFile) {
    let breakpoint_count = climate
        .daily_records
        .iter()
        .map(|record| match record {
            ClimateDailyRecord::NoBreakpoint(_) => 24,
            ClimateDailyRecord::Breakpoint(day) => day.breakpoints.len(),
        })
        .max()
        .unwrap_or(0);
    for index in 1..=breakpoint_count {
        symbols.insert(format!("timem_{index:04}"));
        symbols.insert(format!("intsty_{index:04}"));
    }
    for hour in 1..=24 {
        for root in HOURLY_SYMBOL_ROOTS {
            symbols.insert(format!("{root}_{hour:04}"));
        }
    }
}

fn collect_layer_symbols(symbols: &mut BTreeSet<String>, state: &HillslopeClimateExecutionState) {
    let layer_count = infer_layer_count(state).max(DEFAULT_LAYER_COUNT);
    for layer_index in 1..=layer_count {
        for root in LAYER_SYMBOL_ROOTS {
            symbols.insert(format!("{root}_{layer_index:04}"));
        }
    }
    for lower in 1..=layer_count {
        for upper in lower..=layer_count {
            symbols.insert(format!("wb18_perc_ul_agg_{lower:04}_{upper:04}"));
            symbols.insert(format!("dg_agg_{lower:04}_{upper:04}"));
        }
    }
}

fn collect_frost_symbols(symbols: &mut BTreeSet<String>, state: &HillslopeClimateExecutionState) {
    let layer_count = infer_layer_count(state).max(DEFAULT_LAYER_COUNT);
    let fine_count = infer_frost_fine_count(state, layer_count).max(DEFAULT_FINE_LAYER_COUNT);
    for layer_index in 1..=layer_count {
        for root in FROST_LAYER_SYMBOL_ROOTS {
            symbols.insert(format!("{root}_{layer_index:04}"));
        }
        for fine_index in 1..=fine_count {
            for root in FROST_FINE_SYMBOL_ROOTS {
                symbols.insert(format!("{root}_{layer_index:04}_{fine_index:04}"));
            }
        }
    }
}

fn collect_mofe_symbols(symbols: &mut BTreeSet<String>) {
    for hour in 1..=MOFE_HOURLY_CARRY_ARRAY_COUNT {
        for root in ["ui_SUrunf", "ui_SCrunf", "ui_LfUrf", "ui_LfCrf"] {
            symbols.insert(format!("{root}_{hour:04}"));
        }
    }
}

fn collect_pl_symbols(symbols: &mut BTreeSet<String>, state: &HillslopeClimateExecutionState) {
    let slot_count = infer_runtime_usize(state, "pl_schedule_slot_count").unwrap_or(0);
    let crop_slot_count = infer_max_pl_crop_slots(state, slot_count).unwrap_or(1);
    for slot_index in 1..=slot_count {
        for root in PL_SCHEDULE_SLOT_ROOTS {
            symbols.insert(format!("pl_schedule_slot_{slot_index:04}_{root}"));
        }
        for crop_slot_index in 1..=crop_slot_count {
            for root in PL_SCHEDULE_CROP_ROOTS {
                symbols.insert(format!(
                    "pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}"
                ));
            }
            for root in PL_GROWTH_CROP_ROOTS {
                symbols.insert(format!(
                    "pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}"
                ));
            }
            for root in PL_DECOMP_CROP_ROOTS {
                symbols.insert(format!(
                    "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}"
                ));
            }
            let ncut =
                infer_pl_slot_crop_usize(state, slot_index, crop_slot_index, "ncut").unwrap_or(0);
            for sequence_index in 1..=ncut {
                symbols.insert(format!(
                    "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_cutday_{sequence_index:04}"
                ));
            }
            let ncycle =
                infer_pl_slot_crop_usize(state, slot_index, crop_slot_index, "ncycle").unwrap_or(0);
            for sequence_index in 1..=ncycle {
                for root in PL_DECOMP_GRAZING_INDEXED_CROP_ROOTS {
                    symbols.insert(format!(
                        "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}_{sequence_index:04}"
                    ));
                }
            }
        }
    }
}

fn collect_ofe_symbols(symbols: &mut BTreeSet<String>, contributor_ofe_count: usize) {
    for ofe_index in 1..=contributor_ofe_count.max(1) {
        for root in OFE_SYMBOL_ROOTS {
            symbols.insert(format!("ofe{ofe_index}_{root}"));
        }
        for root in ["bb_seed", "bbb_seed", "flivmx_seed", "hmax_seed"] {
            symbols.insert(format!("pl_growth_ofe{ofe_index}_{root}"));
        }
    }
}

fn collect_erod_symbols(symbols: &mut BTreeSet<String>, state: &HillslopeClimateExecutionState) {
    let class_count = infer_runtime_usize(state, "erod14_class_count")
        .or_else(|| infer_runtime_usize(state, "particle_class_count"))
        .unwrap_or(DEFAULT_PARTICLE_CLASS_COUNT)
        .max(state.contributor_ofe_count)
        .max(DEFAULT_PARTICLE_CLASS_COUNT);
    for class_index in 1..=class_count {
        for root in EROD14_CLASS_ROOTS {
            symbols.insert(format!("{root}_{class_index:04}"));
        }
        for root in EROD15_CLASS_ROOTS {
            symbols.insert(format!("{root}_{class_index:04}"));
        }
    }

    let segment_count = infer_runtime_usize(state, "nelem")
        .unwrap_or(state.contributor_ofe_count)
        .max(1);
    for segment_index in 1..=segment_count {
        for root in EROD18_SEGMENT_ROOTS {
            symbols.insert(format!("{root}_{segment_index:04}"));
        }
    }
}

fn collect_irrigation_symbols(
    symbols: &mut BTreeSet<String>,
    state: &HillslopeClimateExecutionState,
) {
    let depletion_periods =
        infer_runtime_usize(state, "irrigation.depletion.period_count").unwrap_or(0);
    for period_index in 1..=depletion_periods {
        for field in ["start_day", "end_day", "trigger_ratio"] {
            symbols.insert(format!(
                "irrigation.depletion.period_{period_index:04}.{field}"
            ));
        }
    }

    let fixed_events = infer_runtime_usize(state, "irrigation.fixeddate.event_count").unwrap_or(0);
    for event_index in 1..=fixed_events {
        for field in ["year", "day", "depth_m", "duration_s", "rate_m_per_s"] {
            symbols.insert(format!(
                "irrigation.fixeddate.event_{event_index:04}.{field}"
            ));
        }
    }
}

fn infer_layer_count(state: &HillslopeClimateExecutionState) -> usize {
    infer_runtime_usize(state, "wb11_nsl")
        .or_else(|| infer_runtime_usize(state, "nsl"))
        .unwrap_or_else(|| {
            infer_max_suffix(
                state,
                &[
                    "wb19_dg",
                    "dg",
                    "wb19_thetfc",
                    "thetfc",
                    "wb18_perc_theta",
                    "frost.runtime_nfine",
                ],
            )
        })
}

fn infer_frost_fine_count(state: &HillslopeClimateExecutionState, layer_count: usize) -> usize {
    let mut fine_count =
        infer_runtime_usize(state, "frost.runtime_total_fine_layer_count").unwrap_or(0);
    for layer_index in 1..=layer_count {
        let symbol = format!("frost.runtime_nfine_{layer_index:04}");
        fine_count = fine_count.max(infer_runtime_usize(state, &symbol).unwrap_or(0));
    }
    fine_count
        .max(infer_max_frost_fine_suffix(state, FROST_FINE_SYMBOL_ROOTS))
        .max(MAX_FROST_FINE_CONTROL_COUNT * layer_count.max(1))
}

fn infer_max_pl_crop_slots(
    state: &HillslopeClimateExecutionState,
    slot_count: usize,
) -> Option<usize> {
    let mut max_crop_slots: Option<usize> = None;
    for slot_index in 1..=slot_count {
        let symbol = format!("pl_schedule_slot_{slot_index:04}_crop_slots");
        if let Some(count) = infer_runtime_usize(state, &symbol) {
            max_crop_slots = Some(max_crop_slots.unwrap_or(0).max(count));
        }
    }
    max_crop_slots
}

fn infer_pl_slot_crop_usize(
    state: &HillslopeClimateExecutionState,
    slot_index: usize,
    crop_slot_index: usize,
    field: &str,
) -> Option<usize> {
    let symbol = format!("pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{field}");
    infer_runtime_usize(state, &symbol)
}

fn infer_runtime_usize(state: &HillslopeClimateExecutionState, symbol: &str) -> Option<usize> {
    surface_runtime_usize(&state.runtime_surface, symbol).or_else(|| {
        state.persistent_lane_state.as_ref().and_then(|persistent| {
            persistent
                .lane_states()
                .iter()
                .find_map(|lane| surface_runtime_usize(&lane.writeback_surface, symbol))
        })
    })
}

fn surface_runtime_usize(surface: &HillslopeWritebackSurface, symbol: &str) -> Option<usize> {
    let raw = surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .or_else(|| surface.flux_surface.get(&BoundarySymbol::from(symbol)))?
        .as_f64();
    finite_non_negative_integral_usize(raw)
}

fn finite_non_negative_integral_usize(value: f64) -> Option<usize> {
    if !value.is_finite() || value < 0.0 {
        return None;
    }
    let rounded = value.round();
    if (rounded - value).abs() > 1.0e-9 {
        return None;
    }
    format!("{rounded:.0}").parse::<usize>().ok()
}

fn infer_max_suffix(state: &HillslopeClimateExecutionState, roots: &[&str]) -> usize {
    let mut max_suffix = infer_max_surface_suffix(&state.runtime_surface, roots);
    if let Some(persistent_lane_state) = state.persistent_lane_state.as_ref() {
        for lane in persistent_lane_state.lane_states() {
            max_suffix = max_suffix.max(infer_max_surface_suffix(&lane.writeback_surface, roots));
        }
    }
    max_suffix
}

fn infer_max_surface_suffix(surface: &HillslopeWritebackSurface, roots: &[&str]) -> usize {
    surface
        .state_surface
        .keys()
        .chain(surface.flux_surface.keys())
        .filter_map(|symbol| infer_rooted_suffix(symbol.as_str(), roots))
        .max()
        .unwrap_or(0)
}

fn infer_max_frost_fine_suffix(state: &HillslopeClimateExecutionState, roots: &[&str]) -> usize {
    let mut max_suffix = infer_max_surface_frost_fine_suffix(&state.runtime_surface, roots);
    if let Some(persistent_lane_state) = state.persistent_lane_state.as_ref() {
        for lane in persistent_lane_state.lane_states() {
            max_suffix = max_suffix.max(infer_max_surface_frost_fine_suffix(
                &lane.writeback_surface,
                roots,
            ));
        }
    }
    max_suffix
}

fn infer_max_surface_frost_fine_suffix(
    surface: &HillslopeWritebackSurface,
    roots: &[&str],
) -> usize {
    surface
        .state_surface
        .keys()
        .chain(surface.flux_surface.keys())
        .filter_map(|symbol| infer_frost_fine_suffix(symbol.as_str(), roots))
        .max()
        .unwrap_or(0)
}

fn infer_rooted_suffix(symbol: &str, roots: &[&str]) -> Option<usize> {
    for root in roots {
        let prefix = format!("{root}_");
        let Some(suffix) = symbol.strip_prefix(&prefix) else {
            continue;
        };
        let four_digit = suffix.get(..4)?;
        if four_digit.chars().all(|ch| ch.is_ascii_digit()) {
            return four_digit.parse::<usize>().ok();
        }
    }
    None
}

fn infer_frost_fine_suffix(symbol: &str, roots: &[&str]) -> Option<usize> {
    for root in roots {
        let prefix = format!("{root}_");
        let Some(suffix) = symbol.strip_prefix(&prefix) else {
            continue;
        };
        let layer = suffix.get(..4)?;
        let fine = suffix.get(5..9)?;
        if suffix.as_bytes().get(4).copied() == Some(b'_')
            && layer.chars().all(|ch| ch.is_ascii_digit())
            && fine.chars().all(|ch| ch.is_ascii_digit())
        {
            return fine.parse::<usize>().ok();
        }
    }
    None
}

const HOURLY_SYMBOL_ROOTS: &[&str] = &[
    "frost.hourly.frzflg",
    "frost.hourly.ksrf_w_m_k",
    "frost.hourly.qsrf_w_m2",
    "frost.hourly.quf_w_m2",
    "frost.hourly.residue_depth_m",
    "frost.hourly.snow_depth_m",
    "frost.hourly.surface_temp_c",
    "frost.hourly.tilled_frozen_depth_m",
    "frost.hourly.untilled_frozen_depth_m",
    "snow.hourly.density_after_kg_m3",
    "snow.hourly.density_before_kg_m3",
    "snow.hourly.depth_after_m",
    "snow.hourly.depth_available_m",
    "snow.hourly.depth_before_m",
    "snow.hourly.melt_amelt_in",
    "snow.hourly.melt_bmelt_in",
    "snow.hourly.melt_branch_active",
    "snow.hourly.melt_cmelt_in",
    "snow.hourly.melt_dmelt_in",
    "snow.hourly.melt_hrdtf_f",
    "snow.hourly.melt_hrtef_f",
    "snow.hourly.melt_m",
    "snow.hourly.melt_rainin",
    "snow.hourly.melt_raw_m",
    "snow.hourly.melt_vwmph",
    "snow.hourly.melt_wind_adjustment",
    "snow.hourly.rain_m",
    "snow.hourly.rain_released_m",
    "snow.hourly.rain_retained_m",
    "snow.hourly.snowfall_m",
    "snow.hourly.stmtim.active_interval",
    "snow.hourly.stmtim.hrtemp_c",
    "snow.hourly.stmtim.hrrain_m",
    "snow.hourly.stmtim.hrsnow_m",
    "snow.hourly.stmtim.hydrometeor_temperature_c",
    "snow.hourly.stmtim.phase_model",
    "snow.hourly.stmtim.rain_branch",
    "snow.hourly.stmtim.rain_fraction",
    "snow.hourly.stmtim.rain_m",
    "snow.hourly.stmtim.relative_humidity",
    "snow.hourly.stmtim.rst_c",
    "snow.hourly.stmtim.snow_branch",
    "snow.hourly.stmtim.snow_fraction",
    "snow.hourly.stmtim.stmdur_s",
    "snow.hourly.stmtim.wntdur_h",
    "snow.hourly.stmtim.wnttim_h",
    "winter.hourly.air_temp_c",
    "winter.hourly.cloud_fraction",
    "winter.hourly.dewpoint_c",
    "winter.hourly.rad_mj_m2",
    "winter.hourly.wind_m_s",
];

const STATIC_SYMBOLS: &[&str] = &[
    "D",
    "Di",
    "Dc",
    "Df",
    "ER",
    "ET",
    "Ep",
    "Er",
    "Es",
    "Etp",
    "Fh",
    "Fp",
    "G",
    "I",
    "Ie",
    "Irr",
    "Pe",
    "Q",
    "Qd",
    "Qdd",
    "SubRIn",
    "S",
    "Tc",
    "Total-Soil",
    "Ui",
    "UPi",
    "UpStrmQ",
    "Ws",
    "active_grazing_cycle",
    "alpha",
    "avthetafc",
    "avthetafc_avthetadr",
    "avgslp",
    "avrint",
    "azm",
    "beta",
    "cancov",
    "canhgt",
    "cntlen",
    "coca",
    "cpm",
    "cutday",
    "dGdx",
    "datver",
    "day",
    "deglat",
    "detinr",
    "dg",
    "dl",
    "du",
    "effdrr",
    "effdrn",
    "efflen",
    "ealpha",
    "elevm",
    "erod13_core_enabled",
    "erod13_tc_k",
    "erod13_tc_m",
    "erod14_Fh",
    "erod14_Fp",
    "erod14_Qj",
    "erod14_Qj_minus_1",
    "erod14_Vj",
    "erod14_ainftc",
    "erod14_beta",
    "erod14_binftc",
    "erod14_case",
    "erod14_cinftc",
    "erod14_class_count",
    "erod14_ktrato",
    "erod14_ldbot",
    "erod14_lddend",
    "erod14_ldtop",
    "erod14_qin",
    "erod14_qostar",
    "erod14_qout",
    "erod14_slplen",
    "erod14_ssa_soil",
    "erod14_sumg",
    "erod14_wave2_enabled",
    "erod14_xbot",
    "erod14_xdetst",
    "erod14_xtop",
    "erod18_core_enabled",
    "fs",
    "ft",
    "frost.hourly.frzflg",
    "frost.hourly.ksrf_w_m_k",
    "frost.hourly.qsrf_w_m2",
    "frost.hourly.quf_w_m2",
    "frost.hourly.residue_depth_m",
    "frost.hourly.snow_depth_m",
    "frost.hourly.surface_temp_c",
    "frost.hourly.tilled_frozen_depth_m",
    "frost.hourly.untilled_frozen_depth_m",
    "frost.options.fineBot",
    "frost.options.fineTop",
    "frost.options.frost_file_present",
    "frost.options.kfactor1",
    "frost.options.kfactor2",
    "frost.options.kfactor3",
    "frost.options.kresf",
    "frost.options.ksnowf",
    "frost.options.ksoilf",
    "frost.options.wintRed",
    "frost.runtime_dfrost",
    "frost.runtime_dthaw",
    "frost.runtime_fgthwd_flag",
    "frost.runtime_frdp_m",
    "frost.runtime_frwatc_freeze_debit_m",
    "frost.runtime_frwatc_frozen_water_after_m",
    "frost.runtime_frwatc_frozen_water_before_m",
    "frost.runtime_frwatc_net_liquid_delta_m",
    "frost.runtime_frwatc_soil_water_after_m",
    "frost.runtime_frwatc_soil_water_before_m",
    "frost.runtime_frwatc_thaw_credit_m",
    "frost.runtime_infcap_frz",
    "frost.runtime_kftill_w_m_k",
    "frost.runtime_kfutil_w_m_k",
    "frost.runtime_kres_w_m_k",
    "frost.runtime_nft",
    "frost.runtime_residue_depth_m",
    "frost.runtime_shadow_frwatc_residual_m",
    "frost.runtime_shadow_total_water_after_m",
    "frost.runtime_shadow_total_water_before_m",
    "frost.runtime_shadow_wb_delta_m",
    "frost.runtime_tfrdp_m",
    "frost.runtime_thdp_m",
    "frost.runtime_total_fine_layer_count",
    "frost.runtime_tthawd_m",
    "frost.runtime_watbtm_m",
    "frost.runtime_watpdg_m",
    "frost.runtime_ws_frz",
    "frlive",
    "hia",
    "ibrkpt",
    "iclig",
    "intsty_0001",
    "ip",
    "irrigation.depletion.enabled",
    "irrigation.depletion.max_depth_m",
    "irrigation.depletion.min_depth_m",
    "irrigation.depletion.period_count",
    "irrigation.depletion.period_window",
    "irrigation.depletion.system_type",
    "irrigation.depletion.trigger_ratio",
    "irrigation.fixeddate.enabled",
    "irrigation.fixeddate.event_count",
    "irrigation.fixeddate.system_type",
    "irrigation.runtime_depth_m",
    "irrigation.runtime_duration_s",
    "irrigation.runtime_event_index",
    "irrigation.runtime_rate_m_per_s",
    "irrigation.runtime_schedule_source",
    "irrigation.runtime_system_type",
    "iresd_seed",
    "itemp",
    "iwind",
    "keff",
    "kr",
    "kradjf",
    "ksatadj",
    "ksatfac",
    "ksatrec",
    "kslast",
    "lai",
    "lambda",
    "landuse.class_proxy",
    "lkeff",
    "lddend",
    "ldlast",
    "m",
    "management.initial.params.tillay2_m",
    "mon",
    "mshear",
    "mofe.static_lane.contributor_ofe_count",
    "mofe_hourly_carry_arrays_enabled",
    "mofe_hourly_upstream_area_ratio",
    "mxint",
    "nbrkpt",
    "ndep",
    "nelem",
    "ninten",
    "nsl",
    "nslpts",
    "nwsofe",
    "peakro",
    "particle_class_count",
    "phi",
    "pl_order_decomp_before_soil",
    "pl_order_growth_after_decomp",
    "pl_order_watbal_after_growth",
    "pl_schedule_nofe",
    "pl_schedule_rotation_repeats",
    "pl_schedule_rotation_years",
    "pl_schedule_slot_count",
    "pltol",
    "pmet.ep_m",
    "pmet.es_m",
    "pmet.es_storage_return_m",
    "pmet.etke",
    "pmet.etkr",
    "pmet.etks",
    "pmet.etorc_mm",
    "pmet.fwv_m_s",
    "pmet.kcbadj",
    "pmet.kcbcon",
    "pmet.raw_mm",
    "pmet.rew_mm",
    "pmet.rhd_pct",
    "pmet.rn_mj_m2",
    "pmet.taw_mm",
    "pmet.tew_mm",
    "pmet.wfevp_mm",
    "pmet.wftrp_mm",
    "pmetpara.mode.iflget",
    "pmetpara.selected.kcb",
    "pmetpara.selected.rawp",
    "por",
    "prcp",
    "psi",
    "q",
    "qostar",
    "rad",
    "radpot",
    "rilcov",
    "rrc",
    "rrinit",
    "rspace",
    "rtd",
    "rtmass",
    "rtyp",
    "s1",
    "s2",
    "salb",
    "sat",
    "sat_frac",
    "shcrit",
    "shrsol",
    "slflag",
    "slplen",
    "slpinp_0001",
    "snow.options.newsnw",
    "snow.options.rst",
    "snow.options.snow_file_present",
    "snow.options.ssd",
    "snow.post_winter_rain_m",
    "snow.routed_melt_m",
    "snow.hourly.melt_m",
    "snow.hourly.rain_m",
    "snow.runtime_density_kg_m3",
    "snow.runtime_depth_m",
    "snow.runtime_settle_day_count",
    "snow.runtime_swe",
    "soilWaterTotal",
    "SoilWaterTotal",
    "solthk",
    "solwpv",
    "ssc",
    "stmstr",
    "stmdur",
    "sumrtm_seed",
    "sumsrm_seed",
    "sumgdd",
    "swu_effective_pltol",
    "taucn",
    "taufe",
    "tcadjf",
    "tcend",
    "tdpt",
    "te",
    "theta",
    "thetdr",
    "thetfc",
    "timep",
    "timem_0001",
    "tmax",
    "tmin",
    "total_dep",
    "total_detachment_kg",
    "total_deposition_kg",
    "tu",
    "tv",
    "ui_bdrkth",
    "vf",
    "vdmt",
    "veleff",
    "vwind",
    "watcon",
    "watdur",
    "wb11_drainable_storage",
    "wb11_drainage_coefficient",
    "wb11_drainage_fraction",
    "wb11_et_demand",
    "wb11_et_seed_branch_evappm",
    "wb11_et_seed_branch_priestley_taylor",
    "wb11_field_capacity",
    "wb11_lateral_fraction",
    "wb11_nsl",
    "wb11_perc_fraction",
    "wb11_soil_water",
    "wb11_state_seed_completed",
    "wb12_depression_storage_delta",
    "wb12_infiltration",
    "wb12_infiltration_same_pass_lineage",
    "wb12_precip_input",
    "wb12_rainfall_input",
    "wb12_runoff_carryover",
    "wb12_runoff_closure_delta",
    "wb12_runoff_closure_tolerance",
    "wb12_runoff_observed",
    "wb12_runoff_reconciled",
    "wb12_runon_input",
    "wb12_snow_coupling_s",
    "wb12_storage_closure_delta",
    "wb12_storage_closure_tolerance",
    "wb12_storage_initial",
    "wb12_storage_observed",
    "wb12_storage_reconciled",
    "wb13_profile_depth_mm",
    "wb13_profile_fc_store_mm",
    "wb13_profile_fc_tail_mm",
    "wb13_profile_porosity_cap_mm",
    "wb13_profile_wp_store_mm",
    "wb13_storage_total_mm",
    "wb14_effective_conductivity_m_s",
    "wb14_matric_potential_m",
    "wb14_soil_conductivity_m_s",
    "wb16_ealpha_compatibility_seed_used",
    "wb16_peak_method_branch",
    "wb16_qpstar",
    "wb16_tstar",
    "wb16_vstar",
    "wb17_residue_interception",
    "wb18_perc_lane_substeps",
    "wb19_drain_depth",
    "wb19_drain_diameter",
    "wb19_drain_enabled",
    "wb19_drain_spacing",
    "wb19_fcdep",
    "wb19_lateral_anisotropy_ratio",
    "wb19_lateral_capacity_tdv",
    "wb19_lateral_drain_lane_substeps",
    "wb19_q_lateral_potential",
    "wb19_q_lateral_target",
    "wb19_q_lateral_unrealized",
    "wb19_tdvv",
    "wb19_unsdep",
    "wb19_watyld",
    "wb20_forward_solver_lane_enabled",
    "width",
    "wind",
    "winter.hourly.cloud_fraction",
    "winter.hourly.rad_mj_m2",
    "xinput_0001",
    "xdbeg",
    "xc1",
    "xc2",
    "xdend",
    "xdetst",
    "year",
];

const LAYER_SYMBOL_ROOTS: &[&str] = &[
    "UPi",
    "Ui",
    "coca",
    "cpm",
    "dg",
    "por",
    "solthk",
    "ssc",
    "thetdr",
    "thetfc",
    "wb18_perc_fc",
    "wb18_perc_frozen_depth",
    "wb18_perc_frzw",
    "wb18_perc_pei",
    "wb18_perc_ssc",
    "wb18_perc_theta",
    "wb18_perc_ul",
    "wb19_bulk_density_kg_m3",
    "wb19_coca",
    "wb19_dg",
    "wb19_lateral_capacity_active_count",
    "wb19_lateral_conductivity_active_count",
    "wb19_lateral_ssh",
    "wb19_lateral_withdrawal",
    "wb19_por",
    "wb19_solthk",
    "wb19_thetfc",
    "wb19_thetdr",
];

const FROST_LAYER_SYMBOL_ROOTS: &[&str] = &[
    "frost.runtime_fine_thickness_m",
    "frost.runtime_nfine",
    "frost.runtime_nwfrzz_m",
    "frost.runtime_shadow_frozen_depth_m",
    "frost.runtime_shadow_frzw_m",
    "frost.runtime_shadow_soil_water_m",
    "frost.runtime_shadow_soilf_m",
    "frost.runtime_shadow_st_m",
    "frost.runtime_yst_m",
];

const FROST_FINE_SYMBOL_ROOTS: &[&str] = &[
    "frost.runtime_fgfrst",
    "frost.runtime_slfsd_m",
    "frost.runtime_slsic_m",
    "frost.runtime_slsw_theta",
    "frost.runtime_sltime_s",
];

const PL_SCHEDULE_SLOT_ROOTS: &[&str] = &[
    "rotation_index",
    "year_in_rotation",
    "ofe_index",
    "crop_slots",
];
const PL_SCHEDULE_CROP_ROOTS: &[&str] = &[
    "yearly_ref",
    "lanuse",
    "itype",
    "tilseq",
    "conset",
    "drset",
    "imngmt",
];
const PL_GROWTH_CROP_ROOTS: &[&str] = &[
    "bb", "bbb", "beinp", "btemp", "cancov", "decfct", "dlai", "dropfc", "extnct", "flivmx",
    "gddmax", "hia", "hi", "hmax", "imngmt", "itype", "jdharv", "jdplt", "jdstop", "lai", "mgtopt",
    "otemp", "rdmax", "rsr", "rtd", "rtmass", "rtmmax", "rw", "spriod", "sumgdd", "vdmt", "xmxlai",
];
const PL_DECOMP_CROP_ROOTS: &[&str] = &[
    "fbrnag", "fbrnog", "frcut", "frmove", "iresd", "jdburn", "jdcut", "jdherb", "jdmove",
    "jdslge", "mgtopt", "ncut", "ncycle", "oratea", "orater", "resmgt", "sumrtm", "sumsrm",
];
const PL_DECOMP_GRAZING_INDEXED_CROP_ROOTS: &[&str] =
    &["animal", "area", "bodywt", "digest", "gday", "gend"];

const OFE_SYMBOL_ROOTS: &[&str] = &[
    "alpha", "avgslp", "cancov", "canhgt", "frcteq", "frlive", "inrcov", "lai", "rilcov", "rrinit",
    "rrc", "rspace", "rtyp", "slplen", "vdmt", "width",
];

const EROD14_CLASS_ROOTS: &[&str] = &[
    "erod14_fall",
    "erod14_fidel",
    "erod14_frac",
    "erod14_frcflw",
    "erod14_gend",
    "erod14_sedmax",
    "erod14_ssa_class",
    "erod14_tcf1",
    "sed_frac",
];

const EROD15_CLASS_ROOTS: &[&str] = &["particle_flow_fraction", "sediment_concentration_kg_m3"];

const EROD18_SEGMENT_ROOTS: &[&str] = &[
    "ainf", "ainftc", "binf", "binftc", "cinf", "cinftc", "xl", "xu",
];
