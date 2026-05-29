use openwepp_kernel_contract::{
    BoundarySymbol, HillslopeIrrigationDepletionPeriodField,
    HillslopeIrrigationFixedDateEventField, HillslopeProductionFluxSymbol,
    HillslopeProductionStateSymbol, WatershedChannelStateField, WatershedImpoundmentStateField,
    WatershedProductionFluxSymbol, WatershedProductionStateSymbol,
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
fn arch22_watershed_node_scoped_symbol_projection_matches_authority() {
    assert_eq!(
        BoundarySymbol::from(WatershedProductionStateSymbol::Dtchr).as_str(),
        "dtchr"
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
