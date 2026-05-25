# MOFE03 Wave2 Test Matrix

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Contract-derived MOFE03 vectors:
1. `cli03_mofe03_multiofe_runfile_executes_wave2_without_manual_symbol_injection`
- Intent: aligned multi-OFE runfile path reaches Wave-2 without manual symbol injection.
- Expected post-implementation signal: execution provenance includes Wave-2 kernel status (`EROD14-WAVE2`).

2. `cli03_mofe03_single_ofe_policy_disables_wave2_by_default`
- Intent: single-OFE path defaults Wave-2 disabled under production policy.
- Expected signal: execution provenance does not report Wave-2 kernel status and policy surface resolves disabled.

## Ran
- Pre-implementation baseline:
  - `cargo test -p openwepp --test cli03_runner_contract_derived_tests mofe03 -- --nocapture`
  - Result:
    - multi-OFE activation test: failed (expected baseline gap),
    - single-OFE policy test: passed.
- Post-implementation:
  - `cargo test -p openwepp --test cli03_runner_contract_derived_tests mofe03 -- --nocapture`
  - Result: both MOFE03 tests passed.
