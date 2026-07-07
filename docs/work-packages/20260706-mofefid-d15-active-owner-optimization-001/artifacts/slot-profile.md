# Slot Profile (D15A-S1/S2)

Status: **EXECUTED**.

Evidence mode: **Ran** unless labeled Static. Environment/fixture per
`baseline-profile.md`.

## S1 instrumentation decision

The D14 persistent slot diagnostics (`ofe_routing::profile`, opt-in
`OPENWEPP_LANED_SHADOW_PROFILE=1`) plus the existing counters already
attribute the regression to explicit slots without any code change:

- `solver_cfl` (64.28 s) times `prepare_step_alpha` — the base per-cell alpha
  evaluation AND the rev-26 perturbed-depth celerity evaluation — plus dt
  selection and Courant evidence.
- `solver_step` (23.75 s) times the predictor/corrector/TVD/commit/ledger
  sweeps.
- The base/perturbed split is derivable from counters alone:
  `alpha_evaluations − cells×steps = 302,411,532 − 169,360,890 = 133,050,642`
  perturbed evaluations (one per WET cell per step; wet fraction ≈ 78.6 %).

No new runtime instrumentation was added: extending slots inside the hot loop
would perturb the surface being measured, and the two-instrument protocol
(slots + `perf`) reproduces the D14 evidence shape. This satisfies D15A-S1
"reuse ... without changing outputs or control flow".

## S2 function-level perf profile (shadow-on, 95K samples)

```
CARGO_PROFILE_RELEASE_DEBUG=line-tables-only cargo build --release \
  -p openwepp-runner --bin openwepp-cli-hill
OPENWEPP_LANED_SHADOW=1 perf record -F 999 -o perf_shadow_on.data -- \
  taskset -c 4 target/release/openwepp-cli-hill --run-dir shadow_on \
  --run-file p2637.run.toml --output-dir shadow_on/output
```

| % cycles | Symbol |
|---|---|
| 55.27 | `KinematicWaveSolver::run_with_options` (self; step/cfl/alpha/friction inlined) |
| 26.36 | libm `__ieee754_pow_sse2` |
| 6.58 | libm `pow@@GLIBC` |
| 5.68 | kernel |
| 0.94 | `route_buffered_day` closure (seam rate fetches) |

**libm `pow` totals 32.9 %** of the run. Source-line attribution
(`perf report -s srcline`): the top solver lines are `kinematic_wave.rs:203`
(5.5 %, `h_pow = h.powf(1.5)` in the alpha fixed point), `:214-216` (8.1 %,
the fixed-point loop body: `chezy·slope.sqrt()`, `q_new = alpha·h_pow`,
convergence test), plus `friction.rs` lines totaling ~14 % (skin dispatch,
Froude construction, vegetation helper).

## Where the cycles actually go (Static, code read × counters)

Per step on the H2637 mesh (n = 10 cells, ~7.9 wet):

| Work | Count/step | Notes |
|---|---|---|
| alpha fixed-point evaluations (`CellParameters::alpha`) | 17.86 calls, up to 4 friction evaluations each (the laminar-limb fixed-point contraction ~0.5 rarely reaches the 1e-12 relative tolerance in 4 iterations, so the cap usually binds) | one `h.powf(1.5)` per call + per-iteration friction menu |
| redundant `q = alpha·h.powf(1.5)` at base + perturbed depth | 2 × wet ≈ 15.7 `powf` | `prepare_step_alpha` recomputes what the fixed point already produced |
| per-iteration friction waste on this fixture | ~71 friction evaluations | `froude_number` (powi+sqrt+div) computed unconditionally but consumed ONLY by the wave branch, which is dead when `D_r = 0`; `vegetation_resistance_katul` computes `beta` (sqrt+div) before discovering `C_d,veg = 0` ⇒ `L_c = 0` ⇒ 0 |
| `slope.sqrt()` inside the fixed-point loop | ~71 | loop-invariant |
| sweep `powf` (`q_pred`, commit) | 20 | irreducible under bit-identity (new depths) |
| TV(q) homogeneous diagnostic | up to 4(n−1) = 36 `powf` per homogeneous step | recomputes `alpha·h^1.5` although the pre-step values exist in the celerity evaluation and the committed values are produced by the commit loop |
| material-interface detection | 2 loops × (n−1) 9-field `CellParameters` compares per step | mesh is immutable per run; loop-invariant |

## Repeatability

Endpoint runs vary ±1.4 s over 3 runs (baseline-profile.md). Counters are
deterministic (bit-identical trajectory): the profile run reproduced the
blocker-resolution counters exactly.

## Reduced fixture

None needed: the full H2637 run is ~92 s and deterministic; slot + perf
evidence on the endpoint fixture is directly the acceptance surface. (D14 made
the same call.)

## Accounting for the package's named slots

| Package slot | Measured | Basis |
|---|---|---|
| solver math (`cascade_run` = cfl + step + sample + residual) | 91.6 s ≈ 100 % of overhead | slots |
| CFL/celerity slot | 64.28 s (70 %) | slots |
| step sweeps | 23.75 s (26 %) | slots |
| hydrograph sampling (`solver_sample`) | 1.09 s (1.2 %) | slots |
| cascade residual (forcing dispatch, upstream bin-series integration — 16.7 M calls) | ~2.5 s (2.7 %) | slots (subtraction) + perf (seam closure 0.94 %) |
| source-series construction (`rate_series`) | 0.004 s | slots |
| friction operand build (runner side) | 0.013 s | slots |
| mesh/solver setup | 0.012 s | slots |
| runner row observation / publication integration | 0.013 s | slots |
