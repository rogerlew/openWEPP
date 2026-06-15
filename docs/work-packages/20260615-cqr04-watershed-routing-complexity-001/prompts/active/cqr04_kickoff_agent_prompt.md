# CQR04 Kickoff Agent Prompt

You are working in the local repository at `/home/workdir/openWEPP`.

Execute
`docs/work-packages/20260615-cqr04-watershed-routing-complexity-001/package.md`
end to end.

## Scope

Refactor
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`
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
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md`
- `crates/AGENTS.md`

## Non-Goals

- Do not change WS10/WS11/WS20-WS24 routing behavior, equations, constants,
  thresholds, guard strictness, typed guard IDs, symbol names, public APIs, or
  parser/runtime projection behavior.
- Do not edit impoundment logic, runner orchestration, output writers, or
  science contracts.
- Do not broaden the package to dead-code deletion, naming cleanup, or external
  authority suite posture.

## Required Gates

Run and record:

1. Focused WS10/WS11 channel routing tests.
2. `cargo fmt --check`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo test --workspace`
5. `cargo deny check`
6. Before/after `cargo llvm-cov` and `cargo-crap` evidence for the target file.

Reviews and verification are required as package artifacts. If separate
subagents are unavailable or not authorized, perform equivalent independent
local review and verification passes and record that path.
