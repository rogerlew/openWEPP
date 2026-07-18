# docs/standards/AGENTS.md
> Agent playbook for openWEPP standards and reusable guidance.

## Authorship
**This document and all AGENTS.md documents are maintained by GitHub Copilot / Codex / Claude Code, which retain full authorship rights for all AGENTS.md content revisions. Agents may author and revise AGENTS.md documents when and where they see fit.** Revisions must preserve applicable user direction, package scope, review expectations, and higher-precedence governance.

## Mission Snapshot
- Own reusable agent-facing standards, procedures, prompt wording, and refactor guidance.
- Keep root `AGENTS.md` concise by relocating long procedural guidance here.
- Preserve normative pointers so moved obligations remain discoverable.
- Keep standards task-oriented, terse, and aligned with package/contract governance.

## Primary Assets / Key Files
- `docs/standards/kernel-work-package-preparation.md` — required kernel work-package preparation procedure.
- `docs/standards/prompt-wording-guidance.md` — required wording standard for kernel/science execution prompts.
- `docs/standards/mechanical-refactor-authoring-guide.md` — mechanical refactor authoring and closure loop.
- `docs/standards/testing-and-gate-strategy.md` — canonical gate selection,
  lifecycle, receipt, campaign, coverage/CRAP cadence, and assurance-impact
  authority.
- Root `AGENTS.md` — concise router that should point here rather than duplicate long procedures.

## Standard Workflow
1. Identify which workflow the standard governs and whether it is root-wide, package-specific, or contract-specific.
2. Keep normative obligations in one canonical standard; update root or local `AGENTS.md` files with short binding pointers only.
3. If a standard changes package execution, update `docs/work-packages/AGENTS.md` and relevant templates/prompts in the same change.
4. If a standard changes contract authority, update `docs/specifications/science-contracts/AGENTS.md` and the canonical procedure/profile docs as needed.
5. Delegate gate frequency and lifecycle to the testing/gate strategy. Retain
   specialized correctness and evidence obligations in their owning standards.
6. Use concrete path-scoped language and runnable checks; avoid broad inspirational prose.

## Prompt Wording Maintenance
- Kernel/science kickoff prompts should start with an explicit local-repository scope sentence.
- Default execution mode is package-end-to-end; phase-only prompts are exception-only and must state rationale plus next trigger.
- Required reading lists should name exact files.
- Prompts must preserve contract-first sequencing, canonical `SC-*` authority, baseline provenance when applicable, typed guards, no silent defaults, and autonomous execution expectations.
- If a false-positive block occurs, record the shorter resumed prompt shape in package artifacts.

## Validation Checklist
- Confirm referenced paths exist after any move.
- For docs-only changes, run or record doc-path integrity checks when pointers change.
- For standards that mandate code/test gates, ensure affected package prompts and artifact templates name the current commands.
- Keep ASCII unless the edited file already has justified non-ASCII content.

## Common Pitfalls
- Do not duplicate full procedures into root `AGENTS.md`.
- Do not make standards so verbose that agents must load tutorial-length guidance for routine work.
- Do not update prompt templates without checking package process docs for consistency.

## References
- Root guidance: `AGENTS.md`.
- Work packages: `docs/work-packages/AGENTS.md`.
- Science contracts: `docs/specifications/science-contracts/AGENTS.md`.
