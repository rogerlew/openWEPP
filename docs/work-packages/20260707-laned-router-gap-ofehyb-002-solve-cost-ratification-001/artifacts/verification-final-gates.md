# GAP-OFEHYB-002 Final Gates

Freshness note: this subagent batch ran before the final review-fix regression
tests were added. The package closure authority for affected Rust/doc gates is
the final-tree rerun in `artifacts/verification-final-tree-rerun.md`.

Date: 2026-07-07T08:20:16.564617Z

Package: GAP-OFEHYB-002

## Command Log
| # | Command | PASS/FAIL | Exit Code | Log |
|---|---|---|---|---|
| 1 | `git status --short --branch` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/01-command.log` |
| 2 | `git diff --check` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/02-command.log` |
| 3 | `cargo fmt --check` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/03-command.log` |
| 4 | `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md --path docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/04-command.log` |
| 5 | `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/05-command.log` |
| 6 | `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/06-command.log` |
| 7 | `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/07-command.log` |
| 8 | `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/08-command.log` |
| 9 | `cargo test -p openwepp-hillslope-orchestrator ofe_routing -- --nocapture` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/09-command.log` |
| 10 | `cargo clippy --workspace --all-targets -- -D warnings` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/10-command.log` |
| 11 | `cargo nextest run --workspace --profile full` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/11-command.log` |
| 12 | `cargo deny check` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/12-command.log` |
| 13 | `bash tools/release/check_authority_suite_antievasion.sh` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/13-command.log` |
| 14 | `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | PASS | 0 | `/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/14-command.log` |

## Compact tails
### 1. `git status --short --branch`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/01-command.log
```
start: 2026-07-07T08:07:19.409627Z
## main...origin/main [ahead 1]
 M crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs
 M crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs
 M crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs
 M docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md
 M docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md
 M docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/baseline-profile.md
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/binary-after-effective-provenance.txt
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/binary-after-provenance.txt
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-active-hybrid-after-effective-time.log
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-active-hybrid-after-time.log
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-active-hybrid-time.log
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-active-hybrid.log
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch-after-effective/
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch/
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/runner-build-after-effective.log
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/runner-build-after.log
?? docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/runner-build.log
```

### 2. `git diff --check`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/02-command.log
```
Running command 2
git diff --check
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:07:19.566940Z
```

### 3. `cargo fmt --check`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/03-command.log
```
Running command 3
cargo fmt --check
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:07:19.594355Z
```

### 4. `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md --path docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/04-command.log
```
Running command 4
markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md --path docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:07:21.377211Z
✅ 15 files validated, 0 errors, 0 warnings
```

### 5. `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/05-command.log
```
Running command 5
python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:07:21.393313Z
PASS-DEFERRED docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md: 7 binding exposure row(s), 6 science-review-follow-on row(s) not yet consolidated
```

### 6. `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/06-command.log
```
Running command 6
python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:07:21.441257Z
PASS-DEFERRED docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md: 4 binding exposure row(s), 4 science-review-follow-on row(s) not yet consolidated
```

### 7. `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/07-command.log
```
Running command 7
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:07:21.487350Z
PASS: SC unit compliance lint found no findings
```

### 8. `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/08-command.log
```
Running command 8
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:07:21.586751Z
PASS: SC unit compliance lint found no findings
```

### 9. `cargo test -p openwepp-hillslope-orchestrator ofe_routing -- --nocapture`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/09-command.log
```
test ofe_routing::dval::tests::case1_bare_reproduces_steady_magnitude ... ok
test ofe_routing::dval::tests::case4_iwagaki_ko_diagnostic_conserves_and_stays_positive ... ok
test ofe_routing::d10b_reconciliation_tests::case4_manning_tvd_dissipation_is_mass_neutral_and_tv_transient_bounded ... ok
test ofe_routing::d10b_reconciliation_tests::case4_solver_and_oracle_source_histories_agree_exactly ... ok
test ofe_routing::dval::tests::case1_rising_limb_lag_is_green_ampt_operand_limited ... ok
test ofe_routing::kinematic_wave::tests::conservation_residual_converges_with_resolution ... ok
test ofe_routing::d10b_reconciliation_tests::nineteen_ofe_conservation_is_resolution_convergent ... ok
test ofe_routing::iwagaki_oracle::tests::upwind_case4_conserves_and_peaks_after_cutoff ... ok
test ofe_routing::iwagaki_oracle::tests::upwind_single_reach_matches_closed_form ... ok
test ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle has been running for over 60 seconds
test ofe_routing::d10b_reconciliation_tests::case4_manning_solver_converges_to_iwagaki_oracle has been running for over 60 seconds
test ofe_routing::iwagaki_oracle::tests::characteristics_fan_cross_validates_upwind_reference has been running for over 60 seconds
test ofe_routing::iwagaki_oracle::tests::upwind_case4_self_convergence has been running for over 60 seconds
test ofe_routing::iwagaki_oracle::tests::characteristics_fan_cross_validates_upwind_reference ... ok
test ofe_routing::iwagaki_oracle::tests::upwind_case4_self_convergence ... ok
test ofe_routing::d10b_reconciliation_tests::case4_manning_solver_converges_to_iwagaki_oracle ... ok
test ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle ... ok

test result: ok. 92 passed; 0 failed; 1 ignored; 0 measured; 247 filtered out; finished in 154.28s

```

