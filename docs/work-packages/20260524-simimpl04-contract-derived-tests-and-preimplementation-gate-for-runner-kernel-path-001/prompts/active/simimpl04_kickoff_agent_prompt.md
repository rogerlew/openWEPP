# SIMIMPL04 Kickoff Agent Prompt

Scope: local repository contract-derived test and gate package; flat-file
reads/edits and local test execution only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Files:
- `docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/package.md`
- `crates/openwepp-runner/tests/simimpl04_runner_kernel_execution_contract.rs`
- `crates/openwepp-runner/tests/simimpl04_wepp_ui_mode_closure_contract.rs`
- `crates/openwepp-runner/tests/simimpl04_wb13_publication_contract.rs`
- `docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-contract-derived-test-plan.md`
- `docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-expected-fail-pass-matrix.md`
- `docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-preimplementation-contract-gate.md`
- `docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04_disposition.md`
Task: execute SIMIMPL04 end-to-end by implementing contract-derived tests and
recording pre-implementation contract-gate evidence for runner-kernel path
integration, mode closure, and simulation-owned output provenance.
Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults; no silent clamping; no production runner/orchestrator code
edits in this package; tests/gates only.
Autonomy: execute package phases end-to-end and update required SIMIMPL04
artifacts without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Required reading:
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03-contract-amendment-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-owner-surface-gap-closure-map.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-contract-invariant-crosswalk.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simimpl01-pipeline-gap-audit.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
