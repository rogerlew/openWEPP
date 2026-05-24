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

## Core Directives
- `??` in a prompt means provide critical analysis only; do not implement code.
- Ask for clarification when requirements or debug context are ambiguous.
- Keep docs terse: Codex loads context in bulk and does not compress verbose guidance.
- Work packages are autonomous execution specs: front-load planning and context
  so an agent can execute from kickoff through disposition without user
  intervention.
- Work-package authoring must follow `docs/codex_exec_plans.md` for autonomy,
  self-containment, and observable validation expectations.
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

## Kernel Work-Package Preparation Procedure (Required)
Use this procedure when preparing any kernel-affecting work package (including
runtime projection that controls kernel branches). This is a root-level
requirement, not optional package style guidance.

1. Authorize and name the package
- Confirm the package is authorized by the active queue/decision artifacts.
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
  - disposition,
  - worker handoff,
  - dual review artifacts (`review_agent_a.md`, `review_agent_b.md`),
  - dual verification artifacts
    (`verification_agent_a.md`, `verification_agent_b.md`).

3. Encode mandatory `package.md` content
- Status (`queued`), objective, rationale, included/excluded scope, explicit
  deliverables, dependencies, intended write set, phase plan, exit criteria,
  and security-impact gate.
- Explicitly encode autonomous execution intent: the package must contain enough
  context, sequencing, and validation detail for no-intervention execution.
- Explicitly state contract-first sequence:
  1. contracts,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.
- Require truthfulness labeling in evidence artifacts (`Static:` vs `Ran:`).

4. Make physics authority explicit in canonical contracts
- For legacy migration packages, physics must be implemented in canonical
  `SC-*` science contracts first; package-local notes are not authority.
- Physics/equation authority defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` unless explicitly justified.
- Do not invent physics: every equation, constant, guard, and invariant must
  trace to canonical contract text plus provenance citations.
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
- Kickoff prompt must prohibit silent defaults/clamping for domain violations
  and require typed errors/guards.
- Kickoff prompt must instruct autonomous progression through the assigned phase
  and artifact updates without asking the user for "next steps" unless blocked.
- Kickoff prompt must include an explicit end-to-end execution statement (use
  `Autonomy:` line) for the declared scope.
- Kickoff prompt must include a `Required reading` section with explicit path
  references so onboarding/orientation does not require independent discovery.
  At minimum include:
  - `/workdir/openWEPP/AGENTS.md`
  - `/workdir/openWEPP/docs/codex_exec_plans.md`
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

2. Keep each prompt single-phase
- Use one phase per prompt:
  - phase A: contracts only,
  - phase B: contract-derived tests + pre-implementation gate evidence,
  - phase C: production code edits,
  - phase D: verification/disposition artifacts.
- Do not combine all phases into one long prompt.

3. Use concrete path-scoped wording
- Name exact file paths and sections/functions to edit.
- Prefer short imperative language (`read`, `amend`, `add tests`, `record`).
- Avoid broad/open-ended wording that does not constrain scope.
- Add an explicit `Required reading` list before task instructions so agents can
  load orientation context deterministically.

4. Preserve mandatory technical gates in every phase prompt
- Contract-first sequencing.
- Canonical `SC-*` authority requirements.
- Legacy baseline provenance requirement when migration applies.
- Dual review and dual verification requirements where applicable.
- Autonomous execution expectation for the phase (no user intervention unless
  hard-blocked).

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
- `Phase: <A|B|C|D> only.`
- `Required reading (read before edits): <explicit path list>.`
- `Files: <explicit path list>.`
- `Task: <single concrete change objective>.`
- `Constraints: contract-first sequencing; canonical SC authority;`
  `baseline provenance (<if applicable>); typed guards; no silent defaults.`
- `Autonomy: execute this phase end-to-end and update phase artifacts without`
  `requesting additional user direction unless hard-blocked.`
- `Outputs: update listed WB/EROD/CLIM artifacts for this phase only.`

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
