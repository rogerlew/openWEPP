# H2637 timing/profile verification (rev-31 implicit warm-seed/profile-counter)

Evidence scope: Ran only in this task; no source edits.

## Ran commands

1) Build command
- Command: `cargo build --release -p openwepp-runner --bins`
- Run directory: `/home/workdir/openWEPP`
- Exit: `0`
- Log: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/build_openwepp_runner_release.log`

2) Scratch patch + run script
- Command block:
```bash
cd /home/workdir/openWEPP
SCRATCH=docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/h2637-scratch
cp -a tests/fixtures/laned_shadow_h2637/. "$SCRATCH"/
awk -v cnt_file="$SCRATCH/insertions.count" '...'
OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=1 OPENWEPP_LANED_SHADOW_PROFILE=1 \
  /usr/bin/time -v taskset -c 4 target/release/openwepp-cli-hill --run-dir "$SCRATCH" --run-file p2637.run.toml --output-dir "$SCRATCH/output" \
  > "$SCRATCH/run.stdout.log" 2> "$SCRATCH/run.stderr.log"
```
- Exit: `0`
- Patch log: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/h2637-scratch/patch.log`
- Scratch dir: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/h2637-scratch`
- Scratch run stdout: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/h2637-scratch/run.stdout.log`
- Scratch run stderr: `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/h2637-scratch/run.stderr.log`

## Fixture patch verification
- `p2637.man` patched exactly as `enable_native_cropland_routing_coefficients` logic:
  - First line set to `ow-lanuse-1`
  - `1 # Landuse - <Cropland>` replaced with `4 # Landuse - <NativeCropland>`
  - `routing_coefficients\n500.0 0.0 0.0 0.0 0.0` inserted after each matching plant line
- Insertions observed: `19` (`patch.log`)

## Binary fingerprint
- Path: `/home/workdir/openWEPP/target/release/openwepp-cli-hill`
- mtime: `2026-07-06 22:00:41.004321071 -0700`
- size: `9897264` bytes
- sha256: `6e7a1c56ef9b74b6f37a790c98be5f2bfc9119fa7fc40027d953c9e05ae7ae9e`

## Timed/profile metrics
- Command run: `OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=1 OPENWEPP_LANED_SHADOW_PROFILE=1 /usr/bin/time -v taskset -c 4 target/release/openwepp-cli-hill --run-dir /home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/h2637-scratch --run-file p2637.run.toml --output-dir /home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/h2637-scratch/output`
- Exit status: `0`
- `/usr/bin/time -v` times:
  - User: `36.61`
  - Sys: `0.02`
  - Wall: `0:36.65`

- `laned_active_profile` JSON line:
```json
{"solver_runs":27530,"solver_steps":5806728,"solver_steps_homogeneous":0,"solver_steps_source_free":0,"solver_steps_implicit":1146432,"implicit_equilibrium_map_evaluations":274681460,"implicit_branch_evaluations":37241376,"alpha_evaluations":88421812,"solver_cfl_ns":14859981755,"solver_step_ns":5473027693,"solver_sample_ns":284910529}
```

- Required fields:
  - `solver_steps`: `5806728`
  - `solver_steps_implicit`: `1146432`
  - `implicit_equilibrium_map_evaluations`: `274681460`
  - `implicit_branch_evaluations`: `37241376`
  - `alpha_evaluations`: `88421812`
  - `solver_cfl_ns`: `14859981755`
  - `solver_step_ns`: `5473027693`

## Output hashes
- `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/h2637-scratch/output/H2637.loss.json`
  - `725f57233fd60df097a824a2c20f26992a58b3a457594245a9ac91d2278f3cfb`
- `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/h2637-scratch/output/H2637.hbp`
  - `c47c352eb9e3381f05897596ea9667d255e886c3d9dfc4be28d7af9f5e7a17a1`
- `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/h2637-scratch/output/H2637.pass.parquet`
  - `e600a4b09c17d7180dec02f03cb7765936988e86c036e103a9a30df838b05564`
- `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/h2637-scratch/output/openwepp_hillslope_run_manifest.json`
  - `3cb3e98f48b69a1d7df5260b78c3c6fee042a3aa1b46190da49d08298263c898`

## Failure / blocker log
- No blocking failures in final build+run pass. Initial patch script attempts failed transiently due local script-pattern mistakes, then rerun completed successfully with required 19 insertions and `run_exit=0`.
