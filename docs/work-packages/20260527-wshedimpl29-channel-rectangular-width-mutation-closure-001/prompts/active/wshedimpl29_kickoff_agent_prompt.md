# WSHEDIMPL29 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl29-channel-rectangular-width-mutation-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl28-channel-width-boundary-semantics-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl28-channel-width-boundary-semantics-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/dcap.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`
- `/workdir/wepp-forest_260430_baseline/src/hydchn.for`
- `/workdir/wepp-forest_260430_baseline/src/case12.for`
- `/workdir/wepp-forest_260430_baseline/src/case34.for`
- `/workdir/wepp-forest_260430_baseline/src/cchprt.inc`

Files:
- `docs/work-packages/20260527-wshedimpl29-channel-rectangular-width-mutation-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

Task: execute WSHEDIMPL29 objective end-to-end for declared scope, specifically
migrating baseline-authoritative `chnrt.for` rectangular-channel width mutation
semantics by projecting `dcap` eroded-width (`werb`) outcomes into runtime
`widb(i-1)` updates and node-state writeback symbol publication.
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
