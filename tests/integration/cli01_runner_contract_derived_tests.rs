use std::path::PathBuf;

use openwepp_legacy_bridge::policy::CompatibilityPolicy;
use openwepp_legacy_bridge::sidecar::{
    SidecarAdapterError, SidecarAdapterRequest, SidecarContract, SidecarDiscovery, SidecarId,
    SidecarRequirement, SidecarWarningCode, adapt_sidecar_bindings,
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
