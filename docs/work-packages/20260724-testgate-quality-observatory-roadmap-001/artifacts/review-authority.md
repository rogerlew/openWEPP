# Authority And Package-Boundary Review

Evidence class: `Static`

Reviewer role: independent authority/package-boundary subagent

## Findings

1. `CRITICAL`: ADR-0021, ADR-0039, the testing/gate strategy, package
   governance, and executable policy currently make coverage/CRAP blocking.
   A superseding ADR must preserve metrics/taxonomy as QA and CQR targets while
   removing their ordinary closure authority.
2. `CRITICAL`: gate definitions, planner, executor, schemas, fixtures, and
   contract tests mechanically require quality nodes. Typed
   `DEFERRED_TO_QUALITY_CI` must be independently reconstructed.
3. `HIGH`: the active proportionality and coverage-reconstruction packages
   retain conflicting CRAP acceptance and require explicit disposition.
4. `HIGH`: the current full-only collector under-measures snowbench after its
   tests moved to `science-manual`.
5. `HIGH`: CQR Nightly currently recollects global coverage and needs an exact
   QA evidence handoff.
6. `HIGH`: root/package instructions and quality/refactor standards require
   alignment, not only the ADR and workflow.
7. `MEDIUM`: release behavior must be explicit so it cannot silently restore an
   increment gate.

Initial recommendation: split authority, TESTGATE implementation, merged QA,
QA workflow, CQR handoff, and functional qualification.
