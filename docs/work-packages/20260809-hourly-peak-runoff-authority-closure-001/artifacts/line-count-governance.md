# Line-Count Governance

Status: `PASS with existing-file WARN rows`

Evidence mode: `Ran`

No changed Rust file reaches the nonexempt 3,000-line blocking threshold.
Changed existing files at or above the 2,000-line warning threshold are:

- `direct_runtime/00_core_frames.rs`: 2,707 lines.
- `direct_runtime/laned_active.rs`: 2,028 lines.
- `direct_runtime/runoff.rs`: 2,742 lines.
- `00_builders_and_authority.rs`: 2,760 lines.
- `00c_day_input_builder_impl.rs`: 2,981 lines.
- `openwepp-runner/src/hillslope/03_tests.rs`: 2,892 lines.
- `openwepp-runner/tests/watershed_cli_behavior_contract.rs`: 2,996 lines.

The peak implementation removes the obsolete 541-line WB16 projection body
from its split file and does not create a new monolithic source file. These
WARN rows are retained architecture debt, not a package closure blocker.
Concrete follow-on split seams are:

- `00_core_frames.rs`: separate frame/type declarations from constructor and
  domain-validation implementations.
- `laned_active.rs`: separate routing state/executor logic from trace and
  summary aggregation.
- `runoff.rs`: extract the WB14 hourly infiltration/source-custody operator and
  WB16 typed peak operator behind their existing typed interfaces.
- `00_builders_and_authority.rs`: separate authority/configuration types from
  production input builders.
- `00c_day_input_builder_impl.rs`: separate snow/frost liquid preparation from
  canopy/management and final day-frame assembly.
- `03_tests.rs`: extract WB14/WB16 source-custody and peak-publication tests by
  owning phase.
- `watershed_cli_behavior_contract.rs`: extract binary fixture encoders and
  run-directory builders from behavior assertions.

Those mechanical decompositions belong to a separately authorized refactor;
mixing them into this Critical physics correction would enlarge its review
surface without changing the defect closure.
