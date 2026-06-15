# CQR18 Implementation and Test Evidence

Status: closed.

Implementation summary:

- Split `validate_payload` into private helpers for payload extraction,
  payload header reads, directory-key/minor checks, event payload parsing,
  state snapshot validation, required state closure, and trailing-byte checks.
- Added small private structs `PayloadHeader`, `StateEnvelope`, and
  `StateSchemaFields` to carry parsed fields without changing public API.
- Kept existing constants, aliases, CRC functions, cursor contexts, typed HBP
  error IDs, and validation messages.
- Added public-parser characterization tests in
  `tests/integration/infile_hbp_parser_contract.rs`.

Focused characterization branches:

- Schema 1 `payload crc mismatch`
- Schema 2 `raw payload crc mismatch`
- `payload and directory key mismatch`
- `unsupported payload minor`
- `duplicate state id`
- `state entry length mismatch`
- `required state id missing: 1`

Ran before production refactor:

```text
cargo test --test infile_hbp_parser_contract payload_validator -- --nocapture
```

Result: exit code `0`; `3 passed`.

Ran after production refactor:

```text
cargo test --test infile_hbp_parser_contract -- --nocapture
```

Result: exit code `0`; `24 passed`.

Ran after production refactor:

```text
cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr18-hbp-payload-validator-complexity-001/artifacts/lcov_after.info
```

Result: exit code `0`; LCOV saved to `lcov_after.info`.
