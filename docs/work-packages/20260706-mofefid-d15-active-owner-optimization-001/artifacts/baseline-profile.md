# Baseline Profile (D15A-S0)

Status: **EXECUTED**.

Evidence mode: **Ran** (commands below executed in this session) unless labeled
Static.

## Environment

- Base: `main` @ `9f536aad` (scaffold commit over required base `94a7ac3a`),
  clean tree.
- Binary: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`,
  stable toolchain per `rust-toolchain.toml`.
- Host: 48-core Linux 6.8.0-111-generic; runs pinned with `taskset -c 4`.
- Fixture: `tests/fixtures/laned_shadow_h2637` copied to scratchpad run dirs,
  `p2637.man` patched exactly as the `laned_shadow_h2637` native test helper
  (`ow-lanuse-1` datver, `Cropland` → `NativeCropland`, 19
  `routing_coefficients` extensions `500.0 0.0 0.0 0.0 0.0`) — the same recipe
  the D15 blocker-resolution timing refresh used.
- Session note: no `timing_comparator` / `comparator_suite_runner` subagent
  type exists in this session's tool registry; per the package's subagent
  clause the block is recorded and timing ran locally with the exact commands
  below.

## Endpoint timing (profiling OFF)

Command shape:

```
[env] /usr/bin/time -v taskset -c 4 target/release/openwepp-cli-hill \
  --run-dir <dir> --run-file p2637.run.toml --output-dir <dir>/output
