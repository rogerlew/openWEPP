# Gate Results

Status: complete

Evidence mode: `Ran`

Ran:

| Gate | Command | Result |
|---|---|---|
| Pre-fix red test | `cargo test -p openwepp-hillslope-orchestrator wbval02_rejects_daily_radiation_above_baseline_sunmap_potential -- --nocapture` | Expected fail before production edit; runtime surface built instead of failing at `radly`. |
| Pre-fix red test | `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_rejects_physically_impossible_radiation -- --nocapture` | Expected fail before production edit; error symbol remained `winter.hourly.rad_mj_m2_0009`. |
| Targeted WBVAL02 test | `cargo test -p openwepp-hillslope-orchestrator wbval02_rejects_daily_radiation_above_baseline_sunmap_potential -- --nocapture` | Pass. |
| Targeted impossible-radiation test | `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_rejects_physically_impossible_radiation -- --nocapture` | Pass. |
| Targeted climate-runtime regression | `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context -- --nocapture` | Pass, `7` tests. |
| Package tests | `cargo test -p openwepp-hillslope-orchestrator` | Pass, `101` tests. |
| Format | `cargo fmt --check` | Pass. |
| Clippy | `cargo clippy --workspace --all-targets -- -D warnings` | Pass. |
| Release build | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass. |
| Deny | `cargo deny check` | Pass with existing duplicate/unmatched-license warnings. |
| Workspace tests | `cargo test --workspace` | Fail outside WBVAL02: `adr0017_is_accepted_and_registered_as_ratified_governance` assertion in `tests/integration/adr0017_comparator_distrust_ratification_contract.rs` expects a decisions README row not present in the current worktree. |

Six-wrapper validation command pattern:

```text
target/release/openwepp-cli-hill \
  --run-dir /wc1/runs/in/indispensable-presenter/wepp/runs \
  --run-file /tmp/wbval01_rocky_mountain_20260606T000000Z/generated_runfiles_nodiscovery/<p>.toml \
  --output-dir /tmp/wbval02_after/<p> \
  --policy compat
```

Six-wrapper validation result:

- `p2`, `p4`, `p6`, `p9`, `p14`, and `p17` all return `RC=1` with
  `CLIM-RUNTIME-E-017: runtime context symbol radly=486 is out of domain
  (allowed 0 <= radly <= baseline sunmap horizontal daily potential
  (rpoth/r3))`.
- This satisfies the WBVAL02 invalid-upstream acceptance path.

Documentation gate:

- `markdown-doc lint --path docs/work-packages/20260606-wbval02-simimpl28-radbound-defect-closure-001 --no-ignore`
  passed, `22` files validated.
- `markdown-doc lint --path docs/work-packages/README.md --no-ignore` passed.
- `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md --no-ignore`
  passed.
