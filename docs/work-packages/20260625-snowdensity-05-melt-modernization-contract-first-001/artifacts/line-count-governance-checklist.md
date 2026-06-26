# Line-Count Governance Checklist

Status: complete.
Evidence mode: Ran.

```text
wc -l tests/integration/snowdensity05a_melt_contract_guard.rs docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md docs/work-packages/20260625-snowdensity-05-melt-modernization-contract-first-001/package.md Cargo.toml
```

Result:

- `tests/integration/snowdensity05a_melt_contract_guard.rs`: 85 lines
- `SC-SNOWFREEZE-001.md`: 1431 lines
- package `package.md`: 107 lines
- `Cargo.toml`: 650 lines

Disposition: no production Rust file touched. The long science contract is an
existing canonical authority file and is not subject to Rust mechanical-refactor
line-count closure.
