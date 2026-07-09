# Agent Instruction Discovery Index Tooling

Package: `20260709-agent-instruction-discovery-index-tooling-001`
Status: `EXECUTED-COMPLETE-AGENT-INSTRUCTION-DISCOVERY`
Date: 2026-07-09

## Objective

Implement a fast, agent-discoverable way to find applicable `AGENTS.md` files
for an intended write set. The package adds a small command-line helper, a
committed guidance index, and root/work-package playbook pointers so future
agents can run one command before editing instead of manually searching the tree.

## Rationale

Work-package execution now touches many subtrees. Agents are missing nested
`AGENTS.md` files or spending unnecessary time scanning large package/artifact
trees. The current repo has a small instruction set, but the discovery mechanism
is implicit. This package makes instruction discovery an explicit, cheap
pre-edit step.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/README.md`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review and acceptance-test subagents for the instruction
discovery tool, guidance docs, work-package pointers, and command behavior.
Expected outputs are package-local `artifacts/review_agent_a.md`,
`artifacts/verification_agent_a.md`, compact command evidence, and issue lists.
Write access is read-only unless a subagent is explicitly assigned a bounded fix
inside this package's intended write set.

## Scope

In scope:

- new `tools/agents/find-agents` helper;
- new `docs/agent-guidance-map.md`;
- root `AGENTS.md` pointer to the helper;
- `docs/work-packages/AGENTS.md` pre-edit instruction discovery rule;
- package-local prompt and evidence artifacts;
- docs/work-package catalog update.

Out of scope:

- kernel, science-contract, runner, watershed, or M-T3 implementation changes;
- adding new nested `AGENTS.md` files outside this package;
- changing existing package required-reading contents in historical packages;
- modifying unrelated dirty CQR or M-T3 work.

## Intended Write Set

- `tools/agents/find-agents`
- `docs/agent-guidance-map.md`
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260709-agent-instruction-discovery-index-tooling-001/**`

## Phase Plan

### Phase A - Scaffold And Baseline

1. Record current `AGENTS.md` inventory with `rg --files -g 'AGENTS.md'`.
2. Scaffold package directories, `package.md`, active prompt, and artifacts.
3. Record dirty-worktree boundaries and unrelated files to avoid.

### Phase B - Implement Discovery Surfaces

1. Add `tools/agents/find-agents` with:
   - `--all` to list all repository `AGENTS.md` files;
   - `--for <path> [<path> ...]` to list the applicable root-to-nearest
     instruction chain for each intended path;
   - `--json` for machine-readable package evidence.
2. Add `docs/agent-guidance-map.md` listing current agent instruction files,
   read triggers, and the fast command.
3. Add concise pointers to root and work-package `AGENTS.md`.
4. Update `docs/work-packages/README.md` for package discoverability.

### Phase C - Acceptance Tests

Run and record:

- `tools/agents/find-agents --all`;
- `tools/agents/find-agents --for docs/work-packages/README.md`;
- `tools/agents/find-agents --for crates/openwepp-runner/src/lib.rs tests/fixtures/example`;
- `tools/agents/find-agents --json --for tools/owcmp/owcmp`;
- an invalid-mode or no-argument check proving fail-fast usage.

### Phase D - Review, Verification, Closure

1. Dispatch a review subagent for governance/discoverability issues.
2. Dispatch an acceptance-test subagent for independent command behavior checks.
3. Disposition findings.
4. Run final local gates:
   - `git diff --check`
   - markdown-doc lint for touched docs
   - `bash -n tools/agents/find-agents` if shell, or `python -m py_compile` if
     Python implementation
   - the acceptance command set from Phase C
5. Finalize `artifacts/gate-results.md`, `artifacts/disposition.md`, and
   `artifacts/final-disposition.md`.

## Exit Criteria

- `tools/agents/find-agents --all` reports all current repo `AGENTS.md` files.
- `--for` reports root-to-nearest applicable chains for representative docs,
  crates, tests, fixtures, and tools paths.
- `--json` output is parseable JSON and names the same applicable chains as text
  mode.
- Root and work-package playbooks make the command discoverable before edits.
- Package evidence records subagent review and acceptance-test results.
- Docs lint and diff checks pass.
