# Kickoff Prompt - SNOWDENSITY-10.3.5b

You are working in `/home/workdir/openWEPP`. Execute
`docs/work-packages/20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/`
end-to-end.

Autonomy: proceed through contract, tests, implementation, validation, review,
verification, and disposition without asking for next steps unless a declared
hard blocker is reached.

Subagent authorization: none. This package does not explicitly authorize
subagent spawning/delegation; perform review and verification locally unless a
later package amendment explicitly authorizes delegation.

Contract-first sequence is mandatory:

1. Amend `SC-SNOWFREEZE-001`.
2. Add/update contract-derived tests.
3. Record `artifacts/pre-implementation-contract-gate.md`.
4. Only then edit runtime or validation code.

Protected boundaries: default `legacy_rst` behavior must remain unchanged; do
not add parser/runfile/user CLI activation; do not edit fixtures; do not change
public output schemas; do not change snow density, melt, canopy, radiation,
frost, or compatibility-runtime physics.

Required reading:

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/package.md`

Conditional:

- `/home/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/crates/AGENTS.md`
- `/home/workdir/openWEPP/tests/AGENTS.md`

On-demand:

- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/planning/snow-frost-fidelity-strategy.md`
- `/home/workdir/openWEPP/docs/work-packages/20260627-snowdensity-10-3-5a-openwepp-meteorology-crate-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/tests/fixtures/precip_phase_observed/jennings2018/README.md`
- `/home/workdir/openWEPP/references/annotated_bibliography.md`

Required-reading budget metrics must be recorded in
`artifacts/required-reading-map.md`.

Close with `COMPLETE-10-3-5B-HOURLY-PARTITION-JENNINGS-VALIDATED` only if every
current-scope gate has current evidence. If the full local Jennings file2 corpus
is unavailable, close `HOLD-JENNINGS-FILE2-ABSENT` unless the package was
amended before implementation.
