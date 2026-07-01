# Documentation agent

This directory’s documentation agent is the personified maintainer path for CLI and
end-user documentation in `usersum`.

## Scope

- Keep CLI command pages discoverable and current.
- Keep command examples aligned with actual release-binary behavior.
- Route any doc-level validation notes (manual run checks, run-root behavior,
  output expectations) into the relevant CLI page.

## Primary docs

- [CLI run index](cli-run-index.md)
- [openwepp-cli-hill.md](openwepp-cli-hill.md)
- [openwepp-cli-watershed.md](openwepp-cli-watershed.md)
- [openwepp-cli-totalwatsed3.md](openwepp-cli-totalwatsed3.md)
- [open_wepp_runner.md](open_wepp_runner.md)
- [openwepp-snowbench.md](openwepp-snowbench.md)

## Discovery

- If you need a single entrypoint, use
  [usersum/cli-run-index.md](cli-run-index.md).
- If you need architecture context for CLI behavior, use
  [README.md](../README.md) CLI documentation landing section.

## Standard handoff language

- Use this exact handoff target for end-user documentation edits:
  - `handoff: usersum/documentation-agent`
  - `target_path: usersum/documentation-agent.md`
- Scope for this handoff: all updates to end-user CLI pages under `usersum/`
  (including run examples, validation notes, and discoverability links).
