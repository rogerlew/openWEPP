# CQR19 CRAP Before

Status: complete.

Ran: before CRAP command:

```bash
cargo crap --workspace --lcov docs/work-packages/20260615-cqr19-watershed-runtime-types-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr19-watershed-runtime-types-complexity-001/artifacts/crap_before.json
```

Ran: ranked before rows for
`crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs`:

```text
WatershedClimateRuntimeInputError::fmt  line 330  CC 20.0  coverage 0.0                 CRAP 420.0
WatershedRuntimeInputError::fmt         line 78   CC 13.0  coverage 0.0                 CRAP 182.0
WatershedClimateRuntimeInputError::code line 300  CC 19.0  coverage 38.095238095238095  CRAP 104.64053557931109
WatershedRuntimeInputError::code        line 59   CC 13.0  coverage 40.0                CRAP 49.504
```

Static: live target identity for CQR19 is
`WatershedClimateRuntimeInputError::fmt`, not just the file-level snapshot row.
It exceeded the closure threshold before refactor, so the package required
characterization and production decomposition.
