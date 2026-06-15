# CQR03 Kickoff Agent Prompt

You are working in the local repository at `/home/workdir/openWEPP`.

Execute
`docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/package.md`
end to end.

## Scope

Refactor
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
for behavior-preserving CRAP/function-complexity reduction only.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/standards/module-test-enhancement-authoring-guide.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-boundary-contract.md`
- `docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/artifacts/pl03-runtime-adapter-contract.md`
- `docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/artifacts/pl03-runtime-surface-projection-map.md`
- `crates/AGENTS.md`

## Non-Goals

- Do not change parser/runtime seam behavior, public APIs, symbol names, typed
  errors, arithmetic grouping, thresholds, fallback policy, or guard strictness.
- Do not edit management parser logic or hydrology/growth/decomposition kernels.
- Do not broaden the package to dead-code deletion, naming cleanup, or external
  authority suite posture.

## Required Gates

Run and record:

1. Focused runtime-input management tests.
2. Focused parser/runtime seam management tests.
3. `cargo fmt --check`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo test --workspace`
6. `cargo deny check`
7. Before/after `cargo llvm-cov` and `cargo-crap` evidence for the target file.

Subagent authorization: this package explicitly authorizes
spawning/delegating to review and verification subagents for bounded read-only
review of this package's artifacts and source diff. Expected outputs are
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`; write access is limited to package
artifact files. If subagents are unavailable, perform equivalent independent
local reviews and record that path.
