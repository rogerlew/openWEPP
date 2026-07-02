# Codex Recheck — MOFEFID-D7

Date: 2026-07-02
Reviewer: Codex
Reviewed commit: `9db9232e` (`worktree-mofefid-d7`)
Disposition: **HOLD — one package-governance cleanup remains**

## Evidence

Static:
- Read updated `package.md`, `artifacts/execution-report.md`,
  `artifacts/review-disposition.md`, `artifacts/operands.md`, and
  `SC-OFEROUTE-001` rev 8.
- Inspected the delta from held commit `2eb33a7d` to `9db9232e`.
- Rechecked `ofe_routing::dval` and the D-val script/example surfaces.

Ran:
- `cargo nextest run -p openwepp-hillslope-orchestrator dval`
- `/home/workdir/openWEPP/.venv/bin/python tools/dval/compare_dval.py --case 1 --fig4 /home/workdir/openWEPP/references/copyrighted/Papanicolaou2018-supplemental/wrcr23071-sup-0002-2017wr021109-ds01/Figure_4.xlsx --crate-dir .`
- `/home/workdir/openWEPP/.venv/bin/python tools/dval/compare_dval.py --case 4 --ko 200 --fig4 /home/workdir/openWEPP/references/copyrighted/Papanicolaou2018-supplemental/wrcr23071-sup-0002-2017wr021109-ds01/Figure_4.xlsx --crate-dir .`
- `cargo run --example dval_case -p openwepp-hillslope-orchestrator -q -- 4 200`
- `cargo fmt --check`
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
- `git diff --check origin/main...HEAD`
- Shadow-only grep for D-val callers outside `dval.rs`, `dval_case.rs`, and
  `tools/dval/compare_dval.py`.

Not run: full workspace nextest / cargo deny.

## Prior Findings

### CX-D7-001 — Iwagaki No-Rain Forcing

Status: **Closed**

`run_iwagaki` now sets the skin-term rainfall intensity to zero while keeping
the lateral pulse as `rainfall_excess_m_s`. Independent harness rerun for
Case 4 at `k_o=200` produced `NS_trace = 0.3008818922609847`, peak ratio
`0.7887294323507097`, `t_peak = 28.0 s`, and 10-90% rise `20.628 s` versus
reference `20.883 s`. The prior solver-side shock-lag GAP is correctly
withdrawn as a forcing-bug artifact.

### CX-D7-002 — Case 1 Shape Gate

Status: **Closed**

Case 1 is downgraded from `REPRODUCES` to `PARTIAL`. Independent harness rerun
confirmed `NS_trace = 0.8684551801050971`, peak ratio `1.0660709546756058`,
and the failing 10-90% rise comparison (`4999.621 s` openWEPP vs
`3579.914 s` enhanced-WEPP).

### CX-D7-003 — S2 Unit-Convention Overclaim

Status: **Partially closed**

`SC-OFEROUTE-001` rev 8 and `execution-report.md` now correctly say the
skin-term `I`/`nu` convention remains unconfirmed and open. That fixes the
contract overclaim. The package body still needs cleanup, below.

## Remaining Finding

### CX-D7-R1 — `package.md` Still Contains Stale Active Scaffold Claims

Severity: Medium
Evidence: Static
Status: Accepted candidate; merge-blocking for package closure

The top of `package.md` has the corrected rev-8 summary, but the active package
body still carries stale scaffold language that contradicts the executed
disposition:

- The framing still says Iwagaki "does NOT currently reproduce" because
  openWEPP lags the shock timing by ~5 s and that this is a genuine
  shock-capture / celerity gap.
- `D7-S2` still says to pin the `I`/`nu` convention "here" and refers to it as
  "confirmed empirically by D-val", even though the corrected disposition is
  that D7 did not and cannot close that audit.
- The risks still frame the ~5 s shock lag as a possible solver-fidelity
  finding to feed back to D4, although the corrected execution withdrew that
  attribution as a forcing-bug artifact.

This does not undermine the corrected code or the contract rev-8 text, but it
does leave the package's active execution spec internally inconsistent. Under
the work-package gate non-deferral rule, D7 should not close while the package
body still declares a current-scope S2 gate and stale shock-lag objective that
the corrected artifacts explicitly reject.

Required disposition: update `package.md` so the scaffold body matches the
executed rev-8 outcome: Case 4 is operand-limited after corrected `I=0`
forcing, `GAP-OFEROUTE-004` is withdrawn, S2 is explicitly open/follow-on
rather than a completed/current D7 gate, and the old shock-lag risk language is
removed or marked superseded.

## Non-Blocking Notes

- The D-val module remains shadow/analysis-only by grep; no production caller
  was found outside the D-val module/example/script.
- `dval_case` still prints the solver's internal `time_to_peak_s` for Case 4
  (`36.98 s` at `k_o=200`) while the sampled hydrograph peak used by the
  harness is `28.0 s`. The execution report notes this internal/sampled
  discrepancy as residual multi-modality; I am not treating it as a D7 blocker
  because the acceptance verdict uses the sampled trace comparison.
- `compare_dval.py` still does not explicitly check the `cargo run` return
  code before parsing stdout. Failure is not silent, so this remains
  non-blocking.

## Recheck Disposition

Do not merge yet. The code-level and contract-level corrections are sound, and
the prior substantive blockers are closed, but `package.md` needs one cleanup
pass so the package itself no longer contradicts the corrected D7 outcome.
