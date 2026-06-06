# HPHYS0315 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260606-hphys0315-hourly-snowfall-input-lineage-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/unit-governance.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/work-packages/20260606-hphys0314-adr0017-snow-rm-reclassification-route-ledger-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/snowpack-settling-carry-recursion-ledger.json`

Files: the package write set listed in `package.md`.

Task: execute HPHYS0315 end-to-end. Use contract-first sequencing. Trace the
baseline positive `hrsnow` at 2013 day 11 hour 11 through fixed-baseline
`winter.for -> stmtim.for -> snowd.for` and compare to homologous openWEPP
hourly snowfall-depth surfaces for H1/H7/H39 spring-2014 rows.

Constraints: canonical `SC-*` authority; pinned baseline provenance; ADR0017
same-unit/same-lineage proof; typed fail-closed evidence handling; no silent
defaults; no heuristic/proxy process physics substitutions; no downstream
compensation.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts and disposition for all completed phases,
including dual review and dual verification artifacts.
