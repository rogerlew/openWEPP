# HPHYS0282 Kickoff Agent Prompt

Scope: local repository science-contract/kernel governance task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0282-sc-evap-unit-compliance-closure-001/package.md
- /workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md
- /workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md
- /workdir/openWEPP/docs/specifications/science-contracts/index.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- /workdir/openWEPP/docs/specifications/units/boundary-symbol-unit-registry.md
- /workdir/openWEPP/crates/openwepp-sim-contract/src/units.rs
- /workdir/openWEPP/tools/release/check_sc_unit_compliance.sh
- /workdir/openWEPP/tests/integration/hphys0279_sc_unit_compliance_lint_contract.rs
- /workdir/openWEPP/docs/work-packages/20260604-hphys0281-wb11-evappm-condensation-closure-001/artifacts/disposition.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0281-wb11-evappm-condensation-closure-001/artifacts/gate-results.md

Files:
- docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- docs/work-packages/README.md
- docs/work-packages/20260604-hphys0282-sc-evap-unit-compliance-closure-001/**

Task: execute package objective end-to-end for declared scope. Resolve the remaining `SC-EVAP-001` unit-compliance lint findings by aligning canonical Variables and Units rows plus Symbol Alias Map rows with registered WAT output `Ep`, `Es`, and `Er` symbols.

Constraints: contract-first sequencing; canonical SC authority; typed unit governance; no production physics changes; no heuristic or proxy process-physics substitutions; preserve distinct runtime/process rate units and WAT publication depth units; dual independent review and dual verification required.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases, including pre-fix lint evidence, gate results, review finding dispositions, verification artifacts, and worker handoff.
