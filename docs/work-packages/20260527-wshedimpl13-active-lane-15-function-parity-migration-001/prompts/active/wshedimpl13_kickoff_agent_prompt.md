# WSHEDIMPL13 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits and local validation commands only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl13-active-lane-15-function-parity-migration-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl12-worker-handoff-immediate-next-actions-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl12-worker-handoff-immediate-next-actions-closure-001/artifacts/wshedimpl12-follow-on-package-specs.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/impint.for`
- `/workdir/wepp-forest_260430_baseline/src/impflo.for`
- `/workdir/wepp-forest_260430_baseline/src/imphnw.for`

Files:
- `docs/work-packages/20260527-wshedimpl13-active-lane-15-function-parity-migration-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

Task: execute WSHEDIMPL13 objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance; typed guards; no silent defaults/clamping.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
