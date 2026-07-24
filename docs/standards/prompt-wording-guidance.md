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
- Bind gate lifecycle, boundary assignment, escalation, and receipt currency to
  `docs/standards/testing-and-gate-strategy.md`. Require a pre-implementation
  intent plan and exact-diff terminal reconciliation. Name the conservative
  full commands only for critical, campaign, release, or rollback boundaries.
- Contract-first sequencing.
- Canonical `SC-*` authority requirements.
- Legacy baseline provenance requirement when migration applies.
- Explicit prohibition on surrogate, provisional, proxy, empirical stand-in, or
  heuristic process-physics substitutions in production code for all
  kernel/process-physics packages.
- Explicit direct production-path requirement for direct-kernel, publication, or
  cutover packages: the real downstream consumer must read the corrected path,
  and wrappers/adapters/skeleton/shadow paths cannot carry the closure claim.
- DC-ExecPlan autonomous closure expectation: no `HOLD` while source reading,
  implementation, contract/test work, or validation remains possible inside the
  declared envelope.
- Dual review and dual verification requirements where applicable.
- Autonomous execution expectation for the full assigned scope (no user
  intervention unless hard-blocked).
- For conservation-sensitive output/publication work: operand-lineage table,
  anti-tautology fixtures, explicit rejected formulas, independent
  reconstruction from produced outputs, real closure/magnitude audit, and
  metadata/schema alignment. State that one-sided bounds and exact
  self-consistency checks are supporting evidence only.

4a. Explicitly authorize AND require subagent spawning for delegated/heavy work
- Current OpenAI subagent tooling may require explicit user/session
  authorization before any spawn, even when repository governance requires
  delegated review or verification. Repository docs and package text cannot
  override that higher-precedence tool rule by themselves.
- Maintainers should include this standing authorization in the recurring
  openWEPP launch prompt/session instructions:
  `For openWEPP work-package execution, I explicitly authorize Codex to`
  `spawn/delegate to subagents whenever the active work package, AGENTS.md, or`
  `package governance requires or authorizes review, verification, comparator`
  `execution, or parallel agent work.`
- Package prompts still must include the phrase
  `explicitly authorizes subagent spawning/delegation`, name the authorized
  role(s), scope, expected compact outputs/artifacts, and whether the role is
  read-only or has a bounded package write set. This package-local wording is
  repo-governance evidence and lets agents use the standing session
  authorization without asking again.
- Wording such as `dispatch <role>` is not enough by itself; the explicit
  authorization phrase is required in the package prompt. If the standing
  user/session authorization is absent and tool policy blocks spawning, ask for
  one-time authorization or record the block and run the equivalent gate locally
  only when package governance allows local substitution.
- **Required, not optional, for selected heavy batch/closure/comparator work.**
  When a terminal plan, critical classification, campaign/release boundary, or
  transition fallback selects full workspace tests, broad Clippy/deny,
  comparator/parity suites, release gates, population/cohort batches, optional
  operator QA, or explicit metric-package coverage/CRAP, the
  prompt MUST *require* — not merely authorize — spawning the
  `comparator_suite_runner` subagent (gpt-5.3-codex-spark) for those runs, with an
  imperative directive (see the `Subagent requirement:` template line).
- The parent agent **must not** execute heavy batch/closure runs on its own
  premium model when the subagent is available. If the subagent is genuinely
  unavailable (tool-policy block or spawn failure), record that with command-level
  evidence as the justification before running locally.
- If no heavy gate is selected and no other subagents are required, state
  `Subagent requirement: none`.

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
- `DC closure (<if applicable>): close defect <id> end-to-end; do not hold while`
  `source reading, implementation, contract/test work, or validation remains`
  `possible inside the declared envelope; if HOLD is claimed, record a HOLD`
  `legitimacy audit naming the boundary, evidence, considered in-envelope`
  `correction route, and why it cannot close now.`
- `No surrogate physics (<if applicable>): production code must implement actual`
  `contract-backed or baseline-authoritative physics; surrogate/provisional/`
  `proxy/heuristic stand-ins are forbidden. Missing authority is a hold-for-`
  `authority boundary; known in-scope physics is an implementation obligation.`
- `Real consumer proof (<if applicable>): move the public/downstream consumer to`
  `the corrected path and prove wrappers, adapters, skeletons, shadow paths, and`
  `old compatibility paths are not carrying the closure claim.`
- `Conservation/output acceptance (<if applicable>): record operand lineage;`
  `separate plausible aliases in fixtures; reject known wrong formulas; run`
  `independent reconstruction plus real closure/magnitude audit; align`
  `metadata/schema; do not close on one-sided bounds or self-consistency.`
- `Subagent requirement: <none | REQUIRED: spawn comparator_suite_runner for all`
  `heavy batch/closure/comparator runs (cargo test --workspace, suites, gates,`
  `population batches); do NOT run them on the parent model unless the subagent is`
  `unavailable, in which case record command-level evidence. Standing user`
  `authorization for openWEPP subagent delegation is expected in the session.`
  `This prompt explicitly authorizes subagent spawning/delegation to <roles> for`
  `<scope>; outputs: compact metrics + log paths; write access:`
  `<read-only|bounded>.>`
- `Autonomy: execute package phases end-to-end and update required artifacts`
  `without requesting additional user direction unless hard-blocked.`
- `Outputs: update package artifacts/disposition for all completed phases.`

Phase-only exception template:
- `Execution mode: phase-only (exception).`
- `Phase: <A|B|C|D> only.`
- `Exception rationale: <why phase-only is required now>.`
- `Next prompt trigger: <condition that starts follow-on prompt>.`
