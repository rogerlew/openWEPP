use std::error::Error;
use std::io;
#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;

use openwepp_legacy_bridge::policy::CompatibilityPolicy;
use openwepp_legacy_bridge::sidecar::{
    SidecarAdapterError, SidecarAdapterRequest, SidecarContract, SidecarDiscovery, SidecarId,
    SidecarRequirement, SidecarWarningCode, adapt_sidecar_bindings,
};
use openwepp_runner::{
    BinaryRole, HillslopeCliError, ReleaseLintError, ReleaseMetadataError, RunnerError,
};

const RUNNER_CONTRACT: &str = include_str!("../../docs/contracts/openwepp-runner-contract.md");
const BINARY_RELEASE_CONTRACT: &str =
    include_str!("../../docs/contracts/openwepp-binary-release-contract.md");
const HILLSLOPE_CLI_SPEC: &str = include_str!(
    "../../docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md"
);

fn sidecar_contract(id: &str, file_name: &str, requirement: SidecarRequirement) -> SidecarContract {
    SidecarContract::new(
        SidecarId::new(id).expect("valid sidecar id"),
        file_name,
        Vec::new(),
        requirement,
    )
}

fn sidecar_discovery(file_name: &str) -> SidecarDiscovery {
    SidecarDiscovery::new(file_name, PathBuf::from(format!("/tmp/{file_name}")))
}

fn json_error() -> serde_json::Error {
    serde_json::from_str::<serde_json::Value>("{").expect_err("invalid json should fail")
}

fn assert_error_text(message: &str, expected_code: &str, expected_fragments: &[&str]) {
    assert!(
        message.starts_with(expected_code),
        "message should start with {expected_code}: {message}"
    );
    for fragment in expected_fragments {
        assert!(
            message.contains(fragment),
            "message should contain {fragment:?}: {message}"
        );
    }
}

fn assert_hillslope_error_cases(cases: Vec<(HillslopeCliError, &str, Vec<&str>, bool)>) {
    for (error, expected_code, fragments, has_source) in cases {
        assert_eq!(error.code(), expected_code);
        assert_error_text(&error.to_string(), expected_code, &fragments);
        assert_eq!(
            error.source().is_some(),
            has_source,
            "source expectation for {expected_code}"
        );
    }
}

#[test]
fn cli01_contract_surface_declares_runner_commands_and_error_ids() {
    assert!(RUNNER_CONTRACT.contains("open_wepp_runner run-hillslope ..."));
    assert!(RUNNER_CONTRACT.contains("open_wepp_runner release lint --release-dir <path>"));
    assert!(RUNNER_CONTRACT.contains("RUNNER-E-001"));
    assert!(RUNNER_CONTRACT.contains("RUNNER-E-006"));
}

#[test]
fn cli01_contract_surface_declares_required_hillslope_sidecars_and_manifest_schema() {
    for required in [
        "frost.txt",
        "snow.txt",
        "wepp_ui.txt",
        "pmetpara.txt",
        "openwepp-hillslope-run-manifest-v1",
    ] {
        assert!(
            HILLSLOPE_CLI_SPEC.contains(required),
            "hillslope CLI spec should contain {required}"
        );
    }
}

#[test]
fn cli01_contract_surface_declares_release_sidecar_validation_fields() {
    for field in ["schema_valid", "release_lint_level", "validated_utc"] {
        assert!(
            BINARY_RELEASE_CONTRACT.contains(field),
            "binary release contract should require {field}"
        );
    }
}

#[test]
fn cli01_sidecar_contract_missing_required_sidecar_is_hard_failure_in_strict() {
    let request = SidecarAdapterRequest {
        policy: CompatibilityPolicy::Strict,
        contracts: vec![sidecar_contract(
            "snow",
            "snow.txt",
            SidecarRequirement::Required,
        )],
        discovered: Vec::new(),
    };

    let error = adapt_sidecar_bindings(&request).expect_err("missing required sidecar must fail");
    assert!(matches!(
        error,
        SidecarAdapterError::MissingRequiredSidecar { .. }
    ));
    assert_eq!(error.code(), "LSB-E-007");
}

#[test]
fn cli01_sidecar_contract_missing_required_sidecar_is_hard_failure_in_compat() {
    let request = SidecarAdapterRequest {
        policy: CompatibilityPolicy::Compat,
        contracts: vec![sidecar_contract(
            "frost",
            "frost.txt",
            SidecarRequirement::Required,
        )],
        discovered: Vec::new(),
    };

    let error = adapt_sidecar_bindings(&request).expect_err("missing required sidecar must fail");
    assert!(matches!(
        error,
        SidecarAdapterError::MissingRequiredSidecar { .. }
    ));
    assert_eq!(error.code(), "LSB-E-007");
}

