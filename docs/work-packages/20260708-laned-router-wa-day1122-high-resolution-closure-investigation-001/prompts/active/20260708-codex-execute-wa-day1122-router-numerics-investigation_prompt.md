# Codex Kickoff: WA Day-1122 Router Numerics Investigation

Scope: local openWEPP work-package investigation; flat-file reads/edits in
this worktree plus local build/test/run commands. No external services are
required.

Execution mode: package-end-to-end.

## Required Reading

Read first:
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/package.md`
- `docs/work-packages/20260708-laned-router-wa-day1122-high-resolution-closure-investigation-001/artifacts/required-reading-map.md`

Then read the Tier-2 parent evidence named in the package.

## Task

Execute the package phases WA-A through WA-E. Diagnose the
`wa_cascades_forest_h1` day-1122 `dx2p5/dx1p25` active day cascade residual
failure, attribute the completed-rung `dx10/dx5` magnitude amplification, and
close with an evidence-backed classification and follow-on.

## Constraints

- Do not change production mesh policy.
- Keep the active production default fixed at `10 cells/OFE`.
- Do not relax `SC-OFEROUTE-001` closure tolerances.
- Do not tune route coefficients or source values to make the diagnostic rungs
  pass.
- Treat H2637 as synthetic stress only.
- No hybrid implicit-stepper work is in scope.
- If a production numerics fix is needed, stop at a hold unless the package
  explicitly amends scope and completes contract-first authority.

Subagent authorization: this prompt explicitly authorizes spawning/delegating
to review, verification, and diagnostic/comparator subagents. Expected outputs
are package-local review and verification artifacts. Write access is bounded to
package artifacts unless an implementation fix is explicitly assigned.

## Expected Output

Close the package by writing:
- `artifacts/day1122-reproduction.md`
- `artifacts/magnitude-attribution.md`
- `artifacts/numerics-adjudication.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- review and verification artifacts
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`
