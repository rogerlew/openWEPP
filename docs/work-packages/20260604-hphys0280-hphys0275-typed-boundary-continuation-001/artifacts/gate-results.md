# Gate Results

Status: completed/HOLD
Evidence mode: ran

Static: final broad gate HOLD is unrelated to HPHYS0280 typed-boundary implementation and was reproduced on clean `HEAD`.

Ran:
- `cargo fmt --check`: pass.
- `cargo test -p openwepp-unit-boundary direction_degrees_rejects_out_of_range -- --nocapture`: pass.
- `cargo test --test hphys0275_boundary_value_dimensional_typing_contract -- --nocapture`: pass.
- `cargo test --test clim05_snow_runtime_kernel_contract -- --nocapture`: pass.
- `cargo test --test sim_contract_boundary_unit_registry -- --nocapture`: pass.
- `tools/release/check_unit_registry.sh`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo deny check`: pass; emitted existing duplicate crate and unmatched-license warnings.
- `markdown-doc lint --path docs/work-packages/20260604-hphys0280-hphys0275-typed-boundary-continuation-001`: pass; 23 files validated, 0 errors, 0 warnings.
- `cargo test --workspace`: fail; `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage` and `simimpl18_contract_requires_multi_day_storage_state_mutation` fail in `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs` with `HKERNEL-WB11-ET-E-003`.
- Clean `HEAD` check: `CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract simimpl18_contract_requires -- --nocapture` in detached `HEAD 58f985d` worktree also failed on the same two tests.
