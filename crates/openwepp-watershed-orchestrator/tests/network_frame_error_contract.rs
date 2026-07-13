use openwepp_input_contract::parsers::chaninp::ChaninpParseOutcome;
use openwepp_watershed_orchestrator::{
    WatershedNetworkFrameError, runtime_inputs::WatershedRuntimeInputError,
};

#[test]
fn watershed_network_frame_error_display_preserves_all_contract_identities() {
    let cases = [
        (
            WatershedNetworkFrameError::ChaninpNotRuntimeReady {
                observed: ChaninpParseOutcome::NotApplicable,
                chaninp_ipeak: 2,
                channel_ipeak: 4,
            },
            "WSHEDFRAME-E-001 chan.inp parse outcome NotApplicable is not runtime-ready for chaninp ipeak 2 and channel ipeak 4",
            false,
        ),
        (
            WatershedNetworkFrameError::MissingChaninpOptions,
            "WSHEDFRAME-E-002 chan.inp options are missing",
            false,
        ),
        (
            WatershedNetworkFrameError::ChannelIdOutOfRange { channel_id: 7 },
            "WSHEDFRAME-E-003 channel id 7 exceeds typed frame range",
            false,
        ),
        (
            WatershedNetworkFrameError::ImpoundmentIdOutOfRange {
                impoundment_index: 8,
            },
            "WSHEDFRAME-E-004 impoundment index 8 exceeds typed frame range",
            false,
        ),
        (
            WatershedNetworkFrameError::MissingSlopeProfile {
                channel_id: 9,
                slope_profile_count: 2,
            },
            "WSHEDFRAME-E-005 missing slope profile for channel 9; profile_count=2",
            false,
        ),
        (
            WatershedNetworkFrameError::RuntimeInput(
                WatershedRuntimeInputError::ImpoundmentSymbolOutOfDomain {
                    symbol: "qinf".to_string(),
                    value: -1.0,
                    rule: ">= 0",
                },
            ),
            "WSHEDFRAME-E-006 runtime input failed: WS-RUNTIME-E-012: impoundment runtime symbol qinf=-1 violates >= 0",
            true,
        ),
        (
            WatershedNetworkFrameError::MissingRoutedChannelState { node_id: 10 },
            "WSHEDFRAME-E-007 missing routed channel state for node 10",
            false,
        ),
        (
            WatershedNetworkFrameError::MissingRoutedImpoundmentState { node_id: 11 },
            "WSHEDFRAME-E-008 missing routed impoundment state for node 11",
            false,
        ),
        (
            WatershedNetworkFrameError::InvalidGroundwaterAuthority {
                field: "baseflow_threshold_area_ha",
                value: f64::NAN,
            },
            "WSHEDFRAME-E-009 invalid groundwater authority field baseflow_threshold_area_ha=NaN",
            false,
        ),
        (
            WatershedNetworkFrameError::InvalidTerminalPublication {
                node_id: 12,
                field: "peak_runoff_m3_s",
                value: -0.5,
            },
            "WSHEDFRAME-E-010 invalid terminal publication for channel 12: peak_runoff_m3_s=-0.5",
            false,
        ),
    ];

    for (error, expected_display, has_source) in cases {
        assert_eq!(error.to_string(), expected_display);
        assert_eq!(std::error::Error::source(&error).is_some(), has_source);
    }
}
