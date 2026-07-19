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

- `tools/agents/find-agents --for` was run before implementation edits. The
  applicable chains are root `AGENTS.md` for workflows, policy, tools, and
  roadmap files; root plus `crates/AGENTS.md` for planner Rust; root plus
  `tests/AGENTS.md` for integration/Python tests; root plus
  `docs/standards/AGENTS.md` for standards; and root plus
  `docs/work-packages/AGENTS.md` for package/catalog files.
- `.github/workflows/testgate-shadow.yml`
- `.github/workflows/release-gates.yml`
- `tools/local_ci/README.md`
- `tools/release/README.md`
- relevant planner, executor, verifier, policy, schema, fixture, and test files

Read completely at intake: root, crate, test, standard, and work-package
instruction files; ADR-0039; ADR-0040; testing/gate strategy sections 8-10, 12,
14, and 19; local CI selection; local CI operator README; release tooling
README; this package; and this map.

## External Runner Authority

- GitHub's official adding-self-hosted-runners, self-hosted-runner reference,
  secure-use, and billing/usage documentation linked from ADR-0040

## Intake Rule

Refresh this map after discovery and before host or repository edits. Read any
new nearest `AGENTS.md` completely. Record the exact provider and host facts
used by execution; do not rely on this scaffold's 2026-07-18 snapshot if those
facts have changed.
