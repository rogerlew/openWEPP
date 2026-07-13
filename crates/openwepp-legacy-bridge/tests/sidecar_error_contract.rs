use openwepp_legacy_bridge::sidecar::{SidecarAdapterError, SidecarId};

fn sidecar_id(value: &str) -> SidecarId {
    SidecarId::new(value).expect("test sidecar id must be valid")
}

#[test]
fn sidecar_adapter_error_display_preserves_all_contract_identities() {
    let cases = [
        (
            SidecarAdapterError::InvalidSidecarId {
                value: "bad id".to_string(),
            },
            "LSB-E-001",
            "LSB-E-001 invalid sidecar id: bad id",
        ),
        (
            SidecarAdapterError::InvalidFileName {
                context: "canonical",
                value: "../soil.txt".to_string(),
            },
            "LSB-E-002",
            "LSB-E-002 invalid file name (canonical): ../soil.txt",
        ),
        (
            SidecarAdapterError::DuplicateContractId {
                id: sidecar_id("soil"),
            },
            "LSB-E-003",
            "LSB-E-003 duplicate contract id: soil",
        ),
        (
            SidecarAdapterError::DuplicateCanonicalFileName {
                file_name: "soil.txt".to_string(),
            },
            "LSB-E-004",
            "LSB-E-004 duplicate canonical file name: soil.txt",
        ),
        (
            SidecarAdapterError::DuplicateLegacyAlias {
                sidecar_id: sidecar_id("soil"),
                alias_file_name: "legacy.sol".to_string(),
            },
            "LSB-E-005",
            "LSB-E-005 duplicate/invalid alias for soil: legacy.sol",
        ),
        (
            SidecarAdapterError::DuplicateDiscoveredFileName {
                file_name: "soil.txt".to_string(),
            },
            "LSB-E-006",
            "LSB-E-006 duplicate discovered file name: soil.txt",
        ),
        (
            SidecarAdapterError::MissingRequiredSidecar {
                sidecar_id: sidecar_id("soil"),
                canonical_file_name: "soil.txt".to_string(),
            },
            "LSB-E-007",
            "LSB-E-007 missing required sidecar soil (soil.txt)",
        ),
        (
            SidecarAdapterError::LegacyAliasDisallowed {
                sidecar_id: sidecar_id("soil"),
                alias_file_name: "legacy.sol".to_string(),
            },
            "LSB-E-008",
            "LSB-E-008 strict policy disallows alias legacy.sol for soil",
        ),
    ];

    for (error, expected_code, expected_display) in cases {
        assert_eq!(error.code(), expected_code);
        assert_eq!(error.to_string(), expected_display);
        assert!(std::error::Error::source(&error).is_none());
    }
}
