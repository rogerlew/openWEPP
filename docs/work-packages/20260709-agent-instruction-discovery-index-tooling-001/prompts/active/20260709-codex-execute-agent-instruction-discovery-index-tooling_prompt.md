# Execute Agent Instruction Discovery Index Tooling

Scope: local repository documentation/tooling task; flat-file reads/edits only
inside `/home/workdir/openWEPP`; no external connectivity required.

Execution mode: package-end-to-end.

Phase plan: execute all phases in
`docs/work-packages/20260709-agent-instruction-discovery-index-tooling-001/package.md`
sequentially through disposition.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260709-agent-instruction-discovery-index-tooling-001/package.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`

Conditional:

- `docs/work-packages/README.md` before catalog edits.

On-demand:

- current `AGENTS.md` files returned by `rg --files -g 'AGENTS.md'`.

Required-reading budget: `47701` bytes, `OK`; map:
`docs/work-packages/20260709-agent-instruction-discovery-index-tooling-001/artifacts/required-reading-map.md`.

Files:

- `tools/agents/find-agents`
- `docs/agent-guidance-map.md`
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260709-agent-instruction-discovery-index-tooling-001/**`

Task: implement the package objective end-to-end. Add the discovery helper,
guidance map, playbook pointers, package evidence, review, acceptance-test
verification, and final disposition.

Constraints:

- Do not touch unrelated dirty M-T3 or CQR-nightly work.
- No kernel, science-contract, runner, watershed, or physics changes.
- Keep root `AGENTS.md` concise.
- Use `rg --files` semantics for fast discovery; avoid slow recursive `find`
  over large package/artifact trees.

Subagent requirement: REQUIRED: dispatch review and acceptance-test subagents.
This prompt explicitly authorizes subagent spawning/delegation to review and
acceptance-test subagents for this package's docs/tooling changes. Outputs:
`artifacts/review_agent_a.md`, `artifacts/verification_agent_a.md`, compact
command evidence, and issue lists. Write access: read-only unless explicitly
assigned a bounded fix in this package's write set.

Autonomy: execute package phases end-to-end and update required artifacts without
requesting additional user direction unless hard-blocked.
