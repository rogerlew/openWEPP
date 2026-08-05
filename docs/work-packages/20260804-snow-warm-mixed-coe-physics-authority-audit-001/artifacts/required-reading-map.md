# Required Reading Map

Status: complete / scaffold intake

Evidence mode: Static + Ran

Core byte total: `470453`.

Threshold: `WARN` because the total is greater than `400000` and less than
`800000` bytes. The catalog contributes `374222` bytes and remains Core because
it owns package discovery and recent snow-package context. On-demand
authorities are loaded only for their named phase.

| Path | Tier | Rationale | Trigger | Status |
| --- | --- | --- | --- | --- |
| `/home/workdir/openWEPP/AGENTS.md` | Core | Root governance | Always | read |
| `docs/codex_exec_plans.md` | Core | Living ExecPlan rules | Always | read |
| `docs/work-packages/AGENTS.md` | Core | Package lifecycle and review | Always | read |
| `docs/work-packages/README.md` | Core | Package catalog/process | Always | relevant sections read |
| `docs/standards/testing-and-gate-strategy.md` | Core | Direct gate lifecycle | Always | read |
| package-local `package.md` | Core | Scope and acceptance | Always | read after scaffold (`15170` bytes at intake) |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Contract interpretation | Contract reasoning | read |
| `crates/AGENTS.md` | Conditional | Rust interpretation | Rust reasoning | read |
| `tests/AGENTS.md` | Conditional | Test edits | Only if test write authorized | read; no edit triggered |
| `SC-SNOWFREEZE-001.md` | On-demand | Canonical snow authority | Formula audit | relevant sections read |
| `SC-SNOWENERGY-001.md` | On-demand | State-resolved energy authority | Physical audit | relevant sections read |
| `references/50201000/chap3.pdf` | On-demand | 1995 WEPP equations | Formula audit | Section 3.6 read |
| pinned `src/melt.for` | On-demand | Post-2007 legacy lineage | Formula audit | read |
| current CoE Rust source | On-demand | Executable lineage | Formula/runtime audit | read |
| independent physical PDFs | On-demand | First-principles cross-check | Physical audit | Marks 1998, Marks 1999, Ohmura 2001, and Walter 2005 relevant sections read |
| 21L package and accepted tables | On-demand | Frozen diagnostic population | Quantitative audit | read |

Measurement ran from `/home/workdir/openWEPP` with `wc -c` over the six exact
Core paths on `2026-08-05T04:25:28Z`.