#[test]
fn cli01_sidecar_contract_unknown_discovery_warns_in_compat() {
    let request = SidecarAdapterRequest {
        policy: CompatibilityPolicy::Compat,
        contracts: vec![sidecar_contract(
            "wepp_ui",
            "wepp_ui.txt",
            SidecarRequirement::Required,
        )],
        discovered: vec![
            sidecar_discovery("wepp_ui.txt"),
            sidecar_discovery("mystery_sidecar.txt"),
        ],
    };

    let response = adapt_sidecar_bindings(&request).expect("compat unknown should warn");
    assert_eq!(response.bindings.len(), 1);
    assert_eq!(response.warnings.len(), 1);
    assert_eq!(
        response.warnings[0].code,
        SidecarWarningCode::UnknownSidecarIgnored
    );
    assert_eq!(response.warnings[0].code.message_id(), "LSB-W-002");
}

#[test]
fn runner_release_metadata_errors_preserve_codes_display_and_sources() {
    let io_error = ReleaseMetadataError::Io {
        path: PathBuf::from("/tmp/release.json"),
        source: io::Error::new(io::ErrorKind::PermissionDenied, "denied"),
    };
    assert_eq!(io_error.code(), "RELMD-E-001");
    assert_error_text(&io_error.to_string(), "RELMD-E-001", &["/tmp/release.json"]);
    assert!(io_error.source().is_some());

    let serialize_error = ReleaseMetadataError::JsonSerialize {
        source: json_error(),
    };
    assert_eq!(serialize_error.code(), "RELMD-E-002");
    assert_error_text(
        &serialize_error.to_string(),
        "RELMD-E-002",
        &["failed to serialize JSON"],
    );
    assert!(serialize_error.source().is_some());

    let parse_error = ReleaseMetadataError::JsonParse {
        path: PathBuf::from("/tmp/release.json"),
        source: json_error(),
    };
    assert_eq!(parse_error.code(), "RELMD-E-003");
    assert_error_text(
        &parse_error.to_string(),
        "RELMD-E-003",
        &["failed to parse JSON"],
    );
    assert!(parse_error.source().is_some());

    let missing = ReleaseMetadataError::MissingField {
        field: "binary_name",
    };
    assert_eq!(missing.code(), "RELMD-E-004");
    assert_error_text(
        &missing.to_string(),
        "RELMD-E-004",
        &["missing field binary_name"],
    );
    assert!(missing.source().is_none());

    let invalid = ReleaseMetadataError::InvalidField {
        field: "role",
        detail: "bad".to_string(),
    };
    assert_eq!(invalid.code(), "RELMD-E-005");
    assert_error_text(
        &invalid.to_string(),
        "RELMD-E-005",
        &["invalid field role", "bad"],
    );
    assert!(invalid.source().is_none());
}

