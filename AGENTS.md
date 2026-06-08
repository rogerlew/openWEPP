# AGENTS.md
> Coding agent guide for openWEPP.

## Authorship
**This document and all AGENTS.md documents are maintained by GitHub Copilot / Codex, which retain full authorship rights for all AGENTS.md content revisions. Agents may author and revise AGENTS.md documents when and where they see fit.**

## Purpose
- Global, high-signal onboarding map for agent work in openWEPP.
- Keep root guidance concise; place deep details in nested AGENTS.md files and `docs/`.
- Prefer progressive disclosure: read only the docs needed for the current task.

## Instruction Discovery
- Instruction precedence is nearest-to-workdir: global defaults -> repo root -> nested directories.
- When a nested AGENTS.md exists for files you are editing, treat it as the primary local playbook.

## Local Python Environment
- Repo-local Python tooling should use `.venv/bin/python`.
- `.venv` is an untracked local environment; do not commit it.
- If pip is missing, run `.venv/bin/python -m ensurepip --upgrade` before installing packages.

## Core Directives
- `??` in a prompt means provide critical analysis only; do not implement code.
- Ask for clarification when requirements or debug context are ambiguous.
- Keep docs terse: Codex loads context in bulk and does not compress verbose guidance.
- Work packages are autonomous execution specs: front-load planning and context
  so an agent can execute from kickoff through disposition without user
  intervention.
- Work packages must be right-sized to amortize their required scaffolding,
  evidence, review, verification, and gate overhead. Do not split work into
  progressively smaller diagnostic-only packages that merely advance a `HOLD`
  by one symbol, one row, or one observation unless a hard authority, safety,
  validation-cost, or write-set boundary requires it. Prefer one coherent
  package that carries a related seam through contracts, tests, implementation
  or evidence generation, classification, disposition, and the immediately
  actionable follow-through that can be completed autonomously.
- Work-package authoring must follow `docs/codex_exec_plans.md` for autonomy,
  self-containment, and observable validation expectations.
- Defect-closure work must follow ADR-0018 and
  `docs/defect_closure_execplans.md`. Use a Defect-Closure ExecPlan
  (DC-ExecPlan) when the objective is to close an observed invariant violation,
  a fail-closed event on valid input, or a conservation residual. Pure
  validation/characterization packages may keep the standard work-package shape
  only when they explicitly do not own correction.
- DC-ExecPlans must declare a Correction Authority Envelope: defect IDs,
  observed failures, in-scope contracts/source files, allowed edit classes,
  validation surfaces, acceptance criteria, and protected boundaries.
- If a DC-ExecPlan establishes a reproducible in-envelope root cause and the
  expected behavior is supported by canonical `SC-*` authority, pinned-baseline
  provenance, or a contract-authorized physical invariant, it must land the
  contract-first correction in the same package. Do not close as `HOLD` merely
  because more investigation is possible.
- A DC-ExecPlan may close in `HOLD` only at a declared boundary: out-of-envelope
  mechanism, missing or contradictory authority, invalid upstream input with a
  correct typed guard, unavailable evidence, or a different process family /
  contract authority. The handoff's first actionable item must be "close defect
  `<id>`", not "inspect `<function>`" or "trace `<variable>`".
- Every work package must require dual independent reviews with explicit
  finding disposition before closure. Each finding must be marked `accepted`,
  `rejected`, `deferred`, or `follow-up` with rationale; accepted findings must
  be fixed and verified, rejected findings must explain why no change is
  required, and deferred/follow-up findings must be linked from disposition and
  worker-handoff artifacts.
- Every work package must require explicit `.rs` line-count governance
  disposition in review artifacts and checklist artifacts:
  - files at or above `2000` lines are `WARN` and must include decomposition
    rationale plus follow-on split intent,
  - files at or above `3000` lines require refactor before closure unless an
    approved generated/fixture exception is documented with owner and sunset
    plan.
  Package closure is blocked while any `3000+` non-exempt file remains
  undispositioned.
- DC-ExecPlan reviews must also check `HOLD` legitimacy, envelope adequacy, and
  protected-boundary integrity before disposition.
- Prefer architectural work packages that implement, wire up, or complete
  baseline-parity functionality across process seams before narrow residual
  tuning. Residual-focused packages are appropriate after the authoritative
  process path is wired and observable, or when diagnostics prove a confined
  parity defect.
