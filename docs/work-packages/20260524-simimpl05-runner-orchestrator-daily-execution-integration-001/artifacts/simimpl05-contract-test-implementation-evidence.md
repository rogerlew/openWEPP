# simimpl05 contract test implementation evidence

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Activated SIMPIPE contract-derived test by removing `#[ignore]` from:
  - `crates/openwepp-runner/tests/simimpl04_runner_kernel_execution_contract.rs`
- Kept out-of-scope tests in expected-fail ignored posture:
  - `simimpl04_wepp_ui_mode_closure_contract.rs`
  - `simimpl04_wb13_publication_contract.rs`
- Test posture now matches SIMIMPL05 scope boundaries:
  - `GAP-SIMPIPE-001`: active/pass
  - `GAP-SIMMODE-001`: deferred
  - `GAP-SIMOUT-001`: deferred

## Ran
- `cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract`
  - pass
- `cargo test -p openwepp-runner --test simimpl04_runner_kernel_execution_contract --test simimpl04_wepp_ui_mode_closure_contract --test simimpl04_wb13_publication_contract`
  - pass with two expected ignored tests
- `cargo test -p openwepp-runner --test simimpl04_wepp_ui_mode_closure_contract -- --ignored`
  - fails as expected (missing `/mode_selection/wepp_ui/requested`)
- `cargo test -p openwepp-runner --test simimpl04_wb13_publication_contract -- --ignored`
  - fails as expected (missing `/wb13_publication/source`)
