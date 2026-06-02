# Contract-Test Implementation Evidence

Status: complete

Evidence mode: static + ran

Static:

- Added HPHYS0250 scheduler/runner tests for preserving PL runtime activation,
  accepting zero-date established perennial slots, seeding neutral `Ws`, and
  ensuring WB13 consumes final flux-surface `Ep`.
- Added orchestrator tests for WB11 growth/decomposition transition writeback,
  zero-date perennial dispatch, established-perennial initial live-canopy
  projection, and WB15 near-zero interception/liquid roundoff canonicalization.
- Extended the HPHYS0245 trace schema to include PL state and ET/root-uptake
  lineage (`pl_sumgdd`, `pl_vdmt`, `pl_cancov`, `pl_lai`, `pl_rtmass`,
  `pl_rtd`, `Etp`, `UPi`, `Ui`, `Ep`, `Ws`).

Ran:

- `cargo test -p openwepp-hillslope-orchestrator hphys0250_ -- --nocapture`
  passed. Log: `gate-logs/post_impl_hphys0250_orchestrator_tests.log`.
- `cargo test -p openwepp-hillslope-orchestrator management_runtime_projection_assimilates_initial_perennial_live_canopy -- --nocapture`
  passed. Log: `gate-logs/post_impl_initial_canopy_projection_test.log`.
- `cargo test -p openwepp-runner hphys0250_ -- --nocapture` passed. Log:
  `gate-logs/post_impl_hphys0250_runner_tests.log`.
- `cargo test -p openwepp-runner hphys0245_trace -- --nocapture` passed. Log:
  `gate-logs/post_impl_hphys0245_trace_tests.log`.
- `cargo test --test wb17_et_physics_kernel_contract -- --nocapture` passed
  `10/10`. Log: `gate-logs/post_impl_wb17_et_contract_tests.log`.
