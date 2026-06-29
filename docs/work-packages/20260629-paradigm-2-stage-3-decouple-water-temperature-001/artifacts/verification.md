# Verification

Status: `RAN-PASS`

Evidence class: Ran + Static.

## Rust and Dependency Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo test --workspace` | PASS |
| `cargo deny check` | PASS: advisories, bans, licenses, and sources ok |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | PASS, `2` tests |

The first full workspace-test attempt found only stale integration-test contract
markers that still expected `contract_version: 110`; after updating the markers
to v111, the full workspace run passed.

## Focused Stage 3-Decouple Gates

| Gate | Result |
| --- | --- |
| `cargo test --test paradigm2_stage3_decouple_water_temperature` | PASS, `3` tests |
| `cargo test --test paradigm2_stage3_liquid_routing_meltwater_temperature` | PASS, `6` tests |
| `cargo test --test snowdensity03_physics_bulk_offline_contract physics_bulk_runtime_mentions_are_confined_to_authorized_opt_in_surfaces` | PASS |
| `cargo test --test paradigm2_stage0_surface_energy_balance_contract` | PASS |

## Observed Guardrail

Command:

```bash
.venv/bin/python tools/snowfreeze_observed/paradigm2_stage3_decouple_water_temperature.py \
  --hill-binary target/release/openwepp-cli-hill \
  --output-dir target/paradigm2_stage3_decouple_water_temperature \
  --package-artifacts-dir docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001/artifacts
```

Result: PASS.

- Current no-env default: `15` robust fails / `179`.
- Stage 3-Decouple: `15` robust fails / `179`.
- Robust cells decoupled vs default: `0` better / `90` equal / `0` worse.
- Runoff/timing cells decoupled vs default: `0` better / `40` equal / `0`
  worse.
- Real-run elapsed: `113.352 s`.

The JSON and Markdown evidence are recorded in
`paradigm2-stage3-decouple-observed-guardrails.{json,md}`.

## Performance

H2637 direct-production endpoint run with only
`OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1` set: PASS.

- Wall time: `70.68 s`.
- Max RSS: `1150612 KiB`.
- ADR-0025 budget: `<=91.2 s`.
- Ratio to legacy H2637 reference (`9.12 s`): `7.75x`.

See `performance-h2637.md` for command, binary hash, and selected output hashes.

## Markdown

| Gate | Result |
| --- | --- |
| `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/work-packages/README.md --path docs/planning/snow-frost-fidelity-strategy.md --path docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001 --format json` | PASS, `12` files, `0` errors, `0` warnings |
| `markdown-doc validate --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/work-packages/README.md --path docs/planning/snow-frost-fidelity-strategy.md --path docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001` | PASS, `12` files, `0` errors |

`wctl doc-lint --path docs/work-packages/20260629-paradigm-2-stage-3-decouple-water-temperature-001`
also exited `0`, but this checkout's `wctl` wrapper does not expose path options
and reported `0` files validated, so the disposition relies on `markdown-doc`
for scoped Markdown evidence.
