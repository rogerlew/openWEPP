# Required Reading Map

Status: `complete`

## Instruction Discovery

Ran:

```text
tools/agents/find-agents --for docs/ROADMAP.md \
  docs/work-packages/README.md \
  docs/work-packages/20260714-roadmap-prospective-cleanup-001/package.md
```

Applicable chains:

- `docs/ROADMAP.md`: `AGENTS.md`
- `docs/work-packages/README.md`: `AGENTS.md` ->
  `docs/work-packages/AGENTS.md`
- package-local files: `AGENTS.md` -> `docs/work-packages/AGENTS.md`
- `docs/backlog/TRACKER.md` and the hydrograph-resolved sediment note:
  `AGENTS.md`

## Core Reading

| Path | Purpose |
| --- | --- |
| `AGENTS.md` | Repository-wide governance and documentation routing. |
| `docs/work-packages/AGENTS.md` | Package execution, evidence, review, verification, and closure rules. |
| `docs/standards/AGENTS.md` | Documentation-standard routing. |
| `docs/standards/prompt-wording-guidance.md` | Delegation and autonomous-execution wording. |
| `docs/codex_exec_plans.md` | Living ExecPlan and dual-review requirements. |
| `docs/ROADMAP.md` | Complete pre-rewrite planning surface to classify. |

Intake byte count: `191590` bytes for the six files above. Package-local files
add less than `15000` bytes. Total required reading remains below the
`400000`-byte threshold.

The relevant package-catalog regions are inspected on demand to confirm that
removed history remains discoverable; the entire catalog is not required
reading because it is itself a large historical register.

Review-finding remediation additionally required the relevant status and
remaining-scope sections of `docs/backlog/TRACKER.md`,
`docs/backlog/20260704-hydrograph-resolved-sediment-and-routing.md`,
`SC-SED-001#GAP-SED-008..009`, and the W11/HB-06 terminal package records. This
bounded on-demand reading did not approach the 400,000-byte threshold.
