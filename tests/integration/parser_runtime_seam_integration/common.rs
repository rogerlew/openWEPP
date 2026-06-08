use std::path::{Path, PathBuf};

pub(crate) use openwepp_hillslope_orchestrator::{
    HillslopePhaseGraph, HillslopePhaseScheduler, HillslopeWritebackSurface,
    runtime_inputs::{
        HillslopePlRuntimeSurfaces, HillslopeRuntimeInputError, SlopeRuntimeSurfaceOptions,
        build_hillslope_pl_runtime_surfaces_from_management,
        build_hillslope_runtime_surface_from_climate, build_hillslope_runtime_surface_from_frost,
        build_hillslope_runtime_surface_from_management,
        build_hillslope_runtime_surface_from_slope,
        build_hillslope_runtime_surface_from_slope_with_options,
        build_hillslope_runtime_surface_from_snow, build_hillslope_runtime_surface_from_soil,
    },
};

pub(crate) use openwepp_input_contract::parsers::{
    chaninp::{ChaninpParseOptions, parse_chaninp_from_str},
    climate::{ParserMode as ClimateParserMode, parse_climate_from_str},
    frost::{ParseMode as FrostParseMode, parse_frost_from_path},
    management::{
        ManagementParseOutput, ParseMode as ManagementParseMode, YearlyAnnualExtension,
        YearlyCroplandBranch, YearlyPerennialData, YearlyPerennialGrazingCycle, YearlyScenarioData,
        parse_management_from_path,
    },
    slope::{SlopeParserOptions, parse_slope_str},
    snow::{SnowParseOptions, parse_snow_file},
    soil::{SoilParserOptions, parse_soil},
};

pub(crate) use openwepp_kernel_contract::{
    BoundarySymbol, HillslopeAnnualDecompositionAction, HillslopeAnnualGrowthAction,
    HillslopeDecompositionTransitionControl, HillslopeGrowthTransitionControl, HillslopeKernel,
    HillslopeKernelRequest, HillslopePerennialDecompositionAction, HillslopePerennialGrowthAction,
    KernelRunResponse, KernelWritebackPayload, WatershedKernel, WatershedKernelRequest,
};

pub(crate) use openwepp_sim_contract::status::{SimulationPhase, SimulationStatus};
pub(crate) use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};
pub(crate) use openwepp_watershed_orchestrator::{
    execute_watershed_dispatch_with_kernel,
    runtime_inputs::{
        build_watershed_runtime_surface_from_chaninp,
        build_watershed_runtime_surface_from_climate_assignments,
    },
};

pub(crate) const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

pub(crate) const SOIL_VALID_9002: &str = include_str!("../../fixtures/infile/soil/valid_9002.sol");
pub(crate) const SOIL_VALID_7778: &str = include_str!("../../fixtures/infile/soil/valid_7778.sol");
pub(crate) const SLOPE_STRICT_VALID_CANONICAL: &str =
    include_str!("../../fixtures/infile/slope/strict_valid_canonical.slp");
pub(crate) const CHANINP_STRICT_VALID: &str =
    include_str!("../../fixtures/infile/chaninp/strict_valid.chaninp");
pub(crate) const CLIMATE_STRICT_VALID: &str =
    include_str!("../../fixtures/infile/climate/strict_valid.cli");
pub(crate) const CLIMATE_WC1_DAY1: &str =
    include_str!("../../fixtures/infile/climate/wc1_canoga_day1.cli");
pub(crate) const CLIMATE_WC1_STMDUR_CAP: &str =
    include_str!("../../fixtures/infile/climate/wc1_canoga_stmdur_cap.cli");

pub(crate) struct HillslopeSeedProbeKernel {
    pub(crate) invocation_count: usize,
}

