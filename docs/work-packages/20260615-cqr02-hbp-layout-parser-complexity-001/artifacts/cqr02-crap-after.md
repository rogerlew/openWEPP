# CRAP After

Status: complete
Evidence mode: Ran

Command:

```sh
cargo crap --workspace --lcov docs/work-packages/20260615-cqr02-hbp-layout-parser-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr02-hbp-layout-parser-complexity-001/artifacts/crap_after.json
```

Result: every `layout_parser.rs` function has CRAP <= 30.

Highest target rows:

| Function | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: |
| `parse_header` | 20.0 | 100.0 | 20.0 |
| `parse_metadata` | 19.0 | 100.0 | 19.0 |
| `parse_payload_block_entry` | 16.0 | 100.0 | 16.0 |
| `validate_schema2_footer` | 15.0 | 100.0 | 15.0 |
| `parse_directory_entry` | 10.0 | 82.6086956521739 | 10.526012985945592 |
| `parse_layout` | 8.0 | 78.94736842105263 | 8.597171599358507 |
