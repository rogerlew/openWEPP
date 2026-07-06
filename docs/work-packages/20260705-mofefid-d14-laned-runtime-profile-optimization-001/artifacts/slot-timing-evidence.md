# Slot Timing Evidence

Status: **EXECUTED (D14-S2)**.

Evidence mode: **Ran** unless labeled otherwise. Environment and fixture per
`baseline-timing.md` (release CLI, `taskset -c 4`, native-patched H2637,
`OPENWEPP_LANED_SHADOW=1`, 67.6 s shadow-on wall at baseline).

Two independent instruments agree:

1. **`perf record` function-level attribution** (no code change; release +
   `CARGO_PROFILE_RELEASE_DEBUG=line-tables-only` rebuild at `1d7dc75a`).
2. **D14 persistent slot diagnostics** (`ofe_routing::profile` +
   `laned_shadow` runner slots, opt-in `OPENWEPP_LANED_SHADOW_PROFILE=1`,
   stderr JSON at collector finalize; discoverable by local CI agents via the
   focused tests named at the bottom).

## 1. perf function-level profile (shadow-on, 73,274 samples)

```
perf record -F 999 -o perf_shadow_on.data -- taskset -c 4 \
  target/release/openwepp-cli-hill --run-dir run_on --run-file p2637.run.toml \
  --output-dir run_on/output       # with OPENWEPP_LANED_SHADOW=1
```

| % cycles | Symbol |
|---|---|
| 34.14 | `KinematicWaveSolver::run` (self; step/cfl/alpha inlined) |
| 27.39 | `libm __ieee754_pow_sse2` |
| 8.16 | `libm pow@GLIBC` |
| 14.43 | `CellParameters::equivalent_friction` (self) |
| 5.46 | kernel |
| 1.28 | `route_buffered_day` closure (seam rate fetches) |
| 1.16 | `_int_free` |

- **`pow` alone is ~35.6 %** of the entire shadow-on run (skin-term
  `I^0.407` per friction evaluation; `h^1.5` per alpha evaluation and twice
  per cell per step in the discharge updates).
- `perf annotate` line attribution inside `run` lands overwhelmingly on the
  `CellParameters::alpha` fixed-point loop (kinematic_wave.rs:149–162:
  `equivalent_friction` call, `chezy * sqrt(slope)`, `q_new = alpha * h_pow`,
  convergence test), visible in **three distinct inlined copies** — the
  `cfl_dt` loop, the CFL-evidence loop, and the `step` alpha loop.
- Allocation (`malloc`/`_int_free`) totals ~2 % despite 8 `Vec` allocations
  per step — measurable but not dominant.

## 2. Persistent slot diagnostics (H2637 shadow-on, profiling enabled)

```
OPENWEPP_LANED_SHADOW=1 OPENWEPP_LANED_SHADOW_PROFILE=1 /usr/bin/time -v \
  taskset -c 4 target/release/openwepp-cli-hill --run-dir run_on \
  --run-file p2637.run.toml --output-dir run_on/output
```

Profiling-enabled wall: 69.25 s (**+2.4 %** instrumentation overhead vs the
67.6 s profiling-off run; endpoint timing gates always run profiling-off).

Counters:

| Counter | Value |
|---|---|
| rows observed (lane-days) | 13,889 |
| days routed | 622 (of 731) |
| solver runs (OFE-days) | 11,818 |
| solver steps | 10,334,879 (avg 875 / OFE-day) |
| alpha evaluations | 249,324,376 (**24.1 / step**) |
| hydrograph samples | 1,134,300 |
| upstream interpolation calls | 10,155,779 |

Slots (disjoint unless noted; % of the 67.6 s profiling-off wall):

| Slot | Time | Share |
|---|---|---|
| `cascade_run` (contains the three solver slots below) | 66.85 s | 98.9 % |
| — `solver_cfl` (CFL dt selection + Courant evidence) | 34.77 s | 51.4 % |
| — `solver_step` (predictor/corrector/TVD/commit/ledger + its alpha loop) | 31.23 s | 46.2 % |
| — `solver_sample` (hydrograph interpolation + push) | 0.06 s | 0.1 % |
| — `solver_setup` (per-OFE mesh clone + construction) | 0.004 s | ~0 % |
| `operand_build` (runner day-frame operand extraction) | 0.01 s | ~0 % |
| `observe_row` (runner row validation + depth build) | 0.01 s | ~0 % |
| `rate_series` (seam source-rate construction) | 0.003 s | ~0 % |
| `mesh_build` (runner segment construction) | 0.003 s | ~0 % |

Sub-slot residual inside `cascade_run` (66.85 − 34.77 − 31.23 − 0.06 −
0.004 ≈ 0.78 s) is loop bookkeeping, forcing-closure dispatch, and the
upstream-boundary binary searches not separately timed (perf bounds
`interpolate_unit_discharge` under 0.8 %).

## 3. Attribution to the package's named slots

