# Contract-Test Implementation Evidence

Status: complete

Evidence mode: Static

Static: Added and registered
`tests/integration/adr0017_comparator_distrust_ratification_contract.rs`.

Static: `Cargo.toml` registers the test as
`adr0017_comparator_distrust_ratification_contract`.

Static: The test asserts:

- ADR0017 accepted status, ADR0016 accepted amendment, and ADR index status.
- Comparator identity for `wepp_260430_negmeltfix_comparator`,
  `wepp_260430_negmeltfix_comparator_47ac4c32faee`, and
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`.
- Governance surfaces contain peer verdict taxonomy,
  `HARNESS-SURFACE-MISMATCH`, like-for-like unit/lineage requirements,
  independent correctness authority, suspicious-ratio treatment, and owned
  `HOLD`.
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-039` and
  `SC-WATBAL-001#INV-WATBAL-087` are present with producer obligations.
- Package artifacts cannot remain queued placeholders because the test rejects
  `Status: queued` and `Evidence mode: not-run`.

Static: Updated
`tests/integration/hphys0313_snowpack_settling_carry_recursion_contract.rs`
only to keep adjacent contract-version assertions aligned with the new
`SC-SNOWFREEZE-001` and `SC-WATBAL-001` versions.
