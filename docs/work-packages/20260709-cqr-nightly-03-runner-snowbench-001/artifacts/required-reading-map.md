# Required Reading Map

Evidence label: Static/Ran.

Package: `20260709-cqr-nightly-03-runner-snowbench-001`

## Core

| Path | Tier | Rationale | Trigger | Read status |
|---|---|---|---|---|
| `AGENTS.md` | Core | Root repository governance and CQR/package constraints. | Always | Read in session prompt and earlier package execution. |
| `docs/work-packages/AGENTS.md` | Core | Work-package scaffold, commit, gate, review, and subagent rules. | Always | Read. |
| `docs/work-packages/20260709-cqr-nightly-03-runner-snowbench-001/package.md` | Core | Package-local authority and write set. | Always | Created in scaffold; read before implementation. |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | Core | Defines CQR nightly target selection, package shape, gates, and commits. | Always | Read. |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | Behavior-preserving refactor process and gate ladder. | Always | Read. |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | CRAP decomposition and numeric/API identity requirements. | Always | Read. |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | CRAP `<= 30` and coverage-closure policy. | Always | Read. |
| `docs/standards/prompt-wording-guidance.md` | Core | Kickoff prompt and subagent wording requirements. | Always | Read. |
| `crates/AGENTS.md` | Core | Rust crate authoring, validation, and line-count rules. | Crate source target. | Read. |
| `tests/AGENTS.md` | Core | Integration and module-local test conventions. | Characterization tests possible. | Read. |
| `docs/work-packages/20260709-cqr-nightly-03-runner-snowbench-001/artifacts/required-reading-map.md` | Core | Package-local reading budget and trigger map. | Always | Created in scaffold; read before implementation. |

## Conditional

| Path | Tier | Rationale | Trigger | Read status |
|---|---|---|---|---|
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Science-contract handling rules. | Required before contract-authority, science-behavior, or contract-derived-test edits. | Read. |
| `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | Conditional | Snow/frost default/opt-in and diagnostic-surface confinement authority. | Required before contract-authority, science-behavior, or contract-derived-test edits; targeted static inspection is enough for CLI-only refactor. | Targeted static inspection of relevant snowbench/default/opt-in markers completed; full read not required for CLI-only implementation. |
| `docs/standards/local-ci-gate-selection.md` | Conditional | Local focused-gate tier selection and timing evidence. | Required if narrowing expensive focused iteration gates before final closure. | Pending until triggered. |

## On-Demand Source And Tests

| Path | Tier | Rationale | Trigger | Read status |
|---|---|---|---|---|
| `crates/openwepp-runner/src/bin/openwepp-snowbench.rs` | On-demand/source | Target module and likely test host. | Always before implementation. | Read. |
| `tests/integration/snowdensity05f_melt_closure_handoff.rs` | On-demand/test | Confirms snowbench CLI remains confined diagnostic opt-in surface for CoE melt. | Before editing CLI markers or test. | Read. |
| `tests/integration/snowdensity03_physics_bulk_offline_contract.rs` | On-demand/test | Confirms `physics_bulk` mentions stay confined to authorized opt-in surfaces including snowbench. | Before editing CLI markers or test. | Read. |
| Adjacent imported snowbench modules under `crates/openwepp-runner/src/hillslope/` | On-demand/source | Request structs, parsers, and diagnostic runner semantics. | Read the specific module if its API/error behavior becomes necessary for implementation. | Pending until triggered. |

## Budget

Core pre-edit reading excludes the full `SC-SNOWFREEZE-001.md` contract because
this package does not edit contract authority or snow/frost science behavior.
Full contract read is triggered only if implementation expands into
contract-derived tests or contract authority.

Core pre-edit required reading is `103090` bytes, which is below the `OK`
threshold (`<=400000` bytes).