#[test]
fn runner_release_lint_errors_preserve_codes_display_and_sources() {
    let directory_read = ReleaseLintError::DirectoryRead {
        path: PathBuf::from("/tmp/release"),
        source: io::Error::new(io::ErrorKind::NotFound, "missing"),
    };
    assert_eq!(directory_read.code(), "RUNNER-E-005");
    assert_error_text(
        &directory_read.to_string(),
        "RUNNER-E-005",
        &["release directory"],
    );
    assert!(directory_read.source().is_some());

    let invalid_binary = ReleaseLintError::InvalidBinaryName {
        binary_name: "bad".to_string(),
    };
    assert_eq!(invalid_binary.code(), "RUNNER-E-006");
    assert_error_text(
        &invalid_binary.to_string(),
        "RUNNER-E-006",
        &["binary name"],
    );
    assert!(invalid_binary.source().is_none());

    let missing_sidecar = ReleaseLintError::MissingSidecar {
        sidecar_path: PathBuf::from("/tmp/openwepp.sha256.json"),
    };
    assert_eq!(missing_sidecar.code(), "RUNNER-E-005");
    assert_error_text(
        &missing_sidecar.to_string(),
        "RUNNER-E-005",
        &["missing sidecar"],
    );
    assert!(missing_sidecar.source().is_none());

    let sidecar_invalid = ReleaseLintError::SidecarInvalid {
        sidecar_path: PathBuf::from("/tmp/openwepp.sha256.json"),
        source: ReleaseMetadataError::MissingField { field: "sha256" },
    };
    assert_eq!(sidecar_invalid.code(), "RUNNER-E-005");
    assert_error_text(
        &sidecar_invalid.to_string(),
        "RUNNER-E-005",
        &["invalid sidecar", "RELMD-E-004", "sha256"],
    );
    assert!(sidecar_invalid.source().is_some());

    let role_mismatch = ReleaseLintError::SidecarRoleMismatch {
        sidecar_path: PathBuf::from("/tmp/openwepp.sha256.json"),
        expected: BinaryRole::Hillslope,
        observed: "watershed".to_string(),
    };
    assert_eq!(role_mismatch.code(), "RUNNER-E-005");
    assert_error_text(
        &role_mismatch.to_string(),
        "RUNNER-E-005",
        &["role mismatch"],
    );
    assert!(role_mismatch.source().is_none());

    let name_mismatch = ReleaseLintError::SidecarBinaryNameMismatch {
        sidecar_path: PathBuf::from("/tmp/openwepp.sha256.json"),
        expected: "openwepp-cli-hill".to_string(),
        observed: "openwepp-cli-watershed".to_string(),
    };
    assert_eq!(name_mismatch.code(), "RUNNER-E-005");
    assert_error_text(
        &name_mismatch.to_string(),
        "RUNNER-E-005",
        &["binary_name mismatch"],
    );
    assert!(name_mismatch.source().is_none());

    let hbp_pair = ReleaseLintError::HbpPairMismatch {
        watershed: true,
        hillslope: false,
    };
    assert_eq!(hbp_pair.code(), "RUNNER-E-005");
    assert_error_text(&hbp_pair.to_string(), "RUNNER-E-005", &["hbp_supported"]);
    assert!(hbp_pair.source().is_none());

    let no_candidates = ReleaseLintError::NoReleaseCandidates {
        release_dir: PathBuf::from("/tmp/release"),
    };
    assert_eq!(no_candidates.code(), "RUNNER-E-006");
    assert_error_text(
        &no_candidates.to_string(),
        "RUNNER-E-006",
        &["no openwepp_"],
    );
    assert!(no_candidates.source().is_none());
}

#[test]
fn runner_errors_preserve_codes_display_and_sources() {
    let missing = RunnerError::MissingArgument {
        argument: "--run-dir".to_string(),
    };
    assert_eq!(missing.code(), "RUNNER-E-001");
    assert_error_text(&missing.to_string(), "RUNNER-E-001", &["--run-dir"]);
    assert!(missing.source().is_none());

    let binary_missing = RunnerError::HillslopeBinaryMissing {
        path: PathBuf::from("/tmp/openwepp-cli-hill"),
    };
    assert_eq!(binary_missing.code(), "RUNNER-E-002");
    assert_error_text(
        &binary_missing.to_string(),
        "RUNNER-E-002",
        &["hillslope binary"],
    );
    assert!(binary_missing.source().is_none());

    let launch = RunnerError::LaunchFailure {
        source: io::Error::new(io::ErrorKind::PermissionDenied, "denied"),
    };
    assert_eq!(launch.code(), "RUNNER-E-003");
    assert_error_text(&launch.to_string(), "RUNNER-E-003", &["child process"]);
    assert!(launch.source().is_some());

    #[cfg(unix)]
    {
        let nonzero = RunnerError::NonZeroExit {
            status: std::process::ExitStatus::from_raw(2 << 8),
        };
        assert_eq!(nonzero.code(), "RUNNER-E-004");
        assert_error_text(&nonzero.to_string(), "RUNNER-E-004", &["non-zero"]);
        assert!(nonzero.source().is_none());
    }

    let release_lint = RunnerError::ReleaseLint {
        source: ReleaseLintError::InvalidBinaryName {
            binary_name: "bad".to_string(),
        },
    };
    assert_eq!(release_lint.code(), "RUNNER-E-006");
    assert_error_text(&release_lint.to_string(), "RUNNER-E-006", &["binary name"]);
    assert!(release_lint.source().is_some());

    let release_metadata = RunnerError::ReleaseMetadata {
        source: ReleaseMetadataError::MissingField { field: "role" },
    };
    assert_eq!(release_metadata.code(), "RELMD-E-004");
    assert_error_text(
        &release_metadata.to_string(),
        "RELMD-E-004",
        &["missing field role"],
    );
    assert!(release_metadata.source().is_some());
}

