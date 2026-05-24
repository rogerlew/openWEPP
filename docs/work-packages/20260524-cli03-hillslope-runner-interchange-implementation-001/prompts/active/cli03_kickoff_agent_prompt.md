# CLI03 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/contracts/openwepp-hillslope-runfile-contract.md`
- `/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md`
- `/workdir/openWEPP/docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
Files:
- `docs/contracts/openwepp-hillslope-runfile-contract.md`
- `docs/contracts/openwepp-runner-contract.md`
- `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `crates/openwepp-hillslope-output/**`
- `tests/integration/**`
- `docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/artifacts/cli03-contract-test-implementation-evidence.md`
- `docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/artifacts/cli03-preimplementation-contract-gate.md`
- `docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/artifacts/cli03-output-crate-organization-evidence.md`
Task: execute CLI03 end-to-end with contract-first sequencing, including
contract/test authority checks, contract-derived tests, pre-implementation
gate evidence, production runner/CLI implementation, verification, and
disposition artifacts.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`); typed guards; no silent defaults.
Autonomy: execute package phases end-to-end and update required CLI03 artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update required CLI03 artifacts/disposition for all completed phases.

Mandatory sequencing constraints:
- Do not modify production runner/CLI code until:
  1. canonical contract/spec authority is confirmed sufficient,
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Do not add fallback wrappers that mask missing required inputs/outputs.
- Preserve explicit metric-only enforcement and legacy-sidecar precedence
  semantics from canonical contract authority.
- Keep output contracts/serializers/tests in `crates/openwepp-hillslope-output/`
  instead of embedding them inside runner orchestration modules.
