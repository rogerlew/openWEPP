# Review Agent A

Review type: static/tool behavior review with read-only command checks.

Verdict: PASS with one Low finding, accepted and fixed.

## Findings

### Low - Kickoff Prompt Missing Required-Reading Budget

Finding: the active kickoff prompt tiered required reading but omitted the
required `Required-reading budget:` line and required-reading map pointer.

Evidence cited by reviewer:

- `docs/work-packages/20260709-agent-instruction-discovery-index-tooling-001/prompts/active/20260709-codex-execute-agent-instruction-discovery-index-tooling_prompt.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/README.md`

Disposition: accepted.

Fix: added `Required-reading budget: 47701 bytes, OK` and the package-local
required-reading map path to the active kickoff prompt.

## Checks Performed By Reviewer

Ran: `tools/agents/find-agents --all`; matched current
`rg --files -g 'AGENTS.md'` inventory.

Ran: representative `--for` checks for docs, crates, tests/fixtures,
`tools/owcmp`, science contracts, and standards. Chains were root-to-nearest and
correct.

Ran: JSON mode, no-argument fail-fast, and outside-repo path rejection. JSON was
parseable; invalid usage and outside paths exited with code `2`.

Static: the helper avoids Python full-tree traversal and uses
`git ls-files -co --exclude-standard` for tracked/unignored `AGENTS.md`
discovery. Root/work-package pointers are concise and discoverable. Subagent
authorization, gates, and closure criteria are present in the package/prompt.
