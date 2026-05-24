# CLI03 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Phase: B only.
Files:
- `docs/contracts/openwepp-hillslope-runfile-contract.md`
- `docs/contracts/openwepp-runner-contract.md`
- `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- `crates/openwepp-hillslope-output/**`
- `tests/integration/**`
- `docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/artifacts/cli03-contract-test-implementation-evidence.md`
- `docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/artifacts/cli03-preimplementation-contract-gate.md`
- `docs/work-packages/20260524-cli03-hillslope-runner-interchange-implementation-001/artifacts/cli03-output-crate-organization-evidence.md`
Task: implement CLI03 contract-derived tests and record pre-implementation gate
artifacts for Phase B only.
Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`); typed guards; no silent defaults.
Outputs: update listed CLI03 artifacts for this phase only.

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
