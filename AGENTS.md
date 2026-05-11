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
- Do not create or switch git branches unless the user explicitly asks.
- Inherit the agentic governance posture from wepppy. openWEPP is the simulation engine, not a separate culture.

## Project Role
openWEPP is the Rust simulation engine. `wepp-palimpsest` owns the authoritative F90 kernel source and science contracts; wepppy owns orchestration, GIS, climate, and run state. Do not duplicate wepppy concerns in this repo.

## Clean-Room Model (Kernel-Mirror Port)
- openWEPP Rust kernels port from `wepp-palimpsest` F90 kernels only after the upstream kernel has reached `active` status with oracle vectors published.
- Agents may read both the upstream F90 source and the corresponding science contract (`SC-DOMAIN-NNN.md`) when authoring a Rust kernel port.
- Do not invent physics. Replacements citing only Rust idioms are not acceptable. Every kernel constant must trace to a hydraulic citation per the wepp-palimpsest anti-tuning rule.
- Empirical multi-coefficient regressions are rejected per the process-based kernel preference rule carried forward from wepp-palimpsest WB-33 `WB33-C-20`.
- See [docs/decisions/0002-clean-room-model.md](docs/decisions/0002-clean-room-model.md).

## Validation Gates
Before declaring a Rust kernel port complete:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check` (licenses, advisories, sources)
5. Oracle parity harness passes against the wepp-palimpsest binary at the configured tolerance. Semantic parity, not bit parity; see [docs/decisions/0003-parity-semantic-not-bit.md](docs/decisions/0003-parity-semantic-not-bit.md).

## Error Handling
- No broad `Result<_, Box<dyn Error>>` swallowing in production paths; use typed error enums per crate.
- No `.unwrap()` or `.expect()` in production paths. Test code may use them with intent.
- Do not silently mask numerical edge cases (NaN, divide-by-zero, overflow) with default values; surface them as typed errors and let the orchestrator decide.
- `unsafe` blocks require a `// SAFETY: ...` comment explaining the invariant.

## Numerics
See [docs/numerics/README.md](docs/numerics/README.md). The repo commits to semantic parity rather than bit-for-bit reproduction of wepp-palimpsest output. Bit reproducibility across cores / platforms is not a release gate; within-config (single thread, pinned seed) bit reproducibility is.

## Subprocess Orchestration
The watershed CLI spawns the hillslope CLI as a subprocess per hillslope. wepppy spawns openWEPP CLIs as subprocesses. Argument construction must use `std::process::Command` with explicit arg arrays. No shell interpolation. See [docs/decisions/0004-subprocess-hillslope-orchestration.md](docs/decisions/0004-subprocess-hillslope-orchestration.md).

## Agent Feedback Loop
- Treat avoidable friction as diagnostic signal about the codebase, docs, tooling, or task framing.
- Surface unprompted feedback when it would materially improve clarity or interfaces.
- Distinguish confirmed defects / local painpoints / speculative improvements.
- Do not propose speculative redesigns; prefer concrete substrate improvements tied to a task experience.

## Truthfulness
Inherit the truthfulness posture from wepppy. Match verbs to evidence. Label evidence class (`Static:` vs `Ran:`) at the top of reviews and audits. A validator (`cargo check`, `cargo build`) is not the workflow (`cargo test`, oracle harness run). When skipping execution, say so plainly.
