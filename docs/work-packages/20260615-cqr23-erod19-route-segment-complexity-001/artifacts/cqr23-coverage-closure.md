# CQR23 Coverage Closure

Status: complete-with-warning.

Ran: before LCOV:
`cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr23-erod19-route-segment-complexity-001/artifacts/lcov_before.info`

Ran: after LCOV:
`cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr23-erod19-route-segment-complexity-001/artifacts/lcov_after.info`

Ran: target-file before coverage for
`crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`:

- Lines: `476/647` (`73.57%`)
- Functions: `15/17` (`88.24%`)

Ran: target-file after coverage:

- Lines: `766/904` (`84.73%`)
- Functions: `34/36` (`94.44%`)

Static: coverage improved, and the CQR target plus new helper rows meet the
CRAP threshold. Warning remains because target-file line coverage is below the
ADR-0021 `90%` line threshold; this package was scoped to CRAP/cyclomatic
decomposition, not module coverage closure.
