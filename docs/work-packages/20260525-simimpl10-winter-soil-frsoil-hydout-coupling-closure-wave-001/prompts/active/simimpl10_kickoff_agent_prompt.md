# SIMIMPL10 Kickoff Agent Prompt

Scope: local repository winter/frozen-soil coupling integration task; flat-file
reads/edits and local validation only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Files:
- `docs/work-packages/20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001/package.md`
- `crates/openwepp-runner/src/lib.rs`
- `crates/openwepp-runner/tests/simimpl04_wepp_ui_mode_closure_contract.rs`
- `docs/work-packages/20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001/artifacts/simimpl10-preimplementation-contract-gate.md`
- `docs/work-packages/20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001/artifacts/simimpl10-coupling-vector-integration-map.md`
- `docs/work-packages/20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001/artifacts/simimpl10-coupling-validation-matrix.md`
- `docs/work-packages/20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001/artifacts/simimpl10-unresolved-coupling-residual-register.md`
- `docs/work-packages/20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001/artifacts/simimpl10_disposition.md`
Task: execute SIMIMPL10 end-to-end by closing winter/soil/frsoil/hydout
coupling gaps in production execution flow with typed invariants, explicit
boundary provenance, and no silent fallback behavior.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance; typed guards; no silent defaults; no silent clamping; no production
edits before SIMIMPL10 pre-implementation contract gate is recorded.
Autonomy: execute package phases end-to-end and update required SIMIMPL10
artifacts without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Required reading:
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/artifacts/simulation-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-contract-invariant-crosswalk.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/artifacts/simimpl02-routine-owner-surface-gap-closure-map.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03-contract-amendment-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/artifacts/simimpl03_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-contract-derived-test-plan.md`
- `/workdir/openWEPP/docs/work-packages/20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/artifacts/simimpl04-preimplementation-contract-gate.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001/artifacts/simimpl09_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001/artifacts/simimpl09-adapter-boundary-closure-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001/artifacts/simimpl09-timestep-policy-surface-map.md`
