# Execute SNOW-MASS-TRANSITION-LEDGER-PERSISTENCE

Scope: local repository contract, Rust architecture, tests, and bounded local
fixture execution only; no external connectivity.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through
disposition.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260803-snow-mass-transition-ledger-persistence-001/package.md`
- `/home/workdir/openWEPP/docs/standards/testing-and-gate-strategy.md`

Conditional because this package amends a contract and refactors kernel/runtime
projection plus serialization:

- `/home/workdir/openWEPP/docs/standards/AGENTS.md`
- `/home/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/home/workdir/openWEPP/docs/standards/local-ci-gate-selection.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/crates/AGENTS.md`
- `/home/workdir/openWEPP/tests/AGENTS.md`

On-demand:

- `SC-SNOWFREEZE-001.md`, `SC-SNOWENERGY-001.md`, and
  `SC-RUNOFFPART-001.md` for the touched transition and output boundaries.
- The predecessor audit and schema-v4 closure package, especially their
  disposition, operand-lineage, real-consumer, and worker-handoff artifacts.
- The exact source files in the package write set.

Required-reading budget: `540600` local bytes, classification `WARN`; map:
`artifacts/required-reading-map.md`. The work-package catalog dominates the
budget but remains Core because package-chain and active-package reconciliation
are mandatory.

## Task

Execute the package objective end-to-end for the declared write set. Preserve
one authoritative physical calculation, implement two exact linked compact
mass-transition ledgers, and make the large hourly diagnostic payload opt-in at
collection as well as file emission.

Constraints: capture immutable scaffold binaries and baseline evidence before
production edits; then follow contract-first sequencing, canonical `SC-*`
authority, typed guards, no silent defaults, no canonicalize-and-proceed, and no
fixture or observation edits.

No surrogate physics: do not add, approximate, infer, or recompute a physical
operand for either ledger. Split exact existing outcomes and diagnostics only.

Single authority: no second snow state, independently mutable accounting
object, or duplicate closure calculation. The two ledgers must be exact linked
views of the authoritative result.

Capture policy: preserve the existing environment opt-in, translate it into a
typed internal request before snow calculation, preserve absent/empty/disabled
equivalence, and prove the selector cannot affect physics or ordinary outputs.

Real consumer proof: the release CLI must exercise both trace-disabled and
trace-enabled paths. The independent parser must read real schema-v4 JSONL.
Wrappers, adapters, skeletons, shadows, producer-only counters, and unused
payloads cannot carry the claim.

Conservation/output acceptance: record operand lineage; separate plausible
aliases; reject every named wrong formula; independently reconstruct both
ledgers from compact carriers and real JSONL; prove protected output identity.

Performance acceptance: retain exact scaffold and candidate binaries, hash and
copy the package-named retained Snowbird fixture without modifying its source,
run the frozen paired timing/RSS protocol, record every sample and binary hash,
enforce the 5% trace-disabled non-regression and 1% trace-size bounds, and avoid
trading eager copies for hidden heap churn.

Structural discipline: move only the ledger/capture seam, record public-symbol
and field ownership parity, place new code in bounded modules, and disposition
all 2000-line WARN files. Do not hide semantic changes in mechanical movement.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for heavy
quick/frost/full and paired performance/comparator gates; do not run those on
the parent model unless the role is unavailable, in which case retain exact
command-level evidence. This prompt explicitly authorizes subagent
spawning/delegation to two read-only independent reviewers, two read-only
terminal verifiers, and the comparator runner for the scopes and outputs in
`package.md`.

Autonomy: execute all phases and update every required artifact without asking
for additional user direction unless a declared hard blocker is proven.

Outputs: archive this prompt unchanged after use and update package status,
evidence, reviews, finding disposition, verifications, handoff, catalog,
roadmaps, and final disposition.
