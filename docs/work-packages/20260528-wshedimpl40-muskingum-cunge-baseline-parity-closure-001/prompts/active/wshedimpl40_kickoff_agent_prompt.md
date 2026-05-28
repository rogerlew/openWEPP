# WSHEDIMPL40 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl40-muskingum-cunge-baseline-parity-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl39-out-of-scope-gap-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl39-out-of-scope-gap-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl39-out-of-scope-gap-closure-001/artifacts/wshedimpl39_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl05-watershed-wave-routing-state-family-migration-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/wshpek.for`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`

Files:
- `docs/work-packages/20260528-wshedimpl40-muskingum-cunge-baseline-parity-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md` (if required)
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

Task: execute WSHEDIMPL40 objective end-to-end for declared scope by
identifying Muskingum-Cunge (`ipeak >= 4`) implementation gaps relative to the
pinned baseline and closing those gaps to baseline-authoritative parity with
contract-derived evidence.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance; typed guards; no silent defaults/clamping; no heuristic/proxy
process-physics substitutions in production parity paths.
Mandatory gates:
- Do not edit production kernel/runtime code before completing:
  1) contract amendments,
  2) contract-derived tests, and
  3) pre-implementation contract gate artifact.
- Complete dual review artifacts (`review_agent_a.md`, `review_agent_b.md`)
  and dual verification artifacts (`verification_agent_a.md`,
  `verification_agent_b.md`) before final disposition.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

False-positive block fallback (required):
- If a policy false-positive blocks the full kickoff prompt, retry with a
  shorter prompt containing only scope sentence, single phase objective, and
  explicit file list.
- If blocked again, split into micro-prompts by file group and continue.
- Record each block event and resumed prompt shape in package artifacts.
