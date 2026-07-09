use openwepp_kernel_contract::{
    BoundarySymbol, ClimateForcingSymbolSurface, ClimateForcingSymbolSurfaceError,
    HillslopeIrrigationDepletionPeriodField, HillslopeIrrigationFixedDateEventField,
    HillslopeProductionFluxSymbol, HillslopeProductionStateSymbol,
    MAX_CLIMATE_FORCING_SERIES_POINTS, WatershedChannelFluxField, WatershedChannelStateField,
    WatershedImpoundmentFluxField, WatershedImpoundmentStateField, WatershedProductionFluxSymbol,
    WatershedProductionStateSymbol,
};
use std::fs;
use std::path::Path;

#[test]
fn arch22_hillslope_static_symbol_projection_matches_authority() {
    assert_eq!(
        BoundarySymbol::from(HillslopeProductionStateSymbol::Wb11SoilWater).as_str(),
        "wb11_soil_water"
    );
    assert_eq!(
        BoundarySymbol::from(HillslopeProductionStateSymbol::Wb12Infiltration).as_str(),
        "wb12_infiltration"
    );
    assert_eq!(
        BoundarySymbol::from(HillslopeProductionFluxSymbol::Wb11Et).as_str(),
        "ET"
    );
    assert_eq!(
        BoundarySymbol::from(HillslopeProductionFluxSymbol::Wb12RunoffQ).as_str(),
        "Q"
    );
}

#[test]
fn arch22_hillslope_dynamic_irrigation_symbol_projection_matches_authority() {
    let depletion =
        BoundarySymbol::from(HillslopeProductionStateSymbol::IrrigationDepletionPeriod {
            period_index: 7,
            field: HillslopeIrrigationDepletionPeriodField::SprinklerRateMetersPerSecond,
        });
    assert_eq!(
        depletion.as_str(),
        "irrigation.depletion.period_0007.sprinkler_rate_m_per_s"
    );

    let fixeddate =
        BoundarySymbol::from(HillslopeProductionStateSymbol::IrrigationFixedDateEvent {
            event_index: 3,
            field: HillslopeIrrigationFixedDateEventField::ScheduleTerminationFlag,
        });
    assert_eq!(
        fixeddate.as_str(),
        "irrigation.fixeddate.event_0003.schedule_termination_flag"
    );
}

#[test]
fn arch22_wb11_wb12_state_symbols_match_authority_snapshot() {
    use HillslopeProductionStateSymbol as H;

    let cases = [
        (H::Wb11SoilWater, "wb11_soil_water"),
        (H::Wb11EtDemand, "wb11_et_demand"),
        (H::Wb17ResidueInterception, "wb17_residue_interception"),
        (H::Wb11FieldCapacity, "wb11_field_capacity"),
        (H::Wb11PercFraction, "wb11_perc_fraction"),
        (H::Wb11LateralFraction, "wb11_lateral_fraction"),
        (H::Wb11DrainageFraction, "wb11_drainage_fraction"),
        (H::Wb11DrainageCoefficient, "wb11_drainage_coefficient"),
        (H::Wb11DrainableStorage, "wb11_drainable_storage"),
        (H::Wb12RainfallInput, "wb12_rainfall_input"),
        (H::Wb12RunonInput, "wb12_runon_input"),
        (H::Wb12Infiltration, "wb12_infiltration"),
        (
            H::Wb12DepressionStorageDelta,
            "wb12_depression_storage_delta",
        ),
        (H::Wb12RunoffObserved, "wb12_runoff_observed"),
        (
            H::Wb12RunoffClosureTolerance,
            "wb12_runoff_closure_tolerance",
        ),
        (H::Wb12RunoffReconciled, "wb12_runoff_reconciled"),
        (H::Wb12StorageInitial, "wb12_storage_initial"),
        (H::Wb12StorageObserved, "wb12_storage_observed"),
        (
            H::Wb12StorageClosureTolerance,
            "wb12_storage_closure_tolerance",
        ),
        (H::Wb12PrecipInput, "wb12_precip_input"),
        (H::Wb12StorageReconciled, "wb12_storage_reconciled"),
    ];

    for (symbol, expected) in cases {
        assert_eq!(BoundarySymbol::from(symbol).as_str(), expected);
    }
}

