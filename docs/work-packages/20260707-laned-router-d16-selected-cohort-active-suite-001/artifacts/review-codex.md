# Review

Status: COMPLETE. Evidence mode: Static + Ran.

## Findings

### R1: Superseded subagent hybrid command did not enable hybrid

Severity: High. Status: CLOSED.

The initial delegated command log labeled an H2637 run as hybrid but omitted
`OPENWEPP_LANED_ACTIVE_IMPLICIT=1`; the resulting manifest had
`hybrid_implicit_stepping = false`. The package now uses
`artifacts/run_active_suite.py`, which constructs the plain/hybrid environment
programmatically and records `hybrid_implicit_flag_ok` for each completed run.
The superseded subagent command summary was removed from final evidence.

### R2: Do not close selected-cohort evidence from H2637-only results

Severity: High. Status: CLOSED.

The corrected H2637 pair is valid timing/fidelity evidence, but it does not
resolve the selected-cohort suite hold. The final disposition is
`EXECUTED-HOLD-ACTIVE-RUN` because `mn_corn_h4` active plain fails before any
external plain-vs-hybrid comparison.

### R3: Do not patch `canhgt` with a package-local fallback

Severity: High. Status: CLOSED.

The active Rev-21 friction guard is fail-closed by design. Adding a package
local positive canopy height without source authority would be surrogate
physics. The hold audit names the first follow-on as an authority-backed
row-crop `canhgt` runtime publication/source-lift.

## Verdict

GO-WITH-HOLD. The package truthfully executes to the first active-run blocker,
preserves valid H2637 timing/fidelity evidence, and does not overclaim D16
suite resolution or selector readiness.
