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

3a. Tier required-reading to reduce context burn
- Structure required-reading in three tiers:
  - `Core` (always read before edits),
  - `Conditional` (read only when package scope triggers it),
  - `On-demand` (reference set for touched mechanisms only).
- Keep governance/authority discoverable without requiring pre-read of every
  large authority document.
- Require a package-local authority map artifact (`artifacts/required-reading-map.md`)
  and point to it from the kickoff prompt.
- Canonical starter template for this artifact:
  `docs/prompt_templates/required-reading-map-template.md`.
- For mechanical-only refactor prompts, do not force full science-contract
  authoring/profile documents as pre-edit reads unless the refactor touches
  contract/kernel authority.

3b. Record required-reading budget in kickoff prompts
- Kickoff prompts must record local required-reading byte total and threshold
  disposition using the canonical thresholds in
  `docs/standards/kernel-work-package-preparation.md`.
- For `REQUIRES-JUSTIFICATION`, include a short rationale for each heavy
  required pre-read and why it cannot move to `On-demand`.

4. Preserve mandatory technical gates in every prompt
- Contract-first sequencing.
- Canonical `SC-*` authority requirements.
- Legacy baseline provenance requirement when migration applies.
- Explicit prohibition on heuristic/proxy process-physics substitutions in
  production code for migration packages.
- Dual review and dual verification requirements where applicable.
- Autonomous execution expectation for the full assigned scope (no user
  intervention unless hard-blocked).

4a. Explicitly authorize AND require subagent spawning for delegated/heavy work
- If a prompt expects delegated review, verification, comparator execution, or
  other subagent work, it must include the phrase
  `explicitly authorizes subagent spawning/delegation`, name the authorized
  role(s), scope, expected compact outputs/artifacts, and whether the role is
  read-only or has a bounded package write set.
- Wording such as `dispatch <role>` is not enough by itself; the explicit
  authorization phrase is required so subagent tool policy ("spawn only when
  explicitly requested by the user") is satisfied — the kickoff prompt IS that
  explicit request.
- **Required, not optional, for heavy batch/closure/comparator work.** When a
  package runs the full closure loop (`cargo test --workspace`, clippy, deny),
  comparator/parity suites, release gates, or population/cohort batches, the
  prompt MUST *require* — not merely authorize — spawning the
  `comparator_suite_runner` subagent (gpt-5.3-codex-spark) for those runs, with an
  imperative directive (see the `Subagent requirement:` template line).
- The parent agent **must not** execute heavy batch/closure runs on its own
  premium model when the subagent is available. If the subagent is genuinely
  unavailable (tool-policy block or spawn failure), record that with command-level
  evidence as the justification before running locally.
- If no subagents are required, state `Subagent requirement: none`.

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
- `Required reading (read before edits):`
  `Core: <explicit path list>.`
  `Conditional: <path list + trigger>.`
  `On-demand: <path list for touched mechanisms only>.`
- `Required-reading budget: <local_bytes_total>,`
  `<OK|WARN|REQUIRES-JUSTIFICATION>; map: artifacts/required-reading-map.md.`
- `Files: <explicit path list>.`
- `Task: execute package objective end-to-end for declared scope.`
- `Constraints: contract-first sequencing; canonical SC authority;`
  `baseline provenance (<if applicable>); typed guards; no silent defaults;`
  `no canonicalize-and-proceed for domain violations.`
- `Subagent requirement: <none | REQUIRED: spawn comparator_suite_runner for all`
  `heavy batch/closure/comparator runs (cargo test --workspace, suites, gates,`
  `population batches); do NOT run them on the parent model unless the subagent is`
  `unavailable, in which case record command-level evidence. This prompt`
  `explicitly authorizes subagent spawning/delegation to <roles> for <scope>;`
  `outputs: compact metrics + log paths; write access: <read-only|bounded>.>`
- `Autonomy: execute package phases end-to-end and update required artifacts`
  `without requesting additional user direction unless hard-blocked.`
- `Outputs: update package artifacts/disposition for all completed phases.`

Phase-only exception template:
- `Execution mode: phase-only (exception).`
- `Phase: <A|B|C|D> only.`
- `Exception rationale: <why phase-only is required now>.`
- `Next prompt trigger: <condition that starts follow-on prompt>.`
