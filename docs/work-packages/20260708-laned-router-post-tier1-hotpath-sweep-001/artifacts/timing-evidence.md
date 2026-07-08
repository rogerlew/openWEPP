# Timing Evidence

Status: `COMPLETE`
Evidence mode: Static/Ran.

Post-Tier1 baseline from
`20260708-laned-router-tier1-local-numerics-001/artifacts/timing-evidence.md`:

- H2637 active endpoint median user time: `11.90 s`.
- `solver_runs=11590`.
- `solver_steps=10016170`.
- `alpha_evaluations=100161700`.
- `solver_cfl_ns=2488591327`.
- `solver_step_ns=6853399353`.
- `solver_sample_ns=613251801`.

This package will record current release timing only if the release runner and
fixture are available without editing fixtures or required-case bindings.

## Current Run

Ran:

- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
  completed in `1m 03s`.
- Binary:
  `target/release/openwepp-cli-hill`
  `e88c5552f6fa98fae4282eb87095fb271a8dd5c0cf30a97431a483c46a8694e7`,
  `9947696` bytes, mtime `2026-07-08 13:10:12.045330791 -0700`.
- Fixture: copied `/tmp/openwepp_tier1_h2637_active` to
  `/tmp/openwepp_post_tier1_hotpath_h2637_active`; no committed fixture or
  required-case binding edit.
- Command shape:
  `env -u OPENWEPP_LANED_ACTIVE ... /usr/bin/time -f ... taskset -c 4 target/release/openwepp-cli-hill --run-dir /tmp/openwepp_post_tier1_hotpath_h2637_active --run-file p2637.run.toml --output-dir /tmp/openwepp_post_tier1_hotpath_h2637_active/output`.

Endpoint timing, profiling off:

| Run | User | Sys | Wall | Max RSS |
|---:|---:|---:|---:|---:|
| 1 | `11.88 s` | `0.03 s` | `0:12.08` | `21504 KiB` |
| 2 | `11.63 s` | `0.01 s` | `0:11.65` | `21504 KiB` |
| 3 | `11.72 s` | `0.01 s` | `0:11.74` | `21504 KiB` |

Median user time: `11.72 s`, versus the Tier1 recorded median `11.90 s`.

Ran one profiled pass:

```
laned_active_profile {"solver_runs":11590,"solver_steps":10016170,"solver_steps_homogeneous":3149895,"solver_steps_source_free":5632212,"alpha_evaluations":100161700,"solver_cfl_ns":2277134095,"solver_step_ns":6801315523,"solver_sample_ns":604547009}
profile exit=0 user=13.52 sys=0.01 wall=0:13.54 maxrss=21888
```

Slot comparison to Tier1:

| Slot | Tier1 rev-47 | This package | Change |
|---|---:|---:|---:|
| `solver_cfl_ns` | `2488591327` | `2277134095` | `-8.5%` |
| `solver_step_ns` | `6853399353` | `6801315523` | `-0.8%` |
| `solver_sample_ns` | `613251801` | `604547009` | `-1.4%` |

Disposition: the sweep produces a small endpoint improvement and a directly
measured CFL-slot reduction. It is not a new 2x class optimization.
