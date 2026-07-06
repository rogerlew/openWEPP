# Protected Output Evidence

Status: **EXECUTED (D14-S5)**.

Evidence mode: **Ran**. Environment, fixture, and commands per
`baseline-timing.md` (release CLI, `taskset -c 4`, H2637 fixture; `native`
variants patch `p2637.man` exactly as the integration test does).

## Reference set (pre-optimization, commit `1d7dc75a`)

Captured before any D14 code change:

| Artifact | SHA256 |
|---|---|
| legacy/off `H2637.hbp` | `453e441cf065544fccb41737145ed228625cf9386423671e2a63887ecf0c072f` |
| legacy/off `H2637.pass.parquet` | `fd47c341b300860de7007164ad0797cec4f70985958869eff829daf75a20966a` |
| native/off `H2637.hbp` | `948faf82c7edc2a60177b9567a92d8e6999f2d95e1d6f13953fda48b492c0467` |
| native/off `H2637.pass.parquet` | `f0d1be11ee9f24b407479a7cdad7e3229981c49b7d2cadb179d4f0a74027e2a3` |
| native/shadow-on `H2637.hbp` | `948faf82…c0467` (== native/off) |
| native/shadow-on `H2637.pass.parquet` | `f0d1be11…7e2a3` (== native/off) |

## After D14 (instrumentation + OPT-1/2/3)

Ran on the optimized release binary:

- **Default/off byte identity:** legacy/off and native/off `H2637.hbp` +
  `H2637.pass.parquet` are byte-identical to the pre-optimization reference
  copies (`cmp` clean on all four files).
- **Shadow-off manifest:** contains no `laned_shadow` keys (checked on the
  native/off manifest).
- **Shadow-on protected outputs:** `H2637.hbp` and `H2637.pass.parquet`
  SHA256 match the pre-optimization values exactly (`948faf82…c0467`,
  `f0d1be11…7e2a3`) — shadow on/off identity holds and is unchanged by the
  optimization.
- **Routed-path closure/diagnostic parity:** the manifest `laned_shadow`
  block is **bit-identical** to the pre-optimization baseline
  (JSON-canonical comparison of every field):
  `days_seen=731`, `days_routed=622`, `days_uniform_shape=6` (0 with routed
  melt / 6 without), `max_router_conservation_rel=0.5049051203739849`,
  `aggregate_router_conservation_rel=0.08236358856103747`,
  `max_supply_reconstruction_rel=5.434281268840262e-16`,
  `total_source_m3=1769606.816247753`,
  `total_routed_outlet_m3=1678721.4446571462`.
- **Trajectory identity witness:** the D14 counters record exactly the same
  solver trajectory before and after optimization — `solver_steps=10,334,879`,
  `hydrograph_samples=1,134,300`, `upstream_interpolation_calls=10,155,779`
  on both binaries. Only `alpha_evaluations` changed (249,324,376 →
  103,348,790), which is the removed redundant work itself, not a numerical
  change.
- **Instrumentation neutrality:** with `OPENWEPP_LANED_SHADOW_PROFILE`
  unset, the instrumented binary produced byte-identical protected outputs
  and an identical `laned_shadow` manifest block, at unchanged endpoint
  timing (67.64 s vs 67.57–67.68 s baseline) before the optimizations were
  applied. With profiling enabled, the only output difference is one stderr
  `laned_shadow_profile {…}` line (+2.4 % runtime).

## Dev-profile endpoint confirmation

The `#[ignore]`d `h2637_native_shadow_classifies_uniform_shape_after_d12`
integration test (asserts HBP/parquet byte equality off/on plus the D12
uniform-shape counter population) was rerun on the final code — see
`gate-results.md`.
