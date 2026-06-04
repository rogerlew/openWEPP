# Gate Results

Status: completed/HOLD
Evidence mode: Ran

Ran:
- `cargo fmt --check`: pass.
- `tools/release/check_raw_unit_conversions.sh`: pass.
- `cargo test -p openwepp-unit-boundary`: pass, 24 tests.
- `cargo test --test hphys0276_raw_unit_conversion_guard_contract`: pass, 5 tests.
- `cargo test -p openwepp-hillslope-orchestrator runtime_inputs`: pass, 47 tests.
- `cargo test --test hphys0275_boundary_value_dimensional_typing_contract`: pass, 4 tests.
- `cargo test --test clim05_snow_runtime_kernel_contract`: pass, 9 tests.
- `cargo test --test wb19_lateral_drainage_physics_kernel_contract`: pass, 15 tests.
- `cargo clippy -p openwepp-unit-boundary --all-targets -- -D warnings`: pass.
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`: pass.
- `cargo test --test sim_contract_boundary_unit_registry`: pass, 10 tests.
- `cargo clippy --test hphys0276_raw_unit_conversion_guard_contract -- -D warnings`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `cargo deny check`: pass with existing warnings:
  - duplicate `getrandom`, `hashbrown`, `twox-hash`
  - unmatched license allowances `ISC`, `Unicode-DFS-2016`
- `markdown-doc lint --path docs/specifications/unit-governance.md --path docs/specifications/science-contracts/unit-safe-boundary-types-contract.md --path docs/architecture/unit-safe-boundary-types.md --path docs/work-packages/20260603-hphys0276-unit-conversion-helper-and-raw-literal-guard-001/package.md --path docs/work-packages/20260603-hphys0276-unit-conversion-helper-and-raw-literal-guard-001/artifacts --path docs/work-packages/README.md`: pass, 23 files.
- `git diff --check`: pass.
- `cargo test --workspace`: fail in known SIMIMPL18 ET-domain tests with
  `HKERNEL-WB11-ET-E-003` domain violation.

Gate disposition: HPHYS0276 targeted gates pass. Workspace remains HOLD due
known SIMIMPL18 failure outside this package scope.