#[test]
fn arch22_irrigation_scalar_state_symbols_match_authority_snapshot() {
    use HillslopeProductionStateSymbol as H;

    let cases = [
        (H::IrrigRuntimeSource, "irrigation.runtime_schedule_source"),
        (H::IrrigRuntimeDepthMeters, "irrigation.runtime_depth_m"),
        (
            H::IrrigRuntimeDurationSeconds,
            "irrigation.runtime_duration_s",
        ),
        (
            H::IrrigRuntimeRateMetersPerSecond,
            "irrigation.runtime_rate_m_per_s",
        ),
        (H::IrrigRuntimeEventIndex, "irrigation.runtime_event_index"),
        (H::IrrigRuntimeSystemType, "irrigation.runtime_system_type"),
        (H::IrrigDepletionEnabled, "irrigation.depletion.enabled"),
        (
            H::IrrigDepletionSystemType,
            "irrigation.depletion.system_type",
        ),
        (
            H::IrrigDepletionMinDepthMeters,
            "irrigation.depletion.min_depth_m",
        ),
        (
            H::IrrigDepletionMaxDepthMeters,
            "irrigation.depletion.max_depth_m",
        ),
        (
            H::IrrigDepletionPeriodCount,
            "irrigation.depletion.period_count",
        ),
        (H::IrrigFixedDateEnabled, "irrigation.fixeddate.enabled"),
        (
            H::IrrigFixedDateSystemType,
            "irrigation.fixeddate.system_type",
        ),
        (
            H::IrrigFixedDateEventCount,
            "irrigation.fixeddate.event_count",
        ),
    ];

    for (symbol, expected) in cases {
        assert_eq!(BoundarySymbol::from(symbol).as_str(), expected);
    }
}

#[test]
fn arch22_plant_hyetograph_soil_state_symbols_match_authority_snapshot() {
    use HillslopeProductionStateSymbol as H;

    let cases = [
        (H::Wb15PlantCancov, "cancov"),
        (H::Wb15PlantLai, "lai"),
        (H::Wb15PlantVdmt, "vdmt"),
        (H::Wb14HyetographNinten, "ninten"),
        (H::Wb14HyetographNbrkpt, "nbrkpt"),
        (H::Wb14SoilConductivity, "ssc"),
        (H::Wb14SoilLayerDepth, "dg"),
        (H::Wb14SoilThetaResidual, "thetdr"),
        (H::Wb14SoilThetaFieldCapacity, "thetfc"),
    ];

    for (symbol, expected) in cases {
        assert_eq!(BoundarySymbol::from(symbol).as_str(), expected);
    }
}

#[test]
fn arch22_snow_frost_state_symbols_match_authority_snapshot() {
    use HillslopeProductionStateSymbol as H;

    let cases = [
        (H::Wb14SnowFilePresent, "snow.options.snow_file_present"),
        (H::Wb14SnowRst, "snow.options.rst"),
        (H::Wb14SnowNewsnw, "snow.options.newsnw"),
        (H::Wb14SnowSsd, "snow.options.ssd"),
        (H::Wb14SnowRuntimeSwe, "snow.runtime_swe"),
        (H::Wb14FrostFilePresent, "frost.options.frost_file_present"),
        (H::Wb14FrostWintRed, "frost.options.wintRed"),
        (H::Wb14FrostFineTop, "frost.options.fineTop"),
        (H::Wb14FrostFineBot, "frost.options.fineBot"),
        (H::Wb14FrostKsnowf, "frost.options.ksnowf"),
        (H::Wb14FrostKresf, "frost.options.kresf"),
        (H::Wb14FrostKsoilf, "frost.options.ksoilf"),
        (H::Wb14FrostKfactor1, "frost.options.kfactor1"),
        (H::Wb14FrostKfactor2, "frost.options.kfactor2"),
        (H::Wb14FrostKfactor3, "frost.options.kfactor3"),
        (H::Wb14FrostRuntimeDfrost, "frost.runtime_dfrost"),
        (H::Wb14FrostRuntimeDthaw, "frost.runtime_dthaw"),
        (H::Wb14FrostRuntimeNft, "frost.runtime_nft"),
        (H::Wb14FrostRuntimeWsFrz, "frost.runtime_ws_frz"),
        (H::Wb14FrostRuntimeInfcapFrz, "frost.runtime_infcap_frz"),
    ];

    for (symbol, expected) in cases {
        assert_eq!(BoundarySymbol::from(symbol).as_str(), expected);
    }
}

