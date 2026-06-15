# CQR09 Kickoff Prompt

Work in `/home/workdir/openWEPP`.

Execute CQR09 end-to-end for
`docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/`.
Follow `package.md`, `docs/work-packages/AGENTS.md`,
`docs/standards/mechanical-refactor-authoring-guide.md`,
`docs/standards/code-quality-refactor-authoring-guide.md`,
`docs/specifications/science-contracts/AGENTS.md`, and ADR-0021.

Subagent authorization: this package does not require spawning/delegating to
subagents. The executing agent must complete equivalent local dual review and
dual verification artifacts.

Preserve public API, typed guards, error IDs, aliases, symbols, units, parser
compatibility, output formulas, float expression order, and science-contract
behavior. Stop at a declared hard blocker if a production edit would require a
behavior or authority change.
