# Kickoff — CHAN.INP raw cardinality closure

Execution mode: package-end-to-end defect closure
Autonomy: close `CHANINP-RAW-NCHNUM-CARDINALITY` through terminal disposition
without asking for next steps unless a declared hard boundary is proven.

Core reading: root `AGENTS.md`, `docs/codex_exec_plans.md`, work-package
`AGENTS.md`/README, and this package. Conditional applicable reading:
`docs/defect_closure_execplans.md`, science-contract authoring procedure/profile/
index, ADR-0012, ADR-0021, and CQR/module-test guides. On-demand mechanism
reading: target contract/spec/source/tests, pinned legacy files, network-frame
consumer, originating nightly hold, and follow-up ExecPlan. Required-reading
budget: `678351` local bytes, `WARN`, justified in
`artifacts/required-reading-map.md`.

Close the defect end-to-end. Contract and spec first; then failing contract
tests; then independent pre-implementation PASS; only then production. Prove
invalid raw 99 plus two IDs fails `CHN-E-002`, a separate raw-count-closed input
retains 99 before normalization, and the consumer reads normalized topology.
Complete A-H, science-tier coverage, CRAP, full gates, dual review/disposition,
and dual verification. No silent default, clamp, or diagnostic relay.

HOLD legitimacy audit: name/prove the boundary and why no in-envelope correction
can close it. Subagent authorization: this prompt explicitly authorizes
spawning/delegation to independent reviewers/verifiers and heavy coverage/gate
runners under the read/write limits in `package.md`.
