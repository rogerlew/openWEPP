# Gate Results

Status: EXECUTED-HOLD-CASE4-HYBRID-LADDER

| Gate | Status | Evidence |
|------|--------|----------|
| `git diff --check` | PASS | Fresh post-refactor closure run exit `0`; no whitespace or merge-marker issues. Artifact: `verification-closure-gates-postfix.md`; log: `closure-gates-postfix/01-git-diff-check.log`. |
| Markdown/doc lint for touched docs | PASS | Final `markdown-doc lint --path ...` over the new package, Tier-1/Tier-2 scaffolds, touched parent artifacts, catalog, and `SC-OFEROUTE-001`: 27 files validated, 0 errors, 0 warnings. Artifact: `verification-closure-gates-postfix.md`; log: `closure-gates-postfix/10-markdown-doc-lint-final.log`. |
| Contract/profile/BEI checks for `SC-OFEROUTE-001` | PASS | BEI lint exit `0` with `PASS-DEFERRED` posture already present in the contract (`7` rows, `6` science-review-follow-on rows); SC unit compliance lint exit `0`, no findings. Logs: `closure-gates-postfix/08-sc-binding-exposure.log`, `closure-gates-postfix/09-sc-unit-compliance.log`. |
| Focused `ofe_routing` / Lane-D active tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator 'ofe_routing::implicit_recession::tests::branch_warm_seed' --profile quick`: 2/2 passed. `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing --profile quick`: 85/85 passed after the failed ratification vector was marked ignored. |
| Case-4 hybrid oracle ladder | FAIL | Explicit ignored-only ratification run failed as expected: `cargo nextest run -p openwepp-hillslope-orchestrator 'ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle' --profile quick --run-ignored ignored-only`; 1 failed in `150.896 s`, peak errors `22.8% / 15.5% / 10.2%` vs `5%` tolerance. Log: `case4-hybrid-ignored-ratification.log`. |
| Fidelity-tolerance adjudication | BLOCKED | Blocked by Case-4 hybrid ladder failure; no selector promotion or H2637 fidelity ratification claim is made. |
| H2637 active hybrid timing | PASS | Delegated `comparator_suite_runner` release run exit `0`; `36.61 s` user / `0:36.65` wall; profile counters include `solver_steps_implicit=1146432`, `implicit_equilibrium_map_evaluations=274681460`, `implicit_branch_evaluations=37241376`. Artifact: `verification-h2637-timing.md`. |
| Protected-output byte identity with subsystem off | NOT RUN | Not applicable for this write set: no default/off publication surface, default flag, or protected-output path was changed. The runner edit only extends active profile stderr JSON when profiling is enabled. |
| Implicit solve-cost counter evidence | PASS | Unit/profile vectors pass; H2637 active hybrid profile reports `274681460` map evaluations and `37241376` branch evaluations. Artifact: `implementation.md`; timing artifact: `verification-h2637-timing.md`. |
| `cargo fmt --check` | PASS | Fresh post-refactor closure run exit `0`; log: `closure-gates-postfix/02-cargo-fmt-check.log`. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Fresh post-refactor closure run exit `0`; the earlier `too_many_lines` failure was fixed by splitting the Case-4 hybrid harness helpers. Log: `closure-gates-postfix/03-cargo-clippy.log`. |
| `cargo nextest run --workspace --profile full` | PASS | Fresh post-refactor closure run exit `0`: 1428 tests run, 1428 passed, 5 skipped, 4 slow. Log: `closure-gates-postfix/04-cargo-nextest-full.log`. |
| `cargo deny check` | PASS | Fresh post-refactor closure run exit `0`; advisories, bans, licenses, and sources checks passed. Log: `closure-gates-postfix/05-cargo-deny-check.log`. |
| `.rs` line-count governance | PASS | Touched Rust files checked with `wc -l`; 6 files, `warn_count=0`, `block_count=0`, max touched file `kinematic_wave.rs` at 1859 lines. Log: `closure-gates-postfix/06-line-counts.log`. |
| Authority anti-evasion guard | NOT RUN | Not required: this package did not touch required-case bindings, cohort fixtures, or external-authority suite posture. |
