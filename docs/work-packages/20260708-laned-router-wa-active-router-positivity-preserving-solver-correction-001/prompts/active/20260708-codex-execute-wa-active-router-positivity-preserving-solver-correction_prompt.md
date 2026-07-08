# Execute WA Active-Router Positivity-Preserving Solver Correction

Execution mode: package-end-to-end.
Autonomy: execute all phases through disposition without additional user
intervention unless a hard blocker is proven.

Package:
`docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/`

Objective: replace the WA active-router `laned_active_clamp_exceeds_source`
fail-closed outcome with contract-authorized positivity-preserving solver
behavior, or close with an executed hold that proves why the remaining solver
correction is outside this package.

Core required reading:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- package-local `package.md`
- prior WA clamp hold-lift final disposition and worker handoff

Subagent authorization: this package explicitly authorizes
spawning/delegating to review, verification, comparator/timing, and
solver-localization subagents. Expected outputs are package-local review,
verification, timing/comparator, and localization artifacts. Write access is
bounded to package-local artifacts unless a subagent is explicitly assigned an
implementation fix.

Do not promote target-`dx`, relax rev-40's clamp-source guard, tune source
producers or routing coefficients, reintroduce hybrid stepping, or silently
fall back to shadow/DC01 routing.
