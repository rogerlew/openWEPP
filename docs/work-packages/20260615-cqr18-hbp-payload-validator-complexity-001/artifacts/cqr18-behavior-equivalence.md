# CQR18 Behavior Equivalence

Status: closed.

Static: production edits are private helper extraction inside
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`.

Preserved behavior surfaces:

- Public parser APIs remain unchanged.
- Entry payload extraction still validates schema 1 payload bounds and
  `payload crc mismatch` before cursor decoding.
- Schema 2 payload extraction still validates raw block id, raw slice bounds,
  and `raw payload crc mismatch`.
- Payload header reads retain the same byte order and typed cursor context.
- Directory key mismatch still reports `HbpE010`.
- Unsupported schema major still reports `HbpE003`.
- Unsupported payload minor still reports `HbpE013`.
- Event kind `0`, `1`, and `2` cursor advancement is preserved.
- Runoff event output formulas preserve the existing scaled expression order:
  `scaled_i64_to_f64(value)? * SCALE_I64`.
- State snapshot duplicate, registry field, dimension, representation,
  required-flag, trailing-byte, and missing-required-state checks preserve
  error IDs and messages.

Ran: focused characterization before production refactor:

```text
cargo test --test infile_hbp_parser_contract payload_validator -- --nocapture
```

Result: exit code `0`; `3 passed`.

Ran: full HBP parser contract after production refactor:

```text
cargo test --test infile_hbp_parser_contract -- --nocapture
```

Result: exit code `0`; `24 passed`.