#[test]
fn arch22_peak_method_state_symbols_match_authority_snapshot() {
    use HillslopeProductionStateSymbol as H;

    let cases = [
        (H::Wb14Tmax, "tmax"),
        (H::Wb14Tmin, "tmin"),
        (H::Wb16Timep, "timep"),
        (H::Wb16Efflen, "efflen"),
        (H::Wb16Ealpha, "ealpha"),
        (H::Wb16ExponentM, "m"),
        (H::Wb16Peakro, "peakro"),
        (H::Wb16Watdur, "watdur"),
        (H::Wb16MethodBranch, "wb16_peak_method_branch"),
        (H::Wb16Tstar, "wb16_tstar"),
        (H::Wb16Qpstar, "wb16_qpstar"),
        (H::Wb16Vstar, "wb16_vstar"),
    ];

    for (symbol, expected) in cases {
        assert_eq!(BoundarySymbol::from(symbol).as_str(), expected);
    }
}

#[test]
fn arch22_all_hillslope_flux_symbol_projections_match_authority_snapshot() {
    let cases = [
        (HillslopeProductionFluxSymbol::Wb11Et, "ET"),
        (HillslopeProductionFluxSymbol::Wb11Ws, "Ws"),
        (
            HillslopeProductionFluxSymbol::Wb17PlantTranspirationEp,
            "Ep",
        ),
        (HillslopeProductionFluxSymbol::Wb17SoilEvaporationEs, "Es"),
        (
            HillslopeProductionFluxSymbol::Wb17ResidueEvaporationEr,
            "Er",
        ),
        (HillslopeProductionFluxSymbol::Wb11PercLossD, "D"),
        (HillslopeProductionFluxSymbol::Wb11PercRechargePe, "Pe"),
        (HillslopeProductionFluxSymbol::Wb11LateralQ, "q"),
        (HillslopeProductionFluxSymbol::Wb11DrainageQdd, "Qdd"),
        (HillslopeProductionFluxSymbol::Wb11SubhydQd, "Qd"),
        (
            HillslopeProductionFluxSymbol::Wb12RunoffClosureDelta,
            "wb12_runoff_closure_delta",
        ),
        (HillslopeProductionFluxSymbol::Wb12RunoffQ, "Q"),
        (HillslopeProductionFluxSymbol::Wb12SnowCouplingS, "S"),
        (
            HillslopeProductionFluxSymbol::Wb12StorageClosureDelta,
            "wb12_storage_closure_delta",
        ),
        (HillslopeProductionFluxSymbol::Wb15InterceptionI, "I"),
        (HillslopeProductionFluxSymbol::IrrigDailyIrrigation, "Irr"),
    ];

    for (symbol, expected) in cases {
        assert_eq!(BoundarySymbol::from(symbol).as_str(), expected);
    }
}

