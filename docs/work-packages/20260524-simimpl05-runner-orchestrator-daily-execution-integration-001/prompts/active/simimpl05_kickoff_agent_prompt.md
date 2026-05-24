# SIMIMPL05 Kickoff Agent Prompt

Scope: local repository runner/orchestrator integration task; flat-file
reads/edits and local validation only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Files:
- `docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/package.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/tests/simimpl04_runner_kernel_execution_contract.rs`
- `docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/artifacts/simimpl05-preimplementation-contract-gate.md`
- `docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/artifacts/simimpl05-runner-orchestrator-daily-integration-map.md`
- `docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/artifacts/simimpl05_disposition.md`
Task: execute SIMIMPL05 end-to-end by integrating daily runner path execution
with scheduler/kernel lifecycle, preserving typed guard behavior, and closing
`GAP-SIMPIPE-001` for daily lane production flow.
Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults; no silent clamping; no bypass of SIMIMPL04 gate
preconditions; production edits only after SIMIMPL05 pre-implementation gate is
recorded.
Autonomy: execute package phases end-to-end and update required SIMIMPL05
artifacts without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Required reading:
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03-contract-amendment-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-contract-derived-test-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-expected-fail-pass-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-preimplementation-contract-gate.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
