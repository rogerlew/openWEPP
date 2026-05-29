# HILLSTAB07 WB16 Peak-Flow Gap Matrix

Status: complete  
Evidence mode: mixed (`Static` + `Ran`)

## Gap Matrix

| Gap ID | Statement | Evidence | Status |
|---|---|---|---|
| `HILLSTAB07-GAP-001` | WB16 contracts did not explicitly encode canonical `m=1.5` producer authority or the baseline `ealpha` producer-chain lineage. | Static: amended `SC-RUNOFFPART-001` WB16 addendum + revision history (`v22`), and `SC-WATBAL-001` WB16 addendum + revision history (`v41`). | closed |
| `HILLSTAB07-GAP-002` | Runner compatibility seeding of `ealpha=1.0` was silent, masking input-provenance parity state. | Static: `crates/openwepp-runner/src/hillslope/mod.rs` now publishes `wb16_ealpha_compatibility_seed_used`, `wb16_ealpha_seed_policy`, and warning `SIMPIPE-W-003`. Ran: `cargo test --test cli03_runner_contract_derived_tests cli03_fixture_run_publishes_wb16_ealpha_compatibility_seed_provenance` passed. | closed |
| `HILLSTAB07-GAP-003` | Full baseline-authoritative `ealpha` producer-chain migration (`frcfac -> rdat(alpha) -> alphay -> eplane`) remains incomplete in production runtime surfaces. | Static: explicit non-promotable gaps `GAP-RUNOFFPART-005` and `GAP-WATBAL-005` authored. | open (non-promotable) |
