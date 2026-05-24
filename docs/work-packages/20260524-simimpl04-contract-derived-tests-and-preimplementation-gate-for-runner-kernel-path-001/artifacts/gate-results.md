# gate results

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- SIMIMPL04 scope is contract-derived tests and gate/governance evidence only.
- No production runner/orchestrator/kernel source edits were introduced.

## Ran
- Test compilation gate:
  - `cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract --test simimpl04_wepp_ui_mode_closure_contract --test simimpl04_wb13_publication_contract --no-run` (pass)
- Expected-fail evidence runs:
  - `cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract -- --ignored` (expected fail: missing `/execution_provenance/scheduler_kernel_executed`)
  - `cargo test -p openwepp-runner --test simimpl04_wepp_ui_mode_closure_contract -- --ignored` (expected fail: missing `/mode_selection/wepp_ui/requested`)
  - `cargo test -p openwepp-runner --test simimpl04_wb13_publication_contract -- --ignored` (expected fail: missing `/wb13_publication/source`)
- Baseline non-failing run posture:
  - `cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract --test simimpl04_wepp_ui_mode_closure_contract --test simimpl04_wb13_publication_contract` (pass with all tests ignored)

## Gate disposition
- SIMIMPL04 package gate: `GO`.
- SIMIMPL05 production-edit gate: `GO` (contract amendments + contract-derived tests + pre-implementation gate now complete).
