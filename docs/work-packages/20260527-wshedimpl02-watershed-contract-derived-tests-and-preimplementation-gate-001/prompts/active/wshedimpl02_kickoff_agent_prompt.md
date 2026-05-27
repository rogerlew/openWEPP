# WSHEDIMPL02 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl01-watershed-contract-authority-closure-and-gap-normalization-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl01-watershed-contract-authority-closure-and-gap-normalization-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`
- `/workdir/wepp-forest_260430_baseline/src/wshcqi.for`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/chrqin.for`
- `/workdir/wepp-forest_260430_baseline/src/imphnw.for`
- `/workdir/wepp-forest_260430_baseline/src/impflo.for`
- `/workdir/wepp-forest_260430_baseline/src/impmai.for`
- `/workdir/wepp-forest_260430_baseline/src/chnero.for`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`

Files:
- `docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/**`
- `docs/work-packages/README.md`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

Task: execute WSHED03 objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`);
typed guards; no silent defaults/clamping; no heuristic/proxy process-physics
substitutions in production migration paths.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Do not perform production kernel/runtime code edits in WSHEDIMPL02.
- WSHEDIMPL02 owns contract-derived vector and pre-implementation gate closure
  only.
- Ensure contract-derived test vectors and gate evidence are complete before
  downstream runtime migration claims.
- Complete dual review (`review_agent_a.md`, `review_agent_b.md`) and dual
  verification (`verification_agent_a.md`, `verification_agent_b.md`) before
  final disposition.
- Preserve fail-closed typed guard posture for missing/non-finite/domain
  violations and record expected-failure vectors truthfully.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
