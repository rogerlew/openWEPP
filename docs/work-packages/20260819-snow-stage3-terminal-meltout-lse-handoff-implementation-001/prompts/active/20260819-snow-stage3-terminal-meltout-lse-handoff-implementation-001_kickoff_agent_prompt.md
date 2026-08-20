# Execute Stage 3 terminal meltout to the real snow-free owner stack

Scope: local repository science-contract/kernel integration task; flat-file
reads/edits only; no external connectivity, messages, deployment, or release.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through truthful
disposition.

Required reading (before edits):

Core: `AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`, `docs/work-packages/README.md`,
`docs/standards/testing-and-gate-strategy.md`, and this package's `package.md`.

Conditional: read `docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contract-authoring-procedure.md`,
`docs/specifications/science-contracts/kernel-process-contract-profile.md`, and
`docs/specifications/science-contracts/index.md` before contract/kernel
authority edits; read ADR-0012 and pinned baseline sources only for touched
legacy mappings; read the prompt/preparation standards when changing this
scaffold.

On-demand: touched sections of `SC-SNOWENERGY-001`, `SC-SNOWFREEZE-001`, and
applicable land-surface, vegetation, evaporation, soil/frost, water-balance,
infiltration/runoff, and restart contracts; prerequisite package dispositions;
and exact source/test modules named by the owned-file manifest.

Required-reading budget: `495418` local Core bytes at scaffold time, `WARN`;
remeasure before edits using `artifacts/required-reading-map.md`. The package
catalog is large but is a mandatory Core input. Apply `OK <=400000`,
`WARN >400000`, and `REQUIRES-JUSTIFICATION >800000`.

Task: implement the complete localized terminal-event-to-real-receiver
transaction in the actual scheduler candidate, including exact-one liquid
handoff, remaining-time surface forcing rebuild, persisted restart, and atomic
whole-owner rollback. Preserve CoE production ownership and all absent/default
production bytes.

Constraints: contract-first sequence is contracts, contract-derived tests,
passing pre-implementation contract gate, then production edits. Use canonical
SC authority and pinned baseline provenance where applicable. Require typed
fail-closed guards; no silent defaults, unbounded clamps, or canonicalize-and-
proceed on domain violations.

No surrogate physics: production code must use actual contract-backed or
baseline-authoritative physics. Surrogate, provisional, proxy, fitted, fixed-
attenuation, or heuristic stand-ins are forbidden. Missing authority is a
declared hold boundary; known in-scope authority is an implementation
obligation.

Real consumer proof: the actual scheduler and complete V10/LSE-V2/direct-
hydrology owner stack must read the handoff. Wrappers, adapters, skeletons,
snowbench loops, shadow-internal counters, CoE receiving state, and old
compatibility paths cannot carry closure.

Conservation/output acceptance: record operand lineage; separate plausible
aliases with unequal fixtures; reject named wrong formulas; independently
reconstruct actual consumer evidence; run real two-sided closure/magnitude
audits; and align metadata/schema. One-sided bounds and self-consistency are
supporting evidence only.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for heavy full-
workspace/comparator runs; do not run them on the parent model unless genuinely
unavailable and command-level evidence records why. This prompt explicitly
authorizes subagent spawning/delegation to the snow/land-surface science,
hydrology/ownership, Rust correctness, Rust QA, comparator, and two terminal-
verifier roles named in `package.md`; outputs are compact findings/metrics and
artifact/log paths; reviewers/verifiers are read-only and comparator writes are
limited to ignored logs and bounded gate artifacts.

Autonomy: execute the full package and update artifacts without requesting
direction unless hard-blocked. A phase or package may pass only with direct
current evidence for every required gate; otherwise record truthful HOLD with
the blocker. Do not expand the claim beyond mechanical handoff.
