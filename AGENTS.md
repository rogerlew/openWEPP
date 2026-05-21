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
- Do not add fallback wrappers that silently mask missing required dependencies; prefer explicit failures.
- Correctness over completion: do not mark work complete when known invariant, closure, or contract violations remain unresolved.
- Do not create or switch git branches unless the user explicitly asks.
- Inherit the agentic governance posture from wepppy. openWEPP is the simulation engine, not a separate culture.

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
- Do not invent physics. Every kernel constant and invariant must trace to a
  citation or explicit physical rationale.
- Variable naming continuity is required in science contracts: default to
  `wepp-forest`/legacy WEPP symbols for canonical variable tables and equations;
  when openWEPP boundary names differ, provide explicit alias mappings rather
  than replacing canonical symbols.
- Empirical multi-coefficient regressions are rejected per the process-based kernel preference rule carried forward from wepp-palimpsest WB-33 `WB33-C-20`.
- See [docs/decisions/0011-architecture-first-top-down-science-contracts.md](docs/decisions/0011-architecture-first-top-down-science-contracts.md).

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
