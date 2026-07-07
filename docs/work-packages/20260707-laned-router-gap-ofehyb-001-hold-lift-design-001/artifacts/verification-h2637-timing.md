# H2637 Active Hybrid Timing Verification

- Scratch directory: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/h2637-scratch`
- Runner binary: `/home/workdir/openWEPP/target/release/openwepp-cli-hill`
  - `mtime`: `1783409194` (`2026-07-07 00:26:34.961399117 -0700`)
  - `size`: `9897672`
  - `sha256`: `628486b358b94bf87f09880c0e3b687a924b33502967e08fba5145b0a8e72f51`

- `cargo build --release -p openwepp-runner --bins`
  - command log: `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/build_openwepp_runner_release.log`
  - exit code: `0`
- `OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=1 OPENWEPP_LANED_SHADOW_PROFILE=1 /usr/bin/time -v taskset -c 4 target/release/openwepp-cli-hill --run-dir /home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/h2637-scratch --run-file p2637.run.toml --output-dir /home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/h2637-scratch/output`
  - command log: `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/h2637-active-hybrid-timing.log`
  - exit code: `0`
  - `user`: `37.96 s`
  - `system`: `0.02 s`
  - `wall`: `0:37.99`

- Profile fields (from `laned_active_profile` line in log):
  - `solver_steps`: `7381407`
  - `solver_steps_implicit`: `980804`
  - `implicit_equilibrium_map_evaluations`: `151435969`
  - `implicit_branch_evaluations`: `20110816`
  - `alpha_evaluations`: `119746485`

- Timing slots present:
  - `solver_cfl_ns`: `19698741108`
  - `solver_step_ns`: `7443634516`
  - `solver_sample_ns`: `387761935`

- Baseline comparison:
  - rev-31 active hybrid user `36.61 s` → delta `+1.35 s` (`+3.69%`)
  - plain-active user `37.9 s` → delta `+0.06 s` (`+0.16%`)
