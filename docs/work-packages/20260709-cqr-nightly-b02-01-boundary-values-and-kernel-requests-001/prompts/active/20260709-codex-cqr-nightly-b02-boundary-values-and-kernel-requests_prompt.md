# CQR Nightly Batch 02 Target 01 Kickoff

Scope: local repository behavior-preserving CQR work; flat-file reads/edits only
inside `/home/workdir/openWEPP`; no external connectivity.

Execution mode: package-end-to-end.

Phase plan: execute every phase in
`docs/work-packages/20260709-cqr-nightly-b02-01-boundary-values-and-kernel-requests-001/package.md`
through disposition.

Required reading:

Core: `AGENTS.md`, `crates/AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`, the package, the CQR ExecPlan,
the mechanical/CQR guides, ADR-0021, and the target module.

Conditional: the specific `SC-*` contract if reading identifies a
contract-derived invariant; `docs/standards/local-ci-gate-selection.md` when a
focused gate needs tier selection.

On-demand: adjacent callers and the target module's existing tests.

Required-reading budget: `~48 KiB`, `OK`; map:
`artifacts/required-reading-map.md`.

Files: the target module and package-local artifacts only.

Task: close target CRAP above `30` behavior-preservingly, or create a legitimate
local hold with rollback proof. Do not change science formulas, thresholds,
contracts, serialization, fail-closed behavior, public output, or typed API.
Preserve arithmetic and short-circuit order.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for heavy
workspace coverage/CRAP, clippy, full-nextest, deny, and comparator work. Do not
run those on the parent model unless the subagent is unavailable and the package
records command-level evidence. This prompt explicitly authorizes subagent
spawning/delegation to comparator/closure-runner, review, and verification
subagents for CQR metric checks, focused/full gates, typed-boundary identity,
review, and verification. Outputs: package-local compact metrics and review /
verification artifacts. Write access: read-only unless explicitly given a bounded
target-module or package-artifact fix.

Autonomy: execute through disposition without further direction unless a declared
hard blocker occurs.
