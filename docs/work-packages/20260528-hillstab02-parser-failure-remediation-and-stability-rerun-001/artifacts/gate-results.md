# gate-results

Status: complete  
Evidence mode: Ran

Executed:
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass
  - notes: duplicate/license-not-encountered warnings reported; command exit
    status is success (`advisories ok, bans ok, licenses ok, sources ok`).
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` -> pass
- HILLSTAB cohort rerun harness -> pass (execution completed and wrote
  `artifacts/hillstab02-rerun-results.json`)

Disposition alignment:
- Gate execution succeeded, but release readiness remains HOLD due cohort
  functional outcomes (see `hillstab02_disposition.md`).