```

| Case | Run | User | Sys | Wall |
|---|---|---:|---:|---:|
| default/off | 1 | 2.64 s | 0.03 s | 0:03.19 |
| default/off | 2 | 2.38 s | 0.01 s | 0:02.40 |
| default/off | 3 | 2.45 s | 0.01 s | 0:02.47 |
| `OPENWEPP_LANED_SHADOW=1` | 1 | 93.29 s | — | 1:33.32 |
| `OPENWEPP_LANED_SHADOW=1` | 2 | 92.41 s | — | 1:32.45 |
| `OPENWEPP_LANED_SHADOW=1` | 3 | 90.63 s | — | 1:30.66 |

Medians: default/off `2.45 s` user / `2.47 s` wall; shadow-on `92.41 s` user /
`92.45 s` wall. This reproduces the blocker-resolution refresh (`91.59 s` user /
`1:31.67` wall) within run-to-run jitter (±1.4 s over 3 runs). Overhead over
default/off: **+89.96 s** user (median), ~`37x` the default path, ~`3.09x` the
D14 optimized budget (`29.9 s`).

## Slot profile (profiling ON, one run)

`OPENWEPP_LANED_SHADOW=1 OPENWEPP_LANED_SHADOW_PROFILE=1`: `94.19 s` user /
`1:34.23` wall (instrumentation overhead ~+1.9 % vs the shadow-on median).

```
rows_observed=13889 days_seen=731 days_routed=622
solver_runs=11818 solver_steps=16936089 alpha_evaluations=302411532
hydrograph_samples=1412726 upstream_interpolation_calls=16714893
cascade_run_ns=91600147877 solver_cfl_ns=64280756220
solver_step_ns=23751413101 solver_sample_ns=1086945720
operand_build_ns=12614555 observe_row_ns=13154341
mesh_build_ns=2779262 rate_series_ns=4408722 solver_setup_ns=9467052
```

Identical counters to the blocker-resolution profile (16,936,089 steps;
302,411,532 alpha evaluations) — the baseline is deterministic and matches the
recorded regression state.

## Regression decomposition vs D14 (Static, from counters + code read)

| Quantity | D14 post-opt | Current (D10B-corrected) | Factor |
|---|---:|---:|---:|
| solver steps | 10,334,879 | 16,936,089 | 1.64x |
| alpha evaluations / step | 10.0 (one per cell) | 17.86 | 1.79x |
| `solver_cfl` slot | 17.60 s | 64.28 s | 3.65x |
| `solver_step` slot | 10.51 s | 23.75 s | 2.26x |
| `solver_cfl` per step | 1.70 µs | 3.80 µs | 2.23x |
| `solver_step` per step | 1.02 µs | 1.40 µs | 1.38x |
| shadow-on wall | 29.9 s | ~92 s | ~3.06x |

Two independent regression mechanisms, both introduced by the D10B correction
set (rev 24-26) and the D15 blocker fix:

1. **Step count +64 % — contract-mandated.** The rev-26 TRUE kinematic
   celerity (`dq/dh` through the friction fixed point, floored at frozen-alpha)
   is up to ~2x the old frozen-alpha estimate on the laminar limb, so the
   CFL-selected `dt` halves where the old scheme ran at true Courant ~1.8
   (a latent instability, D10B S4). The drain-tail restoration (blocker fix:
   24 h source window + 6 h tail instead of the one-day cap) adds window time
   on late-source days. Step count is the trajectory: it cannot be reduced
   without changing `dt` selection, i.e. without a numerical-method change
   that the package excludes.
2. **Per-step cost +2.2x in the CFL slot / +1.4x in the step slot —
   partially implementation overhead.** The celerity evaluation adds one
   perturbed friction fixed-point per WET cell per step
   (302.4M − 169.4M base = 133.0M perturbed evaluations) and recomputes
   `q = alpha·h^1.5` outside the fixed point at both the base and perturbed
   depths (2 redundant `powf` per wet cell per step). The rev-24 TV(q)
   homogeneous diagnostic recomputes `alpha·h^1.5` for up to 4(n−1) cell-faces
   per homogeneous step even though the same values exist (pre-step `q` from
   the celerity evaluation; committed `q` from the commit loop). `slope.sqrt()`
   is recomputed inside every friction fixed-point iteration. The
   material-interface detector compares full 9-field `CellParameters` per face
   per step instead of a precomputed per-face flag.

Slot-attribution note (Static): `solver_cfl` times `prepare_step_alpha`
(base + perturbed alpha evaluations and celerity) plus dt selection and the
Courant-evidence loop; `solver_step` times the predictor/corrector/TVD/commit
sweeps and the ledger.

## Preservation-witness baseline

SHA256 (shadow-on run):

```
948faf82c7edc2a60177b9567a92d8e6999f2d95e1d6f13953fda48b492c0467  H2637.hbp
725f57233fd60df097a824a2c20f26992a58b3a457594245a9ac91d2278f3cfb  H2637.loss.json
f0d1be11ee9f24b407479a7cdad7e3229981c49b7d2cadb179d4f0a74027e2a3  H2637.pass.parquet
```

Protected outputs are byte-identical between shadow-on and default/off
(`INV-OFEROUTE-010` holds at baseline). Manifest `laned_shadow` block (the
routed-diagnostics witness the optimizations must preserve bit-identically):

```
days_seen=731 days_routed=622
aggregate_router_conservation_rel=3.232606608593326e-13
max_router_conservation_rel=3.3424685678652526e-12
max_supply_reconstruction_rel=5.434281268840262e-16
total_source_m3=1769606.816247753
total_routed_outlet_m3=1837037.5470260957
days_uniform_shape=6 (0 with routed melt / 6 without)
```

Observation carried to the P-phase (Static): `total_routed_outlet_m3` exceeds
`total_source_m3` by ~3.8 % while the clamp-adjusted residual is machine-zero —
by the exact identity `outlet = source + clamp − ΔS`, the positivity-clamp
injection on this fixture is ≥ 67,430 m³ (≥ 3.8 % of source). This is the
landed, booked D10B behavior (`INV-OFEROUTE-006` surfaces the clamp term), but
the active-owner closure identity (`INV-OFEROUTE-012`) must book the clamp term
explicitly or the hillslope-day water balance cannot close exactly under active
routing. Recorded for `operand-lineage.md` / `activation-readiness-audit.md`.
