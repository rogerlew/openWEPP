# Gate Results

Status: complete / pass

Evidence mode: Ran

All commands ran from `/home/workdir/openWEPP`. The terminal source snapshot was
HEAD `4a6948ddbcb652310f4ca063a6c57f9b206a3740` with worktree diff SHA-256
`bd07523f8e0f566c52a152ff4ef6d8dd2c2deadfae5ab760c88c7d6d4d4e4119`.
The later tracked changes are review and closure prose only.

| ID | Exact command | Result / duration | Evidence |
| --- | --- | --- | --- |
| 1 | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | PASS / `15.472 s` | `target/21k-gates/1_cargo_build_openwepp_cli_hill.log` |
| 2 | `.venv/bin/python docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/tools/run_materiality.py --execute` | PASS / `153.163 s` | Receipt/result identities below. |
| 3 | `cargo nextest run --test snow_wet_compaction_operand_authority --no-fail-fast` | PASS / `8/8`, `50.061 s` | `target/21k-gates/3_nextest_snow_wet_compaction_operand_authority.log` |
| 4 | `cargo nextest run -p openwepp-runner -E 'test(boundary_requires_finite_nonnegative_authoritative_source_columns) \| test(replay_uses_generated_melt_and_contact_rain_once)'` | PASS / `2/2`, `12.270 s` | `target/21k-gates/4_nextest_boundary_authoritative_source_columns.log` |
| 5 | `cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(helper_sums_positive_melt_and_contact_rain_and_fails_closed)'` | PASS / `1/1`, `99.944 s` | `target/21k-gates/5_nextest_helper_sums_positive.log` |
| 6a | `cargo nextest run --test snowdensity06b_coe_bound_density_replay` | PASS / `3/3`, `149.281 s` | `target/21k-gates/6a_nextest_snowdensity06b_coe_bound_density_replay.log` |
| 6b | `cargo nextest run --test snowdensity10_3_1a_per_day_cancov` | PASS / `3/3`, `291.622 s` | `target/21k-gates/6b_nextest_snowdensity10_3_1a_per_day_cancov.log` |
| 7 | `cargo fmt --all -- --check` | PASS / `2.978 s` | `target/21k-gates/7_cargo_fmt_check.log` |
| 8 | `cargo clippy --workspace --all-targets -- -D warnings` | PASS / `67.791 s` | `target/21k-gates/8_cargo_clippy_workspace.log` |
| 9r | `cargo nextest run --workspace --profile quick --no-fail-fast -j 2` | PASS / `2181/2181`, `5922.332 s`; `40` skipped | `target/21k-gates/9r_nextest_workspace_profile_quick_retry_j2.log` |
| 10 | `cargo nextest run --workspace --profile frost --no-fail-fast` | PASS / `358/358`, `613.240 s`; `1917` profile-skipped | `target/21k-gates/10_nextest_workspace_profile_frost.log` |
| 11 | `cargo nextest run --workspace --no-fail-fast -j 2` | PASS / `2270/2270`, `6418.711 s`; `5` skipped | `target/21k-gates/11_nextest_workspace_no_fail_fast_j2.log` |
| 12 | `cargo test --workspace --doc` | PASS / `10.435 s`; no doctests defined | `target/21k-gates/12_cargo_test_workspace_doc.log` |
| 13 | `cargo deny check` | PASS / `3.708 s`; existing unused `MIT-0` allowance warning only | `target/21k-gates/13_cargo_deny_check.log` |
| 14 | `target/release/openwepp-assurance validate --all` | PASS / `0.694 s` | `target/21k-gates/14_openwepp_assurance_validate_all.log` |
| 15 | `.venv/bin/python docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/tools/materialize_snowbird_development_cli.py --check` | PASS / `0.188 s` | `target/21k-gates/15_materialize_snowbird_check.log` |
| 16 | `bash tools/release/check_authority_suite_antievasion.sh` | PASS / `0.091 s` | `target/21k-gates/16_check_authority_suite_antievasion.log` |
| 17 | `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | PASS / `3/3`, `0.693 s` | `target/21k-gates/17_nextest_auth11_required_suite.log` |

The first unconstrained quick attempt is retained at
`target/21k-gates/9_nextest_workspace_profile_quick.log`. Host contention
caused two assurance timeouts and two SIGTERMs after `940.357 s`; it is not
counted as a pass or hidden. The exact same source passed the complete quick
suite under `-j 2`, followed by the complete default workspace under `-j 2`.

## Materiality and closure

- Receipt SHA-256:
  `1cd4aa5fb2110eb0445f57de846e2b65b224e7b0704e00a9d6cff1e3d4ca220a`.
- Result SHA-256:
  `25c8150f95d1be81afa7597d93dc271f8df5d82e062c558b231dd1695afab05a`.
- Release binary SHA-256:
  `1934000cd3c2534350af7ab1678325906762798e94dbe245b3895b910bf1382a`.
- Operand reconstruction: `8.353e-17 m` maximum against `1e-12 m`.
- Upstream mass delta: `2.443e-15 m` maximum against `1e-9 m`.
- Stage-3 incoming-liquid closure: `3e-17 m` maximum.
- Density-process closure: `2.274e-13 kg m^-3` maximum against
  `1e-9 kg m^-3`.
- Layer SWE/depth closure: `4.441e-16 m` / `8.882e-16 m` maximum.
- Materiality: `24,046` canonical driver-days and `22,392` density-days
  changed. Maximum density/depth deltas were `174.016 kg m^-3` and
  `0.367071 m`.

The corrected driver totals were `55.7%` to `61.9%` of the retired driver by
site. Stage-3 routing/store/refreeze disposition changed by at most
`0.002363 m`; this is an observed density-mediated response, not an upstream
mass-invariance failure.

The Snowbird development lane matched `39` water years, increased median peak
SWE by `0.122877 m` (`1.291661` ratio; range `1.216323` to `1.441480`), and
changed median peak timing by `0 d`. This is input-sensitivity evidence only.

Machine-readable command evidence is in `target/21k-gates/command-log.json`
(SHA-256 `123291a0e067186a6f8278e67bb83831a1c7a702fb540f26871cdb59775d2a9f`)
and `summary.json` (SHA-256
`1bf9e174dd777e811e3e1999d19355bd704c891aa2d650b4565aba76766ccb0e`).

## Documentation and terminal-diff closure

- `markdown-doc lint` and `markdown-doc validate` pass for the 28-file package
  tree and all seven changed catalog, roadmap, contract-index, and fixture
  documentation surfaces outside it.
- `git diff --check` passes for the tracked worktree, and the staged check
  excluding the generated development CLI passes. The all-staged check reports
  only the development CLI's line-5 terminal space, which is intentionally
  inherited byte-for-byte from canonical `p8.cli`; removing it would violate
  the non-precipitation custody rule. The base-to-worktree reconciliation
  remains `77` tracked paths plus nine untracked closure paths, or `86` total.
- The archived kickoff prompt retains SHA-256
  `a863c62df3b18bb82a7de9d5a38ecf4364d1cfbfb2ae591bbfb9480fa9f1f69e`.
- The `uk2us` preview was intentionally not applied. It proposed changing
  scientific command/field identifiers such as `coe-melt` as well as unrelated
  retained contract prose; applying those substitutions would corrupt exact
  technical names and expand the closure diff.