### 10. `cargo clippy --workspace --all-targets -- -D warnings`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/10-command.log
```
Running command 10
cargo clippy --workspace --all-targets -- -D warnings
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:09:56.121847Z
    Checking openwepp-hillslope-orchestrator v0.1.0 (/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator)
    Checking openwepp-runner v0.1.0 (/home/workdir/openWEPP/crates/openwepp-runner)
    Checking openwepp v0.1.0 (/home/workdir/openWEPP)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.95s
```

### 11. `cargo nextest run --workspace --profile full`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/11-command.log
```
Running command 11
cargo nextest run --workspace --profile full
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:10:06.386047Z
   Compiling openwepp-hillslope-orchestrator v0.1.0 (/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator)
   Compiling openwepp-runner v0.1.0 (/home/workdir/openWEPP/crates/openwepp-runner)
   Compiling openwepp v0.1.0 (/home/workdir/openWEPP)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 8.23s
────────────
 Nextest run ID ad8fd2b3-5ee2-47c3-be1e-4bb42bdd84be with nextest profile: full
    Starting 1435 tests across 163 binaries (4 tests skipped)
        SLOW [> 90.000s] (─────────) openwepp-hillslope-orchestrator ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle
        SLOW [> 90.000s] (─────────) openwepp-hillslope-orchestrator ofe_routing::d10b_reconciliation_tests::case4_manning_solver_converges_to_iwagaki_oracle
        SLOW [> 90.000s] (─────────) openwepp-hillslope-orchestrator ofe_routing::iwagaki_oracle::tests::characteristics_fan_cross_validates_upwind_reference
        SLOW [> 90.000s] (─────────) openwepp-hillslope-orchestrator ofe_routing::iwagaki_oracle::tests::upwind_case4_self_convergence
        SLOW [> 90.000s] (─────────) openwepp::snowdensity05e_melt_adjudication coe_melt_snowbench_runs_both_models_as_diagnostic_only
────────────
     Summary [ 596.529s] 1435 tests run: 1435 passed (5 slow), 4 skipped
```

### 12. `cargo deny check`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/12-command.log
```
Running command 12
cargo deny check
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:20:12.084212Z
advisories ok, bans ok, licenses ok, sources ok
```

### 13. `bash tools/release/check_authority_suite_antievasion.sh`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/13-command.log
```
Running command 13
bash tools/release/check_authority_suite_antievasion.sh
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:20:12.850911Z
PASS: authority suite anti-evasion checks passed.
```

### 14. `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
- status=PASS exit=0
- log=/home/workdir/openWEPP/docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-gates/14-command.log
```
cwd: /home/workdir/openWEPP
start: 2026-07-07T08:20:12.920011Z
   Compiling openwepp-hillslope-orchestrator v0.1.0 (/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator)
warning: unused import: `GRAVITY_M_S2`
  --> crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:19:23
   |
19 | use super::friction::{GRAVITY_M_S2, KINEMATIC_VISCOSITY_M2_S, SKIN_REGIME_REYNOLDS_THRESHOLD};
   |                       ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

   Compiling openwepp-runner v0.1.0 (/home/workdir/openWEPP/crates/openwepp-runner)
warning: `openwepp-hillslope-orchestrator` (lib) generated 1 warning (run `cargo fix --lib -p openwepp-hillslope-orchestrator` to apply 1 suggestion)
   Compiling openwepp v0.1.0 (/home/workdir/openWEPP)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.16s
────────────
 Nextest run ID 223de251-8b8d-4006-acd8-e5ef1df91bb9 with nextest profile: default
    Starting 2 tests across 1 binary
────────────
     Summary [   0.009s] 2 tests run: 2 passed, 0 skipped
```
