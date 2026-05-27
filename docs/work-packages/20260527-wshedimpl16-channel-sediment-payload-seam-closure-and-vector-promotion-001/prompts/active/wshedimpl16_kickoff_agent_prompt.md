# WSHEDIMPL16 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits and local validation commands only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl16-channel-sediment-payload-seam-closure-and-vector-promotion-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl15-watershed-channel-sediment-process-parity-migration-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl15-watershed-channel-sediment-process-parity-migration-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md`
- `/workdir/wepp-forest_260430_baseline/src/chnero.for`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`

Files:
- `docs/work-packages/20260527-wshedimpl16-channel-sediment-payload-seam-closure-and-vector-promotion-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-input-contract/src/parsers/hbp.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-sim-contract/src/symbols.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/erod15_wave3_contract_authority_closure_contract.rs`
- `tests/integration/ws10_watershed_kernel_contract.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

Task: execute WSHEDIMPL16 objective end-to-end for declared scope.
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