| Package slot | Measured share | Basis |
|---|---|---|
| `ofe_routing::cascade` / `KinematicWaveSolver` solver math | ~97 % of shadow overhead (cfl 51.4 % + step 46.2 %); within it, libm `pow` 35.6 % and the friction/alpha fixed-point ~50 % | slots + perf |
| Per-day/OFE setup (mesh build, solver construction) | < 0.1 % | slots |
| Allocation / cloning / vector construction | ~2 % (malloc/free; 8 Vecs per step) | perf |
| Hourly hydrograph sampling/interpolation + source-rate construction | ~0.2 % (samples 0.06 s + rate series 0.003 s + upstream interp < 0.8 %) | slots + perf |
| Friction/forcing operand construction (runner side) | < 0.1 % (operand_build + observe_row) | slots |
| Handoff and closure diagnostics | < 1 % (upstream interp + residual bookkeeping) | slots + perf |
| Runner/publication integration | < 0.1 % | slots |

**Headline: the Lane D overhead is solver math, and half of it is redundant.**
The CFL slot (51.4 %) exists only to *re-evaluate* `alpha` for the same
`(h, q, I)` state that `step` evaluates again: 24.1 alpha evaluations per
step on a 10-cell mesh (≈ 2 × wet-cells in cfl+evidence + 10 in step) where
one evaluation per cell suffices. The skin-term `I^0.407` is recomputed on
every friction evaluation although `I` is constant within a step.

## 4. Repeatability

Baseline endpoint runs vary by ±0.1 s over 3 runs (see
`baseline-timing.md`). The slot run was executed once (its purpose is
attribution, not endpoint acceptance); counters are deterministic
(bit-identical solver trajectory), only the ns slots carry timing jitter.

## 4a. Post-optimization slot profile (OPT-1/2/3 landed)

Same command, optimized binary (profiling-on wall 31.31 s vs 29.8 s
profiling-off — +5 % instrumentation overhead at the faster runtime):

| Counter | Pre-opt | Post-opt |
|---|---|---|
| solver runs | 11,818 | 11,818 (identical) |
| solver steps | 10,334,879 | 10,334,879 (identical — trajectory witness) |
| alpha evaluations | 249,324,376 (24.1/step) | 103,348,790 (**10.0/step** — exactly one per cell per step) |
| hydrograph samples | 1,134,300 | 1,134,300 (identical) |
| upstream interpolation calls | 10,155,779 | 10,155,779 (identical) |

| Slot | Pre-opt | Post-opt |
|---|---|---|
| `cascade_run` | 66.85 s | 28.91 s |
| `solver_cfl` (now: the single per-step alpha evaluation + dt selection + Courant evidence) | 34.77 s | 17.60 s |
| `solver_step` (sweeps only; alpha no longer recomputed) | 31.23 s | 10.51 s |
| `solver_sample` | 0.06 s | 0.05 s |
| runner-side slots | ≤ 0.02 s | ≤ 0.02 s |

Post-opt perf: `KinematicWaveSolver::run` self 52.2 %, libm `pow` 31.0 %
(down from 35.6 % of a run 2.27× longer — absolute pow time ~24 s → ~9 s),
kernel 5.4 %, seam-rate closure 1.9 %. The remaining pow load is the
irreducible-under-bit-identity `h^1.5` (once per alpha evaluation plus twice
per cell per step in the discharge updates); replacing it with
`h * h.sqrt()` is rejected as not bit-identical (OPT-4).

Slot-attribution note: after OPT-1 the `solver_cfl` slot's meaning shifts —
it now times the fused block (one alpha evaluation per cell + dt selection +
Courant evidence) rather than redundant recomputation; `solver_step` times
the pure predictor/corrector/TVD/commit sweeps.

## 5. Where the diagnostics live (local-CI discoverable)

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/profile.rs` —
  thread-local slot accumulator; enable via
  `ofe_routing::profile::set_enabled` (runner arms it from
  `OPENWEPP_LANED_SHADOW_PROFILE=1`).
- Focused tests:
  `kinematic_wave::tests::profile_slots_accumulate_when_enabled_and_stay_zero_when_disabled`,
  `cascade::tests::cascade_profile_counts_setup_and_upstream_interpolation`,
  `laned_shadow::tests::runner_profile_slots_accumulate_for_routed_day`,
  plus the `profile::tests` unit pair.
- Report format: one stderr line `laned_shadow_profile {…}` at collector
  finalize; protected outputs and the manifest are untouched (verified:
  shadow-on HBP/parquet SHA256 and the manifest `laned_shadow` block are
  identical with instrumentation present, profiling on or off).
- Interpretation notes (review-driven): `OPENWEPP_LANED_SHADOW_PROFILE=1`
  without `OPENWEPP_LANED_SHADOW=1` is inert — the profiler is armed by the
  shadow collector, which only exists when the shadow is on. The
  `alpha_evaluations` counter counts `CellParameters::alpha` *calls*
  (dry-cell calls early-return cheaply), so on partially-dry meshes it
  overstates full friction-menu evaluations; on this fixture the wet
  fraction dominates the cost either way. The enable flag is
  process-global while the accumulator is thread-local; the report reads
  the finalizing thread's slots, which is correct for the single-threaded
  hillslope runner (watershed parallelism is subprocess-per-hillslope).