- Substantive implementation work must occur within an authorized
  `docs/work-packages/<id>/` scope, either during explicit package execution
  or as user-directed follow-on work from that package.
- When user-directed follow-on work is substantive and extends beyond the
  current package objective/write set, first evaluate closing out the current
  package and planning a new work-package before proceeding.
- If package scope boundaries are unclear, ask the user whether to continue
  under the current package or open a new one.
- Do not add fallback wrappers that silently mask missing required dependencies; prefer explicit failures.
- Correctness over completion: do not mark work complete when known invariant, closure, or contract violations remain unresolved.
- Do not canonicalize-and-proceed on kernel domain violations. Invalid,
  missing, physically impossible, or out-of-contract process state must fail
  closed with typed guards unless a canonical `SC-*` contract explicitly
  authorizes a bounded tolerance normalization.
- Bounded canonicalization is allowed only for contract-cited roundoff or
  publication-format normalization with explicit threshold, units, provenance,
  tests, and evidence. It must not change process control flow, hide mass
  imbalance, replace missing authority, or convert material negative storage /
  flux / SWE into a valid value.
- Removing, loosening, or converting a fail-closed guard from a prior work
  package requires contract-first amendment, a contract-derived regression test
  proving the old guard is obsolete or overbroad, before/after evidence, and
  accepted dual-review disposition. Without that evidence, preserve the guard
  and keep the package in `HOLD`.
- Do not implement provisional, surrogate, or heuristic process-physics math in production kernel/runtime publication paths.
- For legacy migration scope, required implementation target is baseline-authoritative physics migration from `/workdir/wepp-forest_260430_baseline` into openWEPP architecture, not behavioral approximation.
- If baseline-authoritative process physics is not yet ported, keep disposition in `HOLD` and open a follow-on package; do not close gaps with temporary formulas.
- Do not create or switch git branches unless the user explicitly asks.
- Inherit the agentic governance posture from wepppy. openWEPP is the simulation engine, not a separate culture.
- For kernel-affecting behavior (including runtime projection that controls kernel branches), treat canonical `SC-*` contracts as implementation authority and satisfy:
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- For code-authoring work packages when contract authority is applicable, use contract-first sequencing:
  1. implement required contract amendments,
  2. implement contract-derived tests,
  3. record pre-implementation contract-gate evidence, then
  4. modify production code.
- For edits touching external-authority suite posture, cohort fixtures, or
  required-case bindings, run source-level anti-evasion guards before
  disposition:
  - `bash tools/release/check_authority_suite_antievasion.sh`
  - `cargo test --test auth11_required_suite_obligation_guards_contract`

## Kernel Work-Package Preparation Procedure (Required)
Use this procedure when preparing any kernel-affecting work package (including
runtime projection that controls kernel branches). This is a root-level
requirement, not optional package style guidance.

1. Authorize and name the package
- Confirm the package is authorized by the active queue/decision artifacts.
- Before scaffolding, check that the package is neither too small nor too broad:
  it should contain a coherent closure slice large enough to justify the
  mandatory administrative artifacts and small enough for one agent to execute
  end-to-end without intervention.
- If the proposed package would only produce another narrow observe ledger,
  route label, or single-surface diagnostic, widen it to include the adjacent
  source-line classification, contract/test update, or implementation/evidence
  step when those share the same authority domain and validation surface.
- Split into a smaller package only when adjacent work crosses a distinct
  canonical contract authority, touches unrelated write sets, requires
  materially different validation, or is blocked by evidence that cannot be
  produced in the same autonomous run.
- Use directory format `YYYYMMDD-<slug>-001` under `docs/work-packages/`.
- Add/update the entry in `docs/work-packages/README.md` so intent is
  discoverable.

2. Scaffold required structure
- Create `package.md`, `prompts/active/<id>_kickoff_agent_prompt.md`,
  `prompts/README.md`, `prompts/active/README.md`,
  `prompts/archived/README.md`, and `artifacts/README.md`.
