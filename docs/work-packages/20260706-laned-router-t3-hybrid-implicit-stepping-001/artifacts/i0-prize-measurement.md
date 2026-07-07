# T3-I0 — Prize measurement (H2637 active path)

Status: **EXECUTED**. Evidence mode: **Ran** (release, `taskset -c 4`,
`OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_SHADOW_PROFILE=1`, the D15A
native-patched H2637 fixture; diagnostics counter added to
`ofe_routing::profile` — default-off, no output/manifest change).

```
laned_active_profile solver_runs=11590 solver_steps=10479200
solver_steps_homogeneous=3139424 solver_steps_source_free=5817107
alpha_evaluations=173774272
solver_cfl_ns=24.5e9 solver_step_ns=10.4e9 solver_sample_ns=0.6e9
```

Endpoint wall ≈ 39.6 s with profiling on (≈ 37.7 s off); solver slots ≈ 35 s.

| Eligibility rule | Steps covered | Share | Endpoint projection* |
|---|---:|---:|---|
| STRICT (zero source on every cell AND zero upstream mass — the i0 design's ratifiable default) | 3,139,424 | **30.0 %** | ~29 s (≈1.35x) |
| AGGRESSIVE (zero source; smooth upstream recession inflow allowed) | 5,817,107 | **55.5 %** | ~20.5 s (≈1.9x) |

*Projection assumes implicit `Δt = 900 s` versus the CFL-bound explicit steps
on covered phases (10-50x step reduction ⇒ covered-phase cost ≈ eliminated),
uniform per-step cost, non-solver time unchanged. Composing with Tier-1
(~2.5-4x on the remaining explicit work) projects the full stack at ~12 s
(~3.3x) on this fixture before Tier-2 resolution.

Notes for the I1/I2 design:

- The strict-vs-aggressive gap (25.5 % of steps) is entirely UPSTREAM-FED
  recession — downstream lanes draining while their upstream neighbor's tail
  still discharges. Case 4's post-cutoff phase contains exactly this
  configuration on its downstream reaches, so the I2 oracle acceptance run
  adjudicates the aggressive rule directly; the design ratifies STRICT first
  and holds AGGRESSIVE as an evidence-gated extension (the implicit solve
  handles nonzero `q_in` unchanged — only the switching predicate differs).
- Active-path steps (10.48M) are already ~38 % fewer than the shadow's
  16.9M (rainfall-only supplies); the shadow is not the T3 measurement
  surface.
- Counters land in `ofe_routing::profile` (`solver_steps_homogeneous`,
  `solver_steps_source_free`) and the runner emits a `laned_active_profile`
  stderr line under the existing profile env — same diagnostics-only posture
  as D14.
