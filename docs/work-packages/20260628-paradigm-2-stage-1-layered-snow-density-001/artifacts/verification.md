# Verification

Evidence class: `Ran`

## Commands

| Command | Result |
|---|---|
| `.venv/bin/python tools/snowfreeze_observed/paradigm2_stage1_layered_density.py` | Pass as execution; promotion gates fail: candidate `16/177` vs default `15/179`. |
| `cargo test --test paradigm2_stage1_layered_snow_density` | Pass, `6` tests. |
| `cargo test -p openwepp-hillslope-orchestrator r7b_constructor_type_size_layout_is_bounded -- --nocapture` | Pass; final sizes: `DirectDayConstructorInputs=4008`, `DirectLaneFrame=1136`. |
| `cargo test -p openwepp-runner hillslope::tests::r7g_direct_production_reads_winter_column_snow_not_runtime_carry` | Pass. |
| `cargo fmt --check` | Pass. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass. |
| `cargo deny check` | Pass. |
| `cargo test --workspace` | Pass. |
| `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md --path docs/work-packages/20260628-paradigm-2-stage-1-layered-snow-density-001` | Pass, `7` files, `0` errors, `0` warnings. |

## Real-Run Result

The observed-corpus Stage 1 runner executed the current no-env default and the
opt-in `physics_bulk_multilayer_density_v1` candidate across the
cross-SNOTEL+cancov corpus.

- Default robust profile: `15` fails / `179` score.
- Candidate robust profile: `16` fails / `177` score.
- Candidate better robust cells: `3`.
- Candidate worse robust cells: `6`.
- Activation authorized: `false`.
- Elapsed time: `712.821 s`.

Candidate conservation and layer closure:

- Trace rows: `159986`.
- Max snow-state residual: `8.881784197001252e-16 m`.
- Max partition residual: `5.551115123125783e-17 m`.
- Rows with nonempty layer stack after partition: `65459`.
- Max layer SWE residual after partition: `4.440892098500626e-16 m`.
- Max layer depth residual after partition: `0.0 m`.
- Tolerance: `1e-9 m`.

## Fixed During Verification

- The first observed-corpus run exposed an aggregate density cap precision issue
  at `522.0000000000001 kg m^-3`; the emitted aggregate density is now clamped to
  the active runtime cap.
- The first full workspace run exposed a fixed-size direct-frame guard
  regression. Optional snow runtime mirrors in constructor/lane frames are boxed;
  the focused size guard now passes.
- The next workspace run exposed a stale source-guard assertion for the
  winter-column snow projection helper after the snow state became borrowed; the
  production source was already reading `lane.winter_column.snow`, and the guard
  now checks the current `&snow_lane_state` call shape.
