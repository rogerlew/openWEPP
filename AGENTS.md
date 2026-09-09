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
- `??` means critical analysis only; no implementation.
- Work within authorized packages or user-directed follow-on scope; see
  docs/work-packages/AGENTS.md for execution, independent closure and handoff.
- Keep one maintained `package.md` per work-package. The common guide owns
  consequence-based review counts and reviewer-owned fix verification; no
  universal separate handoff, artifact checklist, or fresh verifier wave.
- Continue the full authorized checkpoint while safe in-scope work remains.
  Correctness takes precedence: unresolved invariants/acceptance cannot close.
- Preserve unrelated dirty/staged work. Do not create/switch branches without
  explicit user direction. Keep scoped commits local unless pushing is authorized.
- TESTGATE and the gate planner are frozen historical tooling. Do not dispatch,
  repair, extend or replace them with prospective authority machinery.
- No silent dependency fallbacks. One canonical numerical solver per physical
  regime; nonconvergence follows its canonical adaptive response or typed failure.
  Production numerical solvers must not accrete historical fallback chains.
  Follow docs/standards/numerical-solver-architecture.md and ADR-0044 for solver
  work; superseded paths are deleted, quarantined historical chains not extended.
- openWEPP inherits wepppy's agentic governance posture.

## Kernel and Science Authority
- For kernel-affecting work, including runtime projection that controls kernel branches, read `docs/specifications/science-contracts/AGENTS.md` before edits.
- For work-package preparation, execution, review, verification, or closure, read `docs/work-packages/AGENTS.md`.
- For prompt wording and reusable standards, read `docs/standards/AGENTS.md` plus the referenced standard.
- Canonical process contract authority lives in `docs/specifications/science-contracts/contracts/SC-<DOMAIN>-<NNN>.md`.
- Read affected canonical sections and their scientific dependencies, including
  in single-file contracts; links alone do not require full recursive onboarding.
  All applicable authority remains binding. Expand reading when impact is unclear.
- Work-package artifacts are evidence, not authority replacement; authoritative science remains an implementation obligation when calibration data are limited, with claims governed by ADR-0042 and `docs/work-packages/AGENTS.md`.
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
  validation requirements, lifecycle timing, campaign deferral, evidence reuse,
  and escalation. Every package declares implementation intent before edits and
  reconciles the exact terminal diff before disposition. Agents select and
  execute applicable validation directly; no validation-planning executable is
  prospective.
- Run every applicable increment requirement directly. Unknown or ambiguous
  production impact receives documented conservative escalation or authority
  clarification and is never silently narrowed. Critical changes retain
  immediate campaign-strength full correctness regression; campaign closure
  and release qualification retain exact-head full-workspace correctness.
- Coverage/CRAP is optional observational QA under ADR-0041 except for an
  explicitly authorized CQR/module-test-enhancement package's declared metrics.
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
- Agent instruction discovery: `docs/agent-guidance-map.md`, `tools/agents/find-agents`.
- Array-native burn-down ExecPlans: R4 hydrology direct paths in `docs/work-packages/r4-burndown-execplan.md`; R5 full OFE-day direct path in `docs/work-packages/r5-burndown-execplan.md`.
- Science contracts: `docs/specifications/science-contracts/AGENTS.md`, `docs/specifications/science-contract-authoring-procedure.md`, `docs/specifications/science-contracts/kernel-process-contract-profile.md`, `docs/specifications/science-contracts/index.md`.
- Standards: `docs/standards/AGENTS.md`, `docs/standards/numerical-solver-architecture.md`, `docs/standards/kernel-work-package-preparation.md`, `docs/standards/prompt-wording-guidance.md`.
- Canonical gate lifecycle: `docs/standards/testing-and-gate-strategy.md`.
- Local CI timing tooling: `tools/local_ci/README.md`.
- Adjudicated CRAP gate: `tools/release/README.md`.
- Rust crates: `crates/AGENTS.md`.
- Tests: `tests/AGENTS.md`.
- Numerics and key ADRs: `docs/numerics/README.md`; ADR-0004 (subprocess), ADR-0011 (architecture), ADR-0012 (baseline), ADR-0017 (comparator), and ADR-0044 (solver anti-accretion).

## Security Guardrails
- Never commit secrets or tokens.
- Preserve validation, typed guards, fail-closed behavior, and serialization safeguards.
- For edits touching external-authority suite posture, cohort fixtures, or required-case bindings, run source-level anti-evasion guards before disposition: `bash tools/release/check_authority_suite_antievasion.sh` and `cargo nextest run --test auth11_required_suite_obligation_guards_contract`.

## Truthfulness
- Match verbs to evidence. Label evidence class (`Static:` vs `Ran:`) at the top of reviews and audits.
- A validator (`cargo check`, `cargo build`) is not the workflow (`cargo nextest run`, comparator harness run).
- When skipping execution, say so plainly.

## Guidance maintenance
Keep root a concise invariant/router. Procedures belong in their task-owning
guides. Find nearest instructions, then targeted module/contract/package evidence
when blocked. Record confirmed tooling friction concisely. Never turn reading
targets into permission to omit authority or stop authorized work.
