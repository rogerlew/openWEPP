# Gate Results

Status: completed
Evidence mode: Ran

Static:
- Required gates were enumerated from package kickoff and executed in order.

Ran:
- `cargo fmt --check` — pass (exit `0`).
- `cargo clippy --workspace --all-targets -- -D warnings` — pass (exit `0`).
- `cargo test -p openwepp-kernel-contract --tests` — pass (exit `0`).
- `cargo test --workspace` — **fail** (exit `101`) due existing unrelated failure:
  - `hphys0289_wb13_rm_snowwater_publication_contract` -> `hphys0289_contract_requires_kernel_to_publish_daily_routed_wmelt`.
- `cargo deny check` — pass with warnings (exit `0`).
