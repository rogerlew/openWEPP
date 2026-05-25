# MOFE02 Cross-File Parity Test Matrix

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- `cli03_mofe02_enables_hillslope_soil_topology_scope_guard_when_slope_and_management_align`
  - Setup: `slope=1`, `management=1`, `soil=2`
  - Expected: parser hard-fail (`SOL-E-007`) via hillslope topology scope guard.
- `cli03_mofe02_rejects_slope_management_and_slope_soil_mismatch`
  - Setup: `slope=2`, `management=1`, `soil=1`
  - Expected: runner hard-fail with typed topology mismatch and pair labels including `slope-management`, `slope-soil`.
- `cli03_mofe02_rejects_management_soil_mismatch`
  - Setup: `slope=2`, `management=1`, `soil=2`
  - Expected: runner hard-fail with typed topology mismatch and pair label `management-soil`.
- `cli03_mofe02_rejects_full_triad_topology_mismatch`
  - Setup: `slope=2`, `management=1`, `soil=3`
  - Expected: runner hard-fail with typed topology mismatch including all three pair labels.

## Ran
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests mofe02`
- Result: `ok` (4 passed).
