use std::{io, path::PathBuf};

use openwepp_runner::{HillslopeCliError, SnowbenchError};

#[test]
fn snowbench_error_display_preserves_all_contract_identities() {
    let cases = [
        (
            SnowbenchError::Io {
                path: PathBuf::from("outputs/forcing.csv"),
                source: io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"),
            },
            "SNOWBENCH-E-001 io error at outputs/forcing.csv: permission denied",
            true,
        ),
        (
            SnowbenchError::Json {
                path: PathBuf::from("outputs/report.json"),
                source: serde_json::Error::io(io::Error::other("json failure")),
            },
            "SNOWBENCH-E-002 failed to write JSON at outputs/report.json: json failure",
            true,
        ),
        (
            SnowbenchError::Runner {
                source: HillslopeCliError::MissingArgument {
                    argument: "--run-dir",
                },
            },
            "SNOWBENCH-E-003 runner input error: CLIHILL-E-001 missing required argument --run-dir",
            true,
        ),
        (
            SnowbenchError::ClimateRuntime {
                detail: "missing hourly forcing".to_string(),
            },
            "SNOWBENCH-E-004 climate runtime forcing error: missing hourly forcing",
            false,
        ),
        (
            SnowbenchError::InvalidInput {
                detail: "no run file".to_string(),
            },
            "SNOWBENCH-E-005 invalid input: no run file",
            false,
        ),
        (
            SnowbenchError::InvalidForcing {
                detail: "negative precipitation".to_string(),
            },
            "SNOWBENCH-E-006 invalid forcing: negative precipitation",
            false,
        ),
        (
            SnowbenchError::OpenweppSnow {
                detail: "missing snow_water_mm".to_string(),
            },
            "SNOWBENCH-E-007 openWEPP snow diagnostic error: missing snow_water_mm",
            false,
        ),
    ];

    for (error, expected_display, has_source) in cases {
        assert_eq!(error.to_string(), expected_display);
        assert_eq!(std::error::Error::source(&error).is_some(), has_source);
    }
}