#[test]
fn arch22_all_dynamic_irrigation_field_suffixes_match_authority_snapshot() {
    let depletion_fields = [
        (
            HillslopeIrrigationDepletionPeriodField::ElementId,
            "element_id",
        ),
        (
            HillslopeIrrigationDepletionPeriodField::StartDoy,
            "start_doy",
        ),
        (
            HillslopeIrrigationDepletionPeriodField::StartYear,
            "start_year",
        ),
        (HillslopeIrrigationDepletionPeriodField::EndDoy, "end_doy"),
        (HillslopeIrrigationDepletionPeriodField::EndYear, "end_year"),
        (
            HillslopeIrrigationDepletionPeriodField::DepletionTriggerRatio,
            "depletion_trigger_ratio",
        ),
        (
            HillslopeIrrigationDepletionPeriodField::SprinklerDepthRatio,
            "sprinkler_depth_ratio",
        ),
        (
            HillslopeIrrigationDepletionPeriodField::SprinklerRateMetersPerSecond,
            "sprinkler_rate_m_per_s",
        ),
        (
            HillslopeIrrigationDepletionPeriodField::SprinklerNozzleFactor,
            "sprinkler_nozzle_factor",
        ),
    ];

    for (field, suffix) in depletion_fields {
        let symbol =
            BoundarySymbol::from(HillslopeProductionStateSymbol::IrrigationDepletionPeriod {
                period_index: 7,
                field,
            });
        assert_eq!(
            symbol.as_str(),
            format!("irrigation.depletion.period_0007.{suffix}")
        );
    }

    let fixeddate_fields = [
        (HillslopeIrrigationFixedDateEventField::OfeId, "ofe_id"),
        (HillslopeIrrigationFixedDateEventField::Day, "day"),
        (HillslopeIrrigationFixedDateEventField::Year, "year"),
        (
            HillslopeIrrigationFixedDateEventField::ScheduleTerminationFlag,
            "schedule_termination_flag",
        ),
        (
            HillslopeIrrigationFixedDateEventField::SprinklerDepthMeters,
            "sprinkler_depth_m",
        ),
        (
            HillslopeIrrigationFixedDateEventField::SprinklerRateMetersPerSecond,
            "sprinkler_rate_m_per_s",
        ),
        (
            HillslopeIrrigationFixedDateEventField::SprinklerNozzleFactor,
            "sprinkler_nozzle_factor",
        ),
    ];

    for (field, suffix) in fixeddate_fields {
        let symbol =
            BoundarySymbol::from(HillslopeProductionStateSymbol::IrrigationFixedDateEvent {
                event_index: 3,
                field,
            });
        assert_eq!(
            symbol.as_str(),
            format!("irrigation.fixeddate.event_0003.{suffix}")
        );
    }
}

#[test]
fn arch22_climate_forcing_surface_error_display_matches_authority_snapshot() {
    let count = MAX_CLIMATE_FORCING_SERIES_POINTS + 1;
    let error = ClimateForcingSymbolSurface::hillslope(count)
        .expect_err("oversized climate forcing surface must fail closed");

    assert_eq!(
        error,
        ClimateForcingSymbolSurfaceError::PointCountOutOfRange {
            count,
            supported_max: MAX_CLIMATE_FORCING_SERIES_POINTS,
        }
    );
    assert_eq!(
        error.to_string(),
        format!(
            "climate forcing point count {count} exceeds supported maximum {MAX_CLIMATE_FORCING_SERIES_POINTS}"
        )
    );
}

#[test]
fn arch22_climate_forcing_symbol_surface_accessors_match_authority_snapshot() {
    let hillslope = ClimateForcingSymbolSurface::hillslope(2)
        .expect("small hillslope climate forcing surface should build");

    assert_eq!(hillslope.point_count(), 2);
    assert_eq!(hillslope.timem_symbols()[0].as_str(), "timem_0001");
    assert_eq!(hillslope.timem_symbols()[1].as_str(), "timem_0002");
    assert_eq!(hillslope.intsty_symbols()[0].as_str(), "intsty_0001");
    assert_eq!(hillslope.intsty_symbols()[1].as_str(), "intsty_0002");

    let watershed = ClimateForcingSymbolSurface::watershed_hillslope(21, 2)
        .expect("small watershed hillslope climate forcing surface should build");

    assert_eq!(watershed.point_count(), 2);
    assert_eq!(watershed.timem_symbols()[0].as_str(), "hs21_timem_0001");
    assert_eq!(watershed.timem_symbols()[1].as_str(), "hs21_timem_0002");
    assert_eq!(watershed.intsty_symbols()[0].as_str(), "hs21_intsty_0001");
    assert_eq!(watershed.intsty_symbols()[1].as_str(), "hs21_intsty_0002");
}