- Pre-create required artifact files as queued placeholders (`Status: queued`,
  `Evidence mode: not-run`) including:
  - contract implementation evidence,
  - contract-test implementation evidence,
  - pre-implementation contract gate,
  - implementation/test evidence,
  - kernel-profile compliance checklist,
  - owned-file manifest,
  - gate results,
  - line-count governance checklist,
  - disposition,
  - worker handoff,
  - dual review artifacts with mandatory finding disposition templates
    (`review_agent_a.md`, `review_agent_b.md`),
  - dual verification artifacts
    (`verification_agent_a.md`, `verification_agent_b.md`).

3. Encode mandatory `package.md` content
- Status (`queued`), objective, rationale, included/excluded scope, explicit
  deliverables, dependencies, intended write set, phase plan, exit criteria,
  and security-impact gate.
- For DC-ExecPlans, encode the Correction Authority Envelope, conversion rule,
  seven-gate bar, `HOLD`-legitimacy boundaries, and defect-shaped handoff.
- Explicitly encode autonomous execution intent: the package must contain enough
  context, sequencing, and validation detail for no-intervention execution.
- Explicitly state contract-first sequence:
  1. contracts,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.
- Explicitly require dual reviews, finding disposition, and verification that no
  review findings remain undispositioned before final package disposition.
- Explicitly require line-count-governance disposition in review and checklist
  artifacts, including owner/sunset metadata for any approved `3000+`
  generated/fixture exception.
- Require truthfulness labeling in evidence artifacts (`Static:` vs `Ran:`).

4. Make physics authority explicit in canonical contracts
- For legacy migration packages, physics must be implemented in canonical
  `SC-*` science contracts first; package-local notes are not authority.
- Physics/equation authority defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Do not invent physics: every equation, constant, guard, and invariant must
  trace to canonical contract text plus provenance citations.
- Migration completeness is required for touched process families (for example
  ET, infiltration, snow/frost, runoff, routing): do not merge or disposition
  as complete with placeholder/proxy equations when baseline-authoritative
  routines are known and in scope for migration.
- Preserve variable naming continuity with legacy WEPP symbols; when runtime
  names differ, record explicit alias mappings in the relevant `SC-*` files.

5. Require baseline reference set in Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- Kernel-relevant canonical contracts in
  `docs/specifications/science-contracts/contracts/SC-*.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` for legacy
  migration/parity packages
- Upstream queue/hold-lift/disposition artifacts that authorize the package

6. Enforce completion gates in the prepared prompt
- Kickoff prompt must prohibit kernel code edits before contract + test + gate
  completion.
- Kickoff prompt must explicitly require canonical `SC-*` physics authority
  updates for migration packages.
- Kickoff prompt must prohibit silent defaults, unbounded clamping, and
  canonicalize-and-proceed behavior for domain violations; require typed
  fail-closed errors/guards unless bounded normalization is explicitly
  contract-authorized.
- Kickoff prompt must instruct autonomous progression through the package phase
  plan and artifact updates through disposition without asking the user for
  "next steps" unless blocked.
- Kickoff prompt must include an explicit end-to-end execution statement (use
  `Autonomy:` line) for the declared scope.
- DC-ExecPlan kickoff prompts must say `Close defect <id> end-to-end`, include
  the Correction Authority Envelope, require conversion to a contract-first fix
  when the seven-gate bar is met, and prohibit relaying intermediate diagnostic
  steps into a new package.
- Kickoff prompt must include a `Required reading` section with explicit path
  references so onboarding/orientation does not require independent discovery.
  At minimum include:
  - `/workdir/openWEPP/AGENTS.md`
  - `/workdir/openWEPP/docs/codex_exec_plans.md`
  - `/workdir/openWEPP/docs/defect_closure_execplans.md` for defect-closure packages
  - `/workdir/openWEPP/docs/work-packages/README.md`
  - the package-local `package.md`
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  - `docs/specifications/science-contracts/index.md`
  - all phase-relevant canonical `SC-*` contracts and decision/queue artifacts.

## Prompt Wording Guidance (Required)
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

## Project Role
openWEPP is the Rust simulation engine. openWEPP owns its architecture and
science-contract authority for openWEPP behavior. wepppy owns orchestration,
GIS, climate, and run state. Do not duplicate wepppy concerns in this repo.

## Strategy and Provenance Model (Explicitly Non-Clean-Room)
- openWEPP is explicitly **not** a clean-room rewrite.
- Architecture is implemented **first**: typed state, module boundaries,
  orchestration flow, and contracts.