impl HillslopeKernel for HillslopeSeedProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "solthk", 0.4);
        assert_state_value(request.state_surface, "dg", 0.1);
        let thetdr = request
            .state_surface
            .get(&BoundarySymbol::from("thetdr"))
            .expect("thetdr should be present")
            .as_f64();
        let thetfc = request
            .state_surface
            .get(&BoundarySymbol::from("thetfc"))
            .expect("thetfc should be present")
            .as_f64();
        assert!(thetdr.is_finite() && thetdr > 0.0);
        assert!(thetfc.is_finite() && thetfc > 0.0);
        assert!(thetfc >= thetdr);
        assert!(
            (thetdr - 0.05).abs() > 1.0e-9 || (thetfc - 0.31).abs() > 1.0e-9,
            "authoritative theta symbols should be correction-lineage projected, not raw parser-theta values"
        );
        assert_state_value(request.state_surface, "nsl", 2.0);
        assert_state_value(request.state_surface, "ssc", 11.5 / 3.6e6);
        assert_state_value(request.state_surface, "dg_0002", 0.15);
        assert_state_value(request.state_surface, "solthk_0002", 0.25);
        assert_state_value(request.state_surface, "wb19_dg_0002", 0.2);
        assert_state_value(request.state_surface, "wb19_solthk_0002", 0.4);
        assert_state_value(request.state_surface, "ssc_0002", 8.0 / 3.6e6);
        let profile_fc_store_mm = request
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_fc_store_mm"))
            .expect("wb13_profile_fc_store_mm should be present")
            .as_f64();
        let profile_wp_store_mm = request
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_wp_store_mm"))
            .expect("wb13_profile_wp_store_mm should be present")
            .as_f64();
        let wb11_nsl_raw = request.state_surface[&BoundarySymbol::from("wb11_nsl")]
            .as_f64()
            .round();
        let wb11_nsl = format!("{wb11_nsl_raw:.0}")
            .parse::<usize>()
            .expect("wb11_nsl should parse as usize");
        let mut layer_fc_store_mm = 0.0_f64;
        let mut layer_wp_store_mm = 0.0_f64;
        for layer_index in 1..=wb11_nsl {
            let dg = request.state_surface
                [&BoundarySymbol::from(format!("wb19_dg_{layer_index:04}"))]
                .as_f64();
            layer_fc_store_mm += request.state_surface
                [&BoundarySymbol::from(format!("wb19_thetfc_{layer_index:04}"))]
                .as_f64()
                * dg
                * 1_000.0;
            layer_wp_store_mm += request.state_surface
                [&BoundarySymbol::from(format!("wb19_thetdr_{layer_index:04}"))]
                .as_f64()
                * dg
                * 1_000.0;
        }
        assert!(
            (profile_fc_store_mm - layer_fc_store_mm).abs() < 1.0e-9,
            "HPHYS0254 seam: normalized primary WB11 layers must represent full profile FC storage"
        );
        assert!(
            (profile_wp_store_mm - layer_wp_store_mm).abs() < 1.0e-9,
            "HPHYS0254 seam: normalized primary WB11 layers must represent full profile WP storage"
        );

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "ARCH17-HS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

pub(crate) struct WatershedSeedProbeKernel {
    pub(crate) invocation_count: usize,
}

impl WatershedKernel for WatershedSeedProbeKernel {
    fn run_watershed_node(&mut self, request: &WatershedKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "ipeak", 3.0);
        assert_state_value(request.state_surface, "nchan", 2.0);
        assert_state_value(request.state_surface, "dtchr", 600.0);
        assert_state_value(request.state_surface, "ntchr", 144.0);
        assert_state_value(request.state_surface, "nchnum", 2.0);
        assert_state_value(request.flux_surface, "cbase", 0.000_001);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::WatershedKernel, "ARCH17-WS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

pub(crate) struct HillslopeClimateProbeKernel {
    pub(crate) invocation_count: usize,
}

pub(crate) struct HillslopeSlopeProbeKernel {
    pub(crate) invocation_count: usize,
}

pub(crate) struct HillslopeSlopeSoilProbeKernel {
    pub(crate) invocation_count: usize,
}

impl HillslopeKernel for HillslopeSlopeProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "nelem", 2.0);
        assert_state_value(request.state_surface, "nwsofe", 2.0);
        assert_state_value(request.state_surface, "nslpts", 3.0);
        assert_state_value(request.state_surface, "slplen", 60.0);
        assert_state_value(request.state_surface, "avgslp", 0.058);
        assert_state_value(request.state_surface, "xinput_0001", 0.0);
        assert_state_value(request.state_surface, "xinput_0002", 0.6);
        assert_state_value(request.state_surface, "slpinp_0002", 0.08);
        assert_state_value(request.state_surface, "ofe2_nslpts", 3.0);
        assert_state_value(request.state_surface, "ofe2_slplen", 40.0);
        assert_state_value(request.state_surface, "ofe2_avgslp", 0.0425);
        assert_state_value(request.state_surface, "ofe2_xinput_0003", 1.0);
        assert_state_value(request.state_surface, "ofe2_slpinp_0003", 0.03);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "ARCH17-HS-SLOPE-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

