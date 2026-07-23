# TESTGATE Agent Dispatch Only

Package: `20260723-testgate-agent-dispatch-only-001`
Status: `COMPLETE`
Defect: `TGD-001`
Cause: `GATE-AUTOMATIC-DISPATCH-PACKAGE-ANCHOR-MISMATCH`

## Objective

Suppress automatic TESTGATE execution on pushes to `main`. Retain only explicit
agent dispatch with an exact intent-package input and comparison base, while
preserving forest1 HEAVY execution and GitHub-hosted verification and
attestation.

## Authority

Roger Lew's 2026-07-23 direction to make TESTGATE dispatch-only after automatic
push runs repeatedly reached package admission without a usable active package
anchor.

## Correction Authority Envelope

- Observed violation: a stable push may contain both a prospective package
  scaffold and its completion. The push event's prior remote head predates the
  active scaffold, so the automatic run cannot use that scaffold as its
  base-commit anchor and may fail with
  `GATE-PACKAGE-CHAIN-ANCHOR-MISSING`.
- In scope: TESTGATE workflow triggers and admission inputs, source-contract
  tests, canonical testing guidance, operator documentation, policy binding,
  package evidence, and catalog state.
- Acceptance: pushes to `main` do not start TESTGATE; explicit
  `workflow_dispatch` requires `intent_package`, accepts an exact `base_ref`,
  and retains current-head, forest1 execution, hosted verification, and
  attestation guards.
- Protected boundaries: no gate selection, threshold, HEAVY implementation,
  runner label, attestation identity, concurrency identity, or receipt-trust
  relaxation.

## Declared Write Set

- `.github/workflows/testgate-shadow.yml`
- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/standards/testing-and-gate-strategy.md`
- `gate-policy/v1/impact-map.json`
- `tools/local_ci/README.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260723-testgate-agent-dispatch-only-001/**`

## Required Reading

- `AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/testing-and-gate-strategy.md`
- `tools/local_ci/README.md`

## Review Authorization

Subagent authorization: this package explicitly authorizes two independent
read-only implementation reviewers and two read-only terminal verifiers for
the workflow, tests, policy, and documentation diff. Expected outputs are
concise findings returned to the parent; reviewers and verifiers may not edit,
commit, push, dispatch, or execute TESTGATE.

## Progress

- [x] Scaffold prospective correction authority before implementation.
- [x] Remove automatic push execution and bind explicit agent dispatch.
- [x] Add negative source-contract coverage for forbidden push triggers.
- [x] Update canonical and operator guidance; rebind policy digest.
- [x] Run focused validation and obtain dual implementation review.
- [x] Close `TGD-001`, complete dual terminal verification, archive the prompt,
  and record final disposition.
- [ ] Push once and verify that the push does not create a TESTGATE run.

## Exit Criteria

- `.github/workflows/testgate-shadow.yml` has only `workflow_dispatch`.
- The intent package remains required and `base_ref` remains explicit and
  fail-closed.
- A focused contract test rejects reintroduction of `push`.
- Forest1 and GitHub-hosted job roles, current-head checks, concurrency, and
  attestation behavior are unchanged.
- Canonical guidance provides the exact agent dispatch command and forbids
  automatic push execution.
- Markdown, YAML/source contract, policy binding, formatting, and diff checks
  pass.
- Dual review and dual verification pass with no open finding.
