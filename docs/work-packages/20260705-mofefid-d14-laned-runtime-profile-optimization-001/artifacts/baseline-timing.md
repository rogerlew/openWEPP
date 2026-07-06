# Baseline Timing

Status: **EXECUTED (D14-S0)** — post-optimization re-measurement recorded in
the "After optimization" section once D14-S4/S5 complete.

Evidence mode: **Ran** unless labeled otherwise.

## Environment

- Commit: `1d7dc75acaacd7565ccc79dab8980bc46cc9f1e2` (main, clean tree).
- Host: Intel Xeon E5-2697 v2 @ 2.70 GHz, 48 logical CPUs, 125 GiB RAM, Linux
  6.8.0-111-generic.
- Toolchain: `rustc 1.92.0 (ded5c06cf 2025-12-08)`, `cargo-nextest 0.9.138`.
- Build: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  (workspace `[profile.release]`: `lto = "thin"`, `codegen-units = 1`).
- Runs pinned to one core with `taskset -c 4`; timing via `/usr/bin/time -v`.
- Fixture: `tests/fixtures/laned_shadow_h2637/` (19 OFEs, 731 days / 2 climate
  years), copied to a scratch run dir per variant. The `native` variants patch
  `p2637.man` exactly as
  `tests/integration/laned_shadow_h2637.rs::enable_native_cropland_routing_coefficients`
  does (datver `ow-lanuse-1`, landuse 4 NativeCropland, 19
  `routing_coefficients` blocks `500.0 0.0 0.0 0.0 0.0`).

## Commands

```sh
# default/off (legacy fixture, unpatched):
/usr/bin/time -v taskset -c 4 target/release/openwepp-cli-hill \
  --run-dir run_off --run-file p2637.run.toml --output-dir run_off/output

# default/off (native-routing patched fixture):
/usr/bin/time -v taskset -c 4 target/release/openwepp-cli-hill \
  --run-dir run_off_native --run-file p2637.run.toml --output-dir run_off_native/output

# Lane D shadow (native-routing patched fixture):
OPENWEPP_LANED_SHADOW=1 /usr/bin/time -v taskset -c 4 target/release/openwepp-cli-hill \
  --run-dir run_on --run-file p2637.run.toml --output-dir run_on/output
```

## Baseline results (3 runs each, commit `1d7dc75a`)

| Variant | Wall (s) | User (s) | Sys (s) | Max RSS (MB) |
|---|---|---|---|---|
| legacy / shadow off | 2.41 / 2.35 / 2.28 | 2.38 / 2.33 / 2.26 | 0.02 / 0.01 / 0.01 | ~21.5 |
| native / shadow off | 2.31 / 2.30 / 2.30 | 2.29 / 2.28 / 2.28 | 0.01 / 0.01 / 0.01 | ~20 |
| native / shadow ON | 67.60 / 67.71 / 67.60 | 67.57 / 67.68 / 67.57 | 0.01 / 0.01 / 0.01 | ~20.5 |

- **Release-grade Lane D shadow overhead: ~65.3 s user CPU**
  (67.6 − 2.3) on this 2-year H2637 fixture — a ~29× multiplier over the
  default path. Repeatability is excellent (±0.1 s across runs).
- The campaign's earlier `+207 s` figure was an estimate from dev-profile
  test runs; this artifact re-pins the baseline at release optimization on a
  pinned core. Static: the dev-profile (`cargo nextest`) H2637 evidence test
  remains the continuity surface (D13 recorded `325.24 s` for its off+on
  double run).
- Memory is flat (~20 MB) with the shadow on or off — the shadow cost is CPU,
  not RSS.

## Protected-output identity (baseline, shadow off vs on)

`sha256sum` on the native-patched fixture outputs (run 3):

- `H2637.hbp`: `948faf82c7edc2a60177b9567a92d8e6999f2d95e1d6f13953fda48b492c0467`
  — identical off/on.
- `H2637.pass.parquet`:
  `f0d1be11ee9f24b407479a7cdad7e3229981c49b7d2cadb179d4f0a74027e2a3`
  — identical off/on.

## Shadow diagnostic reference values (must stay bit-identical through D14)

Manifest `laned_shadow` block, shadow-on baseline at `1d7dc75a`:

```json
{
 "days_seen": 731,
 "days_routed": 622,
 "days_uniform_shape": 6,
 "days_uniform_shape_with_routed_melt": 0,
 "days_uniform_shape_without_routed_melt": 6,
 "max_router_conservation_rel": 0.5049051203739849,
 "aggregate_router_conservation_rel": 0.08236358856103747,
 "max_supply_reconstruction_rel": 5.434281268840262e-16,
 "total_source_m3": 1769606.816247753,
 "total_routed_outlet_m3": 1678721.4446571462
}
```

Note (Static): the aggregate router conservation figure at current main is
`8.24 %` — higher than the `6.0 %` recorded at the rev-15 shadow landing.
The D11 rev-21 dynamic friction operands and the D12 melt limb landed after
that measurement and legitimately moved the diagnostic; the figure remains
inside the `<15 %` diagnostic bound and is not a D14 surface. D14 treats the
values above as the bit-identity reference for behavior preservation.

## After optimization (D14-S4, OPT-1/2/3 landed)

Same commands, environment, and pinned core; optimized release binary
(instrumentation present, profiling off). First series measured on the
immediately-post-OPT build; the table below is the **final-code
confirmation series** (post-`cargo fmt` rebuild, idle machine, 3 runs):

| Variant | Wall (s) | User (s) | Sys (s) | Max RSS (MB) |
|---|---|---|---|---|
| legacy / shadow off | 2.32 / 2.30 / 2.31 | 2.30 / 2.28 / 2.30 | 0.01 / 0.01 / 0.01 | ~21.5 |
| native / shadow off | 2.29 / 2.27 / 2.34 | 2.28 / 2.25 / 2.32 | 0.01 / 0.01 / 0.01 | ~20 |
| native / shadow ON | 29.87 / 30.09 / 29.95 | 29.85 / 29.82 / 29.93 | 0.01 / 0.01 / 0.01 | ~21 |

(The immediately-post-OPT series read 29.78/29.88/29.88 wall — identical
within jitter; one interim confirmation run that overlapped the delegated
full-suite gate read 31.2 s and is discarded as load-contaminated.)

- **Shadow-on endpoint: 67.6 s → 29.9 s wall (2.26×). Shadow overhead:
  65.3 s → 27.6 s user CPU (−58 %).** The shadow multiplier over the
  default path drops from ~29× to ~13×.
- Default/off timing is unchanged (2.3–2.4 s, inside baseline jitter), and
  default/off protected outputs are byte-identical to the pre-optimization
  references (`protected-output-evidence.md`).
- Shadow-on protected outputs and the full `laned_shadow` manifest
  diagnostic block are bit-identical to the pre-optimization baseline; the
  trajectory counters are identical (10,334,879 steps, 1,134,300 samples,
  10,155,779 upstream interpolations before and after), which together with
  the bit-identical diagnostics witnesses trajectory preservation.
