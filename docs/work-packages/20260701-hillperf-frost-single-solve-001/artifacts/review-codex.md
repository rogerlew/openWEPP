# Codex Review - WP-2 Frost Single-Solve

Evidence classes used below:
- Static: source/document/contract inspection only.
- Ran: command executed locally in `.claude/worktrees/hillperf-sub5x`.

Review scope: `d90da7e3..HEAD` on `worktree-hillperf-sub5x`. No production code or tests were modified.

## Findings

### Accepted Candidate - Missing runner-side test for stale no-final frost clear

Static: `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs:193` moves the stale coarse-layer clear contract out of R4A and states it is now covered by the runner authority channel `direct_production_same_day_frost_hydrology_layers` with `clear_no_final_hydrology_layers`.

Static: the production branch exists at `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs:214` and clears/rebalances at `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs:243`. The flag is threaded through `frost_day_context` at `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs:83` and passed from rainfall/snowmelt presence at `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:2042`.

Static: `rg -n "clear_no_final_hydrology_layers|direct_production_same_day_frost_hydrology_layers|no_final_hydrology|same_day_frost_hydrology_layers" crates/openwepp-runner/src crates/openwepp-runner/tests crates/openwepp-hillslope-orchestrator/src/tests` found only production references and the R7G comment; I found no runner-side unit, source-guard, or integration test that forces `clear_no_final_hydrology_layers = true` with a no-final-frozen frost outcome and stale frozen layer state.

Risk: this does not reverse the WP-2 rubric verdict or endpoint evidence, but it is the one requested relocation check that is not test-covered after the migration. Add a runner-side test that proves stale `frozen_depth_m`/`frozen_water_m` are cleared and aggregate storage is rebalanced when same-day liquid exists and the frost outcome has no final frozen projection.

### Deferred Candidate - Stale paired-trace comment

