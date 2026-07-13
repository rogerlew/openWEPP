use openwepp_legacy_bridge::hbp::HbpAdapterError;

#[test]
fn hbp_adapter_error_display_preserves_all_contract_identities() {
    let cases = [
        (
            HbpAdapterError::ContractMinimumBytesTooSmall { minimum_bytes: 3 },
            "HBP-E-001",
            "HBP-E-001 invalid contract minimum_bytes=3 (must be >= 4)",
        ),
        (
            HbpAdapterError::DuplicateLegacyMagicAlias { magic: *b"HBP0" },
            "HBP-E-002",
            "HBP-E-002 duplicate legacy magic alias 0x48425030",
        ),
        (
            HbpAdapterError::CanonicalMagicListedAsLegacyAlias { magic: *b"HBP1" },
            "HBP-E-003",
            "HBP-E-003 canonical magic listed as legacy alias 0x48425031",
        ),
        (
            HbpAdapterError::ShardTooShort {
                observed_bytes: 4,
                minimum_bytes: 8,
            },
            "HBP-E-004",
            "HBP-E-004 shard too short: observed=4 minimum=8",
        ),
        (
            HbpAdapterError::UnknownMagic {
                observed_magic: *b"ABCD",
            },
            "HBP-E-005",
            "HBP-E-005 unknown magic 0x41424344",
        ),
        (
            HbpAdapterError::LegacyMagicDisallowed {
                observed_magic: *b"HBP0",
            },
            "HBP-E-006",
            "HBP-E-006 strict policy disallows legacy magic 0x48425030",
        ),
    ];

    for (error, expected_code, expected_display) in cases {
        assert_eq!(error.code(), expected_code);
        assert_eq!(error.to_string(), expected_display);
        assert!(std::error::Error::source(&error).is_none());
    }
}