impl HillslopeKernel for HillslopeSlopeSoilProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "solthk", 0.4);
        assert_state_value(request.state_surface, "dg", 0.1);
        let thetdr = request
            .state_surface
            .get(&BoundarySymbol::from("thetdr"))
            .expect("thetdr should be present")
            .as_f64();
        let thetfc = request
            .state_surface
            .get(&BoundarySymbol::from("thetfc"))
            .expect("thetfc should be present")
            .as_f64();
        assert!(thetdr.is_finite() && thetdr > 0.0);
        assert!(thetfc.is_finite() && thetfc > 0.0);
        assert!(thetfc >= thetdr);
        assert!(
            (thetdr - 0.05).abs() > 1.0e-9 || (thetfc - 0.31).abs() > 1.0e-9,
            "authoritative theta symbols should be correction-lineage projected, not raw parser-theta values"
        );
        assert_state_value(request.state_surface, "nsl", 2.0);
        assert_state_value(request.state_surface, "ssc", 11.5 / 3.6e6);
        assert_state_value(request.state_surface, "ssc_0002", 8.0 / 3.6e6);
        assert_state_value(request.state_surface, "nelem", 2.0);
        assert_state_value(request.state_surface, "nwsofe", 2.0);
        assert_state_value(request.state_surface, "nslpts", 3.0);
        assert_state_value(request.state_surface, "slplen", 60.0);
        assert_state_value(request.state_surface, "avgslp", 0.058);
        assert_state_value(request.state_surface, "xinput_0002", 0.6);
        assert_state_value(request.state_surface, "slpinp_0002", 0.08);
        assert_state_value(request.state_surface, "ofe2_avgslp", 0.0425);
        assert_state_value(request.state_surface, "ofe2_xinput_0003", 1.0);
        let profile_fc_store_mm = request
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_fc_store_mm"))
            .expect("wb13_profile_fc_store_mm should be present")
            .as_f64();
        let profile_wp_store_mm = request
            .state_surface
            .get(&BoundarySymbol::from("wb13_profile_wp_store_mm"))
            .expect("wb13_profile_wp_store_mm should be present")
            .as_f64();
        assert!(profile_fc_store_mm.is_finite() && profile_fc_store_mm >= profile_wp_store_mm);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "ARCH17-HS-SLOPE-SOIL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

