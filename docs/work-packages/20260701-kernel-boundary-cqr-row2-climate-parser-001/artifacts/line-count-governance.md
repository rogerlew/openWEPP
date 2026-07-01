# Line-Count Governance

Evidence mode: Ran.

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs tests/integration/infile_climate_parser_contract.rs
```

Result:

```text
102 crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs
617 tests/integration/infile_climate_parser_contract.rs
719 total
```

## Disposition

Status: `PASS`.

The row #2 production runtime-input file remains below the 3000-line governance
threshold. No line-count exception is used for this row.
