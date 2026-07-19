# Required Reading Map

Static: package scaffolding on 2026-07-18 identified the following authorities.

## Core

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md`
- `docs/decisions/0040-accelerated-testgate-cutover-on-trusted-self-hosted-runner.md`
- `docs/standards/testing-and-gate-strategy.md`, especially sections 8-10,
  12, 14, and 19
- `docs/standards/local-ci-gate-selection.md`
- this package

## Implementation

- nearest instruction chain returned by `tools/agents/find-agents --for` for
  the final write set
- `.github/workflows/testgate-shadow.yml`
- `.github/workflows/release-gates.yml`
- `tools/local_ci/README.md`
- `tools/release/README.md`
- relevant planner, executor, verifier, policy, schema, fixture, and test files

## External Runner Authority

- GitHub's official adding-self-hosted-runners, self-hosted-runner reference,
  secure-use, and billing/usage documentation linked from ADR-0040

## Intake Rule

Refresh this map after discovery and before host or repository edits. Read any
new nearest `AGENTS.md` completely. Record the exact provider and host facts
used by execution; do not rely on this scaffold's 2026-07-18 snapshot if those
facts have changed.
