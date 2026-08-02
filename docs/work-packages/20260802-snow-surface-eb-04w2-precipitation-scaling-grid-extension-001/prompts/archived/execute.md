# Execute SNOW-SURFACE-EB-04W2

Lifecycle: archived after execution and dual review.

Scope: local empirical precipitation-scaling grid extension; package-local
fixture copies and artifacts only; no external connectivity.

Execution mode: package-end-to-end through disposition.

Required reading: root `AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`, package-local `package.md`, ADR-0042, the testing
strategy, EB-04W1 freeze/results/disposition/tool, and the package required-
reading map.

Task: freeze and execute exactly 20 new `1.6-2.0` cells, combine them with the
24 immutable EB-04W1 `1.0-1.5` anchors, apply the prospectively fixed magnitude,
chronology, compensation, and experiment-budget rules, and publish accessible
figures with Markdown sidecars.

Constraints: no production physics, contract, fixture, observation, selector,
default, assurance, or schema changes. Observations remain calibration-only.
The `2.0` ceiling is final for this forcing branch; no W3 extension is admitted.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent science/QA reviewers and two terminal
verifiers. Each role writes only its named compact package artifact. No heavy
workspace suite or comparator batch is selected.

Autonomy: execute end to end without requesting direction unless a declared
hard blocker prevents current-scope acceptance.
