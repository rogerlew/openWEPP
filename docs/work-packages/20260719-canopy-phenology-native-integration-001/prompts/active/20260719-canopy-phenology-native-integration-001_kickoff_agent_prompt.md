# CANOPY-PHENOLOGY-02 Kickoff

Scope: local repository science-contract/kernel integration task; flat-file
reads and edits only; no external connectivity or external-system actions.

Execution mode: package-end-to-end. Execute every phase in `package.md`
sequentially through truthful disposition.

## Required reading

Core: `/workdir/openWEPP/AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`, `docs/work-packages/README.md`, this package.

Conditional: `docs/specifications/science-contract-authoring-procedure.md`,
`docs/specifications/science-contracts/kernel-process-contract-profile.md`,
`docs/specifications/science-contracts/index.md`,
`docs/standards/testing-and-gate-strategy.md`, `crates/AGENTS.md`,
`tests/AGENTS.md`, and `tests/fixtures/AGENTS.md` all apply.

On-demand: `SC-PLANT-001`, `SC-RESIDUE-001`,
`SC-INFILE-MANAGEMENT-YAML-001`, the completed GSI package, the phenology
backlog, and touched source/consumer files.

Required-reading budget: approximately 530,000 local bytes including package,
`WARN`; map: `artifacts/required-reading-map.md`. The work-package catalog is
large but is core governance; mechanism contracts remain on-demand.

Task: execute the package objective end-to-end within its declared write set.

Constraints: contracts first, then contract-derived tests, then the recorded
pre-implementation gate, then production code. Use canonical SC authority,
typed guards, no silent defaults, and no canonicalize-and-proceed for invalid
domains.

No surrogate physics: production code must implement the ratified GSI/end-point
state and baseline-authoritative canopy relation. Provisional, proxy, fitted,
or heuristic stand-ins are forbidden.

Real consumer proof: prove snow, ET, WB15 interception, residue/frost, and
erosion consume post-phenology state, and prove static/fixed-date compatibility
paths do not carry the claim.

Conservation/output acceptance: record operand lineage, distinguish plausible
aliases, reject fixed-date and aggregate-biomass formulas, independently
reconstruct daily foliar mass closure, run a real multi-season no-drift and
litter-pool audit, and align YAML/runtime semantics.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to one read-only `comparator_suite_runner` for all
TESTGATE-selected heavy workspace/coverage/CRAP commands, two independent
read-only reviewers, and two independent read-only verifiers. Outputs are
compact metrics, log paths, findings, and verdicts; production write access is
none. Do not run heavy closure commands on the parent model unless the runner
subagent is unavailable and command-level evidence records why.

Autonomy: execute through disposition without requesting further direction
unless an actual authority or external-evidence boundary blocks progress.
