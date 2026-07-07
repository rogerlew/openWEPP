# Review Lorentz

Status: GO-WITH-AMENDMENTS. Evidence mode: Static review + read-only shell
inspection.

Reviewer: `rust_code_reviewer` subagent Lorentz.

## Findings

### Medium: Hold Artifacts Not Closure-Ready

Accepted. The reviewer found the hold was legitimate but package artifacts were
still placeholders or `PENDING`. Fixed by adding/updating:

- `artifacts/hold-legitimacy-audit.md`
- `artifacts/promotion-readiness-audit.md`
- `artifacts/timing-and-fidelity.md`
- `artifacts/gate-results.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

### Low: Case-4 Local Log Was Incomplete During Review

Accepted. The focused Case-4 command was still running or the log had not yet
included the tail when the reviewer inspected it. The final log now includes:

- `Summary [ 144.949s] 1 test run: 1 passed (1 slow), 342 skipped`

### Low: Explicit `OPENWEPP_LANED_ACTIVE_IMPLICIT=0` Was Not Separately Run

Accepted. D16 did not implement the promoted-default path, so the explicit
opt-out selector proof is `NOT RUN`. The active plain run is correctly
described as implicit unset, matching current env-opt-in semantics.

## Verdict Disposition

All requested amendments are accepted. The hold remains legitimate because no
current contract or code surface authorizes default promotion over the observed
H2637 active plain-vs-hybrid publication deltas.

