# Execute TESTGATE Agent Dispatch Only

Scope: local openWEPP workflow, test, policy, and documentation correction.

Read and follow `AGENTS.md`, `tests/AGENTS.md`,
`docs/work-packages/AGENTS.md`, `docs/standards/AGENTS.md`,
`docs/standards/testing-and-gate-strategy.md`, `tools/local_ci/README.md`, and
this package.

Remove the TESTGATE push trigger and retain explicit `workflow_dispatch` only.
Require an agent to supply the exact active intent package and comparison base.
Preserve forest1 HEAVY execution, GitHub-hosted verification/attestation,
current-main checks, fail-closed admission, and concurrency.

Add source-contract coverage that forbids automatic push execution. Update
canonical and operator guidance, rebind the policy digest, run focused checks,
obtain dual review and verification, and close the package. Do not manually
dispatch TESTGATE as part of this correction.

Subagent authorization: this prompt explicitly authorizes two independent
read-only implementation reviewers and two read-only terminal verifiers for
the workflow, tests, policy, and documentation diff. Expected outputs are
concise findings returned to the parent. No delegated role may edit, commit,
push, dispatch, or execute TESTGATE.
