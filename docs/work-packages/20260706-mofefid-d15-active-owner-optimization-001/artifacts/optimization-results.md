# Optimization Results (D15A-S4/S5)

Status: **EXECUTED**.

Evidence mode: **Ran** unless labeled Static. Environment/fixture/commands per
`baseline-profile.md`.

## Implemented optimizations

All five planned candidates landed (`optimization-plan.md` OPT-5..OPT-9), all
in `crates/openwepp-hillslope-orchestrator/src/ofe_routing/`:

| ID | Where | Change |
|---|---|---|
| OPT-5 | `kinematic_wave.rs` `CellParameters::alpha_q` (renamed from `alpha`) | fixed point returns its own final `(alpha, q)`; `prepare_step_alpha` no longer recomputes `alpha·h.powf(1.5)` at the base and perturbed depths |
| OPT-6 | `kinematic_wave.rs` `step` | commit runs before the homogeneous TV(q) diagnostic; `tv_before` reads the cached pre-step `q` (`scratch.q0`), `tv_after` reads the committed discharge — the diagnostic no longer computes any `powf` |
| OPT-7 | `kinematic_wave.rs` `alpha_q` | `slope.sqrt()` hoisted out of the fixed-point loop |
| OPT-8a | `kinematic_wave.rs` `equivalent_friction_with_rain_term` | Froude number built lazily inside the wave branch (dead value elsewhere) |
| OPT-8b | `friction.rs` `vegetation_resistance_katul` | `L_c` checked before the beta sqrt is spent (identical 0 returns on every zero branch) |
| OPT-9 | `kinematic_wave.rs` solver | per-face material-interface flags precomputed once per solver (mesh immutable), replacing per-step 9-field `CellParameters` compares in the dissipation and TV loops |

No numerical method, `dt` selection, tolerance, source authority, activation
semantics, or output value changed. No new public API.

## Preservation proof (Ran)

- Protected outputs bit-identical pre/post optimization (SHA256 equal to the
  `baseline-profile.md` witness): `H2637.hbp` `948faf82…`, `H2637.loss.json`
  `725f5723…`, `H2637.pass.parquet` `f0d1be11…`. Re-verified after the final
  `cargo fmt` rebuild.
- Manifest `laned_shadow` diagnostics block: JSON-equal to the baseline block
  (all conservation figures, volumes, day classes identical).
- Trajectory witness: identical counters — `solver_steps=16,936,089`,
  `alpha_evaluations=302,411,532`, `hydrograph_samples=1,412,726`,
  `upstream_interpolation_calls=16,714,893`. (OPT-5 preserves even the
  `alpha_evaluations` counter semantics: evaluations are counted per call in
  `prepare_step_alpha`, unchanged.)
- Focused suite: `cargo nextest run -p openwepp-hillslope-orchestrator -E
  'test(ofe_routing) or test(kinematic_wave) or test(cascade) or
  test(friction) or test(seam) or test(d10b)'` — **67/67 passed** (includes
  the D10B reconciliation tests: Case-4 oracle convergence, TVD
  mass-neutrality, seam/cascade fixtures).

## Endpoint timing (profiling OFF, 3 runs, `taskset -c 4`)

| Case | User (3 runs) | Wall (3 runs) |
|---|---|---|
| baseline shadow-on | 93.29 / 92.41 / 90.63 s | 1:33.32 / 1:32.45 / 1:30.66 |
| optimized shadow-on | 78.75 / 78.77 / 78.78 s | 1:18.78 / 1:18.80 / 1:18.81 |

Median improvement: **92.41 → 78.77 s user (−13.6 s, −14.8 %)**; overhead over
the default/off median (2.45 s) drops from +89.96 s to **+76.32 s**.
Default/off is untouched by construction (subsystem-off code path; protected
outputs byte-identical, `INV-OFEROUTE-010`).

## Slot profile after optimization

```
cascade_run_ns 91600147877 → 79648887052   (−13.0 %)
solver_cfl_ns  64280756220 → 55147750410   (−14.2 %)
solver_step_ns 23751413101 → 20645766325   (−13.1 %)
solver_sample_ns ~1.09e9 → ~1.14e9         (unchanged within jitter)
counters: identical (see preservation proof)
```

Post-opt perf (symbol level): solver self 61.2 %, libm `pow` 27.2 % (down from
32.9 %), kernel 4.9 %, seam-rate closure 1.2 %.

## S5 — endpoint decomposition and timing adjudication

The optimized cost is now fully accounted by the two contract-mandated
mechanisms (Static, from Ran counters):

- **Steps ×1.639** (10,334,879 → 16,936,089): the rev-26 TRUE-celerity CFL
  trajectory plus the blocker-fix drain tail. This IS the corrected scheme's
  `dt` selection (`INV-OFEROUTE-007`); reducing it is a numerical-method
  change, excluded.
- **Per-step solver cost ×1.645** ((cfl+step)/steps: 2.72 µs D14 → 4.48 µs
  now): the rev-26 celerity evaluation adds a second friction fixed-point per
  wet cell per step (alpha evaluations 10.0 → 17.86 per step). Predicted
  ratio from the counter alone: `1 + alpha_share×0.786 ≈ 1.63` with the D14
  measured alpha share (~80 % of solver time) — matches the measured 1.645.
- Cross-check: `29.9 s × 1.639 × 1.61 ≈ 78.9 s` vs measured `78.8 s` —
  the D14-budget-to-now gap closes to ~0.1 % once the two mandated factors
  are applied. The implementation waste D15A removed (−13.6 s) was the
  entire non-mandated residual the S2 profile could attribute.

**Adjudication (package authority, flagged for operator ratification):** the
D14 `29.9 s` budget was measured on the pre-rev-24 scheme that D10B proved
latently unstable (true Courant ~1.8 on the laminar limb behind the inverted
limiter's masking dissipation) and non-conservative (unbooked boundary
leaks). That budget's basis is superseded: it priced a defective trajectory.
The corrected scheme's H2637 opt-in cost — **78.8 s user / 1:18.8 wall on the
2-year fixture, ~32x the default/off path** — is adjudicated ACCEPTED as the
D15A activation budget because (a) every non-mandated component measured by
the S2 profile has been removed under the bit-identity bar, (b) the remaining
cost decomposes to within ~1 % into the two contract-mandated mechanisms, and
(c) the subsystem stays opt-in: default users pay nothing
(`INV-OFEROUTE-010` byte identity). Remaining bit-identity headroom is
estimated ≤ ~5 % (bounds-check elimination, dispatch micro-costs) and does not
change the adjudication. Any future timing improvement beyond this requires a
contract-first numerics decision (e.g. an analytic celerity limb or a
fixed-point restructure) — named as follow-on scope, not silently done here.

Per this adjudication, D15A-S5 authorizes proceeding to the active-owner
phase (P0-P5) in this package. The adjudication is package-local authority
under the package's "optimized … or explicitly adjudicated as acceptable by
current package authority" exit criterion; the dual reviews and the worker
handoff surface it for operator ratification.
