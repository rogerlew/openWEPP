fn stage3_publication_parity_input(
) -> openwepp_hillslope_orchestrator::DirectActiveSnowPartitionInputs {
    use openwepp_hillslope_orchestrator as hw;

    let mut layer = hw::DirectSnowLayerState::new(0.18, 0.40, 450.0, 12.0);
    layer.temperature_c = -8.0;
    layer.cold_content_j_m2 = 0.18 * 1_000.0 * 2_100.0 * 8.0;
    let hourly = [hw::DirectSnowHourlyForcing {
        radiation_mj_m2: 0.2,
        air_temperature_c: -5.0,
        ..hw::DirectSnowHourlyForcing::zero()
    }; 24];
    hw::DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.0,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.18,
        runtime_depth_m: 0.40,
        runtime_density_kg_m3: 450.0,
        runtime_settle_day_count: 12.0,
        liquid_water_retained_m: 0.0,
        tmax_c: -3.0,
        tmin_c: -7.0,
        canopy_cover_fraction: 0.45,
        wind_m_s: 3.0,
        dewpoint_c: -15.0,
        snow_melt_model: hw::SnowMeltModel::CoeLiquidHoldingCapacityV1,
        snow_density_model: hw::SnowDensityModel::PhysicsBulkDensityCompactionV1,
        stage3_liquid_routing_model: hw::SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
        surface_energy_options: hw::DirectSnowSurfaceEnergyOptions {
            longwave_model: hw::SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
            sublimation_model: hw::SnowSurfaceSublimationModel::Disabled,
            daily_solar_radiation_mj_m2: 5.0,
            daily_extraterrestrial_radiation_mj_m2: 10.0,
            daylight: true,
            atmospheric_pressure_pa: 101_324.6,
            turbulent_geometry: hw::DirectSnowTurbulentGeometry::CLIGEN_V1,
            complete_carrier_shadow: false,
        },
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: 0.40,
        coe_boundary_density_kg_m3: 450.0,
        coe_boundary_settle_day_count: 12.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: vec![layer],
        underlying_surface_albedo: 0.2,
        hourly,
    }
}

fn stage3_publication_bytes(
    frame: &DirectRunPublicationFrame,
    suffix: &str,
) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let root = std::env::temp_dir().join(format!(
        "openwepp-stage3-publication-parity-{}-{suffix}",
        std::process::id()
    ));
    std::fs::create_dir(&root).expect("create publication parity directory");
    let wat_path = root.join("output.wat.parquet");
    let pass_path = root.join("output.pass.parquet");

    let wat_rows = build_hillslope_wat_rows_from_direct_publication(frame)
        .expect("build Stage 3 parity WAT rows");
    let mut wat_writer =
        HillslopeWatParquetRowGroupWriter::create(&wat_path, InterchangeVersion::default())
            .expect("create Stage 3 parity WAT writer");
    wat_writer
        .write_rows(&wat_rows)
        .expect("write Stage 3 parity WAT rows");
    wat_writer.close().expect("close Stage 3 parity WAT writer");

    let pass_rows = build_hillslope_pass_rows_from_direct_publication(frame)
        .expect("build Stage 3 parity PASS rows");
    let mut pass_writer =
        HillslopePassParquetRowGroupWriter::create(&pass_path, InterchangeVersion::default())
            .expect("create Stage 3 parity PASS writer");
    pass_writer
        .write_rows(&pass_rows)
        .expect("write Stage 3 parity PASS rows");
    pass_writer
        .close()
        .expect("close Stage 3 parity PASS writer");

    let hbp = build_hbp_output_from_direct_publication(Path::new("H2637.hbp"), frame)
        .expect("build Stage 3 parity HBP");
    let wat = std::fs::read(&wat_path).expect("read Stage 3 parity WAT bytes");
    let pass = std::fs::read(&pass_path).expect("read Stage 3 parity PASS bytes");
    std::fs::remove_file(wat_path).expect("remove Stage 3 parity WAT");
    std::fs::remove_file(pass_path).expect("remove Stage 3 parity PASS");
    std::fs::remove_dir(root).expect("remove Stage 3 parity directory");
    (wat, hbp, pass)
}

#[test]
fn stage3_evaluation_preserves_exact_wat_hbp_and_pass_bytes() {
    use openwepp_hillslope_orchestrator::{
        SnowStage3EvaluationOperator, Wb11HydrologyKernel,
    };

    let inputs = stage3_publication_parity_input();
    let disabled = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect("disabled Stage 3 publication partition");
    let enabled = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
        &inputs,
        SnowStage3EvaluationOperator::SameStatePairedCarrierV1,
    )
    .expect("enabled Stage 3 publication partition");
    assert!(enabled.evaluation.is_some());
    assert_eq!(enabled.authoritative, disabled);

    let mut disabled_frame = r6a_direct_projection_fixture_frame();
    disabled_frame.rows[0].storage.snow_water_mm = disabled.runtime_swe_after_m * 1_000.0;
    disabled_frame.rows[0].storage.snow_depth_mm = disabled.runtime_depth_after_m * 1_000.0;
    let mut enabled_frame = r6a_direct_projection_fixture_frame();
    enabled_frame.rows[0].storage.snow_water_mm =
        enabled.authoritative.runtime_swe_after_m * 1_000.0;
    enabled_frame.rows[0].storage.snow_depth_mm =
        enabled.authoritative.runtime_depth_after_m * 1_000.0;

    let disabled_bytes = stage3_publication_bytes(&disabled_frame, "disabled");
    let enabled_bytes = stage3_publication_bytes(&enabled_frame, "enabled");
    assert_eq!(enabled_bytes.0, disabled_bytes.0, "WAT bytes changed");
    assert_eq!(enabled_bytes.1, disabled_bytes.1, "HBP bytes changed");
    assert_eq!(enabled_bytes.2, disabled_bytes.2, "PASS bytes changed");
}
