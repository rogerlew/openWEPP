# Gate Results

Status: completed/HOLD
Evidence mode: ran

Static: HPHYS0278 focused and governance gates pass. Full workspace remains
HOLD on pre-existing SIMIMPL18/PL14S failures reproduced on clean `HEAD`.

Ran:

- `cargo fmt --check`: pass.
- `cargo test --test sim_contract_boundary_unit_registry -- --nocapture`:
  pass, 13 tests.
- `cargo test -p openwepp-hillslope-output -- --nocapture`: pass, 14 tests.
- `cargo test -p openwepp-watershed-output -- --nocapture`: pass, 4 tests.
- `tools/release/check_unit_registry.sh`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass after
  review-driven fixes.
- `cargo deny check`: pass; existing warnings for duplicate `getrandom`,
  `hashbrown`, `twox-hash`, and unmatched license allowances `ISC` and
  `Unicode-DFS-2016`.
- `cargo test --workspace`: fail/HOLD only on
  `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage`
  and `simimpl18_contract_requires_multi_day_storage_state_mutation`, both with
  `HKERNEL-WB11-ET-E-003`.

Clean `HEAD` confirmation:

- Detached worktree at `c9b1f6e` reproduced the same two PL14S/SIMIMPL18
  failures with `HKERNEL-WB11-ET-E-003`.
