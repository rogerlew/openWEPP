# WSHEDIMPL04 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl04-watershed-runtime-seam-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/imphnw.for`
- `/workdir/wepp-forest_260430_baseline/src/impflo.for`
- `/workdir/wepp-forest_260430_baseline/src/impmai.for`

Files:
- `docs/work-packages/20260527-wshedimpl04-watershed-runtime-seam-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/ws10_watershed_kernel_contract.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

Task: execute WSHED04 objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`);
typed guards; no silent defaults/clamping; no heuristic/proxy process-physics
substitutions in production migration paths.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Do not claim WSHED07 parity-authoritative continuity/regime-transition
  closure from WSHED04 output.
- Preserve fail-closed typed guard posture for missing/non-finite/domain
  violations and surface deterministic seam errors for unsupported projection
  payloads.
- Close manual coefficient-seeding dependency in WS10/WS11/WS12 tests.
- Promote WS12 parser-projection vector out of ignored expected-failure posture
  once seam closure is runtime-true.
- Complete dual review (`review_agent_a.md`, `review_agent_b.md`) and dual
  verification (`verification_agent_a.md`, `verification_agent_b.md`) before
  final disposition.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