- Science contracts are authored top-down from `references/50201000`,
  literature invariants, physical/common-sense invariants, and static legacy
  code analysis.
- Agents may read legacy F90 source and existing science contracts directly for
  provenance mapping and static inspection.
- Canonical legacy provenance/comparator baseline is the pinned worktree
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` (see
  `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`).
- Binary pass serialization (HBP shard format/reader-writer behavior) is still
  implemented based on `/workdir/wepp-forest` contract authority; use
  `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` and
  `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md`
  as upstream sources and record commit SHA provenance in work packages.
- `/workdir/wepp-forest` HEAD remains valid for exploratory discovery, but
  normative contract/spec provenance and baseline comparator references should
  default to the pinned baseline unless explicitly justified.
- Do not invent physics. Every kernel constant and invariant must trace to a
  citation or explicit physical rationale.
- For migration packages, process-physics implementation closure means porting
  baseline-authoritative routines into openWEPP architecture with explicit
  provenance mapping; approximations are not acceptable as final closure.
- Variable naming continuity is required in science contracts: default to
  `wepp-forest`/legacy WEPP symbols for canonical variable tables and equations;
  when openWEPP boundary names differ, provide explicit alias mappings rather
  than replacing canonical symbols.
- Empirical multi-coefficient regressions are rejected per the process-based kernel preference rule carried forward from wepp-palimpsest WB-33 `WB33-C-20`.
- See [docs/decisions/0011-architecture-first-top-down-science-contracts.md](docs/decisions/0011-architecture-first-top-down-science-contracts.md).

## Kernel Contract Governance (Normative)
- Canonical authority location for process contracts is `docs/specifications/science-contracts/contracts/SC-<DOMAIN>-<NNN>.md`.
- Work-package artifacts are evidence, not authority replacement.
- Kernel-affecting packages must include a kernel-profile compliance checklist artifact and remain in `HOLD` when profile/procedure requirements are incomplete.
- Agent discovery order for kernel-contract work:
  1. `AGENTS.md` (this file)
  2. `docs/specifications/science-contract-authoring-procedure.md`
  3. `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  4. `docs/specifications/science-contracts/index.md`

## Validation Gates
Before declaring a Rust kernel implementation complete:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check` (licenses, advisories, sources)
5. Contract invariants and closure checks pass for the touched state surfaces.
6. Legacy comparator deltas are reviewed using confidence tiers:
   - single OFE + daily water-balance: higher-confidence acceptance signal
   - hourly/watershed: investigation signal, not standalone rejection rule

## Error Handling
- No broad `Result<_, Box<dyn Error>>` swallowing in production paths; use typed error enums per crate.
- No `.unwrap()` or `.expect()` in production paths. Test code may use them with intent.
- Do not silently mask numerical edge cases (NaN, divide-by-zero, overflow) with default values; surface them as typed errors and let the orchestrator decide.
- `unsafe` blocks require a `// SAFETY: ...` comment explaining the invariant.

## Numerics
See [docs/numerics/README.md](docs/numerics/README.md). The repo commits to
semantic parity rather than bit-for-bit reproduction. Comparator outputs are
interpreted by confidence tier per ADR-0011. Bit reproducibility across cores /
platforms is not a release gate; within-config (single thread, pinned seed) bit
reproducibility is.

## Subprocess Orchestration
The watershed CLI spawns the hillslope CLI as a subprocess per hillslope. wepppy spawns openWEPP CLIs as subprocesses. Argument construction must use `std::process::Command` with explicit arg arrays. No shell interpolation. See [docs/decisions/0004-subprocess-hillslope-orchestration.md](docs/decisions/0004-subprocess-hillslope-orchestration.md).

## Agent Feedback Loop
- Treat avoidable friction as diagnostic signal about the codebase, docs, tooling, or task framing.
- Surface unprompted feedback when it would materially improve clarity or interfaces.
- Distinguish confirmed defects / local painpoints / speculative improvements.
- Do not propose speculative redesigns; prefer concrete substrate improvements tied to a task experience.

## Truthfulness
Inherit the truthfulness posture from wepppy. Match verbs to evidence. Label evidence class (`Static:` vs `Ran:`) at the top of reviews and audits. A validator (`cargo check`, `cargo build`) is not the workflow (`cargo test`, comparator harness run). When skipping execution, say so plainly.
