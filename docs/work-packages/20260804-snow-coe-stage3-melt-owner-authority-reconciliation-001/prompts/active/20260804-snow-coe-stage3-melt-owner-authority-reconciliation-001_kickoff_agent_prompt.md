# 21N Kickoff Agent Prompt

Scope: local repository science-contract/kernel authority task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through
disposition.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- package-local `package.md`

Conditional, required because canonical contracts and one contract test change:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contract-spec.md`
- `docs/specifications/science-contracts/index.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/standards/prompt-wording-guidance.md`
- `tests/AGENTS.md`

On-demand, required for the touched ownership seam:

- `SC-SNOWFREEZE-001.md`
- `SC-SNOWENERGY-001.md`
- the 21M package and named authority artifacts;
- current CoE/Stage 3 source and affected contract test; and
- pinned libsnobal energy/melt/mass files named by `authority-freeze.json`.

Required-reading budget: `1435550` local bytes,
`REQUIRES-JUSTIFICATION`, because the canonical snow/freeze contract and
mandatory work-package catalog are large. Map:
`artifacts/required-reading-map.md`. Both heavy files are indispensable: the
catalog is mandatory Core governance and the contract contains the active
melt/Stage 3 clauses being superseded.

Files: only the intended write set in `package.md`.

Task: execute 21N end-to-end. Apply the frozen outcome matrix, amend canonical
authority first, reconcile the affected static test second, and leave runtime
behavior unchanged.

Constraints: canonical `SC-*` authority, exact provenance, typed guards, no
silent defaults, and no canonicalize-and-proceed behavior for domain
violations.

No surrogate physics: any admitted production target must use actual
contract-backed Marks/SNOBAL energy, cold-content, melt, and liquid-routing
physics. Surrogate, provisional, proxy, heuristic, or fitted stand-ins are
forbidden. Missing component authority becomes an explicit implementation
hold.

Conservation/output acceptance: preserve one authoritative solid-to-liquid
ledger and one downstream liquid-disposition ledger; identify and reject dual
melt generation and adjacent aliases. This authority-only package does not
claim runtime closure for behavior it does not implement.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to two read-only science/contract reviewers, two read-only
terminal verifiers, and one read-only `comparator_suite_runner` for selected
heavy correctness runs. Outputs: compact findings, metrics, and log paths.
Write access: none.

Autonomy: execute all package phases and update artifacts without requesting
additional user direction unless hard-blocked.
