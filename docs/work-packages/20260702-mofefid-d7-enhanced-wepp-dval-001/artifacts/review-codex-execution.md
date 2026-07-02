# Codex Execution Review — MOFEFID-D7

Date: 2026-07-02
Reviewer: Codex
Reviewed commit: `2eb33a7d` (`worktree-mofefid-d7`)
Disposition: **HOLD — not merge-ready**

## Evidence

Static:
- Read D7 `package.md`, `artifacts/cut-point-map.md`,
  `artifacts/operands.md`, `artifacts/execution-report.md`, and prior
  scaffold review.
- Read `SC-OFEROUTE-001` rev 7 (`INV-OFEROUTE-002`, `INV-OFEROUTE-011`,
  `GAP-OFEROUTE-004`).
- Read local gitignored `references/copyrighted/Papanicolaou2018.md`
  §3.1.4 for Case 4 forcing. No copyrighted source was vendored.
- Inspected `ofe_routing::dval`, `examples/dval_case`, and
  `tools/dval/compare_dval.py`.

Ran:
- `cargo nextest run -p openwepp-hillslope-orchestrator dval`
- `/home/workdir/openWEPP/.venv/bin/python tools/dval/compare_dval.py --case 1 --fig4 /home/workdir/openWEPP/references/copyrighted/Papanicolaou2018-supplemental/wrcr23071-sup-0002-2017wr021109-ds01/Figure_4.xlsx --crate-dir .`
- `/home/workdir/openWEPP/.venv/bin/python tools/dval/compare_dval.py --case 4 --ko 200 --fig4 /home/workdir/openWEPP/references/copyrighted/Papanicolaou2018-supplemental/wrcr23071-sup-0002-2017wr021109-ds01/Figure_4.xlsx --crate-dir .`
- `cargo fmt --check`
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`
- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`
- `git diff --check origin/main...HEAD`
- Shadow-only grep for `ofe_routing::dval`, `dval_case`, `run_iwagaki`,
  and `compare_dval` outside the D-val module/example/script.

Passing checks: D-val tests passed (2/2), harness reproduced the reported Case 1
and Case 4 scalar outputs, fmt/clippy passed, both authority guards passed,
diff check passed, and the shadow-only grep found no production caller outside
the D-val surfaces.

## Findings

### CX-D7-001 — Iwagaki Uses Rainfall Intensity In A No-Rain Experiment

Severity: High
Evidence: Static
Status: Accepted candidate; merge-blocking

`run_iwagaki` correctly uses the three lateral inflow rates as
`rainfall_excess_m_s`, but it also supplies `0.108e-2` as
`rainfall_intensity_m_s` for the first 10 seconds
(`crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs:81` and
`:85`). The solver explicitly uses `rainfall_intensity_m_s` as the skin-term
`I`, distinct from rainfall excess
(`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:208`).
The local Papanicolaou source states that no rain was applied in Case 4; water
was supplied laterally for 10 seconds
(`references/copyrighted/Papanicolaou2018.md:205`).

This invalidates the current Case 4 attribution as written. The extra raindrop
skin-resistance operand can slow the hydrograph and affect peak/noise behavior,
so the claim that the ~5-6 s lag is solver-side and operand-independent
(`artifacts/execution-report.md:50-63`,
`SC-OFEROUTE-001.md:281`) is not proven from like-for-like Iwagaki forcing.

Required disposition: set Case 4 `rainfall_intensity_m_s` to zero while keeping
lateral inflow in `rainfall_excess_m_s`; rerun the Case 4 direct/cascade
comparisons and `k_o` scan; then update `execution-report.md`,
`GAP-OFEROUTE-004`, and the cited-scalar test if the robust facts change.

### CX-D7-002 — Case 1 Reproduction Does Not Prove The Required Shape Gate

Severity: High
Evidence: Ran
Status: Accepted candidate; merge-blocking

The package acceptance model requires each reproducing case to pass named
shape co-conditions in addition to `NS_trace`, peak, and timing
(`package.md:126-134`). The execution report marks Case 1 as reproducing and
says shape co-conditions were applied (`artifacts/execution-report.md:13` and
`:18`), but the only detail is "matching rise-to-steady shape"
(`artifacts/execution-report.md:22-24`) without named bands.

The independent harness output does not make this a free pass: Case 1 has
`NS_trace = 0.868455`, peak ratio 1.066, but the reported 10-90% rise time is
3579.9 s for enhanced-WEPP versus 4999.6 s for openWEPP, and the peak/plateau
time differs by roughly 3000 s. That may still be acceptable for a plateauing
field run, but D7 did not define the pre-run shape/timing bands or explain why
the 10-90% difference passes.

Required disposition: add the named Case 1 shape/timing bands and show the
computed shape metrics pass them, or downgrade Case 1 from `REPRODUCES` to a
more limited/partial verdict.

### CX-D7-003 — Declared S2 Unit-Convention Gate Was Deferred, Not Closed

Severity: High
Evidence: Static
Status: Accepted candidate; merge-blocking

D7-S2 requires the executor to pin the skin-term `I`/`nu` convention against a
fully known case and record it (`package.md:97-100`). The contract still says
the exact crossover and `nu`/`I` unit convention are confirmed empirically by
D-val Case 1/2 reproduction (`SC-OFEROUTE-001.md:151`). The execution report
instead says S2 was "Not independently audited" and "deferred"
(`artifacts/execution-report.md:70-75`).

That is a gate non-deferral problem: D7 cannot be complete while a declared
current-scope stage is deferred and the governing invariant still claims D-val
confirmation.

Required disposition: either execute and record the S2 unit audit, or amend
the package/contract so S2 is explicitly a follow-on and D7 does not claim the
current D-val execution confirmed `INV-OFEROUTE-002`.

## Non-Blocking Notes

- The copyright posture is acceptable: the Python harness verifies the workbook
  sha256 and emits derived scalar metrics, not source rows.
- `compare_dval.py` would be more review-friendly if it checked the `cargo run`
  return code explicitly before parsing stdout, but the current failure mode is
  a crash rather than silent scalar fabrication; I am not treating this as a
  blocker.
- I agree with the contract split between `NS_trace` and paper `Ef_obs`, and
  with leaving `INV-OFEROUTE-011` partial rather than closing it.

## Review Disposition

Do not merge `worktree-mofefid-d7` as written. The branch should be
redispositioned after the three accepted candidates above are fixed or
explicitly re-scoped with contract/package amendments.
