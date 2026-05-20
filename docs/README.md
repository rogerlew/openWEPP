# openWEPP Documentation Index

| Doc | Audience | Purpose |
|---|---|---|
| [architecture/README.md](architecture/README.md) | All | Runtime topology, process model, data flow |
| [specifications/README.md](specifications/README.md) | All | Science-contract authority model and source hierarchy |
| [specifications/science-contracts/README.md](specifications/science-contracts/README.md) | Maintainers | Canonical location and registry for `SC-*` science contracts |
| [contracts/README.md](contracts/README.md) | All | Interface contracts: `.run`, HBP, parquet schemas, CLI ABI |
| [decisions/README.md](decisions/README.md) | All | Architecture decision records (ADRs) |
| [governance/README.md](governance/README.md) | Maintainers | Governance policies, transition plans, lifecycle controls |
| [numerics/README.md](numerics/README.md) | All | Floating-point, RNG, summation policy |
| [backlog/README.md](backlog/README.md) | Maintainers | Concept-stage ideas and promotion criteria before work-package activation |
| [../references/README.md](../references/README.md) | Maintainers | Reference corpus policy, bibliography, and vendoring layout |
| [standards/README.md](standards/README.md) | Maintainers | Rust coding, comments, and QA standards |
| [work-packages/README.md](work-packages/README.md) | All | Dated initiative tracking convention |

End-user documentation lives in [/usersum](../usersum/), following the wepppy `usersum` convention so it can be vendored into wepppy's in-app documentation engine.

Root-level agent guides:
- [../README.md](../README.md) — broad audience identity and scope
- [../AGENTS.md](../AGENTS.md) — Codex coding playbook
- [../CLAUDE.md](../CLAUDE.md) — Claude Code review / debug playbook

Provenance policy:
- [decisions/0011-architecture-first-top-down-science-contracts.md](decisions/0011-architecture-first-top-down-science-contracts.md) — architecture-first strategy, contract authority model, comparator tier policy.