#[test]
fn arch22_watershed_node_scoped_symbol_projection_matches_authority() {
    assert_eq!(
        BoundarySymbol::from(WatershedProductionStateSymbol::Dtchr).as_str(),
        "dtchr"
    );
    assert_eq!(
        BoundarySymbol::from(WatershedProductionStateSymbol::Nchnum).as_str(),
        "nchnum"
    );
    assert_eq!(
        BoundarySymbol::from(WatershedProductionStateSymbol::Ipeak).as_str(),
        "ipeak"
    );
    assert_eq!(
        BoundarySymbol::from(WatershedProductionFluxSymbol::Cbase).as_str(),
        "cbase"
    );

    let channel_roughness = BoundarySymbol::from(WatershedProductionStateSymbol::ChannelNode {
        node_id: 11,
        field: WatershedChannelStateField::Chnn,
    });
    assert_eq!(channel_roughness.as_str(), "ws10_channel_11_chnn");

    let impoundment_h = BoundarySymbol::from(WatershedProductionStateSymbol::ImpoundmentNode {
        node_id: 5,
        field: WatershedImpoundmentStateField::H,
    });
    assert_eq!(impoundment_h.as_str(), "ws10_impoundment_5_h");
}

#[test]
fn arch22_all_watershed_channel_symbols_match_authority_snapshot() {
    let state_fields = [
        (WatershedChannelStateField::Chnn, "chnn"),
        (WatershedChannelStateField::Ctlslp, "ctlslp"),
        (WatershedChannelStateField::Chnk, "chnk"),
        (WatershedChannelStateField::Qpo, "qpo"),
        (WatershedChannelStateField::Durrof, "durrof"),
    ];

    for (field, suffix) in state_fields {
        assert_eq!(field.as_str(), suffix);
        let symbol = BoundarySymbol::from(WatershedProductionStateSymbol::ChannelNode {
            node_id: 11,
            field,
        });
        assert_eq!(symbol.as_str(), format!("ws10_channel_11_{suffix}"));
    }

    assert_eq!(WatershedChannelFluxField::Roff.as_str(), "roff");
    let flux_symbol = BoundarySymbol::from(WatershedProductionFluxSymbol::ChannelNode {
        node_id: 11,
        field: WatershedChannelFluxField::Roff,
    });
    assert_eq!(flux_symbol.as_str(), "ws10_channel_11_roff");
}

#[test]
fn arch22_all_watershed_impoundment_symbols_match_authority_snapshot() {
    let state_fields = [
        (WatershedImpoundmentStateField::H, "h"),
        (WatershedImpoundmentStateField::Hfull, "hfull"),
        (WatershedImpoundmentStateField::Deltat, "deltat"),
        (WatershedImpoundmentStateField::Qinf, "qinf"),
        (WatershedImpoundmentStateField::Qo, "qo"),
        (WatershedImpoundmentStateField::Durout, "durout"),
        (WatershedImpoundmentStateField::Hnext, "hnext"),
    ];

    for (field, suffix) in state_fields {
        assert_eq!(field.as_str(), suffix);
        let symbol = BoundarySymbol::from(WatershedProductionStateSymbol::ImpoundmentNode {
            node_id: 5,
            field,
        });
        assert_eq!(symbol.as_str(), format!("ws10_impoundment_5_{suffix}"));
    }

    assert_eq!(
        WatershedImpoundmentFluxField::OutflowVolume.as_str(),
        "outflow_volume"
    );
    let flux_symbol = BoundarySymbol::from(WatershedProductionFluxSymbol::ImpoundmentNode {
        node_id: 5,
        field: WatershedImpoundmentFluxField::OutflowVolume,
    });
    assert_eq!(flux_symbol.as_str(), "ws10_impoundment_5_outflow_volume");
}