impl HillslopeKernel for HillslopeClimateProbeKernel {
    fn run_hillslope_phase(&mut self, request: &HillslopeKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "datver", 5.3);
        assert_state_value(request.state_surface, "iclig", 1.0);
        assert_state_value(request.state_surface, "itemp", 1.0);
        assert_state_value(request.state_surface, "ibrkpt", 0.0);
        assert_state_value(request.state_surface, "iwind", 0.0);
        assert_state_value(request.state_surface, "prcp", 0.01);
        assert_state_value(request.state_surface, "stmdur", 7_200.0);
        assert_state_value(request.state_surface, "timep", 0.25);
        assert_state_value(request.state_surface, "ip", 2.1);
        assert_state_at_least(request.state_surface, "ninten", 2.0);
        assert_state_value(request.state_surface, "timem_0001", 0.0);
        assert_state_value(request.state_surface, "tmax", 12.0);
        assert_state_value(request.state_surface, "tmin", 2.0);
        assert_state_value(request.state_surface, "rad", 200.0);
        assert_state_value(request.state_surface, "tdpt", -1.0);
        assert_state_value(request.state_surface, "vwind", 3.0);
        assert_state_value(request.state_surface, "obmaxt_0001", 1.0);
        assert_state_value(request.state_surface, "obmaxt_0012", 12.0);
        assert_state_value(request.state_surface, "obmint_0001", -5.0);
        assert_state_value(request.state_surface, "obmint_0012", 6.0);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "CLIM02-HS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}

pub(crate) struct WatershedClimateProbeKernel {
    pub(crate) invocation_count: usize,
}

impl WatershedKernel for WatershedClimateProbeKernel {
    fn run_watershed_node(&mut self, request: &WatershedKernelRequest<'_>) -> KernelRunResponse {
        assert_state_value(request.state_surface, "nclimhs", 3.0);
        assert_state_value(request.state_surface, "hs1_datver", 5.3);
        assert_state_value(request.state_surface, "hs2_datver", 5.3);
        assert_state_value(request.state_surface, "hs3_datver", 5.3);
        assert_state_value(request.state_surface, "hs1_prcp", 0.01);
        assert_state_value(request.state_surface, "hs2_stmdur", 7_200.0);
        assert_state_value(request.state_surface, "hs3_timep", 0.25);
        assert_state_value(request.state_surface, "hs1_ip", 2.1);
        assert_state_at_least(request.state_surface, "hs2_ninten", 2.0);
        assert_state_value(request.state_surface, "hs3_timem_0001", 0.0);
        assert_state_value(request.state_surface, "hs2_tmax", 12.0);
        assert_state_value(request.state_surface, "hs3_tmin", 2.0);

        self.invocation_count += 1;
        KernelRunResponse::new(
            SimulationStatus::ok(SimulationPhase::WatershedKernel, "CLIM02-WS-KERNEL-OK")
                .expect("status should construct"),
            KernelWritebackPayload::empty(),
        )
    }
}
pub(crate) fn assert_state_value(
    surface: &std::collections::BTreeMap<BoundarySymbol, openwepp_kernel_contract::BoundaryValue>,
    symbol: &str,
    expected: f64,
) {
    let value = surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing runtime symbol {symbol}"))
        .as_f64();
    assert!(
        (value - expected).abs() < 1e-12,
        "{symbol} mismatch: {value}"
    );
}

pub(crate) fn assert_state_at_least(
    surface: &std::collections::BTreeMap<BoundarySymbol, openwepp_kernel_contract::BoundaryValue>,
    symbol: &str,
    minimum: f64,
) {
    let value = surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing runtime symbol {symbol}"))
        .as_f64();
    assert!(
        value >= minimum,
        "{symbol} expected >= {minimum}, got {value}"
    );
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Pl16EquationSeed {
    pub(crate) ws: f64,
}

pub(crate) fn seed_pl16_equation_symbols(
    surface: &mut HillslopeWritebackSurface,
    seed: Pl16EquationSeed,
) {
    for (symbol, value) in [
        ("Ws", seed.ws),
        ("tmax", 25.0),
        ("tmin", 13.0),
        ("rad", 210.0),
    ] {
        surface
            .state_surface
            .insert(BoundarySymbol::from(symbol), value.into());
    }

    for (root, value) in [
        ("btemp", 10.0),
        ("otemp", 25.0),
        ("gddmax", 1700.0),
        ("dlai", 0.85),
        ("dropfc", 0.98),
        ("decfct", 0.65),
        ("spriod", 30.0),
        ("bb", 3.6),
        ("beinp", 35.00196),
        ("extnct", 0.65),
        ("hi", 0.5),
        ("xmxlai", 3.5),
        ("rsr", 0.25),
        ("rtmmax", 3.0),
        ("rdmax", 1.51995),
    ] {
        surface
            .state_surface
            .insert(BoundarySymbol::from(root), value.into());
        surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_growth_slot_0001_crop_0001_{root}")),
            value.into(),
        );
    }
}

pub(crate) fn seed_pl17_decomposition_symbols(surface: &mut HillslopeWritebackSurface) {
    for (symbol, value) in [("Ws", 0.8), ("tmax", 25.0), ("tmin", 13.0), ("prcp", 0.003)] {
        surface
            .state_surface
            .insert(BoundarySymbol::from(symbol), value.into());
    }
    for (root, value) in [("oratea", 0.0065), ("orater", 0.0065)] {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("pl_decomp_slot_0001_crop_0001_{root}")),
            value.into(),
        );
        surface
            .state_surface
            .insert(BoundarySymbol::from(root), value.into());
    }
}

pub(crate) fn merge_hillslope_runtime_surfaces(
    mut primary: HillslopeWritebackSurface,
    overlay: HillslopeWritebackSurface,
) -> HillslopeWritebackSurface {
    primary.state_surface.extend(overlay.state_surface);
    primary.flux_surface.extend(overlay.flux_surface);
    primary
}

pub(crate) fn parse_management_fixture(name: &str) -> ManagementParseOutput {
    parse_management_from_path(management_fixture_path(name), ManagementParseMode::Strict)
        .unwrap_or_else(|error| panic!("management fixture {name} should parse: {error}"))
}

pub(crate) fn management_fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("infile")
        .join("management")
        .join(name)
}

