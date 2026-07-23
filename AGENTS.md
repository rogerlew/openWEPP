# AGENTS.md
> Coding agent guide for openWEPP.

## Authorship
**This document and all AGENTS.md documents are maintained by GitHub Copilot / Codex / Claude Code, which retain full authorship rights for all AGENTS.md content revisions. Agents may author and revise AGENTS.md documents when and where they see fit.** Revisions must preserve applicable user direction, package scope, review expectations, and higher-precedence governance.

## Purpose
- Global, high-signal onboarding map for agent work in openWEPP.
- Keep root guidance concise; place deep subsystem details in nested `AGENTS.md` files and `docs/`.
- Prefer progressive disclosure: read only the docs needed for the current task.

## Instruction Discovery
- Instruction precedence is nearest-to-workdir: global defaults -> repo root -> nested directories.
- When a nested `AGENTS.md` exists for files you are editing, treat it as the primary local playbook.
- Use root `AGENTS.md` for repository-wide invariants and routing only.
- Fast lookup: run `tools/agents/find-agents --for <write-path> [...]` before
  edits to list the applicable root-to-nearest instruction chain. Use
  `tools/agents/find-agents --all` for the full inventory and
  `docs/agent-guidance-map.md` for the current index.

## Local Python Environment
- Repo-local Python tooling should use `.venv/bin/python`.
- `.venv` is an untracked local environment; do not commit it.
- If pip is missing, run `.venv/bin/python -m ensurepip --upgrade` before installing packages.

## Core Directives
- `??` in a prompt means provide critical analysis only; do not implement code.
- Ask for clarification when requirements or debug context are ambiguous.
- Keep docs terse: Codex loads context in bulk and does not compress verbose guidance.
- Work packages are autonomous execution specs; front-load enough planning, context, evidence, review, verification, and gates for no-intervention execution.
- Do not split work into tiny diagnostic-only packages unless authority, safety, validation-cost, or write-set boundaries require it.
- Substantive implementation work must occur within an authorized `docs/work-packages/<id>/` scope, or as user-directed follow-on work from that package.
- If follow-on work extends beyond the current package objective/write set, evaluate closing the package and planning a new one first.
- For package-required subagent review, verification, comparator, or parallel
  work, follow `docs/work-packages/AGENTS.md` and the standing user
  authorization wording in `docs/standards/prompt-wording-guidance.md`.
- Producer-only, skeleton-only, counter-only, or shadow-only evidence cannot close
  endpoint, direct-path, publication, or cutover claims. The real downstream
  consumer must be proven to read the new path.
- Do not create or switch git branches unless the user explicitly asks.
- Keep required scaffold/intermediate commits local and push once per stable
  increment unless an explicit remote checkpoint is necessary.
- Do not manually dispatch TESTGATE while a run is queued or active; its stable
  concurrency identity permits one running and only the newest pending run.
- Exception: permanently queued records from the retired pre-pivot Omarchy
  runner are defunct historical metadata, not live queue occupancy. Do not
  block, cancel, or wait on them; preflight only current forest1-runner work.
- Forest1 is the trusted TESTGATE heavy runner. A receipt labeled
  `LOCAL_UNTRUSTED` is normal unsigned forest1 evidence, not a hold, failure,
  or reason to request GitHub-hosted heavy execution. Do not block any workflow
  solely on that label; use retained forest1 receipt/ledger evidence unless an
  explicit package requires an additional publication artifact.
- Do not add fallback wrappers that silently mask missing required dependencies; prefer explicit failures.
- Correctness over completion: do not mark work complete when known invariant, closure, or contract violations remain unresolved.
- Inherit the agentic governance posture from wepppy. openWEPP is the simulation engine, not a separate culture.

## Kernel and Science Authority
- For kernel-affecting work, including runtime projection that controls kernel branches, read `docs/specifications/science-contracts/AGENTS.md` before edits.
- For work-package preparation, execution, review, verification, or closure, read `docs/work-packages/AGENTS.md`.
- For prompt wording and reusable standards, read `docs/standards/AGENTS.md` plus the referenced standard.
- Canonical process contract authority lives in `docs/specifications/science-contracts/contracts/SC-<DOMAIN>-<NNN>.md`.
- Work-package artifacts are evidence, not authority replacement.
- Do not implement provisional, surrogate, or heuristic process-physics math in production kernel/runtime publication paths.
- For legacy migration, implementation target is baseline-authoritative physics migration from `/workdir/wepp-forest_260430_baseline`, not behavioral approximation.

## Project Role
openWEPP is the Rust simulation engine. openWEPP owns its architecture and science-contract authority for openWEPP behavior. wepppy owns orchestration, GIS, climate, and run state. Do not duplicate wepppy concerns in this repo.

## Strategy and Provenance Model
- openWEPP is explicitly not a clean-room rewrite.
- Architecture is implemented first: typed state, module boundaries, orchestration flow, and contracts.
- Science contracts are authored top-down from `references/50201000`, literature invariants, physical/common-sense invariants, and static legacy code analysis.
- Agents may read legacy F90 source and existing science contracts directly for provenance mapping and static inspection.
- Canonical legacy provenance/comparator baseline is `/workdir/wepp-forest_260430_baseline` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, except where a contract or decision explicitly supersedes it.
- `/workdir/wepp-forest` HEAD remains valid for exploratory discovery; normative provenance defaults to the pinned baseline unless explicitly justified.
- Binary pass serialization authority may also reference `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` and `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md` with commit SHA provenance.

