# Prompt Wording Guidance

Status: Active

Relocated from `AGENTS.md` by DOCOPT01. This document remains required, normative guidance before authoring execution prompts for kernel/science packages.

## Guidance
Use this wording standard when authoring execution prompts for kernel/science
packages. This reduces false-positive policy blocks without changing technical
scope.

1. Start with an explicit scope sentence
- State that the task is local repository engineering work.
- State that work is limited to flat-file reads/edits in the worktree.
- State that no external systems/network actions are required.

2. Set execution mode explicitly (default: package end-to-end)
- Default kickoff mode is `Execution mode: package-end-to-end`.
- In default mode, the prompt must instruct the agent to execute all package
  phases in `package.md` sequentially through disposition.
- Single-phase kickoff prompts are exception-only and must be explicitly marked
  `Execution mode: phase-only (exception)` with:
  - `Exception rationale: <why phase-only is required>`
  - `Next prompt trigger: <when/how follow-on prompt starts>`
- Do not use phase-only wording in standard kickoff prompts.

3. Use concrete path-scoped wording
- Name exact file paths and sections/functions to edit.
- Prefer short imperative language (`read`, `amend`, `add tests`, `record`).
- Avoid broad/open-ended wording that does not constrain scope.
- Add an explicit `Required reading` list before task instructions so agents can
  load orientation context deterministically.

4. Preserve mandatory technical gates in every prompt
- Contract-first sequencing.
- Canonical `SC-*` authority requirements.
- Legacy baseline provenance requirement when migration applies.
- Explicit prohibition on heuristic/proxy process-physics substitutions in
  production code for migration packages.
- Dual review and dual verification requirements where applicable.
- Autonomous execution expectation for the full assigned scope (no user
  intervention unless hard-blocked).

5. Required fallback when a false-positive block occurs
- Retry with a shorter prompt that includes only:
  - scope sentence,
  - single phase objective,
  - explicit file list.
- If blocked again, split further into micro-prompts (one file group each).
- Record the block event and resumed prompt shape in package artifacts.

6. Prompt template (copy/paste)
- `Scope: local repository science-contract/kernel migration task; flat-file`
  `reads/edits only; no external connectivity.`
- `Execution mode: package-end-to-end (default).`
- `Phase plan: execute all phases in package.md sequentially through`
  `disposition.`
- `Required reading (read before edits): <explicit path list>.`
- `Files: <explicit path list>.`
- `Task: execute package objective end-to-end for declared scope.`
- `Constraints: contract-first sequencing; canonical SC authority;`
  `baseline provenance (<if applicable>); typed guards; no silent defaults;`
  `no canonicalize-and-proceed for domain violations.`
- `Autonomy: execute package phases end-to-end and update required artifacts`
  `without requesting additional user direction unless hard-blocked.`
- `Outputs: update package artifacts/disposition for all completed phases.`

Phase-only exception template:
- `Execution mode: phase-only (exception).`
- `Phase: <A|B|C|D> only.`
- `Exception rationale: <why phase-only is required now>.`
- `Next prompt trigger: <condition that starts follow-on prompt>.`
