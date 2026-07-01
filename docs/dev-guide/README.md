# openWEPP Developer Guide

Status: living — human-facing onboarding documentation
Audience: developers new to openWEPP
Owner: maintainers (Claude Code maintains this guide)
Last updated: 2026-07-01

This guide is for **human developers** joining openWEPP — especially developers
whose reference points are legacy scientific codebases such as fixed-form
Fortran WEPP, libsnobal, or free-form Fortran SWAT+. openWEPP is deliberately
different from those codebases, in architecture and in process, and the
differences are not incidental style: each one exists because a specific,
recorded failure mode of the legacy model motivated it.

The rest of `docs/` is *reference* material — contracts, specifications, ADRs,
work-package evidence — written for contributors who already know the shape of
the project. This guide is the *narrative* entry point: what the system is, how
a simulation actually flows through the code, why the architecture looks the
way it does, and the vocabulary you need to read everything else.

## Chapters

| # | Chapter | What it answers |
|---|---|---|
| 1 | [Orientation](01-orientation.md) | What are the moving parts? What lives where in the repo? Which documents are authoritative for what? |
| 2 | [Principles](02-principles.md) | Why contract-first instead of matching the legacy binary? Why architecture-first? What does "closure before magnitude" mean? |
| 3 | [The hillslope run, end to end](03-hillslope-codeflow.md) | What happens between `openwepp-cli-hill` starting and parquet appearing on disk? The direct runtime, frames, phases, and publication. |
| 4 | [Watershed runtime and outputs](04-watershed-and-outputs.md) | How hillslope runs compose into a watershed run; HBP handoff; the output surface. |
| 5 | [Concepts and glossary](05-concepts-glossary.md) | The project vocabulary — day frame, phase span, shadow projection, closure, protected outputs, science contracts — plus a Fortran-to-openWEPP translation table. |
| 6 | [How the architecture got here](06-history-and-performance.md) | The performance history that produced the array-native direct runtime, and why "just cache it" rungs were tried and rejected. |
| 7 | [Contributing](07-contributing.md) | How a change actually lands: gates, work packages, review, and the agent-assisted authoring model. |

## Suggested reading paths

- **New contributor, first week:** 1 → 2 → 5, then 3 when you first touch
  runtime code. Chapter 6 is the best background reading for *why* the runtime
  code is shaped the way it is.
- **Scientist / model reviewer:** 2 → 5 → 3. The science-contract authority
  model ([docs/specifications/README.md](../specifications/README.md)) is the
  normative companion.
- **Coming from legacy WEPP Fortran:** 5 first — the translation table maps
  `COMMON` blocks, the daily loop, and WEPP variable names onto their openWEPP
  counterparts — then 3.

## What this guide is not

- Not normative authority. When this guide and a contract, specification, or
  ADR disagree, the contract/specification/ADR wins, and the discrepancy is a
  documentation bug worth reporting.
- Not end-user documentation. CLI usage for modelers lives in
  [/usersum](../../usersum/).
- Not the agent playbooks. Codex reads [AGENTS.md](../../AGENTS.md); Claude
  Code reads [CLAUDE.md](../../CLAUDE.md). Chapter 7 explains how the human
  and agent roles fit together.
