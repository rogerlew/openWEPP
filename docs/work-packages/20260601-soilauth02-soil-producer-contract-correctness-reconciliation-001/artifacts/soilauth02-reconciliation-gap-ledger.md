# SOILAUTH02 Reconciliation Gap Ledger

Status: complete  
Evidence mode: Static + Ran

## Scope
Closure ledger for SOILAUTH01 mismatch set (`SA01-M001..SA01-M004`).

## Resolution Ledger

### `SA01-M001` (P0) - `9002/9003/9005` policy-first ordering
- Resolution: closed.
- Action:
  - parser now accepts policy-first ordering in both strict and compatibility
    modes;
  - parser still accepts header-first ordering for backward files;
  - contract/spec updated to ratify canonical producer policy-first envelope.
- Evidence:
  - `crates/openwepp-input-contract/src/parsers/soil.rs` (`parse_ofe_block`)
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
  - `docs/specifications/wepp-input-files/specs/soil-file.spec.md`
  - parser tests pass.

### `SA01-M002` (P0) - missing explicit `avke` in canonical quoted headers
- Resolution: closed.
- Action:
  - parser now accepts quoted `7778/9002/9003/9005` headers in strict and
    compatibility modes with explicit `avke := 0.0` normalization when omitted;
  - contract/spec updated to ratify the canonical envelope.
- Evidence:
  - `crates/openwepp-input-contract/src/parsers/soil.rs` (`parse_ofe_header_tokens`)
  - updated contracts/spec + parser tests.

### `SA01-M003` (P1) - restrictive-row placement/cardinality drift
- Resolution: closed.
- Action:
  - parser now accepts per-OFE restrictive rows for
    `7778/9002/9003/9005` in strict and compatibility modes;
  - normalization requires pairwise-identical per-OFE restrictive rows and
    lifts to one profile-level restrictive-layer state;
  - trailing restrictive row (if present) must match normalized row.
- Evidence:
  - `crates/openwepp-input-contract/src/parsers/soil.rs`
    (`maybe_parse_ofe_restrictive_row`, footer resolution logic)
  - updated contract/spec text + parser tests.

### `SA01-M004` (P1) - quote-style tokenization gap
- Resolution: closed.
- Action:
  - parser quote tokenizer upgraded to accept both single-quoted and
    double-quoted tokens (with double-quote escape handling);
  - canonical apostrophe-bearing policy-token fixture added.
- Evidence:
  - `crates/openwepp-input-contract/src/parsers/soil.rs`
    (`tokenize_whitespace_and_quotes`)
  - `tests/fixtures/infile/soil/canonical_9002_double_quoted_policy.sol`
  - `tests/integration/soilauth02_soil_producer_reconciliation_contract.rs`

## Closure Measure Status
- `MEASURE-SA02-001`: pass (all P0/P1 mismatches resolved).
- `MEASURE-SA02-002`: pass (canonical-fixture parser tests green).
- `MEASURE-SA02-003`: pass (fixture hash/provenance sidecars added).
