# Baseline Binary And Output Evidence

Status: `complete / PASS`

Evidence mode: `Ran: exact scaffold commit release build and frozen Snowbird fixture`

## Source And Binary

- Scaffold commit: `3490ca1531065c5b0d1b56333eee4725060b0217`.
- Detached source worktree: exact scaffold commit; no production diff.
- Build: `CARGO_TARGET_DIR=/home/workdir/openWEPP/target/snow_mass_transition_ledger_persistence/baseline_build cargo build --release -p openwepp-runner --bins`.
- Preserved binary:
  `target/snow_mass_transition_ledger_persistence/binaries/openwepp-cli-hill-scaffold-3490ca15`.
- SHA-256: `464c87e16f24997753627d83399979b1f4bcc232629196c1d9847a7f9d0bb407`.
- Size: `11232920` bytes; copied mtime `2026-08-03 22:34:26 -0700`.

## Fixture And Selector State

- Immutable source:
  `target/snow_prepeak_liquid_evacuation_physics_audit_v3/fixtures/baseline_replay/snotel_snowbird_ut`.
- Package copy: `target/snow_mass_transition_ledger_persistence/fixtures/snotel_snowbird_ut`.
- Nine-file hash manifest:
  `target/snow_mass_transition_ledger_persistence/manifests/fixture-sha256.txt`;
  manifest SHA-256
  `16571c3b8c0d5fe692ae7ff1552cae78485368d19305275421a4ef2dd448c36e`.
- Stage 3 `layered_thermal_liquid_v1`, multilayer density,
  Harder-Pomeroy hourly phase, CoE liquid holding, and disabled explicit
  longwave/sublimation were frozen. Every other `OPENWEPP_*` variable was
  removed.

## Trace-Enabled Reference

- Command driver:
  `.venv/bin/python docs/work-packages/20260803-snow-mass-transition-ledger-persistence-001/tools/ledger_persistence.py suite --build baseline --binary target/snow_mass_transition_ledger_persistence/binaries/openwepp-cli-hill-scaffold-3490ca15`.
- Schema-v4 rows: `14245`.
- Trace bytes: `659499507`; SHA-256
  `84a64c1b4031584842c4d20023acac92fcffbea946fcce04953fb5a0a339fb5f`.
- WAT SHA-256:
  `e74b8df25485f6e1dd1430a9332c4aab3bafb8498a228f5455611d8081521b75`.
- HBP/PASS SHA-256:
  `d5d3468d361510df069475423f785e2be036e0b353c281d0c32d0f82b583c149`.
- Wall time / peak RSS: `10.2263 s / 35764 KiB`.
- Upstream alias identity maximum error: `1.3878e-17 m`; Stage-3 identity
  maximum error: `1.2272e-17 m`.
- All `8615` Stage-3-enabled rows link exact incoming liquid to the upstream
  routed-liquid handoff; raw signed melt differs from snowpack loss on `3844`
  rows.

## Trace-Disabled Reference

One unmeasured-policy warm-up preceded seven retained measurements:

| Sample | Wall seconds | Peak RSS KiB |
|---|---:|---:|
| 01 | 4.269572 | 35560 |
| 02 | 4.249726 | 35932 |
| 03 | 4.331499 | 35932 |
| 04 | 4.226543 | 35932 |
| 05 | 4.279442 | 35548 |
| 06 | 4.198543 | 35536 |
| 07 | 4.879049 | 36128 |
| median | 4.269572 | 35932 |

Every disabled run reproduced the WAT and HBP/PASS hashes above and produced
no snow JSONL file. Raw receipts live under
`target/snow_mass_transition_ledger_persistence/runs/baseline/`.

## Baseline Type Footprint

An exact-release `rustc --extern` size probe reports:

| Type | Bytes |
|---|---:|
| `DirectSnowLiquidPartition` | 15816 |
| `DirectSnowAccumulationMeltDiagnostics` | 5808 |
| `DirectSnowStage3Diagnostics` | 9432 |

The baseline gate is complete before contract, test, or production edits.
