# Acceptance Test

Ran: focused local acceptance commands after implementation.

## Inventory

Command:

```text
tools/agents/find-agents --all
```

Exit: `0`

Output:

```text
AGENTS.md
crates/AGENTS.md
docs/specifications/science-contracts/AGENTS.md
docs/standards/AGENTS.md
docs/work-packages/AGENTS.md
tests/AGENTS.md
tests/fixtures/AGENTS.md
tools/owcmp/AGENTS.md
```

Verdict: PASS. Output matches current repo inventory.

## Work-Package Path

Command:

```text
tools/agents/find-agents --for docs/work-packages/README.md
```

Exit: `0`

Output:

```text
docs/work-packages/README.md (docs/work-packages/README.md):
  AGENTS.md
  docs/work-packages/AGENTS.md
```

Verdict: PASS.

## Crate And Fixture Paths

Command:

```text
tools/agents/find-agents --for crates/openwepp-runner/src/lib.rs tests/fixtures/example
```

Exit: `0`

Output:

```text
crates/openwepp-runner/src/lib.rs (crates/openwepp-runner/src/lib.rs):
  AGENTS.md
  crates/AGENTS.md

tests/fixtures/example (tests/fixtures/example):
  AGENTS.md
  tests/AGENTS.md
  tests/fixtures/AGENTS.md
```

Verdict: PASS.

## JSON Mode

Command:

```text
tools/agents/find-agents --json --for tools/owcmp/owcmp
```

Exit: `0`

Output:

```json
{
  "agents": [
    "AGENTS.md",
    "crates/AGENTS.md",
    "docs/specifications/science-contracts/AGENTS.md",
    "docs/standards/AGENTS.md",
    "docs/work-packages/AGENTS.md",
    "tests/AGENTS.md",
    "tests/fixtures/AGENTS.md",
    "tools/owcmp/AGENTS.md"
  ],
  "queries": [
    {
      "applicable": [
        "AGENTS.md",
        "tools/owcmp/AGENTS.md"
      ],
      "normalized_path": "tools/owcmp/owcmp",
      "path": "tools/owcmp/owcmp"
    }
  ],
  "repo_root": "/home/workdir/openWEPP"
}
```

Verdict: PASS.

## Usage Failure

Command:

```text
tools/agents/find-agents
```

Exit: `2`

Output:

```text
usage: find-agents [-h] [--all] [--for PATH [PATH ...]] [--json]
find-agents: error: specify --all and/or --for PATH...
```

Verdict: PASS. No-argument usage fails fast.

## Syntax Check

Command:

```text
python3 -m py_compile tools/agents/find-agents
```

Exit: `0`

Verdict: PASS. Generated local `tools/agents/__pycache__` was removed after the
check.