#[test]
fn arch22_watershed_hillslope_payload_symbol_projection_matches_authority() {
    let hillslope_peak =
        BoundarySymbol::from(WatershedProductionStateSymbol::HillslopeContributorPeak {
            hillslope_id: 21,
        });
    assert_eq!(hillslope_peak.as_str(), "hs21_peakro");

    let hillslope_duration = BoundarySymbol::from(
        WatershedProductionStateSymbol::HillslopeContributorDuration { hillslope_id: 21 },
    );
    assert_eq!(hillslope_duration.as_str(), "hs21_watdur");

    let total_detachment = BoundarySymbol::from(
        WatershedProductionStateSymbol::HillslopeContributorTotalDetachmentKg { hillslope_id: 21 },
    );
    assert_eq!(total_detachment.as_str(), "hs21_total_detachment_kg");

    let total_deposition = BoundarySymbol::from(
        WatershedProductionStateSymbol::HillslopeContributorTotalDepositionKg { hillslope_id: 21 },
    );
    assert_eq!(total_deposition.as_str(), "hs21_total_deposition_kg");

    let particle_class_count = BoundarySymbol::from(
        WatershedProductionStateSymbol::HillslopeContributorParticleClassCount { hillslope_id: 21 },
    );
    assert_eq!(particle_class_count.as_str(), "hs21_particle_class_count");

    let concentration = BoundarySymbol::from(
        WatershedProductionStateSymbol::HillslopeContributorSedimentConcentrationKgM3 {
            hillslope_id: 21,
            class_index: 3,
        },
    );
    assert_eq!(
        concentration.as_str(),
        "hs21_sediment_concentration_kg_m3_0003"
    );

    let diameter = BoundarySymbol::from(
        WatershedProductionStateSymbol::HillslopeContributorParticleDiameterMeters {
            hillslope_id: 21,
            class_index: 3,
        },
    );
    assert_eq!(diameter.as_str(), "hs21_particle_diameter_m_0003");

    let fraction = BoundarySymbol::from(
        WatershedProductionStateSymbol::HillslopeContributorParticleFlowFraction {
            hillslope_id: 21,
            class_index: 3,
        },
    );
    assert_eq!(fraction.as_str(), "hs21_particle_flow_fraction_0003");
}

#[test]
fn arch22_hillslope_guard_accessor_signature_is_typed() {
    let source_root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/openwepp-hillslope-orchestrator/src");

    assert!(
        !source_tree_contains_rs(
            &source_root,
            "symbol: &'static str,\n    ) -> Result<f64, Wb11HydrologyKernelGuardError>"
        ),
        "Wb11 guard accessors must not accept raw string symbol parameters"
    );
}

#[test]
fn arch22_watershed_guard_accessor_signature_is_typed() {
    let source = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/crates/openwepp-watershed-orchestrator/src/lib.rs"
    ))
    .expect("watershed orchestrator source should be readable");

    assert!(
        !source.contains("symbol: &str,\n    ) -> Result<f64, Ws10GuardError>"),
        "Ws10 guard accessors must not accept raw string symbol parameters"
    );
}

fn source_tree_contains_rs(root: &Path, needle: &str) -> bool {
    fs::read_dir(root)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .any(|entry| {
            let path = entry.path();
            if path.is_dir() {
                return source_tree_contains_rs(&path, needle);
            }
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                return false;
            }
            fs::read_to_string(path).is_ok_and(|contents| contents.contains(needle))
        })
}
