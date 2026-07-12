# High-A Gate Results

Evidence class: **Ran**

Source commit: `fdf16c9d0b70996e9811acf7879fdfe1fda8a6d8`.
The required delegated gate runner executed every command exactly once and made
no repository edit.

| Gate | Exit | Elapsed | Max RSS | Result |
| --- | ---: | ---: | ---: | --- |
| `cargo fmt --check` | 0 | 2.04 s | 68,736 KB | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | 9.69 s | 408,192 KB | PASS |
| `cargo nextest run --workspace --profile full` | 0 | 574.00 s wall | 710,352 KB | PASS: 1,831/1,831; three skipped; four slow |
| `cargo deny check` | 0 | 1.11 s | 77,420 KB | PASS |
| Exact campaign `markdown-doc lint` | 0 | 0.02 s | 9,600 KB | PASS: initial 23 files; zero errors/warnings |

The four slow full-profile tests were
`case4_manning_solver_converges_to_iwagaki_oracle`,
`characteristics_fan_cross_validates_upwind_reference`,
`upwind_case4_self_convergence`, and
`snowdensity05e_melt_adjudication::coe_melt_snowbench_runs_both_models_as_diagnostic_only`.
No test failed and no failure attribution or rerun was required.

Logs and timing reports are
`/tmp/openwepp-ha-final-{fmt,clippy,nextest-full,deny,doclint}.{log,time}`.
The delegated runner captured their hashes and sizes. After authoring the
verification and transition artifacts, the exact documentation command was
rerun against the final transition worktree: exit `0`, 27 files, zero errors,
and zero warnings. It included the active High-A path once and scanned all
campaign evidence.

Disposition: `PASS`.
