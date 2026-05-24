# SIMIMPL07 Kickoff Agent Prompt

Scope: local repository mode-propagation integration task; flat-file
reads/edits and local validation only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Files:
- `docs/work-packages/20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001/package.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/tests/simimpl04_wepp_ui_mode_closure_contract.rs`
- `docs/work-packages/20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001/artifacts/simimpl07-preimplementation-contract-gate.md`
- `docs/work-packages/20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001/artifacts/simimpl07-mode-propagation-integration-map.md`
- `docs/work-packages/20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001/artifacts/simimpl07-mode-closure-test-matrix.md`
- `docs/work-packages/20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001/artifacts/simimpl07_disposition.md`
Task: execute SIMIMPL07 end-to-end by propagating parsed `wepp_ui`
requested/effective mode into runtime lane selection and enforcing strict typed
closure for branch mismatch.
Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults; no silent clamping; no bypass of SIMIMPL03/SIMIMPL04/SIMIMPL05
prerequisites; production mode-propagation edits only after SIMIMPL07
pre-implementation gate is recorded.
Autonomy: execute package phases end-to-end and update required SIMIMPL07
artifacts without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Required reading:
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03-contract-amendment-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-contract-derived-test-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-preimplementation-contract-gate.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/artifacts/simimpl05-runner-orchestrator-daily-integration-map.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/artifacts/simimpl05_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl06-simulation-owned-wb13-output-publication-001/artifacts/simimpl06_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
