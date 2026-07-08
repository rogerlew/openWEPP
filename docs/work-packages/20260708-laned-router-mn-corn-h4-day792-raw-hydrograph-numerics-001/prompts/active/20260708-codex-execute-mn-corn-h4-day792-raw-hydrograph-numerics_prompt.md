# Codex Execute Prompt

Execute this package end-to-end:
`docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/`.

Goal: isolate and correct, or mechanism-hold, the active-router raw
outlet-hydrograph nonconvergence for `mn_corn_h4`, day 792, lane 1.

Read package governance, `SC-OFEROUTE-001`, the prior attribution package, and
the WA rev-41 solver correction package before code edits.

Do not promote target `dx`, widen tolerances, revive hybrid code, or change
default/off behavior. Implement only contract-authorized solver corrections.

Subagent authorization: this package explicitly authorizes spawning/delegating
to review, verification, comparator/timing, explorer, and bounded worker
subagents. Expected outputs are package-local review, verification,
comparator/timing, mechanism-attribution, and implementation-readiness
artifacts. Write access is read-only for review/verification/comparator/
explorer roles; worker write access is bounded to package artifacts unless the
executing parent explicitly assigns a disjoint implementation write set.
