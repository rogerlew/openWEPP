# HPHYS0313 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/work-packages/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001/artifacts/prior-year-terminal-snowpack-lineage-ledger.json`

Files: the package write set listed in `package.md`.

Task: execute HPHYS0313 end-to-end. Use contract-first sequencing. Add contract
amendments and contract-derived tests before diagnostics or production edits.
Run the split-route diagnostic. Do not edit production kernel code unless the
ledger proves an openWEPP-owned source-line defect.

Constraints: canonical `SC-*` authority; pinned baseline provenance; typed
fail-closed evidence handling; no silent defaults; no heuristic/proxy process
physics substitutions; no branch-predicate, melt-term, WB13, WB17, WB18, WB19,
or WB12 compensation while source ownership remains unresolved.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts and disposition for all completed phases,
including dual review and dual verification artifacts.