Static: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:39` still says both the runner builder and the R4A span emit WP-2 paired-solve trace rows. The R4A solve hook was deleted; current source has the writer call only in the runner builder at `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs:137`.

Risk: behavior is acceptable because `wp2_frost_pair_trace_path()` remains env-gated and inert when `OPENWEPP_WP2_FROST_PAIR_TRACE_PATH` is unset, but the production comment now misdescribes the live hook topology. This is a cleanup candidate, not a package blocker.

## Accepted / Rejected Checks

Static + Ran: I accept the rubric verdict. `SC-SNOWFREEZE-001` binds frost-tube observations as direct `frdp` magnitude authority, while soil-temperature `0 degC` isotherms are timing/duration plus magnitude upper-bound only (`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:2914`). The same contract requires every frost verdict to be gated by snow-depth control and keeps rows without paired observed snow depth inconclusive (`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:2931`, `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md:2964`). Therefore Mandan's isotherm exceedance increase is not verdict-bearing under `INV-SNOWFREEZE-047/048/050`. My rerun of the five-site harness reproduced `0` defect-eligible and `0` `OPENWEPP-DEFECTIVE` sites, with the same classification counts as the checked-in `rubric-after` artifact.

Static: I accept ingress placement and closure shape. `run_day_spans` runs R4C, then `apply_r4w_winter_frost_ingress`, then R4I at `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs:713`. The ingress applies the carried outcome to `percolation_inputs.layers`, mirrors to `subsurface_compute_inputs.layers`, updates `water.soil_water_m`, and computes `frost_liquid_delta_m` from the pre-frost aggregate at `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:870`. The relocated material/no-material clear paths retain the 1e-9 storage asserts at `runoff.rs:890`, `runoff.rs:906`, and `runoff.rs:920`, plus the frozen-water bound at `runoff.rs:936`.

Static: I accept deletion completeness for second-solve semantics. Current direct runtime has no surviving `compute_r4a_winter_frost_partition`, `apply_r4a_winter_frost_outcome`, or `r4a_frost_layers_with_local_partition_excess` definitions/callers. The only current production `compute_direct_winter_frost_partition` call in the direct-production path is the runner authority call. R4A now keeps retained-local-liquid bookkeeping only at `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:847`, and R4X deferred local excess is preserved before saturation at `runoff.rs:310`.

Static: I byte-compared the retained helpers. `latest_r4a_frost_layers` is unchanged from `d90da7e3` after relocation, and `r4a_layers_have_coarse_frost_projection` is unchanged. `r4a_deferred_local_partition_excess_m` is semantically unchanged; my first diff range included one extra closing brace in current output, and after accounting for the range mismatch the body matches.

Static + Ran: I accept the migrated R7G/R7H tests except for the missing runner-side stale-clear coverage finding above. The tests now assert start-of-day frost basis (`direct_runtime_r7g_frost.rs:433`), explicit R4A non-mutation of the layer basis (`direct_runtime_r7g_frost.rs:944`), and kernel-owned warm-thaw outcome behavior (`direct_runtime_r7g_frost.rs:242`). I agree with not asserting `net = thaw - debit` in the frame-level test because the storage phase consumes the explicit day-input `frost_storage_liquid_delta_m` channel and the frwatc ledger includes the ingress handoff term.

Ran: H2637 endpoint independently completed with exit 0 in `33.08 s / 77500 KiB`. The manifest reports source commit `ddc7a5eb3602c25c5dba7750d7342a3a6cf631a3`, direct-production executor selected, WB13 identity statuses pass, hydout-equivalent closure within tolerance, and `direct_runtime_counters.compatibility_edge_invocations = 0`.

Static: I accept the counter choice. The ingress consumes a precomputed outcome and mutates/publishes day-frame state; recording it via `record_dynamic_transfer_publication()` at `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs:714` is coarse but not misleading for the current counter model. A dedicated counter would be nicer telemetry, not a correctness issue.

Static + Ran: line-count governance is reasonable. `runoff.rs` shrank by 472 net lines in the WP-2 range, and the only 3000+ touched file remains the pre-existing runner builder (`4141 -> 4143`) with a follow-up disposition recorded in `artifacts/line-count-governance.md`.

Static: backlog/ROADMAP updates are still pending exit actions per `package.md:66`; I did not treat their absence in `d90da7e3..HEAD` as an implementation defect because the package status is still review/merge pending.

## Commands Run

Ran:
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site site1_sleepers_south_field_vt --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/codex-wp2-rubric/site1_sleepers_south_field_vt --binary target/release/openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site site2_sleepers_w9_hardwood_vt --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/codex-wp2-rubric/site2_sleepers_w9_hardwood_vt --binary target/release/openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site site3_scan_mandan_nd --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/codex-wp2-rubric/site3_scan_mandan_nd --binary target/release/openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site site4_ggd498_morris_mn --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/codex-wp2-rubric/site4_ggd498_morris_mn --binary target/release/openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site site5_reynolds_creek_us_rls_id --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/codex-wp2-rubric/site5_reynolds_creek_us_rls_id --binary target/release/openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/classify_residuals.py --observations-dir tests/fixtures/snowfreeze_observed/observations --output-json target/codex-wp2-rubric/residual_classification.json --output-md target/codex-wp2-rubric/residual_classification.md target/codex-wp2-rubric/site1_sleepers_south_field_vt/comparison_report.json target/codex-wp2-rubric/site2_sleepers_w9_hardwood_vt/comparison_report.json target/codex-wp2-rubric/site3_scan_mandan_nd/comparison_report.json target/codex-wp2-rubric/site4_ggd498_morris_mn/comparison_report.json target/codex-wp2-rubric/site5_reynolds_creek_us_rls_id/comparison_report.json`
- `diff -u docs/work-packages/20260701-hillperf-frost-single-solve-001/artifacts/rubric-after/residual_classification.md target/codex-wp2-rubric/residual_classification.md`
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime_r7g_frost`
- `cargo test -p openwepp-runner direct_publication_source_guards` (matched 0 tests; reran exact guard below)
- `cargo test -p openwepp-runner r7g_direct_production_reads_winter_column_frost_and_deletes_bridge`
- `/usr/bin/time -f '%e s\t%M KiB' target/release/openwepp-cli-hill --run-dir /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/runs --run-file p2637.run --output-dir /tmp/codex-wp2-h2637-out.BESomm --policy compat --legacy-sidecar-discovery` (failed as expected because `p2637.run` is legacy stdin, not TOML)
- `/usr/bin/time -f '%e s\t%M KiB' target/release/openwepp-cli-hill --run-dir /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/runs --run-file <(sed 's|/tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/out|/tmp/codex-wp2-h2637-out.BESomm|g' /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/h2637.run) --output-dir /tmp/codex-wp2-h2637-out.BESomm --policy compat --legacy-sidecar-discovery` (failed because the CLI rejects `/dev/fd/*` runfiles)
- `/usr/bin/time -f '%e s\t%M KiB' target/release/openwepp-cli-hill --run-dir /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/runs --run-file /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/h2637.run --output-dir /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/out --policy compat --legacy-sidecar-discovery`
- `python -m json.tool /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/out/openwepp_hillslope_run_manifest.json`
- `sha256sum /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/out/H2637.hbp /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/out/H2637.loss.json /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/out/H2637.pass.parquet /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/out/H2637.wat.parquet /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/h2637/out/H2637.plot.parquet`
- `wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs`
- Static inspection commands included `git diff --stat d90da7e3..HEAD`, `git diff --name-only d90da7e3..HEAD`, `rg -n "apply_r4a_winter_frost_outcome|compute_r4a_winter_frost_partition|r4a_frost_layers_with_local_partition_excess|compute_direct_winter_frost_partition" ...`, `rg -n "clear_no_final_hydrology_layers|direct_production_same_day_frost_hydrology_layers|no_final_hydrology|same_day_frost_hydrology_layers" ...`, `diff -u <(git show d90da7e3:crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs | sed -n '951,973p') <(sed -n '1000,1022p' crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs)`, `diff -u <(git show d90da7e3:crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs | sed -n '1315,1332p') <(sed -n '1059,1076p' crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs)`, and `diff -u <(git show d90da7e3:crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs | sed -n '1023,1045p') <(sed -n '976,998p' crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs)`, plus read-only `sed`/`nl -ba` line-reference inspections of the files cited above.

Selected command results:
- Rubric rerun: `0` defect-attribution eligible, `0` `OPENWEPP-DEFECTIVE`, primary classifications `{'INCONCLUSIVE': 2, 'SNOW-CONTROL-FAILED': 3}`.
- Focused orchestrator frost tests: `16 passed`.
- Exact runner source guard: `1 passed`.
- H2637 endpoint: `33.08 s`, `77500 KiB`, `compatibility_edge_invocations = 0`.
- Output hashes from my H2637 rerun: `H2637.hbp` `8c5d1383f6284af0e14c47798c1e977c8679f058c4c5b65b72038ecbed6860c2`; `H2637.loss.json` `32977b750cf399c98687910b1ff612d5d11c7b1688c77b7eaeb83fbc99559549`; `H2637.pass.parquet` `2da88a3fffe133c06a58fd1bfbf5fc857b7755d4c1c2978be577c8cb0c29d850`; `H2637.wat.parquet` `a484d008b7996d500e86e4626c18de9f64406d71ba49dc94dc1f776ed7270829`; `H2637.plot.parquet` `19dc44f2e8ae462037cf468413253ce0b1e5a4ecf08da441a3daf2c7dfb04142`.
