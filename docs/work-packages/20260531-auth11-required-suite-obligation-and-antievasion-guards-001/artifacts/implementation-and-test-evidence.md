# AUTH11 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

Static:
- AUTH11 added source-level guardrails for:
  - required-case obligation binding preservation,
  - diff-based anti-evasion review checks,
  - promotion protocol evidence control,
  - in-test anchor assertions.
- Direct-theta cohort now explicitly contains anchored discrepancy classification
  (`valid_9002_reference` expected `exceeds`) rather than silent case removal.

Ran:
- `cargo fmt --check` -> exit 0
- `cargo clippy --workspace --all-targets -- -D warnings` -> exit 0
- `cargo test --workspace` -> exit 0
- `cargo deny check` -> exit 0
- `bash tools/release/check_authority_suite_antievasion.sh --base-ref 0dc1788 --head-ref HEAD` -> exit 0
