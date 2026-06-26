# Verification

Evidence class: Ran.

Verification commands and results are recorded in `gate-results.md`.

Closure gates passed after local formatting/marker corrections:

- Focused 05F integration test.
- `cargo fmt --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo test --workspace`.
- `cargo deny check`.
- `git diff --check`.

Post-review follow-up also passed focused guards after the v81 caveat update:

- `cargo fmt --check`.
- `cargo test --test snowdensity05f_melt_closure_handoff`.
- `cargo test --test snowdensity02_contract_adr_guard --test snowdensity05a_melt_contract_guard --test snowdensity05b_shortwave_source_contract --test snowdensity05c_albedo_state_core --test snowdensity05d_opt_in_coe_melt`.
