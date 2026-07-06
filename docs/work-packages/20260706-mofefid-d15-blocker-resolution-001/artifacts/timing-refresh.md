# Timing Refresh

Status: **EXECUTED-HOLD-BUDGET**.

Evidence mode: Ran.

## Results

Ran release timing against temporary native-patched H2637 fixture copies. Each
copy had `19` `routing_coefficients` extensions inserted, matching the
`laned_shadow_h2637` native test setup. Logs are package-local under
`artifacts/logs/`.

| Case | Result | User | Sys | Wall | Notes |
|---|---:|---:|---:|---:|---|
| default/off | PASS | `2.49 s` | `0.01 s` | `0:02.51` | stable versus D15 rerun default/off `2.58 s` user / `0:02.60` wall |
| `OPENWEPP_LANED_SHADOW=1` | PASS | `91.59 s` | `0.07 s` | `1:31.67` | terminal-bin blocker lifted; endpoint completed |
| `OPENWEPP_LANED_SHADOW=1 OPENWEPP_LANED_SHADOW_PROFILE=1` | PASS | `94.87 s` | `0.09 s` | `1:34.99` | emitted slot profile |

Profile evidence:

```text
solver_runs=11818
solver_steps=16936089
alpha_evaluations=302411532
hydrograph_samples=1412726
upstream_interpolation_calls=16714893
cascade_run_ns=92360232255
solver_cfl_ns=64740711158
solver_step_ns=24024689085
solver_sample_ns=1081231969
```

Comparison to D14:

- Prior D14 optimized shadow endpoint: about `29.9 s` wall/user.
- Current shadow endpoint: `91.59 s` user / `91.67 s` wall.
- Current overhead over default/off: `+89.10 s` user / `+89.16 s` wall.
- Current shadow wall is about `3.06x` the D14 optimized shadow wall and about
  `36.5x` the current default/off wall.
- Solver steps increased from the D14 witness `10,334,879` to `16,936,089`
  (`+6,601,210`, about `+64%`).

Decision: terminal timing now completes, but the D15 activation rerun should
not proceed to production flip. The timing regression is adjudicated as a hold
for the next D15 active-owner/optimization package. The drift is materially
larger than the small default/off drift the operator already accepted.
