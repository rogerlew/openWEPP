# hillstab02-contract-test-implementation-evidence

Status: complete  
Evidence mode: Static

## Added/Updated Fixtures
- Added soil compatibility fixture:
  - `tests/fixtures/infile/soil/compat_quoted_policy_row_9002.sol`
- Added management compatibility fixture:
  - `tests/fixtures/infile/management/compat_tilseq_zero_nonzero_nseq_98_4.man`

## Added Contract-Derived Tests
- `tests/integration/infile_soil_parser_contract.rs`
  - `strict_rejects_quoted_9002_policy_row_with_whitespace_luse`
  - `compatibility_accepts_quoted_9002_policy_row_with_whitespace_luse`
- `tests/integration/infile_management_parser_contract.rs`
  - `strict_mode_rejects_tilseq_zero_when_nseq_nonzero`
  - `compatibility_mode_accepts_tilseq_zero_when_nseq_nonzero`

## Coverage Intent
- Soil tests encode strict rejection and compatibility acceptance for quoted
  disturbed-policy rows with whitespace-bearing `luse`/`stext`.
- Management tests encode strict reference-domain enforcement and
  compatibility-only sentinel acceptance for `tilseq=0`.
