# CQR Nightly Batch 02 Target 03 Kickoff

Scope: local repository behavior-preserving CQR work; flat-file reads/edits only
inside `/home/workdir/openWEPP`; no external connectivity.

Execution mode: package-end-to-end.

Phase plan: execute every phase in
`docs/work-packages/20260709-cqr-nightly-b02-03-soil-parser-001/package.md`
through disposition.

Required reading:

Core: `AGENTS.md`, `crates/AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`, the package, the CQR ExecPlan,
the mechanical/CQR guides, ADR-0021, `SC-INFILE-SOIL-001`, `SC-SOIL-001`, and
the target module.

Conditional: `docs/standards/local-ci-gate-selection.md` when a focused gate
needs tier selection.

On-demand: adjacent parser callers and existing target tests.

Required-reading budget: `~100 KiB`, `OK`; map:
`artifacts/required-reading-map.md`.

Files: the target module and package-local artifacts only.

Task: close target CRAP above `30` behavior-preservingly, or create a legitimate
local hold with rollback proof. Do not change `.sol` grammar, datver policy,
science formulas, thresholds, contracts, serialization, fail-closed behavior,
public output, typed API, token order, or numeric operation order.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for heavy
workspace coverage/CRAP, clippy, full-nextest, deny, and comparator work. Do not
run those on the parent model unless the subagent is unavailable and the package
records command-level evidence. This prompt explicitly authorizes subagent
spawning/delegation to comparator/closure-runner, review, and verification
subagents for CQR metric checks, focused/full gates, `.sol` behavior identity,
review, and verification. Outputs: package-local compact metrics and review /
verification artifacts. Write access: read-only unless explicitly given a
bounded target-module or package-artifact fix.

Autonomy: execute through disposition without further direction unless a declared
hard blocker occurs.
