# Gate Results

Status: complete.

Ran:

- `cargo test --test snowdensity05c_albedo_state_core`:
  `5 passed; 0 failed`.
- `cargo test --test snowdensity05a_melt_contract_guard`:
  `3 passed; 0 failed`.
- `cargo test --test snowdensity05b_shortwave_source_contract`:
  `3 passed; 0 failed`.
- `cargo test --test snowdensity02_contract_adr_guard`:
  `3 passed; 0 failed` after updating its stale contract-version marker from
  v76 to v78.
- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: passed after the stale v76 marker fix.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.
- `wctl doc-lint --path docs/work-packages/README.md`:
  `1 files validated, 0 errors, 0 warnings`.
- `wctl doc-lint --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`:
  `0 files validated, 0 errors, 0 warnings`.
- `wctl doc-lint --path docs/work-packages/20260626-snowdensity-05c-albedo-state-core-001/package.md`:
  `0 files validated, 0 errors, 0 warnings`.
- `wctl doc-lint --path docs/planning/snow-frost-fidelity-strategy.md`:
  `0 files validated, 0 errors, 0 warnings`.
- `git diff --check`: passed.

Static:

- `rg` over `crates/openwepp-hillslope-orchestrator/src` and
  `crates/openwepp-runner/src` shows the new albedo API is defined/exported
  only and is not consumed by production routed melt.
