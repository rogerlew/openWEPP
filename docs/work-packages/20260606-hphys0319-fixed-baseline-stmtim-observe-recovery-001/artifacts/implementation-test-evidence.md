# Implementation and Test Evidence

Status: complete

Evidence mode: Ran

Ran:

- `.venv/bin/python -m py_compile docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/artifacts/hphys0319_fixed_stmtim_observe.py`
  - Result: passed; exit status was `0`.
- `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract hphys0319_recovery_script_is_scoped_and_records_required_tags -- --nocapture`
  - Result: passed; exit status was `0`.
- `.venv/bin/python docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001/artifacts/hphys0319_fixed_stmtim_observe.py`
  - Result: first run failed during temporary fixed-baseline build because the
    patch included `cupdate.inc` directly in `stmtim.for`; final run passed
    after passing `year` and `sdate` from `winter.for` as observe-only
    arguments. Final exit status was `0`.

Final successful command log:

| Step | Exit | Seconds |
|---|---:|---:|
| `baseline_worktree_prune_pre` | `0` | `0.021` |
| `baseline_worktree_add` | `0` | `190.469` |
| `baseline_instrumentation_diff` | `0` | `0.029` |
| `build_fixed_baseline_observe_hill` | `0` | `78.510` |
| `H1_fixed_baseline_stmtim_observe` | `0` | `2.040` |
| `H7_fixed_baseline_stmtim_observe` | `0` | `1.993` |
| `H39_fixed_baseline_stmtim_observe` | `0` | `2.014` |
| `cargo_build_release_openwepp_cli_hill` | `0` | `48.615` |
| `H1_openwepp_hphys0319_trace` | `0` | `22.304` |
| `H7_openwepp_hphys0319_trace` | `0` | `22.369` |
| `H39_openwepp_hphys0319_trace` | `0` | `22.359` |

Key paired result:

- Fixed baseline H1/H7/H39: `wntdur = 11`, adjusted `wnttim = 1`,
  active interval `1`, snow branch `1`, `hrsnow = 0.00074545 m`.
- OpenWEPP H1/H7/H39: `wntdur = 11`, `wnttim = 0`, active interval `0`,
  snow branch `0`, `snow.hourly.stmtim.hrsnow_m_0011 = 0`.
- Classification: `stmtim-active-interval-divergence-hold`.
