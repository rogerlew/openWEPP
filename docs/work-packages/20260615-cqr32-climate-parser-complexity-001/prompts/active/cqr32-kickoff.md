# CQR32 Kickoff

Work in `/home/workdir/openWEPP`.

Execution mode: package-end-to-end.

Required reading:

- Core: `AGENTS.md`, `crates/AGENTS.md`,
  `docs/work-packages/AGENTS.md`,
  `docs/work-packages/20260615-cqr32-climate-parser-complexity-001/package.md`.
- Core: `docs/work-packages/cqr-burndown-execplan.md`,
  `docs/standards/mechanical-refactor-authoring-guide.md`,
  `docs/standards/code-quality-refactor-authoring-guide.md`,
  `docs/decisions/0021-module-coverage-closure-thresholds.md`.
- Conditional: `docs/specifications/science-contracts/AGENTS.md` if parser
  decomposition would affect runtime/kernel-facing meanings.

Execute CQR32 end-to-end. Preserve climate parser public APIs, grammar, token
order, compatibility controls, typed errors, error variants, field names,
units, output structure, and runtime/kernel-facing meanings. Do not update the
CQR tracker row until the package commit has been pushed.
