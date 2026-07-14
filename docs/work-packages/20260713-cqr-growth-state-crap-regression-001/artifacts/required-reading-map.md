# Required Reading Map

Evidence mode: Static.

| Path | Tier | Applicability | Status |
| --- | --- | --- | --- |
| `AGENTS.md` | 1 | Repository governance and validation | Read |
| `crates/AGENTS.md` | 1 | Rust/kernel implementation rules | Read |
| `docs/work-packages/AGENTS.md` | 1 | Package execution and review | Read |
| `docs/specifications/science-contracts/AGENTS.md` | 1 | Kernel contract authority | Read |
| package-local `package.md` | 1 | Scope and closure criteria | Read |
| `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | 2 | Root-cap ordering and output invariants | Read in full |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | 2 | CRAP threshold and eligibility | Read |
| `docs/standards/code-quality-refactor-authoring-guide.md` | 2 | CQR procedure | Read |
| `docs/standards/mechanical-refactor-authoring-guide.md` | 2 | Behavior-preserving extraction | Read |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs` | 2 | Target and characterization tests | Read |
| `docs/work-packages/20260701-kernel-boundary-cqr-row6-growth-decomposition-001/` | 3 | Prior growth CQR precedent | Read relevant package/review/metric evidence |
| `/tmp/openwepp-acrap-live-20260713/workspace-crap.json` | 3 | Fresh measurement provenance | Read target row |

Instruction discovery was run with `tools/agents/find-agents --for` for every
intended write path. `AGENTS.md` and `crates/AGENTS.md` govern the Rust target;
`AGENTS.md` and `docs/work-packages/AGENTS.md` govern package artifacts.
