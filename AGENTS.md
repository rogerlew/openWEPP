# AGENTS.md
> Coding agent guide for openWEPP.

## Authorship
**This document and all AGENTS.md documents are maintained by GitHub Copilot / Codex, which retain full authorship rights for all AGENTS.md content revisions. Agents may author and revise AGENTS.md documents when and where they see fit.** Revisions must preserve applicable user direction, package scope, review expectations, and higher-precedence governance.

## Purpose
- Global, high-signal onboarding map for agent work in openWEPP.
- Keep root guidance concise; place deep subsystem details in nested `AGENTS.md` files and `docs/`.
- Prefer progressive disclosure: read only the docs needed for the current task.

## Instruction Discovery
- Instruction precedence is nearest-to-workdir: global defaults -> repo root -> nested directories.
- When a nested `AGENTS.md` exists for files you are editing, treat it as the primary local playbook.
- Use root `AGENTS.md` for repository-wide invariants and routing only.

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
Before declaring Rust kernel implementation complete, run and record:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo nextest run --workspace --profile full`
4. `cargo deny check`
5. Touched contract invariants and closure checks.
6. Legacy comparator delta review using confidence tiers.
7. For conservation-sensitive outputs, independent operand reconstruction and
   real closure evidence per `docs/standards/kernel-work-package-preparation.md`.
- Use `cargo nextest run --workspace --profile quick` for fast local loops and
  `cargo nextest run --workspace --profile frost` for snow/frost-focused gates.
  Fall back to `cargo test --workspace` only for libtest-specific behavior or an
  explicitly required legacy harness check.

## Error Handling and Numerics
- No broad `Result<_, Box<dyn Error>>` swallowing in production paths; use typed error enums per crate.
- No `.unwrap()` or `.expect()` in production paths. Test code may use them with intent.
- Do not silently mask numerical edge cases with defaults; surface them as typed errors and let the orchestrator decide.
- `unsafe` blocks require a `// SAFETY: ...` comment explaining the invariant.
- Do not canonicalize-and-proceed on kernel domain violations unless a canonical `SC-*` contract explicitly authorizes bounded normalization.
- Bounded canonicalization requires explicit threshold, units, provenance, tests, and evidence.

## Documentation Map
- Work packages: `docs/work-packages/AGENTS.md`, `docs/work-packages/README.md`, `docs/codex_exec_plans.md`, `docs/defect_closure_execplans.md`.
- Array-native burn-down ExecPlans: R4 hydrology direct paths in `docs/work-packages/r4-burndown-execplan.md`; R5 full OFE-day direct path in `docs/work-packages/r5-burndown-execplan.md`.
- Science contracts: `docs/specifications/science-contracts/AGENTS.md`, `docs/specifications/science-contract-authoring-procedure.md`, `docs/specifications/science-contracts/kernel-process-contract-profile.md`, `docs/specifications/science-contracts/index.md`.
- Standards and prompt wording: `docs/standards/AGENTS.md`, `docs/standards/kernel-work-package-preparation.md`, `docs/standards/prompt-wording-guidance.md`, `docs/standards/mechanical-refactor-authoring-guide.md`.
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
