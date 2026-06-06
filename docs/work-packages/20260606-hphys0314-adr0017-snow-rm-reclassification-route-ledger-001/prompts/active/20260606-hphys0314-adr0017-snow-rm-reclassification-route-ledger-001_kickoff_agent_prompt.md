# HPHYS0314 Kickoff Prompt

Scope: local repository science-contract/kernel governance task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260606-hphys0314-adr0017-snow-rm-reclassification-route-ledger-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/unit-governance.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/snowpack-settling-carry-recursion-ledger.json`

Files: the package write set listed in `package.md`.

Task: execute HPHYS0314 end-to-end. Use contract-first sequencing. Reclassify
HPHYS0298-HPHYS0313 snow/`RM` and water-balance rows under ADR0017, publish a
route-consolidation ledger, run full H1..H39 metrics, and produce owned
continuation order for HPHYS0315 and HPHYS0316.

Constraints: canonical `SC-*` authority; ADR0017 taxonomy; pinned baseline
provenance; typed fail-closed evidence handling; no silent defaults; no
heuristic/proxy process physics substitutions; no production physics edits.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts and disposition for all completed phases,
including dual review and dual verification artifacts.