## Validation Gates
- `docs/standards/testing-and-gate-strategy.md` is the canonical authority for
  gate selection, lifecycle timing, campaign deferral, evidence reuse, and
  escalation. Every package must declare implementation intent before edits
  (in `package.md` during transition, then as a machine intent plan after
  cutover) and reconcile the exact terminal diff before disposition.
- Execute every increment gate selected by the authenticated
  terminal plan. Critical changes still require immediate campaign-strength
  full regression and global CRAP. Campaign closure and release qualification
  retain exact-head full-workspace and global-quality obligations.
- Kernel work always retains touched contract invariants, applicable A0/A1/A3
  authority gates, typed guards, and closure checks. Legacy comparator deltas
  use confidence tiers. Conservation-sensitive output work retains independent
  operand reconstruction and real closure evidence per
  `docs/standards/kernel-work-package-preparation.md`.
- Use focused, quick, and domain profiles for edit loops as described in
  `docs/standards/local-ci-gate-selection.md`. Fall back to `cargo test` only
  for libtest-specific behavior or an explicitly required legacy harness.

## Error Handling and Numerics
- No broad `Result<_, Box<dyn Error>>` swallowing in production paths; use typed error enums per crate.
- No `.unwrap()` or `.expect()` in production paths. Test code may use them with intent.
- Do not silently mask numerical edge cases with defaults; surface them as typed errors and let the orchestrator decide.
- `unsafe` blocks require a `// SAFETY: ...` comment explaining the invariant.
- Do not canonicalize-and-proceed on kernel domain violations unless a canonical `SC-*` contract explicitly authorizes bounded normalization.
- Bounded canonicalization requires explicit threshold, units, provenance, tests, and evidence.

## Documentation Map
- Work packages: `docs/work-packages/AGENTS.md`, `docs/work-packages/README.md`, `docs/codex_exec_plans.md`, `docs/defect_closure_execplans.md`.
- Agent instruction discovery: `docs/agent-guidance-map.md`,
  `tools/agents/find-agents`.
- Array-native burn-down ExecPlans: R4 hydrology direct paths in `docs/work-packages/r4-burndown-execplan.md`; R5 full OFE-day direct path in `docs/work-packages/r5-burndown-execplan.md`.
- Science contracts: `docs/specifications/science-contracts/AGENTS.md`, `docs/specifications/science-contract-authoring-procedure.md`, `docs/specifications/science-contracts/kernel-process-contract-profile.md`, `docs/specifications/science-contracts/index.md`.
- Standards and prompt wording: `docs/standards/AGENTS.md`, `docs/standards/kernel-work-package-preparation.md`, `docs/standards/prompt-wording-guidance.md`, `docs/standards/mechanical-refactor-authoring-guide.md`, `docs/standards/local-ci-gate-selection.md`.
- Canonical gate lifecycle: `docs/standards/testing-and-gate-strategy.md`.
- Local CI timing tooling: `tools/local_ci/README.md`.
- Adjudicated CRAP gate: `tools/release/README.md`.
- Rust crates: `crates/AGENTS.md`.
- Tests: `tests/AGENTS.md`.
- Numerics: `docs/numerics/README.md`.
- Subprocess orchestration: `docs/decisions/0004-subprocess-hillslope-orchestration.md`.
- Architecture/science-contract strategy: `docs/decisions/0011-architecture-first-top-down-science-contracts.md`.
- Legacy baseline anchor: `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`.
- Comparator distrust / flag-not-target posture: `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`.

## Subsystem Maps
- Nearest `AGENTS.md` wins. See the documentation map above for local playbooks; add new subsystem entries there rather than duplicating detailed guidance in root.

## Security Guardrails
- Never commit secrets or tokens.
- Preserve validation, typed guards, fail-closed behavior, and serialization safeguards.
- For edits touching external-authority suite posture, cohort fixtures, or required-case bindings, run source-level anti-evasion guards before disposition: `bash tools/release/check_authority_suite_antievasion.sh` and `cargo nextest run --test auth11_required_suite_obligation_guards_contract`.

## Agent Feedback Loop
- Treat avoidable friction as diagnostic signal about codebase, docs, tooling, or task framing.
- Surface concise, actionable feedback when it materially improves clarity or agent-facing interfaces.
- Distinguish confirmed defects, local painpoints, and speculative improvements.

## Truthfulness
- Match verbs to evidence. Label evidence class (`Static:` vs `Ran:`) at the top of reviews and audits.
- A validator (`cargo check`, `cargo build`) is not the workflow (`cargo nextest run`, comparator harness run).
- When skipping execution, say so plainly.

## Root Exclusions
- Do not place long tutorials, prompt templates, package procedures, or contract authoring manuals in this file.
- Do not duplicate subsystem instructions already maintained in nested `AGENTS.md` files.
- Move growing sections to canonical docs and leave short binding pointers here.

## If Blocked
- Check the nearest subsystem `AGENTS.md`, then module README, contract, package, and tests.
- Reuse existing patterns from adjacent code before introducing new abstractions.
- Ask a human when requirements are unclear or an external dependency blocks progress.

## Root Size Policy
- Keep this file within roughly 100-160 lines.
- If a section grows beyond quick onboarding value, move detail to a canonical doc and leave a pointer.
- Prefer stable links over copied prose; re-check line count after major edits.
