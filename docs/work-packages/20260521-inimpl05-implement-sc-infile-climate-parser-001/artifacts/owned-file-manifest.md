# Owned File Manifest — INIMPL05

Evidence mode: Direct listing

## Write-Set Files Changed
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl05-climate/crates/openwepp-input-contract/src/parsers/climate.rs`
  - Status: created
  - Purpose: `SC-INFILE-CLIMATE-001` strict/compat parser implementation and typed error taxonomy.
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl05-climate/tests/integration/infile_climate_parser_contract.rs`
  - Status: created
  - Purpose: contract-level parser behavior tests.
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl05-climate/tests/fixtures/infile/climate/strict_valid.cli`
  - Status: created
  - Purpose: strict valid no-breakpoint fixture.
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl05-climate/tests/fixtures/infile/climate/single_storm_itemp2.cli`
  - Status: created
  - Purpose: legacy single-storm mode fixture.
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl05-climate/tests/fixtures/infile/climate/unsupported_datver.cli`
  - Status: created
  - Purpose: unsupported `datver` rejection fixture.
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl05-climate/tests/fixtures/infile/climate/malformed_daily_arity.cli`
  - Status: created
  - Purpose: malformed daily token-count rejection fixture.
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl05-climate/tests/fixtures/infile/climate/breakpoint_non_monotone.cli`
  - Status: created
  - Purpose: breakpoint cumulative-precip monotonicity rejection fixture.
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl05-climate/tests/fixtures/infile/climate/breakpoint_overflow_51.cli`
  - Status: created
  - Purpose: breakpoint cardinality strict rejection + compat override acceptance fixture.