pub(crate) fn snow_fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("infile")
        .join("snow")
        .join(name)
}

pub(crate) fn frost_fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("infile")
        .join("frost")
        .join(name)
}

pub(crate) fn assert_full_pl_family_coverage(
    management: &ManagementParseOutput,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    assert_pl_ordering_flags(pl_surfaces);
    assert_pl_ofe_seed_coverage(management, pl_surfaces);
    assert_pl_slot_projection_coverage(management, pl_surfaces);
}

pub(crate) fn assert_pl_ordering_flags(pl_surfaces: &HillslopePlRuntimeSurfaces) {
    assert_state_value(
        &pl_surfaces.pl_schedule_surface,
        "pl_order_decomp_before_soil",
        1.0,
    );
    assert_state_value(
        &pl_surfaces.pl_schedule_surface,
        "pl_order_growth_after_decomp",
        1.0,
    );
    assert_state_value(
        &pl_surfaces.pl_schedule_surface,
        "pl_order_watbal_after_growth",
        1.0,
    );
}

pub(crate) fn assert_pl_ofe_seed_coverage(
    management: &ManagementParseOutput,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    for (ofe_position, initial_ref) in management.schedule.ofe_initial_refs.iter().enumerate() {
        let ofe_index = ofe_position + 1;
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_ofe{ofe_index}_initial_ref"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_ofe{ofe_index}_lanuse"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_growth_surface,
            &format!("pl_growth_ofe{ofe_index}_imngmt_seed"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_growth_surface,
            &format!("pl_growth_ofe{ofe_index}_rtyp_seed"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_decomp_surface,
            &format!("pl_decomp_ofe{ofe_index}_iresd_seed"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_decomp_surface,
            &format!("pl_decomp_ofe{ofe_index}_sumrtm_seed"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_decomp_surface,
            &format!("pl_decomp_ofe{ofe_index}_sumsrm_seed"),
        );

        let initial = &management.registries.initials[*initial_ref - 1];
        let openwepp_input_contract::parsers::management::InitialScenarioData::Cropland(data) =
            &initial.data;
        if data.understory_line.is_some() {
            assert_surface_has_symbol(
                &pl_surfaces.pl_decomp_surface,
                &format!("pl_decomp_ofe{ofe_index}_usinrcol_seed"),
            );
            assert_surface_has_symbol(
                &pl_surfaces.pl_decomp_surface,
                &format!("pl_decomp_ofe{ofe_index}_usrilcol_seed"),
            );
        }
    }
}

pub(crate) fn assert_pl_slot_projection_coverage(
    management: &ManagementParseOutput,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    for (slot_position, slot) in management.schedule.slots.iter().enumerate() {
        let slot_index = slot_position + 1;
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_slot_{slot_index:04}_rotation_index"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_slot_{slot_index:04}_year_in_rotation"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_slot_{slot_index:04}_ofe_index"),
        );
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_slot_{slot_index:04}_crop_slots"),
        );

        for (crop_slot_position, yearly_ref) in slot.yearly_refs.iter().enumerate() {
            let crop_slot_index = crop_slot_position + 1;
            let yearly = &management.registries.yearlies[*yearly_ref - 1];
            let YearlyScenarioData::Cropland(cropland) = &yearly.data;
            assert_slot_crop_schedule_symbols(slot_index, crop_slot_index, pl_surfaces);
            assert_slot_crop_growth_common_symbols(slot_index, crop_slot_index, pl_surfaces);
            assert_slot_crop_branch_symbols(
                slot_index,
                crop_slot_index,
                &cropland.branch,
                pl_surfaces,
            );
        }
    }
}

pub(crate) fn assert_slot_crop_schedule_symbols(
    slot_index: usize,
    crop_slot_index: usize,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    for schedule_root in [
        "yearly_ref",
        "lanuse",
        "itype",
        "tilseq",
        "conset",
        "drset",
        "imngmt",
    ] {
        assert_surface_has_symbol(
            &pl_surfaces.pl_schedule_surface,
            &format!("pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{schedule_root}"),
        );
    }
}

pub(crate) fn assert_slot_crop_growth_common_symbols(
    slot_index: usize,
    crop_slot_index: usize,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    for growth_root in [
        "itype", "imngmt", "btemp", "otemp", "gddmax", "dlai", "dropfc", "decfct", "spriod", "bb",
        "beinp", "extnct", "hi", "xmxlai", "rsr", "rtmmax", "rdmax",
    ] {
        assert_surface_has_symbol(
            &pl_surfaces.pl_growth_surface,
            &format!("pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{growth_root}"),
        );
    }
}

pub(crate) fn assert_slot_crop_branch_symbols(
    slot_index: usize,
    crop_slot_index: usize,
    branch: &YearlyCroplandBranch,
    pl_surfaces: &HillslopePlRuntimeSurfaces,
) {
    match branch {
        YearlyCroplandBranch::AnnualOrFallow(_) => {
            for growth_root in ["jdharv", "jdplt", "rw"] {
                assert_surface_has_symbol(
                    &pl_surfaces.pl_growth_surface,
                    &format!(
                        "pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{growth_root}"
                    ),
                );
            }
            for decomp_root in [
                "resmgt", "jdherb", "jdburn", "jdslge", "jdcut", "jdmove", "fbrnag", "fbrnog",
                "frcut", "frmove", "oratea", "orater",
            ] {
                assert_surface_has_symbol(
                    &pl_surfaces.pl_decomp_surface,
                    &format!(
                        "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{decomp_root}"
                    ),
                );
            }
        }
        YearlyCroplandBranch::Perennial(perennial) => {
            for growth_root in ["jdharv", "jdplt", "jdstop", "rw", "mgtopt"] {
                assert_surface_has_symbol(
                    &pl_surfaces.pl_growth_surface,
                    &format!(
                        "pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{growth_root}"
                    ),
                );
            }
            for decomp_root in ["mgtopt", "ncut", "ncycle", "oratea", "orater"] {
                assert_surface_has_symbol(
                    &pl_surfaces.pl_decomp_surface,
                    &format!(
                        "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{decomp_root}"
                    ),
                );
            }
            if perennial.mgtopt == 1 {
                for (position, _) in perennial.cut_days.iter().enumerate() {
                    let cut_index = position + 1;
                    assert_surface_has_symbol(
                        &pl_surfaces.pl_decomp_surface,
                        &format!(
                            "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_cutday_{cut_index:04}"
                        ),
                    );
                }
            }
            if perennial.mgtopt == 2 {
                for (position, _) in perennial.grazing_cycles.iter().enumerate() {
                    let cycle_index = position + 1;
                    for grazing_root in ["gday", "gend", "animal", "bodywt", "area", "digest"] {
                        assert_surface_has_symbol(
                            &pl_surfaces.pl_decomp_surface,
                            &format!(
                                "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{grazing_root}_{cycle_index:04}"
                            ),
                        );
                    }
                }
            }
        }
    }
}

pub(crate) fn assert_surface_has_symbol(
    surface: &std::collections::BTreeMap<BoundarySymbol, openwepp_kernel_contract::BoundaryValue>,
    symbol: &str,
) {
    assert!(
        surface.contains_key(&BoundarySymbol::from(symbol)),
        "missing projected runtime symbol {symbol}"
    );
}

pub(crate) fn assert_merged_pl_seed_aliases(
    surface: &std::collections::BTreeMap<BoundarySymbol, openwepp_kernel_contract::BoundaryValue>,
) {
    for symbol in [
        "lanuse",
        "itype",
        "imngmt",
        "jdharv",
        "jdplt",
        "jdherb",
        "jdburn",
        "jdslge",
        "jdcut",
        "jdmove",
        "fbrnag",
        "fbrnog",
        "frcut",
        "frmove",
        "rw",
        "resmgt",
        "sumgdd",
        "vdmt",
        "cancov",
        "lai",
        "rtmass",
        "rtd",
        "hia",
        "btemp",
        "otemp",
        "gddmax",
        "dlai",
        "dropfc",
        "decfct",
        "spriod",
        "bb",
        "beinp",
        "extnct",
        "hi",
        "xmxlai",
        "rsr",
        "rtmmax",
        "rdmax",
        "iresd_seed",
        "sumrtm_seed",
        "sumsrm_seed",
        "oratea",
        "orater",
    ] {
        assert_surface_has_symbol(surface, symbol);
    }
}

pub(crate) fn usize_to_scalar(value: usize) -> f64 {
    let value_u32 = u32::try_from(value)
        .unwrap_or_else(|_| panic!("value {value} exceeds lossless u32->f64 conversion"));
    f64::from(value_u32)
}
