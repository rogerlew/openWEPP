# Agent Guidance Map

Status: Active
Last updated: 2026-07-09

This map makes repository instruction discovery cheap and explicit. Before
editing, run:

```bash
tools/agents/find-agents --for <path> [<path> ...]
```

For a full inventory, run:

```bash
tools/agents/find-agents --all
```

For package artifacts or automation, add `--json`.

## Instruction Files

| Path | Read when |
|---|---|
| `AGENTS.md` | Always. Root governance, validation gates, science authority routing, dirty-worktree rules. |
| `crates/AGENTS.md` | Before editing Rust crates under `crates/`. |
| `docs/specifications/science-contracts/AGENTS.md` | Before editing science contracts or kernel-affecting contract authority. |
| `docs/standards/AGENTS.md` | Before editing reusable standards, prompt wording, or process guidance. |
| `docs/work-packages/AGENTS.md` | Before scaffolding, executing, reviewing, verifying, or closing work packages. |
| `tests/AGENTS.md` | Before editing tests under `tests/`. |
| `tests/fixtures/AGENTS.md` | Before editing committed fixtures under `tests/fixtures/`. |
| `tools/owcmp/AGENTS.md` | Before editing comparator tooling under `tools/owcmp/`. |

## Examples

```bash
tools/agents/find-agents --for crates/openwepp-runner/src/lib.rs
```

Expected chain:

```text
AGENTS.md
crates/AGENTS.md
```

```bash
tools/agents/find-agents --for docs/work-packages/example/package.md
```

Expected chain:

```text
AGENTS.md
docs/work-packages/AGENTS.md
```

```bash
tools/agents/find-agents --for tests/fixtures/example
```

Expected chain:

```text
AGENTS.md
tests/AGENTS.md
tests/fixtures/AGENTS.md
```

## Package Evidence

Work packages should record the command output in
`artifacts/required-reading-map.md` or equivalent package-local intake evidence
before production edits. The tool reports paths from root to nearest local
instruction file, matching repository instruction precedence.
