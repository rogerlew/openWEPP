# Validation Log

Evidence mode: Static + Ran.

## Diagnostic Evidence

Ran:

```bash
.venv/bin/python tools/snowfreeze_observed/default_activation_active_cap.py
```

Result:

- Disposition: `COMPLETE-DEFAULT-ACTIVATED-UNDER-ACTIVE-CAP`
- Default trace ok: `true`
- Rollback trace ok: `true`
- No-env activated melt trace rows: `112502`
- No-env activated density trace rows: `112502`
- Rollback legacy melt trace rows: `13880`
- Rollback legacy density trace rows: `13880`
- Paired snow-depth rows: `1415`
- Snow-control failures: `498`
- Frost attribution blocker: `SNOW-CONTROL-RESIDUALS-REMAIN`

Artifacts:

- `artifacts/default-activation-active-cap.json`
- `artifacts/default-activation-active-cap.md`

## Focused Tests

Ran:

```bash
cargo test --test snowdensity10_3_15_default_activation_active_cap
cargo test --test snowdensity07_runtime_opt_in
cargo test --test snowdensity08_gate_rerun
cargo test --test snowdensity09_coupled_wat_rerun
cargo test --test snowdensity10_3_8_liquid_holding_capacity
cargo test --test snowdensity10_3_11_spring_compaction_densification
cargo test --test snowdensity10_3_14_policy_b_no_regression_cap_authority
cargo test --test snowdensity03_physics_bulk_offline_contract
```

Result: PASS.

## Closure Gates

Ran:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Result: PASS.

## Anti-Evasion / Protection Scans

Ran:

```bash
bash tools/release/check_authority_suite_antievasion.sh
cargo test --test auth11_required_suite_obligation_guards_contract
rg -n -i "qwet|frzftp" crates || true
rg -n "SNOWDENSITY09|SNOWDENSITY1038|snow-density-model|snow-melt-model" crates/openwepp-runner/src/bin/openwepp-cli-hill.rs || true
```

Result:

- Authority anti-evasion: PASS.
- `auth11_required_suite_obligation_guards_contract`: PASS.
- Qwet/frzftp scan over `crates/`: no matches.
- User CLI selector scan over `openwepp-cli-hill.rs`: no matches.
