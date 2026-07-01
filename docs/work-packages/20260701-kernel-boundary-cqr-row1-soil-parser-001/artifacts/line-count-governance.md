# Line-Count Governance

Evidence mode: Ran.

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs tests/integration/infile_soil_parser_contract.rs
```

Result:

```text
1431 crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs
 451 tests/integration/infile_soil_parser_contract.rs
1882 total
```

## Disposition

Status: `PASS`.

The row #1 production runtime-input file remains below the 3000-line governance
threshold. No line-count exception is used for this row.