#[test]
fn hillslope_cli_path_and_core_errors_preserve_codes_display_and_sources() {
    assert_hillslope_error_cases(vec![
        (
            HillslopeCliError::MissingArgument {
                argument: "--run-dir",
            },
            "CLIHILL-E-001",
            vec!["--run-dir"],
            false,
        ),
        (
            HillslopeCliError::RunDirectoryMissing {
                path: PathBuf::from("/tmp/run"),
            },
            "CLIHILL-E-002",
            vec!["run directory"],
            false,
        ),
        (
            HillslopeCliError::RunFileMissing {
                path: PathBuf::from("/tmp/run/case.run"),
            },
            "CLIHILL-E-003",
            vec!["run file"],
            false,
        ),
        (
            HillslopeCliError::OutputDirectoryCreate {
                path: PathBuf::from("/tmp/out"),
                source: io::Error::new(io::ErrorKind::PermissionDenied, "denied"),
            },
            "CLIHILL-E-004",
            vec!["output directory"],
            true,
        ),
        (
            HillslopeCliError::CoreInputMissing {
                extension: "man",
                run_dir: PathBuf::from("/tmp/run"),
            },
            "CLIHILL-E-005",
            vec![".man"],
            false,
        ),
        (
            HillslopeCliError::CoreInputAmbiguous {
                extension: "sol",
                run_dir: PathBuf::from("/tmp/run"),
                count: 2,
            },
            "CLIHILL-E-006",
            vec![".sol", "count=2"],
            false,
        ),
    ]);
}

#[test]
fn hillslope_cli_sidecar_and_runtime_errors_preserve_codes_display_and_sources() {
    let sidecar_source = SidecarAdapterError::MissingRequiredSidecar {
        sidecar_id: SidecarId::new("snow").expect("valid id"),
        canonical_file_name: "snow.txt".to_string(),
    };

    assert_hillslope_error_cases(vec![
        (
            HillslopeCliError::SidecarContractInvalid {
                detail: "bad contract".to_string(),
            },
            "CLIHILL-E-007",
            vec!["bad contract"],
            false,
        ),
        (
            HillslopeCliError::SidecarAdapter {
                source: sidecar_source,
            },
            "CLIHILL-E-008",
            vec!["sidecar adapter failure", "LSB-E-007"],
            true,
        ),
        (
            HillslopeCliError::SidecarBindingMissing { sidecar_id: "snow" },
            "CLIHILL-E-009",
            vec!["snow"],
            false,
        ),
        (
            HillslopeCliError::ParseFailure {
                surface: "management",
                detail: "bad".to_string(),
            },
            "CLIHILL-E-010",
            vec!["management", "bad"],
            false,
        ),
        (
            HillslopeCliError::RuntimeSurfaceFailure {
                surface: "lane-d",
                detail: "missing coeff".to_string(),
            },
            "CLIHILL-E-011",
            vec!["lane-d", "missing coeff"],
            false,
        ),
        (
            HillslopeCliError::OfeTopologyMismatch {
                slope_ofe_count: 2,
                management_topology_count: 1,
                soil_topology_count: 3,
            },
            "CLIHILL-E-019",
            vec!["slope=2", "management=1", "soil=3", "mismatches=["],
            false,
        ),
    ]);
}

#[test]
fn hillslope_cli_output_errors_preserve_codes_display_and_sources() {
    assert_hillslope_error_cases(vec![
        (
            HillslopeCliError::OutputWrite {
                path: PathBuf::from("/tmp/out/wat.csv"),
                source: io::Error::new(io::ErrorKind::BrokenPipe, "pipe"),
            },
            "CLIHILL-E-012",
            vec!["wat.csv"],
            true,
        ),
        (
            HillslopeCliError::MissingRequiredOutput { output_name: "wat" },
            "CLIHILL-E-013",
            vec!["wat"],
            false,
        ),
    ]);
}

#[test]
fn hillslope_cli_metadata_manifest_and_io_errors_preserve_codes_display_and_sources() {
    assert_hillslope_error_cases(vec![
        (
            HillslopeCliError::ReleaseMetadata {
                source: ReleaseMetadataError::MissingField { field: "role" },
            },
            "CLIHILL-E-014",
            vec!["release metadata failure", "RELMD-E-004"],
            true,
        ),
        (
            HillslopeCliError::ManifestSerialize {
                source: json_error(),
            },
            "CLIHILL-E-015",
            vec!["serialize manifest"],
            true,
        ),
        (
            HillslopeCliError::ManifestWrite {
                path: PathBuf::from("/tmp/out/manifest.json"),
                source: io::Error::new(io::ErrorKind::PermissionDenied, "denied"),
            },
            "CLIHILL-E-016",
            vec!["manifest.json"],
            true,
        ),
        (
            HillslopeCliError::Io {
                path: PathBuf::from("/tmp/run/case.run"),
                source: io::Error::new(io::ErrorKind::NotFound, "missing"),
            },
            "CLIHILL-E-017",
            vec!["case.run"],
            true,
        ),
        (
            HillslopeCliError::TimeFormat {
                detail: "bad utc".to_string(),
            },
            "CLIHILL-E-018",
            vec!["bad utc"],
            false,
        ),
    ]);
}
