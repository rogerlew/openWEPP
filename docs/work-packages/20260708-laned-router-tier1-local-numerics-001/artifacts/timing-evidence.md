# Timing Evidence

Status: `EXECUTED`

Baseline: D15A active endpoint timing recorded `37.50 / 37.48 / 37.44 s`
user on the same H2637 active fixture class. A detached pre-change worktree at
commit `46532c28` also ran the current H2637 active fixture once in `40.14 s`
user; the D15A median remains the package's predeclared timing baseline.

Rev-47 endpoint timing, profiling off, release, `taskset -c 4`:

| Run | User | Sys | Wall |
|---:|---:|---:|---:|
| 1 | `11.96 s` | `0.03 s` | `0:12.03` |
| 2 | `11.85 s` | `0.02 s` | `0:11.88` |
| 3 | `11.90 s` | `0.02 s` | `0:11.93` |

Median user speedup vs D15A active median: `37.48 / 11.90 = 3.15x`.

Slot profile:

```
solver_runs=11590
solver_steps=10016170
solver_steps_homogeneous=3149895
solver_steps_source_free=5632212
alpha_evaluations=100161700
solver_cfl_ns=2488591327
solver_step_ns=6853399353
solver_sample_ns=613251801
```

`perf stat -d`: `11.946992679 s` elapsed, `11.923858 s` user,
`38,903,901,631` cycles, `83,641,343,007` instructions, IPC `2.15`.

Disposition: the backlog `2.5-4x` Tier 1 target is met for the active H2637
endpoint. Remaining performance hold is only the unimplemented
`Re^0.45` approximation envelope.
