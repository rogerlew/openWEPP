# Implementation

Static: package write set is docs/tooling only. No kernel, science-contract,
runner, watershed, M-T3, or CQR-nightly files are in scope.

Implemented:

- Added `tools/agents/find-agents`, an executable Python helper with no
  third-party dependencies.
- Added `docs/agent-guidance-map.md`, a committed instruction-file index with
  examples.
- Updated root `AGENTS.md` with fast lookup guidance and a documentation-map
  pointer.
- Updated `docs/work-packages/AGENTS.md` so work-package execution runs
  instruction discovery before edits and records the result in intake evidence.
- Updated `docs/work-packages/README.md` with reusable guidance and the active
  package pointer.

Tool behavior:

- `--all` lists tracked/unignored repository `AGENTS.md` files using
  `git ls-files -co --exclude-standard -- AGENTS.md */AGENTS.md`.
- `--for <path>...` normalizes each path inside the repo and prints the
  applicable root-to-nearest instruction chain.
- `--json` emits parseable JSON with the full inventory and query chains.
- no arguments fail fast with usage and exit code `2`.

Instruction-discovery intake:

Ran:

```text
tools/agents/find-agents --for tools/agents/find-agents docs/agent-guidance-map.md AGENTS.md docs/work-packages/AGENTS.md docs/work-packages/README.md docs/work-packages/20260709-agent-instruction-discovery-index-tooling-001/package.md
```

Result:

```text
tools/agents/find-agents (tools/agents/find-agents):
  AGENTS.md

docs/agent-guidance-map.md (docs/agent-guidance-map.md):
  AGENTS.md

AGENTS.md (AGENTS.md):
  AGENTS.md

docs/work-packages/AGENTS.md (docs/work-packages/AGENTS.md):
  AGENTS.md
  docs/work-packages/AGENTS.md

docs/work-packages/README.md (docs/work-packages/README.md):
  AGENTS.md
  docs/work-packages/AGENTS.md

docs/work-packages/20260709-agent-instruction-discovery-index-tooling-001/package.md (docs/work-packages/20260709-agent-instruction-discovery-index-tooling-001/package.md):
  AGENTS.md
  docs/work-packages/AGENTS.md
```

Line-count check:

Ran:

```text
wc -l AGENTS.md docs/work-packages/AGENTS.md tools/agents/find-agents docs/agent-guidance-map.md docs/work-packages/20260709-agent-instruction-discovery-index-tooling-001/package.md
```

Result:

```text
  138 AGENTS.md
  251 docs/work-packages/AGENTS.md
  181 tools/agents/find-agents
   75 docs/agent-guidance-map.md
  120 docs/work-packages/20260709-agent-instruction-discovery-index-tooling-001/package.md
  765 total
```
