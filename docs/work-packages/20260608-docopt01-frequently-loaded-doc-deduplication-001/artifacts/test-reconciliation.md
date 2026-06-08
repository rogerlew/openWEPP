# Test Reconciliation

Status: complete
Evidence mode: Ran

Static: Five integration tests pinned detailed authority strings to the old registry-note location. DOCOPT01 intentionally moved/removes those notes from `docs/specifications/science-contracts/index.md`, so these assertions were reconciled to the canonical contracts that already carry the authority.

Reconciled files:
- `tests/integration/hphys0301_h39_forcing_melt_term_producer_contract.rs`: registry-note assertion changed to slim-registry structural assertion; canonical invariant assertions remain on `SC-SNOWFREEZE-001` and `SC-WATBAL-001`.
- `tests/integration/hphys0302_comparator_surface_audit_contract.rs`: registry-note assertion changed to slim-registry structural assertion; canonical invariant assertions remain on `SC-SNOWFREEZE-001` and `SC-WATBAL-001`.
- `tests/integration/erod13_contract_authority_closure_contract.rs`: EROD13 guard-family assertion moved from registry to `SC-SED-001`.
- `tests/integration/erod14_contract_authority_closure_contract.rs`: EROD14 guard-family assertion moved from registry to `SC-SED-001`.
- `tests/integration/erod15_wave3_contract_authority_closure_contract.rs`: HBP routing-boundary authority assertion moved from registry to `SC-SED-001`.

Ran: `cargo test --workspace` after reconciliation.
Result: PASS, exit code 0.

Behavioral assertion semantics: unchanged. Assertions now check the canonical authority location instead of the deduplicated lifecycle registry.
