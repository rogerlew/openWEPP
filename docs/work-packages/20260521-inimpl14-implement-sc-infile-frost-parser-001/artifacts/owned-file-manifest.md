# Owned File Manifest — INIMPL14

Evidence mode: Direct listing

## Parser Implementation
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl14-frost/crates/openwepp-input-contract/src/parsers/frost.rs`
  - Status: created
  - Purpose: `SC-INFILE-FROST-001` parser implementation, strict/compat policy handling, typed errors/warnings, and provenance markers.
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl14-frost/crates/openwepp-input-contract/src/parsers/mod.rs`
  - Status: modified
  - Purpose: export frost parser module.

## Integration Test
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl14-frost/tests/integration/infile_frost_parser_contract.rs`
  - Status: created
  - Purpose: contract behavior validation for strict/compat parser branches.

## Surface Fixtures
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl14-frost/tests/fixtures/infile/frost/strict_valid_two_line.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl14-frost/tests/fixtures/infile/frost/compat_line2_missing.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl14-frost/tests/fixtures/infile/frost/strict_line1_arity_invalid.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl14-frost/tests/fixtures/infile/frost/strict_line2_token_invalid.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl14-frost/tests/fixtures/infile/frost/strict_out_of_range.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl14-frost/tests/fixtures/infile/frost/compat_out_of_range_clamped.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl14-frost/tests/fixtures/infile/frost/prefixed_variant_rejected.txt`
- [DIRECT] `/home/workdir/openWEPP/.worktrees/inimpl14-frost/tests/fixtures/infile/frost/compat_line2_arity_invalid.txt`
  - Status: created
  - Purpose: strict-valid, compat-defaulting, and malformed-input coverage for frost parser contract.
